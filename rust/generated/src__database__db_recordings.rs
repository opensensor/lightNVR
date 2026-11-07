use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_limit(
        _: *mut sqlite3,
        id: ::core::ffi::c_int,
        newVal: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_column_type(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_classify_path(
        absolute_path: *const ::core::ffi::c_char,
        target_uuid: *mut ::core::ffi::c_char,
        object_key: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_resolve_path(
        target_uuid: *const ::core::ffi::c_char,
        object_key: *const ::core::ffi::c_char,
        absolute_path: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_recording_delete(
        recording_id: uint64_t,
        reason: *const ::core::ffi::c_char,
        local_bytes_removed: *mut uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_recording_delete_stale_incomplete(
        recording_id: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_recording_expire_age(
        recording_id: uint64_t,
        cutoff: int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn trim_ascii_whitespace(value: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
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
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type uint64_t = u64;
pub type uint_fast64_t = ::core::ffi::c_ulong;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct memory_order(pub ::core::ffi::c_uint);
impl memory_order {
    pub const memory_order_relaxed: Self = Self(0);
    pub const memory_order_consume: Self = Self(1);
    pub const memory_order_acquire: Self = Self(2);
    pub const memory_order_release: Self = Self(3);
    pub const memory_order_acq_rel: Self = Self(4);
    pub const memory_order_seq_cst: Self = Self(5);
}
pub type atomic_uint_fast64_t = uint_fast64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_metadata_t {
    pub id: uint64_t,
    pub stream_name: [::core::ffi::c_char; 64],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub file_path: [::core::ffi::c_char; 512],
    pub storage_target_uuid: [::core::ffi::c_char; 37],
    pub object_key: [::core::ffi::c_char; 512],
    pub placement_reason: [::core::ffi::c_char; 64],
    pub storage_policy_version: int64_t,
    pub start_time: time_t,
    pub end_time: time_t,
    pub size_bytes: uint64_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_int,
    pub codec: [::core::ffi::c_char; 16],
    pub is_complete: bool,
    pub playback_issue: [::core::ffi::c_char; 32],
    pub trigger_type: [::core::ffi::c_char; 16],
    pub protected: bool,
    pub retention_override_days: ::core::ffi::c_int,
    pub retention_tier: ::core::ffi::c_int,
    pub disk_pressure_eligible: bool,
    pub schedule_restricted: ::core::ffi::c_int,
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
pub const F_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_VARIABLE_NUMBER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const RETENTION_TIER_CRITICAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RETENTION_TIER_IMPORTANT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RETENTION_TIER_STANDARD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const RETENTION_TIER_EPHEMERAL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_MULTI_FILTER_VALUES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MAX_MULTI_FILTER_VALUE_LEN: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
static mut recordings_generation: atomic_uint_fast64_t = 1 as ::core::ffi::c_ulong;
unsafe extern "C" fn bump_recordings_generation() {
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut recordings_generation,
        1 as uint_fast64_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recordings_generation() -> uint64_t {
    return ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut recordings_generation,
    ) as uint64_t;
}
unsafe extern "C" fn backfill_recording_detection_links_locked(
    mut db: *mut sqlite3,
    mut recording_id: uint64_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut recording_sql: *const ::core::ffi::c_char = b"SELECT stream_name, camera_uuid, start_time, end_time FROM recordings WHERE id = ? AND is_complete = 1 AND end_time IS NOT NULL;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        recording_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare completed recording lookup: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, recording_id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc == SQLITE_DONE {
        sqlite3_finalize(stmt);
        return 0 as ::core::ffi::c_int;
    }
    if rc != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to load completed recording for detection backfill: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    let mut camera_uuid: [::core::ffi::c_char; 37] = [0; 37];
    let mut db_stream_name: *const ::core::ffi::c_uchar =
        sqlite3_column_text(stmt, 0 as ::core::ffi::c_int);
    let mut db_camera_uuid: *const ::core::ffi::c_uchar =
        sqlite3_column_text(stmt, 1 as ::core::ffi::c_int);
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        if !db_stream_name.is_null() {
            db_stream_name as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        if !db_camera_uuid.is_null() {
            db_camera_uuid as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    let mut start_time: time_t = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as time_t;
    let mut end_time: time_t = sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as time_t;
    sqlite3_finalize(stmt);
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut update_sql: *const ::core::ffi::c_char = b"UPDATE detections SET recording_id = ?,     camera_uuid = COALESCE(NULLIF(camera_uuid, ''), NULLIF(?, '')) WHERE recording_id IS NULL   AND stream_name = ?   AND timestamp >= ?   AND timestamp <= ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        update_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare recording detection-link backfill: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, recording_id as sqlite3_int64);
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        stmt,
        3 as ::core::ffi::c_int,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, start_time as sqlite3_int64);
    sqlite3_bind_int64(stmt, 5 as ::core::ffi::c_int, end_time as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to backfill recording detection links: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut linked: ::core::ffi::c_int = sqlite3_changes(db);
    sqlite3_finalize(stmt);
    return linked;
}
unsafe extern "C" fn parse_csv_filter_values(
    mut csv: *const ::core::ffi::c_char,
    mut values: *mut [::core::ffi::c_char; 128],
    mut max_values: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if csv.is_null() || *csv == 0 || max_values <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut buffer: [::core::ffi::c_char; 4096] = [0; 4096];
    safe_strcpy(
        &raw mut buffer as *mut ::core::ffi::c_char,
        csv,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        0 as size_t,
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token: *mut ::core::ffi::c_char = strtok_r(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b",\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !token.is_null() && count < max_values {
        token = trim_ascii_whitespace(token);
        if *token != 0 {
            let mut duplicate: bool = r#false != 0;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < count {
                if strcmp(
                    &raw mut *values.offset(i as isize) as *mut ::core::ffi::c_char,
                    token,
                ) == 0 as ::core::ffi::c_int
                {
                    duplicate = r#true != 0;
                    break;
                } else {
                    i += 1;
                }
            }
            if !duplicate {
                safe_strcpy(
                    &raw mut *values.offset(count as isize) as *mut ::core::ffi::c_char,
                    token,
                    MAX_MULTI_FILTER_VALUE_LEN as size_t,
                    0 as size_t,
                );
                count += 1;
            }
        }
        token = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_recording_metadata(
    mut metadata: *const recording_metadata_t,
) -> uint64_t {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut recording_id: uint64_t = 0 as uint64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as uint64_t;
    }
    if metadata.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Recording metadata is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as uint64_t;
    }
    let mut target_uuid: [::core::ffi::c_char; 37] = [
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
    let mut object_key: [::core::ffi::c_char; 512] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut placement_reason: *const ::core::ffi::c_char =
        if (*metadata).placement_reason[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*metadata).placement_reason as *const ::core::ffi::c_char
        } else {
            b"default-target\0".as_ptr() as *const ::core::ffi::c_char
        };
    let mut policy_version: int64_t = (*metadata).storage_policy_version;
    let mut has_target_uuid: bool =
        (*metadata).storage_target_uuid[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    let mut has_object_key: bool =
        (*metadata).object_key[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    if has_target_uuid as ::core::ffi::c_int != has_object_key as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Recording storage identity requires both target UUID and object key\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as uint64_t;
    }
    if has_target_uuid {
        let mut resolved_path: [::core::ffi::c_char; 512] = [0; 512];
        if db_storage_target_resolve_path(
            &raw const (*metadata).storage_target_uuid as *const ::core::ffi::c_char,
            &raw const (*metadata).object_key as *const ::core::ffi::c_char,
            &raw mut resolved_path as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut resolved_path as *mut ::core::ffi::c_char,
                &raw const (*metadata).file_path as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Recording storage identity does not resolve to its file path\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return 0 as uint64_t;
        }
        safe_strcpy(
            &raw mut target_uuid as *mut ::core::ffi::c_char,
            &raw const (*metadata).storage_target_uuid as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut object_key as *mut ::core::ffi::c_char,
            &raw const (*metadata).object_key as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    } else {
        db_storage_target_classify_path(
            &raw const (*metadata).file_path as *const ::core::ffi::c_char,
            &raw mut target_uuid as *mut ::core::ffi::c_char,
            &raw mut object_key as *mut ::core::ffi::c_char,
        );
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO recordings (stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, retention_tier, disk_pressure_eligible, schedule_restricted, camera_uuid, storage_target_uuid, object_key, placement_reason, storage_policy_version) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, COALESCE(NULLIF(?, ''), (SELECT camera_uuid FROM streams WHERE name = ?)), ?, ?, ?, ?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return 0 as uint64_t;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        &raw const (*metadata).stream_name as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw const (*metadata).file_path as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(
        stmt,
        3 as ::core::ffi::c_int,
        (*metadata).start_time as sqlite3_int64,
    );
    if (*metadata).end_time > 0 as time_t {
        sqlite3_bind_int64(
            stmt,
            4 as ::core::ffi::c_int,
            (*metadata).end_time as sqlite3_int64,
        );
    } else {
        sqlite3_bind_null(stmt, 4 as ::core::ffi::c_int);
    }
    sqlite3_bind_int64(
        stmt,
        5 as ::core::ffi::c_int,
        (*metadata).size_bytes as sqlite3_int64,
    );
    sqlite3_bind_int(stmt, 6 as ::core::ffi::c_int, (*metadata).width);
    sqlite3_bind_int(stmt, 7 as ::core::ffi::c_int, (*metadata).height);
    sqlite3_bind_int(stmt, 8 as ::core::ffi::c_int, (*metadata).fps);
    sqlite3_bind_text(
        stmt,
        9 as ::core::ffi::c_int,
        &raw const (*metadata).codec as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(
        stmt,
        10 as ::core::ffi::c_int,
        if (*metadata).is_complete as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    let mut trigger_type: *const ::core::ffi::c_char =
        if (*metadata).trigger_type[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            &raw const (*metadata).trigger_type as *const ::core::ffi::c_char
        } else {
            b"scheduled\0".as_ptr() as *const ::core::ffi::c_char
        };
    sqlite3_bind_text(
        stmt,
        11 as ::core::ffi::c_int,
        trigger_type,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    let mut retention_tier: ::core::ffi::c_int = (*metadata).retention_tier;
    if retention_tier < RETENTION_TIER_CRITICAL || retention_tier > RETENTION_TIER_EPHEMERAL {
        retention_tier = RETENTION_TIER_STANDARD;
    }
    sqlite3_bind_int(stmt, 12 as ::core::ffi::c_int, retention_tier);
    let mut pressure_eligible: bool =
        (*metadata).disk_pressure_eligible as ::core::ffi::c_int != 0 || !(*metadata).protected;
    sqlite3_bind_int(
        stmt,
        13 as ::core::ffi::c_int,
        if pressure_eligible as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    if (*metadata).schedule_restricted < 0 as ::core::ffi::c_int {
        sqlite3_bind_null(stmt, 14 as ::core::ffi::c_int);
    } else {
        sqlite3_bind_int(
            stmt,
            14 as ::core::ffi::c_int,
            if (*metadata).schedule_restricted != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
    }
    sqlite3_bind_text(
        stmt,
        15 as ::core::ffi::c_int,
        &raw const (*metadata).camera_uuid as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        16 as ::core::ffi::c_int,
        &raw const (*metadata).stream_name as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if target_uuid[0usize] != 0 {
        sqlite3_bind_text(
            stmt,
            17 as ::core::ffi::c_int,
            &raw mut target_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            18 as ::core::ffi::c_int,
            &raw mut object_key as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            19 as ::core::ffi::c_int,
            placement_reason,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    } else {
        sqlite3_bind_null(stmt, 17 as ::core::ffi::c_int);
        sqlite3_bind_null(stmt, 18 as ::core::ffi::c_int);
        sqlite3_bind_null(stmt, 19 as ::core::ffi::c_int);
    }
    if policy_version > 0 as int64_t {
        sqlite3_bind_int64(
            stmt,
            20 as ::core::ffi::c_int,
            policy_version as sqlite3_int64,
        );
    } else {
        sqlite3_bind_null(stmt, 20 as ::core::ffi::c_int);
    }
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to add recording metadata: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    } else {
        recording_id = sqlite3_last_insert_rowid(db) as uint64_t;
        bump_recordings_generation();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Added recording metadata with ID %llu\0".as_ptr() as *const ::core::ffi::c_char,
            recording_id as ::core::ffi::c_ulonglong,
        );
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return recording_id;
}
unsafe extern "C" fn update_recording_metadata_with_issue(
    mut id: uint64_t,
    mut end_time: time_t,
    mut size_bytes: uint64_t,
    mut is_complete: bool,
    mut playback_issue: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"UPDATE recordings SET end_time = ?, size_bytes = ?, is_complete = ?, playback_issue = COALESCE(?5, playback_issue) WHERE id = ?4;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, end_time as sqlite3_int64);
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, size_bytes as sqlite3_int64);
    sqlite3_bind_int(
        stmt,
        3 as ::core::ffi::c_int,
        if is_complete as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, id as sqlite3_int64);
    if !playback_issue.is_null() {
        sqlite3_bind_text(
            stmt,
            5 as ::core::ffi::c_int,
            playback_issue,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    } else {
        sqlite3_bind_null(stmt, 5 as ::core::ffi::c_int);
    }
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update recording metadata: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    bump_recordings_generation();
    if is_complete as ::core::ffi::c_int != 0 && end_time > 0 as time_t {
        let mut linked: ::core::ffi::c_int = backfill_recording_detection_links_locked(db, id);
        if linked > 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Linked %d unassigned detections to completed recording %llu\0".as_ptr()
                    as *const ::core::ffi::c_char,
                linked,
                id as ::core::ffi::c_ulonglong,
            );
        } else if linked < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Recording %llu completed but detection-link backfill failed\0".as_ptr()
                    as *const ::core::ffi::c_char,
                id as ::core::ffi::c_ulonglong,
            );
        }
    }
    pthread_mutex_unlock(db_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_recording_metadata(
    mut id: uint64_t,
    mut end_time: time_t,
    mut size_bytes: uint64_t,
    mut is_complete: bool,
) -> ::core::ffi::c_int {
    return update_recording_metadata_with_issue(
        id,
        end_time,
        size_bytes,
        is_complete,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn finalize_recording_reconciliation(
    mut id: uint64_t,
    mut end_time: time_t,
    mut size_bytes: uint64_t,
    mut playback_issue: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if playback_issue.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return update_recording_metadata_with_issue(
        id,
        end_time,
        size_bytes,
        r#true != 0,
        playback_issue,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_recording_start_time(
    mut id: uint64_t,
    mut start_time: time_t,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char =
        b"UPDATE recordings SET start_time = ? WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare update_recording_start_time statement: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, start_time as sqlite3_int64);
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update recording start_time (id=%llu): %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            id as ::core::ffi::c_ulonglong,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    bump_recordings_generation();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Corrected start_time for recording ID %llu to %ld\0".as_ptr()
            as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
        start_time,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_metadata_by_id(
    mut id: uint64_t,
    mut metadata: *mut recording_metadata_t,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if metadata.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_recording_metadata_by_id\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT r.id, r.stream_name, r.file_path, r.start_time, r.end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days, retention_tier, disk_pressure_eligible, schedule_restricted, camera_uuid, r.storage_target_uuid, r.object_key, r.placement_reason, r.storage_policy_version, r.playback_issue FROM recordings r WHERE r.id = ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, id as sqlite3_int64);
    if sqlite3_step(stmt) == SQLITE_ROW {
        (*metadata).id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        let mut stream: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !stream.is_null() {
            safe_strcpy(
                &raw mut (*metadata).stream_name as *mut ::core::ffi::c_char,
                stream,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else {
            (*metadata).stream_name[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut path: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !path.is_null() {
            safe_strcpy(
                &raw mut (*metadata).file_path as *mut ::core::ffi::c_char,
                path,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*metadata).file_path[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*metadata).start_time = sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as time_t;
        if sqlite3_column_type(stmt, 4 as ::core::ffi::c_int) != SQLITE_NULL {
            (*metadata).end_time = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as time_t;
        } else {
            (*metadata).end_time = 0 as time_t;
        }
        (*metadata).size_bytes = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as uint64_t;
        (*metadata).width = sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
        (*metadata).height = sqlite3_column_int(stmt, 7 as ::core::ffi::c_int);
        (*metadata).fps = sqlite3_column_int(stmt, 8 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 9 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*metadata).codec as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            (*metadata).codec[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*metadata).is_complete =
            sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut trigger_type: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 11 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !trigger_type.is_null() {
            safe_strcpy(
                &raw mut (*metadata).trigger_type as *mut ::core::ffi::c_char,
                trigger_type,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*metadata).trigger_type as *mut ::core::ffi::c_char,
                b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*metadata).protected =
            sqlite3_column_int(stmt, 12 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        if sqlite3_column_type(stmt, 13 as ::core::ffi::c_int) != SQLITE_NULL {
            (*metadata).retention_override_days =
                sqlite3_column_int(stmt, 13 as ::core::ffi::c_int);
        } else {
            (*metadata).retention_override_days = -1 as ::core::ffi::c_int;
        }
        (*metadata).retention_tier =
            if sqlite3_column_type(stmt, 14 as ::core::ffi::c_int) != SQLITE_NULL {
                sqlite3_column_int(stmt, 14 as ::core::ffi::c_int)
            } else {
                RETENTION_TIER_STANDARD
            };
        (*metadata).disk_pressure_eligible =
            if sqlite3_column_type(stmt, 15 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 15 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                r#true
            } != 0;
        (*metadata).schedule_restricted =
            if sqlite3_column_type(stmt, 16 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 16 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        let mut camera_uuid: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 17 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*metadata).camera_uuid as *mut ::core::ffi::c_char,
            if !camera_uuid.is_null() {
                camera_uuid
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        let mut target_uuid: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 18 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*metadata).storage_target_uuid as *mut ::core::ffi::c_char,
            if !target_uuid.is_null() {
                target_uuid
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        let mut object_key: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 19 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*metadata).object_key as *mut ::core::ffi::c_char,
            if !object_key.is_null() {
                object_key
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        let mut placement_reason: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 20 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*metadata).placement_reason as *mut ::core::ffi::c_char,
            if !placement_reason.is_null() {
                placement_reason
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        (*metadata).storage_policy_version =
            (if sqlite3_column_type(stmt, 21 as ::core::ffi::c_int) == SQLITE_NULL {
                0 as sqlite3_int64
            } else {
                sqlite3_column_int64(stmt, 21 as ::core::ffi::c_int)
            }) as int64_t;
        let mut playback_issue: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 22 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*metadata).playback_issue as *mut ::core::ffi::c_char,
            if !playback_issue.is_null() {
                playback_issue
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            0 as size_t,
        );
        result = 0 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    if result == 0 as ::core::ffi::c_int
        && (*metadata).storage_target_uuid[0usize] as ::core::ffi::c_int != 0
        && (*metadata).object_key[0usize] as ::core::ffi::c_int != 0
    {
        let mut resolved_path: [::core::ffi::c_char; 512] = [0; 512];
        if db_storage_target_resolve_path(
            &raw mut (*metadata).storage_target_uuid as *mut ::core::ffi::c_char,
            &raw mut (*metadata).object_key as *mut ::core::ffi::c_char,
            &raw mut resolved_path as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*metadata).file_path as *mut ::core::ffi::c_char,
                &raw mut resolved_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Could not resolve storage identity for recording %llu; using compatibility path\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                id as ::core::ffi::c_ulonglong,
            );
        }
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_metadata_by_path(
    mut file_path: *const ::core::ffi::c_char,
    mut metadata: *mut recording_metadata_t,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if file_path.is_null() || metadata.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_recording_metadata_by_path\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"SELECT id, stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days, retention_tier, disk_pressure_eligible, schedule_restricted, camera_uuid, playback_issue FROM recordings WHERE file_path = ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        file_path,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if sqlite3_step(stmt) == SQLITE_ROW {
        (*metadata).id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        let mut stream: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !stream.is_null() {
            safe_strcpy(
                &raw mut (*metadata).stream_name as *mut ::core::ffi::c_char,
                stream,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else {
            (*metadata).stream_name[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut path: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !path.is_null() {
            safe_strcpy(
                &raw mut (*metadata).file_path as *mut ::core::ffi::c_char,
                path,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*metadata).file_path[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*metadata).start_time = sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as time_t;
        if sqlite3_column_type(stmt, 4 as ::core::ffi::c_int) != SQLITE_NULL {
            (*metadata).end_time = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as time_t;
        } else {
            (*metadata).end_time = 0 as time_t;
        }
        (*metadata).size_bytes = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as uint64_t;
        (*metadata).width = sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
        (*metadata).height = sqlite3_column_int(stmt, 7 as ::core::ffi::c_int);
        (*metadata).fps = sqlite3_column_int(stmt, 8 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 9 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*metadata).codec as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            (*metadata).codec[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*metadata).is_complete =
            sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut trigger_type: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 11 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !trigger_type.is_null() {
            safe_strcpy(
                &raw mut (*metadata).trigger_type as *mut ::core::ffi::c_char,
                trigger_type,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*metadata).trigger_type as *mut ::core::ffi::c_char,
                b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*metadata).protected =
            sqlite3_column_int(stmt, 12 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        if sqlite3_column_type(stmt, 13 as ::core::ffi::c_int) != SQLITE_NULL {
            (*metadata).retention_override_days =
                sqlite3_column_int(stmt, 13 as ::core::ffi::c_int);
        } else {
            (*metadata).retention_override_days = -1 as ::core::ffi::c_int;
        }
        (*metadata).retention_tier =
            if sqlite3_column_type(stmt, 14 as ::core::ffi::c_int) != SQLITE_NULL {
                sqlite3_column_int(stmt, 14 as ::core::ffi::c_int)
            } else {
                RETENTION_TIER_STANDARD
            };
        (*metadata).disk_pressure_eligible =
            if sqlite3_column_type(stmt, 15 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 15 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                r#true
            } != 0;
        (*metadata).schedule_restricted =
            if sqlite3_column_type(stmt, 16 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 16 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        let mut camera_uuid: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 17 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*metadata).camera_uuid as *mut ::core::ffi::c_char,
            if !camera_uuid.is_null() {
                camera_uuid
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        let mut playback_issue: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 18 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*metadata).playback_issue as *mut ::core::ffi::c_char,
            if !playback_issue.is_null() {
                playback_issue
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            0 as size_t,
        );
        result = 0 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_metadata(
    mut start_time: time_t,
    mut end_time: time_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut metadata: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if metadata.is_null() || max_count <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_recording_metadata\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: [::core::ffi::c_char; 1024] = [0; 1024];
    snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"SELECT id, stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days, retention_tier, disk_pressure_eligible, schedule_restricted, camera_uuid, playback_issue FROM recordings WHERE is_complete = 1 AND end_time IS NOT NULL\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    if start_time > 0 as time_t {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND start_time >= ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        );
    }
    if end_time > 0 as time_t {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND start_time <= ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        );
    }
    if !stream_name.is_null() {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND stream_name = ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        );
    }
    safe_strcat(
        &raw mut sql as *mut ::core::ffi::c_char,
        b" ORDER BY start_time DESC LIMIT ?;\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    );
    rc = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut param_index: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if start_time > 0 as time_t {
        let c2rust_fresh0 = param_index;
        param_index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh0, start_time as sqlite3_int64);
    }
    if end_time > 0 as time_t {
        let c2rust_fresh1 = param_index;
        param_index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh1, end_time as sqlite3_int64);
    }
    if !stream_name.is_null() {
        let c2rust_fresh2 = param_index;
        param_index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh2,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
    sqlite3_bind_int(stmt, param_index, max_count);
    let mut rc_step: ::core::ffi::c_int = 0;
    loop {
        rc_step = sqlite3_step(stmt);
        if !(rc_step == SQLITE_ROW && count < max_count) {
            break;
        }
        (*metadata.offset(count as isize)).id =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        let mut stream: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !stream.is_null() {
            safe_strcpy(
                &raw mut (*metadata.offset(count as isize)).stream_name as *mut ::core::ffi::c_char,
                stream,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else {
            (*metadata.offset(count as isize)).stream_name[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut path: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !path.is_null() {
            safe_strcpy(
                &raw mut (*metadata.offset(count as isize)).file_path as *mut ::core::ffi::c_char,
                path,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*metadata.offset(count as isize)).file_path[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*metadata.offset(count as isize)).start_time =
            sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as time_t;
        if sqlite3_column_type(stmt, 4 as ::core::ffi::c_int) != SQLITE_NULL {
            (*metadata.offset(count as isize)).end_time =
                sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as time_t;
        } else {
            (*metadata.offset(count as isize)).end_time = 0 as time_t;
        }
        (*metadata.offset(count as isize)).size_bytes =
            sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as uint64_t;
        (*metadata.offset(count as isize)).width =
            sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
        (*metadata.offset(count as isize)).height =
            sqlite3_column_int(stmt, 7 as ::core::ffi::c_int);
        (*metadata.offset(count as isize)).fps = sqlite3_column_int(stmt, 8 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 9 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*metadata.offset(count as isize)).codec as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            (*metadata.offset(count as isize)).codec[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*metadata.offset(count as isize)).is_complete =
            sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut trigger_type: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 11 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !trigger_type.is_null() {
            safe_strcpy(
                &raw mut (*metadata.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                trigger_type,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*metadata.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*metadata.offset(count as isize)).protected =
            sqlite3_column_int(stmt, 12 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        if sqlite3_column_type(stmt, 13 as ::core::ffi::c_int) != SQLITE_NULL {
            (*metadata.offset(count as isize)).retention_override_days =
                sqlite3_column_int(stmt, 13 as ::core::ffi::c_int);
        } else {
            (*metadata.offset(count as isize)).retention_override_days = -1 as ::core::ffi::c_int;
        }
        (*metadata.offset(count as isize)).retention_tier =
            if sqlite3_column_type(stmt, 14 as ::core::ffi::c_int) != SQLITE_NULL {
                sqlite3_column_int(stmt, 14 as ::core::ffi::c_int)
            } else {
                RETENTION_TIER_STANDARD
            };
        (*metadata.offset(count as isize)).disk_pressure_eligible =
            if sqlite3_column_type(stmt, 15 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 15 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                r#true
            } != 0;
        (*metadata.offset(count as isize)).schedule_restricted =
            if sqlite3_column_type(stmt, 16 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 16 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        let mut camera_uuid: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 17 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*metadata.offset(count as isize)).camera_uuid as *mut ::core::ffi::c_char,
            if !camera_uuid.is_null() {
                camera_uuid
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        let mut playback_issue: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 18 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*metadata.offset(count as isize)).playback_issue as *mut ::core::ffi::c_char,
            if !playback_issue.is_null() {
                playback_issue
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            0 as size_t,
        );
        count += 1;
    }
    if rc_step != SQLITE_DONE && rc_step != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Error while fetching recordings: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Found %d recordings in database matching criteria\0".as_ptr()
            as *const ::core::ffi::c_char,
        count,
    );
    return count;
}
unsafe extern "C" fn scope_recording_detection_sql(
    mut sql: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> ::core::ffi::c_int {
    let mut needles: [*const ::core::ffi::c_char; 2] = [
        b"d.stream_name = r.stream_name\0".as_ptr() as *const ::core::ffi::c_char,
        b"d.recording_id = r.id\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut replacements: [*const ::core::ffi::c_char; 2] = [
        b"((d.camera_uuid != '' AND d.camera_uuid = COALESCE(NULLIF(r.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=r.stream_name))) OR ((d.camera_uuid IS NULL OR d.camera_uuid = '') AND d.stream_name IN (SELECT name FROM streams WHERE camera_uuid = COALESCE(NULLIF(r.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=r.stream_name)))))\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"d.recording_id = r.id AND ((d.camera_uuid != '' AND d.camera_uuid = COALESCE(NULLIF(r.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=r.stream_name))) OR ((d.camera_uuid IS NULL OR d.camera_uuid = '') AND d.stream_name IN (SELECT name FROM streams WHERE camera_uuid = COALESCE(NULLIF(r.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=r.stream_name)))))\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        let mut cursor: *mut ::core::ffi::c_char = sql;
        let mut extra: size_t = strlen(replacements[i]).wrapping_sub(strlen(needles[i]));
        loop {
            cursor = strstr(cursor, needles[i]);
            if cursor.is_null() {
                break;
            }
            if strlen(sql).wrapping_add(extra) >= capacity {
                return -1 as ::core::ffi::c_int;
            }
            memmove(
                cursor.offset(strlen(replacements[i]) as isize) as *mut ::core::ffi::c_void,
                cursor.offset(strlen(needles[i]) as isize) as *const ::core::ffi::c_void,
                strlen(cursor.offset(strlen(needles[i]) as isize)).wrapping_add(1 as size_t),
            );
            memcpy(
                cursor as *mut ::core::ffi::c_void,
                replacements[i] as *const ::core::ffi::c_void,
                strlen(replacements[i]),
            );
            cursor = cursor.offset(strlen(replacements[i]) as isize);
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_count(
    mut start_time: time_t,
    mut end_time: time_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut has_detection: ::core::ffi::c_int,
    mut detection_label: *const ::core::ffi::c_char,
    mut protected_filter: ::core::ffi::c_int,
    mut allowed_streams: *const *const ::core::ffi::c_char,
    mut allowed_streams_count: ::core::ffi::c_int,
    mut tag_filter: *const ::core::ffi::c_char,
    mut capture_method_filter: *const ::core::ffi::c_char,
    mut include_in_progress: bool,
) -> ::core::ffi::c_int {
    return get_recording_count_scoped(
        start_time,
        end_time,
        stream_name,
        has_detection,
        detection_label,
        protected_filter,
        allowed_streams,
        allowed_streams_count,
        tag_filter,
        capture_method_filter,
        include_in_progress,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_count_for_cameras(
    mut start_time: time_t,
    mut end_time: time_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut has_detection: ::core::ffi::c_int,
    mut detection_label: *const ::core::ffi::c_char,
    mut protected_filter: ::core::ffi::c_int,
    mut allowed_streams: *const *const ::core::ffi::c_char,
    mut allowed_streams_count: ::core::ffi::c_int,
    mut tag_filter: *const ::core::ffi::c_char,
    mut capture_method_filter: *const ::core::ffi::c_char,
    mut include_in_progress: bool,
) -> ::core::ffi::c_int {
    if allowed_streams.is_null() || allowed_streams_count <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if allowed_streams_count > MAX_STREAMS {
        return -1 as ::core::ffi::c_int;
    }
    return get_recording_count_scoped(
        start_time,
        end_time,
        stream_name,
        has_detection,
        detection_label,
        protected_filter,
        allowed_streams,
        allowed_streams_count,
        tag_filter,
        capture_method_filter,
        include_in_progress,
        r#true != 0,
    );
}
unsafe extern "C" fn get_recording_count_scoped(
    mut start_time: time_t,
    mut end_time: time_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut has_detection: ::core::ffi::c_int,
    mut detection_label: *const ::core::ffi::c_char,
    mut protected_filter: ::core::ffi::c_int,
    mut allowed_streams: *const *const ::core::ffi::c_char,
    mut allowed_streams_count: ::core::ffi::c_int,
    mut tag_filter: *const ::core::ffi::c_char,
    mut capture_method_filter: *const ::core::ffi::c_char,
    mut include_in_progress: bool,
    mut camera_scope: bool,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream_filters: [[::core::ffi::c_char; 128]; 32] = [
        [
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
        ],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
    ];
    let mut detection_label_filters: [[::core::ffi::c_char; 128]; 32] = [
        [
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
        ],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
    ];
    let mut tag_filters: [[::core::ffi::c_char; 128]; 32] = [
        [
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
        ],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
    ];
    let mut capture_method_filters: [[::core::ffi::c_char; 128]; 32] = [
        [
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
        ],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
    ];
    let mut stream_filter_count: ::core::ffi::c_int = parse_csv_filter_values(
        stream_name,
        &raw mut stream_filters as *mut [::core::ffi::c_char; 128],
        MAX_MULTI_FILTER_VALUES,
    );
    let mut detection_label_count: ::core::ffi::c_int = parse_csv_filter_values(
        detection_label,
        &raw mut detection_label_filters as *mut [::core::ffi::c_char; 128],
        MAX_MULTI_FILTER_VALUES,
    );
    let mut tag_filter_count: ::core::ffi::c_int = parse_csv_filter_values(
        tag_filter,
        &raw mut tag_filters as *mut [::core::ffi::c_char; 128],
        MAX_MULTI_FILTER_VALUES,
    );
    let mut capture_method_count: ::core::ffi::c_int = parse_csv_filter_values(
        capture_method_filter,
        &raw mut capture_method_filters as *mut [::core::ffi::c_char; 128],
        MAX_MULTI_FILTER_VALUES,
    );
    let mut sql: [::core::ffi::c_char; 32768] = [0; 32768];
    if include_in_progress {
        safe_strcpy(
            &raw mut sql as *mut ::core::ffi::c_char,
            b"SELECT COUNT(*) FROM recordings r LEFT JOIN streams s ON s.name = r.stream_name WHERE ((r.is_complete = 1 AND r.end_time IS NOT NULL) OR (r.is_complete = 0 AND COALESCE(r.end_time, 0) = 0 AND r.start_time >= CAST(strftime('%s','now') AS INTEGER) - (2 * COALESCE(NULLIF(s.segment_duration, 0), 900))))\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            0 as size_t,
        );
    } else {
        safe_strcpy(
            &raw mut sql as *mut ::core::ffi::c_char,
            b"SELECT COUNT(*) FROM recordings r WHERE r.is_complete = 1 AND r.end_time IS NOT NULL\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            0 as size_t,
        );
    }
    if has_detection == 1 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND (r.trigger_type = 'detection' OR EXISTS (SELECT 1 FROM detections d WHERE d.recording_id = r.id) OR EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source != 'external_motion' AND d.timestamp >= r.start_time AND d.timestamp <= r.end_time) OR EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source = 'external_motion' AND d.timestamp <= r.end_time AND COALESCE(d.event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= r.start_time))\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding detection filter (trigger_type OR recording_id OR timestamp range)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if has_detection == -1 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND (r.trigger_type != 'detection' OR r.trigger_type IS NULL) AND NOT EXISTS (SELECT 1 FROM detections d WHERE d.recording_id = r.id) AND NOT EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source != 'external_motion' AND d.timestamp >= r.start_time AND d.timestamp <= r.end_time) AND NOT EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source = 'external_motion' AND d.timestamp <= r.end_time AND COALESCE(d.event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= r.start_time)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding no-detection filter (no trigger_type AND no linked detections)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if detection_label_count > 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND (EXISTS (SELECT 1 FROM detections d WHERE d.recording_id = r.id AND (\0".as_ptr()
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < detection_label_count {
            if i > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b" OR \0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"d.label LIKE ?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")) OR EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source != 'external_motion' AND d.timestamp >= r.start_time AND d.timestamp <= r.end_time AND (\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < detection_label_count {
            if i_0 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b" OR \0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"d.label LIKE ?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_0 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")) OR EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source = 'external_motion' AND d.timestamp <= r.end_time AND COALESCE(d.event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= r.start_time AND (\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < detection_label_count {
            if i_1 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b" OR \0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"d.label LIKE ?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_1 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")))\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding %d detection_label filters\0".as_ptr() as *const ::core::ffi::c_char,
            detection_label_count,
        );
    }
    if start_time > 0 as time_t {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND r.start_time >= ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding start_time filter: %ld\0".as_ptr() as *const ::core::ffi::c_char,
            start_time,
        );
    }
    if end_time > 0 as time_t {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND r.start_time <= ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding end_time filter: %ld\0".as_ptr() as *const ::core::ffi::c_char,
            end_time,
        );
    }
    if stream_filter_count > 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND r.stream_name IN (\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < stream_filter_count {
            if i_2 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_2 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
    }
    if !allowed_streams.is_null() && allowed_streams_count > 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            if camera_scope as ::core::ffi::c_int != 0 {
                b" AND COALESCE(NULLIF(r.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=r.stream_name)) IN (\0"
                    .as_ptr() as *const ::core::ffi::c_char
            } else {
                b" AND r.stream_name IN (\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_3 < allowed_streams_count {
            if i_3 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_3 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding allowed_streams IN filter (%d streams)\0".as_ptr()
                as *const ::core::ffi::c_char,
            allowed_streams_count,
        );
    }
    if protected_filter == 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND r.protected = 0\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding protected_filter=0 (unprotected only)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if protected_filter == 1 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND r.protected = 1\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding protected_filter=1 (protected only)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if tag_filter_count > 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND EXISTS (SELECT 1 FROM recording_tags rt WHERE rt.recording_id = r.id AND rt.tag IN (\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_4 < tag_filter_count {
            if i_4 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_4 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b"))\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding %d tag filters\0".as_ptr() as *const ::core::ffi::c_char,
            tag_filter_count,
        );
    }
    if capture_method_count > 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND CASE WHEN COALESCE(r.trigger_type, 'scheduled') = 'scheduled' AND r.schedule_restricted = 0 THEN 'continuous' ELSE COALESCE(r.trigger_type, 'scheduled') END IN (\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_5 < capture_method_count {
            if i_5 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_5 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding %d capture_method filters\0".as_ptr() as *const ::core::ffi::c_char,
            capture_method_count,
        );
    }
    if camera_scope as ::core::ffi::c_int != 0
        && scope_recording_detection_sql(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"SQL query for get_recording_count: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sql as *mut ::core::ffi::c_char,
    );
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    rc = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    let mut param_index: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if detection_label_count > 0 as ::core::ffi::c_int {
        let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_6 < detection_label_count {
            let mut label_pattern: [::core::ffi::c_char; 130] = [0; 130];
            snprintf(
                &raw mut label_pattern as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 130]>(),
                b"%%%s%%\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut *(&raw mut detection_label_filters as *mut [::core::ffi::c_char; 128])
                    .offset(i_6 as isize) as *mut ::core::ffi::c_char,
            );
            let c2rust_fresh3 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh3,
                &raw mut label_pattern as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            i_6 += 1;
        }
        let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_7 < detection_label_count {
            let mut label_pattern_0: [::core::ffi::c_char; 130] = [0; 130];
            snprintf(
                &raw mut label_pattern_0 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 130]>(),
                b"%%%s%%\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut *(&raw mut detection_label_filters as *mut [::core::ffi::c_char; 128])
                    .offset(i_7 as isize) as *mut ::core::ffi::c_char,
            );
            let c2rust_fresh4 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh4,
                &raw mut label_pattern_0 as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            i_7 += 1;
        }
        let mut i_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_8 < detection_label_count {
            let mut label_pattern_1: [::core::ffi::c_char; 130] = [0; 130];
            snprintf(
                &raw mut label_pattern_1 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 130]>(),
                b"%%%s%%\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut *(&raw mut detection_label_filters as *mut [::core::ffi::c_char; 128])
                    .offset(i_8 as isize) as *mut ::core::ffi::c_char,
            );
            let c2rust_fresh5 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh5,
                &raw mut label_pattern_1 as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            i_8 += 1;
        }
    }
    if start_time > 0 as time_t {
        let c2rust_fresh6 = param_index;
        param_index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh6, start_time as sqlite3_int64);
    }
    if end_time > 0 as time_t {
        let c2rust_fresh7 = param_index;
        param_index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh7, end_time as sqlite3_int64);
    }
    if stream_filter_count > 0 as ::core::ffi::c_int {
        let mut i_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_9 < stream_filter_count {
            let c2rust_fresh8 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh8,
                &raw mut *(&raw mut stream_filters as *mut [::core::ffi::c_char; 128])
                    .offset(i_9 as isize) as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            i_9 += 1;
        }
    }
    if !allowed_streams.is_null() && allowed_streams_count > 0 as ::core::ffi::c_int {
        let mut i_10: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_10 < allowed_streams_count {
            let c2rust_fresh9 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh9,
                *allowed_streams.offset(i_10 as isize),
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            i_10 += 1;
        }
    }
    let mut i_11: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_11 < tag_filter_count {
        let c2rust_fresh10 = param_index;
        param_index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh10,
            &raw mut *(&raw mut tag_filters as *mut [::core::ffi::c_char; 128])
                .offset(i_11 as isize) as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        i_11 += 1;
    }
    let mut i_12: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_12 < capture_method_count {
        let c2rust_fresh11 = param_index;
        param_index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh11,
            &raw mut *(&raw mut capture_method_filters as *mut [::core::ffi::c_char; 128])
                .offset(i_12 as isize) as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        i_12 += 1;
    }
    if sqlite3_step(stmt) == SQLITE_ROW {
        count = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Error while getting recording count: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        count = -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Total count of recordings matching criteria: %d\0".as_ptr() as *const ::core::ffi::c_char,
        count,
    );
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_metadata_paginated(
    mut start_time: time_t,
    mut end_time: time_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut has_detection: ::core::ffi::c_int,
    mut detection_label: *const ::core::ffi::c_char,
    mut protected_filter: ::core::ffi::c_int,
    mut sort_field: *const ::core::ffi::c_char,
    mut sort_order: *const ::core::ffi::c_char,
    mut metadata: *mut recording_metadata_t,
    mut limit: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut allowed_streams: *const *const ::core::ffi::c_char,
    mut allowed_streams_count: ::core::ffi::c_int,
    mut tag_filter: *const ::core::ffi::c_char,
    mut capture_method_filter: *const ::core::ffi::c_char,
    mut include_in_progress: bool,
) -> ::core::ffi::c_int {
    return get_recording_metadata_paginated_scoped(
        start_time,
        end_time,
        stream_name,
        has_detection,
        detection_label,
        protected_filter,
        sort_field,
        sort_order,
        metadata,
        limit,
        offset,
        allowed_streams,
        allowed_streams_count,
        tag_filter,
        capture_method_filter,
        include_in_progress,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_metadata_paginated_for_cameras(
    mut start_time: time_t,
    mut end_time: time_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut has_detection: ::core::ffi::c_int,
    mut detection_label: *const ::core::ffi::c_char,
    mut protected_filter: ::core::ffi::c_int,
    mut sort_field: *const ::core::ffi::c_char,
    mut sort_order: *const ::core::ffi::c_char,
    mut metadata: *mut recording_metadata_t,
    mut limit: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut allowed_streams: *const *const ::core::ffi::c_char,
    mut allowed_streams_count: ::core::ffi::c_int,
    mut tag_filter: *const ::core::ffi::c_char,
    mut capture_method_filter: *const ::core::ffi::c_char,
    mut include_in_progress: bool,
) -> ::core::ffi::c_int {
    if allowed_streams.is_null() || allowed_streams_count <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if allowed_streams_count > MAX_STREAMS {
        return -1 as ::core::ffi::c_int;
    }
    return get_recording_metadata_paginated_scoped(
        start_time,
        end_time,
        stream_name,
        has_detection,
        detection_label,
        protected_filter,
        sort_field,
        sort_order,
        metadata,
        limit,
        offset,
        allowed_streams,
        allowed_streams_count,
        tag_filter,
        capture_method_filter,
        include_in_progress,
        r#true != 0,
    );
}
unsafe extern "C" fn get_recording_metadata_paginated_scoped(
    mut start_time: time_t,
    mut end_time: time_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut has_detection: ::core::ffi::c_int,
    mut detection_label: *const ::core::ffi::c_char,
    mut protected_filter: ::core::ffi::c_int,
    mut sort_field: *const ::core::ffi::c_char,
    mut sort_order: *const ::core::ffi::c_char,
    mut metadata: *mut recording_metadata_t,
    mut limit: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
    mut allowed_streams: *const *const ::core::ffi::c_char,
    mut allowed_streams_count: ::core::ffi::c_int,
    mut tag_filter: *const ::core::ffi::c_char,
    mut capture_method_filter: *const ::core::ffi::c_char,
    mut include_in_progress: bool,
    mut camera_scope: bool,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream_filters: [[::core::ffi::c_char; 128]; 32] = [
        [
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
        ],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
    ];
    let mut detection_label_filters: [[::core::ffi::c_char; 128]; 32] = [
        [
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
        ],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
    ];
    let mut tag_filters: [[::core::ffi::c_char; 128]; 32] = [
        [
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
        ],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
    ];
    let mut capture_method_filters: [[::core::ffi::c_char; 128]; 32] = [
        [
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
        ],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
        [0; 128],
    ];
    let mut stream_filter_count: ::core::ffi::c_int = parse_csv_filter_values(
        stream_name,
        &raw mut stream_filters as *mut [::core::ffi::c_char; 128],
        MAX_MULTI_FILTER_VALUES,
    );
    let mut detection_label_count: ::core::ffi::c_int = parse_csv_filter_values(
        detection_label,
        &raw mut detection_label_filters as *mut [::core::ffi::c_char; 128],
        MAX_MULTI_FILTER_VALUES,
    );
    let mut tag_filter_count: ::core::ffi::c_int = parse_csv_filter_values(
        tag_filter,
        &raw mut tag_filters as *mut [::core::ffi::c_char; 128],
        MAX_MULTI_FILTER_VALUES,
    );
    let mut capture_method_count: ::core::ffi::c_int = parse_csv_filter_values(
        capture_method_filter,
        &raw mut capture_method_filters as *mut [::core::ffi::c_char; 128],
        MAX_MULTI_FILTER_VALUES,
    );
    if metadata.is_null() || limit <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_recording_metadata_paginated\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut safe_sort_field: [::core::ffi::c_char; 32] =
        ::core::mem::transmute::<[u8; 32], [::core::ffi::c_char; 32]>(
            *b"start_time\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
    if !sort_field.is_null() {
        if strcmp(sort_field, b"id\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(
                sort_field,
                b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                sort_field,
                b"start_time\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                sort_field,
                b"end_time\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                sort_field,
                b"size_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut safe_sort_field as *mut ::core::ffi::c_char,
                sort_field,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                0 as size_t,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Invalid sort field: %s, using default\0".as_ptr() as *const ::core::ffi::c_char,
                sort_field,
            );
        }
    }
    let mut safe_sort_order: [::core::ffi::c_char; 8] =
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"DESC\0\0\0\0");
    if !sort_order.is_null() {
        if strcasecmp(sort_order, b"asc\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            strcpy(
                &raw mut safe_sort_order as *mut ::core::ffi::c_char,
                b"ASC\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if strcasecmp(sort_order, b"desc\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            strcpy(
                &raw mut safe_sort_order as *mut ::core::ffi::c_char,
                b"DESC\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Invalid sort order: %s, using default\0".as_ptr() as *const ::core::ffi::c_char,
                sort_order,
            );
        }
    }
    let mut sql: [::core::ffi::c_char; 32768] = [0; 32768];
    if include_in_progress {
        safe_strcpy(
            &raw mut sql as *mut ::core::ffi::c_char,
            b"SELECT r.id, r.stream_name, r.file_path, r.start_time, r.end_time, r.size_bytes, r.width, r.height, r.fps, r.codec, r.is_complete, r.trigger_type, r.protected, r.retention_override_days, r.retention_tier, r.disk_pressure_eligible, r.schedule_restricted, r.camera_uuid, r.playback_issue FROM recordings r LEFT JOIN streams s ON s.name = r.stream_name WHERE ((r.is_complete = 1 AND r.end_time IS NOT NULL) OR (r.is_complete = 0 AND COALESCE(r.end_time, 0) = 0 AND r.start_time >= CAST(strftime('%s','now') AS INTEGER) - (2 * COALESCE(NULLIF(s.segment_duration, 0), 900))))\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            0 as size_t,
        );
    } else {
        safe_strcpy(
            &raw mut sql as *mut ::core::ffi::c_char,
            b"SELECT r.id, r.stream_name, r.file_path, r.start_time, r.end_time, r.size_bytes, r.width, r.height, r.fps, r.codec, r.is_complete, r.trigger_type, r.protected, r.retention_override_days, r.retention_tier, r.disk_pressure_eligible, r.schedule_restricted, r.camera_uuid, r.playback_issue FROM recordings r WHERE r.is_complete = 1 AND r.end_time IS NOT NULL\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            0 as size_t,
        );
    }
    if has_detection == 1 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND (r.trigger_type = 'detection' OR EXISTS (SELECT 1 FROM detections d WHERE d.recording_id = r.id) OR EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source != 'external_motion' AND d.timestamp >= r.start_time AND d.timestamp <= r.end_time) OR EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source = 'external_motion' AND d.timestamp <= r.end_time AND COALESCE(d.event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= r.start_time))\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding detection filter (trigger_type OR recording_id OR timestamp range)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if has_detection == -1 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND (r.trigger_type != 'detection' OR r.trigger_type IS NULL) AND NOT EXISTS (SELECT 1 FROM detections d WHERE d.recording_id = r.id) AND NOT EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source != 'external_motion' AND d.timestamp >= r.start_time AND d.timestamp <= r.end_time) AND NOT EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source = 'external_motion' AND d.timestamp <= r.end_time AND COALESCE(d.event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= r.start_time)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding no-detection filter (no trigger_type AND no linked detections)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if detection_label_count > 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND (EXISTS (SELECT 1 FROM detections d WHERE d.recording_id = r.id AND (\0".as_ptr()
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < detection_label_count {
            if i > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b" OR \0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"d.label LIKE ?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")) OR EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source != 'external_motion' AND d.timestamp >= r.start_time AND d.timestamp <= r.end_time AND (\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < detection_label_count {
            if i_0 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b" OR \0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"d.label LIKE ?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_0 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")) OR EXISTS (SELECT 1 FROM detections d WHERE d.stream_name = r.stream_name AND d.source = 'external_motion' AND d.timestamp <= r.end_time AND COALESCE(d.event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= r.start_time AND (\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < detection_label_count {
            if i_1 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b" OR \0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"d.label LIKE ?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_1 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")))\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding %d detection_label filters\0".as_ptr() as *const ::core::ffi::c_char,
            detection_label_count,
        );
    }
    if start_time > 0 as time_t {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND r.start_time >= ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding start_time filter to paginated query: %ld\0".as_ptr()
                as *const ::core::ffi::c_char,
            start_time,
        );
    }
    if end_time > 0 as time_t {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND r.start_time <= ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding end_time filter to paginated query: %ld\0".as_ptr()
                as *const ::core::ffi::c_char,
            end_time,
        );
    }
    if stream_filter_count > 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND r.stream_name IN (\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < stream_filter_count {
            if i_2 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_2 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
    }
    if !allowed_streams.is_null() && allowed_streams_count > 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            if camera_scope as ::core::ffi::c_int != 0 {
                b" AND COALESCE(NULLIF(r.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=r.stream_name)) IN (\0"
                    .as_ptr() as *const ::core::ffi::c_char
            } else {
                b" AND r.stream_name IN (\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_3 < allowed_streams_count {
            if i_3 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_3 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding allowed_streams IN filter (%d streams) to paginated query\0".as_ptr()
                as *const ::core::ffi::c_char,
            allowed_streams_count,
        );
    }
    if protected_filter == 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND r.protected = 0\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding protected_filter=0 (unprotected only) to paginated query\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if protected_filter == 1 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND r.protected = 1\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding protected_filter=1 (protected only) to paginated query\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if tag_filter_count > 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND EXISTS (SELECT 1 FROM recording_tags rt WHERE rt.recording_id = r.id AND rt.tag IN (\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_4 < tag_filter_count {
            if i_4 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_4 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b"))\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding %d tag filters to paginated query\0".as_ptr() as *const ::core::ffi::c_char,
            tag_filter_count,
        );
    }
    if capture_method_count > 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND CASE WHEN COALESCE(r.trigger_type, 'scheduled') = 'scheduled' AND r.schedule_restricted = 0 THEN 'continuous' ELSE COALESCE(r.trigger_type, 'scheduled') END IN (\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_5 < capture_method_count {
            if i_5 > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
            );
            i_5 += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b")\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Adding %d capture_method filters to paginated query\0".as_ptr()
                as *const ::core::ffi::c_char,
            capture_method_count,
        );
    }
    let mut order_clause: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut order_clause as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b" ORDER BY r.%s %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut safe_sort_field as *mut ::core::ffi::c_char,
        &raw mut safe_sort_order as *mut ::core::ffi::c_char,
    );
    safe_strcat(
        &raw mut sql as *mut ::core::ffi::c_char,
        &raw mut order_clause as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
    );
    let mut limit_clause: [::core::ffi::c_char; 64] = [0; 64];
    safe_strcpy(
        &raw mut limit_clause as *mut ::core::ffi::c_char,
        b" LIMIT ? OFFSET ?\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcat(
        &raw mut sql as *mut ::core::ffi::c_char,
        &raw mut limit_clause as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
    );
    if camera_scope as ::core::ffi::c_int != 0
        && scope_recording_detection_sql(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32768]>(),
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"SQL query for get_recording_metadata_paginated: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut sql as *mut ::core::ffi::c_char,
    );
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    rc = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    let mut param_index: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if detection_label_count > 0 as ::core::ffi::c_int {
        let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_6 < detection_label_count {
            let mut label_pattern: [::core::ffi::c_char; 130] = [0; 130];
            snprintf(
                &raw mut label_pattern as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 130]>(),
                b"%%%s%%\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut *(&raw mut detection_label_filters as *mut [::core::ffi::c_char; 128])
                    .offset(i_6 as isize) as *mut ::core::ffi::c_char,
            );
            let c2rust_fresh12 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh12,
                &raw mut label_pattern as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            i_6 += 1;
        }
        let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_7 < detection_label_count {
            let mut label_pattern_0: [::core::ffi::c_char; 130] = [0; 130];
            snprintf(
                &raw mut label_pattern_0 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 130]>(),
                b"%%%s%%\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut *(&raw mut detection_label_filters as *mut [::core::ffi::c_char; 128])
                    .offset(i_7 as isize) as *mut ::core::ffi::c_char,
            );
            let c2rust_fresh13 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh13,
                &raw mut label_pattern_0 as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            i_7 += 1;
        }
        let mut i_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_8 < detection_label_count {
            let mut label_pattern_1: [::core::ffi::c_char; 130] = [0; 130];
            snprintf(
                &raw mut label_pattern_1 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 130]>(),
                b"%%%s%%\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut *(&raw mut detection_label_filters as *mut [::core::ffi::c_char; 128])
                    .offset(i_8 as isize) as *mut ::core::ffi::c_char,
            );
            let c2rust_fresh14 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh14,
                &raw mut label_pattern_1 as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            i_8 += 1;
        }
    }
    if start_time > 0 as time_t {
        let c2rust_fresh15 = param_index;
        param_index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh15, start_time as sqlite3_int64);
    }
    if end_time > 0 as time_t {
        let c2rust_fresh16 = param_index;
        param_index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh16, end_time as sqlite3_int64);
    }
    if stream_filter_count > 0 as ::core::ffi::c_int {
        let mut i_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_9 < stream_filter_count {
            let c2rust_fresh17 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh17,
                &raw mut *(&raw mut stream_filters as *mut [::core::ffi::c_char; 128])
                    .offset(i_9 as isize) as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            i_9 += 1;
        }
    }
    if !allowed_streams.is_null() && allowed_streams_count > 0 as ::core::ffi::c_int {
        let mut i_10: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_10 < allowed_streams_count {
            let c2rust_fresh18 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh18,
                *allowed_streams.offset(i_10 as isize),
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            i_10 += 1;
        }
    }
    let mut i_11: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_11 < tag_filter_count {
        let c2rust_fresh19 = param_index;
        param_index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh19,
            &raw mut *(&raw mut tag_filters as *mut [::core::ffi::c_char; 128])
                .offset(i_11 as isize) as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        i_11 += 1;
    }
    let mut i_12: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_12 < capture_method_count {
        let c2rust_fresh20 = param_index;
        param_index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh20,
            &raw mut *(&raw mut capture_method_filters as *mut [::core::ffi::c_char; 128])
                .offset(i_12 as isize) as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        i_12 += 1;
    }
    let c2rust_fresh21 = param_index;
    param_index += 1;
    sqlite3_bind_int(stmt, c2rust_fresh21, limit);
    sqlite3_bind_int(stmt, param_index, offset);
    let mut rc_step: ::core::ffi::c_int = 0;
    loop {
        rc_step = sqlite3_step(stmt);
        if !(rc_step == SQLITE_ROW && count < limit) {
            break;
        }
        (*metadata.offset(count as isize)).id =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        let mut stream: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !stream.is_null() {
            safe_strcpy(
                &raw mut (*metadata.offset(count as isize)).stream_name as *mut ::core::ffi::c_char,
                stream,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else {
            (*metadata.offset(count as isize)).stream_name[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut path: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !path.is_null() {
            safe_strcpy(
                &raw mut (*metadata.offset(count as isize)).file_path as *mut ::core::ffi::c_char,
                path,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*metadata.offset(count as isize)).file_path[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*metadata.offset(count as isize)).start_time =
            sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as time_t;
        if sqlite3_column_type(stmt, 4 as ::core::ffi::c_int) != SQLITE_NULL {
            (*metadata.offset(count as isize)).end_time =
                sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as time_t;
        } else {
            (*metadata.offset(count as isize)).end_time = 0 as time_t;
        }
        (*metadata.offset(count as isize)).size_bytes =
            sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as uint64_t;
        (*metadata.offset(count as isize)).width =
            sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
        (*metadata.offset(count as isize)).height =
            sqlite3_column_int(stmt, 7 as ::core::ffi::c_int);
        (*metadata.offset(count as isize)).fps = sqlite3_column_int(stmt, 8 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 9 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*metadata.offset(count as isize)).codec as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            (*metadata.offset(count as isize)).codec[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*metadata.offset(count as isize)).is_complete =
            sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut trigger_type: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 11 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !trigger_type.is_null() {
            safe_strcpy(
                &raw mut (*metadata.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                trigger_type,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*metadata.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*metadata.offset(count as isize)).protected =
            sqlite3_column_int(stmt, 12 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        if sqlite3_column_type(stmt, 13 as ::core::ffi::c_int) != SQLITE_NULL {
            (*metadata.offset(count as isize)).retention_override_days =
                sqlite3_column_int(stmt, 13 as ::core::ffi::c_int);
        } else {
            (*metadata.offset(count as isize)).retention_override_days = -1 as ::core::ffi::c_int;
        }
        (*metadata.offset(count as isize)).retention_tier =
            if sqlite3_column_type(stmt, 14 as ::core::ffi::c_int) != SQLITE_NULL {
                sqlite3_column_int(stmt, 14 as ::core::ffi::c_int)
            } else {
                RETENTION_TIER_STANDARD
            };
        (*metadata.offset(count as isize)).disk_pressure_eligible =
            if sqlite3_column_type(stmt, 15 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 15 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                r#true
            } != 0;
        (*metadata.offset(count as isize)).schedule_restricted =
            if sqlite3_column_type(stmt, 16 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 16 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        let mut camera_uuid: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 17 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*metadata.offset(count as isize)).camera_uuid as *mut ::core::ffi::c_char,
            if !camera_uuid.is_null() {
                camera_uuid
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        let mut playback_issue: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 18 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*metadata.offset(count as isize)).playback_issue as *mut ::core::ffi::c_char,
            if !playback_issue.is_null() {
                playback_issue
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            0 as size_t,
        );
        count += 1;
    }
    if rc_step != SQLITE_DONE && rc_step != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Error while fetching recordings: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        if camera_scope {
            count = -1 as ::core::ffi::c_int;
        }
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Found %d recordings in database matching criteria (page %d, limit %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        count,
        offset / limit + 1 as ::core::ffi::c_int,
        limit,
    );
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_recording_metadata(mut id: uint64_t) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = storage_recording_delete(
        id,
        b"recording deletion\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<uint64_t>(),
    );
    if rc == 0 as ::core::ffi::c_int {
        bump_recordings_generation();
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_stale_incomplete_recording_metadata(
    mut id: uint64_t,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = storage_recording_delete_stale_incomplete(id);
    if rc == 0 as ::core::ffi::c_int {
        bump_recordings_generation();
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_old_recording_metadata(
    mut max_age: uint64_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut ids: [uint64_t; 256] = [0; 256];
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cutoff: int64_t =
        time(::core::ptr::null_mut::<time_t>()) as int64_t - max_age as int64_t;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"SELECT r.id FROM recordings r LEFT JOIN storage_recording_policies p ON p.recording_id=r.id WHERE r.end_time<?1 AND r.is_complete=1 AND COALESCE(r.retention_override_days,-1)<>0 AND (COALESCE(p.minimum_retention_days,0)=0 OR r.start_time<strftime('%s','now')-p.minimum_retention_days*86400) AND ((r.retention_override_days>0 AND r.start_time<strftime('%s','now')-r.retention_override_days*86400) OR (COALESCE(r.retention_override_days,-1)<0 AND (p.recording_id IS NULL OR (p.maximum_retention_days>0 AND r.start_time<strftime('%s','now')-p.maximum_retention_days*86400)))) AND r.protected=0 AND r.deletion_pending=0 ORDER BY r.end_time LIMIT 256;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, cutoff as sqlite3_int64);
        while count < 256 as ::core::ffi::c_int && sqlite3_step(statement) == SQLITE_ROW
        {
            let c2rust_fresh22 = count;
            count += 1;
            ids[c2rust_fresh22 as usize] = sqlite3_column_int64(
                statement,
                0 as ::core::ffi::c_int,
            ) as uint64_t;
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if storage_recording_expire_age(ids[i as usize], cutoff) == 0 as ::core::ffi::c_int {
            deleted += 1;
        }
        i += 1;
    }
    if deleted > 0 as ::core::ffi::c_int {
        bump_recordings_generation();
    }
    return deleted;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_recording_protected(
    mut id: uint64_t,
    mut protected: bool,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char =
        b"UPDATE recordings SET protected = ? WHERE id = ? AND deletion_pending=0;\0".as_ptr()
            as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(
        stmt,
        1 as ::core::ffi::c_int,
        if protected as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    let mut changed: ::core::ffi::c_int = sqlite3_changes(db);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    if rc != SQLITE_DONE || changed != 1 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update recording protection: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    bump_recordings_generation();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Recording %llu protection set to %s\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
        if protected as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_recording_retention_override(
    mut id: uint64_t,
    mut days: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char =
        b"UPDATE recordings SET retention_override_days = ? WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if days < 0 as ::core::ffi::c_int {
        sqlite3_bind_null(stmt, 1 as ::core::ffi::c_int);
    } else {
        sqlite3_bind_int(stmt, 1 as ::core::ffi::c_int, days);
    }
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update recording retention override: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Recording %llu retention override set to %d days\0".as_ptr()
            as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
        days,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_protected_recordings_count(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !stream_name.is_null() {
        sql = b"SELECT COUNT(*) FROM recordings WHERE protected = 1 AND stream_name = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    } else {
        sql = b"SELECT COUNT(*) FROM recordings WHERE protected = 1;\0".as_ptr()
            as *const ::core::ffi::c_char;
    }
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if !stream_name.is_null() {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
    if sqlite3_step(stmt) == SQLITE_ROW {
        count = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_protected_recordings_count_for_streams(
    mut stream_names: *const *const ::core::ffi::c_char,
    mut stream_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return protected_count_for_scope(stream_names, stream_count, r#false != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_protected_recordings_count_for_cameras(
    mut camera_uuids: *const *const ::core::ffi::c_char,
    mut camera_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return protected_count_for_scope(camera_uuids, camera_count, r#true != 0);
}
unsafe extern "C" fn protected_count_for_scope(
    mut stream_names: *const *const ::core::ffi::c_char,
    mut stream_count: ::core::ffi::c_int,
    mut camera_scope: bool,
) -> ::core::ffi::c_int {
    if stream_names.is_null() || stream_count <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || db_mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut variable_limit: ::core::ffi::c_int =
        sqlite3_limit(db, SQLITE_LIMIT_VARIABLE_NUMBER, -1 as ::core::ffi::c_int);
    let mut batch_limit: ::core::ffi::c_int = if variable_limit > 0 as ::core::ffi::c_int {
        variable_limit
    } else {
        1 as ::core::ffi::c_int
    };
    if batch_limit > 256 as ::core::ffi::c_int {
        batch_limit = 256 as ::core::ffi::c_int;
    }
    let mut total: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut failed: bool = r#false != 0;
    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while offset < stream_count {
        let mut batch_count: ::core::ffi::c_int = stream_count - offset;
        if batch_count > batch_limit {
            batch_count = batch_limit;
        }
        let mut sql_size: size_t =
            (512 as size_t).wrapping_add((batch_count as size_t).wrapping_mul(3 as size_t));
        let mut sql: *mut ::core::ffi::c_char =
            calloc(sql_size, 1 as size_t) as *mut ::core::ffi::c_char;
        if sql.is_null() {
            failed = r#true != 0;
            break;
        } else {
            safe_strcpy(
                sql,
                if camera_scope as ::core::ffi::c_int != 0 {
                    b"SELECT COUNT(*) FROM recordings r WHERE protected=1 AND COALESCE(NULLIF(r.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=r.stream_name)) IN (\0"
                        .as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"SELECT COUNT(*) FROM recordings WHERE protected=1 AND stream_name IN (\0"
                        .as_ptr() as *const ::core::ffi::c_char
                },
                sql_size,
                0 as size_t,
            );
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < batch_count {
                safe_strcat(
                    sql,
                    if i == 0 as ::core::ffi::c_int {
                        b"?\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b",?\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    sql_size,
                );
                i += 1;
            }
            safe_strcat(
                sql,
                b");\0".as_ptr() as *const ::core::ffi::c_char,
                sql_size,
            );
            let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
                db,
                sql,
                -1 as ::core::ffi::c_int,
                &raw mut stmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            free(sql as *mut ::core::ffi::c_void);
            if rc == SQLITE_OK {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < batch_count {
                    sqlite3_bind_text(
                        stmt,
                        i_0 + 1 as ::core::ffi::c_int,
                        *stream_names.offset((offset + i_0) as isize),
                        -1 as ::core::ffi::c_int,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                    i_0 += 1;
                }
                if sqlite3_step(stmt) == SQLITE_ROW {
                    total += sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
                } else {
                    failed = r#true != 0;
                }
            } else {
                failed = r#true != 0;
            }
            if !stmt.is_null() {
                sqlite3_finalize(stmt);
            }
            if failed {
                break;
            }
            offset += batch_limit;
        }
    }
    pthread_mutex_unlock(db_mutex);
    return if failed as ::core::ffi::c_int != 0 {
        -1 as ::core::ffi::c_int
    } else {
        total
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recordings_for_retention(
    mut stream_name: *const ::core::ffi::c_char,
    mut retention_days: ::core::ffi::c_int,
    mut detection_retention_days: ::core::ffi::c_int,
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if stream_name.is_null() || recordings.is_null() || max_count <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_recordings_for_retention\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut regular_cutoff: time_t = if retention_days > 0 as ::core::ffi::c_int {
        now - retention_days as time_t * 86400 as time_t
    } else {
        0 as time_t
    };
    let mut detection_cutoff: time_t = if detection_retention_days > 0 as ::core::ffi::c_int {
        now - detection_retention_days as time_t * 86400 as time_t
    } else {
        0 as time_t
    };
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open a read-only connection for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"get_recordings_for_retention\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT id, stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days FROM recordings WHERE stream_name = ? AND protected = 0 AND deletion_pending=0 AND is_complete = 1 AND NOT EXISTS(SELECT 1 FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.minimum_retention_days>0 AND start_time>=strftime('%s','now')-p.minimum_retention_days*86400) AND (  (trigger_type != 'detection' AND ? > 0 AND start_time < ?)   OR   (trigger_type = 'detection' AND ? > 0 AND start_time < ?)) AND (  retention_override_days IS NULL OR retention_override_days<0   OR (retention_override_days>0 AND start_time < (strftime('%s', 'now') - retention_override_days * 86400))) ORDER BY   CASE WHEN trigger_type = 'detection' THEN 1 ELSE 0 END ASC,   start_time ASC LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(stmt, 2 as ::core::ffi::c_int, retention_days);
    sqlite3_bind_int64(
        stmt,
        3 as ::core::ffi::c_int,
        regular_cutoff as sqlite3_int64,
    );
    sqlite3_bind_int(stmt, 4 as ::core::ffi::c_int, detection_retention_days);
    sqlite3_bind_int64(
        stmt,
        5 as ::core::ffi::c_int,
        detection_cutoff as sqlite3_int64,
    );
    sqlite3_bind_int(stmt, 6 as ::core::ffi::c_int, max_count);
    while sqlite3_step(stmt) == SQLITE_ROW && count < max_count {
        (*recordings.offset(count as isize)).id =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        let mut stream: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !stream.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).stream_name
                    as *mut ::core::ffi::c_char,
                stream,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(count as isize)).stream_name[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut path: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !path.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).file_path as *mut ::core::ffi::c_char,
                path,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(count as isize)).file_path[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*recordings.offset(count as isize)).start_time =
            sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as time_t;
        if sqlite3_column_type(stmt, 4 as ::core::ffi::c_int) != SQLITE_NULL {
            (*recordings.offset(count as isize)).end_time =
                sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as time_t;
        } else {
            (*recordings.offset(count as isize)).end_time = 0 as time_t;
        }
        (*recordings.offset(count as isize)).size_bytes =
            sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as uint64_t;
        (*recordings.offset(count as isize)).width =
            sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
        (*recordings.offset(count as isize)).height =
            sqlite3_column_int(stmt, 7 as ::core::ffi::c_int);
        (*recordings.offset(count as isize)).fps =
            sqlite3_column_int(stmt, 8 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 9 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).codec as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(count as isize)).codec[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*recordings.offset(count as isize)).is_complete =
            sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut trigger_type: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 11 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !trigger_type.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                trigger_type,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*recordings.offset(count as isize)).protected =
            sqlite3_column_int(stmt, 12 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        if sqlite3_column_type(stmt, 13 as ::core::ffi::c_int) != SQLITE_NULL {
            (*recordings.offset(count as isize)).retention_override_days =
                sqlite3_column_int(stmt, 13 as ::core::ffi::c_int);
        } else {
            (*recordings.offset(count as isize)).retention_override_days = -1 as ::core::ffi::c_int;
        }
        count += 1;
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recordings_for_quota_enforcement(
    mut stream_name: *const ::core::ffi::c_char,
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if stream_name.is_null() || recordings.is_null() || max_count <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_recordings_for_quota_enforcement\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open a read-only connection for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"get_recordings_for_quota_enforcement\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT id, stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days FROM recordings WHERE stream_name = ? AND protected = 0 AND deletion_pending=0 AND is_complete = 1 AND NOT EXISTS(SELECT 1 FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.minimum_retention_days>0 AND start_time>=strftime('%s','now')-p.minimum_retention_days*86400) ORDER BY retention_tier DESC, CASE WHEN retention_override_days IS NULL OR retention_override_days < 0 THEN 0 ELSE 1 END ASC, CASE WHEN trigger_type = 'detection' THEN 1 ELSE 0 END ASC, start_time ASC LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(stmt, 2 as ::core::ffi::c_int, max_count);
    while sqlite3_step(stmt) == SQLITE_ROW && count < max_count {
        (*recordings.offset(count as isize)).id =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        let mut stream: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !stream.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).stream_name
                    as *mut ::core::ffi::c_char,
                stream,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(count as isize)).stream_name[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut path: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !path.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).file_path as *mut ::core::ffi::c_char,
                path,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(count as isize)).file_path[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*recordings.offset(count as isize)).start_time =
            sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as time_t;
        if sqlite3_column_type(stmt, 4 as ::core::ffi::c_int) != SQLITE_NULL {
            (*recordings.offset(count as isize)).end_time =
                sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as time_t;
        } else {
            (*recordings.offset(count as isize)).end_time = 0 as time_t;
        }
        (*recordings.offset(count as isize)).size_bytes =
            sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as uint64_t;
        (*recordings.offset(count as isize)).width =
            sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
        (*recordings.offset(count as isize)).height =
            sqlite3_column_int(stmt, 7 as ::core::ffi::c_int);
        (*recordings.offset(count as isize)).fps =
            sqlite3_column_int(stmt, 8 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 9 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).codec as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(count as isize)).codec[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*recordings.offset(count as isize)).is_complete =
            sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut trigger_type: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 11 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !trigger_type.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                trigger_type,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*recordings.offset(count as isize)).protected =
            sqlite3_column_int(stmt, 12 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        if sqlite3_column_type(stmt, 13 as ::core::ffi::c_int) != SQLITE_NULL {
            (*recordings.offset(count as isize)).retention_override_days =
                sqlite3_column_int(stmt, 13 as ::core::ffi::c_int);
        } else {
            (*recordings.offset(count as isize)).retention_override_days = -1 as ::core::ffi::c_int;
        }
        count += 1;
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_orphaned_db_entries(
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
    mut total_checked: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if recordings.is_null() || max_count <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_orphaned_db_entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut total_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pthread_mutex_lock(db_mutex);
    let mut count_sql: *const ::core::ffi::c_char = b"SELECT COUNT(*) FROM recordings WHERE is_complete = 1 AND protected = 0 AND deletion_pending=0 AND NOT EXISTS(SELECT 1 FROM storage_targets t WHERE t.uuid=recordings.storage_target_uuid AND t.target_type='s3') AND NOT EXISTS(SELECT 1 FROM storage_recording_copies c WHERE c.recording_id=recordings.id) AND NOT EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=recordings.id AND j.state<>'completed') ;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        count_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare orphan count query: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if sqlite3_step(stmt) == SQLITE_ROW {
        total_count = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    }
    sqlite3_finalize(stmt);
    if !total_checked.is_null() {
        *total_checked = total_count;
    }
    if total_count == 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(db_mutex);
        return 0 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT id, stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type FROM recordings WHERE is_complete = 1 AND protected = 0 AND deletion_pending=0 AND NOT EXISTS(SELECT 1 FROM storage_targets t WHERE t.uuid=recordings.storage_target_uuid AND t.target_type='s3') AND NOT EXISTS(SELECT 1 FROM storage_recording_copies c WHERE c.recording_id=recordings.id) AND NOT EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=recordings.id AND j.state<>'completed') ORDER BY start_time ASC LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare orphan candidate query: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(stmt, 1 as ::core::ffi::c_int, max_count);
    let mut candidates: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while sqlite3_step(stmt) == SQLITE_ROW && candidates < max_count {
        (*recordings.offset(candidates as isize)).id =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        let mut stream: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !stream.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(candidates as isize)).stream_name
                    as *mut ::core::ffi::c_char,
                stream,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(candidates as isize)).stream_name[0usize] =
                '\0' as ::core::ffi::c_char;
        }
        let mut path: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !path.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(candidates as isize)).file_path
                    as *mut ::core::ffi::c_char,
                path,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(candidates as isize)).file_path[0usize] =
                '\0' as ::core::ffi::c_char;
        }
        (*recordings.offset(candidates as isize)).start_time =
            sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as time_t;
        if sqlite3_column_type(stmt, 4 as ::core::ffi::c_int) != SQLITE_NULL {
            (*recordings.offset(candidates as isize)).end_time =
                sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as time_t;
        } else {
            (*recordings.offset(candidates as isize)).end_time = 0 as time_t;
        }
        (*recordings.offset(candidates as isize)).size_bytes =
            sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as uint64_t;
        (*recordings.offset(candidates as isize)).width =
            sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
        (*recordings.offset(candidates as isize)).height =
            sqlite3_column_int(stmt, 7 as ::core::ffi::c_int);
        (*recordings.offset(candidates as isize)).fps =
            sqlite3_column_int(stmt, 8 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 9 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(candidates as isize)).codec
                    as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(candidates as isize)).codec[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*recordings.offset(candidates as isize)).is_complete =
            sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut trigger_type: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 11 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !trigger_type.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(candidates as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                trigger_type,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*recordings.offset(candidates as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        candidates += 1;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < candidates {
        if (*recordings.offset(i as isize)).file_path[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
            && access(
                &raw mut (*recordings.offset(i as isize)).file_path as *mut ::core::ffi::c_char,
                F_OK,
            ) != 0 as ::core::ffi::c_int
        {
            if count != i {
                *recordings.offset(count as isize) = *recordings.offset(i as isize);
            }
            count += 1;
        }
        i += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Orphan check: %d candidates checked, %d orphaned (total recordings: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        candidates,
        count,
        total_count,
    );
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recordings_for_tiered_retention(
    mut stream_name: *const ::core::ffi::c_char,
    mut base_retention_days: ::core::ffi::c_int,
    mut tier_multipliers: *const ::core::ffi::c_double,
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if tier_multipliers.is_null() || recordings.is_null() || max_count <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_recordings_for_tiered_retention\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut cutoff_critical: time_t = now
        - (base_retention_days as ::core::ffi::c_double
            * *tier_multipliers.offset(RETENTION_TIER_CRITICAL as isize)
            * 86400 as ::core::ffi::c_int as ::core::ffi::c_double) as time_t;
    let mut cutoff_important: time_t = now
        - (base_retention_days as ::core::ffi::c_double
            * *tier_multipliers.offset(RETENTION_TIER_IMPORTANT as isize)
            * 86400 as ::core::ffi::c_int as ::core::ffi::c_double) as time_t;
    let mut cutoff_standard: time_t = now
        - (base_retention_days as ::core::ffi::c_double
            * *tier_multipliers.offset(RETENTION_TIER_STANDARD as isize)
            * 86400 as ::core::ffi::c_int as ::core::ffi::c_double) as time_t;
    let mut cutoff_ephemeral: time_t = now
        - (base_retention_days as ::core::ffi::c_double
            * *tier_multipliers.offset(RETENTION_TIER_EPHEMERAL as isize)
            * 86400 as ::core::ffi::c_int as ::core::ffi::c_double) as time_t;
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open a read-only connection for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"get_recordings_for_tiered_retention\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !stream_name.is_null() {
        sql = b"SELECT id, stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days, retention_tier, disk_pressure_eligible, schedule_restricted FROM recordings WHERE stream_name = ? AND protected = 0 AND deletion_pending=0 AND is_complete = 1 AND NOT EXISTS (SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=recordings.id AND j.state NOT IN ('completed','failed','cancelled')) AND ((  (retention_tier = 0 AND start_time < ?) OR   (retention_tier = 1 AND start_time < ?) OR   (retention_tier = 2 AND start_time < ?) OR   (retention_tier = 3 AND start_time < ?)) OR EXISTS (SELECT 1 FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.enabled=1 AND p.uuid=substr(COALESCE(placement_reason,''),instr(COALESCE(placement_reason,''),':')+1) AND p.maximum_retention_days>0 AND start_time < strftime('%s','now')-p.maximum_retention_days*86400)) AND (retention_override_days IS NULL OR retention_override_days<0   OR (retention_override_days>0 AND start_time < (strftime('%s', 'now') - retention_override_days * 86400))) AND NOT EXISTS (SELECT 1 FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.enabled=1 AND p.uuid=substr(COALESCE(placement_reason,''),instr(COALESCE(placement_reason,''),':')+1) AND p.minimum_retention_days>0 AND start_time >= strftime('%s','now')-p.minimum_retention_days*86400) ORDER BY retention_tier DESC, start_time ASC LIMIT ?;\0"
            .as_ptr() as *const ::core::ffi::c_char;
    } else {
        sql = b"SELECT id, stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days, retention_tier, disk_pressure_eligible, schedule_restricted FROM recordings WHERE protected = 0 AND deletion_pending=0 AND is_complete = 1 AND NOT EXISTS (SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=recordings.id AND j.state NOT IN ('completed','failed','cancelled')) AND ((  (retention_tier = 0 AND start_time < ?) OR   (retention_tier = 1 AND start_time < ?) OR   (retention_tier = 2 AND start_time < ?) OR   (retention_tier = 3 AND start_time < ?)) OR EXISTS (SELECT 1 FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.enabled=1 AND p.uuid=substr(COALESCE(placement_reason,''),instr(COALESCE(placement_reason,''),':')+1) AND p.maximum_retention_days>0 AND start_time < strftime('%s','now')-p.maximum_retention_days*86400)) AND (retention_override_days IS NULL OR retention_override_days<0   OR (retention_override_days>0 AND start_time < (strftime('%s', 'now') - retention_override_days * 86400))) AND NOT EXISTS (SELECT 1 FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.enabled=1 AND p.uuid=substr(COALESCE(placement_reason,''),instr(COALESCE(placement_reason,''),':')+1) AND p.minimum_retention_days>0 AND start_time >= strftime('%s','now')-p.minimum_retention_days*86400) ORDER BY retention_tier DESC, start_time ASC LIMIT ?;\0"
            .as_ptr() as *const ::core::ffi::c_char;
    }
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare tiered retention query: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    let mut param: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if !stream_name.is_null() {
        let c2rust_fresh23 = param;
        param += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh23,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
    let c2rust_fresh24 = param;
    param += 1;
    sqlite3_bind_int64(stmt, c2rust_fresh24, cutoff_critical as sqlite3_int64);
    let c2rust_fresh25 = param;
    param += 1;
    sqlite3_bind_int64(stmt, c2rust_fresh25, cutoff_important as sqlite3_int64);
    let c2rust_fresh26 = param;
    param += 1;
    sqlite3_bind_int64(stmt, c2rust_fresh26, cutoff_standard as sqlite3_int64);
    let c2rust_fresh27 = param;
    param += 1;
    sqlite3_bind_int64(stmt, c2rust_fresh27, cutoff_ephemeral as sqlite3_int64);
    let c2rust_fresh28 = param;
    param += 1;
    sqlite3_bind_int(stmt, c2rust_fresh28, max_count);
    while sqlite3_step(stmt) == SQLITE_ROW && count < max_count {
        (*recordings.offset(count as isize)).id =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        let mut sname: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !sname.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).stream_name
                    as *mut ::core::ffi::c_char,
                sname,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        let mut fpath: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !fpath.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).file_path as *mut ::core::ffi::c_char,
                fpath,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
        (*recordings.offset(count as isize)).start_time =
            sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as time_t;
        (*recordings.offset(count as isize)).end_time =
            if sqlite3_column_type(stmt, 4 as ::core::ffi::c_int) != SQLITE_NULL {
                sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as time_t
            } else {
                0 as time_t
            };
        (*recordings.offset(count as isize)).size_bytes =
            sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as uint64_t;
        (*recordings.offset(count as isize)).width =
            sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
        (*recordings.offset(count as isize)).height =
            sqlite3_column_int(stmt, 7 as ::core::ffi::c_int);
        (*recordings.offset(count as isize)).fps =
            sqlite3_column_int(stmt, 8 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 9 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).codec as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*recordings.offset(count as isize)).is_complete =
            sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut ttype: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 11 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !ttype.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                ttype,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*recordings.offset(count as isize)).protected =
            sqlite3_column_int(stmt, 12 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        (*recordings.offset(count as isize)).retention_override_days =
            if sqlite3_column_type(stmt, 13 as ::core::ffi::c_int) != SQLITE_NULL {
                sqlite3_column_int(stmt, 13 as ::core::ffi::c_int)
            } else {
                -1 as ::core::ffi::c_int
            };
        (*recordings.offset(count as isize)).retention_tier =
            if sqlite3_column_type(stmt, 14 as ::core::ffi::c_int) != SQLITE_NULL {
                sqlite3_column_int(stmt, 14 as ::core::ffi::c_int)
            } else {
                RETENTION_TIER_STANDARD
            };
        (*recordings.offset(count as isize)).disk_pressure_eligible =
            if sqlite3_column_type(stmt, 15 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 15 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                r#true
            } != 0;
        (*recordings.offset(count as isize)).schedule_restricted =
            if sqlite3_column_type(stmt, 16 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 16 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        count += 1;
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Found %d recordings eligible for tiered retention cleanup\0".as_ptr()
            as *const ::core::ffi::c_char,
        count,
    );
    return count;
}
unsafe extern "C" fn get_pressure_cleanup_recordings(
    mut storage_target_uuid: *const ::core::ffi::c_char,
    mut include_unattributed: bool,
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if recordings.is_null()
        || max_count <= 0 as ::core::ffi::c_int
        || !storage_target_uuid.is_null()
            && *storage_target_uuid.offset(0isize) as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_recordings_for_pressure_cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open a read-only connection for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"get_pressure_cleanup_recordings\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut global_sql: *const ::core::ffi::c_char = b"SELECT id, stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days, retention_tier, disk_pressure_eligible, schedule_restricted, COALESCE(storage_target_uuid,''), COALESCE(object_key,''), COALESCE(placement_reason,''), COALESCE(storage_policy_version,0) FROM recordings WHERE protected = 0 AND deletion_pending=0 AND disk_pressure_eligible = 1 AND is_complete = 1 AND deletion_pending=0 AND NOT EXISTS (SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=recordings.id AND j.state NOT IN ('completed','failed','cancelled')) AND NOT EXISTS (SELECT 1 FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.enabled=1 AND p.uuid=substr(COALESCE(placement_reason,''),instr(COALESCE(placement_reason,''),':')+1) AND ((p.migration_target_uuid IS NOT NULL AND (p.archive_after_seconds>=0 OR p.archive_protected=1 OR p.archive_on_pressure=1 OR p.migration_after_days>0)) OR p.required_copy_count>1 OR (p.minimum_retention_days>0 AND start_time>=strftime('%s','now')-p.minimum_retention_days*86400))) ORDER BY COALESCE((SELECT p.pressure_priority FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.uuid=substr(COALESCE(placement_reason,''),instr(COALESCE(placement_reason,''),':')+1)),100) ASC, retention_tier DESC, CASE WHEN retention_override_days IS NULL OR retention_override_days < 0 THEN 0 ELSE 1 END ASC, CASE WHEN trigger_type = 'detection' THEN 1 ELSE 0 END ASC, start_time ASC LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut target_sql: *const ::core::ffi::c_char = b"SELECT id, stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days, retention_tier, disk_pressure_eligible, schedule_restricted, COALESCE(storage_target_uuid,''), COALESCE(object_key,''), COALESCE(placement_reason,''), COALESCE(storage_policy_version,0) FROM recordings WHERE protected = 0 AND deletion_pending=0 AND disk_pressure_eligible = 1 AND is_complete = 1 AND storage_target_uuid = ? AND deletion_pending=0 AND NOT EXISTS (SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=recordings.id AND j.state NOT IN ('completed','failed','cancelled')) AND NOT EXISTS (SELECT 1 FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.enabled=1 AND p.uuid=substr(COALESCE(placement_reason,''),instr(COALESCE(placement_reason,''),':')+1) AND ((p.migration_target_uuid IS NOT NULL AND (p.archive_after_seconds>=0 OR p.archive_protected=1 OR p.archive_on_pressure=1 OR p.migration_after_days>0)) OR p.required_copy_count>1 OR (p.minimum_retention_days>0 AND start_time>=strftime('%s','now')-p.minimum_retention_days*86400))) ORDER BY COALESCE((SELECT p.pressure_priority FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.uuid=substr(COALESCE(placement_reason,''),instr(COALESCE(placement_reason,''),':')+1)),100) ASC, retention_tier DESC, CASE WHEN retention_override_days IS NULL OR retention_override_days < 0 THEN 0 ELSE 1 END ASC, CASE WHEN trigger_type = 'detection' THEN 1 ELSE 0 END ASC, start_time ASC LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut default_target_sql: *const ::core::ffi::c_char = b"SELECT id, stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, protected, retention_override_days, retention_tier, disk_pressure_eligible, schedule_restricted, COALESCE(storage_target_uuid,''), COALESCE(object_key,''), COALESCE(placement_reason,''), COALESCE(storage_policy_version,0) FROM recordings WHERE protected = 0 AND deletion_pending=0 AND disk_pressure_eligible = 1 AND is_complete = 1 AND (storage_target_uuid = ? OR storage_target_uuid IS NULL) AND deletion_pending=0 AND NOT EXISTS (SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=recordings.id AND j.state NOT IN ('completed','failed','cancelled')) AND NOT EXISTS (SELECT 1 FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.enabled=1 AND p.uuid=substr(COALESCE(placement_reason,''),instr(COALESCE(placement_reason,''),':')+1) AND ((p.migration_target_uuid IS NOT NULL AND (p.archive_after_seconds>=0 OR p.archive_protected=1 OR p.archive_on_pressure=1 OR p.migration_after_days>0)) OR p.required_copy_count>1 OR (p.minimum_retention_days>0 AND start_time>=strftime('%s','now')-p.minimum_retention_days*86400))) ORDER BY COALESCE((SELECT p.pressure_priority FROM storage_recording_policies p WHERE p.recording_id=recordings.id AND p.uuid=substr(COALESCE(placement_reason,''),instr(COALESCE(placement_reason,''),':')+1)),100) ASC, retention_tier DESC, CASE WHEN retention_override_days IS NULL OR retention_override_days < 0 THEN 0 ELSE 1 END ASC, CASE WHEN trigger_type = 'detection' THEN 1 ELSE 0 END ASC, start_time ASC LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut selected_sql: *const ::core::ffi::c_char = global_sql;
    if !storage_target_uuid.is_null() {
        selected_sql = if include_unattributed as ::core::ffi::c_int != 0 {
            default_target_sql
        } else {
            target_sql
        };
    }
    rc = sqlite3_prepare_v2(
        db,
        selected_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare pressure cleanup query: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    let mut limit_parameter: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if !storage_target_uuid.is_null() {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            storage_target_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        limit_parameter = 2 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(stmt, limit_parameter, max_count);
    while sqlite3_step(stmt) == SQLITE_ROW && count < max_count {
        (*recordings.offset(count as isize)).id =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        let mut sname: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !sname.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).stream_name
                    as *mut ::core::ffi::c_char,
                sname,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        let mut fpath: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !fpath.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).file_path as *mut ::core::ffi::c_char,
                fpath,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
        (*recordings.offset(count as isize)).start_time =
            sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as time_t;
        (*recordings.offset(count as isize)).end_time =
            if sqlite3_column_type(stmt, 4 as ::core::ffi::c_int) != SQLITE_NULL {
                sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as time_t
            } else {
                0 as time_t
            };
        (*recordings.offset(count as isize)).size_bytes =
            sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as uint64_t;
        (*recordings.offset(count as isize)).width =
            sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
        (*recordings.offset(count as isize)).height =
            sqlite3_column_int(stmt, 7 as ::core::ffi::c_int);
        (*recordings.offset(count as isize)).fps =
            sqlite3_column_int(stmt, 8 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 9 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).codec as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*recordings.offset(count as isize)).is_complete =
            sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut ttype: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 11 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !ttype.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                ttype,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*recordings.offset(count as isize)).protected =
            sqlite3_column_int(stmt, 12 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        (*recordings.offset(count as isize)).retention_override_days =
            if sqlite3_column_type(stmt, 13 as ::core::ffi::c_int) != SQLITE_NULL {
                sqlite3_column_int(stmt, 13 as ::core::ffi::c_int)
            } else {
                -1 as ::core::ffi::c_int
            };
        (*recordings.offset(count as isize)).retention_tier =
            if sqlite3_column_type(stmt, 14 as ::core::ffi::c_int) != SQLITE_NULL {
                sqlite3_column_int(stmt, 14 as ::core::ffi::c_int)
            } else {
                RETENTION_TIER_STANDARD
            };
        (*recordings.offset(count as isize)).disk_pressure_eligible =
            if sqlite3_column_type(stmt, 15 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 15 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                r#true
            } != 0;
        (*recordings.offset(count as isize)).schedule_restricted =
            if sqlite3_column_type(stmt, 16 as ::core::ffi::c_int) != SQLITE_NULL {
                (sqlite3_column_int(stmt, 16 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        let mut target_uuid: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 17 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut object_key: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 18 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut placement_reason: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 19 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*recordings.offset(count as isize)).storage_target_uuid
                as *mut ::core::ffi::c_char,
            if !target_uuid.is_null() {
                target_uuid
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*recordings.offset(count as isize)).object_key as *mut ::core::ffi::c_char,
            if !object_key.is_null() {
                object_key
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*recordings.offset(count as isize)).placement_reason
                as *mut ::core::ffi::c_char,
            if !placement_reason.is_null() {
                placement_reason
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        (*recordings.offset(count as isize)).storage_policy_version =
            sqlite3_column_int64(stmt, 20 as ::core::ffi::c_int) as int64_t;
        count += 1;
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Found %d recordings eligible for disk pressure cleanup\0".as_ptr()
            as *const ::core::ffi::c_char,
        count,
    );
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recordings_for_pressure_cleanup(
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return get_pressure_cleanup_recordings(
        ::core::ptr::null::<::core::ffi::c_char>(),
        r#false != 0,
        recordings,
        max_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recordings_for_pressure_cleanup_target(
    mut storage_target_uuid: *const ::core::ffi::c_char,
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return get_pressure_cleanup_recordings(
        storage_target_uuid,
        r#false != 0,
        recordings,
        max_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recordings_for_pressure_cleanup_default_target(
    mut storage_target_uuid: *const ::core::ffi::c_char,
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return get_pressure_cleanup_recordings(
        storage_target_uuid,
        r#true != 0,
        recordings,
        max_count,
    );
}
unsafe extern "C" fn capture_recording_boundary_locked(
    mut db: *mut sqlite3,
    mut maximum_id: *mut uint64_t,
) -> ::core::ffi::c_int {
    *maximum_id = 0 as uint64_t;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT COALESCE(MAX(id), 0) FROM recordings;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        rc = sqlite3_step(statement);
    }
    if rc == SQLITE_ROW {
        let mut value: sqlite3_int64 = sqlite3_column_int64(statement, 0 as ::core::ffi::c_int);
        if value >= 0 as sqlite3_int64 {
            *maximum_id = value as uint64_t;
        } else {
            rc = SQLITE_ERROR;
        }
    }
    sqlite3_finalize(statement);
    return if rc == SQLITE_ROW {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recordings_capture_startup_boundary(
    mut maximum_id: *mut uint64_t,
) -> ::core::ffi::c_int {
    if maximum_id.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *maximum_id = 0 as uint64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut result: ::core::ffi::c_int = capture_recording_boundary_locked(db, maximum_id);
    pthread_mutex_unlock(mutex);
    return result;
}
unsafe extern "C" fn incomplete_recordings_page(
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
    mut cutoff: time_t,
    mut use_age: bool,
    mut after_id: uint64_t,
    mut maximum_id: *mut uint64_t,
    mut capture_boundary: bool,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || db_mutex.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if recordings.is_null()
        || max_count <= 0 as ::core::ffi::c_int
        || maximum_id.is_null()
        || after_id > INT64_MAX as uint64_t
        || !capture_boundary && *maximum_id > INT64_MAX as uint64_t
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_stale_incomplete_recordings_page\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    if capture_boundary as ::core::ffi::c_int != 0 && after_id == 0 as uint64_t {
        if capture_recording_boundary_locked(db, maximum_id) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to bound stale-incomplete scan: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            pthread_mutex_unlock(db_mutex);
            return -1 as ::core::ffi::c_int;
        }
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT id, stream_name, file_path, start_time, end_time, size_bytes, width, height, fps, codec, is_complete, trigger_type, storage_target_uuid, object_key FROM recordings WHERE (is_complete = 0 OR playback_issue != '') AND deletion_pending = 0 AND (?1 = 0 OR start_time < ?2) AND id > ?3 AND id <= ?4 ORDER BY id ASC LIMIT ?5;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare stale-incomplete query: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(
        stmt,
        1 as ::core::ffi::c_int,
        if use_age as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, cutoff as sqlite3_int64);
    sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, after_id as sqlite3_int64);
    sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, *maximum_id as sqlite3_int64);
    sqlite3_bind_int(stmt, 5 as ::core::ffi::c_int, max_count);
    while count < max_count && {
        rc = sqlite3_step(stmt);
        rc == SQLITE_ROW
    } {
        (*recordings.offset(count as isize)).id =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        let mut sname: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !sname.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).stream_name
                    as *mut ::core::ffi::c_char,
                sname,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(count as isize)).stream_name[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut fpath: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !fpath.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).file_path as *mut ::core::ffi::c_char,
                fpath,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(count as isize)).file_path[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*recordings.offset(count as isize)).start_time =
            sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as time_t;
        (*recordings.offset(count as isize)).end_time =
            if sqlite3_column_type(stmt, 4 as ::core::ffi::c_int) != SQLITE_NULL {
                sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as time_t
            } else {
                0 as time_t
            };
        (*recordings.offset(count as isize)).size_bytes =
            sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as uint64_t;
        (*recordings.offset(count as isize)).width =
            sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
        (*recordings.offset(count as isize)).height =
            sqlite3_column_int(stmt, 7 as ::core::ffi::c_int);
        (*recordings.offset(count as isize)).fps =
            sqlite3_column_int(stmt, 8 as ::core::ffi::c_int);
        let mut codec: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 9 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !codec.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).codec as *mut ::core::ffi::c_char,
                codec,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(count as isize)).codec[0usize] = '\0' as ::core::ffi::c_char;
        }
        (*recordings.offset(count as isize)).is_complete =
            sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        let mut ttype: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 11 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !ttype.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                ttype,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
        (*recordings.offset(count as isize)).protected = r#false != 0;
        (*recordings.offset(count as isize)).retention_override_days = -1 as ::core::ffi::c_int;
        (*recordings.offset(count as isize)).retention_tier = RETENTION_TIER_STANDARD;
        (*recordings.offset(count as isize)).disk_pressure_eligible = r#true != 0;
        (*recordings.offset(count as isize)).schedule_restricted = -1 as ::core::ffi::c_int;
        let mut target: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 12 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !target.is_null() {
            safe_strcpy(
                &raw mut (*recordings.offset(count as isize)).storage_target_uuid
                    as *mut ::core::ffi::c_char,
                target,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        } else {
            (*recordings.offset(count as isize)).storage_target_uuid[0usize] =
                '\0' as ::core::ffi::c_char;
        }
        let mut object_key: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 13 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*recordings.offset(count as isize)).object_key as *mut ::core::ffi::c_char,
            if !object_key.is_null() {
                object_key
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        count += 1;
    }
    let mut failed: bool = count < max_count && rc != SQLITE_DONE;
    if failed {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to read stale-incomplete recordings: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return if failed as ::core::ffi::c_int != 0 {
        -1 as ::core::ffi::c_int
    } else {
        count
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stale_incomplete_recordings_page(
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
    mut cutoff: time_t,
    mut after_id: uint64_t,
    mut maximum_id: *mut uint64_t,
) -> ::core::ffi::c_int {
    return incomplete_recordings_page(
        recordings,
        max_count,
        cutoff,
        r#true != 0,
        after_id,
        maximum_id,
        r#true != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_startup_incomplete_recordings_page(
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
    mut after_id: uint64_t,
    mut maximum_id: uint64_t,
) -> ::core::ffi::c_int {
    return incomplete_recordings_page(
        recordings,
        max_count,
        0 as time_t,
        r#false != 0,
        after_id,
        &raw mut maximum_id,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_stream_storage_bytes(
    mut stream_name: *const ::core::ffi::c_char,
) -> int64_t {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut total_bytes: int64_t = -1 as int64_t;
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open a read-only connection for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"get_stream_storage_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as int64_t;
    }
    let mut sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !stream_name.is_null() {
        sql = b"SELECT COALESCE(SUM(size_bytes), 0) FROM recordings WHERE stream_name = ? AND is_complete = 1;\0"
            .as_ptr() as *const ::core::ffi::c_char;
    } else {
        sql = b"SELECT COALESCE(SUM(size_bytes), 0) FROM recordings WHERE is_complete = 1;\0"
            .as_ptr() as *const ::core::ffi::c_char;
    }
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare storage bytes query: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        db_close_readonly_connection(db);
        return -1 as int64_t;
    }
    if !stream_name.is_null() {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
    if sqlite3_step(stmt) == SQLITE_ROW {
        total_bytes = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return total_bytes;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recent_recording_storage_stats(
    mut since_time: time_t,
    mut total_bytes: *mut uint64_t,
    mut oldest_start_time: *mut time_t,
    mut newest_end_time: *mut time_t,
    mut recording_count: *mut uint64_t,
) -> ::core::ffi::c_int {
    if total_bytes.is_null()
        || oldest_start_time.is_null()
        || newest_end_time.is_null()
        || recording_count.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    *total_bytes = 0 as uint64_t;
    *oldest_start_time = 0 as time_t;
    *newest_end_time = 0 as time_t;
    *recording_count = 0 as uint64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"SELECT COALESCE(SUM(size_bytes), 0),        COALESCE(MIN(start_time), 0),        COALESCE(MAX(end_time), 0), COUNT(*) FROM recordings WHERE is_complete = 1 AND end_time IS NOT NULL   AND end_time >= ? AND size_bytes > 0;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare recent storage stats query: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, since_time as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc == SQLITE_ROW {
        *total_bytes = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        *oldest_start_time = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as time_t;
        *newest_end_time = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as time_t;
        *recording_count = sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as uint64_t;
        rc = SQLITE_OK;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return if rc == SQLITE_OK {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_recording_retention_tier(
    mut id: uint64_t,
    mut tier: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if tier < RETENTION_TIER_CRITICAL || tier > RETENTION_TIER_EPHEMERAL {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid retention tier: %d\0".as_ptr() as *const ::core::ffi::c_char,
            tier,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char =
        b"UPDATE recordings SET retention_tier = ? WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(stmt, 1 as ::core::ffi::c_int, tier);
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set retention tier for recording %llu: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            id as ::core::ffi::c_ulonglong,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Recording %llu retention tier set to %d\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
        tier,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_recording_disk_pressure_eligible(
    mut id: uint64_t,
    mut eligible: bool,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char =
        b"UPDATE recordings SET disk_pressure_eligible = ? WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(
        stmt,
        1 as ::core::ffi::c_int,
        if eligible as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set disk pressure eligibility for recording %llu: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            id as ::core::ffi::c_ulonglong,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Recording %llu disk pressure eligible set to %s\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
        if eligible as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
