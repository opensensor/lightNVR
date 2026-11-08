use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
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
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_double(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_double,
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
    unsafe fn sqlite3_clear_bindings(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_double(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
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
    unsafe fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    unsafe fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    unsafe fn sqlite3_db_mutex(_: *mut sqlite3) -> *mut sqlite3_mutex;
    unsafe fn observation_asset_state_parse(
        name: *const ::core::ffi::c_char,
        state: *mut observation_asset_state_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_observation_bump_revision_locked(
        db: *mut sqlite3,
        source: *const ::core::ffi::c_char,
        observation_uuid: *const ::core::ffi::c_char,
        cause: observation_change_cause_t,
        now_ms: int64_t,
        seq_out: *mut int64_t,
        revision_out: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn event_identity_get_source(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn observation_relay_notify();
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
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
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
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
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const DB_OBS_OK: Self = Self(0);
    pub const DB_OBS_DUPLICATE: Self = Self(1);
    pub const DB_OBS_CAS_LOST: Self = Self(2);
    pub const DB_OBS_ERROR: Self = Self(-1);
    pub const DB_OBS_INVALID: Self = Self(-2);
    pub const DB_OBS_NOT_FOUND: Self = Self(-3);
    pub const DB_OBS_INVALID_CURSOR: Self = Self(-4);
    pub const DB_OBS_CURSOR_EXPIRED: Self = Self(-5);
    pub const DB_OBS_CONFLICT: Self = Self(-6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_asset_state_t(pub ::core::ffi::c_uint);
impl observation_asset_state_t {
    pub const OBS_ASSET_PENDING: Self = Self(0);
    pub const OBS_ASSET_READY: Self = Self(1);
    pub const OBS_ASSET_UNAVAILABLE: Self = Self(2);
    pub const OBS_ASSET_EXPIRED: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_change_cause_t(pub ::core::ffi::c_uint);
impl observation_change_cause_t {
    pub const OBS_CAUSE_CREATED: Self = Self(0);
    pub const OBS_CAUSE_EVIDENCE: Self = Self(1);
    pub const OBS_CAUSE_CLASSIFICATION: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_classification_kind_t(pub ::core::ffi::c_uint);
impl observation_classification_kind_t {
    pub const OBS_CLS_KIND_AUTOMATED: Self = Self(0);
    pub const OBS_CLS_KIND_OWNER_CONFIRMED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_0 {
    pub const DB_OBS_CLS_CREATED: Self = Self(0);
    pub const DB_OBS_CLS_UPDATED: Self = Self(1);
    pub const DB_OBS_CLS_NOOP: Self = Self(2);
    pub const DB_OBS_CLS_REVISION_CONFLICT: Self = Self(3);
    pub const DB_OBS_CLS_OWNER_LOCKED: Self = Self(4);
    pub const DB_OBS_CLS_EVIDENCE_MISMATCH: Self = Self(5);
    pub const DB_OBS_CLS_WRITER_MISMATCH: Self = Self(6);
    pub const DB_OBS_CLS_LIMIT: Self = Self(7);
    pub const DB_OBS_CLS_ERROR: Self = Self(-1);
    pub const DB_OBS_CLS_INVALID: Self = Self(-2);
    pub const DB_OBS_CLS_NOT_FOUND: Self = Self(-3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_classification_t {
    pub key: [::core::ffi::c_char; 66],
    pub writer_namespace: [::core::ffi::c_char; 33],
    pub writer_user_id: int64_t,
    pub classifier: [::core::ffi::c_char; 65],
    pub model_version: [::core::ffi::c_char; 65],
    pub label: [::core::ffi::c_char; 129],
    pub taxonomy_id: [::core::ffi::c_char; 65],
    pub confidence: ::core::ffi::c_double,
    pub classified_at_ms: int64_t,
    pub input_asset_uuid: [::core::ffi::c_char; 37],
    pub input_revision: ::core::ffi::c_int,
    pub kind: observation_classification_kind_t,
    pub annotation_revision: ::core::ffi::c_int,
    pub created_at_ms: int64_t,
    pub updated_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_classification_input_t {
    pub writer_namespace: *const ::core::ffi::c_char,
    pub writer_user_id: int64_t,
    pub name: *const ::core::ffi::c_char,
    pub classifier: *const ::core::ffi::c_char,
    pub model_version: *const ::core::ffi::c_char,
    pub label: *const ::core::ffi::c_char,
    pub taxonomy_id: *const ::core::ffi::c_char,
    pub confidence: ::core::ffi::c_double,
    pub classified_at_ms: int64_t,
    pub input_asset_uuid: *const ::core::ffi::c_char,
    pub input_revision: ::core::ffi::c_int,
    pub kind: observation_classification_kind_t,
    pub expected_revision: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_classification_result_t {
    pub has_annotation: bool,
    pub annotation: observation_classification_t,
    pub observation_revision: ::core::ffi::c_int,
    pub change_seq: int64_t,
    pub observation_asset_uuid: [::core::ffi::c_char; 37],
    pub evidence_state: observation_asset_state_t,
    pub limit: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cls_tx_t {
    pub mutex: *mut pthread_mutex_t,
    pub db: *mut sqlite3,
    pub connection_mutex: *mut sqlite3_mutex,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_state_t {
    pub revision: ::core::ffi::c_int,
    pub asset_uuid: [::core::ffi::c_char; 37],
    pub state: observation_asset_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_classification_set_t {
    pub automated_count: ::core::ffi::c_int,
    pub automated: [observation_classification_t; 8],
    pub owner_confirmed_count: ::core::ffi::c_int,
    pub owner_confirmed: [observation_classification_t; 8],
}
pub type observation_classification_visit_fn = Option<
    unsafe extern "C" fn(
        size_t,
        *const observation_classification_set_t,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_detection_ref_t {
    pub label: *const ::core::ffi::c_char,
    pub timestamp: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_detection_link_t {
    pub linked: bool,
    pub observation_uuid: [::core::ffi::c_char; 37],
    pub has_classification: bool,
    pub owner_confirmed: bool,
    pub classification: observation_classification_t,
}
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const OBS_CLS_SEGMENT_MAX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const OBS_CLS_CLASSIFIER_SIZE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const OBS_CLS_MODEL_VERSION_SIZE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const OBS_CLS_LABEL_SIZE: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const OBS_CLS_TAXONOMY_SIZE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const OBS_CLS_MAX_PER_OBSERVATION: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const OBS_CLS_CONFIDENCE_EPSILON: ::core::ffi::c_double = 1e-6f64;
pub const OBS_DETECTION_LINK_WINDOW_SECONDS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut KIND_NAMES: [*const ::core::ffi::c_char; 2] = [
    b"automated\0".as_ptr() as *const ::core::ffi::c_char,
    b"owner_confirmed\0".as_ptr() as *const ::core::ffi::c_char,
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_classification_kind_name(
    mut kind: observation_classification_kind_t,
) -> *const ::core::ffi::c_char {
    return if kind.0 == observation_classification_kind_t::OBS_CLS_KIND_AUTOMATED.0
        || kind.0 == observation_classification_kind_t::OBS_CLS_KIND_OWNER_CONFIRMED.0
    {
        KIND_NAMES[kind.0 as usize]
    } else {
        b"automated\0".as_ptr() as *const ::core::ffi::c_char
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_classification_kind_parse(
    mut name: *const ::core::ffi::c_char,
    mut kind: *mut observation_classification_kind_t,
) -> ::core::ffi::c_int {
    if name.is_null() || kind.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        if strcmp(name, KIND_NAMES[i as usize]) == 0 as ::core::ffi::c_int {
            *kind = observation_classification_kind_t(i as ::core::ffi::c_uint);
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_classification_segment_is_valid(
    mut segment: *const ::core::ffi::c_char,
) -> bool {
    if segment.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strnlen(
        segment,
        (OBS_CLS_SEGMENT_MAX + 1 as ::core::ffi::c_int) as size_t,
    );
    if length == 0 as size_t || length > OBS_CLS_SEGMENT_MAX as size_t {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < length {
        let mut c: ::core::ffi::c_uchar = *segment.offset(i as isize) as ::core::ffi::c_uchar;
        let mut alnum: bool = c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
            && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
            || c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int;
        if if i == 0 as size_t {
            !alnum as ::core::ffi::c_int
        } else {
            !(alnum as ::core::ffi::c_int != 0
                || c as ::core::ffi::c_int == '_' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '-' as ::core::ffi::c_int)
                as ::core::ffi::c_int
        } != 0
        {
            return r#false != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn empty_text(mut value: *const ::core::ffi::c_char) -> bool {
    return value.is_null() || *value.offset(0isize) == 0;
}
unsafe extern "C" fn optional_len_ok(
    mut value: *const ::core::ffi::c_char,
    mut max: size_t,
) -> bool {
    return empty_text(value) as ::core::ffi::c_int != 0
        || strnlen(value, max.wrapping_add(1 as size_t)) <= max;
}
unsafe extern "C" fn required_len_ok(
    mut value: *const ::core::ffi::c_char,
    mut max: size_t,
) -> bool {
    if empty_text(value) {
        return r#false != 0;
    }
    return strnlen(value, max.wrapping_add(1 as size_t)) <= max;
}
unsafe extern "C" fn valid_input(mut r#in: *const observation_classification_input_t) -> bool {
    return !r#in.is_null()
        && observation_classification_segment_is_valid((*r#in).writer_namespace)
            as ::core::ffi::c_int
            != 0
        && observation_classification_segment_is_valid((*r#in).name) as ::core::ffi::c_int != 0
        && (*r#in).writer_user_id > 0 as int64_t
        && required_len_ok(
            (*r#in).classifier,
            (OBS_CLS_CLASSIFIER_SIZE - 1 as ::core::ffi::c_int) as size_t,
        ) as ::core::ffi::c_int
            != 0
        && optional_len_ok(
            (*r#in).model_version,
            (OBS_CLS_MODEL_VERSION_SIZE - 1 as ::core::ffi::c_int) as size_t,
        ) as ::core::ffi::c_int
            != 0
        && required_len_ok(
            (*r#in).label,
            (OBS_CLS_LABEL_SIZE - 1 as ::core::ffi::c_int) as size_t,
        ) as ::core::ffi::c_int
            != 0
        && optional_len_ok(
            (*r#in).taxonomy_id,
            (OBS_CLS_TAXONOMY_SIZE - 1 as ::core::ffi::c_int) as size_t,
        ) as ::core::ffi::c_int
            != 0
        && (*r#in).confidence.is_finite() as ::core::ffi::c_int != 0
        && (*r#in).confidence >= 0.0f64
        && (*r#in).confidence <= 1.0f64
        && (*r#in).classified_at_ms >= 0 as int64_t
        && lightnvr_uuid_is_valid((*r#in).input_asset_uuid) as ::core::ffi::c_int != 0
        && (*r#in).input_revision >= 1 as ::core::ffi::c_int
        && ((*r#in).kind.0 == observation_classification_kind_t::OBS_CLS_KIND_AUTOMATED.0
            || (*r#in).kind.0
                == observation_classification_kind_t::OBS_CLS_KIND_OWNER_CONFIRMED.0)
        && (*r#in).expected_revision >= 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn current_time_ms() -> int64_t {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_REALTIME, &raw mut now) != 0 as ::core::ffi::c_int {
        return time(::core::ptr::null_mut::<time_t>()) as int64_t * 1000 as int64_t;
    }
    return now.tv_sec as int64_t * 1000 as int64_t + now.tv_nsec as int64_t / 1000000 as int64_t;
}
unsafe extern "C" fn copy_column(
    mut stmt: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
    mut out: *mut ::core::ffi::c_char,
    mut size: size_t,
) {
    let mut value: *const ::core::ffi::c_uchar = sqlite3_column_text(stmt, column);
    safe_strcpy(
        out,
        if !value.is_null() {
            value as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        size,
        0 as size_t,
    );
}
unsafe extern "C" fn bind_text_or_null(
    mut stmt: *mut sqlite3_stmt,
    mut index: ::core::ffi::c_int,
    mut value: *const ::core::ffi::c_char,
) {
    if !empty_text(value) {
        sqlite3_bind_text(
            stmt,
            index,
            value,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    } else {
        sqlite3_bind_null(stmt, index);
    };
}
unsafe extern "C" fn same_optional(
    mut stored: *const ::core::ffi::c_char,
    mut requested: *const ::core::ffi::c_char,
) -> bool {
    return strcmp(
        stored,
        if !requested.is_null() {
            requested
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_annotation(
    mut stmt: *mut sqlite3_stmt,
    mut out: *mut observation_classification_t,
) {
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_classification_t>(),
    );
    copy_column(
        stmt,
        0 as ::core::ffi::c_int,
        &raw mut (*out).key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 66]>(),
    );
    copy_column(
        stmt,
        1 as ::core::ffi::c_int,
        &raw mut (*out).writer_namespace as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 33]>(),
    );
    (*out).writer_user_id = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as int64_t;
    copy_column(
        stmt,
        3 as ::core::ffi::c_int,
        &raw mut (*out).classifier as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    );
    copy_column(
        stmt,
        4 as ::core::ffi::c_int,
        &raw mut (*out).model_version as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    );
    copy_column(
        stmt,
        5 as ::core::ffi::c_int,
        &raw mut (*out).label as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 129]>(),
    );
    copy_column(
        stmt,
        6 as ::core::ffi::c_int,
        &raw mut (*out).taxonomy_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    );
    (*out).confidence = sqlite3_column_double(stmt, 7 as ::core::ffi::c_int);
    (*out).classified_at_ms = sqlite3_column_int64(stmt, 8 as ::core::ffi::c_int) as int64_t;
    copy_column(
        stmt,
        9 as ::core::ffi::c_int,
        &raw mut (*out).input_asset_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    );
    (*out).input_revision = sqlite3_column_int(stmt, 10 as ::core::ffi::c_int);
    let mut kind: [::core::ffi::c_char; 24] = [0; 24];
    copy_column(
        stmt,
        11 as ::core::ffi::c_int,
        &raw mut kind as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
    );
    if observation_classification_kind_parse(
        &raw mut kind as *mut ::core::ffi::c_char,
        &raw mut (*out).kind,
    ) != 0 as ::core::ffi::c_int
    {
        (*out).kind = observation_classification_kind_t::OBS_CLS_KIND_AUTOMATED;
    }
    (*out).annotation_revision = sqlite3_column_int(stmt, 12 as ::core::ffi::c_int);
    (*out).created_at_ms = sqlite3_column_int64(stmt, 13 as ::core::ffi::c_int) as int64_t;
    (*out).updated_at_ms = sqlite3_column_int64(stmt, 14 as ::core::ffi::c_int) as int64_t;
}
unsafe extern "C" fn load_annotation(
    mut db: *mut sqlite3,
    mut observation_uuid: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_char,
    mut out: *mut observation_classification_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut found: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT key, writer_namespace, writer_user_id, classifier, model_version, label, taxonomy_id, confidence, classified_at_ms, input_asset_uuid, input_revision, kind, annotation_revision, created_at_ms, updated_at_ms FROM observation_classifications WHERE observation_uuid = ? AND key = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            observation_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            key,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
        if step == SQLITE_ROW {
            read_annotation(stmt, out);
            found = 1 as ::core::ffi::c_int;
        } else if step == SQLITE_DONE {
            found = 0 as ::core::ffi::c_int;
        }
    }
    if found < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to read classification: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    return found;
}
unsafe extern "C" fn same_payload(
    mut stored: *const observation_classification_t,
    mut r#in: *const observation_classification_input_t,
) -> bool {
    return strcmp(
        &raw const (*stored).classifier as *const ::core::ffi::c_char,
        (*r#in).classifier,
    ) == 0 as ::core::ffi::c_int
        && same_optional(
            &raw const (*stored).model_version as *const ::core::ffi::c_char,
            (*r#in).model_version,
        ) as ::core::ffi::c_int
            != 0
        && strcmp(
            &raw const (*stored).label as *const ::core::ffi::c_char,
            (*r#in).label,
        ) == 0 as ::core::ffi::c_int
        && same_optional(
            &raw const (*stored).taxonomy_id as *const ::core::ffi::c_char,
            (*r#in).taxonomy_id,
        ) as ::core::ffi::c_int
            != 0
        && fabs((*stored).confidence - (*r#in).confidence) <= OBS_CLS_CONFIDENCE_EPSILON
        && (*stored).classified_at_ms == (*r#in).classified_at_ms
        && strcasecmp(
            &raw const (*stored).input_asset_uuid as *const ::core::ffi::c_char,
            (*r#in).input_asset_uuid,
        ) == 0 as ::core::ffi::c_int
        && (*stored).input_revision == (*r#in).input_revision
        && (*stored).kind.0 == (*r#in).kind.0;
}
unsafe extern "C" fn load_observation(
    mut db: *mut sqlite3,
    mut observation_uuid: *const ::core::ffi::c_char,
    mut out: *mut observation_state_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT o.revision, a.uuid, a.state FROM object_observations o JOIN observation_assets a ON a.uuid = o.asset_uuid WHERE o.uuid = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            observation_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
        if step == SQLITE_ROW {
            (*out).revision = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
            copy_column(
                stmt,
                1 as ::core::ffi::c_int,
                &raw mut (*out).asset_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            );
            let mut state: [::core::ffi::c_char; 16] = [0; 16];
            copy_column(
                stmt,
                2 as ::core::ffi::c_int,
                &raw mut state as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            );
            if observation_asset_state_parse(
                &raw mut state as *mut ::core::ffi::c_char,
                &raw mut (*out).state,
            ) != 0 as ::core::ffi::c_int
            {
                (*out).state = observation_asset_state_t::OBS_ASSET_UNAVAILABLE;
            }
            rc = C2Rust_Unnamed_0::DB_OBS_CLS_CREATED.0;
        } else if step == SQLITE_DONE {
            rc = C2Rust_Unnamed_0::DB_OBS_CLS_NOT_FOUND.0;
        }
    }
    if rc == C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to read observation: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    return rc;
}
unsafe extern "C" fn count_keys(
    mut db: *mut sqlite3,
    mut observation_uuid: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM observation_classifications WHERE observation_uuid = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            observation_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if sqlite3_step(stmt) == SQLITE_ROW {
            count = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
        }
    }
    sqlite3_finalize(stmt);
    return count;
}
unsafe extern "C" fn store_annotation(
    mut db: *mut sqlite3,
    mut observation_uuid: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_char,
    mut r#in: *const observation_classification_input_t,
    mut asset_uuid: *const ::core::ffi::c_char,
    mut existing: *const observation_classification_t,
    mut now_ms: int64_t,
) -> ::core::ffi::c_int {
    static mut INSERT_SQL: *const ::core::ffi::c_char = b"INSERT INTO observation_classifications (classifier, model_version, label, taxonomy_id, confidence, classified_at_ms, input_asset_uuid, input_revision, kind, updated_at_ms, observation_uuid, key, writer_namespace, writer_user_id, annotation_revision, created_at_ms) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, 1, ?10);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    static mut UPDATE_SQL: *const ::core::ffi::c_char = b"UPDATE observation_classifications SET classifier = ?1, model_version = ?2, label = ?3, taxonomy_id = ?4, confidence = ?5, classified_at_ms = ?6, input_asset_uuid = ?7, input_revision = ?8, kind = ?9, updated_at_ms = ?10, annotation_revision = annotation_revision + 1 WHERE observation_uuid = ?11 AND key = ?12 AND writer_user_id = ?14 AND annotation_revision = ?15;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        if !existing.is_null() {
            UPDATE_SQL
        } else {
            INSERT_SQL
        },
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            (*r#in).classifier,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        bind_text_or_null(stmt, 2 as ::core::ffi::c_int, (*r#in).model_version);
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            (*r#in).label,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        bind_text_or_null(stmt, 4 as ::core::ffi::c_int, (*r#in).taxonomy_id);
        sqlite3_bind_double(stmt, 5 as ::core::ffi::c_int, (*r#in).confidence);
        sqlite3_bind_int64(
            stmt,
            6 as ::core::ffi::c_int,
            (*r#in).classified_at_ms as sqlite3_int64,
        );
        sqlite3_bind_text(
            stmt,
            7 as ::core::ffi::c_int,
            asset_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(stmt, 8 as ::core::ffi::c_int, (*r#in).input_revision);
        sqlite3_bind_text(
            stmt,
            9 as ::core::ffi::c_int,
            observation_classification_kind_name((*r#in).kind),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 10 as ::core::ffi::c_int, now_ms as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            11 as ::core::ffi::c_int,
            observation_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            12 as ::core::ffi::c_int,
            key,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if !existing.is_null() {
            sqlite3_bind_int64(
                stmt,
                14 as ::core::ffi::c_int,
                (*existing).writer_user_id as sqlite3_int64,
            );
            sqlite3_bind_int(
                stmt,
                15 as ::core::ffi::c_int,
                (*existing).annotation_revision,
            );
        } else {
            sqlite3_bind_text(
                stmt,
                13 as ::core::ffi::c_int,
                (*r#in).writer_namespace,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_int64(
                stmt,
                14 as ::core::ffi::c_int,
                (*r#in).writer_user_id as sqlite3_int64,
            );
        }
        if sqlite3_step(stmt) == SQLITE_DONE && sqlite3_changes(db) == 1 as ::core::ffi::c_int {
            rc = if !existing.is_null() {
                C2Rust_Unnamed_0::DB_OBS_CLS_UPDATED.0
            } else {
                C2Rust_Unnamed_0::DB_OBS_CLS_CREATED.0
            };
        }
    }
    if rc == C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to store classification: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    return rc;
}
unsafe extern "C" fn tx_begin(mut tx: *mut cls_tx_t) -> ::core::ffi::c_int {
    (*tx).mutex = get_db_mutex();
    if (*tx).mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock((*tx).mutex);
    (*tx).db = get_db_handle();
    if (*tx).db.is_null() {
        pthread_mutex_unlock((*tx).mutex);
        return -1 as ::core::ffi::c_int;
    }
    (*tx).connection_mutex = sqlite3_db_mutex((*tx).db);
    sqlite3_mutex_enter((*tx).connection_mutex);
    if sqlite3_exec(
        (*tx).db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to begin classification transaction: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg((*tx).db),
        );
        sqlite3_mutex_leave((*tx).connection_mutex);
        pthread_mutex_unlock((*tx).mutex);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn tx_end(mut tx: *mut cls_tx_t, mut commit: bool) -> bool {
    let mut ok: bool = r#true != 0;
    if commit as ::core::ffi::c_int != 0
        && sqlite3_exec(
            (*tx).db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to commit classification transaction: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg((*tx).db),
        );
        ok = r#false != 0;
    }
    if !commit || !ok {
        sqlite3_exec(
            (*tx).db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    sqlite3_mutex_leave((*tx).connection_mutex);
    pthread_mutex_unlock((*tx).mutex);
    return ok;
}
unsafe extern "C" fn put_locked(
    mut db: *mut sqlite3,
    mut source: *const ::core::ffi::c_char,
    mut observation_uuid: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_char,
    mut r#in: *const observation_classification_input_t,
    mut result: *mut observation_classification_result_t,
) -> ::core::ffi::c_int {
    let mut obs: observation_state_t = observation_state_t {
        revision: 0,
        asset_uuid: [0; 37],
        state: observation_asset_state_t::OBS_ASSET_PENDING,
    };
    let mut rc: ::core::ffi::c_int = load_observation(db, observation_uuid, &raw mut obs);
    if rc != C2Rust_Unnamed_0::DB_OBS_CLS_CREATED.0 {
        return rc;
    }
    (*result).observation_revision = obs.revision;
    safe_strcpy(
        &raw mut (*result).observation_asset_uuid as *mut ::core::ffi::c_char,
        &raw mut obs.asset_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    (*result).evidence_state = obs.state;
    let mut existing: observation_classification_t = observation_classification_t {
        key: [0; 66],
        writer_namespace: [0; 33],
        writer_user_id: 0,
        classifier: [0; 65],
        model_version: [0; 65],
        label: [0; 129],
        taxonomy_id: [0; 65],
        confidence: 0.,
        classified_at_ms: 0,
        input_asset_uuid: [0; 37],
        input_revision: 0,
        kind: observation_classification_kind_t::OBS_CLS_KIND_AUTOMATED,
        annotation_revision: 0,
        created_at_ms: 0,
        updated_at_ms: 0,
    };
    let mut found: ::core::ffi::c_int =
        load_annotation(db, observation_uuid, key, &raw mut existing);
    if found < 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0;
    }
    if found != 0
        && (existing.writer_user_id != (*r#in).writer_user_id
            || strcmp(
                &raw mut existing.writer_namespace as *mut ::core::ffi::c_char,
                (*r#in).writer_namespace,
            ) != 0 as ::core::ffi::c_int)
    {
        return C2Rust_Unnamed_0::DB_OBS_CLS_WRITER_MISMATCH.0;
    }
    if strcasecmp(
        (*r#in).input_asset_uuid,
        &raw mut obs.asset_uuid as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        || (*r#in).input_revision > obs.revision
        || obs.state.0 != observation_asset_state_t::OBS_ASSET_READY.0
            && obs.state.0 != observation_asset_state_t::OBS_ASSET_EXPIRED.0
    {
        return C2Rust_Unnamed_0::DB_OBS_CLS_EVIDENCE_MISMATCH.0;
    }
    if found != 0 {
        (*result).has_annotation = r#true != 0;
        (*result).annotation = existing;
        if same_payload(&raw mut existing, r#in) {
            return C2Rust_Unnamed_0::DB_OBS_CLS_NOOP.0;
        }
        if existing.kind.0 == observation_classification_kind_t::OBS_CLS_KIND_OWNER_CONFIRMED.0
            && (*r#in).kind.0 == observation_classification_kind_t::OBS_CLS_KIND_AUTOMATED.0
        {
            return C2Rust_Unnamed_0::DB_OBS_CLS_OWNER_LOCKED.0;
        }
        if (*r#in).expected_revision != existing.annotation_revision {
            return C2Rust_Unnamed_0::DB_OBS_CLS_REVISION_CONFLICT.0;
        }
    } else {
        if (*r#in).expected_revision != 0 as ::core::ffi::c_int {
            return C2Rust_Unnamed_0::DB_OBS_CLS_REVISION_CONFLICT.0;
        }
        let mut keys: ::core::ffi::c_int = count_keys(db, observation_uuid);
        if keys < 0 as ::core::ffi::c_int {
            return C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0;
        }
        if keys >= OBS_CLS_MAX_PER_OBSERVATION {
            return C2Rust_Unnamed_0::DB_OBS_CLS_LIMIT.0;
        }
    }
    let mut now_ms: int64_t = current_time_ms();
    let mut stored: ::core::ffi::c_int = store_annotation(
        db,
        observation_uuid,
        key,
        r#in,
        &raw mut obs.asset_uuid as *mut ::core::ffi::c_char,
        if found != 0 {
            &raw mut existing
        } else {
            ::core::ptr::null_mut::<observation_classification_t>()
        },
        now_ms,
    );
    if stored != C2Rust_Unnamed_0::DB_OBS_CLS_CREATED.0
        && stored != C2Rust_Unnamed_0::DB_OBS_CLS_UPDATED.0
    {
        return C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0;
    }
    let mut seq: int64_t = 0 as int64_t;
    let mut revision: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db_observation_bump_revision_locked(
        db,
        source,
        observation_uuid,
        observation_change_cause_t::OBS_CAUSE_CLASSIFICATION,
        now_ms,
        &raw mut seq,
        &raw mut revision,
    ) != C2Rust_Unnamed::DB_OBS_OK.0
    {
        return C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0;
    }
    let mut written: observation_classification_t = observation_classification_t {
        key: [0; 66],
        writer_namespace: [0; 33],
        writer_user_id: 0,
        classifier: [0; 65],
        model_version: [0; 65],
        label: [0; 129],
        taxonomy_id: [0; 65],
        confidence: 0.,
        classified_at_ms: 0,
        input_asset_uuid: [0; 37],
        input_revision: 0,
        kind: observation_classification_kind_t::OBS_CLS_KIND_AUTOMATED,
        annotation_revision: 0,
        created_at_ms: 0,
        updated_at_ms: 0,
    };
    if load_annotation(db, observation_uuid, key, &raw mut written) != 1 as ::core::ffi::c_int {
        return C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0;
    }
    (*result).has_annotation = r#true != 0;
    (*result).annotation = written;
    (*result).observation_revision = revision;
    (*result).change_seq = seq;
    return stored;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_classification_put(
    mut observation_uuid: *const ::core::ffi::c_char,
    mut input: *const observation_classification_input_t,
    mut result: *mut observation_classification_result_t,
) -> ::core::ffi::c_int {
    if result.is_null() {
        return C2Rust_Unnamed_0::DB_OBS_CLS_INVALID.0;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_classification_result_t>(),
    );
    (*result).limit = OBS_CLS_MAX_PER_OBSERVATION;
    if !lightnvr_uuid_is_valid(observation_uuid) || !valid_input(input) {
        return C2Rust_Unnamed_0::DB_OBS_CLS_INVALID.0;
    }
    let mut key: [::core::ffi::c_char; 66] = [0; 66];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 66]>(),
        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*input).writer_namespace,
        (*input).name,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 66]>()
    {
        return C2Rust_Unnamed_0::DB_OBS_CLS_INVALID.0;
    }
    let mut source: [::core::ffi::c_char; 160] = [0; 160];
    if event_identity_get_source(
        &raw mut source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Classification write needs the event installation identity\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0;
    }
    let mut tx: cls_tx_t = cls_tx_t {
        mutex: ::core::ptr::null_mut::<pthread_mutex_t>(),
        db: ::core::ptr::null_mut::<sqlite3>(),
        connection_mutex: ::core::ptr::null_mut::<sqlite3_mutex>(),
    };
    if tx_begin(&raw mut tx) != 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0;
    }
    let mut rc: ::core::ffi::c_int = put_locked(
        tx.db,
        &raw mut source as *mut ::core::ffi::c_char,
        observation_uuid,
        &raw mut key as *mut ::core::ffi::c_char,
        input,
        result,
    );
    let mut changed: bool = rc == C2Rust_Unnamed_0::DB_OBS_CLS_CREATED.0
        || rc == C2Rust_Unnamed_0::DB_OBS_CLS_UPDATED.0;
    if !tx_end(&raw mut tx, changed) {
        rc = C2Rust_Unnamed_0::DB_OBS_CLS_ERROR.0;
    }
    if rc == C2Rust_Unnamed_0::DB_OBS_CLS_CREATED.0 || rc == C2Rust_Unnamed_0::DB_OBS_CLS_UPDATED.0
    {
        observation_relay_notify();
    } else if rc < 0 as ::core::ffi::c_int && rc != C2Rust_Unnamed_0::DB_OBS_CLS_NOT_FOUND.0 {
        memset(
            result as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<observation_classification_result_t>(),
        );
        (*result).limit = OBS_CLS_MAX_PER_OBSERVATION;
    } else if rc == C2Rust_Unnamed_0::DB_OBS_CLS_WRITER_MISMATCH.0 {
        (*result).has_annotation = r#false != 0;
        memset(
            &raw mut (*result).annotation as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<observation_classification_t>(),
        );
    }
    return rc;
}
pub const CLS_LIST_SQL: [::core::ffi::c_char; 311] = unsafe {
    ::core::mem::transmute::<
        [u8; 311],
        [::core::ffi::c_char; 311],
    >(
        *b"SELECT key, writer_namespace, writer_user_id, classifier, model_version, label, taxonomy_id, confidence, classified_at_ms, input_asset_uuid, input_revision, kind, annotation_revision, created_at_ms, updated_at_ms FROM observation_classifications WHERE observation_uuid = ? ORDER BY updated_at_ms DESC, key ASC;\0",
    )
};
unsafe extern "C" fn list_with(
    mut stmt: *mut sqlite3_stmt,
    mut observation_uuid: *const ::core::ffi::c_char,
    mut out: *mut observation_classification_set_t,
) -> ::core::ffi::c_int {
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_classification_set_t>(),
    );
    sqlite3_reset(stmt);
    sqlite3_clear_bindings(stmt);
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        observation_uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut step: ::core::ffi::c_int = 0;
    loop {
        step = sqlite3_step(stmt);
        if step != SQLITE_ROW {
            break;
        }
        let mut row: observation_classification_t = observation_classification_t {
            key: [0; 66],
            writer_namespace: [0; 33],
            writer_user_id: 0,
            classifier: [0; 65],
            model_version: [0; 65],
            label: [0; 129],
            taxonomy_id: [0; 65],
            confidence: 0.,
            classified_at_ms: 0,
            input_asset_uuid: [0; 37],
            input_revision: 0,
            kind: observation_classification_kind_t::OBS_CLS_KIND_AUTOMATED,
            annotation_revision: 0,
            created_at_ms: 0,
            updated_at_ms: 0,
        };
        read_annotation(stmt, &raw mut row);
        if row.kind.0 == observation_classification_kind_t::OBS_CLS_KIND_OWNER_CONFIRMED.0 {
            if (*out).owner_confirmed_count < OBS_CLS_MAX_PER_OBSERVATION {
                let c2rust_fresh0 = (*out).owner_confirmed_count;
                (*out).owner_confirmed_count += 1;
                (*out).owner_confirmed[c2rust_fresh0 as usize] = row;
            }
        } else if (*out).automated_count < OBS_CLS_MAX_PER_OBSERVATION {
            let c2rust_fresh1 = (*out).automated_count;
            (*out).automated_count += 1;
            (*out).automated[c2rust_fresh1 as usize] = row;
        }
    }
    if step == SQLITE_DONE {
        return C2Rust_Unnamed::DB_OBS_OK.0;
    }
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_classification_set_t>(),
    );
    return C2Rust_Unnamed::DB_OBS_ERROR.0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_classifications_list(
    mut observation_uuid: *const ::core::ffi::c_char,
    mut out: *mut observation_classification_set_t,
) -> ::core::ffi::c_int {
    if out.is_null() || !lightnvr_uuid_is_valid(observation_uuid) {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_classification_set_t>(),
    );
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        CLS_LIST_SQL.as_ptr(),
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        rc = list_with(stmt, observation_uuid, out);
    }
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to list classifications: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_classifications_list_many(
    mut observation_uuids: *const *const ::core::ffi::c_char,
    mut count: size_t,
    mut visit: observation_classification_visit_fn,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if visit.is_none() || count > 0 as size_t && observation_uuids.is_null() {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut i: size_t = 0 as size_t;
    while i < count {
        if !lightnvr_uuid_is_valid(*observation_uuids.offset(i as isize)) {
            return C2Rust_Unnamed::DB_OBS_INVALID.0;
        }
        i = i.wrapping_add(1);
    }
    if count == 0 as size_t {
        return C2Rust_Unnamed::DB_OBS_OK.0;
    }
    let mut set: *mut observation_classification_set_t =
        malloc(::core::mem::size_of::<observation_classification_set_t>())
            as *mut observation_classification_set_t;
    if set.is_null() {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        free(set as *mut ::core::ffi::c_void);
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        CLS_LIST_SQL.as_ptr(),
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        rc = C2Rust_Unnamed::DB_OBS_OK.0;
        let mut i_0: size_t = 0 as size_t;
        while rc == C2Rust_Unnamed::DB_OBS_OK.0 && i_0 < count {
            if list_with(stmt, *observation_uuids.offset(i_0 as isize), set)
                != C2Rust_Unnamed::DB_OBS_OK.0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to list classifications: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_errmsg(db),
                );
                rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
                break;
            } else {
                let mut visited: ::core::ffi::c_int =
                    visit.expect("non-null function pointer")(i_0, set, context);
                if visited != 0 as ::core::ffi::c_int {
                    rc = visited;
                }
                i_0 = i_0.wrapping_add(1);
            }
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to list classifications: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    free(set as *mut ::core::ffi::c_void);
    return rc;
}
pub const LINK_MATCH_SQL: [::core::ffi::c_char; 201] = unsafe {
    ::core::mem::transmute::<
        [u8; 201],
        [::core::ffi::c_char; 201],
    >(
        *b"SELECT uuid FROM object_observations WHERE camera_uuid = ?1 AND observed_at_ms BETWEEN ?2 AND ?3 AND label = ?4 COLLATE NOCASE ORDER BY abs((observed_at_ms / 1000) - ?5), observed_at_ms, uuid LIMIT 1;\0",
    )
};
unsafe extern "C" fn link_one(
    mut r#match: *mut sqlite3_stmt,
    mut list: *mut sqlite3_stmt,
    mut camera_uuid: *const ::core::ffi::c_char,
    mut r#ref: *const observation_detection_ref_t,
    mut set: *mut observation_classification_set_t,
    mut out: *mut observation_detection_link_t,
) -> ::core::ffi::c_int {
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_detection_link_t>(),
    );
    if (*r#ref).label.is_null() || *(*r#ref).label.offset(0isize) == 0 {
        return C2Rust_Unnamed::DB_OBS_OK.0;
    }
    if (*r#ref).timestamp < 0 as int64_t
        || (*r#ref).timestamp > INT64_MAX as int64_t / 2000 as int64_t
    {
        return C2Rust_Unnamed::DB_OBS_OK.0;
    }
    let mut window: int64_t = OBS_DETECTION_LINK_WINDOW_SECONDS as int64_t;
    sqlite3_reset(r#match);
    sqlite3_clear_bindings(r#match);
    sqlite3_bind_text(
        r#match,
        1 as ::core::ffi::c_int,
        camera_uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_int64(
        r#match,
        2 as ::core::ffi::c_int,
        (((*r#ref).timestamp - window) * 1000 as int64_t) as sqlite3_int64,
    );
    sqlite3_bind_int64(
        r#match,
        3 as ::core::ffi::c_int,
        (((*r#ref).timestamp + window) * 1000 as int64_t + 999 as int64_t) as sqlite3_int64,
    );
    sqlite3_bind_text(
        r#match,
        4 as ::core::ffi::c_int,
        (*r#ref).label,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_int64(
        r#match,
        5 as ::core::ffi::c_int,
        (*r#ref).timestamp as sqlite3_int64,
    );
    let mut step: ::core::ffi::c_int = sqlite3_step(r#match);
    if step == SQLITE_DONE {
        return C2Rust_Unnamed::DB_OBS_OK.0;
    }
    if step != SQLITE_ROW {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    copy_column(
        r#match,
        0 as ::core::ffi::c_int,
        &raw mut (*out).observation_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    );
    if !lightnvr_uuid_is_valid(&raw mut (*out).observation_uuid as *mut ::core::ffi::c_char) {
        memset(
            out as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<observation_detection_link_t>(),
        );
        return C2Rust_Unnamed::DB_OBS_OK.0;
    }
    (*out).linked = r#true != 0;
    if list_with(
        list,
        &raw mut (*out).observation_uuid as *mut ::core::ffi::c_char,
        set,
    ) != C2Rust_Unnamed::DB_OBS_OK.0
    {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    if (*set).owner_confirmed_count > 0 as ::core::ffi::c_int {
        (*out).owner_confirmed = r#true != 0;
        (*out).has_classification = r#true != 0;
        (*out).classification = (*set).owner_confirmed[0usize];
    } else if (*set).automated_count > 0 as ::core::ffi::c_int {
        (*out).has_classification = r#true != 0;
        (*out).classification = (*set).automated[0usize];
    }
    return C2Rust_Unnamed::DB_OBS_OK.0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_link_detections(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut refs: *const observation_detection_ref_t,
    mut count: size_t,
    mut out: *mut observation_detection_link_t,
) -> ::core::ffi::c_int {
    if count == 0 as size_t {
        return C2Rust_Unnamed::DB_OBS_OK.0;
    }
    if refs.is_null() || out.is_null() {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        count.wrapping_mul(::core::mem::size_of::<observation_detection_link_t>()),
    );
    if !lightnvr_uuid_is_valid(camera_uuid) {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut set: *mut observation_classification_set_t =
        malloc(::core::mem::size_of::<observation_classification_set_t>())
            as *mut observation_classification_set_t;
    if set.is_null() {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        free(set as *mut ::core::ffi::c_void);
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut r#match: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut list: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        LINK_MATCH_SQL.as_ptr(),
        -1 as ::core::ffi::c_int,
        &raw mut r#match,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_prepare_v2(
            db,
            CLS_LIST_SQL.as_ptr(),
            -1 as ::core::ffi::c_int,
            &raw mut list,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
    {
        rc = C2Rust_Unnamed::DB_OBS_OK.0;
        let mut i: size_t = 0 as size_t;
        while rc == C2Rust_Unnamed::DB_OBS_OK.0 && i < count {
            rc = link_one(
                r#match,
                list,
                camera_uuid,
                refs.offset(i as isize),
                set,
                out.offset(i as isize),
            );
            i = i.wrapping_add(1);
        }
    }
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to link detections to observations: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        memset(
            out as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            count.wrapping_mul(::core::mem::size_of::<observation_detection_link_t>()),
        );
    }
    sqlite3_finalize(r#match);
    sqlite3_finalize(list);
    db_close_readonly_connection(db);
    free(set as *mut ::core::ffi::c_void);
    return rc;
}
