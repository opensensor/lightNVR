use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
pub struct camera_observation_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub first_video_at: int64_t,
    pub last_video_at: int64_t,
    pub last_recording_at: int64_t,
    pub updated_at: int64_t,
}
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_observation_record(
    mut stream_name: *const ::core::ffi::c_char,
    mut video_at: int64_t,
    mut recording_at: int64_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || mutex.is_null()
        || stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || video_at <= 0 as int64_t && recording_at <= 0 as int64_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO camera_observations(camera_uuid,first_video_at,last_video_at,last_recording_at) SELECT camera_uuid,?,?,? FROM streams WHERE name=? ON CONFLICT(camera_uuid) DO UPDATE SET first_video_at=CASE  WHEN camera_observations.first_video_at=0 AND excluded.first_video_at>0 THEN excluded.first_video_at  ELSE camera_observations.first_video_at END,last_video_at=MAX(camera_observations.last_video_at,excluded.last_video_at),last_recording_at=MAX(camera_observations.last_recording_at,excluded.last_recording_at),updated_at=strftime('%s','now');\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut first_video_at: int64_t = if video_at > 0 as int64_t {
        video_at
    } else {
        0 as int64_t
    };
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_int64(
            statement,
            1 as ::core::ffi::c_int,
            first_video_at as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            2 as ::core::ffi::c_int,
            (if video_at > 0 as int64_t {
                video_at
            } else {
                0 as int64_t
            }) as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            3 as ::core::ffi::c_int,
            (if recording_at > 0 as int64_t {
                recording_at
            } else {
                0 as int64_t
            }) as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_DONE && changed > 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_observation_get(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut observation: *mut camera_observation_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() || camera_uuid.is_null() || observation.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT camera_uuid,first_video_at,last_video_at,last_recording_at,updated_at FROM camera_observations WHERE camera_uuid=?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    memset(
        observation as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<camera_observation_t>(),
    );
    pthread_mutex_lock(mutex);
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
            camera_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    if result == SQLITE_ROW {
        let mut uuid: *const ::core::ffi::c_char =
            sqlite3_column_text(statement, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*observation).camera_uuid as *mut ::core::ffi::c_char,
            if !uuid.is_null() {
                uuid
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        (*observation).first_video_at =
            sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as int64_t;
        (*observation).last_video_at =
            sqlite3_column_int64(statement, 2 as ::core::ffi::c_int) as int64_t;
        (*observation).last_recording_at =
            sqlite3_column_int64(statement, 3 as ::core::ffi::c_int) as int64_t;
        (*observation).updated_at =
            sqlite3_column_int64(statement, 4 as ::core::ffi::c_int) as int64_t;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_ROW {
        0 as ::core::ffi::c_int
    } else if result == SQLITE_DONE {
        1 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
