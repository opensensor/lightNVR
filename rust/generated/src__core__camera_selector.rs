unsafe extern "C" {
    unsafe fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type int64_t = i64;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_selector {
    pub version: ::core::ffi::c_int,
    pub root: *mut selector_node_t,
}
pub type selector_node_t = selector_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct selector_node {
    pub r#type: selector_node_type_t,
    pub children: *mut *mut selector_node,
    pub child_count: ::core::ffi::c_int,
    pub values: *mut *mut ::core::ffi::c_char,
    pub value_count: ::core::ffi::c_int,
    pub bool_value: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct selector_node_type_t(pub ::core::ffi::c_uint);
impl selector_node_type_t {
    pub const SELECTOR_ALL: Self = Self(0);
    pub const SELECTOR_AND: Self = Self(1);
    pub const SELECTOR_OR: Self = Self(2);
    pub const SELECTOR_NOT: Self = Self(3);
    pub const SELECTOR_CAMERA_UUID: Self = Self(4);
    pub const SELECTOR_LOCATION_SUBTREE: Self = Self(5);
    pub const SELECTOR_TAG_ANY: Self = Self(6);
    pub const SELECTOR_TAG_ALL: Self = Self(7);
    pub const SELECTOR_TAG_NONE: Self = Self(8);
    pub const SELECTOR_ENABLED: Self = Self(9);
    pub const SELECTOR_RECORDING_MODE: Self = Self(10);
    pub const SELECTOR_VENDOR: Self = Self(11);
    pub const SELECTOR_MODEL: Self = Self(12);
    pub const SELECTOR_CAPABILITY_ANY: Self = Self(13);
    pub const SELECTOR_CAPABILITY_ALL: Self = Self(14);
    pub const SELECTOR_HEALTH: Self = Self(15);
}
pub type fleet_selector_t = fleet_selector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_selector_explanation_t {
    pub clauses: [[::core::ffi::c_char; 160]; 64],
    pub clause_count: ::core::ffi::c_int,
}
pub type value_validator_t = Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> bool>;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const FLEET_SELECTOR_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FLEET_SELECTOR_MAX_DEPTH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const FLEET_SELECTOR_MAX_NODES: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const FLEET_SELECTOR_MAX_VALUES: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const FLEET_EXPLANATION_MAX_CLAUSES: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const FLEET_EXPLANATION_CLAUSE_MAX: ::core::ffi::c_int = 160 as ::core::ffi::c_int;
unsafe extern "C" fn set_error(
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    if error.is_null()
        || error_size == 0 as size_t
        || *error.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        return;
    }
    let mut args: ::core::ffi::VaList;
    args = c2rust_args.clone();
    vsnprintf(error, error_size, format, args.clone());
}
unsafe extern "C" fn valid_uuid(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null()
        || strlen(value) != (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t
    {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int {
        let c: ::core::ffi::c_char = *value.offset(i as isize);
        if i == 8 as ::core::ffi::c_int
            || i == 13 as ::core::ffi::c_int
            || i == 18 as ::core::ffi::c_int
            || i == 23 as ::core::ffi::c_int
        {
            if c as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                return r#false != 0;
            }
        } else if !(c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            || c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
            || c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int)
        {
            return r#false != 0;
        }
        i += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_nonempty(mut value: *const ::core::ffi::c_char) -> bool {
    return !value.is_null()
        && *value.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && strlen(value) < MAX_STREAM_NAME as size_t;
}
unsafe extern "C" fn valid_recording_mode(mut value: *const ::core::ffi::c_char) -> bool {
    return !value.is_null()
        && (strcmp(value, b"off\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(
                value,
                b"continuous\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(value, b"detection\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn valid_capability(mut value: *const ::core::ffi::c_char) -> bool {
    return !value.is_null()
        && (strcmp(value, b"onvif\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(value, b"ptz\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(
                value,
                b"backchannel\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn valid_health(mut value: *const ::core::ffi::c_char) -> bool {
    return !value.is_null()
        && (strcmp(value, b"unknown\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(value, b"up\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(value, b"degraded\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(value, b"down\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(value, b"disabled\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn selector_node_free(mut node: *mut selector_node_t) {
    if node.is_null() {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*node).child_count {
        selector_node_free(*(*node).children.offset(i as isize));
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*node).value_count {
        free(*(*node).values.offset(i_0 as isize) as *mut ::core::ffi::c_void);
        i_0 += 1;
    }
    free((*node).children as *mut ::core::ffi::c_void);
    free((*node).values as *mut ::core::ffi::c_void);
    free(node as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn parse_values(
    mut json: *const cJSON,
    mut field: *const ::core::ffi::c_char,
    mut validator: value_validator_t,
    mut node: *mut selector_node_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    let mut array: *const cJSON = cJSON_GetObjectItemCaseSensitive(json, field);
    let mut count: ::core::ffi::c_int = if cJSON_IsArray(array) != 0 {
        cJSON_GetArraySize(array)
    } else {
        -1 as ::core::ffi::c_int
    };
    if count < 1 as ::core::ffi::c_int || count > FLEET_SELECTOR_MAX_VALUES {
        set_error(
            error,
            error_size,
            b"%s must contain 1-%d values\0".as_ptr() as *const ::core::ffi::c_char,
            field,
            FLEET_SELECTOR_MAX_VALUES,
        );
        return r#false != 0;
    }
    (*node).values = calloc(
        count as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
    ) as *mut *mut ::core::ffi::c_char;
    if (*node).values.is_null() {
        set_error(
            error,
            error_size,
            b"Out of memory parsing selector\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *const cJSON = cJSON_GetArrayItem(array, i);
        if cJSON_IsString(item) == 0
            || (*item).valuestring.is_null()
            || strlen((*item).valuestring) >= MAX_STREAM_NAME as size_t
            || validator.is_some()
                && !validator.expect("non-null function pointer")((*item).valuestring)
        {
            set_error(
                error,
                error_size,
                b"%s contains an invalid value\0".as_ptr() as *const ::core::ffi::c_char,
                field,
            );
            return r#false != 0;
        }
        *(*node).values.offset(i as isize) = strdup((*item).valuestring);
        if (*(*node).values.offset(i as isize)).is_null() {
            set_error(
                error,
                error_size,
                b"Out of memory parsing selector\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*node).value_count += 1;
        i += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn parse_node(
    mut json: *const cJSON,
    mut depth: ::core::ffi::c_int,
    mut node_count: *mut ::core::ffi::c_int,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> *mut selector_node_t {
    if cJSON_IsObject(json) == 0 {
        set_error(
            error,
            error_size,
            b"Selector expression must be an object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<selector_node_t>();
    }
    if depth > FLEET_SELECTOR_MAX_DEPTH {
        set_error(
            error,
            error_size,
            b"Selector exceeds maximum depth %d\0".as_ptr() as *const ::core::ffi::c_char,
            FLEET_SELECTOR_MAX_DEPTH,
        );
        return ::core::ptr::null_mut::<selector_node_t>();
    }
    *node_count += 1;
    if *node_count > FLEET_SELECTOR_MAX_NODES {
        set_error(
            error,
            error_size,
            b"Selector exceeds maximum node count %d\0".as_ptr() as *const ::core::ffi::c_char,
            FLEET_SELECTOR_MAX_NODES,
        );
        return ::core::ptr::null_mut::<selector_node_t>();
    }
    let mut op_item: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(json, b"op\0".as_ptr() as *const ::core::ffi::c_char);
    if cJSON_IsString(op_item) == 0 || (*op_item).valuestring.is_null() {
        set_error(
            error,
            error_size,
            b"Selector expression requires string op\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<selector_node_t>();
    }
    let mut op: *const ::core::ffi::c_char = (*op_item).valuestring;
    let mut node: *mut selector_node_t =
        calloc(1 as size_t, ::core::mem::size_of::<selector_node_t>()) as *mut selector_node_t;
    if node.is_null() {
        set_error(
            error,
            error_size,
            b"Out of memory parsing selector\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<selector_node_t>();
    }
    if strcmp(op, b"all\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        (*node).r#type = selector_node_type_t::SELECTOR_ALL;
        return node;
    }
    if strcmp(op, b"and\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strcmp(op, b"or\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        let mut children: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            json,
            b"children\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut count: ::core::ffi::c_int = if cJSON_IsArray(children) != 0 {
            cJSON_GetArraySize(children)
        } else {
            -1 as ::core::ffi::c_int
        };
        if count < 1 as ::core::ffi::c_int || count > FLEET_SELECTOR_MAX_VALUES {
            set_error(
                error,
                error_size,
                b"%s requires 1-%d children\0".as_ptr() as *const ::core::ffi::c_char,
                op,
                FLEET_SELECTOR_MAX_VALUES,
            );
            selector_node_free(node);
            return ::core::ptr::null_mut::<selector_node_t>();
        }
        (*node).r#type = selector_node_type_t(
            (if strcmp(op, b"and\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                selector_node_type_t::SELECTOR_AND.0 as ::core::ffi::c_int
            } else {
                selector_node_type_t::SELECTOR_OR.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        (*node).children = calloc(
            count as size_t,
            ::core::mem::size_of::<*mut selector_node>(),
        ) as *mut *mut selector_node;
        if (*node).children.is_null() {
            set_error(
                error,
                error_size,
                b"Out of memory parsing selector\0".as_ptr() as *const ::core::ffi::c_char,
            );
            selector_node_free(node);
            return ::core::ptr::null_mut::<selector_node_t>();
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < count {
            *(*node).children.offset(i as isize) = parse_node(
                cJSON_GetArrayItem(children, i),
                depth + 1 as ::core::ffi::c_int,
                node_count,
                error,
                error_size,
            ) as *mut selector_node;
            if (*(*node).children.offset(i as isize)).is_null() {
                selector_node_free(node);
                return ::core::ptr::null_mut::<selector_node_t>();
            }
            (*node).child_count += 1;
            i += 1;
        }
        return node;
    }
    if strcmp(op, b"not\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        let mut child: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            json,
            b"child\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*node).r#type = selector_node_type_t::SELECTOR_NOT;
        (*node).children = calloc(1 as size_t, ::core::mem::size_of::<*mut selector_node>())
            as *mut *mut selector_node;
        if (*node).children.is_null() {
            set_error(
                error,
                error_size,
                b"Out of memory parsing selector\0".as_ptr() as *const ::core::ffi::c_char,
            );
            selector_node_free(node);
            return ::core::ptr::null_mut::<selector_node_t>();
        }
        *(*node).children.offset(0isize) = parse_node(
            child,
            depth + 1 as ::core::ffi::c_int,
            node_count,
            error,
            error_size,
        ) as *mut selector_node;
        if (*(*node).children.offset(0isize)).is_null() {
            selector_node_free(node);
            return ::core::ptr::null_mut::<selector_node_t>();
        }
        (*node).child_count = 1 as ::core::ffi::c_int;
        return node;
    }
    let mut parsed: bool = r#false != 0;
    if strcmp(op, b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        (*node).r#type = selector_node_type_t::SELECTOR_CAMERA_UUID;
        parsed = parse_values(
            json,
            b"values\0".as_ptr() as *const ::core::ffi::c_char,
            Some(valid_uuid as unsafe extern "C" fn(*const ::core::ffi::c_char) -> bool),
            node,
            error,
            error_size,
        );
    } else if strcmp(
        op,
        b"location_subtree\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut uuid: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            json,
            b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*node).r#type = selector_node_type_t::SELECTOR_LOCATION_SUBTREE;
        if cJSON_IsString(uuid) == 0 || !valid_uuid((*uuid).valuestring) {
            set_error(
                error,
                error_size,
                b"location_subtree requires a valid uuid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            (*node).values = calloc(
                1 as size_t,
                ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
            ) as *mut *mut ::core::ffi::c_char;
            if !(*node).values.is_null() {
                *(*node).values.offset(0isize) = strdup((*uuid).valuestring);
            }
            if !(*node).values.is_null() && !(*(*node).values.offset(0isize)).is_null() {
                (*node).value_count = 1 as ::core::ffi::c_int;
                parsed = r#true != 0;
            } else {
                set_error(
                    error,
                    error_size,
                    b"Out of memory parsing selector\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
    } else if strcmp(op, b"tag_any\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcmp(op, b"tag_all\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(op, b"tag_none\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        (*node).r#type = selector_node_type_t(
            (if strcmp(op, b"tag_any\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                selector_node_type_t::SELECTOR_TAG_ANY.0 as ::core::ffi::c_int
            } else if strcmp(op, b"tag_all\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                selector_node_type_t::SELECTOR_TAG_ALL.0 as ::core::ffi::c_int
            } else {
                selector_node_type_t::SELECTOR_TAG_NONE.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        parsed = parse_values(
            json,
            b"uuids\0".as_ptr() as *const ::core::ffi::c_char,
            Some(valid_uuid as unsafe extern "C" fn(*const ::core::ffi::c_char) -> bool),
            node,
            error,
            error_size,
        );
    } else if strcmp(op, b"enabled\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        let mut value: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            json,
            b"value\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*node).r#type = selector_node_type_t::SELECTOR_ENABLED;
        if cJSON_IsBool(value) == 0 {
            set_error(
                error,
                error_size,
                b"enabled requires boolean value\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            (*node).bool_value = cJSON_IsTrue(value) != 0;
            parsed = r#true != 0;
        }
    } else if strcmp(
        op,
        b"recording_mode\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*node).r#type = selector_node_type_t::SELECTOR_RECORDING_MODE;
        parsed = parse_values(
            json,
            b"values\0".as_ptr() as *const ::core::ffi::c_char,
            Some(valid_recording_mode as unsafe extern "C" fn(*const ::core::ffi::c_char) -> bool),
            node,
            error,
            error_size,
        );
    } else if strcmp(op, b"vendor\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcmp(op, b"model\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        (*node).r#type = selector_node_type_t(
            (if strcmp(op, b"vendor\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                selector_node_type_t::SELECTOR_VENDOR.0 as ::core::ffi::c_int
            } else {
                selector_node_type_t::SELECTOR_MODEL.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        parsed = parse_values(
            json,
            b"values\0".as_ptr() as *const ::core::ffi::c_char,
            Some(valid_nonempty as unsafe extern "C" fn(*const ::core::ffi::c_char) -> bool),
            node,
            error,
            error_size,
        );
    } else if strcmp(
        op,
        b"capability_any\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            op,
            b"capability_all\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        (*node).r#type = selector_node_type_t(
            (if strcmp(
                op,
                b"capability_any\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                selector_node_type_t::SELECTOR_CAPABILITY_ANY.0 as ::core::ffi::c_int
            } else {
                selector_node_type_t::SELECTOR_CAPABILITY_ALL.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        parsed = parse_values(
            json,
            b"values\0".as_ptr() as *const ::core::ffi::c_char,
            Some(valid_capability as unsafe extern "C" fn(*const ::core::ffi::c_char) -> bool),
            node,
            error,
            error_size,
        );
    } else if strcmp(op, b"health\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        (*node).r#type = selector_node_type_t::SELECTOR_HEALTH;
        parsed = parse_values(
            json,
            b"values\0".as_ptr() as *const ::core::ffi::c_char,
            Some(valid_health as unsafe extern "C" fn(*const ::core::ffi::c_char) -> bool),
            node,
            error,
            error_size,
        );
    } else {
        set_error(
            error,
            error_size,
            b"Unknown selector op: %s\0".as_ptr() as *const ::core::ffi::c_char,
            op,
        );
    }
    if !parsed {
        selector_node_free(node);
        return ::core::ptr::null_mut::<selector_node_t>();
    }
    return node;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fleet_selector_parse(
    mut json: *const cJSON,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> *mut fleet_selector_t {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if cJSON_IsObject(json) == 0 {
        set_error(
            error,
            error_size,
            b"selector must be an object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<fleet_selector_t>();
    }
    let mut version: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(json, b"version\0".as_ptr() as *const ::core::ffi::c_char);
    let mut expression: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        json,
        b"expression\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsNumber(version) == 0
        || (*version).valuedouble != FLEET_SELECTOR_VERSION as ::core::ffi::c_double
    {
        set_error(
            error,
            error_size,
            b"selector.version must be %d\0".as_ptr() as *const ::core::ffi::c_char,
            FLEET_SELECTOR_VERSION,
        );
        return ::core::ptr::null_mut::<fleet_selector_t>();
    }
    let mut selector: *mut fleet_selector_t =
        calloc(1 as size_t, ::core::mem::size_of::<fleet_selector_t>()) as *mut fleet_selector_t;
    if selector.is_null() {
        set_error(
            error,
            error_size,
            b"Out of memory parsing selector\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<fleet_selector_t>();
    }
    (*selector).version = FLEET_SELECTOR_VERSION;
    let mut node_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*selector).root = parse_node(
        expression,
        1 as ::core::ffi::c_int,
        &raw mut node_count,
        error,
        error_size,
    );
    if (*selector).root.is_null() {
        free(selector as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<fleet_selector_t>();
    }
    return selector;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fleet_selector_free(mut selector: *mut fleet_selector_t) {
    if selector.is_null() {
        return;
    }
    selector_node_free((*selector).root);
    free(selector as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fleet_health_state_name(
    mut state: fleet_health_state_t,
) -> *const ::core::ffi::c_char {
    match state {
        fleet_health_state_t::FLEET_HEALTH_UP => {
            return b"up\0".as_ptr() as *const ::core::ffi::c_char;
        }
        fleet_health_state_t::FLEET_HEALTH_DEGRADED => {
            return b"degraded\0".as_ptr() as *const ::core::ffi::c_char;
        }
        fleet_health_state_t::FLEET_HEALTH_DOWN => {
            return b"down\0".as_ptr() as *const ::core::ffi::c_char;
        }
        fleet_health_state_t::FLEET_HEALTH_DISABLED => {
            return b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fleet_availability_state_name(
    mut state: fleet_availability_state_t,
) -> *const ::core::ffi::c_char {
    match state {
        fleet_availability_state_t::FLEET_AVAILABILITY_LIVE => {
            return b"live\0".as_ptr() as *const ::core::ffi::c_char;
        }
        fleet_availability_state_t::FLEET_AVAILABILITY_OFFLINE => {
            return b"offline\0".as_ptr() as *const ::core::ffi::c_char;
        }
        fleet_availability_state_t::FLEET_AVAILABILITY_NEVER_CONNECTED => {
            return b"never_connected\0".as_ptr() as *const ::core::ffi::c_char;
        }
        fleet_availability_state_t::FLEET_AVAILABILITY_DISABLED => {
            return b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"never_connected\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fleet_camera_recording_mode(
    mut camera: *const fleet_camera_t,
) -> *const ::core::ffi::c_char {
    if camera.is_null() || !(*camera).record {
        return b"off\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return if (*camera).detection_based_recording as ::core::ffi::c_int != 0 {
        b"detection\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"continuous\0".as_ptr() as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn explain(
    mut explanation: *mut fleet_selector_explanation_t,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    if explanation.is_null() || (*explanation).clause_count >= FLEET_EXPLANATION_MAX_CLAUSES {
        return;
    }
    let mut args: ::core::ffi::VaList;
    args = c2rust_args.clone();
    vsnprintf(
        &raw mut *(&raw mut (*explanation).clauses as *mut [::core::ffi::c_char; 160])
            .offset((*explanation).clause_count as isize) as *mut ::core::ffi::c_char,
        FLEET_EXPLANATION_CLAUSE_MAX as size_t,
        format,
        args.clone(),
    );
    (*explanation).clause_count += 1;
}
unsafe extern "C" fn string_in_values(
    mut value: *const ::core::ffi::c_char,
    mut node: *const selector_node_t,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*node).value_count {
        if strcasecmp(
            if !value.is_null() {
                value
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            *(*node).values.offset(i as isize),
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn camera_has_tag(
    mut camera: *const fleet_camera_t,
    mut uuid: *const ::core::ffi::c_char,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*camera).tag_count {
        if strcasecmp(
            &raw const (*(&raw const (*camera).tags as *const fleet_camera_tag_t)
                .offset(i as isize))
            .uuid as *const ::core::ffi::c_char,
            uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn camera_has_capability(
    mut camera: *const fleet_camera_t,
    mut capability: *const ::core::ffi::c_char,
) -> bool {
    if strcmp(
        capability,
        b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return (*camera).is_onvif;
    }
    if strcmp(capability, b"ptz\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return (*camera).ptz_enabled;
    }
    if strcmp(
        capability,
        b"backchannel\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return (*camera).backchannel_enabled;
    }
    return r#false != 0;
}
unsafe extern "C" fn evaluate_node(
    mut node: *const selector_node_t,
    mut camera: *const fleet_camera_t,
    mut explanation: *mut fleet_selector_explanation_t,
) -> bool {
    let mut original_clause_count: ::core::ffi::c_int = if !explanation.is_null() {
        (*explanation).clause_count
    } else {
        0 as ::core::ffi::c_int
    };
    match (*node).r#type {
        selector_node_type_t::SELECTOR_ALL => {
            explain(
                explanation,
                b"all cameras\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#true != 0;
        }
        selector_node_type_t::SELECTOR_AND => {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*node).child_count {
                if !evaluate_node(*(*node).children.offset(i as isize), camera, explanation) {
                    if !explanation.is_null() {
                        (*explanation).clause_count = original_clause_count;
                    }
                    return r#false != 0;
                }
                i += 1;
            }
            return r#true != 0;
        }
        selector_node_type_t::SELECTOR_OR => {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < (*node).child_count {
                if !explanation.is_null() {
                    (*explanation).clause_count = original_clause_count;
                }
                if evaluate_node(*(*node).children.offset(i_0 as isize), camera, explanation) {
                    return r#true != 0;
                }
                i_0 += 1;
            }
            if !explanation.is_null() {
                (*explanation).clause_count = original_clause_count;
            }
            return r#false != 0;
        }
        selector_node_type_t::SELECTOR_NOT => {
            let mut child_match: bool =
                evaluate_node(*(*node).children.offset(0isize), camera, explanation);
            if !explanation.is_null() {
                (*explanation).clause_count = original_clause_count;
            }
            if !child_match {
                explain(
                    explanation,
                    b"not predicate\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return !child_match;
        }
        selector_node_type_t::SELECTOR_CAMERA_UUID => {
            if string_in_values(
                &raw const (*camera).camera_uuid as *const ::core::ffi::c_char,
                node,
            ) {
                explain(
                    explanation,
                    b"camera_uuid=%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*camera).camera_uuid as *const ::core::ffi::c_char,
                );
                return r#true != 0;
            }
            return r#false != 0;
        }
        selector_node_type_t::SELECTOR_LOCATION_SUBTREE => {
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < (*camera).location_depth {
                if strcasecmp(
                    &raw const *(&raw const (*camera).location_ancestor_uuids
                        as *const [::core::ffi::c_char; 37])
                        .offset(i_1 as isize) as *const ::core::ffi::c_char,
                    *(*node).values.offset(0isize),
                ) == 0 as ::core::ffi::c_int
                {
                    explain(
                        explanation,
                        b"location_subtree=%s\0".as_ptr() as *const ::core::ffi::c_char,
                        *(*node).values.offset(0isize),
                    );
                    return r#true != 0;
                }
                i_1 += 1;
            }
            return r#false != 0;
        }
        selector_node_type_t::SELECTOR_TAG_ANY => {
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2 < (*node).value_count {
                if camera_has_tag(camera, *(*node).values.offset(i_2 as isize)) {
                    explain(
                        explanation,
                        b"tag_any=%s\0".as_ptr() as *const ::core::ffi::c_char,
                        *(*node).values.offset(i_2 as isize),
                    );
                    return r#true != 0;
                }
                i_2 += 1;
            }
            return r#false != 0;
        }
        selector_node_type_t::SELECTOR_TAG_ALL => {
            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_3 < (*node).value_count {
                if !camera_has_tag(camera, *(*node).values.offset(i_3 as isize)) {
                    return r#false != 0;
                }
                i_3 += 1;
            }
            explain(
                explanation,
                b"tag_all (%d tags)\0".as_ptr() as *const ::core::ffi::c_char,
                (*node).value_count,
            );
            return r#true != 0;
        }
        selector_node_type_t::SELECTOR_TAG_NONE => {
            let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_4 < (*node).value_count {
                if camera_has_tag(camera, *(*node).values.offset(i_4 as isize)) {
                    return r#false != 0;
                }
                i_4 += 1;
            }
            explain(
                explanation,
                b"tag_none (%d tags)\0".as_ptr() as *const ::core::ffi::c_char,
                (*node).value_count,
            );
            return r#true != 0;
        }
        selector_node_type_t::SELECTOR_ENABLED => {
            if (*camera).enabled as ::core::ffi::c_int == (*node).bool_value as ::core::ffi::c_int {
                explain(
                    explanation,
                    b"enabled=%s\0".as_ptr() as *const ::core::ffi::c_char,
                    if (*node).bool_value as ::core::ffi::c_int != 0 {
                        b"true\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"false\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                return r#true != 0;
            }
            return r#false != 0;
        }
        selector_node_type_t::SELECTOR_RECORDING_MODE => {
            let mut mode: *const ::core::ffi::c_char = fleet_camera_recording_mode(camera);
            if string_in_values(mode, node) {
                explain(
                    explanation,
                    b"recording_mode=%s\0".as_ptr() as *const ::core::ffi::c_char,
                    mode,
                );
                return r#true != 0;
            }
            return r#false != 0;
        }
        selector_node_type_t::SELECTOR_VENDOR => {
            if string_in_values(
                &raw const (*camera).manufacturer as *const ::core::ffi::c_char,
                node,
            ) {
                explain(
                    explanation,
                    b"vendor=%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*camera).manufacturer as *const ::core::ffi::c_char,
                );
                return r#true != 0;
            }
            return r#false != 0;
        }
        selector_node_type_t::SELECTOR_MODEL => {
            if string_in_values(
                &raw const (*camera).model as *const ::core::ffi::c_char,
                node,
            ) {
                explain(
                    explanation,
                    b"model=%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*camera).model as *const ::core::ffi::c_char,
                );
                return r#true != 0;
            }
            return r#false != 0;
        }
        selector_node_type_t::SELECTOR_CAPABILITY_ANY => {
            let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_5 < (*node).value_count {
                if camera_has_capability(camera, *(*node).values.offset(i_5 as isize)) {
                    explain(
                        explanation,
                        b"capability_any=%s\0".as_ptr() as *const ::core::ffi::c_char,
                        *(*node).values.offset(i_5 as isize),
                    );
                    return r#true != 0;
                }
                i_5 += 1;
            }
            return r#false != 0;
        }
        selector_node_type_t::SELECTOR_CAPABILITY_ALL => {
            let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_6 < (*node).value_count {
                if !camera_has_capability(camera, *(*node).values.offset(i_6 as isize)) {
                    return r#false != 0;
                }
                i_6 += 1;
            }
            explain(
                explanation,
                b"capability_all (%d capabilities)\0".as_ptr() as *const ::core::ffi::c_char,
                (*node).value_count,
            );
            return r#true != 0;
        }
        selector_node_type_t::SELECTOR_HEALTH => {
            let mut health: *const ::core::ffi::c_char = fleet_health_state_name((*camera).health);
            if string_in_values(health, node) {
                explain(
                    explanation,
                    b"health=%s\0".as_ptr() as *const ::core::ffi::c_char,
                    health,
                );
                return r#true != 0;
            }
            return r#false != 0;
        }
        _ => {}
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fleet_selector_matches(
    mut selector: *const fleet_selector_t,
    mut camera: *const fleet_camera_t,
    mut explanation: *mut fleet_selector_explanation_t,
) -> bool {
    if selector.is_null() || (*selector).root.is_null() || camera.is_null() {
        return r#false != 0;
    }
    if !explanation.is_null() {
        memset(
            explanation as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<fleet_selector_explanation_t>(),
        );
    }
    return evaluate_node((*selector).root, camera, explanation);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
