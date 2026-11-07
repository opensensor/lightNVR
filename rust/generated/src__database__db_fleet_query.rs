use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn difftime(__time1: time_t, __time0: time_t) -> ::core::ffi::c_double;
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
    unsafe fn sqlite3_busy_timeout(_: *mut sqlite3, ms: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
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
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn url_strip_credentials(
        url: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
    unsafe fn metrics_snapshot_all(
        out_array: *mut stream_metrics_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn metrics_get_max_streams() -> ::core::ffi::c_int;
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strpbrk(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
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
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::core::ffi::c_uint,
    pub __writers: ::core::ffi::c_uint,
    pub __wrphase_futex: ::core::ffi::c_uint,
    pub __writers_futex: ::core::ffi::c_uint,
    pub __pad3: ::core::ffi::c_uint,
    pub __pad4: ::core::ffi::c_uint,
    pub __cur_writer: ::core::ffi::c_int,
    pub __shared: ::core::ffi::c_int,
    pub __pad1: ::core::ffi::c_ulong,
    pub __pad2: ::core::ffi::c_ulong,
    pub __flags: ::core::ffi::c_uint,
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
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub type uint64_t = u64;
pub type int_fast64_t = ::core::ffi::c_long;
pub type uint_fast64_t = ::core::ffi::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_metrics_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub active: bool,
    pub health_status: atomic_int,
    pub stream_up: atomic_int,
    pub health_observed: bool,
    pub health_changed_at: time_t,
    pub last_packet_ts: atomic_int_fast64_t,
    pub last_frame_ts: atomic_int_fast64_t,
    pub current_fps: ::core::ffi::c_double,
    pub current_bitrate_bps: ::core::ffi::c_double,
    pub connection_latency_ms: ::core::ffi::c_double,
    pub configured_fps: ::core::ffi::c_double,
    pub frames_total: atomic_uint_fast64_t,
    pub frames_dropped: atomic_uint_fast64_t,
    pub bytes_received: atomic_uint_fast64_t,
    pub reconnects_total: atomic_uint_fast64_t,
    pub uptime_seconds: atomic_uint_fast64_t,
    pub error_decode: atomic_uint_fast64_t,
    pub error_timeout: atomic_uint_fast64_t,
    pub error_protocol: atomic_uint_fast64_t,
    pub error_io: atomic_uint_fast64_t,
    pub recording_active: atomic_int,
    pub recording_bytes_written: atomic_uint_fast64_t,
    pub recording_segments_total: atomic_uint_fast64_t,
    pub recording_gaps_total: atomic_uint_fast64_t,
    pub last_completed_segment_ts: atomic_int_fast64_t,
    pub ring: [metrics_ring_sample_t; 720],
    pub ring_head: ::core::ffi::c_int,
    pub ring_count: ::core::ffi::c_int,
    pub last_segment_end_time: time_t,
    pub expected_segment_duration: ::core::ffi::c_int,
    pub observation_persisted_at: time_t,
    pub stream_start_time: time_t,
    pub source_frames_total: [atomic_uint_fast64_t; 4],
    pub source_bytes_received: [atomic_uint_fast64_t; 4],
    pub prev_source_frames: [uint64_t; 4],
    pub prev_source_bytes: [uint64_t; 4],
    pub prev_sample_time: time_t,
    pub lock: pthread_rwlock_t,
}
pub type atomic_uint_fast64_t = uint_fast64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct metrics_ring_sample_t {
    pub fps: ::core::ffi::c_float,
    pub bitrate_kbps: ::core::ffi::c_float,
    pub timestamp: time_t,
}
pub type atomic_int_fast64_t = int_fast64_t;
pub type atomic_int = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_health_status_t(pub ::core::ffi::c_uint);
impl stream_health_status_t {
    pub const STREAM_HEALTH_UP: Self = Self(0);
    pub const STREAM_HEALTH_DEGRADED: Self = Self(1);
    pub const STREAM_HEALTH_DOWN: Self = Self(2);
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const FLEET_CAMERA_MAX_TAGS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const FLEET_CAMERA_MAX_LOCATION_DEPTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const STREAM_HEALTH_FRAME_TIMEOUT_DOWN: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
unsafe extern "C" fn copy_column(
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut stmt: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
) {
    let mut value: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, column) as *const ::core::ffi::c_char;
    safe_strcpy(
        destination,
        if !value.is_null() {
            value
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        destination_size,
        0 as size_t,
    );
}
unsafe extern "C" fn copy_safe_address(
    mut raw_url: *const ::core::ffi::c_char,
    mut address: *mut ::core::ffi::c_char,
    mut address_size: size_t,
) -> ::core::ffi::c_int {
    let mut stripped: [::core::ffi::c_char; 512] = [0; 512];
    if url_strip_credentials(
        raw_url,
        &raw mut stripped as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut scheme_end: *const ::core::ffi::c_char = strstr(
        &raw mut stripped as *mut ::core::ffi::c_char,
        b"://\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if scheme_end.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut authority: *const ::core::ffi::c_char =
        scheme_end.offset(3 as ::core::ffi::c_int as isize);
    let mut end: *const ::core::ffi::c_char =
        strpbrk(authority, b"/?#\0".as_ptr() as *const ::core::ffi::c_char);
    let mut length: size_t = if !end.is_null() {
        end.offset_from(&raw mut stripped as *mut ::core::ffi::c_char) as size_t
    } else {
        strlen(&raw mut stripped as *mut ::core::ffi::c_char)
    };
    if length == 0 as size_t || length >= address_size {
        return -1 as ::core::ffi::c_int;
    }
    memcpy(
        address as *mut ::core::ffi::c_void,
        &raw mut stripped as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        length,
    );
    *address.offset(length as isize) = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn load_location_ancestors(
    mut camera: *mut fleet_camera_t,
    mut ancestor_csv: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if ancestor_csv.is_null()
        || *ancestor_csv.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut copy: [::core::ffi::c_char; 1184] = [0; 1184];
    if strlen(ancestor_csv) >= ::core::mem::size_of::<[::core::ffi::c_char; 1184]>() {
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut copy as *mut ::core::ffi::c_char,
        ancestor_csv,
        ::core::mem::size_of::<[::core::ffi::c_char; 1184]>(),
        0 as size_t,
    );
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token: *mut ::core::ffi::c_char = strtok_r(
        &raw mut copy as *mut ::core::ffi::c_char,
        b",\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !token.is_null() {
        if (*camera).location_depth >= FLEET_CAMERA_MAX_LOCATION_DEPTH
            || strlen(token) != (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t
        {
            return -1 as ::core::ffi::c_int;
        }
        safe_strcpy(
            &raw mut *(&raw mut (*camera).location_ancestor_uuids as *mut [::core::ffi::c_char; 37])
                .offset((*camera).location_depth as isize) as *mut ::core::ffi::c_char,
            token,
            CAMERA_UUID_STRING_SIZE as size_t,
            0 as size_t,
        );
        (*camera).location_depth += 1;
        token = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn load_fleet_cameras_on_connection(
    mut db: *mut sqlite3,
    mut stream_name: *const ::core::ffi::c_char,
    mut cameras: *mut *mut fleet_camera_t,
    mut count: *mut ::core::ffi::c_int,
    mut by_uuid: bool,
) -> ::core::ffi::c_int {
    if db.is_null() || cameras.is_null() || count.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
    *count = 0 as ::core::ffi::c_int;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count_sql: *const ::core::ffi::c_char = if !stream_name.is_null() {
        if by_uuid as ::core::ffi::c_int != 0 {
            b"SELECT count(*) FROM streams WHERE camera_uuid = ?;\0".as_ptr()
                as *const ::core::ffi::c_char
        } else {
            b"SELECT count(*) FROM streams WHERE name = ?;\0".as_ptr() as *const ::core::ffi::c_char
        }
    } else {
        b"SELECT count(*) FROM streams;\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        count_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK && !stream_name.is_null() {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
    let mut camera_count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW {
        camera_count = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if camera_count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to count fleet cameras: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    if camera_count == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut loaded: *mut fleet_camera_t = calloc(
        camera_count as size_t,
        ::core::mem::size_of::<fleet_camera_t>(),
    ) as *mut fleet_camera_t;
    if loaded.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut select_prefix: *const ::core::ffi::c_char = b"WITH RECURSIVE location_tree(uuid, parent_uuid, name, path, ancestors) AS ( SELECT uuid, parent_uuid, name, name, uuid FROM camera_locations  WHERE parent_uuid IS NULL  UNION ALL  SELECT child.uuid, child.parent_uuid, child.name,         parent.path || ' / ' || child.name,         parent.ancestors || ',' || child.uuid  FROM camera_locations child  JOIN location_tree parent ON child.parent_uuid = parent.uuid) SELECT s.camera_uuid, s.name, s.url, s.tags, s.enabled,        s.streaming_enabled, s.record,        s.detection_based_recording, s.is_onvif, s.ptz_enabled,        s.backchannel_enabled, s.location_uuid,        COALESCE(loc.name, ''), COALESCE(loc.path, ''),        COALESCE(loc.ancestors, ''),        COALESCE(observation.first_video_at, 0),        COALESCE(observation.last_video_at, 0),        COALESCE(observation.last_recording_at, 0),        COALESCE(tag.uuid, ''), COALESCE(tag.label, '') FROM streams s LEFT JOIN location_tree loc ON loc.uuid = s.location_uuid LEFT JOIN camera_observations observation        ON observation.camera_uuid = s.camera_uuid LEFT JOIN camera_tag_assignments assignment        ON assignment.camera_uuid = s.camera_uuid LEFT JOIN camera_tags tag ON tag.uuid = assignment.tag_uuid \0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut select_suffix: *const ::core::ffi::c_char =
        b"ORDER BY s.camera_uuid, tag.label COLLATE NOCASE;\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut sql: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        select_prefix,
        if !stream_name.is_null() {
            if by_uuid as ::core::ffi::c_int != 0 {
                b"WHERE s.camera_uuid = ? \0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"WHERE s.name = ? \0".as_ptr() as *const ::core::ffi::c_char
            }
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        select_suffix,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
    {
        free(loaded as *mut ::core::ffi::c_void);
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
            b"Failed to prepare fleet inventory query: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        free(loaded as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    if !stream_name.is_null() {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
    let mut loaded_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut camera: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    loop {
        rc = sqlite3_step(stmt);
        if rc != SQLITE_ROW {
            break;
        }
        let mut camera_uuid: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if camera_uuid.is_null() {
            rc = SQLITE_CORRUPT;
            break;
        } else {
            if camera.is_null()
                || strcmp(
                    &raw mut (*camera).camera_uuid as *mut ::core::ffi::c_char,
                    camera_uuid,
                ) != 0 as ::core::ffi::c_int
            {
                if loaded_count >= camera_count {
                    rc = SQLITE_TOOBIG;
                    break;
                } else {
                    let c2rust_fresh0 = loaded_count;
                    loaded_count += 1;
                    camera = loaded.offset(c2rust_fresh0 as isize);
                    copy_column(
                        &raw mut (*camera).camera_uuid as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                        stmt,
                        0 as ::core::ffi::c_int,
                    );
                    copy_column(
                        &raw mut (*camera).name as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                        stmt,
                        1 as ::core::ffi::c_int,
                    );
                    let mut raw_url: [::core::ffi::c_char; 512] = [0; 512];
                    copy_column(
                        &raw mut raw_url as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                        stmt,
                        2 as ::core::ffi::c_int,
                    );
                    if copy_safe_address(
                        &raw mut raw_url as *mut ::core::ffi::c_char,
                        &raw mut (*camera).address as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    ) != 0 as ::core::ffi::c_int
                    {
                        (*camera).address[0usize] = '\0' as ::core::ffi::c_char;
                    }
                    copy_column(
                        &raw mut (*camera).legacy_tags as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                        stmt,
                        3 as ::core::ffi::c_int,
                    );
                    (*camera).enabled = sqlite3_column_int(stmt, 4 as ::core::ffi::c_int)
                        != 0 as ::core::ffi::c_int;
                    (*camera).streaming_enabled = sqlite3_column_int(stmt, 5 as ::core::ffi::c_int)
                        != 0 as ::core::ffi::c_int;
                    (*camera).record = sqlite3_column_int(stmt, 6 as ::core::ffi::c_int)
                        != 0 as ::core::ffi::c_int;
                    (*camera).detection_based_recording =
                        sqlite3_column_int(stmt, 7 as ::core::ffi::c_int)
                            != 0 as ::core::ffi::c_int;
                    (*camera).is_onvif = sqlite3_column_int(stmt, 8 as ::core::ffi::c_int)
                        != 0 as ::core::ffi::c_int;
                    (*camera).ptz_enabled = sqlite3_column_int(stmt, 9 as ::core::ffi::c_int)
                        != 0 as ::core::ffi::c_int;
                    (*camera).backchannel_enabled =
                        sqlite3_column_int(stmt, 10 as ::core::ffi::c_int)
                            != 0 as ::core::ffi::c_int;
                    copy_column(
                        &raw mut (*camera).location_uuid as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                        stmt,
                        11 as ::core::ffi::c_int,
                    );
                    copy_column(
                        &raw mut (*camera).location_name as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                        stmt,
                        12 as ::core::ffi::c_int,
                    );
                    copy_column(
                        &raw mut (*camera).location_path as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                        stmt,
                        13 as ::core::ffi::c_int,
                    );
                    let mut ancestors: *const ::core::ffi::c_char =
                        sqlite3_column_text(stmt, 14 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    if load_location_ancestors(camera, ancestors) != 0 as ::core::ffi::c_int {
                        rc = SQLITE_TOOBIG;
                        break;
                    } else {
                        (*camera).first_video_at =
                            sqlite3_column_int64(stmt, 15 as ::core::ffi::c_int) as int64_t;
                        (*camera).last_frame_ts =
                            sqlite3_column_int64(stmt, 16 as ::core::ffi::c_int) as int64_t;
                        (*camera).last_recording_at =
                            sqlite3_column_int64(stmt, 17 as ::core::ffi::c_int) as int64_t;
                        let mut administratively_disabled: bool =
                            !(*camera).enabled || !(*camera).streaming_enabled;
                        (*camera).health = fleet_health_state_t(
                            (if administratively_disabled as ::core::ffi::c_int != 0 {
                                fleet_health_state_t::FLEET_HEALTH_DISABLED.0 as ::core::ffi::c_int
                            } else {
                                fleet_health_state_t::FLEET_HEALTH_UNKNOWN.0 as ::core::ffi::c_int
                            }) as ::core::ffi::c_uint,
                        );
                        (*camera).availability = fleet_availability_state_t(
                            (if administratively_disabled as ::core::ffi::c_int != 0 {
                                fleet_availability_state_t::FLEET_AVAILABILITY_DISABLED.0
                                    as ::core::ffi::c_int
                            } else if (*camera).first_video_at > 0 as int64_t {
                                fleet_availability_state_t::FLEET_AVAILABILITY_OFFLINE.0
                                    as ::core::ffi::c_int
                            } else {
                                fleet_availability_state_t::FLEET_AVAILABILITY_NEVER_CONNECTED.0
                                    as ::core::ffi::c_int
                            }) as ::core::ffi::c_uint,
                        );
                    }
                }
            }
            let mut tag_uuid: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 18 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            if !(!tag_uuid.is_null()
                && *tag_uuid.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int)
            {
                continue;
            }
            if (*camera).tag_count >= FLEET_CAMERA_MAX_TAGS {
                rc = SQLITE_TOOBIG;
                break;
            } else {
                let c2rust_fresh1 = (*camera).tag_count;
                (*camera).tag_count += 1;
                let mut tag: *mut fleet_camera_tag_t = (&raw mut (*camera).tags
                    as *mut fleet_camera_tag_t)
                    .offset(c2rust_fresh1 as isize);
                copy_column(
                    &raw mut (*tag).uuid as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                    stmt,
                    18 as ::core::ffi::c_int,
                );
                copy_column(
                    &raw mut (*tag).label as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    stmt,
                    19 as ::core::ffi::c_int,
                );
            }
        }
    }
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE || loaded_count != camera_count {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to load fleet inventory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        free(loaded as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    *cameras = loaded;
    *count = loaded_count;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn load_fleet_cameras(
    mut stream_name: *const ::core::ffi::c_char,
    mut readonly: bool,
    mut cameras: *mut *mut fleet_camera_t,
    mut count: *mut ::core::ffi::c_int,
    mut by_uuid: bool,
) -> ::core::ffi::c_int {
    if cameras.is_null() || count.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
    *count = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    let mut mutex: *mut pthread_mutex_t = ::core::ptr::null_mut::<pthread_mutex_t>();
    if readonly {
        if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_busy_timeout(db, 1000 as ::core::ffi::c_int);
        if sqlite3_exec(
            db,
            b"BEGIN;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) != SQLITE_OK
        {
            db_close_readonly_connection(db);
            return -1 as ::core::ffi::c_int;
        }
    } else {
        db = get_db_handle();
        if db.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        mutex = get_db_mutex();
        pthread_mutex_lock(mutex);
    }
    let mut result: ::core::ffi::c_int =
        load_fleet_cameras_on_connection(db, stream_name, cameras, count, by_uuid);
    if !mutex.is_null() {
        pthread_mutex_unlock(mutex);
    } else {
        db_close_readonly_connection(db);
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_fleet_camera_load_on_connection(
    mut db: *mut sqlite3,
    mut cameras: *mut *mut fleet_camera_t,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return load_fleet_cameras_on_connection(
        db,
        ::core::ptr::null::<::core::ffi::c_char>(),
        cameras,
        count,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_fleet_camera_load(
    mut cameras: *mut *mut fleet_camera_t,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return load_fleet_cameras(
        ::core::ptr::null::<::core::ffi::c_char>(),
        r#false != 0,
        cameras,
        count,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_fleet_camera_load_readonly(
    mut cameras: *mut *mut fleet_camera_t,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return load_fleet_cameras(
        ::core::ptr::null::<::core::ffi::c_char>(),
        r#true != 0,
        cameras,
        count,
        r#false != 0,
    );
}
unsafe extern "C" fn find_camera_by_name(
    mut stream_name: *const ::core::ffi::c_char,
    mut readonly: bool,
    mut camera: *mut fleet_camera_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || camera.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut loaded: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if load_fleet_cameras(
        stream_name,
        readonly,
        &raw mut loaded,
        &raw mut count,
        r#false != 0,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if count == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if count != 1 as ::core::ffi::c_int {
        free(loaded as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    *camera = *loaded.offset(0isize);
    free(loaded as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_fleet_camera_find_by_name(
    mut stream_name: *const ::core::ffi::c_char,
    mut camera: *mut fleet_camera_t,
) -> ::core::ffi::c_int {
    return find_camera_by_name(stream_name, r#false != 0, camera);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_fleet_camera_find_by_name_readonly(
    mut stream_name: *const ::core::ffi::c_char,
    mut camera: *mut fleet_camera_t,
) -> ::core::ffi::c_int {
    return find_camera_by_name(stream_name, r#true != 0, camera);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_fleet_camera_find_by_uuid_readonly(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut camera: *mut fleet_camera_t,
) -> ::core::ffi::c_int {
    if camera_uuid.is_null() || *camera_uuid.offset(0isize) == 0 || camera.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut loaded: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if load_fleet_cameras(
        camera_uuid,
        r#true != 0,
        &raw mut loaded,
        &raw mut count,
        r#true != 0,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if count != 1 as ::core::ffi::c_int {
        free(loaded as *mut ::core::ffi::c_void);
        return if count == 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    *camera = *loaded.offset(0isize);
    free(loaded as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fleet_camera_capture_runtime_health(
    mut cameras: *mut fleet_camera_t,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if count == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if cameras.is_null() || count < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut maximum: ::core::ffi::c_int = metrics_get_max_streams();
    if maximum <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut metrics: *mut stream_metrics_t = calloc(
        maximum as size_t,
        ::core::mem::size_of::<stream_metrics_t>(),
    ) as *mut stream_metrics_t;
    if metrics.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut metric_count: ::core::ffi::c_int = metrics_snapshot_all(metrics, maximum);
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if metric_count < 0 as ::core::ffi::c_int || metric_count > maximum || now < 0 as time_t {
        free(metrics as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut table_size: size_t = 1 as size_t;
    while table_size < (metric_count * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t
    {
        table_size <<= 1 as ::core::ffi::c_int;
    }
    let mut metric_index: *mut ::core::ffi::c_int =
        malloc(table_size.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()))
            as *mut ::core::ffi::c_int;
    if metric_index.is_null() {
        free(metrics as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut i: size_t = 0 as size_t;
    while i < table_size {
        *metric_index.offset(i as isize) = -1 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < metric_count {
        let mut hash: uint64_t = 1469598103934665603 as uint64_t;
        let mut p: *const ::core::ffi::c_uchar =
            &raw mut (*metrics.offset(i_0 as isize)).stream_name as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_uchar;
        while *p != 0 {
            hash = ((hash ^ *p as uint64_t) as ::core::ffi::c_ulonglong)
                .wrapping_mul(1099511628211 as ::core::ffi::c_ulonglong)
                as uint64_t;
            p = p.offset(1);
        }
        let mut slot: size_t = hash as size_t & table_size.wrapping_sub(1 as size_t);
        while *metric_index.offset(slot as isize) >= 0 as ::core::ffi::c_int {
            slot = slot.wrapping_add(1 as size_t) & table_size.wrapping_sub(1 as size_t);
        }
        *metric_index.offset(slot as isize) = i_0;
        i_0 += 1;
    }
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < count {
        if !(*cameras.offset(i_1 as isize)).enabled
            || !(*cameras.offset(i_1 as isize)).streaming_enabled
        {
            (*cameras.offset(i_1 as isize)).health = fleet_health_state_t::FLEET_HEALTH_DISABLED;
            (*cameras.offset(i_1 as isize)).availability =
                fleet_availability_state_t::FLEET_AVAILABILITY_DISABLED;
        } else {
            let mut hash_0: uint64_t = 1469598103934665603 as uint64_t;
            let mut p_0: *const ::core::ffi::c_uchar = &raw mut (*cameras.offset(i_1 as isize)).name
                as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_uchar;
            while *p_0 != 0 {
                hash_0 = ((hash_0 ^ *p_0 as uint64_t) as ::core::ffi::c_ulonglong)
                    .wrapping_mul(1099511628211 as ::core::ffi::c_ulonglong)
                    as uint64_t;
                p_0 = p_0.offset(1);
            }
            let mut slot_0: size_t = hash_0 as size_t & table_size.wrapping_sub(1 as size_t);
            let mut probes: size_t = 0 as size_t;
            while probes < table_size {
                let mut j: ::core::ffi::c_int = *metric_index.offset(slot_0 as isize);
                if j < 0 as ::core::ffi::c_int {
                    break;
                }
                if strcmp(
                    &raw mut (*cameras.offset(i_1 as isize)).name as *mut ::core::ffi::c_char,
                    &raw mut (*metrics.offset(j as isize)).stream_name as *mut ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    slot_0 =
                        slot_0.wrapping_add(1 as size_t) & table_size.wrapping_sub(1 as size_t);
                    probes = probes.wrapping_add(1);
                } else {
                    match stream_health_status_t(
                        (*metrics.offset(j as isize)).health_status as ::core::ffi::c_uint,
                    ) {
                        stream_health_status_t::STREAM_HEALTH_UP => {
                            (*cameras.offset(i_1 as isize)).health =
                                fleet_health_state_t::FLEET_HEALTH_UP;
                        }
                        stream_health_status_t::STREAM_HEALTH_DEGRADED => {
                            (*cameras.offset(i_1 as isize)).health =
                                fleet_health_state_t::FLEET_HEALTH_DEGRADED;
                        }
                        stream_health_status_t::STREAM_HEALTH_DOWN => {
                            (*cameras.offset(i_1 as isize)).health =
                                fleet_health_state_t::FLEET_HEALTH_DOWN;
                        }
                        _ => {}
                    }
                    let mut runtime_last_frame: int64_t =
                        (*metrics.offset(j as isize)).last_frame_ts as int64_t;
                    if runtime_last_frame > (*cameras.offset(i_1 as isize)).last_frame_ts {
                        (*cameras.offset(i_1 as isize)).last_frame_ts = runtime_last_frame;
                    }
                    if (*cameras.offset(i_1 as isize)).first_video_at == 0 as int64_t
                        && runtime_last_frame > 0 as int64_t
                    {
                        (*cameras.offset(i_1 as isize)).first_video_at = runtime_last_frame;
                    }
                    (*cameras.offset(i_1 as isize)).health_changed_at =
                        (*metrics.offset(j as isize)).health_changed_at as int64_t;
                    (*cameras.offset(i_1 as isize)).current_fps =
                        (*metrics.offset(j as isize)).current_fps;
                    let mut expected_segment: ::core::ffi::c_int = if (*metrics.offset(j as isize))
                        .expected_segment_duration
                        > 0 as ::core::ffi::c_int
                    {
                        (*metrics.offset(j as isize)).expected_segment_duration
                    } else {
                        30 as ::core::ffi::c_int
                    };
                    let mut recorder_window: ::core::ffi::c_int =
                        expected_segment * 2 as ::core::ffi::c_int + 30 as ::core::ffi::c_int;
                    if recorder_window < 90 as ::core::ffi::c_int {
                        recorder_window = 90 as ::core::ffi::c_int;
                    }
                    let mut last_segment: time_t =
                        (*metrics.offset(j as isize)).last_completed_segment_ts;
                    (*cameras.offset(i_1 as isize)).recording_active = last_segment > 0 as time_t
                        && difftime(now, last_segment) <= recorder_window as ::core::ffi::c_double;
                    if runtime_last_frame > 0 as int64_t
                        && difftime(now, runtime_last_frame as time_t)
                            <= STREAM_HEALTH_FRAME_TIMEOUT_DOWN as ::core::ffi::c_double
                    {
                        (*cameras.offset(i_1 as isize)).availability =
                            fleet_availability_state_t::FLEET_AVAILABILITY_LIVE;
                    } else {
                        (*cameras.offset(i_1 as isize)).availability = fleet_availability_state_t(
                            (if (*cameras.offset(i_1 as isize)).first_video_at > 0 as int64_t {
                                fleet_availability_state_t::FLEET_AVAILABILITY_OFFLINE.0
                                    as ::core::ffi::c_int
                            } else {
                                fleet_availability_state_t::FLEET_AVAILABILITY_NEVER_CONNECTED.0
                                    as ::core::ffi::c_int
                            }) as ::core::ffi::c_uint,
                        );
                    }
                    break;
                }
            }
        }
        i_1 += 1;
    }
    free(metric_index as *mut ::core::ffi::c_void);
    free(metrics as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fleet_camera_enrich_runtime_health(
    mut cameras: *mut fleet_camera_t,
    mut count: ::core::ffi::c_int,
) {
    fleet_camera_capture_runtime_health(cameras, count);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
