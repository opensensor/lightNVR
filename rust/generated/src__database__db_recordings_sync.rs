unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
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
    unsafe fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int)
    -> sqlite3_int64;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn log_set_thread_context(
        component: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    );
    unsafe fn is_shutdown_initiated() -> bool;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn update_recording_metadata(
        id: uint64_t,
        end_time: time_t,
        size_bytes: uint64_t,
        is_complete: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn get_recording_metadata_by_id(
        id: uint64_t,
        metadata: *mut recording_metadata_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn interruptible_sleep_init(s: *mut interruptible_sleep_t) -> ::core::ffi::c_int;
    unsafe fn interruptible_sleep_destroy(s: *mut interruptible_sleep_t);
    unsafe fn interruptible_sleep_wait(s: *mut interruptible_sleep_t, seconds: ::core::ffi::c_int);
    unsafe fn interruptible_sleep_wake(s: *mut interruptible_sleep_t);
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
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
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
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
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
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
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
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
pub struct interruptible_sleep_t {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub woken: bool,
    pub clock_id: clockid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub thread: pthread_t,
    pub running: bool,
    pub interval_seconds: ::core::ffi::c_int,
    pub mutex: pthread_mutex_t,
    pub wake: interruptible_sleep_t,
    pub startup_time: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sync_item_t {
    pub id: uint64_t,
    pub file_path: [::core::ffi::c_char; 512],
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut sync_thread: C2Rust_Unnamed_0 = C2Rust_Unnamed_0 {
    thread: 0,
    running: r#false != 0,
    interval_seconds: 60 as ::core::ffi::c_int,
    mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __glibc_reserved: 0,
            __list: __pthread_list_t {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    wake: interruptible_sleep_t {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __glibc_reserved: 0,
                __list: __pthread_list_t {
                    __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                    __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
                },
            },
        },
        cond: pthread_cond_t {
            __data: __pthread_cond_s {
                __wseq: __atomic_wide_counter { __value64: 0 },
                __g1_start: __atomic_wide_counter { __value64: 0 },
                __g_size: [0; 2],
                __g1_orig_size: 0,
                __wrefs: 0,
                __g_signals: [0; 2],
                __unused_initialized_1: 0,
                __unused_initialized_2: 0,
            },
        },
        woken: false,
        clock_id: 0,
    },
    startup_time: 0 as time_t,
};
unsafe extern "C" fn sync_recording_file_size(
    mut recording_id: uint64_t,
    mut file_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if stat(file_path, &raw mut st) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"File not found for recording %llu: %s\0".as_ptr() as *const ::core::ffi::c_char,
            recording_id as ::core::ffi::c_ulonglong,
            file_path,
        );
        return -1 as ::core::ffi::c_int;
    }
    if st.st_size > 0 as __off_t {
        let mut metadata: recording_metadata_t = recording_metadata_t {
            id: 0,
            stream_name: [0; 64],
            camera_uuid: [0; 37],
            file_path: [0; 512],
            storage_target_uuid: [0; 37],
            object_key: [0; 512],
            placement_reason: [0; 64],
            storage_policy_version: 0,
            start_time: 0,
            end_time: 0,
            size_bytes: 0,
            width: 0,
            height: 0,
            fps: 0,
            codec: [0; 16],
            is_complete: false,
            playback_issue: [0; 32],
            trigger_type: [0; 16],
            protected: false,
            retention_override_days: 0,
            retention_tier: 0,
            disk_pressure_eligible: false,
            schedule_restricted: 0,
        };
        if get_recording_metadata_by_id(recording_id, &raw mut metadata) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to get metadata for recording %llu\0".as_ptr()
                    as *const ::core::ffi::c_char,
                recording_id as ::core::ffi::c_ulonglong,
            );
            return -1 as ::core::ffi::c_int;
        }
        if metadata.size_bytes != st.st_size as uint64_t {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Syncing file size for recording %llu: %llu bytes (was %llu bytes)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                recording_id as ::core::ffi::c_ulonglong,
                st.st_size as ::core::ffi::c_ulonglong,
                metadata.size_bytes as ::core::ffi::c_ulonglong,
            );
            if update_recording_metadata(
                recording_id,
                metadata.end_time,
                st.st_size as uint64_t,
                metadata.is_complete,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to update file size for recording %llu\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    recording_id as ::core::ffi::c_ulonglong,
                );
                return -1 as ::core::ffi::c_int;
            }
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn sync_recordings_needing_size_update() -> ::core::ffi::c_int {
    if is_shutdown_initiated() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Shutdown in progress, aborting recording sync\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut updated_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut error_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized for recording sync\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"SELECT id, file_path FROM recordings WHERE size_bytes = 0 AND is_complete = 1 AND start_time >= ? ORDER BY id ASC LIMIT 1000\0"
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
            b"Failed to prepare sync query: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(
        stmt,
        1 as ::core::ffi::c_int,
        sync_thread.startup_time as sqlite3_int64,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Syncing recordings with size_bytes=0 since startup time %ld\0".as_ptr()
            as *const ::core::ffi::c_char,
        sync_thread.startup_time,
    );
    let mut items: *mut sync_item_t = ::core::ptr::null_mut::<sync_item_t>();
    let mut item_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut item_capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while sqlite3_step(stmt) == SQLITE_ROW {
        if is_shutdown_initiated() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Shutdown in progress, aborting recording sync query\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            break;
        } else {
            if item_count >= item_capacity {
                item_capacity = if item_capacity == 0 as ::core::ffi::c_int {
                    64 as ::core::ffi::c_int
                } else {
                    item_capacity * 2 as ::core::ffi::c_int
                };
                let mut new_items: *mut sync_item_t = realloc(
                    items as *mut ::core::ffi::c_void,
                    (item_capacity as size_t).wrapping_mul(::core::mem::size_of::<sync_item_t>()),
                ) as *mut sync_item_t;
                if new_items.is_null() {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to allocate memory for sync items\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    break;
                } else {
                    items = new_items;
                }
            }
            (*items.offset(item_count as isize)).id =
                sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
            let mut path: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            if !path.is_null() {
                safe_strcpy(
                    &raw mut (*items.offset(item_count as isize)).file_path
                        as *mut ::core::ffi::c_char,
                    path,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    0 as size_t,
                );
            } else {
                (*items.offset(item_count as isize)).file_path[0usize] =
                    '\0' as ::core::ffi::c_char;
            }
            item_count += 1;
        }
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    if item_count > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Found %d recordings needing size sync\0".as_ptr() as *const ::core::ffi::c_char,
            item_count,
        );
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < item_count {
        if is_shutdown_initiated() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Shutdown in progress, aborting recording sync after %d/%d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i,
                item_count,
            );
            break;
        } else {
            if (*items.offset(i as isize)).file_path[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                let mut result: ::core::ffi::c_int = sync_recording_file_size(
                    (*items.offset(i as isize)).id,
                    &raw mut (*items.offset(i as isize)).file_path as *mut ::core::ffi::c_char,
                );
                if result > 0 as ::core::ffi::c_int {
                    updated_count += 1;
                } else if result < 0 as ::core::ffi::c_int {
                    error_count += 1;
                }
            }
            i += 1;
        }
    }
    free(items as *mut ::core::ffi::c_void);
    if updated_count > 0 as ::core::ffi::c_int || error_count > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Recording sync complete: %d updated, %d errors (checked %d recordings)\0".as_ptr()
                as *const ::core::ffi::c_char,
            updated_count,
            error_count,
            item_count,
        );
    }
    return updated_count;
}
unsafe extern "C" fn sync_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"RecordingSync\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Recording sync thread started with interval: %d seconds (syncing recordings since %ld)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        sync_thread.interval_seconds,
        sync_thread.startup_time,
    );
    if !is_shutdown_initiated() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Performing initial recording sync\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sync_recordings_needing_size_update();
    }
    while sync_thread.running as ::core::ffi::c_int != 0 && !is_shutdown_initiated() {
        interruptible_sleep_wait(&raw mut sync_thread.wake, sync_thread.interval_seconds);
        if !sync_thread.running || is_shutdown_initiated() as ::core::ffi::c_int != 0 {
            break;
        }
        sync_recordings_needing_size_update();
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Recording sync thread exiting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_recording_sync_thread(
    mut interval_seconds: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    static mut mutex_initialized: bool = r#false != 0;
    if !mutex_initialized {
        if pthread_mutex_init(
            &raw mut sync_thread.mutex,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to initialize recording sync thread mutex\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        mutex_initialized = r#true != 0;
    }
    pthread_mutex_lock(&raw mut sync_thread.mutex);
    if sync_thread.running {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Recording sync thread is already running\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut sync_thread.mutex);
        return 0 as ::core::ffi::c_int;
    }
    sync_thread.startup_time = time(::core::ptr::null_mut::<time_t>());
    sync_thread.interval_seconds = if interval_seconds < 10 as ::core::ffi::c_int {
        10 as ::core::ffi::c_int
    } else {
        interval_seconds
    };
    sync_thread.running = r#true != 0;
    if interruptible_sleep_init(&raw mut sync_thread.wake) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize recording sync wake handle\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        sync_thread.running = r#false != 0;
        pthread_mutex_unlock(&raw mut sync_thread.mutex);
        return -1 as ::core::ffi::c_int;
    }
    if pthread_create(
        &raw mut sync_thread.thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            sync_thread_func
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create recording sync thread\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sync_thread.running = r#false != 0;
        interruptible_sleep_destroy(&raw mut sync_thread.wake);
        pthread_mutex_unlock(&raw mut sync_thread.mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut sync_thread.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Recording sync thread started with interval: %d seconds\0".as_ptr()
            as *const ::core::ffi::c_char,
        sync_thread.interval_seconds,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_recording_sync_thread() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut sync_thread.mutex);
    if !sync_thread.running {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Recording sync thread is not running\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut sync_thread.mutex);
        return 0 as ::core::ffi::c_int;
    }
    sync_thread.running = r#false != 0;
    pthread_mutex_unlock(&raw mut sync_thread.mutex);
    interruptible_sleep_wake(&raw mut sync_thread.wake);
    if pthread_join(
        sync_thread.thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to join recording sync thread\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    interruptible_sleep_destroy(&raw mut sync_thread.wake);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Recording sync thread stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn force_recording_sync() -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Forcing immediate recording sync\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return sync_recordings_needing_size_update();
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
