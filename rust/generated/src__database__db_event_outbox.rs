use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn event_registry_find(
        r#type: *const ::core::ffi::c_char,
    ) -> *const event_type_definition_t;
    unsafe fn event_envelope_serialize(
        event: *const event_envelope_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut ::core::ffi::c_char;
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
    unsafe fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_free(_: *mut ::core::ffi::c_void);
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
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
    unsafe fn sqlite3_column_bytes(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
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
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
pub struct event_severity_t(pub ::core::ffi::c_uint);
impl event_severity_t {
    pub const EVENT_SEVERITY_INFO: Self = Self(0);
    pub const EVENT_SEVERITY_WARNING: Self = Self(1);
    pub const EVENT_SEVERITY_ERROR: Self = Self(2);
    pub const EVENT_SEVERITY_CRITICAL: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_sensitivity_t(pub ::core::ffi::c_uint);
impl event_sensitivity_t {
    pub const EVENT_SENSITIVITY_OPERATIONAL: Self = Self(0);
    pub const EVENT_SENSITIVITY_INTERNAL: Self = Self(1);
    pub const EVENT_SENSITIVITY_RESTRICTED: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_media_policy_t(pub ::core::ffi::c_uint);
impl event_media_policy_t {
    pub const EVENT_MEDIA_FORBIDDEN: Self = Self(0);
    pub const EVENT_MEDIA_REFERENCE_ALLOWED: Self = Self(1);
    pub const EVENT_MEDIA_BINARY_OPT_IN: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_expected_rate_t(pub ::core::ffi::c_uint);
impl event_expected_rate_t {
    pub const EVENT_RATE_LOW: Self = Self(0);
    pub const EVENT_RATE_MEDIUM: Self = Self(1);
    pub const EVENT_RATE_HIGH: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_subject_kind_t(pub ::core::ffi::c_uint);
impl event_subject_kind_t {
    pub const EVENT_SUBJECT_CAMERA: Self = Self(0);
    pub const EVENT_SUBJECT_STORAGE: Self = Self(1);
    pub const EVENT_SUBJECT_SYSTEM: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_type_definition_t {
    pub r#type: *const ::core::ffi::c_char,
    pub family: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub severity: event_severity_t,
    pub dynamic_severity: bool,
    pub minimum_severity: event_severity_t,
    pub maximum_severity: event_severity_t,
    pub sensitivity: event_sensitivity_t,
    pub media_policy: event_media_policy_t,
    pub expected_rate: event_expected_rate_t,
    pub subject_kind: event_subject_kind_t,
    pub default_expiry_seconds: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_envelope_t {
    pub specversion: [::core::ffi::c_char; 4],
    pub id: [::core::ffi::c_char; 37],
    pub r#type: [::core::ffi::c_char; 96],
    pub source: [::core::ffi::c_char; 128],
    pub subject: [::core::ffi::c_char; 96],
    pub time: [::core::ffi::c_char; 32],
    pub datacontenttype: [::core::ffi::c_char; 32],
    pub occurred_at: time_t,
    pub expires_at: time_t,
    pub severity: event_severity_t,
    pub data: *mut cJSON,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_outbox_enqueue_result_t(pub ::core::ffi::c_int);
impl event_outbox_enqueue_result_t {
    pub const EVENT_OUTBOX_ENQUEUED: Self = Self(0);
    pub const EVENT_OUTBOX_DUPLICATE: Self = Self(1);
    pub const EVENT_OUTBOX_FULL: Self = Self(2);
    pub const EVENT_OUTBOX_ERROR: Self = Self(-1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_outbox_limits_t {
    pub max_rows: int64_t,
    pub max_bytes: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_outbox_item_t {
    pub row_id: int64_t,
    pub event_id: [::core::ffi::c_char; 37],
    pub event_source: [::core::ffi::c_char; 128],
    pub event_type: [::core::ffi::c_char; 96],
    pub subject: [::core::ffi::c_char; 96],
    pub destination: [::core::ffi::c_char; 64],
    pub topic: [::core::ffi::c_char; 512],
    pub envelope_json: *mut ::core::ffi::c_char,
    pub envelope_bytes: size_t,
    pub severity: event_severity_t,
    pub attempt_count: ::core::ffi::c_int,
    pub next_attempt_at: int64_t,
    pub expires_at: int64_t,
    pub created_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_outbox_stats_t {
    pub total_rows: int64_t,
    pub total_bytes: int64_t,
    pub pending_rows: int64_t,
    pub delivering_rows: int64_t,
    pub delivered_rows: int64_t,
    pub dead_rows: int64_t,
    pub due_rows: int64_t,
    pub oldest_pending_at: int64_t,
}
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
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
pub const EVENT_TYPE_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const EVENT_ENVELOPE_MAX_BYTES: ::core::ffi::c_uint =
    (96 as ::core::ffi::c_uint).wrapping_mul(1024 as ::core::ffi::c_uint);
pub const EVENT_OUTBOX_DESTINATION_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_TOPIC_MAX: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_ERROR_MAX: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_DEFAULT_MAX_ROWS: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_DEFAULT_MAX_BYTES: ::core::ffi::c_longlong = 64 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong;
pub const EVENT_OUTBOX_DEFAULT_LEASE_SECONDS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"EventOutboxDB\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn valid_text(
    mut value: *const ::core::ffi::c_char,
    mut maximum: size_t,
    mut required: bool,
) -> bool {
    if value.is_null() || *value.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return !required;
    }
    if strlen(value) >= maximum {
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
unsafe extern "C" fn normalized_limits(
    mut limits: *const event_outbox_limits_t,
) -> event_outbox_limits_t {
    let mut result: event_outbox_limits_t = event_outbox_limits_t {
        max_rows: EVENT_OUTBOX_DEFAULT_MAX_ROWS as int64_t,
        max_bytes: EVENT_OUTBOX_DEFAULT_MAX_BYTES as int64_t,
    };
    if !limits.is_null() {
        if (*limits).max_rows > 0 as int64_t {
            result.max_rows = (*limits).max_rows;
        }
        if (*limits).max_bytes > 0 as int64_t {
            result.max_bytes = (*limits).max_bytes;
        }
    }
    return result;
}
unsafe extern "C" fn execute_sql(
    mut db: *mut sqlite3,
    mut sql: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut message: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int = sqlite3_exec(db, sql, None, NULL, &raw mut message);
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Outbox SQL failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if !message.is_null() {
                message
            } else {
                sqlite3_errmsg(db)
            },
        );
    }
    sqlite3_free(message as *mut ::core::ffi::c_void);
    return if rc == SQLITE_OK {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn rollback(mut db: *mut sqlite3) {
    sqlite3_exec(
        db,
        b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn capacity_locked(
    mut db: *mut sqlite3,
    mut rows: *mut int64_t,
    mut bytes: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT count(*),COALESCE(sum(envelope_bytes),0) FROM event_outbox;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW {
        *rows = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
        *bytes = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as int64_t;
        rc = SQLITE_DONE;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn delete_candidate_locked(
    mut db: *mut sqlite3,
    mut terminal: bool,
    mut incoming: event_severity_t,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char = if terminal as ::core::ffi::c_int != 0 {
        b"DELETE FROM event_outbox WHERE id=(SELECT id FROM event_outbox WHERE state IN ('delivered','dead') ORDER BY updated_at,id LIMIT 1);\0"
            .as_ptr() as *const ::core::ffi::c_char
    } else {
        b"DELETE FROM event_outbox WHERE id=(SELECT id FROM event_outbox WHERE state='pending' AND severity<? ORDER BY severity,created_at,id LIMIT 1);\0"
            .as_ptr() as *const ::core::ffi::c_char
    };
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK && !terminal {
        sqlite3_bind_int(
            stmt,
            1 as ::core::ffi::c_int,
            incoming.0 as ::core::ffi::c_int,
        );
    }
    if rc == SQLITE_OK {
        rc = sqlite3_step(stmt);
    }
    let mut changed: ::core::ffi::c_int = if rc == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        -1 as ::core::ffi::c_int
    };
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return changed;
}
unsafe extern "C" fn has_capacity(
    mut rows: int64_t,
    mut bytes: int64_t,
    mut payload_bytes: int64_t,
    mut limits: *const event_outbox_limits_t,
) -> bool {
    return rows < (*limits).max_rows
        && payload_bytes <= (*limits).max_bytes
        && bytes <= (*limits).max_bytes - payload_bytes;
}
unsafe extern "C" fn duplicate_locked(
    mut db: *mut sqlite3,
    mut event: *const event_envelope_t,
    mut destination: *const ::core::ffi::c_char,
    mut row_id: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT id FROM event_outbox WHERE event_source=? AND event_id=? AND destination=? LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw const (*event).source as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw const (*event).id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            destination,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if rc == SQLITE_ROW {
        if !row_id.is_null() {
            *row_id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
        }
        found = 1 as ::core::ffi::c_int;
    } else if rc != SQLITE_DONE {
        found = -1 as ::core::ffi::c_int;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return found;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_outbox_enqueue(
    mut event: *const event_envelope_t,
    mut destination: *const ::core::ffi::c_char,
    mut topic: *const ::core::ffi::c_char,
    mut limits: *const event_outbox_limits_t,
    mut row_id: *mut int64_t,
    mut shed_count: *mut ::core::ffi::c_int,
) -> event_outbox_enqueue_result_t {
    if !row_id.is_null() {
        *row_id = 0 as int64_t;
    }
    if !shed_count.is_null() {
        *shed_count = 0 as ::core::ffi::c_int;
    }
    if !valid_text(
        destination,
        EVENT_OUTBOX_DESTINATION_MAX as size_t,
        r#true != 0,
    ) || !valid_text(topic, EVENT_OUTBOX_TOPIC_MAX as size_t, r#true != 0)
    {
        return event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR;
    }
    let mut validation_error: [::core::ffi::c_char; 256] = [
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
    let mut serialized: *mut ::core::ffi::c_char = event_envelope_serialize(
        event,
        &raw mut validation_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if serialized.is_null() {
        return event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR;
    }
    let mut serialized_bytes: int64_t = strlen(serialized) as int64_t;
    let mut definition: *const event_type_definition_t =
        event_registry_find(&raw const (*event).r#type as *const ::core::ffi::c_char);
    let mut bounds: event_outbox_limits_t = normalized_limits(limits);
    if definition.is_null()
        || bounds.max_rows < 1 as int64_t
        || bounds.max_bytes < 1 as int64_t
        || serialized_bytes > bounds.max_bytes
    {
        free(serialized as *mut ::core::ffi::c_void);
        return event_outbox_enqueue_result_t::EVENT_OUTBOX_FULL;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        free(serialized as *mut ::core::ffi::c_void);
        return event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR;
    }
    pthread_mutex_lock(mutex);
    if execute_sql(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(mutex);
        free(serialized as *mut ::core::ffi::c_void);
        return event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR;
    }
    let mut duplicate: ::core::ffi::c_int = duplicate_locked(db, event, destination, row_id);
    if duplicate != 0 as ::core::ffi::c_int {
        let mut transaction_result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if duplicate < 0 as ::core::ffi::c_int {
            rollback(db);
        } else {
            transaction_result =
                execute_sql(db, b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char);
            if transaction_result != 0 as ::core::ffi::c_int {
                rollback(db);
            }
        }
        pthread_mutex_unlock(mutex);
        free(serialized as *mut ::core::ffi::c_void);
        return event_outbox_enqueue_result_t(
            if duplicate > 0 as ::core::ffi::c_int && transaction_result == 0 as ::core::ffi::c_int
            {
                event_outbox_enqueue_result_t::EVENT_OUTBOX_DUPLICATE.0
            } else {
                event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR.0
            },
        );
    }
    let mut rows: int64_t = 0 as int64_t;
    let mut bytes: int64_t = 0 as int64_t;
    if capacity_locked(db, &raw mut rows, &raw mut bytes) != 0 as ::core::ffi::c_int {
        rollback(db);
        pthread_mutex_unlock(mutex);
        free(serialized as *mut ::core::ffi::c_void);
        return event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR;
    }
    let mut shed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !has_capacity(rows, bytes, serialized_bytes, &raw mut bounds) {
        let mut changed: ::core::ffi::c_int =
            delete_candidate_locked(db, r#true != 0, (*event).severity);
        if changed == 0 as ::core::ffi::c_int
            && (*event).severity.0 >= event_severity_t::EVENT_SEVERITY_ERROR.0
        {
            changed = delete_candidate_locked(db, r#false != 0, (*event).severity);
            if changed > 0 as ::core::ffi::c_int {
                shed += changed;
            }
        }
        if changed < 0 as ::core::ffi::c_int
            || changed == 0 as ::core::ffi::c_int
            || capacity_locked(db, &raw mut rows, &raw mut bytes) != 0 as ::core::ffi::c_int
        {
            if changed < 0 as ::core::ffi::c_int {
                rollback(db);
                pthread_mutex_unlock(mutex);
                free(serialized as *mut ::core::ffi::c_void);
                return event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR;
            }
            rollback(db);
            pthread_mutex_unlock(mutex);
            free(serialized as *mut ::core::ffi::c_void);
            return event_outbox_enqueue_result_t::EVENT_OUTBOX_FULL;
        }
    }
    let mut now: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO event_outbox(event_id,event_source,event_type,subject,destination,topic,envelope_json,envelope_bytes,severity,next_attempt_at,expires_at,created_at,updated_at) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw const (*event).id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw const (*event).source as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw const (*event).r#type as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            4 as ::core::ffi::c_int,
            &raw const (*event).subject as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            5 as ::core::ffi::c_int,
            destination,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            6 as ::core::ffi::c_int,
            topic,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            7 as ::core::ffi::c_int,
            serialized,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            stmt,
            8 as ::core::ffi::c_int,
            serialized_bytes as sqlite3_int64,
        );
        sqlite3_bind_int(
            stmt,
            9 as ::core::ffi::c_int,
            (*event).severity.0 as ::core::ffi::c_int,
        );
        sqlite3_bind_int64(stmt, 10 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int64(
            stmt,
            11 as ::core::ffi::c_int,
            (*event).expires_at as int64_t as sqlite3_int64,
        );
        sqlite3_bind_int64(stmt, 12 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int64(stmt, 13 as ::core::ffi::c_int, now as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if rc != SQLITE_DONE
        || execute_sql(db, b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        rollback(db);
        pthread_mutex_unlock(mutex);
        free(serialized as *mut ::core::ffi::c_void);
        return event_outbox_enqueue_result_t::EVENT_OUTBOX_ERROR;
    }
    if !row_id.is_null() {
        *row_id = sqlite3_last_insert_rowid(db) as int64_t;
    }
    if !shed_count.is_null() {
        *shed_count = shed;
    }
    pthread_mutex_unlock(mutex);
    free(serialized as *mut ::core::ffi::c_void);
    return event_outbox_enqueue_result_t::EVENT_OUTBOX_ENQUEUED;
}
unsafe extern "C" fn expire_locked(
    mut db: *mut sqlite3,
    mut now: int64_t,
    mut expired_count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"UPDATE event_outbox SET state='dead',updated_at=?,dead_at=?,lease_expires_at=NULL,last_error='expired' WHERE state IN ('pending','delivering') AND expires_at<=?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, now as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    if !expired_count.is_null() {
        *expired_count = if rc == SQLITE_DONE {
            sqlite3_changes(db)
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn release_stale_leases_locked(
    mut db: *mut sqlite3,
    mut now: int64_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"UPDATE event_outbox SET state='pending',next_attempt_at=?,updated_at=?,lease_expires_at=NULL WHERE state='delivering' AND lease_expires_at<=? AND expires_at>?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, now as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn copy_column(
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
    mut stmt: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
) {
    let mut value: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, column) as *const ::core::ffi::c_char;
    safe_strcpy(
        output,
        if !value.is_null() {
            value
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        output_size,
        0 as size_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_outbox_item_clear(mut item: *mut event_outbox_item_t) {
    if item.is_null() {
        return;
    }
    free((*item).envelope_json as *mut ::core::ffi::c_void);
    memset(
        item as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_outbox_item_t>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_outbox_claim_due(
    mut destination: *const ::core::ffi::c_char,
    mut now: int64_t,
    mut lease_seconds: ::core::ffi::c_int,
    mut item: *mut event_outbox_item_t,
) -> ::core::ffi::c_int {
    if !valid_text(
        destination,
        EVENT_OUTBOX_DESTINATION_MAX as size_t,
        r#true != 0,
    ) || item.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        item as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_outbox_item_t>(),
    );
    if now <= 0 as int64_t {
        now = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    }
    if lease_seconds <= 0 as ::core::ffi::c_int {
        lease_seconds = EVENT_OUTBOX_DEFAULT_LEASE_SECONDS;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    if execute_sql(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    if expire_locked(db, now, ::core::ptr::null_mut::<::core::ffi::c_int>())
        != 0 as ::core::ffi::c_int
        || release_stale_leases_locked(db, now) != 0 as ::core::ffi::c_int
    {
        rollback(db);
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT id,event_id,event_source,event_type,subject,destination,topic,envelope_json,envelope_bytes,severity,attempt_count,next_attempt_at,expires_at,created_at FROM event_outbox WHERE destination=? AND state='pending' AND next_attempt_at<=? AND expires_at>? ORDER BY severity DESC,next_attempt_at,id LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            destination,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, now as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    if rc == SQLITE_DONE {
        sqlite3_finalize(stmt);
        if execute_sql(db, b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            rollback(db);
            pthread_mutex_unlock(mutex);
            return -1 as ::core::ffi::c_int;
        }
        pthread_mutex_unlock(mutex);
        return 0 as ::core::ffi::c_int;
    }
    if rc != SQLITE_ROW {
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
        rollback(db);
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    (*item).row_id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*item).event_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        stmt,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*item).event_source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        stmt,
        2 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*item).event_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        stmt,
        3 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*item).subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        stmt,
        4 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*item).destination as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        stmt,
        5 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*item).topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        stmt,
        6 as ::core::ffi::c_int,
    );
    let mut envelope: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 7 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    let mut envelope_size: ::core::ffi::c_int = sqlite3_column_bytes(stmt, 7 as ::core::ffi::c_int);
    let mut stored_size: int64_t = sqlite3_column_int64(stmt, 8 as ::core::ffi::c_int) as int64_t;
    (*item).severity =
        event_severity_t(sqlite3_column_int(stmt, 9 as ::core::ffi::c_int) as ::core::ffi::c_uint);
    (*item).attempt_count =
        sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int;
    (*item).next_attempt_at = sqlite3_column_int64(stmt, 11 as ::core::ffi::c_int) as int64_t;
    (*item).expires_at = sqlite3_column_int64(stmt, 12 as ::core::ffi::c_int) as int64_t;
    (*item).created_at = sqlite3_column_int64(stmt, 13 as ::core::ffi::c_int) as int64_t;
    if envelope.is_null()
        || envelope_size <= 0 as ::core::ffi::c_int
        || stored_size != envelope_size as int64_t
        || envelope_size > EVENT_ENVELOPE_MAX_BYTES as ::core::ffi::c_int
    {
        sqlite3_finalize(stmt);
        rollback(db);
        pthread_mutex_unlock(mutex);
        db_event_outbox_item_clear(item);
        return -1 as ::core::ffi::c_int;
    }
    (*item).envelope_json =
        malloc((envelope_size as size_t).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if (*item).envelope_json.is_null() {
        sqlite3_finalize(stmt);
        rollback(db);
        pthread_mutex_unlock(mutex);
        db_event_outbox_item_clear(item);
        return -1 as ::core::ffi::c_int;
    }
    memcpy(
        (*item).envelope_json as *mut ::core::ffi::c_void,
        envelope as *const ::core::ffi::c_void,
        envelope_size as size_t,
    );
    *(*item).envelope_json.offset(envelope_size as isize) = '\0' as ::core::ffi::c_char;
    (*item).envelope_bytes = envelope_size as size_t;
    sqlite3_finalize(stmt);
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        b"UPDATE event_outbox SET state='delivering',attempt_count=attempt_count+1,last_attempt_at=?,lease_expires_at=?,updated_at=? WHERE id=? AND state='pending';\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int64(
            stmt,
            2 as ::core::ffi::c_int,
            (now + lease_seconds as int64_t) as sqlite3_int64,
        );
        sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int64(
            stmt,
            4 as ::core::ffi::c_int,
            (*item).row_id as sqlite3_int64,
        );
        rc = sqlite3_step(stmt);
    }
    let mut changed: ::core::ffi::c_int = if rc == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if rc != SQLITE_DONE
        || changed != 1 as ::core::ffi::c_int
        || execute_sql(db, b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        rollback(db);
        pthread_mutex_unlock(mutex);
        db_event_outbox_item_clear(item);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(mutex);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn transition_delivering(
    mut row_id: int64_t,
    mut sql: *const ::core::ffi::c_char,
    mut first_time: int64_t,
    mut second_time: int64_t,
    mut error: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if row_id <= 0 as int64_t || !valid_text(error, EVENT_OUTBOX_ERROR_MAX as size_t, r#false != 0)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, first_time as sqlite3_int64);
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, second_time as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            if !error.is_null() {
                error
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, row_id as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    let mut changed: ::core::ffi::c_int = if rc == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_outbox_mark_delivered(
    mut row_id: int64_t,
    mut delivered_at: int64_t,
) -> ::core::ffi::c_int {
    if delivered_at <= 0 as int64_t {
        delivered_at = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    }
    return transition_delivering(
        row_id,
        b"UPDATE event_outbox SET state='delivered',delivered_at=?,updated_at=?,dead_at=NULL,lease_expires_at=NULL,last_error=? WHERE id=? AND state='delivering';\0"
            .as_ptr() as *const ::core::ffi::c_char,
        delivered_at,
        delivered_at,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_outbox_mark_retry(
    mut row_id: int64_t,
    mut next_attempt_at: int64_t,
    mut error: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if next_attempt_at <= 0 as int64_t {
        return -1 as ::core::ffi::c_int;
    }
    return transition_delivering(
        row_id,
        b"UPDATE event_outbox SET state='pending',next_attempt_at=?,updated_at=?,dead_at=NULL,lease_expires_at=NULL,last_error=? WHERE id=? AND state='delivering';\0"
            .as_ptr() as *const ::core::ffi::c_char,
        next_attempt_at,
        time(::core::ptr::null_mut::<time_t>()) as int64_t,
        error,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_outbox_mark_dead(
    mut row_id: int64_t,
    mut failed_at: int64_t,
    mut error: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if failed_at <= 0 as int64_t {
        failed_at = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    }
    return transition_delivering(
        row_id,
        b"UPDATE event_outbox SET state='dead',updated_at=?,dead_at=?,lease_expires_at=NULL,last_error=? WHERE id=? AND state='delivering';\0"
            .as_ptr() as *const ::core::ffi::c_char,
        failed_at,
        failed_at,
        error,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_outbox_expire(
    mut now: int64_t,
    mut expired_count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !expired_count.is_null() {
        *expired_count = 0 as ::core::ffi::c_int;
    }
    if now <= 0 as int64_t {
        now = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut result: ::core::ffi::c_int = expire_locked(db, now, expired_count);
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_outbox_prune_terminal(
    mut updated_before: int64_t,
    mut limit: ::core::ffi::c_int,
    mut deleted_count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !deleted_count.is_null() {
        *deleted_count = 0 as ::core::ffi::c_int;
    }
    if updated_before <= 0 as int64_t
        || limit <= 0 as ::core::ffi::c_int
        || limit > 100000 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM event_outbox WHERE id IN (SELECT id FROM event_outbox WHERE state IN ('delivered','dead') AND updated_at<? ORDER BY updated_at,id LIMIT ?);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(
            stmt,
            1 as ::core::ffi::c_int,
            updated_before as sqlite3_int64,
        );
        sqlite3_bind_int(stmt, 2 as ::core::ffi::c_int, limit);
        rc = sqlite3_step(stmt);
    }
    if !deleted_count.is_null() {
        *deleted_count = if rc == SQLITE_DONE {
            sqlite3_changes(db)
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_outbox_count_undelivered_type(
    mut event_type: *const ::core::ffi::c_char,
    mut count: *mut int64_t,
) -> ::core::ffi::c_int {
    if count.is_null() || !valid_text(event_type, EVENT_TYPE_MAX as size_t, r#true != 0) {
        return -1 as ::core::ffi::c_int;
    }
    *count = 0 as int64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM event_outbox WHERE event_type=? AND state IN ('pending','delivering');\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            event_type,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
        if rc == SQLITE_ROW {
            *count = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_ROW {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_outbox_get_stats(
    mut destination: *const ::core::ffi::c_char,
    mut now: int64_t,
    mut stats: *mut event_outbox_stats_t,
) -> ::core::ffi::c_int {
    if stats.is_null()
        || !destination.is_null()
            && !valid_text(
                destination,
                EVENT_OUTBOX_DESTINATION_MAX as size_t,
                r#true != 0,
            )
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_outbox_stats_t>(),
    );
    if now <= 0 as int64_t {
        now = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut group_sql: *const ::core::ffi::c_char = if !destination.is_null() {
        b"SELECT state,count(*),COALESCE(sum(envelope_bytes),0),min(created_at) FROM event_outbox WHERE destination=? GROUP BY state;\0"
            .as_ptr() as *const ::core::ffi::c_char
    } else {
        b"SELECT state,count(*),COALESCE(sum(envelope_bytes),0),min(created_at) FROM event_outbox GROUP BY state;\0"
            .as_ptr() as *const ::core::ffi::c_char
    };
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        group_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    if !destination.is_null() {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            destination,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
    loop {
        rc = sqlite3_step(stmt);
        if rc != SQLITE_ROW {
            break;
        }
        let mut state: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut rows: int64_t = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as int64_t;
        let mut bytes: int64_t = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as int64_t;
        let mut oldest: int64_t = sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as int64_t;
        (*stats).total_rows += rows;
        (*stats).total_bytes += bytes;
        if !state.is_null()
            && strcmp(state, b"pending\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            (*stats).pending_rows = rows;
            (*stats).oldest_pending_at = oldest;
        } else if !state.is_null()
            && strcmp(
                state,
                b"delivering\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            (*stats).delivering_rows = rows;
        } else if !state.is_null()
            && strcmp(state, b"delivered\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            (*stats).delivered_rows = rows;
        } else if !state.is_null()
            && strcmp(state, b"dead\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            (*stats).dead_rows = rows;
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if rc != SQLITE_DONE {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut due_sql: *const ::core::ffi::c_char = if !destination.is_null() {
        b"SELECT count(*) FROM event_outbox WHERE destination=? AND state='pending' AND next_attempt_at<=? AND expires_at>?;\0"
            .as_ptr() as *const ::core::ffi::c_char
    } else {
        b"SELECT count(*) FROM event_outbox WHERE state='pending' AND next_attempt_at<=? AND expires_at>?;\0"
            .as_ptr() as *const ::core::ffi::c_char
    };
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        due_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        let mut parameter: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if !destination.is_null() {
            let c2rust_fresh0 = parameter;
            parameter += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh0,
                destination,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        let c2rust_fresh1 = parameter;
        parameter += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh1, now as sqlite3_int64);
        sqlite3_bind_int64(stmt, parameter, now as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    if rc == SQLITE_ROW {
        (*stats).due_rows = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_ROW {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
