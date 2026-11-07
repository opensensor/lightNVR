unsafe extern "C" {
    pub type sqlite3;
    pub type fleet_selector;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
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
    unsafe fn db_camera_collection_get(
        uuid: *const ::core::ffi::c_char,
        collection: *mut camera_collection_t,
    ) -> db_camera_collection_result_t;
    unsafe fn db_camera_collection_list_members(
        collection_uuid: *const ::core::ffi::c_char,
        camera_uuids: *mut [::core::ffi::c_char; 37],
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_camera_collection_get_on_connection(
        db: *mut sqlite3,
        uuid: *const ::core::ffi::c_char,
        collection: *mut camera_collection_t,
    ) -> db_camera_collection_result_t;
    unsafe fn db_camera_collection_list_members_on_connection(
        db: *mut sqlite3,
        collection_uuid: *const ::core::ffi::c_char,
        camera_uuids: *mut [::core::ffi::c_char; 37],
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
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
pub struct authorization_decision_t(pub ::core::ffi::c_uint);
impl authorization_decision_t {
    pub const AUTHZ_DECISION_DENY: Self = Self(0);
    pub const AUTHZ_DECISION_ALLOW: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_source_t(pub ::core::ffi::c_uint);
impl authorization_decision_source_t {
    pub const AUTHZ_SOURCE_NONE: Self = Self(0);
    pub const AUTHZ_SOURCE_LEGACY_ROLE: Self = Self(1);
    pub const AUTHZ_SOURCE_POLICY_GRANT: Self = Self(2);
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
pub struct camera_collection_filter_result_t(pub ::core::ffi::c_int);
impl camera_collection_filter_result_t {
    pub const CAMERA_COLLECTION_FILTER_OK: Self = Self(0);
    pub const CAMERA_COLLECTION_FILTER_NOT_FOUND: Self = Self(-1);
    pub const CAMERA_COLLECTION_FILTER_INVALID_SELECTOR: Self = Self(-2);
    pub const CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY: Self = Self(-3);
    pub const CAMERA_COLLECTION_FILTER_DATABASE_ERROR: Self = Self(-4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_collection_filter_t {
    pub active: bool,
    pub smart_selector: *mut fleet_selector_t,
    pub member_uuids: *mut [::core::ffi::c_char; 37],
    pub member_count: ::core::ffi::c_int,
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
pub const CAMERA_COLLECTION_MAX_MEMBERS: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn camera_collection_filter_free(
    mut filter: *mut camera_collection_filter_t,
) {
    if filter.is_null() {
        return;
    }
    fleet_selector_free((*filter).smart_selector);
    free((*filter).member_uuids as *mut ::core::ffi::c_void);
    memset(
        filter as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<camera_collection_filter_t>(),
    );
}
unsafe extern "C" fn load_filter(
    mut connection: *mut sqlite3,
    mut collection_uuid: *const ::core::ffi::c_char,
    mut user: *const user_t,
    mut shared_only: bool,
    mut filter: *mut camera_collection_filter_t,
) -> camera_collection_filter_result_t {
    if filter.is_null() || user.is_null() && !shared_only {
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    memset(
        filter as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<camera_collection_filter_t>(),
    );
    if collection_uuid.is_null()
        || *collection_uuid.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OK;
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
    let mut result: db_camera_collection_result_t =
        db_camera_collection_result_t(if !connection.is_null() {
            db_camera_collection_get_on_connection(connection, collection_uuid, &raw mut collection)
                .0
        } else {
            db_camera_collection_get(collection_uuid, &raw mut collection).0
        });
    let mut can_manage: bool = r#false != 0;
    if result.0 == db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0
        && !user.is_null()
        && !shared_only
    {
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
        can_manage = authorization_evaluate(
            user,
            authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
            ::core::ptr::null::<fleet_camera_t>(),
            &raw mut evaluation,
        ) == 0 as ::core::ffi::c_int
            && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
    }
    let mut visible: bool = result.0 == db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0
        && if shared_only as ::core::ffi::c_int != 0 {
            collection.is_shared as ::core::ffi::c_int
        } else {
            (can_manage as ::core::ffi::c_int != 0
                || collection.is_shared as ::core::ffi::c_int != 0
                || collection.owner_user_id > 0 as int64_t
                    && collection.owner_user_id == (*user).id) as ::core::ffi::c_int
        } != 0;
    if !visible {
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_NOT_FOUND;
    }
    (*filter).active = r#true != 0;
    if strcmp(
        &raw mut collection.collection_type as *mut ::core::ffi::c_char,
        b"smart\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut selector_json: *mut cJSON =
            cJSON_Parse(&raw mut collection.selector_json as *mut ::core::ffi::c_char);
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
        (*filter).smart_selector = fleet_selector_parse(
            selector_json,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        );
        cJSON_Delete(selector_json);
        if (*filter).smart_selector.is_null() {
            camera_collection_filter_free(filter);
            return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_INVALID_SELECTOR;
        }
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OK;
    }
    (*filter).member_count = collection.member_count;
    if (*filter).member_count < 0 as ::core::ffi::c_int
        || (*filter).member_count > CAMERA_COLLECTION_MAX_MEMBERS
    {
        camera_collection_filter_free(filter);
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    if (*filter).member_count == 0 as ::core::ffi::c_int {
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OK;
    }
    (*filter).member_uuids = calloc(
        (*filter).member_count as size_t,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    ) as *mut [::core::ffi::c_char; 37];
    if (*filter).member_uuids.is_null() {
        camera_collection_filter_free(filter);
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY;
    }
    let mut count: ::core::ffi::c_int = if !connection.is_null() {
        db_camera_collection_list_members_on_connection(
            connection,
            &raw mut collection.uuid as *mut ::core::ffi::c_char,
            (*filter).member_uuids,
            (*filter).member_count,
        )
    } else {
        db_camera_collection_list_members(
            &raw mut collection.uuid as *mut ::core::ffi::c_char,
            (*filter).member_uuids,
            (*filter).member_count,
        )
    };
    if count < 0 as ::core::ffi::c_int {
        camera_collection_filter_free(filter);
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    (*filter).member_count = count;
    return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn camera_collection_filter_load(
    mut collection_uuid: *const ::core::ffi::c_char,
    mut user: *const user_t,
    mut filter: *mut camera_collection_filter_t,
) -> camera_collection_filter_result_t {
    return load_filter(
        ::core::ptr::null_mut::<sqlite3>(),
        collection_uuid,
        user,
        r#false != 0,
        filter,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn camera_collection_filter_load_for_authorization(
    mut collection_uuid: *const ::core::ffi::c_char,
    mut filter: *mut camera_collection_filter_t,
) -> camera_collection_filter_result_t {
    return load_filter(
        ::core::ptr::null_mut::<sqlite3>(),
        collection_uuid,
        ::core::ptr::null::<user_t>(),
        r#true != 0,
        filter,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn camera_collection_filter_load_on_connection(
    mut connection: *mut sqlite3,
    mut collection_uuid: *const ::core::ffi::c_char,
    mut filter: *mut camera_collection_filter_t,
) -> camera_collection_filter_result_t {
    if connection.is_null() {
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    return load_filter(
        connection,
        collection_uuid,
        ::core::ptr::null::<user_t>(),
        r#true != 0,
        filter,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn camera_collection_filter_matches(
    mut filter: *const camera_collection_filter_t,
    mut camera: *const fleet_camera_t,
) -> bool {
    if filter.is_null() || !(*filter).active {
        return r#true != 0;
    }
    if camera.is_null() {
        return r#false != 0;
    }
    if !(*filter).smart_selector.is_null() {
        return fleet_selector_matches(
            (*filter).smart_selector,
            camera,
            ::core::ptr::null_mut::<fleet_selector_explanation_t>(),
        );
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*filter).member_count {
        if strcasecmp(
            &raw mut *(*filter).member_uuids.offset(i as isize) as *mut ::core::ffi::c_char,
            &raw const (*camera).camera_uuid as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn resolve_stream_names(
    mut collection_uuid: *const ::core::ffi::c_char,
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut shared_authorization_boundary: bool,
    mut stream_names: *mut *mut *mut ::core::ffi::c_char,
    mut stream_count: *mut ::core::ffi::c_int,
) -> camera_collection_filter_result_t {
    if collection_uuid.is_null()
        || *collection_uuid.offset(0isize) == 0
        || user.is_null() && !shared_authorization_boundary
        || stream_names.is_null()
        || stream_count.is_null()
    {
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    *stream_names = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    *stream_count = 0 as ::core::ffi::c_int;
    let mut filter: camera_collection_filter_t = camera_collection_filter_t {
        active: false,
        smart_selector: ::core::ptr::null_mut::<fleet_selector_t>(),
        member_uuids: ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>(),
        member_count: 0,
    };
    let mut result: camera_collection_filter_result_t = camera_collection_filter_result_t(
        if shared_authorization_boundary as ::core::ffi::c_int != 0 {
            camera_collection_filter_load_for_authorization(collection_uuid, &raw mut filter).0
        } else {
            camera_collection_filter_load(collection_uuid, user, &raw mut filter).0
        },
    );
    if result.0 != camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OK.0 {
        return result;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db_fleet_camera_load(&raw mut cameras, &raw mut camera_count) != 0 as ::core::ffi::c_int {
        camera_collection_filter_free(&raw mut filter);
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    let mut names: *mut *mut ::core::ffi::c_char = (if camera_count > 0 as ::core::ffi::c_int {
        calloc(
            camera_count as size_t,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
        )
    } else {
        NULL
    }) as *mut *mut ::core::ffi::c_char;
    if camera_count > 0 as ::core::ffi::c_int && names.is_null() {
        free(cameras as *mut ::core::ffi::c_void);
        camera_collection_filter_free(&raw mut filter);
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY;
    }
    if !shared_authorization_boundary
        && authorization_filter_cameras(user, action, cameras, &raw mut camera_count)
            != 0 as ::core::ffi::c_int
    {
        free(names as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
        camera_collection_filter_free(&raw mut filter);
        return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < camera_count {
        if camera_collection_filter_matches(&raw mut filter, cameras.offset(i as isize)) {
            let mut name_size: size_t =
                strlen(&raw mut (*cameras.offset(i as isize)).name as *mut ::core::ffi::c_char)
                    .wrapping_add(1 as size_t);
            *names.offset(*stream_count as isize) = malloc(name_size) as *mut ::core::ffi::c_char;
            if (*names.offset(*stream_count as isize)).is_null() {
                camera_collection_filter_free_stream_names(names, *stream_count);
                free(cameras as *mut ::core::ffi::c_void);
                camera_collection_filter_free(&raw mut filter);
                return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY;
            }
            memcpy(
                *names.offset(*stream_count as isize) as *mut ::core::ffi::c_void,
                &raw mut (*cameras.offset(i as isize)).name as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                name_size,
            );
            *stream_count += 1;
        }
        i += 1;
    }
    free(cameras as *mut ::core::ffi::c_void);
    camera_collection_filter_free(&raw mut filter);
    *stream_names = names;
    return camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn camera_collection_filter_resolve_stream_names(
    mut collection_uuid: *const ::core::ffi::c_char,
    mut user: *const user_t,
    mut stream_names: *mut *mut *mut ::core::ffi::c_char,
    mut stream_count: *mut ::core::ffi::c_int,
) -> camera_collection_filter_result_t {
    return resolve_stream_names(
        collection_uuid,
        user,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        r#false != 0,
        stream_names,
        stream_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn camera_collection_filter_resolve_stream_names_for_action(
    mut collection_uuid: *const ::core::ffi::c_char,
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut stream_names: *mut *mut *mut ::core::ffi::c_char,
    mut stream_count: *mut ::core::ffi::c_int,
) -> camera_collection_filter_result_t {
    return resolve_stream_names(
        collection_uuid,
        user,
        action,
        r#false != 0,
        stream_names,
        stream_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn camera_collection_filter_resolve_stream_names_for_authorization(
    mut collection_uuid: *const ::core::ffi::c_char,
    mut stream_names: *mut *mut *mut ::core::ffi::c_char,
    mut stream_count: *mut ::core::ffi::c_int,
) -> camera_collection_filter_result_t {
    return resolve_stream_names(
        collection_uuid,
        ::core::ptr::null::<user_t>(),
        authorization_action_t::AUTHZ_ACTION_INVALID,
        r#true != 0,
        stream_names,
        stream_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn camera_collection_filter_free_stream_names(
    mut stream_names: *mut *mut ::core::ffi::c_char,
    mut stream_count: ::core::ffi::c_int,
) {
    if stream_names.is_null() {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < stream_count {
        free(*stream_names.offset(i as isize) as *mut ::core::ffi::c_void);
        i += 1;
    }
    free(stream_names as *mut ::core::ffi::c_void);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
