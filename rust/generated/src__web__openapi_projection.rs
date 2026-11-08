unsafe extern "C" {
    pub type authorization_snapshot;
    unsafe fn cJSON_Delete(item_0: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsBool(item_0: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item_0: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item_0: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item_0: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item_0: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item_0: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_DetachItemViaPointer(parent: *mut cJSON, item_0: *mut cJSON) -> *mut cJSON;
    unsafe fn cJSON_DeleteItemFromObjectCaseSensitive(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
    );
    unsafe fn cJSON_Duplicate(item_0: *const cJSON, recurse: cJSON_bool) -> *mut cJSON;
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
    unsafe fn cJSON_AddObjectToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn authorization_action_from_key(
        key: *const ::core::ffi::c_char,
    ) -> authorization_action_t;
    unsafe fn authorization_snapshot_allows_non_scoped_identity(
        snapshot: *const authorization_snapshot_t,
    ) -> bool;
    unsafe fn authorization_snapshot_setup_complete(
        snapshot: *const authorization_snapshot_t,
    ) -> ::core::ffi::c_int;
    unsafe fn authorization_snapshot_camera_count(
        snapshot: *const authorization_snapshot_t,
    ) -> ::core::ffi::c_int;
    unsafe fn authorization_snapshot_evaluate(
        snapshot: *mut authorization_snapshot_t,
        action: authorization_action_t,
        camera_index: ::core::ffi::c_int,
        resource: *const ::core::ffi::c_char,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub type openapi_access_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const cJSON) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct projection_t {
    pub document: *mut cJSON,
    pub canonical: *const cJSON,
    pub access: openapi_access_fn,
    pub access_context: *mut ::core::ffi::c_void,
    pub operations: *mut cJSON,
    pub marks: *mut cJSON,
    pub queue: *mut cJSON,
    pub filtering: bool,
    pub collecting: bool,
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
pub type authorization_snapshot_t = authorization_snapshot;
pub const MAX_DEPTH: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const MAX_REFERENCE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const AUTH: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"x-lightnvr-authorization\0")
};
pub const VISIBILITY: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"x-lightnvr-visibility\0")
};
unsafe extern "C" fn item(
    mut node: *const cJSON,
    mut key: *const ::core::ffi::c_char,
) -> *mut cJSON {
    return cJSON_GetObjectItemCaseSensitive(node, key);
}
unsafe extern "C" fn string_is(
    mut node: *const cJSON,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    return cJSON_IsString(node) != 0
        && strcmp((*node).valuestring, value) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn unique_keys(mut node: *const cJSON) -> bool {
    if cJSON_IsObject(node) == 0 {
        return r#true != 0;
    }
    let mut a: *const cJSON = (*node).child;
    while !a.is_null() {
        if (*a).string.is_null() {
            return r#false != 0;
        }
        let mut b: *const cJSON = (*a).next;
        while !b.is_null() {
            if (*b).string.is_null() || strcmp((*a).string, (*b).string) == 0 as ::core::ffi::c_int
            {
                return r#false != 0;
            }
            b = (*b).next;
        }
        a = (*a).next;
    }
    return r#true != 0;
}
unsafe extern "C" fn is_method(mut key: *const ::core::ffi::c_char) -> bool {
    let mut methods: [*const ::core::ffi::c_char; 8] = [
        b"get\0".as_ptr() as *const ::core::ffi::c_char,
        b"put\0".as_ptr() as *const ::core::ffi::c_char,
        b"post\0".as_ptr() as *const ::core::ffi::c_char,
        b"delete\0".as_ptr() as *const ::core::ffi::c_char,
        b"patch\0".as_ptr() as *const ::core::ffi::c_char,
        b"options\0".as_ptr() as *const ::core::ffi::c_char,
        b"head\0".as_ptr() as *const ::core::ffi::c_char,
        b"trace\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 8]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if strcmp(key, methods[i]) == 0 {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn predicate_valid(mut predicate: *const cJSON) -> ::core::ffi::c_int {
    if cJSON_IsObject(predicate) == 0 || !unique_keys(predicate) {
        return -1 as ::core::ffi::c_int;
    }
    let mut version: *mut cJSON = item(
        predicate,
        b"version\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut kind: *mut cJSON = item(predicate, b"kind\0".as_ptr() as *const ::core::ffi::c_char);
    if cJSON_IsNumber(version) == 0
        || (*version).valuedouble != 1 as ::core::ffi::c_int as ::core::ffi::c_double
        || cJSON_IsString(kind) == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut simple: bool = string_is(
        kind,
        b"authenticated\0".as_ptr() as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || string_is(kind, b"public\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || string_is(
            kind,
            b"bootstrap_setup\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        || string_is(
            kind,
            b"non_scoped_identity\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        || string_is(
            kind,
            b"identity_management\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0;
    let mut camera: bool = string_is(kind, b"camera\0".as_ptr() as *const ::core::ffi::c_char)
        as ::core::ffi::c_int
        != 0
        || string_is(
            kind,
            b"non_scoped_camera\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        || string_is(
            kind,
            b"global_or_camera\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0;
    let mut global: bool = string_is(kind, b"global\0".as_ptr() as *const ::core::ffi::c_char)
        as ::core::ffi::c_int
        != 0
        || string_is(
            kind,
            b"non_scoped_global\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0;
    if !simple && !camera && !global
        || cJSON_GetArraySize(predicate)
            != if simple as ::core::ffi::c_int != 0 {
                2 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            }
    {
        return -1 as ::core::ffi::c_int;
    }
    if simple {
        return if !item(
            predicate,
            b"requirements\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    let mut requirements: *mut cJSON = item(
        predicate,
        b"requirements\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsArray(requirements) == 0 || (*requirements).child.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut req: *mut cJSON = (*requirements).child;
    while !req.is_null() {
        if cJSON_IsObject(req) == 0
            || !unique_keys(req)
            || cJSON_GetArraySize(req) != 2 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut action: *mut cJSON = item(req, b"action\0".as_ptr() as *const ::core::ffi::c_char);
        let mut resource: *mut cJSON =
            item(req, b"resource\0".as_ptr() as *const ::core::ffi::c_char);
        if cJSON_IsString(action) == 0
            || authorization_action_from_key((*action).valuestring).0
                == authorization_action_t::AUTHZ_ACTION_INVALID.0
            || !(string_is(resource, b"raw\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
                || string_is(
                    resource,
                    b"runtime\0".as_ptr() as *const ::core::ffi::c_char,
                ) as ::core::ffi::c_int
                    != 0
                || string_is(resource, b"live\0".as_ptr() as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int
                    != 0)
            || (global as ::core::ffi::c_int != 0
                || string_is(
                    kind,
                    b"global_or_camera\0".as_ptr() as *const ::core::ffi::c_char,
                ) as ::core::ffi::c_int
                    != 0)
                && !string_is(resource, b"raw\0".as_ptr() as *const ::core::ffi::c_char)
        {
            return -1 as ::core::ffi::c_int;
        }
        req = (*req).next as *mut cJSON;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapi_snapshot_access(
    mut opaque: *mut ::core::ffi::c_void,
    mut predicate: *const cJSON,
) -> ::core::ffi::c_int {
    let mut snapshot: *mut authorization_snapshot_t = opaque as *mut authorization_snapshot_t;
    if snapshot.is_null() || predicate_valid(predicate) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut kind: *mut cJSON = item(predicate, b"kind\0".as_ptr() as *const ::core::ffi::c_char);
    if string_is(
        kind,
        b"authenticated\0".as_ptr() as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || string_is(kind, b"public\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    if string_is(
        kind,
        b"bootstrap_setup\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        let mut complete: ::core::ffi::c_int = authorization_snapshot_setup_complete(snapshot);
        if complete < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if complete == 0 {
            return 1 as ::core::ffi::c_int;
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
        if authorization_snapshot_evaluate(
            snapshot,
            authorization_action_t::AUTHZ_SYSTEM_ADMIN,
            -1 as ::core::ffi::c_int,
            b"raw\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut evaluation,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        return if evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if string_is(
        kind,
        b"non_scoped_identity\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        return if authorization_snapshot_allows_non_scoped_identity(snapshot) as ::core::ffi::c_int
            != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if string_is(
        kind,
        b"identity_management\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        if authorization_snapshot_allows_non_scoped_identity(snapshot) {
            return 1 as ::core::ffi::c_int;
        }
        let mut evaluation_0: authorization_evaluation_t = authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        };
        if authorization_snapshot_evaluate(
            snapshot,
            authorization_action_t::AUTHZ_USERS_MANAGE,
            -1 as ::core::ffi::c_int,
            b"raw\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut evaluation_0,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        return if evaluation_0.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    let mut global: bool = string_is(kind, b"global\0".as_ptr() as *const ::core::ffi::c_char)
        as ::core::ffi::c_int
        != 0
        || string_is(
            kind,
            b"non_scoped_global\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0;
    if (string_is(
        kind,
        b"non_scoped_global\0".as_ptr() as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || string_is(
            kind,
            b"non_scoped_camera\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0)
        && !authorization_snapshot_allows_non_scoped_identity(snapshot)
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = if global as ::core::ffi::c_int != 0 {
        0 as ::core::ffi::c_int
    } else {
        authorization_snapshot_camera_count(snapshot)
    };
    let mut index: ::core::ffi::c_int = if global as ::core::ffi::c_int != 0
        || string_is(
            kind,
            b"global_or_camera\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    while index < count {
        let mut allowed: bool = r#true != 0;
        let mut req: *mut cJSON = (*item(
            predicate,
            b"requirements\0".as_ptr() as *const ::core::ffi::c_char,
        ))
        .child;
        while !req.is_null() {
            let mut evaluation_1: authorization_evaluation_t = authorization_evaluation_t {
                decision: authorization_decision_t::AUTHZ_DECISION_DENY,
                source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
                policy_version: 0,
                grant_uuid: [0; 37],
                token_uuid: [0; 37],
                role_uuid: [0; 37],
                role_name: [0; 128],
                explanation: [0; 256],
            };
            if authorization_snapshot_evaluate(
                snapshot,
                authorization_action_from_key(
                    (*item(req, b"action\0".as_ptr() as *const ::core::ffi::c_char)).valuestring,
                ),
                index,
                (*item(req, b"resource\0".as_ptr() as *const ::core::ffi::c_char)).valuestring,
                &raw mut evaluation_1,
            ) != 0 as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            if evaluation_1.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
                allowed = r#false != 0;
                break;
            } else {
                req = (*req).next as *mut cJSON;
            }
        }
        if allowed {
            return 1 as ::core::ffi::c_int;
        }
        index += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_token(
    mut start: *const ::core::ffi::c_char,
    mut length: size_t,
    mut out: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> ::core::ffi::c_int {
    let mut written: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < length {
        let mut value: ::core::ffi::c_char = *start.offset(i as isize);
        if value as ::core::ffi::c_int == '~' as ::core::ffi::c_int {
            i = i.wrapping_add(1);
            if i >= length
                || *start.offset(i as isize) as ::core::ffi::c_int != '0' as ::core::ffi::c_int
                    && *start.offset(i as isize) as ::core::ffi::c_int != '1' as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            value = (if *start.offset(i as isize) as ::core::ffi::c_int == '0' as ::core::ffi::c_int
            {
                '~' as ::core::ffi::c_int
            } else {
                '/' as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
        }
        if written.wrapping_add(1 as size_t) >= capacity {
            return -1 as ::core::ffi::c_int;
        }
        let c2rust_fresh0 = written;
        written = written.wrapping_add(1);
        *out.offset(c2rust_fresh0 as isize) = value;
        i = i.wrapping_add(1);
    }
    *out.offset(written as isize) = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn resolve_pointer(
    mut root: *const cJSON,
    mut reference_0: *const ::core::ffi::c_char,
) -> *const cJSON {
    if reference_0.is_null()
        || strncmp(
            reference_0,
            b"#/\0".as_ptr() as *const ::core::ffi::c_char,
            2 as size_t,
        ) != 0
        || strlen(reference_0) >= MAX_REFERENCE as size_t
    {
        return ::core::ptr::null::<cJSON>();
    }
    let mut node: *const cJSON = root;
    let mut part: *const ::core::ffi::c_char = reference_0.offset(2 as ::core::ffi::c_int as isize);
    loop {
        let mut slash: *const ::core::ffi::c_char = strchr(part, '/' as ::core::ffi::c_int);
        let mut length: size_t = if !slash.is_null() {
            slash.offset_from(part) as size_t
        } else {
            strlen(part)
        };
        let mut key: [::core::ffi::c_char; 2048] = [0; 2048];
        if decode_token(
            part,
            length,
            &raw mut key as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        ) != 0 as ::core::ffi::c_int
        {
            return ::core::ptr::null::<cJSON>();
        }
        if cJSON_IsObject(node) != 0 {
            node = item(node, &raw mut key as *mut ::core::ffi::c_char);
        } else {
            return ::core::ptr::null::<cJSON>();
        }
        if node.is_null() {
            return ::core::ptr::null::<cJSON>();
        }
        if slash.is_null() {
            return node;
        }
        part = slash.offset(1 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn component_category(mut name: *const ::core::ffi::c_char) -> bool {
    let mut categories: [*const ::core::ffi::c_char; 8] = [
        b"schemas\0".as_ptr() as *const ::core::ffi::c_char,
        b"responses\0".as_ptr() as *const ::core::ffi::c_char,
        b"parameters\0".as_ptr() as *const ::core::ffi::c_char,
        b"examples\0".as_ptr() as *const ::core::ffi::c_char,
        b"requestBodies\0".as_ptr() as *const ::core::ffi::c_char,
        b"headers\0".as_ptr() as *const ::core::ffi::c_char,
        b"securitySchemes\0".as_ptr() as *const ::core::ffi::c_char,
        b"links\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 8]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if strcmp(categories[i], name) == 0 {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn mark_component(
    mut p: *mut projection_t,
    mut category: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut definitions: *mut cJSON = item(
        item(
            (*p).document,
            b"components\0".as_ptr() as *const ::core::ffi::c_char,
        ),
        category,
    );
    if !component_category(category)
        || cJSON_IsObject(definitions) == 0
        || item(definitions, name).is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    if !(*p).collecting {
        return 0 as ::core::ffi::c_int;
    }
    let mut marks: *mut cJSON = item((*p).marks, category);
    if marks.is_null() && {
        marks = cJSON_AddObjectToObject((*p).marks, category);
        marks.is_null()
    } {
        return -1 as ::core::ffi::c_int;
    }
    if !item(marks, name).is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_AddBoolToObject(marks, name, r#true).is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut entry: *mut cJSON = cJSON_CreateObject();
    if entry.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if cJSON_AddStringToObject(
        entry,
        b"category\0".as_ptr() as *const ::core::ffi::c_char,
        category,
    )
    .is_null()
        || cJSON_AddStringToObject(
            entry,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        )
        .is_null()
        || cJSON_AddItemToArray((*p).queue, entry) == 0
    {
        cJSON_Delete(entry);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn reference(
    mut p: *mut projection_t,
    mut value: *const cJSON,
) -> ::core::ffi::c_int {
    if cJSON_IsString(value) == 0 || resolve_pointer((*p).canonical, (*value).valuestring).is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut r#ref: *const ::core::ffi::c_char = (*value).valuestring;
    if strncmp(
        r#ref,
        b"#/components/\0".as_ptr() as *const ::core::ffi::c_char,
        13 as size_t,
    ) != 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut category_start: *const ::core::ffi::c_char =
        r#ref.offset(13 as ::core::ffi::c_int as isize);
    let mut slash: *const ::core::ffi::c_char = strchr(category_start, '/' as ::core::ffi::c_int);
    if slash.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut name_start: *const ::core::ffi::c_char = slash.offset(1 as ::core::ffi::c_int as isize);
    let mut end: *const ::core::ffi::c_char = strchr(name_start, '/' as ::core::ffi::c_int);
    let mut category: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut name: [::core::ffi::c_char; 2048] = [0; 2048];
    if decode_token(
        category_start,
        slash.offset_from(category_start) as size_t,
        &raw mut category as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
    ) != 0
        || decode_token(
            name_start,
            if !end.is_null() {
                end.offset_from(name_start) as size_t
            } else {
                strlen(name_start)
            },
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        ) != 0
    {
        return -1 as ::core::ffi::c_int;
    }
    return mark_component(
        p,
        &raw mut category as *mut ::core::ffi::c_char,
        &raw mut name as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn member(
    mut strings: *const cJSON,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut n: *const cJSON = if !strings.is_null() {
        (*strings).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !n.is_null() {
        if string_is(n, value) {
            return r#true != 0;
        }
        n = (*n).next;
    }
    return r#false != 0;
}
unsafe extern "C" fn add_string(
    mut array: *mut cJSON,
    mut value: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if member(array, value) {
        return 0 as ::core::ffi::c_int;
    }
    let mut string: *mut cJSON = cJSON_CreateString(value);
    if string.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if cJSON_AddItemToArray(array, string) == 0 {
        cJSON_Delete(string);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn erase(mut parent: *mut cJSON, mut child: *mut cJSON) {
    cJSON_Delete(cJSON_DetachItemViaPointer(parent, child));
}
unsafe extern "C" fn literal_schema_keyword(mut key: *const ::core::ffi::c_char) -> bool {
    let mut keys: [*const ::core::ffi::c_char; 32] = [
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        b"enum\0".as_ptr() as *const ::core::ffi::c_char,
        b"const\0".as_ptr() as *const ::core::ffi::c_char,
        b"default\0".as_ptr() as *const ::core::ffi::c_char,
        b"example\0".as_ptr() as *const ::core::ffi::c_char,
        b"examples\0".as_ptr() as *const ::core::ffi::c_char,
        b"title\0".as_ptr() as *const ::core::ffi::c_char,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        b"$comment\0".as_ptr() as *const ::core::ffi::c_char,
        b"format\0".as_ptr() as *const ::core::ffi::c_char,
        b"pattern\0".as_ptr() as *const ::core::ffi::c_char,
        b"minLength\0".as_ptr() as *const ::core::ffi::c_char,
        b"maxLength\0".as_ptr() as *const ::core::ffi::c_char,
        b"minimum\0".as_ptr() as *const ::core::ffi::c_char,
        b"maximum\0".as_ptr() as *const ::core::ffi::c_char,
        b"exclusiveMinimum\0".as_ptr() as *const ::core::ffi::c_char,
        b"exclusiveMaximum\0".as_ptr() as *const ::core::ffi::c_char,
        b"multipleOf\0".as_ptr() as *const ::core::ffi::c_char,
        b"minItems\0".as_ptr() as *const ::core::ffi::c_char,
        b"maxItems\0".as_ptr() as *const ::core::ffi::c_char,
        b"uniqueItems\0".as_ptr() as *const ::core::ffi::c_char,
        b"minContains\0".as_ptr() as *const ::core::ffi::c_char,
        b"maxContains\0".as_ptr() as *const ::core::ffi::c_char,
        b"minProperties\0".as_ptr() as *const ::core::ffi::c_char,
        b"maxProperties\0".as_ptr() as *const ::core::ffi::c_char,
        b"required\0".as_ptr() as *const ::core::ffi::c_char,
        b"dependentRequired\0".as_ptr() as *const ::core::ffi::c_char,
        b"readOnly\0".as_ptr() as *const ::core::ffi::c_char,
        b"writeOnly\0".as_ptr() as *const ::core::ffi::c_char,
        b"deprecated\0".as_ptr() as *const ::core::ffi::c_char,
        b"contentEncoding\0".as_ptr() as *const ::core::ffi::c_char,
        b"contentMediaType\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 32]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if strcmp(key, keys[i]) == 0 {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return strncmp(
        key,
        b"x-\0".as_ptr() as *const ::core::ffi::c_char,
        2 as size_t,
    ) == 0;
}
unsafe extern "C" fn schema_map(
    mut p: *mut projection_t,
    mut map: *mut cJSON,
    mut depth: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if cJSON_IsObject(map) == 0 || !unique_keys(map) {
        return -1 as ::core::ffi::c_int;
    }
    let mut entry: *mut cJSON = (*map).child;
    while !entry.is_null() {
        if walk_schema(
            p,
            entry,
            depth.wrapping_add(1 as ::core::ffi::c_uint),
            r#false != 0,
        ) != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        entry = (*entry).next as *mut cJSON;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn discriminator(
    mut p: *mut projection_t,
    mut schema: *mut cJSON,
) -> ::core::ffi::c_int {
    let mut disc: *mut cJSON = item(
        schema,
        b"discriminator\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if disc.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsObject(disc) == 0
        || !unique_keys(disc)
        || cJSON_IsString(item(
            disc,
            b"propertyName\0".as_ptr() as *const ::core::ffi::c_char,
        )) == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut field: *mut cJSON = (*disc).child;
    while !field.is_null() {
        if strcmp((*field).string, AUTH.as_ptr()) == 0
            || strcmp((*field).string, VISIBILITY.as_ptr()) == 0
        {
            return -1 as ::core::ffi::c_int;
        }
        if strcmp(
            (*field).string,
            b"propertyName\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
            && strcmp(
                (*field).string,
                b"mapping\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0
            && strncmp(
                (*field).string,
                b"x-\0".as_ptr() as *const ::core::ffi::c_char,
                2 as size_t,
            ) != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        field = (*field).next as *mut cJSON;
    }
    let mut mapping: *mut cJSON = item(disc, b"mapping\0".as_ptr() as *const ::core::ffi::c_char);
    if mapping.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsObject(mapping) == 0 || !unique_keys(mapping) {
        return -1 as ::core::ffi::c_int;
    }
    let mut choices: *mut cJSON = item(schema, b"oneOf\0".as_ptr() as *const ::core::ffi::c_char);
    if choices.is_null() {
        choices = item(schema, b"anyOf\0".as_ptr() as *const ::core::ffi::c_char);
    }
    let mut m: *mut cJSON = (*mapping).child;
    let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    while !m.is_null() {
        next = (*m).next as *mut cJSON;
        if cJSON_IsString(m) == 0 {
            return -1 as ::core::ffi::c_int;
        }
        let mut target: *const cJSON = ::core::ptr::null::<cJSON>();
        if strncmp(
            (*m).valuestring,
            b"#/\0".as_ptr() as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0
        {
            target = resolve_pointer((*p).canonical, (*m).valuestring);
        } else {
            target = item(
                item(
                    item(
                        (*p).canonical,
                        b"components\0".as_ptr() as *const ::core::ffi::c_char,
                    ),
                    b"schemas\0".as_ptr() as *const ::core::ffi::c_char,
                ),
                (*m).valuestring,
            );
        }
        if target.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        's_74: {
            if (*p).filtering as ::core::ffi::c_int != 0 && !choices.is_null() {
                let mut found: bool = r#false != 0;
                let mut choice: *mut cJSON = (*choices).child;
                while !choice.is_null() {
                    let mut r#ref: *mut cJSON =
                        item(choice, b"$ref\0".as_ptr() as *const ::core::ffi::c_char);
                    if cJSON_IsString(r#ref) != 0
                        && resolve_pointer((*p).canonical, (*r#ref).valuestring) == target
                    {
                        found = r#true != 0;
                    }
                    choice = (*choice).next as *mut cJSON;
                }
                if !found {
                    erase(mapping, m);
                    break 's_74;
                }
            }
            if strncmp(
                (*m).valuestring,
                b"#/\0".as_ptr() as *const ::core::ffi::c_char,
                2 as size_t,
            ) == 0
            {
                if reference(p, m) != 0 {
                    return -1 as ::core::ffi::c_int;
                }
            } else if mark_component(
                p,
                b"schemas\0".as_ptr() as *const ::core::ffi::c_char,
                (*m).valuestring,
            ) != 0
            {
                return -1 as ::core::ffi::c_int;
            }
        }
        m = next;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn walk_schema(
    mut p: *mut projection_t,
    mut node: *mut cJSON,
    mut depth: ::core::ffi::c_uint,
    mut alternative: bool,
) -> ::core::ffi::c_int {
    if depth > MAX_DEPTH as ::core::ffi::c_uint {
        return -1 as ::core::ffi::c_int;
    }
    if cJSON_IsBool(node) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsObject(node) == 0 || !unique_keys(node) {
        return -1 as ::core::ffi::c_int;
    }
    let mut visibility: *mut cJSON = item(node, VISIBILITY.as_ptr());
    if !visibility.is_null() && (!alternative || predicate_valid(visibility) != 0) {
        return -1 as ::core::ffi::c_int;
    }
    if !item(node, AUTH.as_ptr()).is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut child: *mut cJSON = (*node).child;
    let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    while !child.is_null() {
        next = (*child).next as *mut cJSON;
        let mut key: *const ::core::ffi::c_char = (*child).string;
        if strcmp(key, VISIBILITY.as_ptr()) == 0 {
            if (*p).filtering {
                erase(node, child);
            }
        } else if strcmp(key, b"$ref\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            if reference(p, child) != 0 {
                return -1 as ::core::ffi::c_int;
            }
        } else if strcmp(key, b"$id\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"$anchor\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"$dynamicRef\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(
                key,
                b"$dynamicAnchor\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                key,
                b"$recursiveRef\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                key,
                b"$recursiveAnchor\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
        {
            return -1 as ::core::ffi::c_int;
        } else if strcmp(key, b"properties\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(
                key,
                b"patternProperties\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(key, b"$defs\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"definitions\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(
                key,
                b"dependentSchemas\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
        {
            if schema_map(p, child, depth.wrapping_add(1 as ::core::ffi::c_uint)) != 0 {
                return -1 as ::core::ffi::c_int;
            }
        } else if strcmp(key, b"anyOf\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"oneOf\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"allOf\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"prefixItems\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            let mut variants: bool = strcmp(key, b"anyOf\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
                || strcmp(key, b"oneOf\0".as_ptr() as *const ::core::ffi::c_char) == 0;
            if cJSON_IsArray(child) == 0 || (*child).child.is_null() {
                return -1 as ::core::ffi::c_int;
            }
            let mut branch: *mut cJSON = (*child).child;
            let mut after: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
            while !branch.is_null() {
                after = (*branch).next as *mut cJSON;
                let mut predicate: *mut cJSON = item(branch, VISIBILITY.as_ptr());
                if !predicate.is_null() && (!variants || predicate_valid(predicate) != 0) {
                    return -1 as ::core::ffi::c_int;
                }
                's_115: {
                    if !predicate.is_null() && (*p).filtering as ::core::ffi::c_int != 0 {
                        let mut allowed: ::core::ffi::c_int = (*p)
                            .access
                            .expect("non-null function pointer")(
                            (*p).access_context, predicate
                        );
                        if allowed != 0 as ::core::ffi::c_int && allowed != 1 as ::core::ffi::c_int
                        {
                            return -1 as ::core::ffi::c_int;
                        }
                        if allowed == 0 {
                            erase(child, branch);
                            break 's_115;
                        }
                    }
                    if walk_schema(
                        p,
                        branch,
                        depth.wrapping_add(1 as ::core::ffi::c_uint),
                        variants,
                    ) != 0
                    {
                        return -1 as ::core::ffi::c_int;
                    }
                }
                branch = after;
            }
            if (*child).child.is_null() {
                return -1 as ::core::ffi::c_int;
            }
        } else if strcmp(key, b"items\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"contains\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(
                key,
                b"additionalProperties\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                key,
                b"unevaluatedProperties\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                key,
                b"unevaluatedItems\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(
                key,
                b"propertyNames\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(key, b"not\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"if\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"then\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"else\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(
                key,
                b"contentSchema\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
        {
            if walk_schema(
                p,
                child,
                depth.wrapping_add(1 as ::core::ffi::c_uint),
                r#false != 0,
            ) != 0
            {
                return -1 as ::core::ffi::c_int;
            }
        } else if strcmp(
            key,
            b"discriminator\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
            if strcmp(key, b"xml\0".as_ptr() as *const ::core::ffi::c_char) == 0
                || strcmp(
                    key,
                    b"externalDocs\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0
            {
                if walk_object(
                    p,
                    child,
                    depth.wrapping_add(1 as ::core::ffi::c_uint),
                    r#false != 0,
                ) != 0
                {
                    return -1 as ::core::ffi::c_int;
                }
            } else if strcmp(key, b"$schema\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
                if !string_is(
                    child,
                    b"https://json-schema.org/draft/2020-12/schema\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ) && !string_is(
                    child,
                    b"https://spec.openapis.org/oas/3.1/dialect/base\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ) {
                    return -1 as ::core::ffi::c_int;
                }
            } else if !literal_schema_keyword(key) {
                return -1 as ::core::ffi::c_int;
            }
        }
        child = next;
    }
    return discriminator(p, node);
}
unsafe extern "C" fn security(
    mut p: *mut projection_t,
    mut value: *mut cJSON,
) -> ::core::ffi::c_int {
    if cJSON_IsArray(value) == 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut requirement: *mut cJSON = (*value).child;
    while !requirement.is_null() {
        if cJSON_IsObject(requirement) == 0 || !unique_keys(requirement) {
            return -1 as ::core::ffi::c_int;
        }
        let mut scheme: *mut cJSON = (*requirement).child;
        while !scheme.is_null() {
            if cJSON_IsArray(scheme) == 0
                || mark_component(
                    p,
                    b"securitySchemes\0".as_ptr() as *const ::core::ffi::c_char,
                    (*scheme).string,
                ) != 0
            {
                return -1 as ::core::ffi::c_int;
            }
            let mut scope: *mut cJSON = (*scheme).child;
            while !scope.is_null() {
                if cJSON_IsString(scope) == 0 {
                    return -1 as ::core::ffi::c_int;
                }
                scope = (*scope).next as *mut cJSON;
            }
            scheme = (*scheme).next as *mut cJSON;
        }
        requirement = (*requirement).next as *mut cJSON;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn link_allowed(
    mut p: *mut projection_t,
    mut link: *const cJSON,
    mut depth: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if depth > MAX_DEPTH as ::core::ffi::c_uint || cJSON_IsObject(link) == 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut r#ref: *mut cJSON = item(link, b"$ref\0".as_ptr() as *const ::core::ffi::c_char);
    if !r#ref.is_null() {
        if cJSON_IsString(r#ref) == 0 {
            return -1 as ::core::ffi::c_int;
        }
        let mut resolved: *const cJSON = resolve_pointer((*p).canonical, (*r#ref).valuestring);
        if resolved.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        return link_allowed(p, resolved, depth.wrapping_add(1 as ::core::ffi::c_uint));
    }
    let mut id: *mut cJSON = item(
        link,
        b"operationId\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut operation_ref: *mut cJSON = item(
        link,
        b"operationRef\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !id.is_null() && !operation_ref.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if !operation_ref.is_null() {
        if cJSON_IsString(operation_ref) == 0
            || strncmp(
                (*operation_ref).valuestring,
                b"#/paths/\0".as_ptr() as *const ::core::ffi::c_char,
                8 as size_t,
            ) != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut operation: *const cJSON =
            resolve_pointer((*p).canonical, (*operation_ref).valuestring);
        if operation.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        id = item(
            operation,
            b"operationId\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if cJSON_IsString(id) == 0 {
        return -1 as ::core::ffi::c_int;
    }
    if member((*p).operations, (*id).valuestring) {
        return 1 as ::core::ffi::c_int;
    }
    return if (*p).filtering as ::core::ffi::c_int != 0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn object_map(
    mut p: *mut projection_t,
    mut map: *mut cJSON,
    mut depth: ::core::ffi::c_uint,
    mut links: bool,
) -> ::core::ffi::c_int {
    if cJSON_IsObject(map) == 0 || !unique_keys(map) {
        return -1 as ::core::ffi::c_int;
    }
    let mut entry: *mut cJSON = (*map).child;
    let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    while !entry.is_null() {
        next = (*entry).next as *mut cJSON;
        's_11: {
            if links {
                let mut allowed: ::core::ffi::c_int =
                    link_allowed(p, entry, 0 as ::core::ffi::c_uint);
                if allowed < 0 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                if allowed == 0 {
                    erase(map, entry);
                    break 's_11;
                }
            }
            if walk_object(
                p,
                entry,
                depth.wrapping_add(1 as ::core::ffi::c_uint),
                r#false != 0,
            ) != 0
            {
                return -1 as ::core::ffi::c_int;
            }
        }
        entry = next;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn walk_object(
    mut p: *mut projection_t,
    mut node: *mut cJSON,
    mut depth: ::core::ffi::c_uint,
    mut operation: bool,
) -> ::core::ffi::c_int {
    if depth > MAX_DEPTH as ::core::ffi::c_uint {
        return -1 as ::core::ffi::c_int;
    }
    if cJSON_IsArray(node) != 0 {
        let mut child: *mut cJSON = (*node).child;
        while !child.is_null() {
            if walk_object(
                p,
                child,
                depth.wrapping_add(1 as ::core::ffi::c_uint),
                r#false != 0,
            ) != 0
            {
                return -1 as ::core::ffi::c_int;
            }
            child = (*child).next as *mut cJSON;
        }
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsObject(node) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !unique_keys(node) {
        return -1 as ::core::ffi::c_int;
    }
    let mut child_0: *mut cJSON = (*node).child;
    let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    while !child_0.is_null() {
        next = (*child_0).next as *mut cJSON;
        let mut key: *const ::core::ffi::c_char = (*child_0).string;
        if strcmp(key, AUTH.as_ptr()) == 0 {
            if !operation || predicate_valid(child_0) != 0 {
                return -1 as ::core::ffi::c_int;
            }
            if (*p).filtering {
                erase(node, child_0);
            }
        } else if strcmp(key, VISIBILITY.as_ptr()) == 0
            || strcmp(key, b"callbacks\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"webhooks\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(
                key,
                b"externalValue\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
        {
            return -1 as ::core::ffi::c_int;
        } else if strcmp(key, b"$ref\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            if reference(p, child_0) != 0 {
                return -1 as ::core::ffi::c_int;
            }
        } else if strcmp(
            key,
            b"operationRef\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            if cJSON_IsString(child_0) == 0
                || resolve_pointer((*p).canonical, (*child_0).valuestring).is_null()
            {
                return -1 as ::core::ffi::c_int;
            }
        } else if strcmp(key, b"schema\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            if walk_schema(
                p,
                child_0,
                depth.wrapping_add(1 as ::core::ffi::c_uint),
                r#false != 0,
            ) != 0
            {
                return -1 as ::core::ffi::c_int;
            }
        } else if strcmp(key, b"security\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            if security(p, child_0) != 0 {
                return -1 as ::core::ffi::c_int;
            }
        } else if (strcmp(key, b"parameters\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"requestBody\0".as_ptr() as *const ::core::ffi::c_char) == 0)
            && (!item(
                node,
                b"operationId\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .is_null()
                || !item(
                    node,
                    b"operationRef\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null())
            && !operation
        {
            if strcmp(key, b"parameters\0".as_ptr() as *const ::core::ffi::c_char) == 0
                && (cJSON_IsObject(child_0) == 0 || !unique_keys(child_0))
            {
                return -1 as ::core::ffi::c_int;
            }
        } else if strcmp(key, b"content\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"responses\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"encoding\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"variables\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"headers\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"examples\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"links\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            if object_map(
                p,
                child_0,
                depth.wrapping_add(1 as ::core::ffi::c_uint),
                strcmp(key, b"links\0".as_ptr() as *const ::core::ffi::c_char) == 0,
            ) != 0
            {
                return -1 as ::core::ffi::c_int;
            }
        } else if !(strcmp(key, b"example\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"value\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"default\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strncmp(
                key,
                b"x-\0".as_ptr() as *const ::core::ffi::c_char,
                2 as size_t,
            ) == 0)
        {
            if walk_object(
                p,
                child_0,
                depth.wrapping_add(1 as ::core::ffi::c_uint),
                r#false != 0,
            ) != 0
            {
                return -1 as ::core::ffi::c_int;
            }
        }
        child_0 = next;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn paths(mut p: *mut projection_t, mut select: bool) -> ::core::ffi::c_int {
    let mut map: *mut cJSON = item(
        (*p).document,
        b"paths\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsObject(map) == 0 || !unique_keys(map) {
        return -1 as ::core::ffi::c_int;
    }
    let mut path: *mut cJSON = (*map).child;
    let mut next_path: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    while !path.is_null() {
        next_path = (*path).next as *mut cJSON;
        if cJSON_IsObject(path) == 0
            || !unique_keys(path)
            || *(*path).string.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut methods: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut op: *mut cJSON = (*path).child;
        let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
        while !op.is_null() {
            next = (*op).next as *mut cJSON;
            's_33: {
                if !is_method((*op).string) {
                    if strcmp(
                        (*op).string,
                        b"$ref\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0
                        || strcmp((*op).string, AUTH.as_ptr()) == 0
                        || strcmp((*op).string, VISIBILITY.as_ptr()) == 0
                    {
                        return -1 as ::core::ffi::c_int;
                    }
                    if walk_object(p, op, 1 as ::core::ffi::c_uint, r#false != 0) != 0 {
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    if cJSON_IsObject(op) == 0 {
                        return -1 as ::core::ffi::c_int;
                    }
                    let mut id: *mut cJSON =
                        item(op, b"operationId\0".as_ptr() as *const ::core::ffi::c_char);
                    let mut predicate: *mut cJSON = item(op, AUTH.as_ptr());
                    if cJSON_IsString(id) == 0 || *(*id).valuestring.offset(0isize) == 0 {
                        return -1 as ::core::ffi::c_int;
                    }
                    if select {
                        if predicate_valid(predicate) != 0 {
                            return -1 as ::core::ffi::c_int;
                        }
                        let mut allowed: ::core::ffi::c_int = (*p)
                            .access
                            .expect("non-null function pointer")(
                            (*p).access_context, predicate
                        );
                        if allowed != 0 as ::core::ffi::c_int && allowed != 1 as ::core::ffi::c_int
                        {
                            return -1 as ::core::ffi::c_int;
                        }
                        if allowed == 0 {
                            erase(path, op);
                            break 's_33;
                        } else if add_string((*p).operations, (*id).valuestring) != 0 {
                            return -1 as ::core::ffi::c_int;
                        }
                    } else if walk_object(p, op, 1 as ::core::ffi::c_uint, r#true != 0) != 0 {
                        return -1 as ::core::ffi::c_int;
                    }
                    methods += 1;
                }
            }
            op = next;
        }
        if select as ::core::ffi::c_int != 0 && methods == 0 as ::core::ffi::c_int {
            erase(map, path);
        }
        path = next_path;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn root_fields(mut p: *mut projection_t) -> ::core::ffi::c_int {
    let mut field: *mut cJSON = (*(*p).document).child;
    while !field.is_null() {
        if !(strcmp(
            (*field).string,
            b"components\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
            || strcmp(
                (*field).string,
                b"paths\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0)
        {
            if strcmp(
                (*field).string,
                b"webhooks\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
                || strcmp((*field).string, AUTH.as_ptr()) == 0
                || strcmp((*field).string, VISIBILITY.as_ptr()) == 0
            {
                return -1 as ::core::ffi::c_int;
            }
            if strcmp(
                (*field).string,
                b"jsonSchemaDialect\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
                && !string_is(
                    field,
                    b"https://json-schema.org/draft/2020-12/schema\0".as_ptr()
                        as *const ::core::ffi::c_char,
                )
                && !string_is(
                    field,
                    b"https://spec.openapis.org/oas/3.1/dialect/base\0".as_ptr()
                        as *const ::core::ffi::c_char,
                )
            {
                return -1 as ::core::ffi::c_int;
            }
            if strcmp(
                (*field).string,
                b"security\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            {
                if security(p, field) != 0 {
                    return -1 as ::core::ffi::c_int;
                }
            } else if strncmp(
                (*field).string,
                b"x-\0".as_ptr() as *const ::core::ffi::c_char,
                2 as size_t,
            ) != 0
            {
                if walk_object(p, field, 1 as ::core::ffi::c_uint, r#false != 0) != 0 {
                    return -1 as ::core::ffi::c_int;
                }
            }
        }
        field = (*field).next as *mut cJSON;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn components(mut p: *mut projection_t) -> ::core::ffi::c_int {
    let mut all: *mut cJSON = item(
        (*p).document,
        b"components\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if all.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsObject(all) == 0 || !unique_keys(all) {
        return -1 as ::core::ffi::c_int;
    }
    let mut category: *mut cJSON = (*all).child;
    while !category.is_null() {
        if !component_category((*category).string)
            || cJSON_IsObject(category) == 0
            || !unique_keys(category)
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut definition: *mut cJSON = (*category).child;
        while !definition.is_null() {
            let mut error: ::core::ffi::c_int = if strcmp(
                (*category).string,
                b"schemas\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            {
                walk_schema(p, definition, 1 as ::core::ffi::c_uint, r#false != 0)
            } else {
                walk_object(p, definition, 1 as ::core::ffi::c_uint, r#false != 0)
            };
            if error != 0 {
                return -1 as ::core::ffi::c_int;
            }
            definition = (*definition).next as *mut cJSON;
        }
        category = (*category).next as *mut cJSON;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapi_project(
    mut canonical: *const cJSON,
    mut access: openapi_access_fn,
    mut context: *mut ::core::ffi::c_void,
    mut result: *mut *mut cJSON,
) -> ::core::ffi::c_int {
    let mut path_map: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut all: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut tags: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if result.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *result = ::core::ptr::null_mut::<cJSON>();
    if cJSON_IsObject(canonical) == 0
        || !unique_keys(canonical)
        || access.is_none()
        || !string_is(
            item(
                canonical,
                b"openapi\0".as_ptr() as *const ::core::ffi::c_char,
            ),
            b"3.1.1\0".as_ptr() as *const ::core::ffi::c_char,
        )
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut p: projection_t = projection_t {
        document: ::core::ptr::null_mut::<cJSON>(),
        canonical: canonical,
        access: access,
        access_context: context,
        operations: ::core::ptr::null_mut::<cJSON>(),
        marks: ::core::ptr::null_mut::<cJSON>(),
        queue: ::core::ptr::null_mut::<cJSON>(),
        filtering: false,
        collecting: false,
    };
    p.document = cJSON_Duplicate(canonical, r#true);
    p.operations = cJSON_CreateArray();
    p.marks = cJSON_CreateObject();
    p.queue = cJSON_CreateArray();
    let mut status: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    '_done: {
        if !(p.document.is_null()
            || p.operations.is_null()
            || p.marks.is_null()
            || p.queue.is_null())
        {
            path_map = item(
                p.document,
                b"paths\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if cJSON_IsObject(path_map) != 0 {
                let mut path: *mut cJSON = (*path_map).child;
                while !path.is_null() {
                    let mut op: *mut cJSON = (*path).child;
                    while !op.is_null() {
                        if !((*op).string.is_null() || !is_method((*op).string)) {
                            let mut id: *mut cJSON =
                                item(op, b"operationId\0".as_ptr() as *const ::core::ffi::c_char);
                            if cJSON_IsString(id) == 0
                                || member(p.operations, (*id).valuestring) as ::core::ffi::c_int
                                    != 0
                                || predicate_valid(item(op, AUTH.as_ptr())) != 0
                                || add_string(p.operations, (*id).valuestring) != 0
                            {
                                break '_done;
                            }
                        }
                        op = (*op).next as *mut cJSON;
                    }
                    path = (*path).next as *mut cJSON;
                }
                if !(paths(&raw mut p, r#false != 0) != 0
                    || root_fields(&raw mut p) != 0
                    || components(&raw mut p) != 0)
                {
                    cJSON_Delete(p.operations);
                    p.operations = cJSON_CreateArray();
                    if !p.operations.is_null() {
                        p.filtering = r#true != 0;
                        if paths(&raw mut p, r#true != 0) == 0 {
                            p.collecting = r#true != 0;
                            if !(paths(&raw mut p, r#false != 0) != 0
                                || root_fields(&raw mut p) != 0)
                            {
                                let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while index < cJSON_GetArraySize(p.queue) {
                                    let mut entry: *mut cJSON = cJSON_GetArrayItem(p.queue, index);
                                    let mut category: *const ::core::ffi::c_char = (*item(
                                        entry,
                                        b"category\0".as_ptr() as *const ::core::ffi::c_char,
                                    ))
                                    .valuestring;
                                    let mut definition: *mut cJSON = item(
                                        item(
                                            item(
                                                p.document,
                                                b"components\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            ),
                                            category,
                                        ),
                                        (*item(
                                            entry,
                                            b"name\0".as_ptr() as *const ::core::ffi::c_char,
                                        ))
                                        .valuestring,
                                    );
                                    if definition.is_null() {
                                        break '_done;
                                    }
                                    if strcmp(
                                        category,
                                        b"schemas\0".as_ptr() as *const ::core::ffi::c_char,
                                    ) == 0
                                    {
                                        if walk_schema(
                                            &raw mut p,
                                            definition,
                                            1 as ::core::ffi::c_uint,
                                            r#false != 0,
                                        ) != 0
                                        {
                                            break '_done;
                                        }
                                    } else if walk_object(
                                        &raw mut p,
                                        definition,
                                        1 as ::core::ffi::c_uint,
                                        r#false != 0,
                                    ) != 0
                                    {
                                        break '_done;
                                    }
                                    index += 1;
                                }
                                all = item(
                                    p.document,
                                    b"components\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                let mut category_0: *mut cJSON = if !all.is_null() {
                                    (*all).child
                                } else {
                                    ::core::ptr::null_mut::<cJSON>()
                                };
                                let mut next_category: *mut cJSON =
                                    ::core::ptr::null_mut::<cJSON>();
                                while !category_0.is_null() {
                                    next_category = (*category_0).next as *mut cJSON;
                                    let mut definition_0: *mut cJSON = (*category_0).child;
                                    let mut next: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
                                    while !definition_0.is_null() {
                                        next = (*definition_0).next as *mut cJSON;
                                        if item(
                                            item(p.marks, (*category_0).string),
                                            (*definition_0).string,
                                        )
                                        .is_null()
                                        {
                                            erase(category_0, definition_0);
                                        }
                                        definition_0 = next;
                                    }
                                    if (*category_0).child.is_null() {
                                        erase(all, category_0);
                                    }
                                    category_0 = next_category;
                                }
                                tags = item(
                                    p.document,
                                    b"tags\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                's_280: {
                                    if !tags.is_null() {
                                        if cJSON_IsArray(tags) == 0 {
                                            break '_done;
                                        } else {
                                            let mut tag: *mut cJSON = (*tags).child;
                                            let mut next_0: *mut cJSON =
                                                ::core::ptr::null_mut::<cJSON>();
                                            loop {
                                                if tag.is_null() {
                                                    break 's_280;
                                                }
                                                next_0 = (*tag).next as *mut cJSON;
                                                let mut name: *mut cJSON = item(
                                                    tag,
                                                    b"name\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                let mut used: bool = r#false != 0;
                                                if cJSON_IsString(name) == 0 {
                                                    break '_done;
                                                }
                                                let mut path_0: *mut cJSON = (*path_map).child;
                                                while !path_0.is_null() && !used {
                                                    let mut op_0: *mut cJSON = (*path_0).child;
                                                    while !op_0.is_null() {
                                                        if is_method((*op_0).string) as ::core::ffi::c_int != 0
                                                            && member(
                                                                item(
                                                                    op_0,
                                                                    b"tags\0".as_ptr() as *const ::core::ffi::c_char,
                                                                ),
                                                                (*name).valuestring,
                                                            ) as ::core::ffi::c_int != 0
                                                        {
                                                            used = r#true != 0;
                                                        }
                                                        op_0 = (*op_0).next as *mut cJSON;
                                                    }
                                                    path_0 = (*path_0).next as *mut cJSON;
                                                }
                                                if !used {
                                                    erase(tags, tag);
                                                }
                                                tag = next_0;
                                            }
                                        }
                                    }
                                }
                                cJSON_DeleteItemFromObjectCaseSensitive(
                                    p.document,
                                    b"x-lightnvr-validators\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                p.canonical = p.document;
                                p.filtering = r#false != 0;
                                p.collecting = r#false != 0;
                                if !(paths(&raw mut p, r#false != 0) != 0
                                    || root_fields(&raw mut p) != 0
                                    || components(&raw mut p) != 0)
                                {
                                    *result = p.document;
                                    p.document = ::core::ptr::null_mut::<cJSON>();
                                    status = 0 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cJSON_Delete(p.document);
    cJSON_Delete(p.operations);
    cJSON_Delete(p.marks);
    cJSON_Delete(p.queue);
    return status;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
