use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn system_health_scope_name(scope: system_health_scope_t) -> *const ::core::ffi::c_char;
    unsafe fn system_health_condition_code(
        condition: system_health_condition_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn cJSON_ParseWithOpts(
        value: *const ::core::ffi::c_char,
        return_parse_end: *mut *const ::core::ffi::c_char,
        require_null_terminated: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_free(object: *mut ::core::ffi::c_void);
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
    unsafe fn sqlite3_bind_null(_: *mut sqlite3_stmt, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_scope_t(pub ::core::ffi::c_uint);
impl system_health_scope_t {
    pub const SYSTEM_HEALTH_SCOPE_PROCESS: Self = Self(0);
    pub const SYSTEM_HEALTH_SCOPE_CONTAINER: Self = Self(1);
    pub const SYSTEM_HEALTH_SCOPE_HOST: Self = Self(2);
    pub const SYSTEM_HEALTH_SCOPE_FILESYSTEM: Self = Self(3);
    pub const SYSTEM_HEALTH_SCOPE_DEVICE: Self = Self(4);
    pub const SYSTEM_HEALTH_SCOPE_COUNT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_severity_t(pub ::core::ffi::c_uint);
impl system_health_severity_t {
    pub const SYSTEM_HEALTH_SEVERITY_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_SEVERITY_WARNING: Self = Self(1);
    pub const SYSTEM_HEALTH_SEVERITY_ERROR: Self = Self(2);
    pub const SYSTEM_HEALTH_SEVERITY_CRITICAL: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_state_t(pub ::core::ffi::c_uint);
impl system_health_state_t {
    pub const SYSTEM_HEALTH_STATE_UNKNOWN: Self = Self(0);
    pub const SYSTEM_HEALTH_STATE_HEALTHY: Self = Self(1);
    pub const SYSTEM_HEALTH_STATE_PENDING: Self = Self(2);
    pub const SYSTEM_HEALTH_STATE_OPEN: Self = Self(3);
    pub const SYSTEM_HEALTH_STATE_RECOVERING: Self = Self(4);
    pub const SYSTEM_HEALTH_STATE_CLOSED: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_condition_t(pub ::core::ffi::c_uint);
impl system_health_condition_t {
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW: Self = Self(0);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL: Self = Self(1);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_SWAP_THRASH: Self = Self(2);
    pub const SYSTEM_HEALTH_CONDITION_CPU_SATURATION: Self = Self(3);
    pub const SYSTEM_HEALTH_CONDITION_CPU_THROTTLED: Self = Self(4);
    pub const SYSTEM_HEALTH_CONDITION_IO_PRESSURE: Self = Self(5);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW: Self = Self(6);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW: Self = Self(7);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY: Self = Self(8);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED: Self = Self(9);
    pub const SYSTEM_HEALTH_CONDITION_THERMAL_HIGH: Self = Self(10);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN: Self = Self(11);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE: Self = Self(12);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED: Self = Self(13);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_JUMP: Self = Self(14);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION: Self = Self(15);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION: Self = Self(16);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED: Self = Self(17);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL: Self = Self(18);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL: Self = Self(19);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_CORRECTED: Self = Self(20);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE: Self = Self(21);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED: Self = Self(22);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_POWER_UNSTABLE: Self = Self(23);
    pub const SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE: Self = Self(24);
    pub const SYSTEM_HEALTH_CONDITION_UNEXPECTED_RESTART: Self = Self(25);
    pub const SYSTEM_HEALTH_CONDITION_EVENT_DELIVERY_DEGRADED: Self = Self(26);
    pub const SYSTEM_HEALTH_CONDITION_COUNT: Self = Self(27);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_system_health_result_t(pub ::core::ffi::c_int);
impl db_system_health_result_t {
    pub const DB_SYSTEM_HEALTH_OK: Self = Self(0);
    pub const DB_SYSTEM_HEALTH_RESUMED: Self = Self(1);
    pub const DB_SYSTEM_HEALTH_NOT_FOUND: Self = Self(2);
    pub const DB_SYSTEM_HEALTH_INVALID: Self = Self(-2);
    pub const DB_SYSTEM_HEALTH_CONFLICT: Self = Self(-3);
    pub const DB_SYSTEM_HEALTH_ERROR: Self = Self(-1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_incident_action_t(pub ::core::ffi::c_uint);
impl system_health_incident_action_t {
    pub const SYSTEM_HEALTH_INCIDENT_OPEN: Self = Self(0);
    pub const SYSTEM_HEALTH_INCIDENT_ESCALATE: Self = Self(1);
    pub const SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE: Self = Self(2);
    pub const SYSTEM_HEALTH_INCIDENT_RECOVER: Self = Self(3);
    pub const SYSTEM_HEALTH_INCIDENT_ONE_SHOT: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_reconciliation_t(pub ::core::ffi::c_uint);
impl system_health_reconciliation_t {
    pub const SYSTEM_HEALTH_RECONCILIATION_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING: Self = Self(1);
    pub const SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING: Self = Self(2);
    pub const SYSTEM_HEALTH_RECONCILIATION_RECONCILED: Self = Self(3);
    pub const SYSTEM_HEALTH_RECONCILIATION_DELIVERY_FAILED: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_signal_t {
    pub condition: system_health_condition_t,
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub state: system_health_state_t,
    pub severity: system_health_severity_t,
    pub observed_at_ms: int64_t,
    pub observation_json: [::core::ffi::c_char; 2049],
    pub event_id: [::core::ffi::c_char; 37],
    pub reconciliation: system_health_reconciliation_t,
    pub boot_id: [::core::ffi::c_char; 64],
    pub run_id: [::core::ffi::c_char; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_record_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub condition_code: [::core::ffi::c_char; 64],
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub state: system_health_state_t,
    pub severity: system_health_severity_t,
    pub first_seen_at_ms: int64_t,
    pub last_seen_at_ms: int64_t,
    pub closed_at_ms: int64_t,
    pub observation_json: [::core::ffi::c_char; 2049],
    pub alert_event_id: [::core::ffi::c_char; 37],
    pub recovery_event_id: [::core::ffi::c_char; 37],
    pub reconciliation: system_health_reconciliation_t,
    pub boot_id: [::core::ffi::c_char; 64],
    pub run_id: [::core::ffi::c_char; 37],
    pub revision: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_cursor_t {
    pub valid: bool,
    pub last_seen_at_ms: int64_t,
    pub uuid: [::core::ffi::c_char; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_transition_t {
    pub id: int64_t,
    pub transition_uuid: [::core::ffi::c_char; 37],
    pub incident_uuid: [::core::ffi::c_char; 37],
    pub action: system_health_incident_action_t,
    pub from_state: system_health_state_t,
    pub from_state_valid: bool,
    pub to_state: system_health_state_t,
    pub severity: system_health_severity_t,
    pub observed_at_ms: int64_t,
    pub observation_json: [::core::ffi::c_char; 2049],
    pub event_id: [::core::ffi::c_char; 37],
    pub reconciliation: system_health_reconciliation_t,
    pub boot_id: [::core::ffi::c_char; 64],
    pub run_id: [::core::ffi::c_char; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_transition_cursor_t {
    pub valid: bool,
    pub observed_at_ms: int64_t,
    pub id: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_process_run_t {
    pub run_id: [::core::ffi::c_char; 37],
    pub boot_id: [::core::ffi::c_char; 64],
    pub started_at_ms: int64_t,
    pub closed_at_ms: int64_t,
    pub clean_close: bool,
}
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
pub const SYSTEM_HEALTH_ID_LENGTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_INCIDENT_SUBJECT_MAX: ::core::ffi::c_uint = SYSTEM_HEALTH_ID_LENGTH;
pub const SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX: ::core::ffi::c_uint = 2049 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_INCIDENT_BOOT_ID_MAX: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_INCIDENT_PAGE_MAX: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SYSTEM_HEALTH_TRANSITION_RETENTION_BATCH_MAX: ::core::ffi::c_int =
    1000 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
unsafe extern "C" fn transaction_begin(mut database: *mut sqlite3) -> bool {
    return sqlite3_exec(
        database,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK;
}
unsafe extern "C" fn transaction_finish(mut database: *mut sqlite3, mut success: bool) -> bool {
    if success as ::core::ffi::c_int != 0
        && sqlite3_exec(
            database,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK
    {
        return r#true != 0;
    }
    sqlite3_exec(
        database,
        b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    return r#false != 0;
}
unsafe extern "C" fn valid_logical_id(
    mut value: *const ::core::ffi::c_char,
    mut capacity: size_t,
) -> bool {
    let mut length: size_t = 0;
    if value.is_null()
        || capacity < 2 as size_t
        || {
            length = strnlen(value, capacity);
            length == 0 as size_t
        }
        || length >= capacity
    {
        return r#false != 0;
    }
    let mut index: size_t = 0 as size_t;
    while index < length {
        let mut current: ::core::ffi::c_uchar =
            *value.offset(index as isize) as ::core::ffi::c_uchar;
        if !(*(*__ctype_b_loc()).offset(current as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            || current as ::core::ffi::c_int == '_' as ::core::ffi::c_int
            || current as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || current as ::core::ffi::c_int == ':' as ::core::ffi::c_int
            || current as ::core::ffi::c_int == '-' as ::core::ffi::c_int)
        {
            return r#false != 0;
        }
        index = index.wrapping_add(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn normalize_observation(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
) -> bool {
    if input.is_null()
        || strnlen(input, SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX as size_t)
            >= SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX as size_t
    {
        return r#false != 0;
    }
    let mut root: *mut cJSON = cJSON_ParseWithOpts(
        input,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        r#true,
    );
    if cJSON_IsObject(root) == 0 {
        cJSON_Delete(root);
        return r#false != 0;
    }
    let mut encoded: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if encoded.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strlen(encoded);
    let mut valid: bool =
        length >= 2 as size_t && length < SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX as size_t;
    if valid {
        safe_strcpy(
            output,
            encoded,
            SYSTEM_HEALTH_INCIDENT_OBSERVATION_MAX as size_t,
            0 as size_t,
        );
    }
    cJSON_free(encoded as *mut ::core::ffi::c_void);
    return valid;
}
unsafe extern "C" fn state_name(mut state: system_health_state_t) -> *const ::core::ffi::c_char {
    match state {
        system_health_state_t::SYSTEM_HEALTH_STATE_OPEN => {
            return b"open\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_state_t::SYSTEM_HEALTH_STATE_RECOVERING => {
            return b"recovering\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_state_t::SYSTEM_HEALTH_STATE_CLOSED => {
            return b"closed\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return ::core::ptr::null::<::core::ffi::c_char>(),
    };
}
unsafe extern "C" fn parse_state(
    mut value: *const ::core::ffi::c_char,
    mut valid: *mut bool,
) -> system_health_state_t {
    *valid = r#true != 0;
    if strcmp(value, b"open\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return system_health_state_t::SYSTEM_HEALTH_STATE_OPEN;
    }
    if strcmp(
        value,
        b"recovering\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return system_health_state_t::SYSTEM_HEALTH_STATE_RECOVERING;
    }
    if strcmp(value, b"closed\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return system_health_state_t::SYSTEM_HEALTH_STATE_CLOSED;
    }
    *valid = r#false != 0;
    return system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN;
}
unsafe extern "C" fn severity_name(
    mut severity: system_health_severity_t,
) -> *const ::core::ffi::c_char {
    match severity {
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING => {
            return b"warning\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_ERROR => {
            return b"error\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL => {
            return b"critical\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return ::core::ptr::null::<::core::ffi::c_char>(),
    };
}
unsafe extern "C" fn parse_severity(
    mut value: *const ::core::ffi::c_char,
) -> system_health_severity_t {
    if strcmp(value, b"warning\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING;
    }
    if strcmp(value, b"error\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return system_health_severity_t::SYSTEM_HEALTH_SEVERITY_ERROR;
    }
    if strcmp(value, b"critical\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL;
    }
    return system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE;
}
unsafe extern "C" fn reconciliation_name(
    mut value: system_health_reconciliation_t,
) -> *const ::core::ffi::c_char {
    match value {
        system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING => {
            return b"alert_pending\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING => {
            return b"recovery_pending\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_RECONCILED => {
            return b"reconciled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_DELIVERY_FAILED => {
            return b"delivery_failed\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn parse_reconciliation(
    mut value: *const ::core::ffi::c_char,
) -> system_health_reconciliation_t {
    if strcmp(
        value,
        b"alert_pending\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING;
    }
    if strcmp(
        value,
        b"recovery_pending\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING;
    }
    if strcmp(
        value,
        b"reconciled\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_RECONCILED;
    }
    if strcmp(
        value,
        b"delivery_failed\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_DELIVERY_FAILED;
    }
    return system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE;
}
unsafe extern "C" fn action_name(
    mut action: system_health_incident_action_t,
) -> *const ::core::ffi::c_char {
    match action {
        system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_OPEN => {
            return b"open\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_ESCALATE => {
            return b"escalation\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE => {
            return b"material_change\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER => {
            return b"recovery\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_ONE_SHOT => {
            return b"one_shot\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn parse_action(
    mut value: *const ::core::ffi::c_char,
    mut valid: *mut bool,
) -> system_health_incident_action_t {
    *valid = r#true != 0;
    if strcmp(value, b"open\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_OPEN;
    }
    if strcmp(
        value,
        b"escalation\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_ESCALATE;
    }
    if strcmp(
        value,
        b"material_change\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE;
    }
    if strcmp(value, b"recovery\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER;
    }
    if strcmp(value, b"one_shot\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_ONE_SHOT;
    }
    *valid = r#false != 0;
    return system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_OPEN;
}
unsafe extern "C" fn parse_scope(
    mut value: *const ::core::ffi::c_char,
    mut valid: *mut bool,
) -> system_health_scope_t {
    *valid = r#true != 0;
    if strcmp(value, b"process\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS;
    }
    if strcmp(value, b"container\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER;
    }
    if strcmp(value, b"host\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST;
    }
    if strcmp(
        value,
        b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return system_health_scope_t::SYSTEM_HEALTH_SCOPE_FILESYSTEM;
    }
    if strcmp(value, b"device\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return system_health_scope_t::SYSTEM_HEALTH_SCOPE_DEVICE;
    }
    *valid = r#false != 0;
    return system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST;
}
unsafe extern "C" fn copy_column(
    mut destination: *mut ::core::ffi::c_char,
    mut capacity: size_t,
    mut statement: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
) {
    let mut text: *const ::core::ffi::c_char =
        sqlite3_column_text(statement, column) as *const ::core::ffi::c_char;
    safe_strcpy(
        destination,
        if !text.is_null() {
            text
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        capacity,
        0 as size_t,
    );
}
unsafe extern "C" fn populate_incident(
    mut statement: *mut sqlite3_stmt,
    mut incident: *mut system_health_incident_record_t,
) -> bool {
    let mut scope_valid: bool = false;
    let mut state_valid: bool = false;
    memset(
        incident as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_incident_record_t>(),
    );
    copy_column(
        &raw mut (*incident).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*incident).condition_code as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*incident).subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        2 as ::core::ffi::c_int,
    );
    (*incident).scope = parse_scope(
        sqlite3_column_text(statement, 3 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
        &raw mut scope_valid,
    );
    (*incident).state = parse_state(
        sqlite3_column_text(statement, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
        &raw mut state_valid,
    );
    (*incident).severity = parse_severity(
        sqlite3_column_text(statement, 5 as ::core::ffi::c_int) as *const ::core::ffi::c_char
    );
    (*incident).first_seen_at_ms =
        sqlite3_column_int64(statement, 6 as ::core::ffi::c_int) as int64_t;
    (*incident).last_seen_at_ms =
        sqlite3_column_int64(statement, 7 as ::core::ffi::c_int) as int64_t;
    (*incident).closed_at_ms = sqlite3_column_int64(statement, 8 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*incident).observation_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2049]>(),
        statement,
        9 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*incident).alert_event_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        10 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*incident).recovery_event_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        11 as ::core::ffi::c_int,
    );
    (*incident).reconciliation = parse_reconciliation(sqlite3_column_text(
        statement,
        12 as ::core::ffi::c_int,
    ) as *const ::core::ffi::c_char);
    copy_column(
        &raw mut (*incident).boot_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        13 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*incident).run_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        14 as ::core::ffi::c_int,
    );
    (*incident).revision = sqlite3_column_int64(statement, 15 as ::core::ffi::c_int) as int64_t;
    return scope_valid as ::core::ffi::c_int != 0
        && state_valid as ::core::ffi::c_int != 0
        && (*incident).severity.0 != system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0;
}
unsafe extern "C" fn get_active_locked(
    mut database: *mut sqlite3,
    mut condition_code: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut incident: *mut system_health_incident_record_t,
) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char = b"SELECT uuid,condition_code,subject,scope,state,severity,first_seen_at_ms,last_seen_at_ms,COALESCE(closed_at_ms,0),last_observation_json,COALESCE(alert_event_id,''),COALESCE(recovery_event_id,''),reconciliation_state,boot_id,run_id,revision FROM system_health_incidents WHERE condition_code=? AND subject=? AND state<>'closed' LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            condition_code,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
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
        found = if populate_incident(statement, incident) as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    } else if result != SQLITE_DONE {
        found = -1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return found;
}
unsafe extern "C" fn get_uuid_locked(
    mut database: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut incident: *mut system_health_incident_record_t,
) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char = b"SELECT uuid,condition_code,subject,scope,state,severity,first_seen_at_ms,last_seen_at_ms,COALESCE(closed_at_ms,0),last_observation_json,COALESCE(alert_event_id,''),COALESCE(recovery_event_id,''),reconciliation_state,boot_id,run_id,revision FROM system_health_incidents WHERE uuid=? LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            uuid,
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
        found = if populate_incident(statement, incident) as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    } else if result != SQLITE_DONE {
        found = -1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return found;
}
unsafe extern "C" fn valid_signal(
    mut action: system_health_incident_action_t,
    mut signal: *const system_health_incident_signal_t,
    mut normalized: *mut ::core::ffi::c_char,
) -> bool {
    let mut condition_code: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if signal.is_null()
        || action_name(action).is_null()
        || (*signal).observed_at_ms <= 0 as int64_t
        || {
            condition_code = system_health_condition_code((*signal).condition);
            condition_code.is_null()
        }
        || !valid_logical_id(
            &raw const (*signal).subject as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        )
        || (*signal).scope.0 < system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS.0
        || (*signal).scope.0 >= system_health_scope_t::SYSTEM_HEALTH_SCOPE_COUNT.0
        || reconciliation_name((*signal).reconciliation).is_null()
        || !valid_logical_id(
            &raw const (*signal).boot_id as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        )
        || !lightnvr_uuid_is_valid(&raw const (*signal).run_id as *const ::core::ffi::c_char)
        || (*signal).event_id[0usize] as ::core::ffi::c_int != 0
            && !lightnvr_uuid_is_valid(&raw const (*signal).event_id as *const ::core::ffi::c_char)
        || !normalize_observation(
            &raw const (*signal).observation_json as *const ::core::ffi::c_char,
            normalized,
        )
    {
        return r#false != 0;
    }
    if action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0 {
        return (*signal).state.0 == system_health_state_t::SYSTEM_HEALTH_STATE_CLOSED.0
            && ((*signal).severity.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                || !severity_name((*signal).severity).is_null());
    }
    if severity_name((*signal).severity).is_null() {
        return r#false != 0;
    }
    if action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE.0 {
        return (*signal).state.0 == system_health_state_t::SYSTEM_HEALTH_STATE_OPEN.0
            || (*signal).state.0 == system_health_state_t::SYSTEM_HEALTH_STATE_RECOVERING.0;
    }
    if action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_ONE_SHOT.0 {
        return (*signal).state.0 == system_health_state_t::SYSTEM_HEALTH_STATE_CLOSED.0;
    }
    return (*signal).state.0 == system_health_state_t::SYSTEM_HEALTH_STATE_OPEN.0;
}
unsafe extern "C" fn insert_incident_locked(
    mut database: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut condition_code: *const ::core::ffi::c_char,
    mut signal: *const system_health_incident_signal_t,
    mut normalized: *const ::core::ffi::c_char,
    mut closed: bool,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO system_health_incidents(uuid,condition_code,subject,scope,state,severity,first_seen_at_ms,last_seen_at_ms,closed_at_ms,last_observation_json,alert_event_id,recovery_event_id,reconciliation_state,boot_id,run_id) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            condition_code,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw const (*signal).subject as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            system_health_scope_name((*signal).scope),
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            if closed as ::core::ffi::c_int != 0 {
                b"closed\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"open\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            severity_name((*signal).severity),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(
            statement,
            7 as ::core::ffi::c_int,
            (*signal).observed_at_ms as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            8 as ::core::ffi::c_int,
            (*signal).observed_at_ms as sqlite3_int64,
        );
        if closed {
            sqlite3_bind_int64(
                statement,
                9 as ::core::ffi::c_int,
                (*signal).observed_at_ms as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(statement, 9 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            10 as ::core::ffi::c_int,
            normalized,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*signal).event_id[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                11 as ::core::ffi::c_int,
                &raw const (*signal).event_id as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 11 as ::core::ffi::c_int);
        }
        sqlite3_bind_null(statement, 12 as ::core::ffi::c_int);
        sqlite3_bind_text(
            statement,
            13 as ::core::ffi::c_int,
            reconciliation_name((*signal).reconciliation),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            14 as ::core::ffi::c_int,
            &raw const (*signal).boot_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            15 as ::core::ffi::c_int,
            &raw const (*signal).run_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
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
unsafe extern "C" fn insert_transition_locked(
    mut database: *mut sqlite3,
    mut transition_uuid: *const ::core::ffi::c_char,
    mut incident_uuid: *const ::core::ffi::c_char,
    mut action: system_health_incident_action_t,
    mut from_state: *const ::core::ffi::c_char,
    mut to_state: *const ::core::ffi::c_char,
    mut severity: system_health_severity_t,
    mut signal: *const system_health_incident_signal_t,
    mut normalized: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO system_health_incident_transitions(transition_uuid,incident_uuid,kind,from_state,to_state,severity,observed_at_ms,safe_observation_json,event_id,reconciliation_state,boot_id,run_id) VALUES(?,?,?,?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            transition_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            incident_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            action_name(action),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        if !from_state.is_null() {
            sqlite3_bind_text(
                statement,
                4 as ::core::ffi::c_int,
                from_state,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
        } else {
            sqlite3_bind_null(statement, 4 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            to_state,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            severity_name(severity),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(
            statement,
            7 as ::core::ffi::c_int,
            (*signal).observed_at_ms as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            normalized,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*signal).event_id[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                9 as ::core::ffi::c_int,
                &raw const (*signal).event_id as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 9 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            10 as ::core::ffi::c_int,
            reconciliation_name((*signal).reconciliation),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            11 as ::core::ffi::c_int,
            &raw const (*signal).boot_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            12 as ::core::ffi::c_int,
            &raw const (*signal).run_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
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
unsafe extern "C" fn update_resume_locked(
    mut database: *mut sqlite3,
    mut existing: *const system_health_incident_record_t,
    mut signal: *const system_health_incident_signal_t,
    mut normalized: *const ::core::ffi::c_char,
    mut reopen: bool,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char = b"UPDATE system_health_incidents SET state=?,last_seen_at_ms=?,last_observation_json=?,alert_event_id=COALESCE(?,alert_event_id),reconciliation_state=?,boot_id=?,run_id=?,revision=revision+1 WHERE uuid=? AND revision=?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            if reopen as ::core::ffi::c_int != 0 {
                b"open\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                state_name((*existing).state)
            },
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(
            statement,
            2 as ::core::ffi::c_int,
            (*signal).observed_at_ms as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            normalized,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*signal).event_id[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                4 as ::core::ffi::c_int,
                &raw const (*signal).event_id as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 4 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            reconciliation_name((*signal).reconciliation),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            &raw const (*signal).boot_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            7 as ::core::ffi::c_int,
            &raw const (*signal).run_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            &raw const (*existing).uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            9 as ::core::ffi::c_int,
            (*existing).revision as sqlite3_int64,
        );
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(database)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if result == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn update_transition_locked(
    mut database: *mut sqlite3,
    mut existing: *const system_health_incident_record_t,
    mut action: system_health_incident_action_t,
    mut signal: *const system_health_incident_signal_t,
    mut normalized: *const ::core::ffi::c_char,
    mut target_state: system_health_state_t,
    mut target_severity: system_health_severity_t,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char = b"UPDATE system_health_incidents SET state=?,severity=?,last_seen_at_ms=?,closed_at_ms=?,last_observation_json=?,alert_event_id=CASE WHEN ?=0 THEN COALESCE(?,alert_event_id) ELSE alert_event_id END,recovery_event_id=CASE WHEN ?=1 THEN COALESCE(?,recovery_event_id) ELSE recovery_event_id END,reconciliation_state=?,boot_id=?,run_id=?,revision=revision+1 WHERE uuid=? AND revision=?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut recovery: bool =
        action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            state_name(target_state),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            severity_name(target_severity),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(
            statement,
            3 as ::core::ffi::c_int,
            (*signal).observed_at_ms as sqlite3_int64,
        );
        if recovery {
            sqlite3_bind_int64(
                statement,
                4 as ::core::ffi::c_int,
                (*signal).observed_at_ms as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(statement, 4 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            normalized,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            6 as ::core::ffi::c_int,
            if recovery as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        if (*signal).event_id[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                7 as ::core::ffi::c_int,
                &raw const (*signal).event_id as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 7 as ::core::ffi::c_int);
        }
        sqlite3_bind_int(
            statement,
            8 as ::core::ffi::c_int,
            if recovery as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        if (*signal).event_id[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                9 as ::core::ffi::c_int,
                &raw const (*signal).event_id as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 9 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            10 as ::core::ffi::c_int,
            reconciliation_name((*signal).reconciliation),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            11 as ::core::ffi::c_int,
            &raw const (*signal).boot_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            12 as ::core::ffi::c_int,
            &raw const (*signal).run_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            13 as ::core::ffi::c_int,
            &raw const (*existing).uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            14 as ::core::ffi::c_int,
            (*existing).revision as sqlite3_int64,
        );
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(database)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if result == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_incident_apply(
    mut action: system_health_incident_action_t,
    mut signal: *const system_health_incident_signal_t,
    mut incident_out: *mut system_health_incident_record_t,
) -> db_system_health_result_t {
    let mut target_state: system_health_state_t =
        system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN;
    let mut target_severity: system_health_severity_t =
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE;
    let mut normalized: [::core::ffi::c_char; 2049] = [0; 2049];
    if incident_out.is_null()
        || !valid_signal(
            action,
            signal,
            &raw mut normalized as *mut ::core::ffi::c_char,
        )
    {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_INVALID;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    let mut condition_code: *const ::core::ffi::c_char =
        system_health_condition_code((*signal).condition);
    let mut incident_uuid: [::core::ffi::c_char; 37] = [0; 37];
    let mut transition_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if lightnvr_uuid_generate_v4(&raw mut incident_uuid as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
        || lightnvr_uuid_generate_v4(&raw mut transition_uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    pthread_mutex_lock(mutex);
    if !transaction_begin(database) {
        pthread_mutex_unlock(mutex);
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    let mut existing: system_health_incident_record_t = system_health_incident_record_t {
        uuid: [0; 37],
        condition_code: [0; 64],
        subject: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
        severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        first_seen_at_ms: 0,
        last_seen_at_ms: 0,
        closed_at_ms: 0,
        observation_json: [0; 2049],
        alert_event_id: [0; 37],
        recovery_event_id: [0; 37],
        reconciliation: system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE,
        boot_id: [0; 64],
        run_id: [0; 37],
        revision: 0,
    };
    let mut found: ::core::ffi::c_int = get_active_locked(
        database,
        condition_code,
        &raw const (*signal).subject as *const ::core::ffi::c_char,
        &raw mut existing,
    );
    let mut outcome: db_system_health_result_t = db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    let mut success: bool = r#false != 0;
    '_finish: {
        if found >= 0 as ::core::ffi::c_int {
            if action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_ONE_SHOT.0 {
                if !(insert_incident_locked(
                    database,
                    &raw mut incident_uuid as *mut ::core::ffi::c_char,
                    condition_code,
                    signal,
                    &raw mut normalized as *mut ::core::ffi::c_char,
                    r#true != 0,
                ) != 0 as ::core::ffi::c_int
                    || insert_transition_locked(
                        database,
                        &raw mut transition_uuid as *mut ::core::ffi::c_char,
                        &raw mut incident_uuid as *mut ::core::ffi::c_char,
                        action,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"closed\0".as_ptr() as *const ::core::ffi::c_char,
                        (*signal).severity,
                        signal,
                        &raw mut normalized as *mut ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int)
                {
                    success = get_uuid_locked(
                        database,
                        &raw mut incident_uuid as *mut ::core::ffi::c_char,
                        incident_out,
                    ) == 1 as ::core::ffi::c_int;
                    outcome = db_system_health_result_t(if success as ::core::ffi::c_int != 0 {
                        db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
                    } else {
                        db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR.0
                    });
                }
            } else if action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_OPEN.0 {
                if found == 0 as ::core::ffi::c_int {
                    if !(insert_incident_locked(
                        database,
                        &raw mut incident_uuid as *mut ::core::ffi::c_char,
                        condition_code,
                        signal,
                        &raw mut normalized as *mut ::core::ffi::c_char,
                        r#false != 0,
                    ) != 0 as ::core::ffi::c_int
                        || insert_transition_locked(
                            database,
                            &raw mut transition_uuid as *mut ::core::ffi::c_char,
                            &raw mut incident_uuid as *mut ::core::ffi::c_char,
                            action,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"open\0".as_ptr() as *const ::core::ffi::c_char,
                            (*signal).severity,
                            signal,
                            &raw mut normalized as *mut ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int)
                    {
                        success = get_uuid_locked(
                            database,
                            &raw mut incident_uuid as *mut ::core::ffi::c_char,
                            incident_out,
                        ) == 1 as ::core::ffi::c_int;
                        outcome =
                            db_system_health_result_t(if success as ::core::ffi::c_int != 0 {
                                db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
                            } else {
                                db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR.0
                            });
                    }
                } else if (*signal).observed_at_ms < existing.last_seen_at_ms
                    || (*signal).scope.0 != existing.scope.0
                    || (*signal).severity.0 != existing.severity.0
                {
                    outcome = db_system_health_result_t::DB_SYSTEM_HEALTH_CONFLICT;
                } else {
                    let mut reopen: bool =
                        existing.state.0 == system_health_state_t::SYSTEM_HEALTH_STATE_RECOVERING.0;
                    if update_resume_locked(
                        database,
                        &raw mut existing,
                        signal,
                        &raw mut normalized as *mut ::core::ffi::c_char,
                        reopen,
                    ) == 0 as ::core::ffi::c_int
                    {
                        if !(reopen as ::core::ffi::c_int != 0
                            && insert_transition_locked(
                                database,
                                &raw mut transition_uuid as *mut ::core::ffi::c_char,
                                &raw mut existing.uuid as *mut ::core::ffi::c_char,
                                system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE,
                                b"recovering\0".as_ptr() as *const ::core::ffi::c_char,
                                b"open\0".as_ptr() as *const ::core::ffi::c_char,
                                existing.severity,
                                signal,
                                &raw mut normalized as *mut ::core::ffi::c_char,
                            ) != 0 as ::core::ffi::c_int)
                        {
                            success = get_uuid_locked(
                                database,
                                &raw mut existing.uuid as *mut ::core::ffi::c_char,
                                incident_out,
                            ) == 1 as ::core::ffi::c_int;
                            outcome = db_system_health_result_t(
                                if success as ::core::ffi::c_int != 0 {
                                    db_system_health_result_t::DB_SYSTEM_HEALTH_RESUMED.0
                                } else {
                                    db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR.0
                                },
                            );
                        }
                    }
                }
            } else if found == 0 as ::core::ffi::c_int {
                outcome = db_system_health_result_t::DB_SYSTEM_HEALTH_NOT_FOUND;
            } else if (*signal).observed_at_ms < existing.last_seen_at_ms
                || (*signal).scope.0 != existing.scope.0
            {
                outcome = db_system_health_result_t::DB_SYSTEM_HEALTH_CONFLICT;
            } else {
                target_state = existing.state;
                target_severity = existing.severity;
                if action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_ESCALATE.0 {
                    if (*signal).severity.0 <= existing.severity.0 {
                        outcome = db_system_health_result_t::DB_SYSTEM_HEALTH_CONFLICT;
                        break '_finish;
                    } else {
                        target_state = system_health_state_t::SYSTEM_HEALTH_STATE_OPEN;
                        target_severity = (*signal).severity;
                    }
                } else if action.0
                    == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE.0
                {
                    if (*signal).severity.0 != existing.severity.0 {
                        outcome = db_system_health_result_t::DB_SYSTEM_HEALTH_CONFLICT;
                        break '_finish;
                    } else {
                        target_state = (*signal).state;
                    }
                } else if action.0
                    == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0
                {
                    if (*signal).severity.0
                        != system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
                        && (*signal).severity.0 != existing.severity.0
                    {
                        outcome = db_system_health_result_t::DB_SYSTEM_HEALTH_CONFLICT;
                        break '_finish;
                    } else {
                        target_state = system_health_state_t::SYSTEM_HEALTH_STATE_CLOSED;
                    }
                } else {
                    outcome = db_system_health_result_t::DB_SYSTEM_HEALTH_INVALID;
                    break '_finish;
                }
                if !(update_transition_locked(
                    database,
                    &raw mut existing,
                    action,
                    signal,
                    &raw mut normalized as *mut ::core::ffi::c_char,
                    target_state,
                    target_severity,
                ) != 0 as ::core::ffi::c_int
                    || insert_transition_locked(
                        database,
                        &raw mut transition_uuid as *mut ::core::ffi::c_char,
                        &raw mut existing.uuid as *mut ::core::ffi::c_char,
                        action,
                        state_name(existing.state),
                        state_name(target_state),
                        target_severity,
                        signal,
                        &raw mut normalized as *mut ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int)
                {
                    success = get_uuid_locked(
                        database,
                        &raw mut existing.uuid as *mut ::core::ffi::c_char,
                        incident_out,
                    ) == 1 as ::core::ffi::c_int;
                    outcome = db_system_health_result_t(if success as ::core::ffi::c_int != 0 {
                        db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
                    } else {
                        db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR.0
                    });
                }
            }
        }
    }
    success = success as ::core::ffi::c_int != 0
        && transaction_finish(database, r#true != 0) as ::core::ffi::c_int != 0;
    if !success {
        transaction_finish(database, r#false != 0);
    }
    pthread_mutex_unlock(mutex);
    return db_system_health_result_t(if success as ::core::ffi::c_int != 0 {
        outcome.0
    } else if outcome.0 == db_system_health_result_t::DB_SYSTEM_HEALTH_CONFLICT.0
        || outcome.0 == db_system_health_result_t::DB_SYSTEM_HEALTH_NOT_FOUND.0
        || outcome.0 == db_system_health_result_t::DB_SYSTEM_HEALTH_INVALID.0
    {
        outcome.0
    } else {
        db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_incident_get_active(
    mut condition: system_health_condition_t,
    mut subject: *const ::core::ffi::c_char,
    mut incident_out: *mut system_health_incident_record_t,
) -> db_system_health_result_t {
    let mut code: *const ::core::ffi::c_char = system_health_condition_code(condition);
    if code.is_null()
        || !valid_logical_id(subject, SYSTEM_HEALTH_INCIDENT_SUBJECT_MAX as size_t)
        || incident_out.is_null()
    {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_INVALID;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut found: ::core::ffi::c_int = get_active_locked(database, code, subject, incident_out);
    pthread_mutex_unlock(mutex);
    return db_system_health_result_t(if found > 0 as ::core::ffi::c_int {
        db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
    } else if found == 0 as ::core::ffi::c_int {
        db_system_health_result_t::DB_SYSTEM_HEALTH_NOT_FOUND.0
    } else {
        db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_incident_get_uuid(
    mut incident_uuid: *const ::core::ffi::c_char,
    mut incident_out: *mut system_health_incident_record_t,
) -> db_system_health_result_t {
    if !lightnvr_uuid_is_valid(incident_uuid) || incident_out.is_null() {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_INVALID;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut found: ::core::ffi::c_int = get_uuid_locked(database, incident_uuid, incident_out);
    pthread_mutex_unlock(mutex);
    return db_system_health_result_t(if found > 0 as ::core::ffi::c_int {
        db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
    } else if found == 0 as ::core::ffi::c_int {
        db_system_health_result_t::DB_SYSTEM_HEALTH_NOT_FOUND.0
    } else {
        db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_incident_list(
    mut include_closed: bool,
    mut cursor: *const system_health_incident_cursor_t,
    mut incidents: *mut system_health_incident_record_t,
    mut max_count: ::core::ffi::c_int,
    mut next_cursor: *mut system_health_incident_cursor_t,
) -> ::core::ffi::c_int {
    if incidents.is_null()
        || next_cursor.is_null()
        || max_count < 1 as ::core::ffi::c_int
        || max_count > SYSTEM_HEALTH_INCIDENT_PAGE_MAX
        || !cursor.is_null()
            && (*cursor).valid as ::core::ffi::c_int != 0
            && ((*cursor).last_seen_at_ms <= 0 as int64_t
                || !lightnvr_uuid_is_valid(&raw const (*cursor).uuid as *const ::core::ffi::c_char))
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT uuid,condition_code,subject,scope,state,severity,first_seen_at_ms,last_seen_at_ms,COALESCE(closed_at_ms,0),last_observation_json,COALESCE(alert_event_id,''),COALESCE(recovery_event_id,''),reconciliation_state,boot_id,run_id,revision FROM system_health_incidents WHERE (? OR state<>'closed') AND (?=0 OR last_seen_at_ms<? OR (last_seen_at_ms=? AND uuid<?)) ORDER BY last_seen_at_ms DESC,uuid DESC LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut has_cursor: bool = !cursor.is_null() && (*cursor).valid as ::core::ffi::c_int != 0;
    if result == SQLITE_OK {
        sqlite3_bind_int(
            statement,
            1 as ::core::ffi::c_int,
            if include_closed as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int(
            statement,
            2 as ::core::ffi::c_int,
            if has_cursor as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int64(
            statement,
            3 as ::core::ffi::c_int,
            (if has_cursor as ::core::ffi::c_int != 0 {
                (*cursor).last_seen_at_ms
            } else {
                0 as int64_t
            }) as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            4 as ::core::ffi::c_int,
            (if has_cursor as ::core::ffi::c_int != 0 {
                (*cursor).last_seen_at_ms
            } else {
                0 as int64_t
            }) as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            if has_cursor as ::core::ffi::c_int != 0 {
                &raw const (*cursor).uuid as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            6 as ::core::ffi::c_int,
            max_count + 1 as ::core::ffi::c_int,
        );
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut more: bool = r#false != 0;
    while result == SQLITE_OK || result == SQLITE_ROW {
        result = sqlite3_step(statement);
        if result != SQLITE_ROW {
            break;
        }
        let mut row: system_health_incident_record_t = system_health_incident_record_t {
            uuid: [0; 37],
            condition_code: [0; 64],
            subject: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
            severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
            first_seen_at_ms: 0,
            last_seen_at_ms: 0,
            closed_at_ms: 0,
            observation_json: [0; 2049],
            alert_event_id: [0; 37],
            recovery_event_id: [0; 37],
            reconciliation: system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE,
            boot_id: [0; 64],
            run_id: [0; 37],
            revision: 0,
        };
        if !populate_incident(statement, &raw mut row) {
            result = SQLITE_ERROR;
            break;
        } else if count == max_count {
            more = r#true != 0;
            break;
        } else {
            let c2rust_fresh0 = count;
            count += 1;
            *incidents.offset(c2rust_fresh0 as isize) = row;
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    memset(
        next_cursor as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_incident_cursor_t>(),
    );
    if more as ::core::ffi::c_int != 0 && count > 0 as ::core::ffi::c_int {
        (*next_cursor).valid = r#true != 0;
        (*next_cursor).last_seen_at_ms =
            (*incidents.offset((count - 1 as ::core::ffi::c_int) as isize)).last_seen_at_ms;
        safe_strcpy(
            &raw mut (*next_cursor).uuid as *mut ::core::ffi::c_char,
            &raw mut (*incidents.offset((count - 1 as ::core::ffi::c_int) as isize)).uuid
                as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    }
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_DONE || more as ::core::ffi::c_int != 0 {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn populate_transition(
    mut statement: *mut sqlite3_stmt,
    mut transition: *mut system_health_incident_transition_t,
) -> bool {
    let mut action_valid: bool = false;
    let mut from_valid: bool = r#true != 0;
    let mut to_valid: bool = false;
    memset(
        transition as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_incident_transition_t>(),
    );
    (*transition).id = sqlite3_column_int64(statement, 0 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*transition).transition_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*transition).incident_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        2 as ::core::ffi::c_int,
    );
    (*transition).action = parse_action(
        sqlite3_column_text(statement, 3 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
        &raw mut action_valid,
    );
    let mut from: *const ::core::ffi::c_char =
        sqlite3_column_text(statement, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    (*transition).from_state_valid =
        !from.is_null() && *from.offset(0isize) as ::core::ffi::c_int != 0;
    if (*transition).from_state_valid {
        (*transition).from_state = parse_state(from, &raw mut from_valid);
    }
    (*transition).to_state = parse_state(
        sqlite3_column_text(statement, 5 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
        &raw mut to_valid,
    );
    (*transition).severity = parse_severity(
        sqlite3_column_text(statement, 6 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
    );
    (*transition).observed_at_ms =
        sqlite3_column_int64(statement, 7 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*transition).observation_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2049]>(),
        statement,
        8 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*transition).event_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        9 as ::core::ffi::c_int,
    );
    (*transition).reconciliation = parse_reconciliation(sqlite3_column_text(
        statement,
        10 as ::core::ffi::c_int,
    ) as *const ::core::ffi::c_char);
    copy_column(
        &raw mut (*transition).boot_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        11 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*transition).run_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        12 as ::core::ffi::c_int,
    );
    return action_valid as ::core::ffi::c_int != 0
        && from_valid as ::core::ffi::c_int != 0
        && to_valid as ::core::ffi::c_int != 0
        && (*transition).severity.0 != system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_transition_list(
    mut incident_uuid: *const ::core::ffi::c_char,
    mut cursor: *const system_health_transition_cursor_t,
    mut transitions: *mut system_health_incident_transition_t,
    mut max_count: ::core::ffi::c_int,
    mut next_cursor: *mut system_health_transition_cursor_t,
) -> ::core::ffi::c_int {
    if !lightnvr_uuid_is_valid(incident_uuid)
        || transitions.is_null()
        || next_cursor.is_null()
        || max_count < 1 as ::core::ffi::c_int
        || max_count > SYSTEM_HEALTH_INCIDENT_PAGE_MAX
        || !cursor.is_null()
            && (*cursor).valid as ::core::ffi::c_int != 0
            && ((*cursor).observed_at_ms <= 0 as int64_t || (*cursor).id <= 0 as int64_t)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT id,transition_uuid,incident_uuid,kind,COALESCE(from_state,''),to_state,severity,observed_at_ms,safe_observation_json,COALESCE(event_id,''),reconciliation_state,boot_id,run_id FROM system_health_incident_transitions WHERE incident_uuid=? AND (?=0 OR observed_at_ms<? OR (observed_at_ms=? AND id<?)) ORDER BY observed_at_ms DESC,id DESC LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut has_cursor: bool = !cursor.is_null() && (*cursor).valid as ::core::ffi::c_int != 0;
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            incident_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            2 as ::core::ffi::c_int,
            if has_cursor as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int64(
            statement,
            3 as ::core::ffi::c_int,
            (if has_cursor as ::core::ffi::c_int != 0 {
                (*cursor).observed_at_ms
            } else {
                0 as int64_t
            }) as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            4 as ::core::ffi::c_int,
            (if has_cursor as ::core::ffi::c_int != 0 {
                (*cursor).observed_at_ms
            } else {
                0 as int64_t
            }) as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            5 as ::core::ffi::c_int,
            (if has_cursor as ::core::ffi::c_int != 0 {
                (*cursor).id
            } else {
                0 as int64_t
            }) as sqlite3_int64,
        );
        sqlite3_bind_int(
            statement,
            6 as ::core::ffi::c_int,
            max_count + 1 as ::core::ffi::c_int,
        );
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut more: bool = r#false != 0;
    while result == SQLITE_OK || result == SQLITE_ROW {
        result = sqlite3_step(statement);
        if result != SQLITE_ROW {
            break;
        }
        let mut row: system_health_incident_transition_t = system_health_incident_transition_t {
            id: 0,
            transition_uuid: [0; 37],
            incident_uuid: [0; 37],
            action: system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_OPEN,
            from_state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
            from_state_valid: false,
            to_state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
            severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
            observed_at_ms: 0,
            observation_json: [0; 2049],
            event_id: [0; 37],
            reconciliation: system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE,
            boot_id: [0; 64],
            run_id: [0; 37],
        };
        if !populate_transition(statement, &raw mut row) {
            result = SQLITE_ERROR;
            break;
        } else if count == max_count {
            more = r#true != 0;
            break;
        } else {
            let c2rust_fresh1 = count;
            count += 1;
            *transitions.offset(c2rust_fresh1 as isize) = row;
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    memset(
        next_cursor as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_transition_cursor_t>(),
    );
    if more as ::core::ffi::c_int != 0 && count > 0 as ::core::ffi::c_int {
        (*next_cursor).valid = r#true != 0;
        (*next_cursor).observed_at_ms =
            (*transitions.offset((count - 1 as ::core::ffi::c_int) as isize)).observed_at_ms;
        (*next_cursor).id = (*transitions.offset((count - 1 as ::core::ffi::c_int) as isize)).id;
    }
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_DONE || more as ::core::ffi::c_int != 0 {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_transition_list_pending(
    mut after_id: int64_t,
    mut transitions: *mut system_health_incident_transition_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if after_id < 0 as int64_t
        || transitions.is_null()
        || max_count < 1 as ::core::ffi::c_int
        || max_count > SYSTEM_HEALTH_INCIDENT_PAGE_MAX
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT id,transition_uuid,incident_uuid,kind,COALESCE(from_state,''),to_state,severity,observed_at_ms,safe_observation_json,COALESCE(event_id,''),reconciliation_state,boot_id,run_id FROM system_health_incident_transitions WHERE id>? AND event_id IS NOT NULL AND reconciliation_state IN ('alert_pending','recovery_pending') ORDER BY id ASC LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_int64(
            statement,
            1 as ::core::ffi::c_int,
            after_id as sqlite3_int64,
        );
        sqlite3_bind_int(statement, 2 as ::core::ffi::c_int, max_count);
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while result == SQLITE_OK || result == SQLITE_ROW {
        result = sqlite3_step(statement);
        if result != SQLITE_ROW {
            break;
        }
        let c2rust_fresh2 = count;
        count += 1;
        if populate_transition(statement, transitions.offset(c2rust_fresh2 as isize)) {
            continue;
        }
        result = SQLITE_ERROR;
        break;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_DONE {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_transition_set_reconciliation(
    mut incident_uuid: *const ::core::ffi::c_char,
    mut event_id: *const ::core::ffi::c_char,
    mut reconciliation: system_health_reconciliation_t,
) -> db_system_health_result_t {
    if !lightnvr_uuid_is_valid(incident_uuid)
        || !lightnvr_uuid_is_valid(event_id)
        || reconciliation_name(reconciliation).is_null()
    {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_INVALID;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    let mut transition_sql: *const ::core::ffi::c_char = b"UPDATE system_health_incident_transitions SET reconciliation_state=? WHERE incident_uuid=? AND event_id=?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    if !transaction_begin(database) {
        pthread_mutex_unlock(mutex);
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        transition_sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            reconciliation_name(reconciliation),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            incident_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            event_id,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(database)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if result == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
        let mut incident_sql: *const ::core::ffi::c_char = b"UPDATE system_health_incidents SET reconciliation_state=COALESCE((SELECT reconciliation_state FROM system_health_incident_transitions WHERE incident_uuid=? AND reconciliation_state IN ('alert_pending','recovery_pending') ORDER BY id DESC LIMIT 1),?),revision=revision+1 WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        result = sqlite3_prepare_v2(
            database,
            incident_sql,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                incident_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
                reconciliation_name(reconciliation),
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            sqlite3_bind_text(
                statement,
                3 as ::core::ffi::c_int,
                incident_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            result = sqlite3_step(statement);
        }
        changed = if result == SQLITE_DONE {
            sqlite3_changes(database)
        } else {
            0 as ::core::ffi::c_int
        };
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
    }
    let mut success: bool = result == SQLITE_DONE
        && changed == 1 as ::core::ffi::c_int
        && transaction_finish(database, r#true != 0) as ::core::ffi::c_int != 0;
    if !success {
        transaction_finish(database, r#false != 0);
    }
    pthread_mutex_unlock(mutex);
    return db_system_health_result_t(if success as ::core::ffi::c_int != 0 {
        db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
    } else if result == SQLITE_DONE && changed == 0 as ::core::ffi::c_int {
        db_system_health_result_t::DB_SYSTEM_HEALTH_NOT_FOUND.0
    } else {
        db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_incident_set_reconciliation(
    mut incident_uuid: *const ::core::ffi::c_char,
    mut recovery_event: bool,
    mut event_id: *const ::core::ffi::c_char,
    mut reconciliation: system_health_reconciliation_t,
) -> db_system_health_result_t {
    if !lightnvr_uuid_is_valid(incident_uuid)
        || !event_id.is_null()
            && *event_id.offset(0isize) as ::core::ffi::c_int != 0
            && !lightnvr_uuid_is_valid(event_id)
        || reconciliation_name(reconciliation).is_null()
    {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_INVALID;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = if recovery_event as ::core::ffi::c_int != 0 {
        b"UPDATE system_health_incidents SET recovery_event_id=?,reconciliation_state=?,revision=revision+1 WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char
    } else {
        b"UPDATE system_health_incidents SET alert_event_id=?,reconciliation_state=?,revision=revision+1 WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char
    };
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        if !event_id.is_null() && *event_id.offset(0isize) as ::core::ffi::c_int != 0 {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                event_id,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 1 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            reconciliation_name(reconciliation),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            incident_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(database)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return db_system_health_result_t(if result != SQLITE_DONE {
        db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR.0
    } else if changed == 1 as ::core::ffi::c_int {
        db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
    } else {
        db_system_health_result_t::DB_SYSTEM_HEALTH_NOT_FOUND.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_transition_prune(
    mut closed_before_ms: int64_t,
    mut limit: ::core::ffi::c_int,
    mut deleted_count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if closed_before_ms <= 0 as int64_t
        || limit < 1 as ::core::ffi::c_int
        || limit > SYSTEM_HEALTH_TRANSITION_RETENTION_BATCH_MAX
        || deleted_count.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"DELETE FROM system_health_incident_transitions WHERE id IN (SELECT t.id FROM system_health_incident_transitions t JOIN system_health_incidents i ON i.uuid=t.incident_uuid WHERE i.state='closed' AND t.observed_at_ms<? ORDER BY t.observed_at_ms,t.id LIMIT ?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_int64(
            statement,
            1 as ::core::ffi::c_int,
            closed_before_ms as sqlite3_int64,
        );
        sqlite3_bind_int(statement, 2 as ::core::ffi::c_int, limit);
        result = sqlite3_step(statement);
    }
    *deleted_count = if result == SQLITE_DONE {
        sqlite3_changes(database)
    } else {
        0 as ::core::ffi::c_int
    };
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
unsafe extern "C" fn populate_run(
    mut statement: *mut sqlite3_stmt,
    mut run: *mut system_health_process_run_t,
) -> bool {
    memset(
        run as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_process_run_t>(),
    );
    copy_column(
        &raw mut (*run).run_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*run).boot_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        1 as ::core::ffi::c_int,
    );
    (*run).started_at_ms = sqlite3_column_int64(statement, 2 as ::core::ffi::c_int) as int64_t;
    (*run).closed_at_ms = sqlite3_column_int64(statement, 3 as ::core::ffi::c_int) as int64_t;
    (*run).clean_close =
        sqlite3_column_int(statement, 4 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    return lightnvr_uuid_is_valid(&raw mut (*run).run_id as *mut ::core::ffi::c_char)
        as ::core::ffi::c_int
        != 0
        && valid_logical_id(
            &raw mut (*run).boot_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        ) as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn latest_run_locked(
    mut database: *mut sqlite3,
    mut run: *mut system_health_process_run_t,
) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        b"SELECT run_id,boot_id,started_at_ms,COALESCE(closed_at_ms,0),clean_close FROM system_health_process_runs ORDER BY started_at_ms DESC,run_id DESC LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        result = sqlite3_step(statement);
    }
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if result == SQLITE_ROW {
        found = if populate_run(statement, run) as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    } else if result != SQLITE_DONE {
        found = -1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return found;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_run_open(
    mut run_id: *const ::core::ffi::c_char,
    mut boot_id: *const ::core::ffi::c_char,
    mut started_at_ms: int64_t,
    mut run_out: *mut system_health_process_run_t,
    mut previous_out: *mut system_health_process_run_t,
    mut had_previous: *mut bool,
) -> db_system_health_result_t {
    if run_out.is_null()
        || previous_out.is_null()
        || had_previous.is_null()
        || !valid_logical_id(boot_id, SYSTEM_HEALTH_INCIDENT_BOOT_ID_MAX as size_t)
        || started_at_ms <= 0 as int64_t
        || !run_id.is_null() && !lightnvr_uuid_is_valid(run_id)
    {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_INVALID;
    }
    let mut generated: [::core::ffi::c_char; 37] = [0; 37];
    if run_id.is_null() {
        if lightnvr_uuid_generate_v4(&raw mut generated as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
        }
        run_id = &raw mut generated as *mut ::core::ffi::c_char;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    pthread_mutex_lock(mutex);
    if !transaction_begin(database) {
        pthread_mutex_unlock(mutex);
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    let mut previous: ::core::ffi::c_int = latest_run_locked(database, previous_out);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = if previous < 0 as ::core::ffi::c_int {
        SQLITE_ERROR
    } else {
        sqlite3_prepare_v2(
            database,
            b"INSERT INTO system_health_process_runs(run_id,boot_id,started_at_ms) VALUES(?,?,?);\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        )
    };
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            run_id,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            boot_id,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            3 as ::core::ffi::c_int,
            started_at_ms as sqlite3_int64,
        );
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    let mut success: bool = result == SQLITE_DONE;
    if success {
        safe_strcpy(
            &raw mut (*run_out).run_id as *mut ::core::ffi::c_char,
            run_id,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*run_out).boot_id as *mut ::core::ffi::c_char,
            boot_id,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        (*run_out).started_at_ms = started_at_ms;
        (*run_out).closed_at_ms = 0 as int64_t;
        (*run_out).clean_close = r#false != 0;
    }
    success = transaction_finish(database, success);
    pthread_mutex_unlock(mutex);
    if !success {
        return db_system_health_result_t(if result == SQLITE_CONSTRAINT {
            db_system_health_result_t::DB_SYSTEM_HEALTH_CONFLICT.0
        } else {
            db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR.0
        });
    }
    *had_previous = previous > 0 as ::core::ffi::c_int;
    if !*had_previous {
        memset(
            previous_out as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<system_health_process_run_t>(),
        );
    }
    return db_system_health_result_t::DB_SYSTEM_HEALTH_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_run_close(
    mut run_id: *const ::core::ffi::c_char,
    mut closed_at_ms: int64_t,
) -> db_system_health_result_t {
    if !lightnvr_uuid_is_valid(run_id) || closed_at_ms <= 0 as int64_t {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_INVALID;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    pthread_mutex_lock(mutex);
    if !transaction_begin(database) {
        pthread_mutex_unlock(mutex);
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        database,
        b"UPDATE system_health_process_runs SET closed_at_ms=?,clean_close=1 WHERE run_id=? AND clean_close=0 AND started_at_ms<=?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_int64(
            statement,
            1 as ::core::ffi::c_int,
            closed_at_ms as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            run_id,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            3 as ::core::ffi::c_int,
            closed_at_ms as sqlite3_int64,
        );
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(database)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    let mut success: bool = result == SQLITE_DONE;
    success = transaction_finish(database, success);
    pthread_mutex_unlock(mutex);
    if !success {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    return db_system_health_result_t(if changed == 1 as ::core::ffi::c_int {
        db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
    } else {
        db_system_health_result_t::DB_SYSTEM_HEALTH_NOT_FOUND.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_system_health_run_latest(
    mut run_out: *mut system_health_process_run_t,
) -> db_system_health_result_t {
    if run_out.is_null() {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_INVALID;
    }
    let mut database: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if database.is_null() || mutex.is_null() {
        return db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut found: ::core::ffi::c_int = latest_run_locked(database, run_out);
    pthread_mutex_unlock(mutex);
    return db_system_health_result_t(if found > 0 as ::core::ffi::c_int {
        db_system_health_result_t::DB_SYSTEM_HEALTH_OK.0
    } else if found == 0 as ::core::ffi::c_int {
        db_system_health_result_t::DB_SYSTEM_HEALTH_NOT_FOUND.0
    } else {
        db_system_health_result_t::DB_SYSTEM_HEALTH_ERROR.0
    });
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
