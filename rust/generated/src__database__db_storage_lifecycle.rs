use ::f128;
use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
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
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_get(
        uuid: *const ::core::ffi::c_char,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_mount_guard_active(target: *const storage_target_t) -> bool;
    unsafe fn db_storage_migration_create_operation(
        recording_id: uint64_t,
        destination_target_uuid: *const ::core::ffi::c_char,
        operation: *const ::core::ffi::c_char,
        owner_user_id: int64_t,
        job: *mut storage_migration_job_t,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_pool_get(
        uuid: *const ::core::ffi::c_char,
        pool: *mut storage_pool_t,
    ) -> db_storage_pool_result_t;
    unsafe fn storage_recording_expire_policy(recording_id: uint64_t) -> ::core::ffi::c_int;
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
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lifecycle_candidate_t {
    pub recording_id: uint64_t,
    pub source_target_uuid: [::core::ffi::c_char; 37],
    pub required_copy_count: ::core::ffi::c_int,
    pub current_copy_count: ::core::ffi::c_int,
    pub replication_pool_uuid: [::core::ffi::c_char; 37],
    pub migration_after_days: ::core::ffi::c_int,
    pub keep_hot: bool,
    pub archive_due: bool,
    pub migration_target_uuid: [::core::ffi::c_char; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_pool_member_t {
    pub target_uuid: [::core::ffi::c_char; 37],
    pub position: ::core::ffi::c_int,
    pub weight: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_pool_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub strategy: [::core::ffi::c_char; 24],
    pub enabled: bool,
    pub allocation_cursor: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub member_count: ::core::ffi::c_int,
    pub members: [storage_pool_member_t; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_target_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub target_type: [::core::ffi::c_char; 24],
    pub root_path: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub is_default: bool,
    pub storage_class: [::core::ffi::c_char; 16],
    pub reserve_bytes: uint64_t,
    pub high_watermark_pct: ::core::ffi::c_double,
    pub low_watermark_pct: ::core::ffi::c_double,
    pub health_status: [::core::ffi::c_char; 16],
    pub capacity_bytes: uint64_t,
    pub available_bytes: uint64_t,
    pub filesystem_device: uint64_t,
    pub last_probe_at: int64_t,
    pub last_success_at: int64_t,
    pub last_error: [::core::ffi::c_char; 256],
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub recording_count: uint64_t,
    pub recording_bytes: uint64_t,
    pub mount_required: bool,
    pub mount_guard_path: [::core::ffi::c_char; 512],
    pub migration_bandwidth_bps: uint64_t,
    pub archival_window_start_minute: ::core::ffi::c_int,
    pub archival_window_end_minute: ::core::ffi::c_int,
    pub replica_count: uint64_t,
    pub replica_bytes: uint64_t,
    pub endpoint: [::core::ffi::c_char; 512],
    pub region: [::core::ffi::c_char; 64],
    pub bucket: [::core::ffi::c_char; 128],
    pub credential_ref: [::core::ffi::c_char; 128],
    pub archive_budget_bytes: uint64_t,
}
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_target_result_t(pub ::core::ffi::c_int);
impl db_storage_target_result_t {
    pub const DB_STORAGE_TARGET_OK: Self = Self(0);
    pub const DB_STORAGE_TARGET_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_TARGET_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_TARGET_INVALID: Self = Self(-3);
    pub const DB_STORAGE_TARGET_STALE: Self = Self(-4);
    pub const DB_STORAGE_TARGET_LIMIT: Self = Self(-5);
    pub const DB_STORAGE_TARGET_IN_USE: Self = Self(-6);
    pub const DB_STORAGE_TARGET_UNAVAILABLE: Self = Self(-7);
    pub const DB_STORAGE_TARGET_ERROR: Self = Self(-8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_pool_result_t(pub ::core::ffi::c_int);
impl db_storage_pool_result_t {
    pub const DB_STORAGE_POOL_OK: Self = Self(0);
    pub const DB_STORAGE_POOL_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_POOL_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_POOL_INVALID: Self = Self(-3);
    pub const DB_STORAGE_POOL_STALE: Self = Self(-4);
    pub const DB_STORAGE_POOL_LIMIT: Self = Self(-5);
    pub const DB_STORAGE_POOL_IN_USE: Self = Self(-6);
    pub const DB_STORAGE_POOL_UNAVAILABLE: Self = Self(-7);
    pub const DB_STORAGE_POOL_ERROR: Self = Self(-8);
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
pub type violation_t = violation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct violation {
    pub next: *mut violation,
    pub policy_uuid: [::core::ffi::c_char; 37],
    pub resolve_uuid: [::core::ffi::c_char; 37],
    pub recording_id: uint64_t,
    pub scope: [::core::ffi::c_char; 96],
    pub r#type: [::core::ffi::c_char; 32],
    pub details: [::core::ffi::c_char; 256],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const PTHREAD_MUTEX_TIMED_NP: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE_NP: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK_NP: Self = Self(2);
    pub const PTHREAD_MUTEX_ADAPTIVE_NP: Self = Self(3);
    pub const PTHREAD_MUTEX_NORMAL: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK: Self = Self(2);
    pub const PTHREAD_MUTEX_DEFAULT: Self = Self(0);
    pub const PTHREAD_MUTEX_FAST_NP: Self = Self(0);
}
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL_0 as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL_0 as *const ::core::ffi::c_char;
pub const LIGHTNVR_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const LIFECYCLE_CANDIDATE_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
unsafe extern "C" fn policies_present(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT EXISTS(SELECT 1 FROM storage_policies) OR EXISTS(SELECT 1 FROM storage_policy_versions);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut present: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW {
        present = sqlite3_column_int(statement, 0 as ::core::ffi::c_int);
    }
    sqlite3_finalize(statement);
    return present;
}
unsafe extern "C" fn open_policy_reader(mut db: *mut *mut sqlite3) -> ::core::ffi::c_int {
    if db_open_readonly_connection(db) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut present: ::core::ffi::c_int = policies_present(*db);
    if present <= 0 as ::core::ffi::c_int {
        db_close_readonly_connection(*db);
        *db = ::core::ptr::null_mut::<sqlite3>();
    }
    return present;
}
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
unsafe extern "C" fn used_target(
    mut db: *mut sqlite3,
    mut recording_id: uint64_t,
    mut target_uuid: *const ::core::ffi::c_char,
) -> bool {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT 1 FROM recordings r WHERE r.id=? AND r.storage_target_uuid=? UNION ALL SELECT 1 FROM storage_recording_copies c WHERE c.recording_id=? AND c.target_uuid=? LIMIT 1;\0"
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
            target_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            3 as ::core::ffi::c_int,
            recording_id as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            target_uuid,
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
    return result == SQLITE_ROW;
}
unsafe extern "C" fn select_replica_target(
    mut candidate: *const lifecycle_candidate_t,
    mut uuid: *mut ::core::ffi::c_char,
) -> bool {
    let mut pool: storage_pool_t = storage_pool_t {
        uuid: [0; 37],
        name: [0; 128],
        strategy: [0; 24],
        enabled: false,
        allocation_cursor: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
        member_count: 0,
        members: [storage_pool_member_t {
            target_uuid: [0; 37],
            position: 0,
            weight: 0,
        }; 128],
    };
    if db_storage_pool_get(
        &raw const (*candidate).replication_pool_uuid as *const ::core::ffi::c_char,
        &raw mut pool,
    )
    .0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
        || !pool.enabled
    {
        return r#false != 0;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut selected: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut best_free: uint64_t = 0 as uint64_t;
    let mut start: ::core::ffi::c_int = if strcmp(
        &raw mut pool.strategy as *mut ::core::ffi::c_char,
        b"round_robin\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        pool.allocation_cursor % pool.member_count
    } else {
        0 as ::core::ffi::c_int
    };
    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while offset < pool.member_count {
        let mut index: ::core::ffi::c_int = (start + offset) % pool.member_count;
        let mut target: storage_target_t = storage_target_t {
            uuid: [0; 37],
            name: [0; 128],
            target_type: [0; 24],
            root_path: [0; 512],
            enabled: false,
            is_default: false,
            storage_class: [0; 16],
            reserve_bytes: 0,
            high_watermark_pct: 0.,
            low_watermark_pct: 0.,
            health_status: [0; 16],
            capacity_bytes: 0,
            available_bytes: 0,
            filesystem_device: 0,
            last_probe_at: 0,
            last_success_at: 0,
            last_error: [0; 256],
            revision: 0,
            created_at: 0,
            updated_at: 0,
            recording_count: 0,
            recording_bytes: 0,
            mount_required: false,
            mount_guard_path: [0; 512],
            migration_bandwidth_bps: 0,
            archival_window_start_minute: 0,
            archival_window_end_minute: 0,
            replica_count: 0,
            replica_bytes: 0,
            endpoint: [0; 512],
            region: [0; 64],
            bucket: [0; 128],
            credential_ref: [0; 128],
            archive_budget_bytes: 0,
        };
        if !(db_storage_target_get(
            &raw mut (*(&raw mut pool.members as *mut storage_pool_member_t).offset(index as isize))
                .target_uuid as *mut ::core::ffi::c_char,
            &raw mut target,
        )
        .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
            || !target.enabled
            || strcmp(
                &raw mut target.health_status as *mut ::core::ffi::c_char,
                b"healthy\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            || target.mount_required as ::core::ffi::c_int != 0
                && !db_storage_target_mount_guard_active(&raw mut target)
            || target.available_bytes <= target.reserve_bytes)
        {
            pthread_mutex_lock(mutex);
            let mut used: bool = used_target(
                db,
                (*candidate).recording_id,
                &raw mut target.uuid as *mut ::core::ffi::c_char,
            );
            pthread_mutex_unlock(mutex);
            if !used {
                let mut free_bytes: uint64_t =
                    target.available_bytes.wrapping_sub(target.reserve_bytes);
                if selected < 0 as ::core::ffi::c_int
                    || strcmp(
                        &raw mut pool.strategy as *mut ::core::ffi::c_char,
                        b"most_free\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    || free_bytes > best_free
                {
                    selected = index;
                    best_free = free_bytes;
                    if strcmp(
                        &raw mut pool.strategy as *mut ::core::ffi::c_char,
                        b"most_free\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
            }
        }
        offset += 1;
    }
    if selected < 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    safe_strcpy(
        uuid,
        &raw mut (*(&raw mut pool.members as *mut storage_pool_member_t).offset(selected as isize))
            .target_uuid as *mut ::core::ffi::c_char,
        LIGHTNVR_UUID_STRING_SIZE as size_t,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn lifecycle_schedule(mut max_jobs: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if max_jobs < 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if max_jobs > LIFECYCLE_CANDIDATE_MAX {
        max_jobs = LIFECYCLE_CANDIDATE_MAX;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    let mut present: ::core::ffi::c_int = open_policy_reader(&raw mut db);
    if present <= 0 as ::core::ffi::c_int {
        return present;
    }
    let mut candidates: [lifecycle_candidate_t; 128] = [lifecycle_candidate_t {
        recording_id: 0,
        source_target_uuid: [0; 37],
        required_copy_count: 0,
        current_copy_count: 0,
        replication_pool_uuid: [0; 37],
        migration_after_days: 0,
        keep_hot: false,
        archive_due: false,
        migration_target_uuid: [0; 37],
    }; 128];
    memset(
        &raw mut candidates as *mut lifecycle_candidate_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[lifecycle_candidate_t; 128]>(),
    );
    let mut sql: *const ::core::ffi::c_char = b"WITH eligible AS (SELECT r.id,COALESCE(r.storage_target_uuid,'') source,p.required_copy_count,(SELECT count(*) FROM storage_recording_copies c WHERE c.recording_id=r.id) copies,COALESCE(p.replication_pool_uuid,'') pool,p.migration_after_days,COALESCE(p.migration_target_uuid,'') destination,(p.hot_residency_seconds>=0 AND r.start_time>strftime('%s','now')-p.hot_residency_seconds) keep_hot,(p.migration_target_uuid IS NOT NULL AND r.storage_target_uuid<>p.migration_target_uuid AND ((r.protected=1 AND p.archive_protected=1) OR (p.migration_after_days>0 AND r.start_time<=strftime('%s','now')-p.migration_after_days*86400) OR (p.migration_after_days=0 AND p.archive_after_seconds>=0 AND r.start_time<=strftime('%s','now')-p.archive_after_seconds))) age_due,(p.archive_on_pressure=1 AND hot.target_type='filesystem' AND hot.capacity_bytes>0 AND (hot.available_bytes<=hot.reserve_bytes OR 100.0*(hot.capacity_bytes-hot.available_bytes)/hot.capacity_bytes>=hot.high_watermark_pct)) pressure_due,EXISTS(SELECT 1 FROM storage_recording_copies c WHERE c.recording_id=r.id AND c.target_uuid=p.migration_target_uuid) archived,r.start_time,(r.protected AND p.archive_protected) protected_priority FROM recordings r JOIN storage_recording_policies p ON p.recording_id=r.id AND p.enabled=1 AND p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1)) JOIN storage_targets hot ON hot.uuid=r.storage_target_uuid WHERE r.is_complete=1 AND r.deletion_pending=0 AND (r.protected=1 OR r.retention_override_days=0 OR (r.retention_override_days>0 AND r.start_time>strftime('%s','now')-r.retention_override_days*86400) OR (COALESCE(r.retention_override_days,-1)<0 AND (p.maximum_retention_days=0 OR r.start_time> strftime('%s','now')-p.maximum_retention_days*86400))) AND NOT EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=r.id AND j.state<>'completed')) SELECT id,source,required_copy_count,copies,pool,migration_after_days,destination,(keep_hot AND NOT pressure_due),(age_due OR pressure_due) FROM eligible WHERE required_copy_count>1+copies OR (destination<>'' AND destination<>source AND (age_due OR pressure_due) AND (NOT archived OR ((NOT keep_hot OR pressure_due) AND required_copy_count<=copies))) ORDER BY pressure_due DESC,protected_priority DESC,start_time LIMIT ?;\0"
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
        sqlite3_bind_int(statement, 1 as ::core::ffi::c_int, max_jobs);
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (result == SQLITE_OK || result == SQLITE_ROW) && count < max_jobs {
        result = sqlite3_step(statement);
        if result != SQLITE_ROW {
            break;
        }
        let c2rust_fresh0 = count;
        count += 1;
        let mut candidate: *mut lifecycle_candidate_t =
            (&raw mut candidates as *mut lifecycle_candidate_t).offset(c2rust_fresh0 as isize);
        (*candidate).recording_id =
            sqlite3_column_int64(statement, 0 as ::core::ffi::c_int) as uint64_t;
        copy_column(
            &raw mut (*candidate).source_target_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            statement,
            1 as ::core::ffi::c_int,
        );
        (*candidate).required_copy_count = sqlite3_column_int(statement, 2 as ::core::ffi::c_int);
        (*candidate).current_copy_count = sqlite3_column_int(statement, 3 as ::core::ffi::c_int);
        copy_column(
            &raw mut (*candidate).replication_pool_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            statement,
            4 as ::core::ffi::c_int,
        );
        (*candidate).migration_after_days = sqlite3_column_int(statement, 5 as ::core::ffi::c_int);
        (*candidate).keep_hot =
            sqlite3_column_int(statement, 7 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        (*candidate).archive_due =
            sqlite3_column_int(statement, 8 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        copy_column(
            &raw mut (*candidate).migration_target_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            statement,
            6 as ::core::ffi::c_int,
        );
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    db_close_readonly_connection(db);
    if result != SQLITE_DONE && result != SQLITE_ROW {
        return -1 as ::core::ffi::c_int;
    }
    let mut scheduled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut candidate_0: *mut lifecycle_candidate_t =
            (&raw mut candidates as *mut lifecycle_candidate_t).offset(index as isize);
        let mut destination: [::core::ffi::c_char; 37] = [
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
        let mut operation: *const ::core::ffi::c_char =
            b"move\0".as_ptr() as *const ::core::ffi::c_char;
        's_117: {
            if (*candidate_0).required_copy_count
                > 1 as ::core::ffi::c_int + (*candidate_0).current_copy_count
            {
                operation = b"copy\0".as_ptr() as *const ::core::ffi::c_char;
                if !select_replica_target(
                    candidate_0,
                    &raw mut destination as *mut ::core::ffi::c_char,
                ) {
                    break 's_117;
                }
            } else if (*candidate_0).archive_due {
                safe_strcpy(
                    &raw mut destination as *mut ::core::ffi::c_char,
                    &raw mut (*candidate_0).migration_target_uuid as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                    0 as size_t,
                );
                if (*candidate_0).keep_hot {
                    operation = b"copy\0".as_ptr() as *const ::core::ffi::c_char;
                }
            } else {
                break 's_117;
            }
            let mut job: storage_migration_job_t = storage_migration_job_t {
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
            let mut created: db_storage_migration_result_t = db_storage_migration_create_operation(
                (*candidate_0).recording_id,
                &raw mut destination as *mut ::core::ffi::c_char,
                operation,
                0 as int64_t,
                &raw mut job,
            );
            if created.0 == db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0 {
                scheduled += 1;
            }
        }
        index += 1;
    }
    return scheduled;
}
unsafe extern "C" fn lifecycle_expire(mut maximum: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if maximum < 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if maximum > LIFECYCLE_CANDIDATE_MAX {
        maximum = LIFECYCLE_CANDIDATE_MAX;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    let mut present: ::core::ffi::c_int = open_policy_reader(&raw mut db);
    if present <= 0 as ::core::ffi::c_int {
        return present;
    }
    let mut ids: [uint64_t; 128] = [0; 128];
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char = b"SELECT r.id FROM recordings r JOIN storage_recording_policies p ON p.recording_id=r.id WHERE r.is_complete=1 AND r.protected=0 AND r.deletion_pending=0 AND COALESCE(r.retention_override_days,-1)<>0 AND (p.minimum_retention_days=0 OR r.start_time<strftime('%s','now')-p.minimum_retention_days*86400) AND ((r.retention_override_days>0 AND r.start_time<strftime('%s','now')-r.retention_override_days*86400) OR (COALESCE(r.retention_override_days,-1)<0 AND p.maximum_retention_days>0 AND r.start_time<strftime('%s','now')-p.maximum_retention_days*86400)) AND NOT EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=r.id AND j.state IN('copying','verifying','committing')) ORDER BY r.start_time LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_int(stmt, 1 as ::core::ffi::c_int, maximum);
        while count < maximum && {
            result = sqlite3_step(stmt);
            result == SQLITE_ROW
        } {
            let c2rust_fresh1 = count;
            count += 1;
            ids[c2rust_fresh1 as usize] =
                sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    db_close_readonly_connection(db);
    if result != SQLITE_DONE && result != SQLITE_ROW {
        return -1 as ::core::ffi::c_int;
    }
    let mut accepted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if storage_recording_expire_policy(ids[i as usize]) >= 0 as ::core::ffi::c_int {
            accepted += 1;
        }
        i += 1;
    }
    return accepted;
}
unsafe extern "C" fn upsert_violation(
    mut db: *mut sqlite3,
    mut policy_uuid: *const ::core::ffi::c_char,
    mut recording_id: uint64_t,
    mut scope_key: *const ::core::ffi::c_char,
    mut r#type: *const ::core::ffi::c_char,
    mut details: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"INSERT INTO storage_policy_violations(uuid,policy_uuid,recording_id,scope_key,violation_type,details) SELECT ?1,?2,NULLIF(?3,0),?4,?5,?6 WHERE EXISTS(SELECT 1 FROM storage_policies WHERE uuid=?2) AND (?3=0 OR EXISTS(SELECT 1 FROM recordings WHERE id=?3)) ON CONFLICT(policy_uuid,scope_key,violation_type) DO UPDATE SET details=excluded.details,last_seen_at=strftime('%s','now'),resolved_at=NULL;\0"
            .as_ptr() as *const ::core::ffi::c_char,
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
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            policy_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            3 as ::core::ffi::c_int,
            recording_id as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            scope_key,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            r#type,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            details,
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
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn collect_violation(
    mut head: *mut *mut violation_t,
    mut policy_uuid: *const ::core::ffi::c_char,
    mut recording_id: uint64_t,
    mut scope: *const ::core::ffi::c_char,
    mut r#type: *const ::core::ffi::c_char,
    mut details: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut item: *mut violation_t =
        calloc(1 as size_t, ::core::mem::size_of::<violation_t>()) as *mut violation_t;
    if item.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut (*item).policy_uuid as *mut ::core::ffi::c_char,
        policy_uuid,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    (*item).recording_id = recording_id;
    safe_strcpy(
        &raw mut (*item).scope as *mut ::core::ffi::c_char,
        scope,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*item).r#type as *mut ::core::ffi::c_char,
        r#type,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*item).details as *mut ::core::ffi::c_char,
        details,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    (*item).next = *head as *mut violation;
    *head = item;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn compare_violations(
    mut left: *const ::core::ffi::c_void,
    mut right: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const violation_t = *(left as *const *const violation_t);
    let mut b: *const violation_t = *(right as *const *const violation_t);
    let mut result: ::core::ffi::c_int = strcmp(
        &raw const (*a).policy_uuid as *const ::core::ffi::c_char,
        &raw const (*b).policy_uuid as *const ::core::ffi::c_char,
    );
    if result == 0 {
        result = strcmp(
            &raw const (*a).scope as *const ::core::ffi::c_char,
            &raw const (*b).scope as *const ::core::ffi::c_char,
        );
    }
    if result == 0 {
        result = strcmp(
            &raw const (*a).r#type as *const ::core::ffi::c_char,
            &raw const (*b).r#type as *const ::core::ffi::c_char,
        );
    }
    return result;
}
unsafe extern "C" fn collect_resolutions(
    mut db: *mut sqlite3,
    mut active: *mut violation_t,
    mut count: ::core::ffi::c_int,
    mut resolved: *mut *mut violation_t,
) -> ::core::ffi::c_int {
    let mut index: *mut *mut violation_t = (if count != 0 {
        calloc(count as size_t, ::core::mem::size_of::<*mut violation_t>())
    } else {
        NULL_0
    }) as *mut *mut violation_t;
    if count != 0 && index.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut item: *mut violation_t = active;
    while !item.is_null() {
        let c2rust_fresh2 = i;
        i += 1;
        *index.offset(c2rust_fresh2 as isize) = item;
        item = (*item).next as *mut violation_t;
    }
    if count != 0 {
        qsort(
            index as *mut ::core::ffi::c_void,
            count as size_t,
            ::core::mem::size_of::<*mut violation_t>(),
            Some(
                compare_violations
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT uuid,policy_uuid,scope_key,violation_type FROM storage_policy_violations WHERE resolved_at IS NULL;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    while (result == SQLITE_OK || result == SQLITE_ROW) && {
        result = sqlite3_step(statement);
        result == SQLITE_ROW
    } {
        let mut key: violation_t = violation {
            next: ::core::ptr::null_mut::<violation>(),
            policy_uuid: [0; 37],
            resolve_uuid: [0; 37],
            recording_id: 0,
            scope: [0; 96],
            r#type: [0; 32],
            details: [0; 256],
        };
        copy_column(
            &raw mut key.policy_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            statement,
            1 as ::core::ffi::c_int,
        );
        copy_column(
            &raw mut key.scope as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            statement,
            2 as ::core::ffi::c_int,
        );
        copy_column(
            &raw mut key.r#type as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            statement,
            3 as ::core::ffi::c_int,
        );
        let mut key_pointer: *mut violation_t = &raw mut key;
        if count != 0
            && !bsearch(
                &raw mut key_pointer as *const ::core::ffi::c_void,
                index as *const ::core::ffi::c_void,
                count as size_t,
                ::core::mem::size_of::<*mut violation_t>(),
                Some(
                    compare_violations
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            )
            .is_null()
        {
            continue;
        }
        let mut item_0: *mut violation_t =
            calloc(1 as size_t, ::core::mem::size_of::<violation_t>()) as *mut violation_t;
        if item_0.is_null() {
            result = SQLITE_NOMEM;
            break;
        } else {
            copy_column(
                &raw mut (*item_0).resolve_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                statement,
                0 as ::core::ffi::c_int,
            );
            (*item_0).next = *resolved as *mut violation;
            *resolved = item_0;
        }
    }
    sqlite3_finalize(statement);
    free(index as *mut ::core::ffi::c_void);
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn write_violations(mut items: *mut violation_t) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    while !items.is_null() {
        pthread_mutex_lock(mutex);
        if sqlite3_exec(
            db,
            b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL_0,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) != SQLITE_OK
        {
            pthread_mutex_unlock(mutex);
            return -1 as ::core::ffi::c_int;
        }
        let mut result: ::core::ffi::c_int = SQLITE_OK;
        let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !items.is_null() && count < 64 as ::core::ffi::c_int && result == SQLITE_OK {
            if (*items).resolve_uuid[0usize] != 0 {
                let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                result = sqlite3_prepare_v2(
                    db,
                    b"UPDATE storage_policy_violations SET resolved_at=strftime('%s','now') WHERE uuid=? AND resolved_at IS NULL;\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    &raw mut statement,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                if result == SQLITE_OK {
                    sqlite3_bind_text(
                        statement,
                        1 as ::core::ffi::c_int,
                        &raw mut (*items).resolve_uuid as *mut ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                    result = if sqlite3_step(statement) == SQLITE_DONE {
                        SQLITE_OK
                    } else {
                        SQLITE_ERROR
                    };
                }
                sqlite3_finalize(statement);
            } else if upsert_violation(
                db,
                &raw mut (*items).policy_uuid as *mut ::core::ffi::c_char,
                (*items).recording_id,
                &raw mut (*items).scope as *mut ::core::ffi::c_char,
                &raw mut (*items).r#type as *mut ::core::ffi::c_char,
                &raw mut (*items).details as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                result = SQLITE_ERROR;
            }
            count += 1;
            items = (*items).next as *mut violation_t;
        }
        if result != SQLITE_OK
            || sqlite3_exec(
                db,
                b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL_0,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ) != SQLITE_OK
        {
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL_0,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            pthread_mutex_unlock(mutex);
            return -1 as ::core::ffi::c_int;
        }
        pthread_mutex_unlock(mutex);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn lifecycle_reconcile() -> ::core::ffi::c_int {
    let mut failed_sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut target_sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pool_sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut minimum_sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    let mut present: ::core::ffi::c_int = open_policy_reader(&raw mut db);
    if present <= 0 as ::core::ffi::c_int {
        return present;
    }
    let mut violations: *mut violation_t = ::core::ptr::null_mut::<violation_t>();
    let mut resolved: *mut violation_t = ::core::ptr::null_mut::<violation_t>();
    if sqlite3_exec(
        db,
        b"BEGIN;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL_0,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    let mut active: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut copy_sql: *const ::core::ffi::c_char = b"SELECT p.uuid,r.id,p.required_copy_count,1+count(c.uuid) FROM recordings r JOIN storage_recording_policies p ON p.recording_id=r.id AND p.enabled=1 AND p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1)) LEFT JOIN storage_recording_copies c ON c.recording_id=r.id WHERE r.is_complete=1 AND p.required_copy_count>1 GROUP BY p.uuid,r.id HAVING 1+count(c.uuid)<p.required_copy_count;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        copy_sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    while (result == SQLITE_OK || result == SQLITE_ROW) && {
        result = sqlite3_step(statement);
        result == SQLITE_ROW
    } {
        let mut policy: *const ::core::ffi::c_char =
            sqlite3_column_text(statement, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut recording_id: uint64_t =
            sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as uint64_t;
        let mut scope: [::core::ffi::c_char; 64] = [0; 64];
        let mut details: [::core::ffi::c_char; 160] = [0; 160];
        snprintf(
            &raw mut scope as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"recording:%llu\0".as_ptr() as *const ::core::ffi::c_char,
            recording_id as ::core::ffi::c_ulonglong,
        );
        snprintf(
            &raw mut details as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
            b"requires %d copies; %d verified\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_column_int(statement, 2 as ::core::ffi::c_int),
            sqlite3_column_int(statement, 3 as ::core::ffi::c_int),
        );
        if collect_violation(
            &raw mut violations,
            policy,
            recording_id,
            &raw mut scope as *mut ::core::ffi::c_char,
            b"copy_count\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut details as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            active += 1;
        } else {
            result = SQLITE_NOMEM;
            break;
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    '_cleanup: {
        if result == SQLITE_DONE {
            failed_sql = b"SELECT p.uuid,r.id,j.uuid,COALESCE(j.last_error,'') FROM storage_migration_jobs j JOIN recordings r ON r.id=j.recording_id JOIN storage_recording_policies p ON p.recording_id=r.id AND p.uuid=substr(r.placement_reason,instr(r.placement_reason,':')+1) WHERE j.state='failed' AND NOT EXISTS(SELECT 1 FROM storage_migration_jobs newer WHERE newer.recording_id=j.recording_id AND newer.state='completed' AND newer.created_at>=j.created_at);\0"
                .as_ptr() as *const ::core::ffi::c_char;
            statement = ::core::ptr::null_mut::<sqlite3_stmt>();
            result = sqlite3_prepare_v2(
                db,
                failed_sql,
                -1 as ::core::ffi::c_int,
                &raw mut statement,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            while (result == SQLITE_OK || result == SQLITE_ROW) && {
                result = sqlite3_step(statement);
                result == SQLITE_ROW
            } {
                let mut policy_0: *const ::core::ffi::c_char =
                    sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char;
                let mut recording_id_0: uint64_t =
                    sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as uint64_t;
                let mut job: *const ::core::ffi::c_char =
                    sqlite3_column_text(statement, 2 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char;
                let mut error: *const ::core::ffi::c_char =
                    sqlite3_column_text(statement, 3 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char;
                let mut scope_0: [::core::ffi::c_char; 96] = [0; 96];
                let mut details_0: [::core::ffi::c_char; 256] = [0; 256];
                snprintf(
                    &raw mut scope_0 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
                    b"job:%s\0".as_ptr() as *const ::core::ffi::c_char,
                    if !job.is_null() {
                        job
                    } else {
                        b"unknown\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                snprintf(
                    &raw mut details_0 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    if !error.is_null() {
                        error
                    } else {
                        b"Migration failed\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                if collect_violation(
                    &raw mut violations,
                    policy_0,
                    recording_id_0,
                    &raw mut scope_0 as *mut ::core::ffi::c_char,
                    b"migration_failed\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut details_0 as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    active += 1;
                } else {
                    result = SQLITE_NOMEM;
                    break;
                }
            }
            if !statement.is_null() {
                sqlite3_finalize(statement);
            }
            statement = ::core::ptr::null_mut::<sqlite3_stmt>();
            if result == SQLITE_DONE {
                target_sql = b"SELECT p.uuid,t.uuid,t.name,COALESCE(t.health_status,'unknown') FROM storage_policies p JOIN storage_targets t ON t.uuid=p.primary_target_uuid WHERE p.enabled=1 AND p.primary_pool_uuid IS NULL AND (t.enabled=0 OR t.health_status<>'healthy');\0"
                    .as_ptr() as *const ::core::ffi::c_char;
                statement = ::core::ptr::null_mut::<sqlite3_stmt>();
                result = sqlite3_prepare_v2(
                    db,
                    target_sql,
                    -1 as ::core::ffi::c_int,
                    &raw mut statement,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                while (result == SQLITE_OK || result == SQLITE_ROW) && {
                    result = sqlite3_step(statement);
                    result == SQLITE_ROW
                } {
                    let mut policy_1: *const ::core::ffi::c_char =
                        sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    let mut target: *const ::core::ffi::c_char =
                        sqlite3_column_text(statement, 1 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    let mut name: *const ::core::ffi::c_char =
                        sqlite3_column_text(statement, 2 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    let mut health: *const ::core::ffi::c_char =
                        sqlite3_column_text(statement, 3 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    let mut scope_1: [::core::ffi::c_char; 96] = [0; 96];
                    let mut details_1: [::core::ffi::c_char; 200] = [0; 200];
                    snprintf(
                        &raw mut scope_1 as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
                        b"target:%s\0".as_ptr() as *const ::core::ffi::c_char,
                        if !target.is_null() {
                            target
                        } else {
                            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                    snprintf(
                        &raw mut details_1 as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 200]>(),
                        b"%s is %s\0".as_ptr() as *const ::core::ffi::c_char,
                        if !name.is_null() {
                            name
                        } else {
                            b"Primary target\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        if !health.is_null() {
                            health
                        } else {
                            b"unavailable\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                    if collect_violation(
                        &raw mut violations,
                        policy_1,
                        0 as uint64_t,
                        &raw mut scope_1 as *mut ::core::ffi::c_char,
                        b"target_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut details_1 as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        active += 1;
                    } else {
                        result = SQLITE_NOMEM;
                        break;
                    }
                }
                if !statement.is_null() {
                    sqlite3_finalize(statement);
                }
                statement = ::core::ptr::null_mut::<sqlite3_stmt>();
                if result == SQLITE_DONE {
                    pool_sql = b"SELECT p.uuid,sp.uuid,sp.name FROM storage_policies p JOIN storage_pools sp ON sp.uuid=p.primary_pool_uuid WHERE p.enabled=1 AND (sp.enabled=0 OR NOT EXISTS(SELECT 1 FROM storage_pool_members pm JOIN storage_targets t ON t.uuid=pm.target_uuid WHERE pm.pool_uuid=sp.uuid AND t.enabled=1 AND t.health_status='healthy' AND t.available_bytes>t.reserve_bytes));\0"
                        .as_ptr() as *const ::core::ffi::c_char;
                    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
                    result = sqlite3_prepare_v2(
                        db,
                        pool_sql,
                        -1 as ::core::ffi::c_int,
                        &raw mut statement,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    );
                    while (result == SQLITE_OK || result == SQLITE_ROW) && {
                        result = sqlite3_step(statement);
                        result == SQLITE_ROW
                    } {
                        let mut policy_2: *const ::core::ffi::c_char =
                            sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                                as *const ::core::ffi::c_char;
                        let mut pool: *const ::core::ffi::c_char =
                            sqlite3_column_text(statement, 1 as ::core::ffi::c_int)
                                as *const ::core::ffi::c_char;
                        let mut name_0: *const ::core::ffi::c_char =
                            sqlite3_column_text(statement, 2 as ::core::ffi::c_int)
                                as *const ::core::ffi::c_char;
                        let mut scope_2: [::core::ffi::c_char; 96] = [0; 96];
                        let mut details_2: [::core::ffi::c_char; 200] = [0; 200];
                        snprintf(
                            &raw mut scope_2 as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
                            b"pool:%s\0".as_ptr() as *const ::core::ffi::c_char,
                            if !pool.is_null() {
                                pool
                            } else {
                                b"unknown\0".as_ptr() as *const ::core::ffi::c_char
                            },
                        );
                        snprintf(
                            &raw mut details_2 as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 200]>(),
                            b"%s has no healthy member\0".as_ptr() as *const ::core::ffi::c_char,
                            if !name_0.is_null() {
                                name_0
                            } else {
                                b"Primary pool\0".as_ptr() as *const ::core::ffi::c_char
                            },
                        );
                        if collect_violation(
                            &raw mut violations,
                            policy_2,
                            0 as uint64_t,
                            &raw mut scope_2 as *mut ::core::ffi::c_char,
                            b"target_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut details_2 as *mut ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            active += 1;
                        } else {
                            result = SQLITE_NOMEM;
                            break;
                        }
                    }
                    if !statement.is_null() {
                        sqlite3_finalize(statement);
                    }
                    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
                    if result == SQLITE_DONE {
                        minimum_sql = b"SELECT p.uuid,p.minimum_retention_days,COALESCE(SUM(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN r.size_bytes ELSE 0 END),0),MIN(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN r.start_time END),MAX(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN r.start_time END),CASE WHEN p.primary_pool_uuid IS NOT NULL THEN COALESCE((SELECT SUM(group_usable) FROM (SELECT MAX(MAX(CAST(t.capacity_bytes*t.high_watermark_pct/100 AS INTEGER)-t.reserve_bytes,0)) group_usable FROM storage_pool_members pm JOIN storage_targets t ON t.uuid=pm.target_uuid WHERE pm.pool_uuid=p.primary_pool_uuid AND t.enabled=1 GROUP BY CASE WHEN t.filesystem_device<>0 THEN printf('%llu',t.filesystem_device) ELSE t.uuid END)),0) ELSE MAX(CAST(pt.capacity_bytes*pt.high_watermark_pct/100 AS INTEGER)-pt.reserve_bytes,0) END FROM storage_policies p LEFT JOIN storage_targets pt ON pt.uuid=p.primary_target_uuid LEFT JOIN recordings r ON p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1)) WHERE p.enabled=1 AND p.minimum_retention_days>0 GROUP BY p.uuid;\0"
                            .as_ptr() as *const ::core::ffi::c_char;
                        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
                        result = sqlite3_prepare_v2(
                            db,
                            minimum_sql,
                            -1 as ::core::ffi::c_int,
                            &raw mut statement,
                            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                        );
                        while (result == SQLITE_OK || result == SQLITE_ROW) && {
                            result = sqlite3_step(statement);
                            result == SQLITE_ROW
                        } {
                            let mut policy_3: *const ::core::ffi::c_char =
                                sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                                    as *const ::core::ffi::c_char;
                            let mut minimum_days: ::core::ffi::c_int =
                                sqlite3_column_int(statement, 1 as ::core::ffi::c_int);
                            let mut bytes: ::f128::f128 = ::f128::f128::new(sqlite3_column_int64(
                                statement,
                                2 as ::core::ffi::c_int,
                            ));
                            let mut first: int64_t =
                                sqlite3_column_int64(statement, 3 as ::core::ffi::c_int) as int64_t;
                            let mut last: int64_t =
                                sqlite3_column_int64(statement, 4 as ::core::ffi::c_int) as int64_t;
                            let mut usable: ::f128::f128 = ::f128::f128::new(sqlite3_column_int64(
                                statement,
                                5 as ::core::ffi::c_int,
                            ));
                            let mut sample_days: ::f128::f128 =
                                if first > 0 as int64_t && last >= first {
                                    ::f128::f128::new(last - first) / ::f128::f128::new(86400.0)
                                        + ::f128::f128::new(1.0)
                                } else {
                                    ::f128::f128::new(0.0)
                                };
                            let mut daily: ::f128::f128 = if sample_days > ::f128::f128::new(0.0) {
                                bytes / sample_days
                            } else {
                                ::f128::f128::new(0.0)
                            };
                            let mut forecast_days: ::f128::f128 = if daily > ::f128::f128::new(0.0)
                            {
                                usable / daily
                            } else {
                                ::f128::f128::new(0.0)
                            };
                            if !(daily > ::f128::f128::new(0.0)
                                && forecast_days < ::f128::f128::new(minimum_days))
                            {
                                continue;
                            }
                            let mut details_3: [::core::ffi::c_char; 200] = [0; 200];
                            snprintf(
                                &raw mut details_3 as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 200]>(),
                                b"%.1Lf forecast days is below %d-day minimum\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                forecast_days,
                                minimum_days,
                            );
                            if collect_violation(
                                &raw mut violations,
                                policy_3,
                                0 as uint64_t,
                                b"policy\0".as_ptr() as *const ::core::ffi::c_char,
                                b"minimum_retention\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut details_3 as *mut ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            {
                                active += 1;
                            } else {
                                result = SQLITE_NOMEM;
                                break;
                            }
                        }
                        if !statement.is_null() {
                            sqlite3_finalize(statement);
                        }
                        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
                        if result == SQLITE_DONE {
                            if collect_resolutions(db, violations, active, &raw mut resolved)
                                == 0 as ::core::ffi::c_int
                            {
                                db_close_readonly_connection(db);
                                if write_violations(violations) != 0 as ::core::ffi::c_int
                                    || write_violations(resolved) != 0 as ::core::ffi::c_int
                                {
                                    active = -1 as ::core::ffi::c_int;
                                }
                                break '_cleanup;
                            }
                        }
                    }
                }
            }
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage lifecycle reconciliation read failed: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        db_close_readonly_connection(db);
        active = -1 as ::core::ffi::c_int;
    }
    while !resolved.is_null() {
        let mut next: *mut violation_t = (*resolved).next;
        free(resolved as *mut ::core::ffi::c_void);
        resolved = next;
    }
    while !violations.is_null() {
        let mut next_0: *mut violation_t = (*violations).next;
        free(violations as *mut ::core::ffi::c_void);
        violations = next_0;
    }
    return active;
}
unsafe extern "C" fn log_lifecycle_duration(
    mut task: *const ::core::ffi::c_char,
    mut start: timespec,
    mut result: ::core::ffi::c_int,
) {
    let mut end: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut end);
    let mut elapsed_ms: ::core::ffi::c_double =
        (end.tv_sec - start.tv_sec) as ::core::ffi::c_double * 1000.0f64
            + (end.tv_nsec - start.tv_nsec) as ::core::ffi::c_double / 1000000.0f64;
    if result < 0 as ::core::ffi::c_int || elapsed_ms >= 1000.0f64 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage lifecycle %s: result=%d duration_ms=%.1f\0".as_ptr()
                as *const ::core::ffi::c_char,
            task,
            result,
            elapsed_ms,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage lifecycle %s: result=%d duration_ms=%.1f\0".as_ptr()
                as *const ::core::ffi::c_char,
            task,
            result,
            elapsed_ms,
        );
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_lifecycle_schedule(
    mut max_jobs: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut start);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Storage lifecycle schedule: starting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut result: ::core::ffi::c_int = lifecycle_schedule(max_jobs);
    log_lifecycle_duration(
        b"schedule\0".as_ptr() as *const ::core::ffi::c_char,
        start,
        result,
    );
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_lifecycle_expire(
    mut maximum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut start);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Storage lifecycle expire: starting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut result: ::core::ffi::c_int = lifecycle_expire(maximum);
    log_lifecycle_duration(
        b"expire\0".as_ptr() as *const ::core::ffi::c_char,
        start,
        result,
    );
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_lifecycle_reconcile() -> ::core::ffi::c_int {
    let mut start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut start);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Storage lifecycle reconcile: starting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    static mut reconcile_mutex: pthread_mutex_t = pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0 as ::core::ffi::c_int,
            __count: 0 as ::core::ffi::c_uint,
            __owner: 0 as ::core::ffi::c_int,
            __nusers: 0 as ::core::ffi::c_uint,
            __kind: C2Rust_Unnamed::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
            __spins: 0 as ::core::ffi::c_short,
            __glibc_reserved: 0 as ::core::ffi::c_short,
            __list: __pthread_internal_list {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    };
    pthread_mutex_lock(&raw mut reconcile_mutex);
    let mut result: ::core::ffi::c_int = lifecycle_reconcile();
    pthread_mutex_unlock(&raw mut reconcile_mutex);
    log_lifecycle_duration(
        b"reconcile\0".as_ptr() as *const ::core::ffi::c_char,
        start,
        result,
    );
    return result;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const ::core::ffi::c_void,
    mut __base: *const ::core::ffi::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut ::core::ffi::c_void {
    let mut __p: *const ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>();
    let mut __comparison: ::core::ffi::c_int = 0;
    while __nmemb != 0 {
        __p = (__base as *const ::core::ffi::c_char)
            .offset((__nmemb >> 1 as ::core::ffi::c_int).wrapping_mul(__size) as isize)
            as *const ::core::ffi::c_void;
        __comparison = Some(__compar.expect("non-null function pointer"))
            .expect("non-null function pointer")(__key, __p);
        if __comparison == 0 as ::core::ffi::c_int {
            return __p as *mut ::core::ffi::c_void;
        }
        if __comparison > 0 as ::core::ffi::c_int {
            __base = (__p as *const ::core::ffi::c_char).offset(__size as isize)
                as *const ::core::ffi::c_void;
            __nmemb = __nmemb.wrapping_sub(1);
        }
        __nmemb >>= 1 as ::core::ffi::c_int;
    }
    return NULL;
}
