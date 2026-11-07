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
    unsafe fn sqlite3_busy_timeout(_: *mut sqlite3, ms: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sqlite3_free(_: *mut ::core::ffi::c_void);
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
    unsafe fn sqlite3_column_type(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    unsafe fn memmove(
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
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_t {
    pub label: [::core::ffi::c_char; 32],
    pub confidence: ::core::ffi::c_float,
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub width: ::core::ffi::c_float,
    pub height: ::core::ffi::c_float,
    pub track_id: ::core::ffi::c_int,
    pub zone_id: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_result_t {
    pub count: ::core::ffi::c_int,
    pub detections: [detection_t; 20],
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
pub struct detection_interval_t {
    pub start_time: time_t,
    pub end_time: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_label_summary_t {
    pub label: [::core::ffi::c_char; 32],
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_detection_summary_t {
    pub has_detection: bool,
    pub label_count: ::core::ffi::c_int,
    pub labels: [detection_label_summary_t; 10],
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_VARIABLE_NUMBER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MAX_LABEL_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
unsafe extern "C" fn store_detections_with_source(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
    mut timestamp: time_t,
    mut recording_id: uint64_t,
    mut source: *const ::core::ffi::c_char,
    mut open_interval: bool,
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
            b"Database not initialized when trying to store detections\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if stream_name.is_null() || result.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for store_detections_in_db: stream_name=%p, result=%p\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            result,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*result).count < 0 as ::core::ffi::c_int || (*result).count > MAX_DETECTIONS {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid detection count: %d (expected 0..%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*result).count,
            20 as ::core::ffi::c_int,
        );
        return -1 as ::core::ffi::c_int;
    }
    if timestamp == 0 as time_t {
        timestamp = time(::core::ptr::null_mut::<time_t>());
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Storing %d detections in database for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        (*result).count,
        stream_name,
    );
    if (*result).count > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"First detection: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*(&raw const (*result).detections as *const detection_t).offset(0isize))
                .label as *const ::core::ffi::c_char,
            ((*result).detections[0usize].confidence * 100.0f32) as ::core::ffi::c_double,
            (*result).detections[0usize].x as ::core::ffi::c_double,
            (*result).detections[0usize].y as ::core::ffi::c_double,
            (*result).detections[0usize].width as ::core::ffi::c_double,
            (*result).detections[0usize].height as ::core::ffi::c_double,
        );
    }
    pthread_mutex_lock(db_mutex);
    let mut err_msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    rc = sqlite3_exec(
        db,
        b"BEGIN TRANSACTION;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        &raw mut err_msg,
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to begin transaction: %s\0".as_ptr() as *const ::core::ffi::c_char,
            err_msg,
        );
        sqlite3_free(err_msg as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO detections (stream_name, timestamp, label, confidence, x, y, width, height, track_id, zone_id, recording_id, source, event_end_time, camera_uuid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, COALESCE((SELECT camera_uuid FROM recordings WHERE id = NULLIF(?, 0)), NULLIF(?, ''), (SELECT camera_uuid FROM streams WHERE name = ?)));\0"
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
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*result).count {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, timestamp as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw const (*(&raw const (*result).detections as *const detection_t).offset(i as isize))
                .label as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_double(
            stmt,
            4 as ::core::ffi::c_int,
            (*result).detections[i as usize].confidence as ::core::ffi::c_double,
        );
        sqlite3_bind_double(
            stmt,
            5 as ::core::ffi::c_int,
            (*result).detections[i as usize].x as ::core::ffi::c_double,
        );
        sqlite3_bind_double(
            stmt,
            6 as ::core::ffi::c_int,
            (*result).detections[i as usize].y as ::core::ffi::c_double,
        );
        sqlite3_bind_double(
            stmt,
            7 as ::core::ffi::c_int,
            (*result).detections[i as usize].width as ::core::ffi::c_double,
        );
        sqlite3_bind_double(
            stmt,
            8 as ::core::ffi::c_int,
            (*result).detections[i as usize].height as ::core::ffi::c_double,
        );
        sqlite3_bind_int(
            stmt,
            9 as ::core::ffi::c_int,
            (*result).detections[i as usize].track_id,
        );
        sqlite3_bind_text(
            stmt,
            10 as ::core::ffi::c_int,
            &raw const (*(&raw const (*result).detections as *const detection_t).offset(i as isize))
                .zone_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        if recording_id > 0 as uint64_t {
            sqlite3_bind_int64(
                stmt,
                11 as ::core::ffi::c_int,
                recording_id as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(stmt, 11 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            stmt,
            12 as ::core::ffi::c_int,
            if !source.is_null() {
                source
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        if open_interval {
            sqlite3_bind_null(stmt, 13 as ::core::ffi::c_int);
        } else {
            sqlite3_bind_int64(stmt, 13 as ::core::ffi::c_int, timestamp as sqlite3_int64);
        }
        sqlite3_bind_int64(
            stmt,
            14 as ::core::ffi::c_int,
            recording_id as sqlite3_int64,
        );
        sqlite3_bind_text(
            stmt,
            15 as ::core::ffi::c_int,
            if !camera_uuid.is_null() {
                camera_uuid
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            16 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        rc = sqlite3_step(stmt);
        if rc != SQLITE_DONE {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to insert detection %d: %s\0".as_ptr() as *const ::core::ffi::c_char,
                i,
                sqlite3_errmsg(db),
            );
            sqlite3_finalize(stmt);
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            pthread_mutex_unlock(db_mutex);
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_reset(stmt);
        sqlite3_clear_bindings(stmt);
        i += 1;
    }
    sqlite3_finalize(stmt);
    rc = sqlite3_exec(
        db,
        b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        &raw mut err_msg,
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to commit transaction: %s\0".as_ptr() as *const ::core::ffi::c_char,
            err_msg,
        );
        sqlite3_free(err_msg as *mut ::core::ffi::c_void);
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(db_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Successfully stored %d detections in database for stream %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*result).count,
        stream_name,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn store_detections_in_db(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
    mut timestamp: time_t,
    mut recording_id: uint64_t,
) -> ::core::ffi::c_int {
    return store_detections_with_source(
        ::core::ptr::null::<::core::ffi::c_char>(),
        stream_name,
        result,
        timestamp,
        recording_id,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn store_detections_in_db_for_camera(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
    mut timestamp: time_t,
    mut recording_id: uint64_t,
) -> ::core::ffi::c_int {
    return store_detections_with_source(
        camera_uuid,
        stream_name,
        result,
        timestamp,
        recording_id,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn store_external_motion_detections(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
    mut timestamp: time_t,
    mut recording_id: uint64_t,
) -> ::core::ffi::c_int {
    return store_detections_with_source(
        ::core::ptr::null::<::core::ffi::c_char>(),
        stream_name,
        result,
        timestamp,
        recording_id,
        b"external_motion\0".as_ptr() as *const ::core::ffi::c_char,
        r#true != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_external_motion_detections(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut end_time: time_t,
) -> ::core::ffi::c_int {
    if camera_uuid.is_null()
        || *camera_uuid.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if end_time == 0 as time_t {
        end_time = time(::core::ptr::null_mut::<time_t>());
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"UPDATE detections SET event_end_time = CASE WHEN ? < timestamp THEN timestamp ELSE ? END WHERE source = 'external_motion' AND event_end_time IS NULL AND (camera_uuid = ? OR ((camera_uuid IS NULL OR camera_uuid = '') AND stream_name = ?));\0"
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
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, end_time as sqlite3_int64);
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, end_time as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            camera_uuid,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            4 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
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
    pthread_mutex_unlock(db_mutex);
    return changed;
}
unsafe extern "C" fn get_external_motion_intervals_for_identity(
    mut identity: *const ::core::ffi::c_char,
    mut by_camera_uuid: bool,
    mut start_time: time_t,
    mut end_time: time_t,
    mut intervals: *mut detection_interval_t,
    mut max_intervals: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if identity.is_null() || intervals.is_null() || max_intervals <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: [::core::ffi::c_char; 1024] = [0; 1024];
    snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) FROM detections d WHERE %s AND source = 'external_motion' AND timestamp <= ? AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ? GROUP BY timestamp, event_end_time ORDER BY timestamp ASC LIMIT ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if by_camera_uuid as ::core::ffi::c_int != 0 {
            b"((d.camera_uuid != '' AND d.camera_uuid = ?1) OR ((d.camera_uuid IS NULL OR d.camera_uuid = '') AND d.stream_name IN (SELECT name FROM streams WHERE camera_uuid = ?1)))\0"
                .as_ptr() as *const ::core::ffi::c_char
        } else {
            b"stream_name = ?1\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        identity,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, end_time as sqlite3_int64);
    sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, start_time as sqlite3_int64);
    sqlite3_bind_int(stmt, 4 as ::core::ffi::c_int, max_intervals);
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut step_result: ::core::ffi::c_int = 0;
    loop {
        step_result = sqlite3_step(stmt);
        if !(step_result == SQLITE_ROW && count < max_intervals) {
            break;
        }
        (*intervals.offset(count as isize)).start_time =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as time_t;
        (*intervals.offset(count as isize)).end_time =
            sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as time_t;
        count += 1;
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return if step_result == SQLITE_DONE || step_result == SQLITE_ROW {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_external_motion_detection_intervals(
    mut stream_name: *const ::core::ffi::c_char,
    mut start_time: time_t,
    mut end_time: time_t,
    mut intervals: *mut detection_interval_t,
    mut max_intervals: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return get_external_motion_intervals_for_identity(
        stream_name,
        r#false != 0,
        start_time,
        end_time,
        intervals,
        max_intervals,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_external_motion_detection_intervals_by_camera_uuid(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut start_time: time_t,
    mut end_time: time_t,
    mut intervals: *mut detection_interval_t,
    mut max_intervals: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return get_external_motion_intervals_for_identity(
        camera_uuid,
        r#true != 0,
        start_time,
        end_time,
        intervals,
        max_intervals,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_detections_from_db_time_range(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *mut detection_result_t,
    mut max_age: uint64_t,
    mut start_time: time_t,
    mut end_time: time_t,
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
    if stream_name.is_null() || result.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_detections_from_db_time_range\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    pthread_mutex_lock(db_mutex);
    let mut sql: [::core::ffi::c_char; 2048] = [0; 2048];
    if start_time > 0 as time_t && end_time > 0 as time_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Getting detections for stream %s between %lld and %lld (MAX_DETECTIONS=%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            start_time as ::core::ffi::c_longlong,
            end_time as ::core::ffi::c_longlong,
            20 as ::core::ffi::c_int,
        );
        snprintf(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"SELECT label, confidence, x, y, width, height FROM (SELECT label, confidence, x, y, width, height, timestamp FROM detections WHERE stream_name = ? AND source != 'external_motion' AND timestamp >= ? AND timestamp <= ? UNION ALL SELECT label, confidence, x, y, width, height, timestamp FROM detections WHERE stream_name = ? AND source = 'external_motion' AND timestamp <= ? AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?) ORDER BY timestamp DESC LIMIT ?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
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
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, start_time as sqlite3_int64);
        sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, end_time as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            4 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 5 as ::core::ffi::c_int, end_time as sqlite3_int64);
        sqlite3_bind_int64(stmt, 6 as ::core::ffi::c_int, start_time as sqlite3_int64);
        sqlite3_bind_int(stmt, 7 as ::core::ffi::c_int, MAX_DETECTIONS);
    } else if start_time > 0 as time_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Getting detections for stream %s from %lld\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
            start_time as ::core::ffi::c_longlong,
        );
        snprintf(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"SELECT label, confidence, x, y, width, height FROM (SELECT label, confidence, x, y, width, height, timestamp FROM detections WHERE stream_name = ? AND source != 'external_motion' AND timestamp >= ? UNION ALL SELECT label, confidence, x, y, width, height, timestamp FROM detections WHERE stream_name = ? AND source = 'external_motion' AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?) ORDER BY timestamp DESC LIMIT ?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
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
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, start_time as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, start_time as sqlite3_int64);
        sqlite3_bind_int(stmt, 5 as ::core::ffi::c_int, MAX_DETECTIONS);
    } else if end_time > 0 as time_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Getting detections for stream %s until %lld\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
            end_time as ::core::ffi::c_longlong,
        );
        snprintf(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"SELECT label, confidence, x, y, width, height FROM detections WHERE stream_name = ? AND timestamp <= ? ORDER BY timestamp DESC LIMIT ?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
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
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, end_time as sqlite3_int64);
        sqlite3_bind_int(stmt, 3 as ::core::ffi::c_int, MAX_DETECTIONS);
    } else if max_age > 0 as uint64_t {
        let mut cutoff_time: time_t = time(::core::ptr::null_mut::<time_t>()) - max_age as time_t;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Getting detections for stream %s since %lld (max age %llu seconds)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            cutoff_time as ::core::ffi::c_longlong,
            max_age as ::core::ffi::c_ulonglong,
        );
        snprintf(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"SELECT label, confidence, x, y, width, height FROM (SELECT label, confidence, x, y, width, height, timestamp FROM detections WHERE stream_name = ? AND source != 'external_motion' AND timestamp >= ? UNION ALL SELECT label, confidence, x, y, width, height, timestamp FROM detections WHERE stream_name = ? AND source = 'external_motion' AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?) ORDER BY timestamp DESC LIMIT ?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
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
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, cutoff_time as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, cutoff_time as sqlite3_int64);
        sqlite3_bind_int(stmt, 5 as ::core::ffi::c_int, MAX_DETECTIONS);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Getting latest detections for stream %s (no time filters)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        snprintf(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"SELECT label, confidence, x, y, width, height FROM detections WHERE stream_name = ? ORDER BY timestamp DESC LIMIT ?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
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
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int(stmt, 2 as ::core::ffi::c_int, MAX_DETECTIONS);
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while sqlite3_step(stmt) == SQLITE_ROW && count < MAX_DETECTIONS {
        let mut label: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut confidence: ::core::ffi::c_float =
            sqlite3_column_double(stmt, 1 as ::core::ffi::c_int) as ::core::ffi::c_float;
        let mut x: ::core::ffi::c_float =
            sqlite3_column_double(stmt, 2 as ::core::ffi::c_int) as ::core::ffi::c_float;
        let mut y: ::core::ffi::c_float =
            sqlite3_column_double(stmt, 3 as ::core::ffi::c_int) as ::core::ffi::c_float;
        let mut width: ::core::ffi::c_float =
            sqlite3_column_double(stmt, 4 as ::core::ffi::c_int) as ::core::ffi::c_float;
        let mut height: ::core::ffi::c_float =
            sqlite3_column_double(stmt, 5 as ::core::ffi::c_int) as ::core::ffi::c_float;
        if !label.is_null() {
            safe_strcpy(
                &raw mut (*(&raw mut (*result).detections as *mut detection_t)
                    .offset(count as isize))
                .label as *mut ::core::ffi::c_char,
                label,
                MAX_LABEL_LENGTH as size_t,
                0 as size_t,
            );
        } else {
            safe_strcpy(
                &raw mut (*(&raw mut (*result).detections as *mut detection_t)
                    .offset(count as isize))
                .label as *mut ::core::ffi::c_char,
                b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
                MAX_LABEL_LENGTH as size_t,
                0 as size_t,
            );
        }
        (*result).detections[count as usize].confidence = confidence;
        (*result).detections[count as usize].x = x;
        (*result).detections[count as usize].y = y;
        (*result).detections[count as usize].width = width;
        (*result).detections[count as usize].height = height;
        count += 1;
    }
    (*result).count = count;
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Found %d detections in database for stream %s (start=%lld, end=%lld)\0".as_ptr()
            as *const ::core::ffi::c_char,
        count,
        stream_name,
        start_time as ::core::ffi::c_longlong,
        end_time as ::core::ffi::c_longlong,
    );
    return count;
}
unsafe extern "C" fn read_detection_rows(
    mut db: *mut sqlite3,
    mut stream_name: *const ::core::ffi::c_char,
    mut maximum: ::core::ffi::c_int,
    mut result: *mut detection_result_t,
    mut timestamps: *mut time_t,
    mut end_timestamps: *mut time_t,
    mut max_age: uint64_t,
    mut start_time: time_t,
    mut end_time: time_t,
) -> ::core::ffi::c_int {
    if db.is_null()
        || stream_name.is_null()
        || timestamps.is_null()
        || maximum < 0 as ::core::ffi::c_int
        || maximum > MAX_DETECTIONS
    {
        return -1 as ::core::ffi::c_int;
    }
    if maximum == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: [::core::ffi::c_char; 2048] = [0; 2048];
    if start_time > 0 as time_t && end_time > 0 as time_t {
        snprintf(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"SELECT timestamp, interval_end, label, confidence, x, y, width, height FROM (SELECT timestamp, COALESCE(event_end_time, timestamp) AS interval_end, label, confidence, x, y, width, height FROM detections WHERE stream_name = ? AND source != 'external_motion' AND timestamp >= ? AND timestamp <= ? UNION ALL SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) AS interval_end, label, confidence, x, y, width, height FROM detections WHERE stream_name = ? AND source = 'external_motion' AND timestamp <= ? AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?) ORDER BY timestamp DESC LIMIT ?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
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
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, start_time as sqlite3_int64);
        sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, end_time as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            4 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 5 as ::core::ffi::c_int, end_time as sqlite3_int64);
        sqlite3_bind_int64(stmt, 6 as ::core::ffi::c_int, start_time as sqlite3_int64);
        sqlite3_bind_int(stmt, 7 as ::core::ffi::c_int, MAX_DETECTIONS);
    } else if start_time > 0 as time_t {
        snprintf(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"SELECT timestamp, interval_end, label, confidence, x, y, width, height FROM (SELECT timestamp, COALESCE(event_end_time, timestamp) AS interval_end, label, confidence, x, y, width, height FROM detections WHERE stream_name = ? AND source != 'external_motion' AND timestamp >= ? UNION ALL SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) AS interval_end, label, confidence, x, y, width, height FROM detections WHERE stream_name = ? AND source = 'external_motion' AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?) ORDER BY timestamp DESC LIMIT ?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
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
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, start_time as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, start_time as sqlite3_int64);
        sqlite3_bind_int(stmt, 5 as ::core::ffi::c_int, MAX_DETECTIONS);
    } else if end_time > 0 as time_t {
        snprintf(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)), label, confidence, x, y, width, height FROM detections WHERE stream_name = ? AND timestamp <= ? ORDER BY timestamp DESC LIMIT ?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
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
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, end_time as sqlite3_int64);
        sqlite3_bind_int(stmt, 3 as ::core::ffi::c_int, MAX_DETECTIONS);
    } else if max_age > 0 as uint64_t {
        let mut cutoff_time: time_t = time(::core::ptr::null_mut::<time_t>()) - max_age as time_t;
        snprintf(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"SELECT timestamp, interval_end, label, confidence, x, y, width, height FROM (SELECT timestamp, COALESCE(event_end_time, timestamp) AS interval_end, label, confidence, x, y, width, height FROM detections WHERE stream_name = ? AND source != 'external_motion' AND timestamp >= ? UNION ALL SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) AS interval_end, label, confidence, x, y, width, height FROM detections WHERE stream_name = ? AND source = 'external_motion' AND COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?) ORDER BY timestamp DESC LIMIT ?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
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
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, cutoff_time as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, cutoff_time as sqlite3_int64);
        sqlite3_bind_int(stmt, 5 as ::core::ffi::c_int, MAX_DETECTIONS);
    } else {
        snprintf(
            &raw mut sql as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"SELECT timestamp, COALESCE(event_end_time, CAST(strftime('%%s','now') AS INTEGER)), label, confidence, x, y, width, height FROM detections WHERE stream_name = ? ORDER BY timestamp DESC LIMIT ?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
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
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int(stmt, 2 as ::core::ffi::c_int, MAX_DETECTIONS);
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while count < maximum && {
        rc = sqlite3_step(stmt);
        rc == SQLITE_ROW
    } {
        *timestamps.offset(count as isize) =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as time_t;
        if !end_timestamps.is_null() {
            *end_timestamps.offset(count as isize) =
                sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as time_t;
        }
        if !result.is_null() {
            let mut detection: *mut detection_t =
                (&raw mut (*result).detections as *mut detection_t).offset(count as isize);
            let mut label: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            safe_strcpy(
                &raw mut (*detection).label as *mut ::core::ffi::c_char,
                if !label.is_null() {
                    label
                } else {
                    b"unknown\0".as_ptr() as *const ::core::ffi::c_char
                },
                MAX_LABEL_LENGTH as size_t,
                0 as size_t,
            );
            (*detection).confidence =
                sqlite3_column_double(stmt, 3 as ::core::ffi::c_int) as ::core::ffi::c_float;
            (*detection).x =
                sqlite3_column_double(stmt, 4 as ::core::ffi::c_int) as ::core::ffi::c_float;
            (*detection).y =
                sqlite3_column_double(stmt, 5 as ::core::ffi::c_int) as ::core::ffi::c_float;
            (*detection).width =
                sqlite3_column_double(stmt, 6 as ::core::ffi::c_int) as ::core::ffi::c_float;
            (*detection).height =
                sqlite3_column_double(stmt, 7 as ::core::ffi::c_int) as ::core::ffi::c_float;
        }
        count += 1;
    }
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE && count < maximum {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to read detection rows: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    if !result.is_null() {
        (*result).count = count;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_detection_time_ranges(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
    mut timestamps: *mut time_t,
    mut end_timestamps: *mut time_t,
    mut max_age: uint64_t,
    mut start_time: time_t,
    mut end_time: time_t,
) -> ::core::ffi::c_int {
    if result.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    pthread_mutex_lock(mutex);
    let mut count: ::core::ffi::c_int = read_detection_rows(
        get_db_handle(),
        stream_name,
        (*result).count,
        ::core::ptr::null_mut::<detection_result_t>(),
        timestamps,
        end_timestamps,
        max_age,
        start_time,
        end_time,
    );
    pthread_mutex_unlock(mutex);
    return if count < 0 as ::core::ffi::c_int {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_detection_results_for_camera_readonly(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut legacy_stream_name: *const ::core::ffi::c_char,
    mut result: *mut detection_result_t,
    mut timestamps: *mut time_t,
    mut end_timestamps: *mut time_t,
    mut max_age: uint64_t,
    mut start_time: time_t,
    mut end_time: time_t,
) -> ::core::ffi::c_int {
    if camera_uuid.is_null()
        || *camera_uuid == 0
        || legacy_stream_name.is_null()
        || result.is_null()
        || timestamps.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    let mut connection: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut connection) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_busy_timeout(connection, 1000 as ::core::ffi::c_int);
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut lower: time_t = start_time;
    if start_time <= 0 as time_t && end_time <= 0 as time_t && max_age > 0 as uint64_t {
        lower = now - max_age as time_t;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT timestamp, COALESCE(event_end_time, CASE WHEN source='external_motion' THEN ?5 ELSE timestamp END), label,confidence,x,y,width,height FROM detections WHERE (camera_uuid=?1 OR ((camera_uuid IS NULL OR camera_uuid='') AND stream_name=?2)) AND (?3<=0 OR (source='external_motion' AND COALESCE(event_end_time,?5)>=?3) OR (source!='external_motion' AND timestamp>=?3)) AND (?4<=0 OR timestamp<=?4) ORDER BY timestamp DESC LIMIT ?6\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        connection,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            camera_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            legacy_stream_name,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(statement, 3 as ::core::ffi::c_int, lower as sqlite3_int64);
        sqlite3_bind_int64(
            statement,
            4 as ::core::ffi::c_int,
            end_time as sqlite3_int64,
        );
        sqlite3_bind_int64(statement, 5 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int(statement, 6 as ::core::ffi::c_int, MAX_DETECTIONS);
        loop {
            rc = sqlite3_step(statement);
            if rc != SQLITE_ROW {
                break;
            }
            let mut index: ::core::ffi::c_int = (*result).count;
            let mut detection: *mut detection_t =
                (&raw mut (*result).detections as *mut detection_t).offset(index as isize);
            *timestamps.offset(index as isize) =
                sqlite3_column_int64(statement, 0 as ::core::ffi::c_int) as time_t;
            if !end_timestamps.is_null() {
                *end_timestamps.offset(index as isize) =
                    sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as time_t;
            }
            let mut label: *const ::core::ffi::c_char =
                sqlite3_column_text(statement, 2 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
            safe_strcpy(
                &raw mut (*detection).label as *mut ::core::ffi::c_char,
                if !label.is_null() {
                    label
                } else {
                    b"unknown\0".as_ptr() as *const ::core::ffi::c_char
                },
                MAX_LABEL_LENGTH as size_t,
                0 as size_t,
            );
            (*detection).confidence =
                sqlite3_column_double(statement, 3 as ::core::ffi::c_int) as ::core::ffi::c_float;
            (*detection).x =
                sqlite3_column_double(statement, 4 as ::core::ffi::c_int) as ::core::ffi::c_float;
            (*detection).y =
                sqlite3_column_double(statement, 5 as ::core::ffi::c_int) as ::core::ffi::c_float;
            (*detection).width =
                sqlite3_column_double(statement, 6 as ::core::ffi::c_int) as ::core::ffi::c_float;
            (*detection).height =
                sqlite3_column_double(statement, 7 as ::core::ffi::c_int) as ::core::ffi::c_float;
            (*result).count += 1;
        }
    }
    sqlite3_finalize(statement);
    db_close_readonly_connection(connection);
    if rc != SQLITE_DONE {
        memset(
            result as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<detection_result_t>(),
        );
        return -1 as ::core::ffi::c_int;
    }
    return (*result).count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_detection_results_readonly(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *mut detection_result_t,
    mut timestamps: *mut time_t,
    mut end_timestamps: *mut time_t,
    mut max_age: uint64_t,
    mut start_time: time_t,
    mut end_time: time_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null() || result.is_null() || timestamps.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    let mut connection: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut connection) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_busy_timeout(connection, 1000 as ::core::ffi::c_int);
    let mut count: ::core::ffi::c_int = read_detection_rows(
        connection,
        stream_name,
        MAX_DETECTIONS,
        result,
        timestamps,
        end_timestamps,
        max_age,
        start_time,
        end_time,
    );
    db_close_readonly_connection(connection);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_detection_timestamps(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
    mut timestamps: *mut time_t,
    mut max_age: uint64_t,
    mut start_time: time_t,
    mut end_time: time_t,
) -> ::core::ffi::c_int {
    return get_detection_time_ranges(
        stream_name,
        result,
        timestamps,
        ::core::ptr::null_mut::<time_t>(),
        max_age,
        start_time,
        end_time,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_detections_from_db(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *mut detection_result_t,
    mut max_age: uint64_t,
) -> ::core::ffi::c_int {
    return get_detections_from_db_time_range(
        stream_name,
        result,
        max_age,
        0 as time_t,
        0 as time_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn has_detections_in_time_range(
    mut stream_name: *const ::core::ffi::c_char,
    mut start_time: time_t,
    mut end_time: time_t,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut has_detections: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream name is required for has_detections_in_time_range\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Checking for detections: stream=%s, start=%lld, end=%lld\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
        start_time as ::core::ffi::c_longlong,
        end_time as ::core::ffi::c_longlong,
    );
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"SELECT (EXISTS(SELECT 1 FROM detections WHERE stream_name = ? AND source != 'external_motion' AND timestamp >= ? AND timestamp <= ? LIMIT 1) OR EXISTS(SELECT 1 FROM detections WHERE stream_name = ? AND source = 'external_motion' AND timestamp <= ? AND COALESCE(event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= ? LIMIT 1));\0"
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
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, start_time as sqlite3_int64);
    sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, end_time as sqlite3_int64);
    sqlite3_bind_text(
        stmt,
        4 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 5 as ::core::ffi::c_int, end_time as sqlite3_int64);
    sqlite3_bind_int64(stmt, 6 as ::core::ffi::c_int, start_time as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc == SQLITE_ROW {
        has_detections = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Detection check result for stream %s: %d\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
            has_detections,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to check for detections: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return has_detections;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_old_detections(mut max_age: uint64_t) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut deleted_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
    let mut sql: *const ::core::ffi::c_char = b"DELETE FROM detections WHERE CASE WHEN source = 'external_motion' AND event_end_time IS NULL THEN CAST(strftime('%s','now') AS INTEGER) ELSE COALESCE(event_end_time, timestamp) END < ?;\0"
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
    let mut cutoff_time: time_t = time(::core::ptr::null_mut::<time_t>()) - max_age as time_t;
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, cutoff_time as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to delete old detections: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    deleted_count = sqlite3_changes(db);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Deleted %d old detections from database\0".as_ptr() as *const ::core::ffi::c_char,
        deleted_count,
    );
    return deleted_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_old_detections_for_stream(
    mut stream_name: *const ::core::ffi::c_char,
    mut max_age: uint64_t,
    mut batch_limit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut deleted_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || batch_limit <= 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
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
    let mut sql: *const ::core::ffi::c_char = b"DELETE FROM detections WHERE id IN (  SELECT id FROM detections  WHERE stream_name = ?    AND CASE WHEN source = 'external_motion' AND event_end_time IS NULL             THEN CAST(strftime('%s','now') AS INTEGER)             ELSE COALESCE(event_end_time, timestamp) END < ?  LIMIT ?);\0"
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
            b"Failed to prepare detection prune statement: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut cutoff_time: time_t = time(::core::ptr::null_mut::<time_t>()) - max_age as time_t;
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, cutoff_time as sqlite3_int64);
    sqlite3_bind_int(stmt, 3 as ::core::ffi::c_int, batch_limit);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prune detections for stream %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    deleted_count = sqlite3_changes(db);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return deleted_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_detection_labels_summary(
    mut stream_name: *const ::core::ffi::c_char,
    mut start_time: time_t,
    mut end_time: time_t,
    mut labels: *mut detection_label_summary_t,
    mut max_labels: ::core::ffi::c_int,
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
    if stream_name.is_null() || labels.is_null() || max_labels <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_detection_labels_summary\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        labels as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (max_labels as size_t).wrapping_mul(::core::mem::size_of::<detection_label_summary_t>()),
    );
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"SELECT label, SUM(cnt) AS cnt FROM (SELECT label, COUNT(*) AS cnt FROM detections WHERE stream_name = ? AND source != 'external_motion' AND timestamp >= ? AND timestamp <= ? GROUP BY label UNION ALL SELECT label, COUNT(*) AS cnt FROM detections WHERE stream_name = ? AND source = 'external_motion' AND timestamp <= ? AND COALESCE(event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= ? GROUP BY label) GROUP BY label ORDER BY cnt DESC LIMIT ?;\0"
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
            b"Failed to prepare statement for get_detection_labels_summary: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, start_time as sqlite3_int64);
    sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, end_time as sqlite3_int64);
    sqlite3_bind_text(
        stmt,
        4 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 5 as ::core::ffi::c_int, end_time as sqlite3_int64);
    sqlite3_bind_int64(stmt, 6 as ::core::ffi::c_int, start_time as sqlite3_int64);
    sqlite3_bind_int(stmt, 7 as ::core::ffi::c_int, max_labels);
    loop {
        rc = sqlite3_step(stmt);
        if !(rc == SQLITE_ROW && count < max_labels) {
            break;
        }
        let mut label: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut label_count: ::core::ffi::c_int = sqlite3_column_int(stmt, 1 as ::core::ffi::c_int);
        if !label.is_null() {
            safe_strcpy(
                &raw mut (*labels.offset(count as isize)).label as *mut ::core::ffi::c_char,
                label,
                MAX_LABEL_LENGTH as size_t,
                0 as size_t,
            );
            (*labels.offset(count as isize)).count = label_count;
            count += 1;
        }
    }
    if rc != SQLITE_DONE && rc != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to fetch detection labels: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return count;
}
pub const RECORDING_DETECTION_QUERY_BATCH_SIZE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_detection_summaries_on_connection(
    mut db: *mut sqlite3,
    mut recordings: *const recording_metadata_t,
    mut count: ::core::ffi::c_int,
    mut summaries: *mut recording_detection_summary_t,
) -> ::core::ffi::c_int {
    return detection_summaries_for_scope(db, recordings, count, summaries, r#false != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_detection_summaries_for_cameras_on_connection(
    mut db: *mut sqlite3,
    mut recordings: *const recording_metadata_t,
    mut count: ::core::ffi::c_int,
    mut summaries: *mut recording_detection_summary_t,
) -> ::core::ffi::c_int {
    return detection_summaries_for_scope(db, recordings, count, summaries, r#true != 0);
}
unsafe extern "C" fn detection_summaries_for_scope(
    mut db: *mut sqlite3,
    mut recordings: *const recording_metadata_t,
    mut count: ::core::ffi::c_int,
    mut summaries: *mut recording_detection_summary_t,
    mut camera_scope: bool,
) -> ::core::ffi::c_int {
    if db.is_null()
        || recordings.is_null()
        || count <= 0 as ::core::ffi::c_int
        || summaries.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        summaries as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (count as size_t).wrapping_mul(::core::mem::size_of::<recording_detection_summary_t>()),
    );
    let mut batch_limit: ::core::ffi::c_int =
        sqlite3_limit(db, SQLITE_LIMIT_VARIABLE_NUMBER, -1 as ::core::ffi::c_int)
            / 5 as ::core::ffi::c_int;
    if batch_limit < 1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if batch_limit > RECORDING_DETECTION_QUERY_BATCH_SIZE {
        batch_limit = RECORDING_DETECTION_QUERY_BATCH_SIZE;
    }
    let mut batch_start: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while batch_start < count {
        let mut batch_count: ::core::ffi::c_int = count - batch_start;
        if batch_count > batch_limit {
            batch_count = batch_limit;
        }
        let mut sql: [::core::ffi::c_char; 8192] = [0; 8192];
        safe_strcpy(
            &raw mut sql as *mut ::core::ffi::c_char,
            b"WITH requested(recording_id, stream_name, start_time, end_time, camera_uuid) AS (VALUES \0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
            0 as size_t,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < batch_count {
            if i > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"(?,?,?,?,?)\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
            );
            i += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b"), label_counts AS (SELECT q.recording_id, d.label, COUNT(*) AS cnt FROM requested q CROSS JOIN detections d ON d.recording_id = q.recording_id \0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        );
        if camera_scope {
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"AND ((d.camera_uuid != '' AND d.camera_uuid = COALESCE(NULLIF(q.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=q.stream_name))) OR ((d.camera_uuid IS NULL OR d.camera_uuid = '') AND d.stream_name IN (SELECT name FROM streams WHERE camera_uuid = COALESCE(NULLIF(q.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=q.stream_name))))) \0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
            );
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b"WHERE d.source != 'external_motion' GROUP BY q.recording_id, d.label UNION ALL SELECT q.recording_id, d.label, COUNT(*) AS cnt FROM requested q CROSS JOIN detections d ON \0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        );
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            if camera_scope as ::core::ffi::c_int != 0 {
                b"((d.camera_uuid != '' AND d.camera_uuid = COALESCE(NULLIF(q.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=q.stream_name))) OR ((d.camera_uuid IS NULL OR d.camera_uuid = '') AND d.stream_name IN (SELECT name FROM streams WHERE camera_uuid = COALESCE(NULLIF(q.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=q.stream_name)))))\0"
                    .as_ptr() as *const ::core::ffi::c_char
            } else {
                b"d.stream_name = q.stream_name\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        );
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND d.timestamp >= q.start_time AND d.timestamp <= q.end_time WHERE (d.recording_id IS NULL OR d.recording_id != q.recording_id) AND d.source != 'external_motion' GROUP BY q.recording_id, d.label \0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        );
        let mut legacy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while legacy
            < if camera_scope as ::core::ffi::c_int != 0 {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }
        {
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"UNION ALL SELECT q.recording_id, d.label, COUNT(*) AS cnt FROM requested q \0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
            );
            if camera_scope as ::core::ffi::c_int != 0 && legacy != 0 {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b"CROSS JOIN streams legacy ON legacy.camera_uuid = COALESCE(NULLIF(q.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=q.stream_name)) CROSS JOIN detections d ON d.stream_name = legacy.name AND (d.camera_uuid IS NULL OR d.camera_uuid = '') \0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                );
            } else {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b"CROSS JOIN detections d ON \0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                );
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    if camera_scope as ::core::ffi::c_int != 0 {
                        b"d.camera_uuid = COALESCE(NULLIF(q.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=q.stream_name)) \0"
                            .as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"d.stream_name = q.stream_name \0".as_ptr() as *const ::core::ffi::c_char
                    },
                    ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"AND d.timestamp <= q.end_time WHERE d.source = 'external_motion' AND COALESCE(d.event_end_time, CAST(strftime('%s','now') AS INTEGER)) >= q.start_time GROUP BY q.recording_id, d.label \0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
            );
            legacy += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b") SELECT recording_id, label, SUM(cnt) AS label_count FROM label_counts GROUP BY recording_id, label ORDER BY recording_id, label_count DESC, label;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        );
        let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
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
                b"Failed to prepare recording detection summaries: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            return -1 as ::core::ffi::c_int;
        }
        let mut param_index: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < batch_count {
            let mut recording: *const recording_metadata_t =
                recordings.offset((batch_start + i_0) as isize);
            let c2rust_fresh0 = param_index;
            param_index += 1;
            sqlite3_bind_int64(stmt, c2rust_fresh0, (*recording).id as sqlite3_int64);
            let c2rust_fresh1 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh1,
                &raw const (*recording).stream_name as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            let c2rust_fresh2 = param_index;
            param_index += 1;
            sqlite3_bind_int64(
                stmt,
                c2rust_fresh2,
                (*recording).start_time as sqlite3_int64,
            );
            let c2rust_fresh3 = param_index;
            param_index += 1;
            sqlite3_bind_int64(stmt, c2rust_fresh3, (*recording).end_time as sqlite3_int64);
            let c2rust_fresh4 = param_index;
            param_index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh4,
                &raw const (*recording).camera_uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                SQLITE_STATIC,
            );
            i_0 += 1;
        }
        loop {
            rc = sqlite3_step(stmt);
            if rc != SQLITE_ROW {
                break;
            }
            let mut recording_id: uint64_t =
                sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
            let mut label: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            let mut label_count: ::core::ffi::c_int =
                sqlite3_column_int(stmt, 2 as ::core::ffi::c_int);
            if label.is_null() {
                continue;
            }
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < batch_count {
                let mut output_index: ::core::ffi::c_int = batch_start + i_1;
                if (*recordings.offset(output_index as isize)).id != recording_id {
                    i_1 += 1;
                } else {
                    let mut summary: *mut recording_detection_summary_t =
                        summaries.offset(output_index as isize);
                    (*summary).has_detection = r#true != 0;
                    if (*summary).label_count < MAX_DETECTION_LABELS {
                        let mut output: *mut detection_label_summary_t =
                            (&raw mut (*summary).labels as *mut detection_label_summary_t)
                                .offset((*summary).label_count as isize);
                        safe_strcpy(
                            &raw mut (*output).label as *mut ::core::ffi::c_char,
                            label,
                            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                            0 as size_t,
                        );
                        (*output).count = label_count;
                        (*summary).label_count += 1;
                    }
                    break;
                }
            }
        }
        sqlite3_finalize(stmt);
        if rc != SQLITE_DONE {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to fetch recording detection summaries: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            return -1 as ::core::ffi::c_int;
        }
        batch_start += batch_limit;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_detection_summaries(
    mut recordings: *const recording_metadata_t,
    mut count: ::core::ffi::c_int,
    mut summaries: *mut recording_detection_summary_t,
) -> ::core::ffi::c_int {
    if recordings.is_null() || count <= 0 as ::core::ffi::c_int || summaries.is_null() {
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
    let mut result: ::core::ffi::c_int =
        get_recording_detection_summaries_on_connection(db, recordings, count, summaries);
    db_close_readonly_connection(db);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_detection_summaries_for_cameras(
    mut recordings: *const recording_metadata_t,
    mut count: ::core::ffi::c_int,
    mut summaries: *mut recording_detection_summary_t,
) -> ::core::ffi::c_int {
    if recordings.is_null() || count <= 0 as ::core::ffi::c_int || summaries.is_null() {
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
    let mut result: ::core::ffi::c_int =
        get_recording_detection_summaries_for_cameras_on_connection(
            db, recordings, count, summaries,
        );
    db_close_readonly_connection(db);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_all_unique_detection_labels(
    mut labels: *mut [::core::ffi::c_char; 32],
    mut max_labels: ::core::ffi::c_int,
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
    if labels.is_null() || max_labels <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_all_unique_detection_labels\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        labels as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (max_labels as size_t).wrapping_mul(MAX_LABEL_LENGTH as size_t),
    );
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"SELECT DISTINCT label FROM detections WHERE label IS NOT NULL AND TRIM(label) <> '' ORDER BY label ASC LIMIT ?;\0"
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
            b"Failed to prepare statement for get_all_unique_detection_labels: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(stmt, 1 as ::core::ffi::c_int, max_labels);
    loop {
        rc = sqlite3_step(stmt);
        if !(rc == SQLITE_ROW && count < max_labels) {
            break;
        }
        let mut label: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !label.is_null() {
            safe_strcpy(
                &raw mut *labels.offset(count as isize) as *mut ::core::ffi::c_char,
                label,
                MAX_LABEL_LENGTH as size_t,
                0 as size_t,
            );
            count += 1;
        }
    }
    if rc != SQLITE_DONE && rc != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to fetch unique detection labels: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return count;
}
static mut UNIQUE_LABEL_STEP_SQL: [::core::ffi::c_char; 115] = unsafe {
    ::core::mem::transmute::<
        [u8; 115],
        [::core::ffi::c_char; 115],
    >(
        *b"SELECT MIN(label) FROM detections WHERE stream_name = ? AND label > ? AND label IS NOT NULL AND TRIM(label) <> '';\0",
    )
};
unsafe extern "C" fn merge_unique_label(
    mut labels: *mut [::core::ffi::c_char; 32],
    mut count: ::core::ffi::c_int,
    mut max_labels: ::core::ffi::c_int,
    mut label: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut stored: [::core::ffi::c_char; 32] = [0; 32];
    safe_strcpy(
        &raw mut stored as *mut ::core::ffi::c_char,
        label,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        0 as size_t,
    );
    if stored[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return count;
    }
    let mut insert_at: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while insert_at < count
        && strcmp(
            &raw mut *labels.offset(insert_at as isize) as *mut ::core::ffi::c_char,
            &raw mut stored as *mut ::core::ffi::c_char,
        ) < 0 as ::core::ffi::c_int
    {
        insert_at += 1;
    }
    if insert_at < count
        && strcmp(
            &raw mut *labels.offset(insert_at as isize) as *mut ::core::ffi::c_char,
            &raw mut stored as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return count;
    }
    if insert_at >= max_labels {
        return count;
    }
    let mut move_count: ::core::ffi::c_int = if count < max_labels {
        count - insert_at
    } else {
        max_labels - insert_at - 1 as ::core::ffi::c_int
    };
    if move_count > 0 as ::core::ffi::c_int {
        memmove(
            &raw mut *labels.offset((insert_at + 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw mut *labels.offset(insert_at as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            (move_count as size_t).wrapping_mul(MAX_LABEL_LENGTH as size_t),
        );
    }
    safe_strcpy(
        &raw mut *labels.offset(insert_at as isize) as *mut ::core::ffi::c_char,
        &raw mut stored as *mut ::core::ffi::c_char,
        MAX_LABEL_LENGTH as size_t,
        0 as size_t,
    );
    return if count < max_labels {
        count + 1 as ::core::ffi::c_int
    } else {
        count
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_unique_detection_labels_for_streams_on_connection(
    mut db: *mut sqlite3,
    mut stream_names: *const *const ::core::ffi::c_char,
    mut stream_count: ::core::ffi::c_int,
    mut labels: *mut [::core::ffi::c_char; 32],
    mut max_labels: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stream_names.is_null()
        || stream_count <= 0 as ::core::ffi::c_int
        || labels.is_null()
        || max_labels <= 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        labels as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (max_labels as size_t).wrapping_mul(MAX_LABEL_LENGTH as size_t),
    );
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        &raw const UNIQUE_LABEL_STEP_SQL as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare detection label walk: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut failed: bool = r#false != 0;
    let mut cursor: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while s < stream_count && !failed {
        let mut stream_name: *const ::core::ffi::c_char = *stream_names.offset(s as isize);
        if !(stream_name.is_null()
            || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int)
        {
            free(cursor as *mut ::core::ffi::c_void);
            cursor = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut step: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while step < max_labels {
                sqlite3_reset(stmt);
                sqlite3_clear_bindings(stmt);
                sqlite3_bind_text(
                    stmt,
                    1 as ::core::ffi::c_int,
                    stream_name,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
                sqlite3_bind_text(
                    stmt,
                    2 as ::core::ffi::c_int,
                    if !cursor.is_null() {
                        cursor
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                rc = sqlite3_step(stmt);
                if rc != SQLITE_ROW {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Detection label walk failed for stream %s: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        stream_name,
                        sqlite3_errmsg(db),
                    );
                    failed = r#true != 0;
                    break;
                } else {
                    if sqlite3_column_type(stmt, 0 as ::core::ffi::c_int) == SQLITE_NULL {
                        break;
                    }
                    let mut label: *const ::core::ffi::c_char =
                        sqlite3_column_text(stmt, 0 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    if label.is_null() {
                        break;
                    }
                    let mut next: *mut ::core::ffi::c_char = strdup(label);
                    if next.is_null() {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Out of memory walking detection labels\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        failed = r#true != 0;
                        break;
                    } else {
                        free(cursor as *mut ::core::ffi::c_void);
                        cursor = next;
                        count = merge_unique_label(labels, count, max_labels, label);
                        step += 1;
                    }
                }
            }
        }
        s += 1;
    }
    free(cursor as *mut ::core::ffi::c_void);
    sqlite3_finalize(stmt);
    return if failed as ::core::ffi::c_int != 0 {
        -1 as ::core::ffi::c_int
    } else {
        count
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_unique_detection_labels_for_streams(
    mut stream_names: *const *const ::core::ffi::c_char,
    mut stream_count: ::core::ffi::c_int,
    mut labels: *mut [::core::ffi::c_char; 32],
    mut max_labels: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stream_names.is_null()
        || stream_count <= 0 as ::core::ffi::c_int
        || labels.is_null()
        || max_labels <= 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = get_unique_detection_labels_for_streams_on_connection(
        db,
        stream_names,
        stream_count,
        labels,
        max_labels,
    );
    db_close_readonly_connection(db);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_unique_detection_labels_for_cameras_on_connection(
    mut db: *mut sqlite3,
    mut camera_uuids: *const *const ::core::ffi::c_char,
    mut camera_count: ::core::ffi::c_int,
    mut labels: *mut [::core::ffi::c_char; 32],
    mut max_labels: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if camera_uuids.is_null()
        || camera_count <= 0 as ::core::ffi::c_int
        || labels.is_null()
        || max_labels <= 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        labels as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (max_labels as size_t).wrapping_mul(MAX_LABEL_LENGTH as size_t),
    );
    let mut queries: [*const ::core::ffi::c_char; 2] = [
        b"SELECT MIN(label) FROM detections WHERE camera_uuid=? AND label>? AND label IS NOT NULL AND TRIM(label)<>'';\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"SELECT MIN(label) FROM detections WHERE stream_name=(SELECT name FROM streams WHERE camera_uuid=?) AND label>? AND label IS NOT NULL AND TRIM(label)<>'' AND (camera_uuid IS NULL OR camera_uuid='');\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut branch: size_t = 0 as size_t;
    while branch
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if sqlite3_prepare_v2(
            db,
            queries[branch],
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) != SQLITE_OK
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut camera: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while camera < camera_count {
            if !((*camera_uuids.offset(camera as isize)).is_null()
                || *(*camera_uuids.offset(camera as isize)).offset(0isize) == 0)
            {
                let mut cursor: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut step: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while step < max_labels {
                    sqlite3_reset(stmt);
                    sqlite3_clear_bindings(stmt);
                    sqlite3_bind_text(
                        stmt,
                        1 as ::core::ffi::c_int,
                        *camera_uuids.offset(camera as isize),
                        -1 as ::core::ffi::c_int,
                        SQLITE_STATIC,
                    );
                    sqlite3_bind_text(
                        stmt,
                        2 as ::core::ffi::c_int,
                        if !cursor.is_null() {
                            cursor
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        -1 as ::core::ffi::c_int,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                    if sqlite3_step(stmt) != SQLITE_ROW {
                        free(cursor as *mut ::core::ffi::c_void);
                        sqlite3_finalize(stmt);
                        return -1 as ::core::ffi::c_int;
                    }
                    if sqlite3_column_type(stmt, 0 as ::core::ffi::c_int) == SQLITE_NULL {
                        break;
                    }
                    let mut label: *const ::core::ffi::c_char =
                        sqlite3_column_text(stmt, 0 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    let mut next: *mut ::core::ffi::c_char = if !label.is_null() {
                        strdup(label)
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    };
                    if next.is_null() {
                        free(cursor as *mut ::core::ffi::c_void);
                        sqlite3_finalize(stmt);
                        return -1 as ::core::ffi::c_int;
                    }
                    free(cursor as *mut ::core::ffi::c_void);
                    cursor = next;
                    count = merge_unique_label(labels, count, max_labels, label);
                    step += 1;
                }
                free(cursor as *mut ::core::ffi::c_void);
            }
            camera += 1;
        }
        sqlite3_finalize(stmt);
        branch = branch.wrapping_add(1);
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_unique_detection_labels_for_cameras(
    mut camera_uuids: *const *const ::core::ffi::c_char,
    mut camera_count: ::core::ffi::c_int,
    mut labels: *mut [::core::ffi::c_char; 32],
    mut max_labels: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if camera_uuids.is_null()
        || camera_count <= 0 as ::core::ffi::c_int
        || labels.is_null()
        || max_labels <= 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = get_unique_detection_labels_for_cameras_on_connection(
        db,
        camera_uuids,
        camera_count,
        labels,
        max_labels,
    );
    db_close_readonly_connection(db);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_detections_recording_id(
    mut stream_name: *const ::core::ffi::c_char,
    mut recording_id: uint64_t,
    mut since_time: time_t,
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
    if stream_name.is_null() || recording_id == 0 as uint64_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for update_detections_recording_id\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"UPDATE detections SET recording_id = ?, camera_uuid = COALESCE((SELECT camera_uuid FROM recordings WHERE id = ?), camera_uuid) WHERE stream_name = ? AND timestamp >= ? AND (recording_id IS NULL OR recording_id = 0);\0"
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
            b"Failed to prepare statement for update_detections_recording_id: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, recording_id as sqlite3_int64);
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, recording_id as sqlite3_int64);
    sqlite3_bind_text(
        stmt,
        3 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, since_time as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update detections with recording_id: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut updated: ::core::ffi::c_int = sqlite3_changes(db);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    if updated > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Updated %d detections with recording_id %lu for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            updated,
            recording_id as ::core::ffi::c_ulong,
            stream_name,
        );
    }
    return updated;
}
pub const MAX_DETECTION_LABELS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
