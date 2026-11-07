use ::libc;
unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_get(
        uuid: *const ::core::ffi::c_char,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_mount_guard_active(target: *const storage_target_t) -> bool;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_clear_bindings(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    unsafe fn copy_trimmed_value(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
        input: *const ::core::ffi::c_char,
        input_size: size_t,
    ) -> size_t;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const _ISupper: Self = Self(256);
    pub const _ISlower: Self = Self(512);
    pub const _ISalpha: Self = Self(1024);
    pub const _ISdigit: Self = Self(2048);
    pub const _ISxdigit: Self = Self(4096);
    pub const _ISspace: Self = Self(8192);
    pub const _ISprint: Self = Self(16384);
    pub const _ISgraph: Self = Self(32768);
    pub const _ISblank: Self = Self(1);
    pub const _IScntrl: Self = Self(2);
    pub const _ISpunct: Self = Self(4);
    pub const _ISalnum: Self = Self(8);
}
pub const STORAGE_TARGET_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const STORAGE_POOL_MAX_COUNT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const STORAGE_POOL_MEMBER_MAX: ::core::ffi::c_int = STORAGE_TARGET_MAX_COUNT;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_FOREIGNKEY: ::core::ffi::c_int =
    SQLITE_CONSTRAINT | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_PRIMARYKEY: ::core::ffi::c_int =
    SQLITE_CONSTRAINT | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_UNIQUE: ::core::ffi::c_int =
    SQLITE_CONSTRAINT | (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
unsafe extern "C" fn copy_column(
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
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
        destination_size,
        0 as size_t,
    );
}
unsafe extern "C" fn load_members_locked(
    mut db: *mut sqlite3,
    mut pool: *mut storage_pool_t,
) -> db_storage_pool_result_t {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT target_uuid,position,weight FROM storage_pool_members WHERE pool_uuid=? ORDER BY position,target_uuid;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut (*pool).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
    (*pool).member_count = 0 as ::core::ffi::c_int;
    while result == SQLITE_OK || result == SQLITE_ROW {
        result = sqlite3_step(statement);
        if result != SQLITE_ROW {
            break;
        }
        if (*pool).member_count >= STORAGE_POOL_MEMBER_MAX {
            sqlite3_finalize(statement);
            return db_storage_pool_result_t::DB_STORAGE_POOL_LIMIT;
        }
        let c2rust_fresh0 = (*pool).member_count;
        (*pool).member_count += 1;
        let mut member: *mut storage_pool_member_t =
            (&raw mut (*pool).members as *mut storage_pool_member_t).offset(c2rust_fresh0 as isize);
        copy_column(
            &raw mut (*member).target_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            statement,
            0 as ::core::ffi::c_int,
        );
        (*member).position = sqlite3_column_int(statement, 1 as ::core::ffi::c_int);
        (*member).weight = sqlite3_column_int(statement, 2 as ::core::ffi::c_int);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return db_storage_pool_result_t(if result == SQLITE_DONE {
        db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
    } else {
        db_storage_pool_result_t::DB_STORAGE_POOL_ERROR.0
    });
}
unsafe extern "C" fn populate(mut statement: *mut sqlite3_stmt, mut pool: *mut storage_pool_t) {
    memset(
        pool as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_pool_t>(),
    );
    copy_column(
        &raw mut (*pool).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*pool).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*pool).strategy as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        statement,
        2 as ::core::ffi::c_int,
    );
    (*pool).enabled =
        sqlite3_column_int(statement, 3 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*pool).allocation_cursor = sqlite3_column_int(statement, 4 as ::core::ffi::c_int);
    (*pool).revision = sqlite3_column_int64(statement, 5 as ::core::ffi::c_int) as int64_t;
    (*pool).created_at = sqlite3_column_int64(statement, 6 as ::core::ffi::c_int) as int64_t;
    (*pool).updated_at = sqlite3_column_int64(statement, 7 as ::core::ffi::c_int) as int64_t;
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut pool: *mut storage_pool_t,
) -> db_storage_pool_result_t {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT uuid,name,strategy,enabled,allocation_cursor,revision,created_at,updated_at FROM storage_pools WHERE uuid=? LIMIT 1;\0"
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
    if result != SQLITE_ROW {
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        return db_storage_pool_result_t(if result == SQLITE_DONE {
            db_storage_pool_result_t::DB_STORAGE_POOL_NOT_FOUND.0
        } else {
            db_storage_pool_result_t::DB_STORAGE_POOL_ERROR.0
        });
    }
    populate(statement, pool);
    sqlite3_finalize(statement);
    return load_members_locked(db, pool);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_pool_validate(
    mut pool: *mut storage_pool_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> db_storage_pool_result_t {
    if !error.is_null() && error_size != 0 {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if pool.is_null() {
        return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
    }
    let mut name: [::core::ffi::c_char; 128] = [0; 128];
    if copy_trimmed_value(
        &raw mut name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        &raw mut (*pool).name as *mut ::core::ffi::c_char,
        0 as size_t,
    ) == 0 as size_t
    {
        if !error.is_null() && error_size != 0 {
            snprintf(
                error,
                error_size,
                b"pool name cannot be blank\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
    }
    let mut cursor: *const ::core::ffi::c_uchar =
        &raw mut name as *mut ::core::ffi::c_char as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
        }
        cursor = cursor.offset(1);
    }
    safe_strcpy(
        &raw mut (*pool).name as *mut ::core::ffi::c_char,
        &raw mut name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    if strcmp(
        &raw mut (*pool).strategy as *mut ::core::ffi::c_char,
        b"most_free\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut (*pool).strategy as *mut ::core::ffi::c_char,
            b"round_robin\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut (*pool).strategy as *mut ::core::ffi::c_char,
            b"priority\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        if !error.is_null() && error_size != 0 {
            snprintf(
                error,
                error_size,
                b"strategy must be most_free, round_robin, or priority\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
    }
    if (*pool).member_count < 1 as ::core::ffi::c_int
        || (*pool).member_count > STORAGE_POOL_MEMBER_MAX
    {
        if !error.is_null() && error_size != 0 {
            snprintf(
                error,
                error_size,
                b"pool requires at least one target\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*pool).member_count {
        let mut member: *mut storage_pool_member_t =
            (&raw mut (*pool).members as *mut storage_pool_member_t).offset(index as isize);
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
        if !lightnvr_uuid_is_valid(&raw mut (*member).target_uuid as *mut ::core::ffi::c_char)
            || db_storage_target_get(
                &raw mut (*member).target_uuid as *mut ::core::ffi::c_char,
                &raw mut target,
            )
            .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
        {
            if !error.is_null() && error_size != 0 {
                snprintf(
                    error,
                    error_size,
                    b"pool target does not exist\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
        }
        if (*member).weight < 1 as ::core::ffi::c_int
            || (*member).weight > 1000 as ::core::ffi::c_int
        {
            if !error.is_null() && error_size != 0 {
                snprintf(
                    error,
                    error_size,
                    b"member weight must be 1-1000\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
        }
        (*member).position = index;
        let mut prior: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while prior < index {
            if strcmp(
                &raw mut (*member).target_uuid as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut (*pool).members as *mut storage_pool_member_t)
                    .offset(prior as isize))
                .target_uuid as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                if !error.is_null() && error_size != 0 {
                    snprintf(
                        error,
                        error_size,
                        b"pool targets must be unique\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
            }
            prior += 1;
        }
        index += 1;
    }
    return db_storage_pool_result_t::DB_STORAGE_POOL_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_pool_count() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM storage_pools;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(statement) == SQLITE_ROW
    {
        count = sqlite3_column_int(statement, 0 as ::core::ffi::c_int);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_pool_list(
    mut pools: *mut storage_pool_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if pools.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT uuid,name,strategy,enabled,allocation_cursor,revision,created_at,updated_at FROM storage_pools ORDER BY name COLLATE NOCASE,uuid;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (result == SQLITE_OK || result == SQLITE_ROW) && count < max_count {
        result = sqlite3_step(statement);
        if result != SQLITE_ROW {
            break;
        }
        populate(statement, pools.offset(count as isize));
        if load_members_locked(db, pools.offset(count as isize)).0
            != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
        {
            count = -1 as ::core::ffi::c_int;
            break;
        } else {
            count += 1;
        }
    }
    if count >= 0 as ::core::ffi::c_int && result != SQLITE_DONE && result != SQLITE_ROW {
        count = -1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_pool_get(
    mut uuid: *const ::core::ffi::c_char,
    mut pool: *mut storage_pool_t,
) -> db_storage_pool_result_t {
    if !lightnvr_uuid_is_valid(uuid) || pool.is_null() {
        return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_pool_result_t::DB_STORAGE_POOL_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_storage_pool_result_t = get_locked(db, uuid, pool);
    pthread_mutex_unlock(mutex);
    return result;
}
unsafe extern "C" fn write_members_locked(
    mut db: *mut sqlite3,
    mut pool: *const storage_pool_t,
) -> bool {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"INSERT INTO storage_pool_members(pool_uuid,target_uuid,position,weight) VALUES(?,?,?,?);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return r#false != 0;
    }
    let mut success: bool = r#true != 0;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*pool).member_count {
        sqlite3_reset(statement);
        sqlite3_clear_bindings(statement);
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw const (*pool).uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw const (*(&raw const (*pool).members as *const storage_pool_member_t)
                .offset(index as isize))
            .target_uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(statement, 3 as ::core::ffi::c_int, index);
        sqlite3_bind_int(
            statement,
            4 as ::core::ffi::c_int,
            (*pool).members[index as usize].weight,
        );
        if sqlite3_step(statement) != SQLITE_DONE {
            success = r#false != 0;
            break;
        } else {
            index += 1;
        }
    }
    sqlite3_finalize(statement);
    return success;
}
unsafe extern "C" fn sqlite_result(
    mut db: *mut sqlite3,
    mut result: ::core::ffi::c_int,
) -> db_storage_pool_result_t {
    let mut code: ::core::ffi::c_int = sqlite3_extended_errcode(db);
    return db_storage_pool_result_t(
        if result == SQLITE_CONSTRAINT
            || code == SQLITE_CONSTRAINT_UNIQUE
            || code == SQLITE_CONSTRAINT_PRIMARYKEY
        {
            db_storage_pool_result_t::DB_STORAGE_POOL_CONFLICT.0
        } else {
            db_storage_pool_result_t::DB_STORAGE_POOL_ERROR.0
        },
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_pool_create(
    mut pool: *mut storage_pool_t,
) -> db_storage_pool_result_t {
    if db_storage_pool_validate(
        pool,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
    )
    .0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
        || lightnvr_uuid_generate_v4(&raw mut (*pool).uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_pool_result_t::DB_STORAGE_POOL_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut count_statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM storage_pools;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut count_statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(count_statement) == SQLITE_ROW
    {
        count = sqlite3_column_int(count_statement, 0 as ::core::ffi::c_int);
    }
    if !count_statement.is_null() {
        sqlite3_finalize(count_statement);
    }
    if count < 0 as ::core::ffi::c_int || count >= STORAGE_POOL_MAX_COUNT {
        pthread_mutex_unlock(mutex);
        return db_storage_pool_result_t(if count < 0 as ::core::ffi::c_int {
            db_storage_pool_result_t::DB_STORAGE_POOL_ERROR.0
        } else {
            db_storage_pool_result_t::DB_STORAGE_POOL_LIMIT.0
        });
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"INSERT INTO storage_pools(uuid,name,strategy,enabled) VALUES(?,?,?,?);\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK
        && sqlite3_exec(
            db,
            b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK
    {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut (*pool).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw mut (*pool).name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw mut (*pool).strategy as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            4 as ::core::ffi::c_int,
            if (*pool).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        result = sqlite3_step(statement);
        if result == SQLITE_DONE
            && write_members_locked(db, pool) as ::core::ffi::c_int != 0
            && sqlite3_exec(
                db,
                b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ) == SQLITE_OK
        {
            result = SQLITE_DONE;
        } else {
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            if result == SQLITE_DONE {
                result = SQLITE_ERROR;
            }
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    let mut outcome: db_storage_pool_result_t =
        db_storage_pool_result_t(if result == SQLITE_DONE {
            get_locked(db, &raw mut (*pool).uuid as *mut ::core::ffi::c_char, pool).0
        } else {
            sqlite_result(db, result).0
        });
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_pool_update(
    mut pool: *mut storage_pool_t,
    mut expected_revision: int64_t,
) -> db_storage_pool_result_t {
    if pool.is_null()
        || !lightnvr_uuid_is_valid(&raw mut (*pool).uuid as *mut ::core::ffi::c_char)
        || expected_revision < 1 as int64_t
        || db_storage_pool_validate(
            pool,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as size_t,
        )
        .0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
    {
        return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_pool_result_t::DB_STORAGE_POOL_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut began: bool = sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = if began as ::core::ffi::c_int != 0 {
        sqlite3_prepare_v2(
            db,
            b"UPDATE storage_pools SET name=?,strategy=?,enabled=?,revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=? AND revision=?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        )
    } else {
        SQLITE_ERROR
    };
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut (*pool).name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw mut (*pool).strategy as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            3 as ::core::ffi::c_int,
            if (*pool).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            &raw mut (*pool).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            5 as ::core::ffi::c_int,
            expected_revision as sqlite3_int64,
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
    if changed == 1 as ::core::ffi::c_int {
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
        result = sqlite3_prepare_v2(
            db,
            b"DELETE FROM storage_pool_members WHERE pool_uuid=?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                &raw mut (*pool).uuid as *mut ::core::ffi::c_char,
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
        if result == SQLITE_DONE
            && write_members_locked(db, pool) as ::core::ffi::c_int != 0
            && sqlite3_exec(
                db,
                b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ) == SQLITE_OK
        {
            result = SQLITE_DONE;
        } else {
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            if result == SQLITE_DONE {
                result = SQLITE_ERROR;
            }
        }
    } else if began {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    let mut outcome: db_storage_pool_result_t = db_storage_pool_result_t::DB_STORAGE_POOL_OK;
    if result != SQLITE_DONE {
        outcome = sqlite_result(db, result);
    } else if changed == 0 as ::core::ffi::c_int {
        let mut existing: storage_pool_t = storage_pool_t {
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
        outcome = db_storage_pool_result_t(
            if get_locked(
                db,
                &raw mut (*pool).uuid as *mut ::core::ffi::c_char,
                &raw mut existing,
            )
            .0 == db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
            {
                db_storage_pool_result_t::DB_STORAGE_POOL_STALE.0
            } else {
                db_storage_pool_result_t::DB_STORAGE_POOL_NOT_FOUND.0
            },
        );
    } else {
        outcome = get_locked(db, &raw mut (*pool).uuid as *mut ::core::ffi::c_char, pool);
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_pool_delete(
    mut uuid: *const ::core::ffi::c_char,
    mut expected_revision: int64_t,
) -> db_storage_pool_result_t {
    if !lightnvr_uuid_is_valid(uuid) || expected_revision < 1 as int64_t {
        return db_storage_pool_result_t::DB_STORAGE_POOL_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_pool_result_t::DB_STORAGE_POOL_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM storage_pools WHERE uuid=? AND revision=?;\0".as_ptr()
            as *const ::core::ffi::c_char,
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
        sqlite3_bind_int64(
            statement,
            2 as ::core::ffi::c_int,
            expected_revision as sqlite3_int64,
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
    let mut outcome: db_storage_pool_result_t = db_storage_pool_result_t::DB_STORAGE_POOL_OK;
    if result == SQLITE_CONSTRAINT || sqlite3_extended_errcode(db) == SQLITE_CONSTRAINT_FOREIGNKEY {
        outcome = db_storage_pool_result_t::DB_STORAGE_POOL_IN_USE;
    } else if result != SQLITE_DONE {
        outcome = db_storage_pool_result_t::DB_STORAGE_POOL_ERROR;
    } else if changed == 0 as ::core::ffi::c_int {
        let mut existing: storage_pool_t = storage_pool_t {
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
        outcome = db_storage_pool_result_t(
            if get_locked(db, uuid, &raw mut existing).0
                == db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
            {
                db_storage_pool_result_t::DB_STORAGE_POOL_STALE.0
            } else {
                db_storage_pool_result_t::DB_STORAGE_POOL_NOT_FOUND.0
            },
        );
    } else {
        outcome = db_storage_pool_result_t::DB_STORAGE_POOL_OK;
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_pool_allocate(
    mut uuid: *const ::core::ffi::c_char,
    mut exclude_target_uuid: *const ::core::ffi::c_char,
    mut target: *mut storage_target_t,
) -> db_storage_pool_result_t {
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
    let mut loaded: db_storage_pool_result_t = db_storage_pool_get(uuid, &raw mut pool);
    if loaded.0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0 || target.is_null() {
        return loaded;
    }
    if !pool.enabled {
        return db_storage_pool_result_t::DB_STORAGE_POOL_UNAVAILABLE;
    }
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
        let mut candidate: storage_target_t = storage_target_t {
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
        if !(!exclude_target_uuid.is_null()
            && *exclude_target_uuid.offset(0isize) as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut pool.members as *mut storage_pool_member_t)
                    .offset(index as isize))
                .target_uuid as *mut ::core::ffi::c_char,
                exclude_target_uuid,
            ) == 0 as ::core::ffi::c_int)
        {
            if !(db_storage_target_get(
                &raw mut (*(&raw mut pool.members as *mut storage_pool_member_t)
                    .offset(index as isize))
                .target_uuid as *mut ::core::ffi::c_char,
                &raw mut candidate,
            )
            .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
                || !candidate.enabled
                || strcmp(
                    &raw mut candidate.target_type as *mut ::core::ffi::c_char,
                    b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut candidate.health_status as *mut ::core::ffi::c_char,
                    b"healthy\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                || candidate.mount_required as ::core::ffi::c_int != 0
                    && !db_storage_target_mount_guard_active(&raw mut candidate)
                || candidate.available_bytes <= candidate.reserve_bytes)
            {
                let mut free_bytes: uint64_t = candidate
                    .available_bytes
                    .wrapping_sub(candidate.reserve_bytes);
                if selected < 0 as ::core::ffi::c_int
                    || strcmp(
                        &raw mut pool.strategy as *mut ::core::ffi::c_char,
                        b"most_free\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    || free_bytes > best_free
                {
                    selected = index;
                    best_free = free_bytes;
                    *target = candidate;
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
        return db_storage_pool_result_t::DB_STORAGE_POOL_UNAVAILABLE;
    }
    if strcmp(
        &raw mut pool.strategy as *mut ::core::ffi::c_char,
        b"round_robin\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut db: *mut sqlite3 = get_db_handle();
        let mut mutex: *mut pthread_mutex_t = get_db_mutex();
        if !db.is_null() && !mutex.is_null() {
            pthread_mutex_lock(mutex);
            let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            if sqlite3_prepare_v2(
                db,
                b"UPDATE storage_pools SET allocation_cursor=? WHERE uuid=?;\0".as_ptr()
                    as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                &raw mut statement,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ) == SQLITE_OK
            {
                sqlite3_bind_int(
                    statement,
                    1 as ::core::ffi::c_int,
                    (selected + 1 as ::core::ffi::c_int) % pool.member_count,
                );
                sqlite3_bind_text(
                    statement,
                    2 as ::core::ffi::c_int,
                    &raw mut pool.uuid as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                sqlite3_step(statement);
            }
            if !statement.is_null() {
                sqlite3_finalize(statement);
            }
            pthread_mutex_unlock(mutex);
        }
    }
    return db_storage_pool_result_t::DB_STORAGE_POOL_OK;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
