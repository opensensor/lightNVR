use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_free(_: *mut ::core::ffi::c_void);
    unsafe fn sqlite3_extended_errcode(db: *mut sqlite3) -> ::core::ffi::c_int;
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
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_migration_job_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub recording_id: uint64_t,
    pub owner_user_id: int64_t,
    pub operation: [::core::ffi::c_char; 8],
    pub source_target_uuid: [::core::ffi::c_char; 37],
    pub source_object_key: [::core::ffi::c_char; 512],
    pub destination_target_uuid: [::core::ffi::c_char; 37],
    pub destination_object_key: [::core::ffi::c_char; 512],
    pub state: [::core::ffi::c_char; 24],
    pub checksum: [::core::ffi::c_char; 65],
    pub bytes_total: uint64_t,
    pub bytes_copied: uint64_t,
    pub attempt_count: ::core::ffi::c_int,
    pub max_attempts: ::core::ffi::c_int,
    pub next_attempt_at: int64_t,
    pub last_error: [::core::ffi::c_char; 256],
    pub cancel_requested: bool,
    pub bandwidth_limit_bps: uint64_t,
    pub window_start_minute: ::core::ffi::c_int,
    pub window_end_minute: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub started_at: int64_t,
    pub completed_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_migration_result_t(pub ::core::ffi::c_int);
