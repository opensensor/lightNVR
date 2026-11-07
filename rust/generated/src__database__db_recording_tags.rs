use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
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
    unsafe fn copy_trimmed_value(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
        input: *const ::core::ffi::c_char,
        input_size: size_t,
    ) -> size_t;
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
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub struct recording_tag_list_t {
    pub count: ::core::ffi::c_int,
    pub tags: [[::core::ffi::c_char; 128]; 64],
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
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_LIMIT_VARIABLE_NUMBER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const MAX_RECORDING_TAGS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MAX_TAG_LENGTH: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const RECORDING_TAG_QUERY_BATCH_SIZE: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recording_tag_add(
    mut recording_id: uint64_t,
    mut tag: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if tag.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut trimmed: [::core::ffi::c_char; 128] = [0; 128];
    if copy_trimmed_value(
        &raw mut trimmed as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        tag,
        0 as size_t,
    ) == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mtx: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mtx);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char =
        b"INSERT OR IGNORE INTO recording_tags (recording_id, tag) VALUES (?, ?);\0".as_ptr()
            as *const ::core::ffi::c_char;
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
            b"Failed to prepare tag insert: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(mtx);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, recording_id as sqlite3_int64);
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw mut trimmed as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recording_tag_remove(
    mut recording_id: uint64_t,
    mut tag: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if tag.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut trimmed: [::core::ffi::c_char; 128] = [0; 128];
    if copy_trimmed_value(
        &raw mut trimmed as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        tag,
        0 as size_t,
    ) == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mtx: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mtx);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char =
        b"DELETE FROM recording_tags WHERE recording_id = ? AND tag = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
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
            b"Failed to prepare tag delete: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(mtx);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, recording_id as sqlite3_int64);
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw mut trimmed as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recording_tag_get(
    mut recording_id: uint64_t,
    mut tags: *mut [::core::ffi::c_char; 128],
    mut max_tags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mtx: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || tags.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mtx);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char =
        b"SELECT tag FROM recording_tags WHERE recording_id = ? ORDER BY tag;\0".as_ptr()
            as *const ::core::ffi::c_char;
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
            b"Failed to prepare tag select: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(mtx);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, recording_id as sqlite3_int64);
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while sqlite3_step(stmt) == SQLITE_ROW && count < max_tags {
        let mut t: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !t.is_null() {
            safe_strcpy(
                &raw mut *tags.offset(count as isize) as *mut ::core::ffi::c_char,
                t,
                MAX_TAG_LENGTH as size_t,
                0 as size_t,
            );
            count += 1;
        }
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recording_tag_get_batch(
    mut recording_ids: *const uint64_t,
    mut count: ::core::ffi::c_int,
    mut tag_lists: *mut recording_tag_list_t,
) -> ::core::ffi::c_int {
    if recording_ids.is_null() || count <= 0 as ::core::ffi::c_int || tag_lists.is_null() {
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
    memset(
        tag_lists as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (count as size_t).wrapping_mul(::core::mem::size_of::<recording_tag_list_t>()),
    );
    let mut total: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut batch_start: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while batch_start < count {
        let mut batch_count: ::core::ffi::c_int = count - batch_start;
        if batch_count > RECORDING_TAG_QUERY_BATCH_SIZE {
            batch_count = RECORDING_TAG_QUERY_BATCH_SIZE;
        }
        let mut sql: [::core::ffi::c_char; 2048] = [0; 2048];
        safe_strcpy(
            &raw mut sql as *mut ::core::ffi::c_char,
            b"SELECT recording_id, tag FROM recording_tags WHERE recording_id IN (\0".as_ptr()
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            0 as size_t,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < batch_count {
            if i > 0 as ::core::ffi::c_int {
                safe_strcat(
                    &raw mut sql as *mut ::core::ffi::c_char,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
                );
            }
            safe_strcat(
                &raw mut sql as *mut ::core::ffi::c_char,
                b"?\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            );
            i += 1;
        }
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b") ORDER BY recording_id, tag;\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
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
                b"Failed to prepare batch tag select: %s\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            db_close_readonly_connection(db);
            return -1 as ::core::ffi::c_int;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < batch_count {
            sqlite3_bind_int64(
                stmt,
                i_0 + 1 as ::core::ffi::c_int,
                *recording_ids.offset((batch_start + i_0) as isize) as sqlite3_int64,
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
            let mut tag: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            if tag.is_null() {
                continue;
            }
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < batch_count {
                let mut output_index: ::core::ffi::c_int = batch_start + i_1;
                if *recording_ids.offset(output_index as isize) != recording_id {
                    i_1 += 1;
                } else {
                    let mut list: *mut recording_tag_list_t =
                        tag_lists.offset(output_index as isize);
                    if (*list).count < MAX_RECORDING_TAGS {
                        safe_strcpy(
                            &raw mut *(&raw mut (*list).tags as *mut [::core::ffi::c_char; 128])
                                .offset((*list).count as isize)
                                as *mut ::core::ffi::c_char,
                            tag,
                            MAX_TAG_LENGTH as size_t,
                            0 as size_t,
                        );
                        (*list).count += 1;
                        total += 1;
                    }
                    break;
                }
            }
        }
        if rc != SQLITE_DONE {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to fetch batch recording tags: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            sqlite3_finalize(stmt);
            db_close_readonly_connection(db);
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_finalize(stmt);
        batch_start += RECORDING_TAG_QUERY_BATCH_SIZE;
    }
    db_close_readonly_connection(db);
    return total;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recording_tag_set(
    mut recording_id: uint64_t,
    mut tags: *mut *const ::core::ffi::c_char,
    mut tag_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mtx: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mtx);
    let mut del_stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM recording_tags WHERE recording_id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut del_stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare tag delete-all: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(mtx);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(
        del_stmt,
        1 as ::core::ffi::c_int,
        recording_id as sqlite3_int64,
    );
    sqlite3_step(del_stmt);
    sqlite3_finalize(del_stmt);
    if tag_count > 0 as ::core::ffi::c_int && !tags.is_null() {
        let mut ins_stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare_v2(
            db,
            b"INSERT OR IGNORE INTO recording_tags (recording_id, tag) VALUES (?, ?);\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut ins_stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to prepare tag insert: %s\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            pthread_mutex_unlock(mtx);
            return -1 as ::core::ffi::c_int;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < tag_count {
            if !(*tags.offset(i as isize)).is_null() {
                let mut trimmed: [::core::ffi::c_char; 128] = [0; 128];
                if copy_trimmed_value(
                    &raw mut trimmed as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                    *tags.offset(i as isize),
                    0 as size_t,
                ) != 0 as size_t
                {
                    sqlite3_reset(ins_stmt);
                    sqlite3_bind_int64(
                        ins_stmt,
                        1 as ::core::ffi::c_int,
                        recording_id as sqlite3_int64,
                    );
                    sqlite3_bind_text(
                        ins_stmt,
                        2 as ::core::ffi::c_int,
                        &raw mut trimmed as *mut ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                        SQLITE_STATIC,
                    );
                    sqlite3_step(ins_stmt);
                }
            }
            i += 1;
        }
        sqlite3_finalize(ins_stmt);
    }
    pthread_mutex_unlock(mtx);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recording_tag_get_all_unique(
    mut tags: *mut [::core::ffi::c_char; 128],
    mut max_tags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mtx: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || tags.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mtx);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char =
        b"SELECT DISTINCT tag FROM recording_tags ORDER BY tag;\0".as_ptr()
            as *const ::core::ffi::c_char;
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
            b"Failed to prepare unique tags select: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(mtx);
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while sqlite3_step(stmt) == SQLITE_ROW && count < max_tags {
        let mut t: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !t.is_null() {
            safe_strcpy(
                &raw mut *tags.offset(count as isize) as *mut ::core::ffi::c_char,
                t,
                MAX_TAG_LENGTH as size_t,
                0 as size_t,
            );
            count += 1;
        }
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recording_tag_get_unique_for_streams(
    mut stream_names: *const *const ::core::ffi::c_char,
    mut stream_count: ::core::ffi::c_int,
    mut tags: *mut [::core::ffi::c_char; 128],
    mut max_tags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return unique_tags_for_scope(stream_names, stream_count, tags, max_tags, r#false != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recording_tag_get_unique_for_cameras(
    mut camera_uuids: *const *const ::core::ffi::c_char,
    mut camera_count: ::core::ffi::c_int,
    mut tags: *mut [::core::ffi::c_char; 128],
    mut max_tags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return unique_tags_for_scope(camera_uuids, camera_count, tags, max_tags, r#true != 0);
}
unsafe extern "C" fn unique_tags_for_scope(
    mut stream_names: *const *const ::core::ffi::c_char,
    mut stream_count: ::core::ffi::c_int,
    mut tags: *mut [::core::ffi::c_char; 128],
    mut max_tags: ::core::ffi::c_int,
    mut camera_scope: bool,
) -> ::core::ffi::c_int {
    if stream_names.is_null()
        || stream_count <= 0 as ::core::ffi::c_int
        || tags.is_null()
        || max_tags <= 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut variable_limit: ::core::ffi::c_int =
        sqlite3_limit(db, SQLITE_LIMIT_VARIABLE_NUMBER, -1 as ::core::ffi::c_int);
    if variable_limit < 2 as ::core::ffi::c_int {
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    let mut batch_limit: ::core::ffi::c_int = variable_limit - 1 as ::core::ffi::c_int;
    if batch_limit > 256 as ::core::ffi::c_int {
        batch_limit = 256 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut final_rc: ::core::ffi::c_int = SQLITE_DONE;
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
            final_rc = SQLITE_NOMEM;
            break;
        } else {
            safe_strcpy(
                sql,
                if camera_scope as ::core::ffi::c_int != 0 {
                    b"SELECT DISTINCT rt.tag FROM recording_tags rt JOIN recordings r ON r.id=rt.recording_id WHERE COALESCE(NULLIF(r.camera_uuid,''),(SELECT camera_uuid FROM streams WHERE name=r.stream_name)) IN (\0"
                        .as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"SELECT DISTINCT rt.tag FROM recording_tags rt JOIN recordings r ON r.id=rt.recording_id WHERE r.stream_name IN (\0"
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
                b") ORDER BY rt.tag LIMIT ?;\0".as_ptr() as *const ::core::ffi::c_char,
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
            if rc != SQLITE_OK {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to prepare scoped recording tag query: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    sqlite3_errmsg(db),
                );
                if !stmt.is_null() {
                    sqlite3_finalize(stmt);
                }
                final_rc = rc;
                break;
            } else {
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
                sqlite3_bind_int(stmt, batch_count + 1 as ::core::ffi::c_int, max_tags);
                loop {
                    rc = sqlite3_step(stmt);
                    if rc != SQLITE_ROW {
                        break;
                    }
                    let mut tag: *const ::core::ffi::c_char =
                        sqlite3_column_text(stmt, 0 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    if tag.is_null() {
                        continue;
                    }
                    let mut insert_at: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while insert_at < count
                        && strcmp(
                            &raw mut *tags.offset(insert_at as isize) as *mut ::core::ffi::c_char,
                            tag,
                        ) < 0 as ::core::ffi::c_int
                    {
                        insert_at += 1;
                    }
                    if insert_at < count
                        && strcmp(
                            &raw mut *tags.offset(insert_at as isize) as *mut ::core::ffi::c_char,
                            tag,
                        ) == 0 as ::core::ffi::c_int
                    {
                        continue;
                    }
                    if insert_at < max_tags {
                        let mut move_count: ::core::ffi::c_int = if count < max_tags {
                            count - insert_at
                        } else {
                            max_tags - insert_at - 1 as ::core::ffi::c_int
                        };
                        if move_count > 0 as ::core::ffi::c_int {
                            memmove(
                                &raw mut *tags
                                    .offset((insert_at + 1 as ::core::ffi::c_int) as isize)
                                    as *mut ::core::ffi::c_char
                                    as *mut ::core::ffi::c_void,
                                &raw mut *tags.offset(insert_at as isize)
                                    as *mut ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                (move_count as size_t).wrapping_mul(MAX_TAG_LENGTH as size_t),
                            );
                        }
                        safe_strcpy(
                            &raw mut *tags.offset(insert_at as isize) as *mut ::core::ffi::c_char,
                            tag,
                            MAX_TAG_LENGTH as size_t,
                            0 as size_t,
                        );
                        if count < max_tags {
                            count += 1;
                        }
                    }
                }
                if !stmt.is_null() {
                    sqlite3_finalize(stmt);
                }
                if rc != SQLITE_DONE {
                    final_rc = rc;
                    break;
                } else {
                    offset += batch_limit;
                }
            }
        }
    }
    db_close_readonly_connection(db);
    return if final_rc == SQLITE_DONE {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recording_tag_batch_add(
    mut recording_ids: *const uint64_t,
    mut count: ::core::ffi::c_int,
    mut tag: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if recording_ids.is_null() || tag.is_null() || count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut trimmed: [::core::ffi::c_char; 128] = [0; 128];
    if copy_trimmed_value(
        &raw mut trimmed as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        tag,
        0 as size_t,
    ) == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mtx: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mtx);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char =
        b"INSERT OR IGNORE INTO recording_tags (recording_id, tag) VALUES (?, ?);\0".as_ptr()
            as *const ::core::ffi::c_char;
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
            b"Failed to prepare batch tag insert: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(mtx);
        return -1 as ::core::ffi::c_int;
    }
    let mut success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        sqlite3_reset(stmt);
        sqlite3_bind_int64(
            stmt,
            1 as ::core::ffi::c_int,
            *recording_ids.offset(i as isize) as sqlite3_int64,
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw mut trimmed as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        if sqlite3_step(stmt) == SQLITE_DONE {
            success += 1;
        }
        i += 1;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recording_tag_batch_remove(
    mut recording_ids: *const uint64_t,
    mut count: ::core::ffi::c_int,
    mut tag: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if recording_ids.is_null() || tag.is_null() || count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut trimmed: [::core::ffi::c_char; 128] = [0; 128];
    if copy_trimmed_value(
        &raw mut trimmed as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        tag,
        0 as size_t,
    ) == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mtx: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mtx);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char =
        b"DELETE FROM recording_tags WHERE recording_id = ? AND tag = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
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
            b"Failed to prepare batch tag delete: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(mtx);
        return -1 as ::core::ffi::c_int;
    }
    let mut success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        sqlite3_reset(stmt);
        sqlite3_bind_int64(
            stmt,
            1 as ::core::ffi::c_int,
            *recording_ids.offset(i as isize) as sqlite3_int64,
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw mut trimmed as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        if sqlite3_step(stmt) == SQLITE_DONE {
            success += 1;
        }
        i += 1;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_recording_tag_get_recordings_by_tag(
    mut tag: *const ::core::ffi::c_char,
    mut recording_ids: *mut uint64_t,
    mut max_ids: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if tag.is_null() || recording_ids.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut trimmed: [::core::ffi::c_char; 128] = [0; 128];
    if copy_trimmed_value(
        &raw mut trimmed as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        tag,
        0 as size_t,
    ) == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mtx: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mtx);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char =
        b"SELECT recording_id FROM recording_tags WHERE tag = ? ORDER BY recording_id DESC;\0"
            .as_ptr() as *const ::core::ffi::c_char;
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
            b"Failed to prepare recordings-by-tag select: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(mtx);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        &raw mut trimmed as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while sqlite3_step(stmt) == SQLITE_ROW && count < max_ids {
        let c2rust_fresh0 = count;
        count += 1;
        *recording_ids.offset(c2rust_fresh0 as isize) =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return count;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
