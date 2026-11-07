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
    unsafe fn sscanf(
        __s: *const ::core::ffi::c_char,
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
    unsafe fn sqlite3_free(_: *mut ::core::ffi::c_void);
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
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_double(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    unsafe fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
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
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zone_point_t {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_zone_t {
    pub id: [::core::ffi::c_char; 64],
    pub stream_name: [::core::ffi::c_char; 256],
    pub name: [::core::ffi::c_char; 64],
    pub enabled: bool,
    pub color: [::core::ffi::c_char; 8],
    pub polygon: [zone_point_t; 32],
    pub polygon_count: ::core::ffi::c_int,
    pub filter_classes: [::core::ffi::c_char; 256],
    pub min_confidence: ::core::ffi::c_float,
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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAX_ZONE_NAME: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MAX_ZONE_ID: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MIN_ZONE_POINTS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAX_ZONE_POINTS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MAX_ZONES_PER_STREAM: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn polygon_to_json(
    mut polygon: *const zone_point_t,
    mut count: ::core::ffi::c_int,
    mut json_out: *mut ::core::ffi::c_char,
    mut max_len: size_t,
) -> ::core::ffi::c_int {
    if polygon.is_null()
        || count <= 0 as ::core::ffi::c_int
        || json_out.is_null()
        || max_len == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut offset: size_t = 0 as size_t;
    let mut written: ::core::ffi::c_int = 0;
    written = snprintf(
        json_out.offset(offset as isize),
        max_len.wrapping_sub(offset),
        b"[\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if written < 0 as ::core::ffi::c_int || written as size_t >= max_len.wrapping_sub(offset) {
        return -1 as ::core::ffi::c_int;
    }
    offset = offset.wrapping_add(written as size_t);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if i > 0 as ::core::ffi::c_int {
            written = snprintf(
                json_out.offset(offset as isize),
                max_len.wrapping_sub(offset),
                b",\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if written < 0 as ::core::ffi::c_int
                || written as size_t >= max_len.wrapping_sub(offset)
            {
                return -1 as ::core::ffi::c_int;
            }
            offset = offset.wrapping_add(written as size_t);
        }
        written = snprintf(
            json_out.offset(offset as isize),
            max_len.wrapping_sub(offset),
            b"{\"x\":%.6f,\"y\":%.6f}\0".as_ptr() as *const ::core::ffi::c_char,
            (*polygon.offset(i as isize)).x as ::core::ffi::c_double,
            (*polygon.offset(i as isize)).y as ::core::ffi::c_double,
        );
        if written < 0 as ::core::ffi::c_int || written as size_t >= max_len.wrapping_sub(offset) {
            return -1 as ::core::ffi::c_int;
        }
        offset = offset.wrapping_add(written as size_t);
        i += 1;
    }
    written = snprintf(
        json_out.offset(offset as isize),
        max_len.wrapping_sub(offset),
        b"]\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if written < 0 as ::core::ffi::c_int || written as size_t >= max_len.wrapping_sub(offset) {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_to_polygon(
    mut json: *const ::core::ffi::c_char,
    mut polygon: *mut zone_point_t,
    mut count: *mut ::core::ffi::c_int,
    mut max_points: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if json.is_null() || polygon.is_null() || count.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *count = 0 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = json;
    while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != '[' as ::core::ffi::c_int {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int != '[' as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    p = p.offset(1);
    while *p as ::core::ffi::c_int != 0 && *count < max_points {
        while *p as ::core::ffi::c_int != 0
            && (*p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\r' as ::core::ffi::c_int)
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == ']' as ::core::ffi::c_int {
            break;
        }
        if *p as ::core::ffi::c_int != '{' as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        let mut x: ::core::ffi::c_float = 0.;
        let mut y: ::core::ffi::c_float = 0.;
        if sscanf(
            p,
            b"{\"x\":%f,\"y\":%f}\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut x,
            &raw mut y,
        ) == 2 as ::core::ffi::c_int
        {
            (*polygon.offset(*count as isize)).x = x;
            (*polygon.offset(*count as isize)).y = y;
            *count += 1;
            while *p as ::core::ffi::c_int != 0
                && *p as ::core::ffi::c_int != '}' as ::core::ffi::c_int
            {
                p = p.offset(1);
            }
            if *p as ::core::ffi::c_int == '}' as ::core::ffi::c_int {
                p = p.offset(1);
            }
        } else {
            return -1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detection_zone_point_count_valid(
    mut point_count: ::core::ffi::c_int,
) -> bool {
    return point_count >= MIN_ZONE_POINTS && point_count <= MAX_ZONE_POINTS;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn save_detection_zones(
    mut stream_name: *const ::core::ffi::c_char,
    mut zones: *const detection_zone_t,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stream_name.is_null()
        || zones.is_null()
        || count < 0 as ::core::ffi::c_int
        || count > MAX_ZONES_PER_STREAM
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for save_detection_zones (count=%d, max=%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            count,
            16 as ::core::ffi::c_int,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if !detection_zone_point_count_valid((*zones.offset(i as isize)).polygon_count) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Invalid polygon point count: %d\0".as_ptr() as *const ::core::ffi::c_char,
                (*zones.offset(i as isize)).polygon_count,
            );
            return -1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut err_msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if sqlite3_exec(
        db,
        b"BEGIN TRANSACTION;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        &raw mut err_msg,
    ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to begin transaction: %s\0".as_ptr() as *const ::core::ffi::c_char,
            err_msg,
        );
        sqlite3_free(err_msg as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut delete_sql: *const ::core::ffi::c_char =
        b"DELETE FROM detection_zones WHERE stream_name = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        delete_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare delete statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
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
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to delete existing zones: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut insert_sql: *const ::core::ffi::c_char = b"INSERT INTO detection_zones (id, stream_name, name, enabled, color, polygon, filter_classes, min_confidence, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < count {
        let mut zone: *const detection_zone_t = zones.offset(i_0 as isize);
        rc = sqlite3_prepare_v2(
            db,
            insert_sql,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to prepare insert statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            return -1 as ::core::ffi::c_int;
        }
        let mut polygon_json: [::core::ffi::c_char; 2048] = [0; 2048];
        if polygon_to_json(
            &raw const (*zone).polygon as *const zone_point_t,
            (*zone).polygon_count,
            &raw mut polygon_json as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to convert polygon to JSON for zone %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw const (*zone).id as *const ::core::ffi::c_char,
            );
            sqlite3_finalize(stmt);
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw const (*zone).id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw const (*zone).name as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int(
            stmt,
            4 as ::core::ffi::c_int,
            if (*zone).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            stmt,
            5 as ::core::ffi::c_int,
            &raw const (*zone).color as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            6 as ::core::ffi::c_int,
            &raw mut polygon_json as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            7 as ::core::ffi::c_int,
            &raw const (*zone).filter_classes as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_double(
            stmt,
            8 as ::core::ffi::c_int,
            (*zone).min_confidence as ::core::ffi::c_double,
        );
        sqlite3_bind_int64(stmt, 9 as ::core::ffi::c_int, now as sqlite3_int64);
        sqlite3_bind_int64(stmt, 10 as ::core::ffi::c_int, now as sqlite3_int64);
        rc = sqlite3_step(stmt);
        sqlite3_finalize(stmt);
        if rc != SQLITE_DONE {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to insert zone %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*zone).id as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            return -1 as ::core::ffi::c_int;
        }
        i_0 += 1;
    }
    if sqlite3_exec(
        db,
        b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        &raw mut err_msg,
    ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to commit transaction: %s\0".as_ptr() as *const ::core::ffi::c_char,
            err_msg,
        );
        sqlite3_free(err_msg as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Saved %d detection zones for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        count,
        stream_name,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_detection_zones(
    mut stream_name: *const ::core::ffi::c_char,
    mut zones: *mut detection_zone_t,
    mut max_zones: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stream_name.is_null() || zones.is_null() || max_zones <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_detection_zones\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT id, stream_name, name, enabled, color, polygon, filter_classes, min_confidence FROM detection_zones WHERE stream_name = ? ORDER BY created_at;\0"
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
            b"Failed to prepare select statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while sqlite3_step(stmt) == SQLITE_ROW && count < max_zones {
        let mut zone: *mut detection_zone_t = zones.offset(count as isize);
        safe_strcpy(
            &raw mut (*zone).id as *mut ::core::ffi::c_char,
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
            MAX_ZONE_ID as size_t,
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*zone).stream_name as *mut ::core::ffi::c_char,
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
            MAX_STREAM_NAME as size_t,
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*zone).name as *mut ::core::ffi::c_char,
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
            MAX_ZONE_NAME as size_t,
            0 as size_t,
        );
        (*zone).enabled =
            sqlite3_column_int(stmt, 3 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        safe_strcpy(
            &raw mut (*zone).color as *mut ::core::ffi::c_char,
            sqlite3_column_text(stmt, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
            8 as size_t,
            0 as size_t,
        );
        let mut polygon_json: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 5 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if json_to_polygon(
            polygon_json,
            &raw mut (*zone).polygon as *mut zone_point_t,
            &raw mut (*zone).polygon_count,
            MAX_ZONE_POINTS,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to parse polygon for zone %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*zone).id as *mut ::core::ffi::c_char,
            );
        } else {
            let mut filter_classes: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 6 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            if !filter_classes.is_null() {
                safe_strcpy(
                    &raw mut (*zone).filter_classes as *mut ::core::ffi::c_char,
                    filter_classes,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    0 as size_t,
                );
            } else {
                (*zone).filter_classes[0usize] = '\0' as ::core::ffi::c_char;
            }
            (*zone).min_confidence =
                sqlite3_column_double(stmt, 7 as ::core::ffi::c_int) as ::core::ffi::c_float;
            count += 1;
        }
    }
    sqlite3_finalize(stmt);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_detection_zones(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char =
        b"DELETE FROM detection_zones WHERE stream_name = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_detection_zone(
    mut zone_id: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if zone_id.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char =
        b"DELETE FROM detection_zones WHERE id = ?;\0".as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        zone_id,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_zone_enabled(
    mut zone_id: *const ::core::ffi::c_char,
    mut enabled: bool,
) -> ::core::ffi::c_int {
    if zone_id.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char =
        b"UPDATE detection_zones SET enabled = ?, updated_at = ? WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(
        stmt,
        1 as ::core::ffi::c_int,
        if enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int64(
        stmt,
        2 as ::core::ffi::c_int,
        time(::core::ptr::null_mut::<time_t>()) as sqlite3_int64,
    );
    sqlite3_bind_text(
        stmt,
        3 as ::core::ffi::c_int,
        zone_id,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
