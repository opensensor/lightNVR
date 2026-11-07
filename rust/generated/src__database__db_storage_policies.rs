use ::libc;
unsafe extern "C" {
    pub type fleet_selector;
    pub type sqlite3_stmt;
    pub type sqlite3;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_get(
        uuid: *const ::core::ffi::c_char,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
    unsafe fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_selector_parse(
        json: *const cJSON,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut fleet_selector_t;
    unsafe fn fleet_selector_free(selector: *mut fleet_selector_t);
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_storage_pool_get(
        uuid: *const ::core::ffi::c_char,
        pool: *mut storage_pool_t,
    ) -> db_storage_pool_result_t;
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
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
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
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type uint64_t = u64;
pub type uint_fast64_t = ::core::ffi::c_ulong;
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
pub struct storage_policy_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub enabled: bool,
    pub priority: ::core::ffi::c_int,
    pub selector_json: [::core::ffi::c_char; 8192],
    pub primary_target_uuid: [::core::ffi::c_char; 37],
    pub primary_pool_uuid: [::core::ffi::c_char; 37],
    pub fallback_mode: [::core::ffi::c_char; 16],
    pub fallback_target_uuid: [::core::ffi::c_char; 37],
    pub minimum_retention_days: ::core::ffi::c_int,
    pub desired_retention_days: ::core::ffi::c_int,
    pub maximum_retention_days: ::core::ffi::c_int,
    pub required_copy_count: ::core::ffi::c_int,
    pub replication_pool_uuid: [::core::ffi::c_char; 37],
    pub migration_after_days: ::core::ffi::c_int,
    pub migration_target_uuid: [::core::ffi::c_char; 37],
    pub pressure_priority: ::core::ffi::c_int,
    pub archive_after_seconds: ::core::ffi::c_int,
    pub hot_residency_seconds: ::core::ffi::c_int,
    pub archive_protected: bool,
    pub archive_on_pressure: bool,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_policy_result_t(pub ::core::ffi::c_int);
impl db_storage_policy_result_t {
    pub const DB_STORAGE_POLICY_OK: Self = Self(0);
    pub const DB_STORAGE_POLICY_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_POLICY_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_POLICY_INVALID: Self = Self(-3);
    pub const DB_STORAGE_POLICY_STALE: Self = Self(-4);
    pub const DB_STORAGE_POLICY_LIMIT: Self = Self(-5);
    pub const DB_STORAGE_POLICY_ERROR: Self = Self(-6);
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
pub type fleet_selector_t = fleet_selector;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type atomic_uint_fast64_t = uint_fast64_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct memory_order(pub ::core::ffi::c_uint);
impl memory_order {
    pub const memory_order_relaxed: Self = Self(0);
    pub const memory_order_consume: Self = Self(1);
    pub const memory_order_acquire: Self = Self(2);
    pub const memory_order_release: Self = Self(3);
    pub const memory_order_acq_rel: Self = Self(4);
    pub const memory_order_seq_cst: Self = Self(5);
}
pub const STORAGE_POLICY_SELECTOR_MAX: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const STORAGE_POLICY_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT_UNIQUE: ::core::ffi::c_int =
    SQLITE_CONSTRAINT | (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
static mut policy_generation: atomic_uint_fast64_t = 1 as ::core::ffi::c_ulong;
unsafe extern "C" fn set_error(
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    if error.is_null()
        || error_size == 0 as size_t
        || *error.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        return;
    }
    let mut arguments: ::core::ffi::VaList;
    arguments = c2rust_args.clone();
    vsnprintf(error, error_size, format, arguments.clone());
}
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
unsafe extern "C" fn populate(mut statement: *mut sqlite3_stmt, mut policy: *mut storage_policy_t) {
    memset(
        policy as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_policy_t>(),
    );
    copy_column(
        &raw mut (*policy).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*policy).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        1 as ::core::ffi::c_int,
    );
    (*policy).enabled =
        sqlite3_column_int(statement, 2 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*policy).priority = sqlite3_column_int(statement, 3 as ::core::ffi::c_int);
    copy_column(
        &raw mut (*policy).selector_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        statement,
        4 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*policy).primary_target_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        5 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*policy).fallback_mode as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        statement,
        6 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*policy).fallback_target_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        7 as ::core::ffi::c_int,
    );
    (*policy).revision = sqlite3_column_int64(statement, 8 as ::core::ffi::c_int) as int64_t;
    (*policy).created_at = sqlite3_column_int64(statement, 9 as ::core::ffi::c_int) as int64_t;
    (*policy).updated_at = sqlite3_column_int64(statement, 10 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*policy).primary_pool_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        11 as ::core::ffi::c_int,
    );
    (*policy).minimum_retention_days = sqlite3_column_int(statement, 12 as ::core::ffi::c_int);
    (*policy).desired_retention_days = sqlite3_column_int(statement, 13 as ::core::ffi::c_int);
    (*policy).maximum_retention_days = sqlite3_column_int(statement, 14 as ::core::ffi::c_int);
    (*policy).required_copy_count = sqlite3_column_int(statement, 15 as ::core::ffi::c_int);
    copy_column(
        &raw mut (*policy).replication_pool_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        16 as ::core::ffi::c_int,
    );
    (*policy).migration_after_days = sqlite3_column_int(statement, 17 as ::core::ffi::c_int);
    copy_column(
        &raw mut (*policy).migration_target_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        18 as ::core::ffi::c_int,
    );
    (*policy).pressure_priority = sqlite3_column_int(statement, 19 as ::core::ffi::c_int);
    (*policy).archive_after_seconds = sqlite3_column_int(statement, 20 as ::core::ffi::c_int);
    (*policy).hot_residency_seconds = sqlite3_column_int(statement, 21 as ::core::ffi::c_int);
    (*policy).archive_protected =
        sqlite3_column_int(statement, 22 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*policy).archive_on_pressure =
        sqlite3_column_int(statement, 23 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn valid_name(
    mut input: *const ::core::ffi::c_char,
    mut normalized: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    if input.is_null() || copy_trimmed_value(normalized, size, input, 0 as size_t) == 0 as size_t {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = normalized as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn normalize_selector(
    mut policy: *mut storage_policy_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    let mut json: *mut cJSON =
        cJSON_Parse(&raw mut (*policy).selector_json as *mut ::core::ffi::c_char);
    if json.is_null() {
        set_error(
            error,
            error_size,
            b"selector must be valid JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut selector_error: [::core::ffi::c_char; 256] = [
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
    ];
    let mut selector: *mut fleet_selector_t = fleet_selector_parse(
        json,
        &raw mut selector_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if selector.is_null() {
        set_error(
            error,
            error_size,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            if selector_error[0usize] as ::core::ffi::c_int != 0 {
                &raw mut selector_error as *mut ::core::ffi::c_char
            } else {
                b"invalid selector\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        cJSON_Delete(json);
        return r#false != 0;
    }
    let mut canonical: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(json);
    let mut valid: bool = !canonical.is_null()
        && strnlen(
            canonical,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        ) < ::core::mem::size_of::<[::core::ffi::c_char; 8192]>();
    if valid {
        safe_strcpy(
            &raw mut (*policy).selector_json as *mut ::core::ffi::c_char,
            canonical,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
            0 as size_t,
        );
    } else {
        set_error(
            error,
            error_size,
            b"selector exceeds %d bytes\0".as_ptr() as *const ::core::ffi::c_char,
            STORAGE_POLICY_SELECTOR_MAX - 1 as ::core::ffi::c_int,
        );
    }
    free(canonical as *mut ::core::ffi::c_void);
    fleet_selector_free(selector);
    cJSON_Delete(json);
    return valid;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_policy_validate(
    mut policy: *mut storage_policy_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> db_storage_policy_result_t {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if policy.is_null() {
        set_error(
            error,
            error_size,
            b"storage policy is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    if !valid_name(
        &raw mut (*policy).name as *mut ::core::ffi::c_char,
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    ) {
        set_error(
            error,
            error_size,
            b"policy name cannot be blank\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    safe_strcpy(
        &raw mut (*policy).name as *mut ::core::ffi::c_char,
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    if (*policy).priority < -1000000 as ::core::ffi::c_int
        || (*policy).priority > 1000000 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"priority must be between -1000000 and 1000000\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    if !normalize_selector(policy, error, error_size) {
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
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
    if !lightnvr_uuid_is_valid(&raw mut (*policy).primary_target_uuid as *mut ::core::ffi::c_char)
        || db_storage_target_get(
            &raw mut (*policy).primary_target_uuid as *mut ::core::ffi::c_char,
            &raw mut target,
        )
        .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
        || strcmp(
            &raw mut target.target_type as *mut ::core::ffi::c_char,
            b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"primary target does not exist\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    if (*policy).primary_pool_uuid[0usize] != 0 {
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
        if !lightnvr_uuid_is_valid(&raw mut (*policy).primary_pool_uuid as *mut ::core::ffi::c_char)
            || db_storage_pool_get(
                &raw mut (*policy).primary_pool_uuid as *mut ::core::ffi::c_char,
                &raw mut pool,
            )
            .0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
        {
            set_error(
                error,
                error_size,
                b"primary pool does not exist\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
        }
    }
    let mut named_fallback: bool = strcmp(
        &raw mut (*policy).fallback_mode as *mut ::core::ffi::c_char,
        b"target\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut valid_mode: bool = named_fallback as ::core::ffi::c_int != 0
        || strcmp(
            &raw mut (*policy).fallback_mode as *mut ::core::ffi::c_char,
            b"default\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw mut (*policy).fallback_mode as *mut ::core::ffi::c_char,
            b"pause\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw mut (*policy).fallback_mode as *mut ::core::ffi::c_char,
            b"fail\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    if !valid_mode {
        set_error(
            error,
            error_size,
            b"fallback_mode must be default, target, pause, or fail\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    if named_fallback {
        if !lightnvr_uuid_is_valid(
            &raw mut (*policy).fallback_target_uuid as *mut ::core::ffi::c_char,
        ) || strcmp(
            &raw mut (*policy).fallback_target_uuid as *mut ::core::ffi::c_char,
            &raw mut (*policy).primary_target_uuid as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || db_storage_target_get(
                &raw mut (*policy).fallback_target_uuid as *mut ::core::ffi::c_char,
                &raw mut target,
            )
            .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
            || strcmp(
                &raw mut target.target_type as *mut ::core::ffi::c_char,
                b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            set_error(
                error,
                error_size,
                b"fallback target must exist and differ from primary\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
        }
    } else if (*policy).fallback_target_uuid[0usize] as ::core::ffi::c_int
        != '\0' as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"fallback_target_uuid is only valid for target fallback\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    if (*policy).minimum_retention_days < 0 as ::core::ffi::c_int
        || (*policy).desired_retention_days < 0 as ::core::ffi::c_int
        || (*policy).maximum_retention_days < 0 as ::core::ffi::c_int
        || (*policy).minimum_retention_days > 36500 as ::core::ffi::c_int
        || (*policy).desired_retention_days > 36500 as ::core::ffi::c_int
        || (*policy).maximum_retention_days > 36500 as ::core::ffi::c_int
        || (*policy).desired_retention_days > 0 as ::core::ffi::c_int
            && (*policy).minimum_retention_days > (*policy).desired_retention_days
        || (*policy).maximum_retention_days > 0 as ::core::ffi::c_int
            && ((*policy).desired_retention_days > 0 as ::core::ffi::c_int
                && (*policy).desired_retention_days > (*policy).maximum_retention_days
                || (*policy).minimum_retention_days > (*policy).maximum_retention_days)
    {
        set_error(
            error,
            error_size,
            b"retention days must satisfy minimum <= desired <= maximum\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    if (*policy).required_copy_count == 0 as ::core::ffi::c_int {
        (*policy).required_copy_count = 1 as ::core::ffi::c_int;
    }
    if (*policy).required_copy_count < 1 as ::core::ffi::c_int
        || (*policy).required_copy_count > 8 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"required_copy_count must be 1-8\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    if (*policy).required_copy_count > 1 as ::core::ffi::c_int {
        let mut pool_0: storage_pool_t = storage_pool_t {
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
        if !lightnvr_uuid_is_valid(
            &raw mut (*policy).replication_pool_uuid as *mut ::core::ffi::c_char,
        ) || db_storage_pool_get(
            &raw mut (*policy).replication_pool_uuid as *mut ::core::ffi::c_char,
            &raw mut pool_0,
        )
        .0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
        {
            set_error(
                error,
                error_size,
                b"replication pool cannot satisfy required copy count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
        }
        let mut usable_members: ::core::ffi::c_int = pool_0.member_count;
        let mut may_contain_primary: bool = r#false != 0;
        if (*policy).primary_pool_uuid[0usize] != 0 {
            let mut primary_pool: storage_pool_t = storage_pool_t {
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
                &raw mut (*policy).primary_pool_uuid as *mut ::core::ffi::c_char,
                &raw mut primary_pool,
            )
            .0 == db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
            {
                let mut replica: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while replica < pool_0.member_count {
                    let mut primary: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while primary < primary_pool.member_count {
                        if strcmp(
                            &raw mut (*(&raw mut pool_0.members as *mut storage_pool_member_t)
                                .offset(replica as isize))
                            .target_uuid as *mut ::core::ffi::c_char,
                            &raw mut (*(&raw mut primary_pool.members
                                as *mut storage_pool_member_t)
                                .offset(primary as isize))
                            .target_uuid as *mut ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            may_contain_primary = r#true != 0;
                        }
                        primary += 1;
                    }
                    replica += 1;
                }
            }
        } else {
            let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while index < pool_0.member_count {
                if strcmp(
                    &raw mut (*(&raw mut pool_0.members as *mut storage_pool_member_t)
                        .offset(index as isize))
                    .target_uuid as *mut ::core::ffi::c_char,
                    &raw mut (*policy).primary_target_uuid as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    usable_members -= 1;
                }
                index += 1;
            }
        }
        if (*policy).primary_pool_uuid[0usize] as ::core::ffi::c_int != 0
            && may_contain_primary as ::core::ffi::c_int != 0
            && pool_0.member_count < (*policy).required_copy_count
            || (*policy).primary_pool_uuid[0usize] == 0
                && usable_members < (*policy).required_copy_count - 1 as ::core::ffi::c_int
            || (*policy).primary_pool_uuid[0usize] as ::core::ffi::c_int != 0
                && !may_contain_primary
                && pool_0.member_count < (*policy).required_copy_count - 1 as ::core::ffi::c_int
        {
            set_error(
                error,
                error_size,
                b"replication pool cannot satisfy distinct copy count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
        }
    } else if (*policy).replication_pool_uuid[0usize] != 0 {
        set_error(
            error,
            error_size,
            b"replication_pool_uuid requires more than one copy\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    if (*policy).migration_after_days < 0 as ::core::ffi::c_int
        || (*policy).migration_after_days > 36500 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"migration_after_days must be 0-36500\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    if (*policy).archive_after_seconds < -1 as ::core::ffi::c_int
        || (*policy).hot_residency_seconds < -1 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"Archive ages must be -1 (disabled/inherited) or nonnegative seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    if (*policy).migration_after_days > 0 as ::core::ffi::c_int
        || (*policy).migration_target_uuid[0usize] as ::core::ffi::c_int != 0
    {
        if !lightnvr_uuid_is_valid(
            &raw mut (*policy).migration_target_uuid as *mut ::core::ffi::c_char,
        ) || db_storage_target_get(
            &raw mut (*policy).migration_target_uuid as *mut ::core::ffi::c_char,
            &raw mut target,
        )
        .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
        {
            set_error(
                error,
                error_size,
                b"migration target does not exist\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
        }
        if strcmp(
            &raw mut (*policy).migration_target_uuid as *mut ::core::ffi::c_char,
            &raw mut (*policy).primary_target_uuid as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            set_error(
                error,
                error_size,
                b"migration target must differ from initial target\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
        }
        let mut pool_uuids: [*const ::core::ffi::c_char; 2] = [
            &raw mut (*policy).primary_pool_uuid as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_char,
            &raw mut (*policy).replication_pool_uuid as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_char,
        ];
        let mut pool_index: size_t = 0 as size_t;
        while pool_index < 2 as size_t {
            if *pool_uuids[pool_index].offset(0isize) != 0 {
                let mut pool_1: storage_pool_t = storage_pool_t {
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
                if db_storage_pool_get(pool_uuids[pool_index], &raw mut pool_1).0
                    == db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
                {
                    let mut member: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while member < pool_1.member_count {
                        if strcmp(
                            &raw mut (*(&raw mut pool_1.members as *mut storage_pool_member_t)
                                .offset(member as isize))
                            .target_uuid as *mut ::core::ffi::c_char,
                            &raw mut (*policy).migration_target_uuid as *mut ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            set_error(
                                error,
                                error_size,
                                b"migration target must not be an initial or replica pool member\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
                        }
                        member += 1;
                    }
                }
            }
            pool_index = pool_index.wrapping_add(1);
        }
    } else if (*policy).archive_protected as ::core::ffi::c_int != 0
        || (*policy).archive_on_pressure as ::core::ffi::c_int != 0
        || (*policy).archive_after_seconds >= 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"archive rules require a migration_target_uuid\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    if (*policy).pressure_priority < -1000000 as ::core::ffi::c_int
        || (*policy).pressure_priority > 1000000 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"pressure_priority must be between -1000000 and 1000000\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    return db_storage_policy_result_t::DB_STORAGE_POLICY_OK;
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut policy: *mut storage_policy_t,
) -> db_storage_policy_result_t {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT uuid,name,enabled,priority,selector_json,primary_target_uuid,fallback_mode,COALESCE(fallback_target_uuid,''),revision,created_at,updated_at,COALESCE(primary_pool_uuid,''),minimum_retention_days,desired_retention_days,maximum_retention_days,required_copy_count,COALESCE(replication_pool_uuid,''),migration_after_days,COALESCE(migration_target_uuid,''),pressure_priority,archive_after_seconds,hot_residency_seconds,archive_protected,archive_on_pressure FROM storage_policies WHERE uuid=? LIMIT 1;\0"
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
    let mut outcome: db_storage_policy_result_t =
        db_storage_policy_result_t::DB_STORAGE_POLICY_NOT_FOUND;
    if result == SQLITE_ROW {
        populate(statement, policy);
        outcome = db_storage_policy_result_t::DB_STORAGE_POLICY_OK;
    } else if result != SQLITE_DONE {
        outcome = db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_policy_count() -> ::core::ffi::c_int {
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
        b"SELECT count(*) FROM storage_policies;\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn db_storage_policy_list(
    mut policies: *mut storage_policy_t,
    mut max_count: ::core::ffi::c_int,
    mut enabled_only: bool,
) -> ::core::ffi::c_int {
    if policies.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = if enabled_only as ::core::ffi::c_int != 0 {
        b"SELECT uuid,name,enabled,priority,selector_json,primary_target_uuid,fallback_mode,COALESCE(fallback_target_uuid,''),revision,created_at,updated_at,COALESCE(primary_pool_uuid,''),minimum_retention_days,desired_retention_days,maximum_retention_days,required_copy_count,COALESCE(replication_pool_uuid,''),migration_after_days,COALESCE(migration_target_uuid,''),pressure_priority,archive_after_seconds,hot_residency_seconds,archive_protected,archive_on_pressure FROM storage_policies WHERE enabled=1 ORDER BY priority DESC,name COLLATE NOCASE,uuid;\0"
            .as_ptr() as *const ::core::ffi::c_char
    } else {
        b"SELECT uuid,name,enabled,priority,selector_json,primary_target_uuid,fallback_mode,COALESCE(fallback_target_uuid,''),revision,created_at,updated_at,COALESCE(primary_pool_uuid,''),minimum_retention_days,desired_retention_days,maximum_retention_days,required_copy_count,COALESCE(replication_pool_uuid,''),migration_after_days,COALESCE(migration_target_uuid,''),pressure_priority,archive_after_seconds,hot_residency_seconds,archive_protected,archive_on_pressure FROM storage_policies ORDER BY priority DESC,name COLLATE NOCASE,uuid;\0"
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
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if result == SQLITE_OK {
        while count < max_count && {
            result = sqlite3_step(statement);
            result == SQLITE_ROW
        } {
            let c2rust_fresh0 = count;
            count += 1;
            populate(statement, policies.offset(c2rust_fresh0 as isize));
        }
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
pub unsafe extern "C" fn db_storage_policy_get(
    mut uuid: *const ::core::ffi::c_char,
    mut policy: *mut storage_policy_t,
) -> db_storage_policy_result_t {
    if !lightnvr_uuid_is_valid(uuid) || policy.is_null() {
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_storage_policy_result_t = get_locked(db, uuid, policy);
    pthread_mutex_unlock(mutex);
    return result;
}
unsafe extern "C" fn sqlite_result(
    mut db: *mut sqlite3,
    mut result: ::core::ffi::c_int,
) -> db_storage_policy_result_t {
    if result == SQLITE_CONSTRAINT || sqlite3_extended_errcode(db) == SQLITE_CONSTRAINT_UNIQUE {
        return db_storage_policy_result_t::DB_STORAGE_POLICY_CONFLICT;
    }
    return db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_policy_create(
    mut policy: *mut storage_policy_t,
) -> db_storage_policy_result_t {
    let mut error: [::core::ffi::c_char; 256] = [
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
    ];
    if db_storage_policy_validate(
        policy,
        &raw mut error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    )
    .0 != db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0
    {
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    if lightnvr_uuid_generate_v4(&raw mut (*policy).uuid as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut count_statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM storage_policies;\0".as_ptr() as *const ::core::ffi::c_char,
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
    if count < 0 as ::core::ffi::c_int || count >= STORAGE_POLICY_MAX_COUNT {
        pthread_mutex_unlock(mutex);
        return db_storage_policy_result_t(if count < 0 as ::core::ffi::c_int {
            db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR.0
        } else {
            db_storage_policy_result_t::DB_STORAGE_POLICY_LIMIT.0
        });
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO storage_policies(uuid,name,enabled,priority,selector_json,primary_target_uuid,fallback_mode,fallback_target_uuid,primary_pool_uuid,minimum_retention_days,desired_retention_days,maximum_retention_days,required_copy_count,replication_pool_uuid,migration_after_days,migration_target_uuid,pressure_priority,archive_after_seconds,hot_residency_seconds,archive_protected,archive_on_pressure) VALUES(?,?,?,?,?,?,?,NULLIF(?,''),NULLIF(?,''),?,?,?,?,NULLIF(?,''),?,NULLIF(?,''),?,?,?,?,?);\0"
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
            &raw mut (*policy).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw mut (*policy).name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            3 as ::core::ffi::c_int,
            if (*policy).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int(statement, 4 as ::core::ffi::c_int, (*policy).priority);
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            &raw mut (*policy).selector_json as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            &raw mut (*policy).primary_target_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            7 as ::core::ffi::c_int,
            &raw mut (*policy).fallback_mode as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            &raw mut (*policy).fallback_target_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            9 as ::core::ffi::c_int,
            &raw mut (*policy).primary_pool_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            10 as ::core::ffi::c_int,
            (*policy).minimum_retention_days,
        );
        sqlite3_bind_int(
            statement,
            11 as ::core::ffi::c_int,
            (*policy).desired_retention_days,
        );
        sqlite3_bind_int(
            statement,
            12 as ::core::ffi::c_int,
            (*policy).maximum_retention_days,
        );
        sqlite3_bind_int(
            statement,
            13 as ::core::ffi::c_int,
            (*policy).required_copy_count,
        );
        sqlite3_bind_text(
            statement,
            14 as ::core::ffi::c_int,
            &raw mut (*policy).replication_pool_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            15 as ::core::ffi::c_int,
            (*policy).migration_after_days,
        );
        sqlite3_bind_text(
            statement,
            16 as ::core::ffi::c_int,
            &raw mut (*policy).migration_target_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            17 as ::core::ffi::c_int,
            (*policy).pressure_priority,
        );
        sqlite3_bind_int(
            statement,
            18 as ::core::ffi::c_int,
            (*policy).archive_after_seconds,
        );
        sqlite3_bind_int(
            statement,
            19 as ::core::ffi::c_int,
            (*policy).hot_residency_seconds,
        );
        sqlite3_bind_int(
            statement,
            20 as ::core::ffi::c_int,
            (*policy).archive_protected as ::core::ffi::c_int,
        );
        sqlite3_bind_int(
            statement,
            21 as ::core::ffi::c_int,
            (*policy).archive_on_pressure as ::core::ffi::c_int,
        );
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    let mut outcome: db_storage_policy_result_t =
        db_storage_policy_result_t(if result == SQLITE_DONE {
            db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0
        } else {
            sqlite_result(db, result).0
        });
    if outcome.0 == db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0 {
        outcome = get_locked(
            db,
            &raw mut (*policy).uuid as *mut ::core::ffi::c_char,
            policy,
        );
    }
    pthread_mutex_unlock(mutex);
    if outcome.0 == db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0 {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
            &raw mut policy_generation,
            1 as uint_fast64_t,
        );
    }
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_policy_update(
    mut policy: *mut storage_policy_t,
    mut expected_revision: int64_t,
) -> db_storage_policy_result_t {
    let mut error: [::core::ffi::c_char; 256] = [
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
    ];
    if !lightnvr_uuid_is_valid(if !policy.is_null() {
        &raw mut (*policy).uuid as *mut ::core::ffi::c_char
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }) || expected_revision < 1 as int64_t
        || db_storage_policy_validate(
            policy,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        )
        .0 != db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0
    {
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE storage_policies SET name=?,enabled=?,priority=?,selector_json=?,primary_target_uuid=?,fallback_mode=?,fallback_target_uuid=NULLIF(?,''),primary_pool_uuid=NULLIF(?,''),minimum_retention_days=?,desired_retention_days=?,maximum_retention_days=?,required_copy_count=?,replication_pool_uuid=NULLIF(?,''),migration_after_days=?,migration_target_uuid=NULLIF(?,''),pressure_priority=?,revision=revision+1,archive_after_seconds=?19,hot_residency_seconds=?20,archive_protected=?21,archive_on_pressure=?22,updated_at=strftime('%s','now') WHERE uuid=?17 AND revision=?18;\0"
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
            &raw mut (*policy).name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            2 as ::core::ffi::c_int,
            if (*policy).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_int(statement, 3 as ::core::ffi::c_int, (*policy).priority);
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            &raw mut (*policy).selector_json as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            &raw mut (*policy).primary_target_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            &raw mut (*policy).fallback_mode as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            7 as ::core::ffi::c_int,
            &raw mut (*policy).fallback_target_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            &raw mut (*policy).primary_pool_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            9 as ::core::ffi::c_int,
            (*policy).minimum_retention_days,
        );
        sqlite3_bind_int(
            statement,
            10 as ::core::ffi::c_int,
            (*policy).desired_retention_days,
        );
        sqlite3_bind_int(
            statement,
            11 as ::core::ffi::c_int,
            (*policy).maximum_retention_days,
        );
        sqlite3_bind_int(
            statement,
            12 as ::core::ffi::c_int,
            (*policy).required_copy_count,
        );
        sqlite3_bind_text(
            statement,
            13 as ::core::ffi::c_int,
            &raw mut (*policy).replication_pool_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            14 as ::core::ffi::c_int,
            (*policy).migration_after_days,
        );
        sqlite3_bind_text(
            statement,
            15 as ::core::ffi::c_int,
            &raw mut (*policy).migration_target_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            16 as ::core::ffi::c_int,
            (*policy).pressure_priority,
        );
        sqlite3_bind_text(
            statement,
            17 as ::core::ffi::c_int,
            &raw mut (*policy).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            18 as ::core::ffi::c_int,
            expected_revision as sqlite3_int64,
        );
        sqlite3_bind_int(
            statement,
            19 as ::core::ffi::c_int,
            (*policy).archive_after_seconds,
        );
        sqlite3_bind_int(
            statement,
            20 as ::core::ffi::c_int,
            (*policy).hot_residency_seconds,
        );
        sqlite3_bind_int(
            statement,
            21 as ::core::ffi::c_int,
            (*policy).archive_protected as ::core::ffi::c_int,
        );
        sqlite3_bind_int(
            statement,
            22 as ::core::ffi::c_int,
            (*policy).archive_on_pressure as ::core::ffi::c_int,
        );
        result = sqlite3_step(statement);
    }
    let mut outcome: db_storage_policy_result_t = db_storage_policy_result_t::DB_STORAGE_POLICY_OK;
    if result != SQLITE_DONE {
        outcome = sqlite_result(db, result);
    } else if sqlite3_changes(db) == 0 as ::core::ffi::c_int {
        let mut existing: storage_policy_t = storage_policy_t {
            uuid: [0; 37],
            name: [0; 128],
            enabled: false,
            priority: 0,
            selector_json: [0; 8192],
            primary_target_uuid: [0; 37],
            primary_pool_uuid: [0; 37],
            fallback_mode: [0; 16],
            fallback_target_uuid: [0; 37],
            minimum_retention_days: 0,
            desired_retention_days: 0,
            maximum_retention_days: 0,
            required_copy_count: 0,
            replication_pool_uuid: [0; 37],
            migration_after_days: 0,
            migration_target_uuid: [0; 37],
            pressure_priority: 0,
            archive_after_seconds: 0,
            hot_residency_seconds: 0,
            archive_protected: false,
            archive_on_pressure: false,
            revision: 0,
            created_at: 0,
            updated_at: 0,
        };
        outcome = db_storage_policy_result_t(
            if get_locked(
                db,
                &raw mut (*policy).uuid as *mut ::core::ffi::c_char,
                &raw mut existing,
            )
            .0 == db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0
            {
                db_storage_policy_result_t::DB_STORAGE_POLICY_STALE.0
            } else {
                db_storage_policy_result_t::DB_STORAGE_POLICY_NOT_FOUND.0
            },
        );
    } else {
        outcome = get_locked(
            db,
            &raw mut (*policy).uuid as *mut ::core::ffi::c_char,
            policy,
        );
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if outcome.0 == db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0 {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
            &raw mut policy_generation,
            1 as uint_fast64_t,
        );
    }
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_policy_delete(
    mut uuid: *const ::core::ffi::c_char,
    mut expected_revision: int64_t,
) -> db_storage_policy_result_t {
    if !lightnvr_uuid_is_valid(uuid) || expected_revision < 1 as int64_t {
        return db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM storage_policies WHERE uuid=? AND revision=?;\0".as_ptr()
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
    let mut outcome: db_storage_policy_result_t = db_storage_policy_result_t::DB_STORAGE_POLICY_OK;
    if result != SQLITE_DONE {
        outcome = db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR;
    } else if sqlite3_changes(db) == 0 as ::core::ffi::c_int {
        let mut existing: storage_policy_t = storage_policy_t {
            uuid: [0; 37],
            name: [0; 128],
            enabled: false,
            priority: 0,
            selector_json: [0; 8192],
            primary_target_uuid: [0; 37],
            primary_pool_uuid: [0; 37],
            fallback_mode: [0; 16],
            fallback_target_uuid: [0; 37],
            minimum_retention_days: 0,
            desired_retention_days: 0,
            maximum_retention_days: 0,
            required_copy_count: 0,
            replication_pool_uuid: [0; 37],
            migration_after_days: 0,
            migration_target_uuid: [0; 37],
            pressure_priority: 0,
            archive_after_seconds: 0,
            hot_residency_seconds: 0,
            archive_protected: false,
            archive_on_pressure: false,
            revision: 0,
            created_at: 0,
            updated_at: 0,
        };
        outcome = db_storage_policy_result_t(
            if get_locked(db, uuid, &raw mut existing).0
                == db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0
            {
                db_storage_policy_result_t::DB_STORAGE_POLICY_STALE.0
            } else {
                db_storage_policy_result_t::DB_STORAGE_POLICY_NOT_FOUND.0
            },
        );
    } else {
        outcome = db_storage_policy_result_t::DB_STORAGE_POLICY_OK;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if outcome.0 == db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0 {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
            &raw mut policy_generation,
            1 as uint_fast64_t,
        );
    }
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_policy_generation() -> uint64_t {
    return ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut policy_generation,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
