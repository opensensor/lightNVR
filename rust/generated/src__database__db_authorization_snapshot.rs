unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type authorization_context;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_busy_timeout(_: *mut sqlite3, ms: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int)
    -> sqlite3_int64;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn authorization_context_create_on_connection(
        connection: *mut sqlite3,
        observed_at: int64_t,
    ) -> *mut authorization_context_t;
    unsafe fn authorization_context_free(context: *mut authorization_context_t);
    unsafe fn authorization_evaluate_in_context(
        context: *mut authorization_context_t,
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_api_token_get_active_on_connection(
        db: *mut sqlite3,
        observed_at: int64_t,
        token_uuid: *const ::core::ffi::c_char,
        token: *mut api_token_t,
    ) -> db_api_token_result_t;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
    unsafe fn db_fleet_camera_load_on_connection(
        db: *mut sqlite3,
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_camera_capture_runtime_health(
        cameras: *mut fleet_camera_t,
        count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_health_capture_go2rtc_activity(
        cameras: *mut fleet_camera_t,
        camera_count: ::core::ffi::c_int,
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
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint64_t = u64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
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
pub type authorization_context_t = authorization_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_snapshot {
    pub connection: *mut sqlite3,
    pub context: *mut authorization_context_t,
    pub principal: user_t,
    pub inventory: *mut fleet_camera_t,
    pub observations: *mut camera_observations_t,
    pub count: ::core::ffi::c_int,
    pub observed_at: int64_t,
    pub runtime_started: timespec,
    pub runtime_finished: timespec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_observations_t {
    pub raw: camera_health_observation_t,
    pub runtime: camera_health_observation_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_health_observation_t {
    pub health: fleet_health_state_t,
    pub availability: fleet_availability_state_t,
    pub health_changed_at: int64_t,
    pub first_video_at: int64_t,
    pub last_frame_ts: int64_t,
    pub current_fps: ::core::ffi::c_double,
    pub recording_active: bool,
}
pub type authorization_snapshot_t = authorization_snapshot;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_api_token_result_t(pub ::core::ffi::c_int);
impl db_api_token_result_t {
    pub const DB_API_TOKEN_OK: Self = Self(0);
    pub const DB_API_TOKEN_ERROR: Self = Self(-1);
    pub const DB_API_TOKEN_NOT_FOUND: Self = Self(-2);
    pub const DB_API_TOKEN_INVALID: Self = Self(-3);
    pub const DB_API_TOKEN_LIMIT: Self = Self(-4);
    pub const DB_API_TOKEN_EXPIRED: Self = Self(-5);
    pub const DB_API_TOKEN_REVOKED: Self = Self(-6);
    pub const DB_API_TOKEN_INACTIVE_OWNER: Self = Self(-7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_token_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub user_id: int64_t,
    pub created_by_user_id: int64_t,
    pub description: [::core::ffi::c_char; 128],
    pub token_prefix: [::core::ffi::c_char; 16],
    pub action_mask: uint64_t,
    pub scope_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub collection_uuid: [::core::ffi::c_char; 37],
    pub expires_at: int64_t,
    pub revoked_at: int64_t,
    pub last_used_at: int64_t,
    pub created_at: int64_t,
}
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
unsafe extern "C" fn health_observation(
    mut camera: *const fleet_camera_t,
) -> camera_health_observation_t {
    return camera_health_observation_t {
        health: (*camera).health,
        availability: (*camera).availability,
        health_changed_at: (*camera).health_changed_at,
        first_video_at: (*camera).first_video_at,
        last_frame_ts: (*camera).last_frame_ts,
        current_fps: (*camera).current_fps,
        recording_active: (*camera).recording_active,
    };
}
unsafe extern "C" fn apply_observation(
    mut camera: *mut fleet_camera_t,
    mut observation: *const camera_health_observation_t,
) {
    (*camera).health = (*observation).health;
    (*camera).availability = (*observation).availability;
    (*camera).health_changed_at = (*observation).health_changed_at;
    (*camera).first_video_at = (*observation).first_video_at;
    (*camera).last_frame_ts = (*observation).last_frame_ts;
    (*camera).current_fps = (*observation).current_fps;
    (*camera).recording_active = (*observation).recording_active;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_snapshot_free(mut snapshot: *mut authorization_snapshot_t) {
    if snapshot.is_null() {
        return;
    }
    authorization_context_free((*snapshot).context);
    free((*snapshot).inventory as *mut ::core::ffi::c_void);
    free((*snapshot).observations as *mut ::core::ffi::c_void);
    if !(*snapshot).connection.is_null() {
        db_close_readonly_connection((*snapshot).connection);
    }
    free(snapshot as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn load_principal(
    mut snapshot: *mut authorization_snapshot_t,
    mut input: *const user_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        (*snapshot).connection,
        b"SELECT role,is_active,authorization_mode FROM users WHERE id=?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, (*input).id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if rc == SQLITE_DONE {
        result = -2 as ::core::ffi::c_int;
    } else if rc == SQLITE_ROW {
        let mut mode: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut role: ::core::ffi::c_int = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
        if sqlite3_column_int(stmt, 1 as ::core::ffi::c_int) == 0 {
            result = -2 as ::core::ffi::c_int;
        } else if mode.is_null()
            || strcmp(mode, b"legacy\0".as_ptr() as *const ::core::ffi::c_char) != 0
                && strcmp(mode, b"policy\0".as_ptr() as *const ::core::ffi::c_char) != 0
            || role < user_role_t::USER_ROLE_ADMIN.0 as ::core::ffi::c_int
            || role > user_role_t::USER_ROLE_API.0 as ::core::ffi::c_int
        {
            result = -1 as ::core::ffi::c_int;
        } else {
            let mut user: *mut user_t = &raw mut (*snapshot).principal;
            (*user).id = (*input).id;
            (*user).role = user_role_t(role as ::core::ffi::c_uint);
            (*user).is_active = r#true != 0;
            safe_strcpy(
                &raw mut (*user).authorization_mode as *mut ::core::ffi::c_char,
                mode,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            (*user).authenticated_via_scoped_token = (*input).authenticated_via_scoped_token;
            safe_strcpy(
                &raw mut (*user).api_token_uuid as *mut ::core::ffi::c_char,
                &raw const (*input).api_token_uuid as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*user).authentication_method as *mut ::core::ffi::c_char,
                &raw const (*input).authentication_method as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
                0 as size_t,
            );
            result = 0 as ::core::ffi::c_int;
        }
    }
    sqlite3_finalize(stmt);
    return result;
}
unsafe extern "C" fn inventory_within_bound(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM streams;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_step(stmt) == SQLITE_ROW {
        let mut count: sqlite3_int64 = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int);
        if count >= 0 as sqlite3_int64 && count <= MAX_STREAMS as sqlite3_int64 {
            result = 0 as ::core::ffi::c_int;
        }
    }
    sqlite3_finalize(stmt);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_snapshot_create(
    mut principal: *const user_t,
    mut out: *mut *mut authorization_snapshot_t,
) -> ::core::ffi::c_int {
    if out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *out = ::core::ptr::null_mut::<authorization_snapshot_t>();
    if principal.is_null() || (*principal).id <= 0 as int64_t {
        return -2 as ::core::ffi::c_int;
    }
    let mut snapshot: *mut authorization_snapshot_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<authorization_snapshot_t>(),
    ) as *mut authorization_snapshot_t;
    if snapshot.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    '_fail: {
        if db_open_readonly_connection(&raw mut (*snapshot).connection) == 0 as ::core::ffi::c_int {
            sqlite3_busy_timeout((*snapshot).connection, 1000 as ::core::ffi::c_int);
            if sqlite3_exec(
                (*snapshot).connection,
                b"BEGIN;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ) == SQLITE_OK
            {
                result = load_principal(snapshot, principal);
                if result == 0 as ::core::ffi::c_int {
                    result = -1 as ::core::ffi::c_int;
                    (*snapshot).observed_at = time(::core::ptr::null_mut::<time_t>()) as int64_t;
                    if (*snapshot).observed_at >= 0 as int64_t {
                        if (*snapshot).principal.authenticated_via_scoped_token {
                            let mut token: api_token_t = api_token_t {
                                uuid: [0; 37],
                                user_id: 0,
                                created_by_user_id: 0,
                                description: [0; 128],
                                token_prefix: [0; 16],
                                action_mask: 0,
                                scope_type: [0; 16],
                                selector_json: [0; 8192],
                                collection_uuid: [0; 37],
                                expires_at: 0,
                                revoked_at: 0,
                                last_used_at: 0,
                                created_at: 0,
                            };
                            let mut token_result: db_api_token_result_t =
                                db_api_token_get_active_on_connection(
                                    (*snapshot).connection,
                                    (*snapshot).observed_at,
                                    &raw mut (*snapshot).principal.api_token_uuid
                                        as *mut ::core::ffi::c_char,
                                    &raw mut token,
                                );
                            if token_result.0 != db_api_token_result_t::DB_API_TOKEN_OK.0
                                || token.user_id != (*principal).id
                            {
                                result = if token_result.0
                                    == db_api_token_result_t::DB_API_TOKEN_ERROR.0
                                {
                                    -1 as ::core::ffi::c_int
                                } else {
                                    -2 as ::core::ffi::c_int
                                };
                                break '_fail;
                            }
                        }
                        (*snapshot).context = authorization_context_create_on_connection(
                            (*snapshot).connection,
                            (*snapshot).observed_at,
                        );
                        if !((*snapshot).context.is_null()
                            || inventory_within_bound((*snapshot).connection)
                                != 0 as ::core::ffi::c_int
                            || db_fleet_camera_load_on_connection(
                                (*snapshot).connection,
                                &raw mut (*snapshot).inventory,
                                &raw mut (*snapshot).count,
                            ) != 0 as ::core::ffi::c_int)
                        {
                            if (*snapshot).count > 0 as ::core::ffi::c_int {
                                (*snapshot).observations = calloc(
                                    (*snapshot).count as size_t,
                                    ::core::mem::size_of::<camera_observations_t>(),
                                )
                                    as *mut camera_observations_t;
                                if (*snapshot).observations.is_null() {
                                    break '_fail;
                                } else {
                                    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    while i < (*snapshot).count {
                                        (*(*snapshot).observations.offset(i as isize)).raw =
                                            health_observation(
                                                (*snapshot).inventory.offset(i as isize),
                                            );
                                        i += 1;
                                    }
                                    if clock_gettime(
                                        CLOCK_MONOTONIC,
                                        &raw mut (*snapshot).runtime_started,
                                    ) != 0 as ::core::ffi::c_int
                                        || fleet_camera_capture_runtime_health(
                                            (*snapshot).inventory,
                                            (*snapshot).count,
                                        ) != 0 as ::core::ffi::c_int
                                    {
                                        break '_fail;
                                    } else {
                                        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                        while i_0 < (*snapshot).count {
                                            (*(*snapshot).observations.offset(i_0 as isize))
                                                .runtime = health_observation(
                                                (*snapshot).inventory.offset(i_0 as isize),
                                            );
                                            i_0 += 1;
                                        }
                                        if fleet_health_capture_go2rtc_activity(
                                            (*snapshot).inventory,
                                            (*snapshot).count,
                                        ) != 0 as ::core::ffi::c_int
                                            || clock_gettime(
                                                CLOCK_MONOTONIC,
                                                &raw mut (*snapshot).runtime_finished,
                                            ) != 0 as ::core::ffi::c_int
                                        {
                                            break '_fail;
                                        }
                                    }
                                }
                            }
                            *out = snapshot;
                            return 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    authorization_snapshot_free(snapshot);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_snapshot_allows_non_scoped_identity(
    mut snapshot: *const authorization_snapshot_t,
) -> bool {
    return !snapshot.is_null() && !(*snapshot).principal.authenticated_via_scoped_token;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_snapshot_setup_complete(
    mut snapshot: *const authorization_snapshot_t,
) -> ::core::ffi::c_int {
    if snapshot.is_null() || (*snapshot).connection.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        (*snapshot).connection,
        b"SELECT (SELECT value FROM system_settings WHERE key='setup_complete');\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(statement) == SQLITE_ROW
    {
        let mut value: *const ::core::ffi::c_char =
            sqlite3_column_text(statement, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        result = if !value.is_null()
            && *value.offset(0isize) as ::core::ffi::c_int == '1' as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    sqlite3_finalize(statement);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_snapshot_camera_count(
    mut snapshot: *const authorization_snapshot_t,
) -> ::core::ffi::c_int {
    return if !snapshot.is_null() {
        (*snapshot).count
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_snapshot_evaluate(
    mut snapshot: *mut authorization_snapshot_t,
    mut action: authorization_action_t,
    mut camera_index: ::core::ffi::c_int,
    mut resource: *const ::core::ffi::c_char,
    mut evaluation: *mut authorization_evaluation_t,
) -> ::core::ffi::c_int {
    if !evaluation.is_null() {
        memset(
            evaluation as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<authorization_evaluation_t>(),
        );
    }
    if snapshot.is_null()
        || resource.is_null()
        || evaluation.is_null()
        || camera_index < -1 as ::core::ffi::c_int
        || camera_index >= (*snapshot).count
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut raw: bool = strcmp(resource, b"raw\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int;
    let mut runtime: bool = strcmp(
        resource,
        b"runtime\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    if !raw
        && !runtime
        && strcmp(resource, b"live\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if camera_index == -1 as ::core::ffi::c_int {
        return authorization_evaluate_in_context(
            (*snapshot).context,
            &raw mut (*snapshot).principal,
            action,
            ::core::ptr::null::<fleet_camera_t>(),
            evaluation,
        );
    }
    let mut camera: fleet_camera_t = *(*snapshot).inventory.offset(camera_index as isize);
    if raw {
        apply_observation(
            &raw mut camera,
            &raw mut (*(*snapshot).observations.offset(camera_index as isize)).raw,
        );
    } else if runtime {
        apply_observation(
            &raw mut camera,
            &raw mut (*(*snapshot).observations.offset(camera_index as isize)).runtime,
        );
    }
    return authorization_evaluate_in_context(
        (*snapshot).context,
        &raw mut (*snapshot).principal,
        action,
        &raw mut camera,
        evaluation,
    );
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
