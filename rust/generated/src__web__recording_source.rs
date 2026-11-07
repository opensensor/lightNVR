unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
    unsafe fn cJSON_AddBoolToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        boolean: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddNumberToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        number: ::core::ffi::c_double,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn storage_source_available(id: uint64_t) -> bool;
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
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub type uint64_t = u64;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_source_add_status(mut object: *mut cJSON, mut id: uint64_t) {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if object.is_null() || db.is_null() || mutex.is_null() {
        return;
    }
    let mut available: bool = storage_source_available(id);
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char = b"SELECT r.deletion_pending,COALESCE(t.target_type,'filesystem'),COALESCE(t.health_status,'unknown'),COALESCE(r.storage_target_uuid,''),1+(SELECT count(*) FROM storage_recording_copies c WHERE c.recording_id=r.id),EXISTS(SELECT 1 FROM storage_recording_copies c JOIN storage_targets a ON a.uuid=c.target_uuid WHERE c.recording_id=r.id AND a.target_type='s3'),EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=r.id AND j.state NOT IN('completed','failed','cancelled')),COALESCE((SELECT state FROM storage_retrieval_jobs WHERE recording_id=r.id),''),COALESCE(r.storage_policy_uuid,''),COALESCE(r.storage_policy_version,0),r.retention_override_days FROM recordings r LEFT JOIN storage_targets t ON t.uuid=r.storage_target_uuid WHERE r.id=?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        if sqlite3_step(statement) == SQLITE_ROW {
            let mut pending: bool =
                sqlite3_column_int(statement, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
            let mut r#type: *const ::core::ffi::c_char =
                sqlite3_column_text(statement, 1 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
            let mut retrieval: *const ::core::ffi::c_char =
                sqlite3_column_text(statement, 7 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
            let mut state: *const ::core::ffi::c_char = if pending as ::core::ffi::c_int != 0 {
                b"deletion_pending\0".as_ptr() as *const ::core::ffi::c_char
            } else if strcmp(
                retrieval,
                b"queued\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
                || strcmp(
                    retrieval,
                    b"fetching\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0
            {
                b"preparing\0".as_ptr() as *const ::core::ffi::c_char
            } else if !available {
                b"unavailable\0".as_ptr() as *const ::core::ffi::c_char
            } else if sqlite3_column_int(statement, 6 as ::core::ffi::c_int) != 0 {
                b"archiving\0".as_ptr() as *const ::core::ffi::c_char
            } else if strcmp(r#type, b"s3\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
                b"archived\0".as_ptr() as *const ::core::ffi::c_char
            } else if sqlite3_column_int(statement, 5 as ::core::ffi::c_int) != 0 {
                b"hot_and_archive\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"hot\0".as_ptr() as *const ::core::ffi::c_char
            };
            cJSON_AddStringToObject(
                object,
                b"storage_state\0".as_ptr() as *const ::core::ffi::c_char,
                state,
            );
            cJSON_AddBoolToObject(
                object,
                b"deletion_pending\0".as_ptr() as *const ::core::ffi::c_char,
                pending as cJSON_bool,
            );
            cJSON_AddBoolToObject(
                object,
                b"external_source\0".as_ptr() as *const ::core::ffi::c_char,
                (strcmp(r#type, b"s3\0".as_ptr() as *const ::core::ffi::c_char) == 0)
                    as ::core::ffi::c_int,
            );
            cJSON_AddStringToObject(
                object,
                b"storage_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_text(statement, 3 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                object,
                b"durable_copy_count\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_int(statement, 4 as ::core::ffi::c_int) as ::core::ffi::c_double,
            );
            cJSON_AddStringToObject(
                object,
                b"storage_policy_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_text(statement, 8 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                object,
                b"storage_policy_version\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_int64(statement, 9 as ::core::ffi::c_int) as ::core::ffi::c_double,
            );
            let mut retention: ::core::ffi::c_int =
                if sqlite3_column_type(statement, 10 as ::core::ffi::c_int) == SQLITE_NULL {
                    -1 as ::core::ffi::c_int
                } else {
                    sqlite3_column_int(statement, 10 as ::core::ffi::c_int)
                };
            cJSON_AddStringToObject(
                object,
                b"retention_mode\0".as_ptr() as *const ::core::ffi::c_char,
                if retention < 0 as ::core::ffi::c_int {
                    b"inherit\0".as_ptr() as *const ::core::ffi::c_char
                } else if retention == 0 as ::core::ffi::c_int {
                    b"indefinite\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"finite\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
