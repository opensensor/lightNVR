use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
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
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
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
}
pub type size_t = usize;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __glibc_reserved: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_route_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub destination_key: [::core::ffi::c_char; 128],
    pub scope_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub predicate_json: [::core::ffi::c_char; 4096],
    pub schedule_json: [::core::ffi::c_char; 8192],
    pub event_types: [[::core::ffi::c_char; 96]; 32],
    pub event_type_count: ::core::ffi::c_int,
    pub debounce_seconds: ::core::ffi::c_int,
    pub cooldown_seconds: ::core::ffi::c_int,
    pub grouping_window_seconds: ::core::ffi::c_int,
    pub max_events_per_minute: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_suppression_result_t(pub ::core::ffi::c_int);
impl event_suppression_result_t {
    pub const EVENT_SUPPRESSION_ERROR: Self = Self(-1);
    pub const EVENT_SUPPRESSION_STALE: Self = Self(-2);
    pub const EVENT_SUPPRESSION_PERMIT: Self = Self(0);
    pub const EVENT_SUPPRESSION_DEBOUNCE: Self = Self(1);
    pub const EVENT_SUPPRESSION_COOLDOWN: Self = Self(2);
    pub const EVENT_SUPPRESSION_GROUPING: Self = Self(3);
    pub const EVENT_SUPPRESSION_RATE: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_route_suppression_state_t {
    pub last_observed_at: int64_t,
    pub last_allowed_at: int64_t,
    pub rate_window_started_at: int64_t,
    pub rate_window_count: ::core::ffi::c_int,
    pub group_started_at: int64_t,
    pub suppressed_count: int64_t,
    pub last_allowed_event_id: [::core::ffi::c_char; 37],
    pub last_reason: [::core::ffi::c_char; 16],
}
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const _ISupper: Self = Self(256);
    pub const _ISlower: Self = Self(512);
    pub const _ISalpha: Self = Self(1024);
    pub const _ISdigit: Self = Self(2048);
    pub const _ISxdigit: Self = Self(4096);
    pub const _ISspace: Self = Self(8192);
    pub const _ISprint: Self = Self(16384);
    pub const _ISgraph: Self = Self(32768);
    pub const _ISblank: Self = Self(1);
    pub const _IScntrl: Self = Self(2);
    pub const _ISpunct: Self = Self(4);
    pub const _ISalnum: Self = Self(8);
}
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const EVENT_TYPE_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const EVENT_SUBJECT_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
unsafe extern "C" fn valid_text(
    mut value: *const ::core::ffi::c_char,
    mut maximum: size_t,
) -> bool {
    if value.is_null()
        || *value.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strlen(value) >= maximum
    {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = value as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_input(
    mut route: *const event_route_t,
    mut event_type: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut now: int64_t,
) -> bool {
    return !route.is_null()
        && lightnvr_uuid_is_valid(&raw const (*route).uuid as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        && (*route).revision >= 1 as int64_t
        && now > 0 as int64_t
        && valid_text(event_type, EVENT_TYPE_MAX as size_t) as ::core::ffi::c_int != 0
        && valid_text(subject, EVENT_SUBJECT_MAX as size_t) as ::core::ffi::c_int != 0;
}
unsafe extern "C" fn valid_route_identity(
    mut route_uuid: *const ::core::ffi::c_char,
    mut route_revision: int64_t,
    mut event_type: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut now: int64_t,
) -> bool {
    return lightnvr_uuid_is_valid(route_uuid) as ::core::ffi::c_int != 0
        && route_revision >= 1 as int64_t
        && now > 0 as int64_t
        && valid_text(event_type, EVENT_TYPE_MAX as size_t) as ::core::ffi::c_int != 0
        && valid_text(subject, EVENT_SUBJECT_MAX as size_t) as ::core::ffi::c_int != 0;
}
unsafe extern "C" fn begin(mut db: *mut sqlite3) -> bool {
    return sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK;
}
unsafe extern "C" fn finish(mut db: *mut sqlite3, mut success: bool) -> bool {
    if success as ::core::ffi::c_int != 0
        && sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK
    {
        return r#true != 0;
    }
    sqlite3_exec(
        db,
        b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    return r#false != 0;
}
unsafe extern "C" fn route_is_current_locked(
    mut db: *mut sqlite3,
    mut route: *const event_route_t,
) -> event_suppression_result_t {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT enabled,revision FROM event_routes WHERE uuid=?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw const (*route).uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut outcome: event_suppression_result_t =
        event_suppression_result_t::EVENT_SUPPRESSION_ERROR;
    if result == SQLITE_ROW {
        let mut enabled: bool =
            sqlite3_column_int(statement, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut revision: int64_t =
            sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as int64_t;
        outcome = event_suppression_result_t(
            if enabled as ::core::ffi::c_int != 0 && revision == (*route).revision {
                event_suppression_result_t::EVENT_SUPPRESSION_PERMIT.0
            } else {
                event_suppression_result_t::EVENT_SUPPRESSION_STALE.0
            },
        );
    } else if result == SQLITE_DONE {
        outcome = event_suppression_result_t::EVENT_SUPPRESSION_STALE;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return outcome;
}
unsafe extern "C" fn load_current_route_locked(
    mut db: *mut sqlite3,
    mut route_uuid: *const ::core::ffi::c_char,
    mut route_revision: int64_t,
    mut route: *mut event_route_t,
) -> event_suppression_result_t {
    memset(
        route as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_route_t>(),
    );
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT enabled,revision,debounce_seconds,cooldown_seconds,grouping_window_seconds,max_events_per_minute FROM event_routes WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            route_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut outcome: event_suppression_result_t =
        event_suppression_result_t::EVENT_SUPPRESSION_ERROR;
    if result == SQLITE_ROW {
        let mut enabled: bool =
            sqlite3_column_int(statement, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut revision: int64_t =
            sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as int64_t;
        if !enabled || revision != route_revision {
            outcome = event_suppression_result_t::EVENT_SUPPRESSION_STALE;
        } else {
            safe_strcpy(
                &raw mut (*route).uuid as *mut ::core::ffi::c_char,
                route_uuid,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            (*route).revision = revision;
            (*route).debounce_seconds = sqlite3_column_int(statement, 2 as ::core::ffi::c_int);
            (*route).cooldown_seconds = sqlite3_column_int(statement, 3 as ::core::ffi::c_int);
            (*route).grouping_window_seconds =
                sqlite3_column_int(statement, 4 as ::core::ffi::c_int);
            (*route).max_events_per_minute = sqlite3_column_int(statement, 5 as ::core::ffi::c_int);
            outcome = event_suppression_result_t::EVENT_SUPPRESSION_PERMIT;
        }
    } else if result == SQLITE_DONE {
        outcome = event_suppression_result_t::EVENT_SUPPRESSION_STALE;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return outcome;
}
unsafe extern "C" fn load_state_locked(
    mut db: *mut sqlite3,
    mut route_uuid: *const ::core::ffi::c_char,
    mut event_type: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut state: *mut event_route_suppression_state_t,
) -> ::core::ffi::c_int {
    memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_route_suppression_state_t>(),
    );
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT last_observed_at,last_allowed_at,rate_window_started_at,rate_window_count,group_started_at,suppressed_count,last_allowed_event_id,last_reason FROM event_route_suppression_state WHERE route_uuid=? AND event_type=? AND subject=?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            route_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            event_type,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            subject,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if result == SQLITE_ROW {
        (*state).last_observed_at =
            sqlite3_column_int64(statement, 0 as ::core::ffi::c_int) as int64_t;
        (*state).last_allowed_at =
            sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as int64_t;
        (*state).rate_window_started_at =
            sqlite3_column_int64(statement, 2 as ::core::ffi::c_int) as int64_t;
        (*state).rate_window_count = sqlite3_column_int(statement, 3 as ::core::ffi::c_int);
        (*state).group_started_at =
            sqlite3_column_int64(statement, 4 as ::core::ffi::c_int) as int64_t;
        (*state).suppressed_count =
            sqlite3_column_int64(statement, 5 as ::core::ffi::c_int) as int64_t;
        let mut event_id: *const ::core::ffi::c_char =
            sqlite3_column_text(statement, 6 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut reason: *const ::core::ffi::c_char =
            sqlite3_column_text(statement, 7 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*state).last_allowed_event_id as *mut ::core::ffi::c_char,
            if !event_id.is_null() {
                event_id
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*state).last_reason as *mut ::core::ffi::c_char,
            if !reason.is_null() {
                reason
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        found = 1 as ::core::ffi::c_int;
    } else if result != SQLITE_DONE {
        found = -1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return found;
}
unsafe extern "C" fn within_interval(
    mut now: int64_t,
    mut previous: int64_t,
    mut seconds: ::core::ffi::c_int,
) -> bool {
    if seconds <= 0 as ::core::ffi::c_int || previous <= 0 as int64_t {
        return r#false != 0;
    }
    return now < previous || now - previous < seconds as int64_t;
}
unsafe extern "C" fn latest_state_time(
    mut state: *const event_route_suppression_state_t,
    mut now: int64_t,
) -> int64_t {
    let mut latest: int64_t = now;
    if (*state).last_observed_at > latest {
        latest = (*state).last_observed_at;
    }
    if (*state).last_allowed_at > latest {
        latest = (*state).last_allowed_at;
    }
    if (*state).rate_window_started_at > latest {
        latest = (*state).rate_window_started_at;
    }
    if (*state).group_started_at > latest {
        latest = (*state).group_started_at;
    }
    return latest;
}
unsafe extern "C" fn reason_name(
    mut result: event_suppression_result_t,
) -> *const ::core::ffi::c_char {
    match result {
        event_suppression_result_t::EVENT_SUPPRESSION_DEBOUNCE => {
            return b"debounce\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_suppression_result_t::EVENT_SUPPRESSION_COOLDOWN => {
            return b"cooldown\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_suppression_result_t::EVENT_SUPPRESSION_GROUPING => {
            return b"grouping\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_suppression_result_t::EVENT_SUPPRESSION_RATE => {
            return b"rate\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"allowed\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
unsafe extern "C" fn decide(
    mut route: *const event_route_t,
    mut state: *const event_route_suppression_state_t,
    mut now: int64_t,
) -> event_suppression_result_t {
    if within_interval(now, (*state).last_observed_at, (*route).debounce_seconds) {
        return event_suppression_result_t::EVENT_SUPPRESSION_DEBOUNCE;
    }
    if within_interval(now, (*state).last_allowed_at, (*route).cooldown_seconds) {
        return event_suppression_result_t::EVENT_SUPPRESSION_COOLDOWN;
    }
    if within_interval(
        now,
        (*state).group_started_at,
        (*route).grouping_window_seconds,
    ) {
        return event_suppression_result_t::EVENT_SUPPRESSION_GROUPING;
    }
    let mut active_rate_window: bool = (*state).rate_window_started_at > 0 as int64_t
        && (now < (*state).rate_window_started_at
            || now - (*state).rate_window_started_at < 60 as int64_t);
    if (*route).max_events_per_minute > 0 as ::core::ffi::c_int
        && active_rate_window as ::core::ffi::c_int != 0
        && (*state).rate_window_count >= (*route).max_events_per_minute
    {
        return event_suppression_result_t::EVENT_SUPPRESSION_RATE;
    }
    return event_suppression_result_t::EVENT_SUPPRESSION_PERMIT;
}
unsafe extern "C" fn save_state_locked(
    mut db: *mut sqlite3,
    mut route: *const event_route_t,
    mut event_type: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut state: *const event_route_suppression_state_t,
    mut updated_at: int64_t,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO event_route_suppression_state(route_uuid,event_type,subject,last_observed_at,last_allowed_at,rate_window_started_at,rate_window_count,group_started_at,suppressed_count,last_allowed_event_id,last_reason,updated_at) VALUES(?,?,?,?,?,?,?,?,?,?,?,?) ON CONFLICT(route_uuid,event_type,subject) DO UPDATE SET last_observed_at=excluded.last_observed_at,last_allowed_at=excluded.last_allowed_at,rate_window_started_at=excluded.rate_window_started_at,rate_window_count=excluded.rate_window_count,group_started_at=excluded.group_started_at,suppressed_count=excluded.suppressed_count,last_allowed_event_id=excluded.last_allowed_event_id,last_reason=excluded.last_reason,updated_at=excluded.updated_at;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw const (*route).uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            event_type,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            subject,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            4 as ::core::ffi::c_int,
            (*state).last_observed_at as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            5 as ::core::ffi::c_int,
            (*state).last_allowed_at as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            6 as ::core::ffi::c_int,
            (*state).rate_window_started_at as sqlite3_int64,
        );
        sqlite3_bind_int(
            statement,
            7 as ::core::ffi::c_int,
            (*state).rate_window_count,
        );
        sqlite3_bind_int64(
            statement,
            8 as ::core::ffi::c_int,
            (*state).group_started_at as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            9 as ::core::ffi::c_int,
            (*state).suppressed_count as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            10 as ::core::ffi::c_int,
            &raw const (*state).last_allowed_event_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            11 as ::core::ffi::c_int,
            &raw const (*state).last_reason as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            12 as ::core::ffi::c_int,
            updated_at as sqlite3_int64,
        );
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_suppression_check(
    mut route: *const event_route_t,
    mut event_type: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut now: int64_t,
) -> event_suppression_result_t {
    if !valid_input(route, event_type, subject, now) {
        return event_suppression_result_t::EVENT_SUPPRESSION_ERROR;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return event_suppression_result_t::EVENT_SUPPRESSION_ERROR;
    }
    pthread_mutex_lock(mutex);
    if !begin(db) {
        pthread_mutex_unlock(mutex);
        return event_suppression_result_t::EVENT_SUPPRESSION_ERROR;
    }
    let mut current: event_suppression_result_t = route_is_current_locked(db, route);
    let mut state: event_route_suppression_state_t = event_route_suppression_state_t {
        last_observed_at: 0,
        last_allowed_at: 0,
        rate_window_started_at: 0,
        rate_window_count: 0,
        group_started_at: 0,
        suppressed_count: 0,
        last_allowed_event_id: [0; 37],
        last_reason: [0; 16],
    };
    let mut found: ::core::ffi::c_int =
        if current.0 == event_suppression_result_t::EVENT_SUPPRESSION_PERMIT.0 {
            load_state_locked(
                db,
                &raw const (*route).uuid as *const ::core::ffi::c_char,
                event_type,
                subject,
                &raw mut state,
            )
        } else {
            -1 as ::core::ffi::c_int
        };
    if current.0 != event_suppression_result_t::EVENT_SUPPRESSION_PERMIT.0
        || found < 0 as ::core::ffi::c_int
    {
        finish(db, r#false != 0);
        pthread_mutex_unlock(mutex);
        return event_suppression_result_t(
            if current.0 == event_suppression_result_t::EVENT_SUPPRESSION_STALE.0 {
                current.0
            } else {
                event_suppression_result_t::EVENT_SUPPRESSION_ERROR.0
            },
        );
    }
    let mut decision: event_suppression_result_t = decide(route, &raw mut state, now);
    let mut success: bool = r#true != 0;
    if decision.0 != event_suppression_result_t::EVENT_SUPPRESSION_PERMIT.0 {
        if now > state.last_observed_at {
            state.last_observed_at = now;
        }
        if state.suppressed_count < INT64_MAX as int64_t {
            state.suppressed_count += 1;
        }
        safe_strcpy(
            &raw mut state.last_reason as *mut ::core::ffi::c_char,
            reason_name(decision),
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        success = save_state_locked(
            db,
            route,
            event_type,
            subject,
            &raw mut state,
            latest_state_time(&raw mut state, now),
        ) == 0 as ::core::ffi::c_int;
    }
    success = finish(db, success);
    pthread_mutex_unlock(mutex);
    return event_suppression_result_t(if success as ::core::ffi::c_int != 0 {
        decision.0
    } else {
        event_suppression_result_t::EVENT_SUPPRESSION_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_suppression_record_allowed(
    mut route_uuid: *const ::core::ffi::c_char,
    mut route_revision: int64_t,
    mut event_type: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut event_id: *const ::core::ffi::c_char,
    mut now: int64_t,
) -> event_suppression_result_t {
    if !valid_route_identity(route_uuid, route_revision, event_type, subject, now)
        || !lightnvr_uuid_is_valid(event_id)
    {
        return event_suppression_result_t::EVENT_SUPPRESSION_ERROR;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return event_suppression_result_t::EVENT_SUPPRESSION_ERROR;
    }
    pthread_mutex_lock(mutex);
    if !begin(db) {
        pthread_mutex_unlock(mutex);
        return event_suppression_result_t::EVENT_SUPPRESSION_ERROR;
    }
    let mut route: event_route_t = event_route_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_key: [0; 128],
        scope_type: [0; 16],
        selector_json: [0; 8192],
        predicate_json: [0; 4096],
        schedule_json: [0; 8192],
        event_types: [[0; 96]; 32],
        event_type_count: 0,
        debounce_seconds: 0,
        cooldown_seconds: 0,
        grouping_window_seconds: 0,
        max_events_per_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut current: event_suppression_result_t =
        load_current_route_locked(db, route_uuid, route_revision, &raw mut route);
    let mut state: event_route_suppression_state_t = event_route_suppression_state_t {
        last_observed_at: 0,
        last_allowed_at: 0,
        rate_window_started_at: 0,
        rate_window_count: 0,
        group_started_at: 0,
        suppressed_count: 0,
        last_allowed_event_id: [0; 37],
        last_reason: [0; 16],
    };
    let mut found: ::core::ffi::c_int =
        if current.0 == event_suppression_result_t::EVENT_SUPPRESSION_PERMIT.0 {
            load_state_locked(db, route_uuid, event_type, subject, &raw mut state)
        } else {
            -1 as ::core::ffi::c_int
        };
    if current.0 != event_suppression_result_t::EVENT_SUPPRESSION_PERMIT.0
        || found < 0 as ::core::ffi::c_int
    {
        finish(db, r#false != 0);
        pthread_mutex_unlock(mutex);
        return event_suppression_result_t(
            if current.0 == event_suppression_result_t::EVENT_SUPPRESSION_STALE.0 {
                current.0
            } else {
                event_suppression_result_t::EVENT_SUPPRESSION_ERROR.0
            },
        );
    }
    if found > 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut state.last_allowed_event_id as *mut ::core::ffi::c_char,
            event_id,
        ) == 0 as ::core::ffi::c_int
    {
        let mut committed: bool = finish(db, r#true != 0);
        pthread_mutex_unlock(mutex);
        return event_suppression_result_t(if committed as ::core::ffi::c_int != 0 {
            event_suppression_result_t::EVENT_SUPPRESSION_PERMIT.0
        } else {
            event_suppression_result_t::EVENT_SUPPRESSION_ERROR.0
        });
    }
    let mut effective_now: int64_t = latest_state_time(&raw mut state, now);
    state.last_observed_at = effective_now;
    state.last_allowed_at = effective_now;
    state.group_started_at = if route.grouping_window_seconds > 0 as ::core::ffi::c_int {
        effective_now
    } else {
        0 as int64_t
    };
    let mut active_rate_window: bool = state.rate_window_started_at > 0 as int64_t
        && effective_now - state.rate_window_started_at < 60 as int64_t;
    if route.max_events_per_minute > 0 as ::core::ffi::c_int {
        if !active_rate_window {
            state.rate_window_started_at = effective_now;
            state.rate_window_count = 1 as ::core::ffi::c_int;
        } else if state.rate_window_count < INT_MAX {
            state.rate_window_count += 1;
        }
    } else {
        state.rate_window_started_at = 0 as int64_t;
        state.rate_window_count = 0 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut state.last_allowed_event_id as *mut ::core::ffi::c_char,
        event_id,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut state.last_reason as *mut ::core::ffi::c_char,
        b"allowed\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    let mut success: bool = save_state_locked(
        db,
        &raw mut route,
        event_type,
        subject,
        &raw mut state,
        effective_now,
    ) == 0 as ::core::ffi::c_int;
    success = finish(db, success);
    pthread_mutex_unlock(mutex);
    return event_suppression_result_t(if success as ::core::ffi::c_int != 0 {
        event_suppression_result_t::EVENT_SUPPRESSION_PERMIT.0
    } else {
        event_suppression_result_t::EVENT_SUPPRESSION_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_suppression_get(
    mut route_uuid: *const ::core::ffi::c_char,
    mut event_type: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut state: *mut event_route_suppression_state_t,
) -> ::core::ffi::c_int {
    if !lightnvr_uuid_is_valid(route_uuid)
        || !valid_text(event_type, EVENT_TYPE_MAX as size_t)
        || !valid_text(subject, EVENT_SUBJECT_MAX as size_t)
        || state.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut result: ::core::ffi::c_int =
        load_state_locked(db, route_uuid, event_type, subject, state);
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_suppression_prune(
    mut updated_before: int64_t,
    mut limit: ::core::ffi::c_int,
    mut deleted_count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !deleted_count.is_null() {
        *deleted_count = 0 as ::core::ffi::c_int;
    }
    if updated_before <= 0 as int64_t
        || limit <= 0 as ::core::ffi::c_int
        || limit > 10000 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM event_route_suppression_state WHERE rowid IN (SELECT rowid FROM event_route_suppression_state WHERE updated_at<? ORDER BY updated_at,rowid LIMIT ?);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_int64(
            statement,
            1 as ::core::ffi::c_int,
            updated_before as sqlite3_int64,
        );
        sqlite3_bind_int(statement, 2 as ::core::ffi::c_int, limit);
        result = sqlite3_step(statement);
    }
    if !deleted_count.is_null() && result == SQLITE_DONE {
        *deleted_count = sqlite3_changes(db);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