impl db_storage_migration_result_t {
    pub const DB_STORAGE_MIGRATION_OK: Self = Self(0);
    pub const DB_STORAGE_MIGRATION_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_MIGRATION_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_MIGRATION_INVALID: Self = Self(-3);
    pub const DB_STORAGE_MIGRATION_SOURCE_INCOMPLETE: Self = Self(-4);
    pub const DB_STORAGE_MIGRATION_TARGET_UNAVAILABLE: Self = Self(-5);
    pub const DB_STORAGE_MIGRATION_SOURCE_CHANGED: Self = Self(-6);
    pub const DB_STORAGE_MIGRATION_ERROR: Self = Self(-7);
}
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const STORAGE_MIGRATION_MAX_VISIBLE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
unsafe extern "C" fn copy_column(
    mut destination: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut statement: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
) {
    let mut value: *const ::core::ffi::c_char =
        sqlite3_column_text(statement, column) as *const ::core::ffi::c_char;
    safe_strcpy(
        destination,
        if !value.is_null() {
            value
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        size,
        0 as size_t,
    );
}
unsafe extern "C" fn populate(
    mut statement: *mut sqlite3_stmt,
    mut job: *mut storage_migration_job_t,
) {
    memset(
        job as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_migration_job_t>(),
    );
    copy_column(
        &raw mut (*job).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    (*job).recording_id = sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as uint64_t;
    (*job).owner_user_id = sqlite3_column_int64(statement, 2 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*job).operation as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
        statement,
        3 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*job).source_target_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        4 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*job).source_object_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        5 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*job).destination_target_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        6 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*job).destination_object_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        7 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*job).state as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        statement,
        8 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*job).checksum as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
        statement,
        9 as ::core::ffi::c_int,
    );
    (*job).bytes_total = sqlite3_column_int64(statement, 10 as ::core::ffi::c_int) as uint64_t;
    (*job).bytes_copied = sqlite3_column_int64(statement, 11 as ::core::ffi::c_int) as uint64_t;
    (*job).attempt_count = sqlite3_column_int(statement, 12 as ::core::ffi::c_int);
    (*job).max_attempts = sqlite3_column_int(statement, 13 as ::core::ffi::c_int);
    (*job).next_attempt_at = sqlite3_column_int64(statement, 14 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*job).last_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        statement,
        15 as ::core::ffi::c_int,
    );
    (*job).cancel_requested =
        sqlite3_column_int(statement, 16 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*job).bandwidth_limit_bps =
        sqlite3_column_int64(statement, 17 as ::core::ffi::c_int) as uint64_t;
    (*job).window_start_minute = sqlite3_column_int(statement, 18 as ::core::ffi::c_int);
    (*job).window_end_minute = sqlite3_column_int(statement, 19 as ::core::ffi::c_int);
    (*job).revision = sqlite3_column_int64(statement, 20 as ::core::ffi::c_int) as int64_t;
    (*job).created_at = sqlite3_column_int64(statement, 21 as ::core::ffi::c_int) as int64_t;
    (*job).updated_at = sqlite3_column_int64(statement, 22 as ::core::ffi::c_int) as int64_t;
    (*job).started_at = sqlite3_column_int64(statement, 23 as ::core::ffi::c_int) as int64_t;
    (*job).completed_at = sqlite3_column_int64(statement, 24 as ::core::ffi::c_int) as int64_t;
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut job: *mut storage_migration_job_t,
) -> db_storage_migration_result_t {
    let mut sql: *const ::core::ffi::c_char = b"SELECT j.uuid,j.recording_id,COALESCE(j.owner_user_id,0),j.operation,j.source_target_uuid,j.source_object_key,j.destination_target_uuid,j.destination_object_key,j.state,j.checksum,j.bytes_total,j.bytes_copied,j.attempt_count,j.max_attempts,j.next_attempt_at,j.last_error,j.cancel_requested,j.bandwidth_limit_bps,j.window_start_minute,j.window_end_minute,j.revision,j.created_at,j.updated_at,COALESCE(j.started_at,0),COALESCE(j.completed_at,0) FROM storage_migration_jobs j WHERE j.uuid=? LIMIT 1;\0"
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
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut outcome: db_storage_migration_result_t =
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_NOT_FOUND;
    if result == SQLITE_ROW {
        populate(statement, job);
        outcome = db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK;
    } else if result != SQLITE_DONE {
        outcome = db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_create(
    mut recording_id: uint64_t,
    mut destination_target_uuid: *const ::core::ffi::c_char,
    mut owner_user_id: int64_t,
    mut job: *mut storage_migration_job_t,
) -> db_storage_migration_result_t {
    return db_storage_migration_create_operation(
        recording_id,
        destination_target_uuid,
        b"move\0".as_ptr() as *const ::core::ffi::c_char,
        owner_user_id,
        job,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_create_operation(
    mut recording_id: uint64_t,
    mut destination_target_uuid: *const ::core::ffi::c_char,
    mut operation: *const ::core::ffi::c_char,
    mut owner_user_id: int64_t,
    mut job: *mut storage_migration_job_t,
) -> db_storage_migration_result_t {
    if recording_id == 0 as uint64_t
        || owner_user_id < 0 as int64_t
        || job.is_null()
        || !lightnvr_uuid_is_valid(destination_target_uuid)
        || operation.is_null()
        || strcmp(operation, b"move\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
            && strcmp(operation, b"copy\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
    {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut recording_sql: *const ::core::ffi::c_char = b"SELECT is_complete,COALESCE(storage_target_uuid,''),COALESCE(object_key,''),MAX(COALESCE(size_bytes,0),0) FROM recordings WHERE id=? AND deletion_pending=0 LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        recording_sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_int64(
            statement,
            1 as ::core::ffi::c_int,
            recording_id as sqlite3_int64,
        );
        result = sqlite3_step(statement);
    }
    if result != SQLITE_ROW {
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        pthread_mutex_unlock(mutex);
        return db_storage_migration_result_t(if result == SQLITE_DONE {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_NOT_FOUND.0
        } else {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0
        });
    }
    let mut complete: bool =
        sqlite3_column_int(statement, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    let mut source_target: [::core::ffi::c_char; 37] = [0; 37];
    let mut source_key: [::core::ffi::c_char; 512] = [0; 512];
    let mut source_target_value: *const ::core::ffi::c_char =
        sqlite3_column_text(statement, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    let mut source_key_value: *const ::core::ffi::c_char =
        sqlite3_column_text(statement, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    safe_strcpy(
        &raw mut source_target as *mut ::core::ffi::c_char,
        if !source_target_value.is_null() {
            source_target_value
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut source_key as *mut ::core::ffi::c_char,
        if !source_key_value.is_null() {
            source_key_value
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut bytes_total: uint64_t =
        sqlite3_column_int64(statement, 3 as ::core::ffi::c_int) as uint64_t;
    sqlite3_finalize(statement);
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if !complete
        || source_target[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || source_key[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        pthread_mutex_unlock(mutex);
        return db_storage_migration_result_t(if !complete {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_SOURCE_INCOMPLETE.0
        } else {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID.0
        });
    }
    if strcmp(
        &raw mut source_target as *mut ::core::ffi::c_char,
        destination_target_uuid,
    ) == 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(mutex);
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_CONFLICT;
    }
    let mut existing_destination_key: [::core::ffi::c_char; 512] = [
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
    result = sqlite3_prepare_v2(
        db,
        b"SELECT object_key FROM storage_recording_copies WHERE recording_id=? AND target_uuid=? LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_int64(
            statement,
            1 as ::core::ffi::c_int,
            recording_id as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            destination_target_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
        if result == SQLITE_ROW {
            safe_strcpy(
                &raw mut existing_destination_key as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if result == SQLITE_ROW
        && strcmp(operation, b"copy\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(mutex);
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_CONFLICT;
    }
    if result != SQLITE_DONE && result != SQLITE_ROW {
        pthread_mutex_unlock(mutex);
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    let mut target_sql: *const ::core::ffi::c_char = b"SELECT enabled,health_status,root_path,migration_bandwidth_bps,archival_window_start_minute,archival_window_end_minute,target_type,archive_budget_bytes,recording_bytes+replica_bytes+COALESCE((SELECT sum(j.bytes_total) FROM storage_migration_jobs j WHERE j.destination_target_uuid=storage_targets.uuid AND j.state NOT IN('completed','cleanup_pending') AND j.artifacts_cleaned=0 AND NOT EXISTS(SELECT 1 FROM storage_recording_copies c WHERE c.recording_id=j.recording_id AND c.target_uuid=j.destination_target_uuid)),0) FROM storage_targets WHERE uuid=? LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    result = sqlite3_prepare_v2(
        db,
        target_sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            destination_target_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    if result != SQLITE_ROW {
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        pthread_mutex_unlock(mutex);
        return db_storage_migration_result_t(if result == SQLITE_DONE {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_NOT_FOUND.0
        } else {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0
        });
    }
    let mut enabled: bool =
        sqlite3_column_int(statement, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    let mut health: *const ::core::ffi::c_char =
        sqlite3_column_text(statement, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    let mut root_value: *const ::core::ffi::c_char =
        sqlite3_column_text(statement, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    let mut health_status: [::core::ffi::c_char; 16] = [0; 16];
    let mut destination_root: [::core::ffi::c_char; 512] = [0; 512];
    let mut bandwidth_limit_bps: uint64_t =
        sqlite3_column_int64(statement, 3 as ::core::ffi::c_int) as uint64_t;
    let mut window_start_minute: ::core::ffi::c_int =
        sqlite3_column_int(statement, 4 as ::core::ffi::c_int);
    let mut window_end_minute: ::core::ffi::c_int =
        sqlite3_column_int(statement, 5 as ::core::ffi::c_int);
    let mut object_destination: bool = strcmp(
        sqlite3_column_text(statement, 6 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
        b"s3\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut budget: uint64_t = sqlite3_column_int64(statement, 7 as ::core::ffi::c_int) as uint64_t;
    let mut occupied: uint64_t =
        sqlite3_column_int64(statement, 8 as ::core::ffi::c_int) as uint64_t;
    safe_strcpy(
        &raw mut health_status as *mut ::core::ffi::c_char,
        if !health.is_null() {
            health
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut destination_root as *mut ::core::ffi::c_char,
        if !root_value.is_null() {
            root_value
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    sqlite3_finalize(statement);
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if object_destination as ::core::ffi::c_int != 0
        && existing_destination_key[0usize] == 0
        && budget != 0
        && (occupied > budget || bytes_total > budget.wrapping_sub(occupied))
    {
        pthread_mutex_unlock(mutex);
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_TARGET_UNAVAILABLE;
    }
    if !enabled
        || health_status[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strcmp(
            &raw mut health_status as *mut ::core::ffi::c_char,
            b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw mut health_status as *mut ::core::ffi::c_char,
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(mutex);
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_TARGET_UNAVAILABLE;
    }
    let mut destination_path: [::core::ffi::c_char; 512] = [0; 512];
    let mut destination_key: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut destination_key as *mut ::core::ffi::c_char,
        &raw mut source_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    if existing_destination_key[0usize] != 0 {
        safe_strcpy(
            &raw mut destination_key as *mut ::core::ffi::c_char,
            &raw mut existing_destination_key as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    } else if object_destination {
        snprintf(
            &raw mut destination_key as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"recordings/%llu/%s.mp4\0".as_ptr() as *const ::core::ffi::c_char,
            recording_id as ::core::ffi::c_ulonglong,
            &raw mut uuid as *mut ::core::ffi::c_char,
        );
    }
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut destination_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut destination_root as *mut ::core::ffi::c_char,
        &raw mut source_key as *mut ::core::ffi::c_char,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        || (written as size_t)
            .wrapping_add(strlen(
                b".migration-\0".as_ptr() as *const ::core::ffi::c_char
            ))
            .wrapping_add(strlen(&raw mut uuid as *mut ::core::ffi::c_char))
            .wrapping_add(strlen(b".part\0".as_ptr() as *const ::core::ffi::c_char))
            >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        pthread_mutex_unlock(mutex);
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut insert_sql: *const ::core::ffi::c_char = b"INSERT INTO storage_migration_jobs(uuid,recording_id,owner_user_id,operation,source_target_uuid,source_object_key,destination_target_uuid,destination_object_key,bytes_total,bandwidth_limit_bps,window_start_minute,window_end_minute) VALUES(?,?,?,?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    result = sqlite3_prepare_v2(
        db,
        insert_sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            2 as ::core::ffi::c_int,
            recording_id as sqlite3_int64,
        );
        if owner_user_id > 0 as int64_t {
            sqlite3_bind_int64(
                statement,
                3 as ::core::ffi::c_int,
                owner_user_id as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(statement, 3 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            operation,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            &raw mut source_target as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            &raw mut source_key as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            7 as ::core::ffi::c_int,
            destination_target_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            &raw mut destination_key as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            9 as ::core::ffi::c_int,
            bytes_total as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            10 as ::core::ffi::c_int,
            bandwidth_limit_bps as sqlite3_int64,
        );
        sqlite3_bind_int(statement, 11 as ::core::ffi::c_int, window_start_minute);
        sqlite3_bind_int(statement, 12 as ::core::ffi::c_int, window_end_minute);
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result != SQLITE_DONE {
        pthread_mutex_unlock(mutex);
        return db_storage_migration_result_t(if result == SQLITE_CONSTRAINT {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_CONFLICT.0
        } else {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0
        });
    }
    let mut outcome: db_storage_migration_result_t =
        get_locked(db, &raw mut uuid as *mut ::core::ffi::c_char, job);
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_get(
    mut uuid: *const ::core::ffi::c_char,
    mut job: *mut storage_migration_job_t,
) -> db_storage_migration_result_t {
    if !lightnvr_uuid_is_valid(uuid) || job.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_storage_migration_result_t = get_locked(db, uuid, job);
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_list(
    mut jobs: *mut storage_migration_job_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if jobs.is_null()
        || max_count < 1 as ::core::ffi::c_int
        || max_count > STORAGE_MIGRATION_MAX_VISIBLE
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT j.uuid,j.recording_id,COALESCE(j.owner_user_id,0),j.operation,j.source_target_uuid,j.source_object_key,j.destination_target_uuid,j.destination_object_key,j.state,j.checksum,j.bytes_total,j.bytes_copied,j.attempt_count,j.max_attempts,j.next_attempt_at,j.last_error,j.cancel_requested,j.bandwidth_limit_bps,j.window_start_minute,j.window_end_minute,j.revision,j.created_at,j.updated_at,COALESCE(j.started_at,0),COALESCE(j.completed_at,0) FROM storage_migration_jobs j ORDER BY j.created_at DESC,j.uuid LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
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
        sqlite3_bind_int(statement, 1 as ::core::ffi::c_int, max_count);
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (result == SQLITE_OK || result == SQLITE_ROW) && count < max_count {
        result = sqlite3_step(statement);
        if result != SQLITE_ROW {
            break;
        }
        let c2rust_fresh0 = count;
        count += 1;
        populate(statement, jobs.offset(c2rust_fresh0 as isize));
    }
    if result != SQLITE_DONE && result != SQLITE_ROW {
        count = -1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_claim_due(
    mut job: *mut storage_migration_job_t,
) -> ::core::ffi::c_int {
    if job.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut select_sql: *const ::core::ffi::c_char = b"SELECT j.uuid,j.recording_id,COALESCE(j.owner_user_id,0),j.operation,j.source_target_uuid,j.source_object_key,j.destination_target_uuid,j.destination_object_key,j.state,j.checksum,j.bytes_total,j.bytes_copied,j.attempt_count,j.max_attempts,j.next_attempt_at,j.last_error,j.cancel_requested,j.bandwidth_limit_bps,j.window_start_minute,j.window_end_minute,j.revision,j.created_at,j.updated_at,COALESCE(j.started_at,0),COALESCE(j.completed_at,0) FROM storage_migration_jobs j WHERE (j.state='cleanup_pending' AND j.next_attempt_at<=strftime('%s','now')) OR (j.cancel_requested=0 AND j.attempt_count<j.max_attempts AND (j.state IN('queued','copying','verifying','committing') OR (j.state='retry_wait' AND j.next_attempt_at<=strftime('%s','now'))) AND (j.window_start_minute=j.window_end_minute OR (j.window_start_minute<j.window_end_minute AND  (CAST(strftime('%H','now','localtime') AS INTEGER)*60+  CAST(strftime('%M','now','localtime') AS INTEGER))>=j.window_start_minute AND  (CAST(strftime('%H','now','localtime') AS INTEGER)*60+  CAST(strftime('%M','now','localtime') AS INTEGER))<j.window_end_minute) OR (j.window_start_minute>j.window_end_minute AND ( (CAST(strftime('%H','now','localtime') AS INTEGER)*60+  CAST(strftime('%M','now','localtime') AS INTEGER))>=j.window_start_minute OR  (CAST(strftime('%H','now','localtime') AS INTEGER)*60+  CAST(strftime('%M','now','localtime') AS INTEGER))<j.window_end_minute)))) ORDER BY CASE WHEN j.state='cleanup_pending' THEN 0 ELSE 1 END,j.created_at,j.uuid LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        select_sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        result = sqlite3_step(statement);
    }
    if result == SQLITE_DONE {
        sqlite3_finalize(statement);
        pthread_mutex_unlock(mutex);
        return 0 as ::core::ffi::c_int;
    }
    if result != SQLITE_ROW {
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    populate(statement, job);
    sqlite3_finalize(statement);
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if strcmp(
        &raw mut (*job).state as *mut ::core::ffi::c_char,
        b"cleanup_pending\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        let mut safe_update_sql: *const ::core::ffi::c_char = b"UPDATE storage_migration_jobs SET state='copying',attempt_count=attempt_count+1,bytes_copied=0,last_error='',next_attempt_at=0,started_at=COALESCE(started_at,strftime('%s','now')),updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        result = sqlite3_prepare_v2(
            db,
            safe_update_sql,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                &raw mut (*job).uuid as *mut ::core::ffi::c_char,
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
        if result != SQLITE_DONE
            || get_locked(db, &raw mut (*job).uuid as *mut ::core::ffi::c_char, job).0
                != db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
        {
            pthread_mutex_unlock(mutex);
            return -1 as ::core::ffi::c_int;
        }
    }
    pthread_mutex_unlock(mutex);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn valid_worker_state(mut state: *const ::core::ffi::c_char) -> bool {
    return !state.is_null()
        && (strcmp(state, b"copying\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(state, b"verifying\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(
                state,
                b"committing\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_update_progress(
    mut uuid: *const ::core::ffi::c_char,
    mut state: *const ::core::ffi::c_char,
    mut bytes_copied: uint64_t,
    mut bytes_total: uint64_t,
) -> db_storage_migration_result_t {
    if !lightnvr_uuid_is_valid(uuid)
        || !valid_worker_state(state)
        || bytes_copied > bytes_total
        || bytes_total > INT64_MAX as uint64_t
    {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE storage_migration_jobs SET state=?,bytes_copied=?,bytes_total=?,updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=? AND state NOT IN('completed','failed','cancelled');\0"
        .as_ptr() as *const ::core::ffi::c_char;
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
            state,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            2 as ::core::ffi::c_int,
            bytes_copied as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            3 as ::core::ffi::c_int,
            bytes_total as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            uuid,
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
    return db_storage_migration_result_t(
        if result == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
        } else {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0
        },
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_defer_source(
    mut uuid: *const ::core::ffi::c_char,
) -> db_storage_migration_result_t {
    if !lightnvr_uuid_is_valid(uuid) {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"UPDATE storage_migration_jobs SET state='retry_wait',attempt_count=MAX(attempt_count-1,0),last_error='Waiting for verified archive retrieval',next_attempt_at=strftime('%s','now')+30,updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=? AND state='copying';\0"
            .as_ptr() as *const ::core::ffi::c_char,
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
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return db_storage_migration_result_t(if changed == 1 as ::core::ffi::c_int {
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
    } else {
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_record_failure(
    mut job: *const storage_migration_job_t,
    mut error: *const ::core::ffi::c_char,
    mut retryable: bool,
) -> db_storage_migration_result_t {
    if job.is_null()
        || !lightnvr_uuid_is_valid(&raw const (*job).uuid as *const ::core::ffi::c_char)
        || error.is_null()
    {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    let mut retry: bool =
        retryable as ::core::ffi::c_int != 0 && (*job).attempt_count < (*job).max_attempts;
    let mut shift: ::core::ffi::c_int = if (*job).attempt_count > 8 as ::core::ffi::c_int {
        8 as ::core::ffi::c_int
    } else {
        (*job).attempt_count
    };
    let mut backoff: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << shift;
    let mut normalized_error: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut normalized_error as *mut ::core::ffi::c_char,
        error,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    let mut sql: *const ::core::ffi::c_char = if retry as ::core::ffi::c_int != 0 {
        b"UPDATE storage_migration_jobs SET state='retry_wait',last_error=?,next_attempt_at=strftime('%s','now')+?,updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char
    } else {
        b"UPDATE storage_migration_jobs SET state='failed',last_error=?,next_attempt_at=0,completed_at=strftime('%s','now'),updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char
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
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut normalized_error as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        let mut parameter: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
        if retry {
            let c2rust_fresh1 = parameter;
            parameter += 1;
            sqlite3_bind_int(statement, c2rust_fresh1, backoff);
        }
        sqlite3_bind_text(
            statement,
            parameter,
            &raw const (*job).uuid as *const ::core::ffi::c_char,
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
    pthread_mutex_unlock(mutex);
    return db_storage_migration_result_t(if result == SQLITE_DONE {
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
    } else {
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_commit_location(
    mut job: *const storage_migration_job_t,
    mut destination_path: *const ::core::ffi::c_char,
    mut checksum: *const ::core::ffi::c_char,
) -> db_storage_migration_result_t {
    if job.is_null()
        || !lightnvr_uuid_is_valid(&raw const (*job).uuid as *const ::core::ffi::c_char)
        || destination_path.is_null()
        || *destination_path.offset(0isize) as ::core::ffi::c_int != 0
            && *destination_path.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        || checksum.is_null()
        || strlen(checksum) != 64 as size_t
    {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut error: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut result: ::core::ffi::c_int = sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        &raw mut error,
    );
    if !error.is_null() {
        sqlite3_free(error as *mut ::core::ffi::c_void);
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if result == SQLITE_OK {
        let mut recording_sql: *const ::core::ffi::c_char = b"UPDATE recordings SET storage_target_uuid=?,object_key=?,file_path=?,archive_checksum=? WHERE id=? AND is_complete=1 AND deletion_pending=0 AND storage_target_uuid=? AND object_key=? AND COALESCE((SELECT required_copy_count FROM storage_recording_policies p WHERE p.recording_id=recordings.id),1)<=1+(SELECT count(*) FROM storage_recording_copies c WHERE c.recording_id=recordings.id AND c.target_uuid<>?1);\0"
            .as_ptr() as *const ::core::ffi::c_char;
        result = sqlite3_prepare_v2(
            db,
            recording_sql,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                &raw const (*job).destination_target_uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
                &raw const (*job).destination_object_key as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                3 as ::core::ffi::c_int,
                destination_path,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                4 as ::core::ffi::c_int,
                checksum,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_int64(
                statement,
                5 as ::core::ffi::c_int,
                (*job).recording_id as sqlite3_int64,
            );
            sqlite3_bind_text(
                statement,
                6 as ::core::ffi::c_int,
                &raw const (*job).source_target_uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                7 as ::core::ffi::c_int,
                &raw const (*job).source_object_key as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            result = sqlite3_step(statement);
        }
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if result == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
        result = sqlite3_prepare_v2(
            db,
            b"DELETE FROM storage_recording_copies WHERE recording_id=? AND target_uuid=? AND object_key=?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_int64(
                statement,
                1 as ::core::ffi::c_int,
                (*job).recording_id as sqlite3_int64,
            );
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
                &raw const (*job).destination_target_uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                3 as ::core::ffi::c_int,
                &raw const (*job).destination_object_key as *const ::core::ffi::c_char,
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
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if result == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
        let mut job_sql: *const ::core::ffi::c_char = b"UPDATE storage_migration_jobs SET state='cleanup_pending',checksum=?,bytes_copied=bytes_total,last_error='',updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=? AND state NOT IN('completed','failed','cancelled') AND cancel_requested=0;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        result = sqlite3_prepare_v2(
            db,
            job_sql,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                checksum,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
                &raw const (*job).uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            result = sqlite3_step(statement);
        }
        changed = if result == SQLITE_DONE {
            sqlite3_changes(db)
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    let mut committed: bool = result == SQLITE_DONE && changed == 1 as ::core::ffi::c_int;
    error = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if committed {
        result = sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            &raw mut error,
        );
        committed = result == SQLITE_OK;
    } else {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    if !error.is_null() {
        sqlite3_free(error as *mut ::core::ffi::c_void);
    }
    if !committed && result != SQLITE_OK {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    pthread_mutex_unlock(mutex);
    if committed {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK;
    }
    return db_storage_migration_result_t(if changed == 0 as ::core::ffi::c_int {
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_SOURCE_CHANGED.0
    } else {
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_complete_cleanup(
    mut uuid: *const ::core::ffi::c_char,
) -> db_storage_migration_result_t {
    if !lightnvr_uuid_is_valid(uuid) {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE storage_migration_jobs SET state='completed',last_error='',next_attempt_at=0,completed_at=strftime('%s','now'),updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=? AND state='cleanup_pending';\0"
        .as_ptr() as *const ::core::ffi::c_char;
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
            uuid,
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
    return db_storage_migration_result_t(
        if result == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
        } else {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0
        },
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_defer_cleanup(
    mut job: *const storage_migration_job_t,
    mut error: *const ::core::ffi::c_char,
) -> db_storage_migration_result_t {
    if job.is_null()
        || !lightnvr_uuid_is_valid(&raw const (*job).uuid as *const ::core::ffi::c_char)
        || error.is_null()
    {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    let mut shift: ::core::ffi::c_int = if (*job).attempt_count > 8 as ::core::ffi::c_int {
        8 as ::core::ffi::c_int
    } else {
        (*job).attempt_count
    };
    let mut backoff: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << shift;
    let mut normalized_error: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut normalized_error as *mut ::core::ffi::c_char,
        error,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    let mut sql: *const ::core::ffi::c_char = b"UPDATE storage_migration_jobs SET last_error=?,attempt_count=attempt_count+1,next_attempt_at=strftime('%s','now')+?,updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=? AND state='cleanup_pending';\0"
        .as_ptr() as *const ::core::ffi::c_char;
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
            &raw mut normalized_error as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(statement, 2 as ::core::ffi::c_int, backoff);
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw const (*job).uuid as *const ::core::ffi::c_char,
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
    return db_storage_migration_result_t(
        if result == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
        } else {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0
        },
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_commit_copy(
    mut job: *const storage_migration_job_t,
    mut checksum: *const ::core::ffi::c_char,
) -> db_storage_migration_result_t {
    if job.is_null()
        || strcmp(
            &raw const (*job).operation as *const ::core::ffi::c_char,
            b"copy\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(&raw const (*job).uuid as *const ::core::ffi::c_char)
        || checksum.is_null()
        || strlen(checksum) != 64 as size_t
        || (*job).bytes_total > INT64_MAX as uint64_t
    {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut copy_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if lightnvr_uuid_generate_v4(&raw mut copy_uuid as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut result: ::core::ffi::c_int = sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut source_matches: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if result == SQLITE_OK {
        let mut source_sql: *const ::core::ffi::c_char = b"SELECT count(*) FROM recordings WHERE id=? AND is_complete=1 AND deletion_pending=0 AND storage_target_uuid=? AND object_key=?;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        result = sqlite3_prepare_v2(
            db,
            source_sql,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_int64(
                statement,
                1 as ::core::ffi::c_int,
                (*job).recording_id as sqlite3_int64,
            );
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
                &raw const (*job).source_target_uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                3 as ::core::ffi::c_int,
                &raw const (*job).source_object_key as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            if sqlite3_step(statement) == SQLITE_ROW {
                source_matches = sqlite3_column_int(statement, 0 as ::core::ffi::c_int);
                result = SQLITE_OK;
            } else {
                result = SQLITE_ERROR;
            }
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if result == SQLITE_OK && source_matches == 1 as ::core::ffi::c_int {
        let mut copy_sql: *const ::core::ffi::c_char = b"INSERT INTO storage_recording_copies(uuid,recording_id,target_uuid,object_key,checksum,size_bytes) VALUES(?,?,?,?,?,?);\0"
            .as_ptr() as *const ::core::ffi::c_char;
        result = sqlite3_prepare_v2(
            db,
            copy_sql,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                &raw mut copy_uuid as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_int64(
                statement,
                2 as ::core::ffi::c_int,
                (*job).recording_id as sqlite3_int64,
            );
            sqlite3_bind_text(
                statement,
                3 as ::core::ffi::c_int,
                &raw const (*job).destination_target_uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                4 as ::core::ffi::c_int,
                &raw const (*job).destination_object_key as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                5 as ::core::ffi::c_int,
                checksum,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_int64(
                statement,
                6 as ::core::ffi::c_int,
                (*job).bytes_total as sqlite3_int64,
            );
            result = sqlite3_step(statement);
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if changed != 0 {
        let mut job_sql: *const ::core::ffi::c_char = b"UPDATE storage_migration_jobs SET state='completed',checksum=?,bytes_copied=bytes_total,last_error='',next_attempt_at=0,completed_at=strftime('%s','now'),updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=? AND state='committing' AND cancel_requested=0;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        result = sqlite3_prepare_v2(
            db,
            job_sql,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                checksum,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
                &raw const (*job).uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            result = sqlite3_step(statement);
        }
        changed = if result == SQLITE_DONE {
            sqlite3_changes(db)
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    let mut committed: bool = result == SQLITE_DONE && changed == 1 as ::core::ffi::c_int;
    if committed {
        committed = sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK;
    }
    if !committed {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    let mut extended: ::core::ffi::c_int = sqlite3_extended_errcode(db);
    pthread_mutex_unlock(mutex);
    if committed {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK;
    }
    if source_matches != 1 as ::core::ffi::c_int {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_SOURCE_CHANGED;
    }
    return db_storage_migration_result_t(if extended == SQLITE_CONSTRAINT {
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_CONFLICT.0
    } else {
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_cancel_requested(
    mut uuid: *const ::core::ffi::c_char,
) -> bool {
    if !lightnvr_uuid_is_valid(uuid) {
        return r#false != 0;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return r#false != 0;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut requested: bool = r#false != 0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT cancel_requested FROM storage_migration_jobs WHERE uuid=? LIMIT 1;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
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
        requested = sqlite3_step(statement) == SQLITE_ROW
            && sqlite3_column_int(statement, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return requested;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_request_cancel(
    mut uuid: *const ::core::ffi::c_char,
    mut job: *mut storage_migration_job_t,
) -> db_storage_migration_result_t {
    if !lightnvr_uuid_is_valid(uuid) || job.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut current: storage_migration_job_t = storage_migration_job_t {
        uuid: [0; 37],
        recording_id: 0,
        owner_user_id: 0,
        operation: [0; 8],
        source_target_uuid: [0; 37],
        source_object_key: [0; 512],
        destination_target_uuid: [0; 37],
        destination_object_key: [0; 512],
        state: [0; 24],
        checksum: [0; 65],
        bytes_total: 0,
        bytes_copied: 0,
        attempt_count: 0,
        max_attempts: 0,
        next_attempt_at: 0,
        last_error: [0; 256],
        cancel_requested: false,
        bandwidth_limit_bps: 0,
        window_start_minute: 0,
        window_end_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
        started_at: 0,
        completed_at: 0,
    };
    let mut outcome: db_storage_migration_result_t = get_locked(db, uuid, &raw mut current);
    if outcome.0 != db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0 {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    let mut immediately: bool = strcmp(
        &raw mut current.state as *mut ::core::ffi::c_char,
        b"queued\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw mut current.state as *mut ::core::ffi::c_char,
            b"retry_wait\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    let mut running: bool = strcmp(
        &raw mut current.state as *mut ::core::ffi::c_char,
        b"copying\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw mut current.state as *mut ::core::ffi::c_char,
            b"verifying\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw mut current.state as *mut ::core::ffi::c_char,
            b"committing\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    if !immediately && !running {
        pthread_mutex_unlock(mutex);
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_CONFLICT;
    }
    let mut sql: *const ::core::ffi::c_char = if immediately as ::core::ffi::c_int != 0 {
        b"UPDATE storage_migration_jobs SET state='cancelled',cancel_requested=1,last_error='Cancelled by operator',next_attempt_at=0,completed_at=strftime('%s','now'),updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char
    } else {
        b"UPDATE storage_migration_jobs SET cancel_requested=1,last_error='Cancellation requested by operator',updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char
    };
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
            uuid,
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
    outcome = db_storage_migration_result_t(
        if result == SQLITE_DONE && sqlite3_changes(db) == 1 as ::core::ffi::c_int {
            get_locked(db, uuid, job).0
        } else {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0
        },
    );
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_mark_cancelled(
    mut uuid: *const ::core::ffi::c_char,
    mut job: *mut storage_migration_job_t,
) -> db_storage_migration_result_t {
    if !lightnvr_uuid_is_valid(uuid) || job.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE storage_migration_jobs SET state='cancelled',cancel_requested=1,last_error='Cancelled by operator',next_attempt_at=0,completed_at=strftime('%s','now'),updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=? AND cancel_requested=1 AND state IN('copying','verifying','committing');\0"
        .as_ptr() as *const ::core::ffi::c_char;
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
            uuid,
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
    let mut outcome: db_storage_migration_result_t =
        db_storage_migration_result_t(if changed == 1 as ::core::ffi::c_int {
            get_locked(db, uuid, job).0
        } else {
            db_storage_migration_result_t::DB_STORAGE_MIGRATION_CONFLICT.0
        });
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_migration_retry(
    mut uuid: *const ::core::ffi::c_char,
    mut job: *mut storage_migration_job_t,
) -> db_storage_migration_result_t {
    if !lightnvr_uuid_is_valid(uuid) || job.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE storage_migration_jobs SET state='queued',cancel_requested=0,checksum='',bytes_copied=0,attempt_count=0,next_attempt_at=0,artifacts_cleaned=0,last_error='',started_at=NULL,completed_at=NULL,updated_at=strftime('%s','now'),revision=revision+1 WHERE uuid=? AND state IN('failed','cancelled') AND EXISTS(SELECT 1 FROM recordings r WHERE r.id=storage_migration_jobs.recording_id AND r.deletion_pending=0);\0"
        .as_ptr() as *const ::core::ffi::c_char;
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
            uuid,
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
    let mut outcome: db_storage_migration_result_t =
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK;
    if result == SQLITE_CONSTRAINT {
        outcome = db_storage_migration_result_t::DB_STORAGE_MIGRATION_CONFLICT;
    } else if changed == 1 as ::core::ffi::c_int {
        outcome = get_locked(db, uuid, job);
    } else {
        let mut current: storage_migration_job_t = storage_migration_job_t {
            uuid: [0; 37],
            recording_id: 0,
            owner_user_id: 0,
            operation: [0; 8],
            source_target_uuid: [0; 37],
            source_object_key: [0; 512],
            destination_target_uuid: [0; 37],
            destination_object_key: [0; 512],
            state: [0; 24],
            checksum: [0; 65],
            bytes_total: 0,
            bytes_copied: 0,
            attempt_count: 0,
            max_attempts: 0,
            next_attempt_at: 0,
            last_error: [0; 256],
            cancel_requested: false,
            bandwidth_limit_bps: 0,
            window_start_minute: 0,
            window_end_minute: 0,
            revision: 0,
            created_at: 0,
            updated_at: 0,
            started_at: 0,
            completed_at: 0,
        };
        outcome = db_storage_migration_result_t(
            if get_locked(db, uuid, &raw mut current).0
                == db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
            {
                db_storage_migration_result_t::DB_STORAGE_MIGRATION_CONFLICT.0
            } else {
                db_storage_migration_result_t::DB_STORAGE_MIGRATION_NOT_FOUND.0
            },
        );
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
