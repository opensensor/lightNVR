use ::libc;
unsafe extern "C" {
    pub type fleet_selector;
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn event_registry_find(
        r#type: *const ::core::ffi::c_char,
    ) -> *const event_type_definition_t;
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
    unsafe fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn system_health_condition_from_code(
        code: *const ::core::ffi::c_char,
        condition_out: *mut system_health_condition_t,
    ) -> bool;
    unsafe fn fleet_selector_parse(
        json: *const cJSON,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut fleet_selector_t;
    unsafe fn fleet_selector_free(selector: *mut fleet_selector_t);
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_event_destination_key_exists(key: *const ::core::ffi::c_char) -> bool;
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
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type uint64_t = u64;
pub type uint_fast64_t = ::core::ffi::c_ulong;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_severity_t(pub ::core::ffi::c_uint);
impl event_severity_t {
    pub const EVENT_SEVERITY_INFO: Self = Self(0);
    pub const EVENT_SEVERITY_WARNING: Self = Self(1);
    pub const EVENT_SEVERITY_ERROR: Self = Self(2);
    pub const EVENT_SEVERITY_CRITICAL: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_sensitivity_t(pub ::core::ffi::c_uint);
impl event_sensitivity_t {
    pub const EVENT_SENSITIVITY_OPERATIONAL: Self = Self(0);
    pub const EVENT_SENSITIVITY_INTERNAL: Self = Self(1);
    pub const EVENT_SENSITIVITY_RESTRICTED: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_media_policy_t(pub ::core::ffi::c_uint);
impl event_media_policy_t {
    pub const EVENT_MEDIA_FORBIDDEN: Self = Self(0);
    pub const EVENT_MEDIA_REFERENCE_ALLOWED: Self = Self(1);
    pub const EVENT_MEDIA_BINARY_OPT_IN: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_expected_rate_t(pub ::core::ffi::c_uint);
impl event_expected_rate_t {
    pub const EVENT_RATE_LOW: Self = Self(0);
    pub const EVENT_RATE_MEDIUM: Self = Self(1);
    pub const EVENT_RATE_HIGH: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_subject_kind_t(pub ::core::ffi::c_uint);
impl event_subject_kind_t {
    pub const EVENT_SUBJECT_CAMERA: Self = Self(0);
    pub const EVENT_SUBJECT_STORAGE: Self = Self(1);
    pub const EVENT_SUBJECT_SYSTEM: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_type_definition_t {
    pub r#type: *const ::core::ffi::c_char,
    pub family: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub severity: event_severity_t,
    pub dynamic_severity: bool,
    pub minimum_severity: event_severity_t,
    pub maximum_severity: event_severity_t,
    pub sensitivity: event_sensitivity_t,
    pub media_policy: event_media_policy_t,
    pub expected_rate: event_expected_rate_t,
    pub subject_kind: event_subject_kind_t,
    pub default_expiry_seconds: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_route_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub destination_key: [::core::ffi::c_char; 128],
    pub scope_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub predicate_json: [::core::ffi::c_char; 4096],
    pub schedule_json: [::core::ffi::c_char; 8192],
    pub event_types: [[::core::ffi::c_char; 96]; 32],
    pub event_type_count: ::core::ffi::c_int,
    pub debounce_seconds: ::core::ffi::c_int,
    pub cooldown_seconds: ::core::ffi::c_int,
    pub grouping_window_seconds: ::core::ffi::c_int,
    pub max_events_per_minute: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_event_route_result_t(pub ::core::ffi::c_int);
impl db_event_route_result_t {
    pub const DB_EVENT_ROUTE_OK: Self = Self(0);
    pub const DB_EVENT_ROUTE_NOT_FOUND: Self = Self(-1);
    pub const DB_EVENT_ROUTE_CONFLICT: Self = Self(-2);
    pub const DB_EVENT_ROUTE_INVALID: Self = Self(-3);
    pub const DB_EVENT_ROUTE_STALE: Self = Self(-4);
    pub const DB_EVENT_ROUTE_LIMIT: Self = Self(-5);
    pub const DB_EVENT_ROUTE_ERROR: Self = Self(-6);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_condition_t(pub ::core::ffi::c_uint);
impl system_health_condition_t {
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW: Self = Self(0);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL: Self = Self(1);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_SWAP_THRASH: Self = Self(2);
    pub const SYSTEM_HEALTH_CONDITION_CPU_SATURATION: Self = Self(3);
    pub const SYSTEM_HEALTH_CONDITION_CPU_THROTTLED: Self = Self(4);
    pub const SYSTEM_HEALTH_CONDITION_IO_PRESSURE: Self = Self(5);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW: Self = Self(6);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW: Self = Self(7);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY: Self = Self(8);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED: Self = Self(9);
    pub const SYSTEM_HEALTH_CONDITION_THERMAL_HIGH: Self = Self(10);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN: Self = Self(11);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE: Self = Self(12);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED: Self = Self(13);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_JUMP: Self = Self(14);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION: Self = Self(15);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION: Self = Self(16);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED: Self = Self(17);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL: Self = Self(18);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL: Self = Self(19);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_CORRECTED: Self = Self(20);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE: Self = Self(21);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED: Self = Self(22);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_POWER_UNSTABLE: Self = Self(23);
    pub const SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE: Self = Self(24);
    pub const SYSTEM_HEALTH_CONDITION_UNEXPECTED_RESTART: Self = Self(25);
    pub const SYSTEM_HEALTH_CONDITION_EVENT_DELIVERY_DEGRADED: Self = Self(26);
    pub const SYSTEM_HEALTH_CONDITION_COUNT: Self = Self(27);
}
pub type fleet_selector_t = fleet_selector;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
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
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const EVENT_TYPE_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const EVENT_ROUTE_UUID_MAX: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const EVENT_ROUTE_MAX_TYPES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const EVENT_ROUTE_MAX_COUNT: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const EVENT_ROUTE_DEFAULT_DESTINATION: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"mqtt:default\0") };
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
static mut ROUTE_GENERATION: atomic_uint_fast64_t = 1 as ::core::ffi::c_ulong;
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
unsafe extern "C" fn valid_uuid(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null()
        || strlen(value) != (EVENT_ROUTE_UUID_MAX - 1 as ::core::ffi::c_int) as size_t
    {
        return r#false != 0;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < EVENT_ROUTE_UUID_MAX - 1 as ::core::ffi::c_int {
        let mut character: ::core::ffi::c_uchar =
            *value.offset(index as isize) as ::core::ffi::c_uchar;
        if index == 8 as ::core::ffi::c_int
            || index == 13 as ::core::ffi::c_int
            || index == 18 as ::core::ffi::c_int
            || index == 23 as ::core::ffi::c_int
        {
            if character as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                return r#false != 0;
            }
        } else if *(*__ctype_b_loc()).offset(character as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISxdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return r#false != 0;
        }
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_text(
    mut value: *const ::core::ffi::c_char,
    mut maximum: size_t,
    mut required: bool,
) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strnlen(value, maximum);
    if length == maximum || required as ::core::ffi::c_int != 0 && length == 0 as size_t {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = value as *const ::core::ffi::c_uchar;
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
unsafe extern "C" fn key_allowed(
    mut key: *const ::core::ffi::c_char,
    mut allowed: *const *const ::core::ffi::c_char,
    mut allowed_count: size_t,
) -> bool {
    if key.is_null() {
        return r#false != 0;
    }
    let mut index: size_t = 0 as size_t;
    while index < allowed_count {
        if strcmp(key, *allowed.offset(index as isize)) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        index = index.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn validate_object_keys(
    mut object: *const cJSON,
    mut allowed: *const *const ::core::ffi::c_char,
    mut allowed_count: size_t,
    mut context: *const ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    let mut item: *const cJSON = if !object.is_null() {
        (*object).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !item.is_null() {
        if !key_allowed((*item).string, allowed, allowed_count) {
            set_error(
                error,
                error_size,
                b"%s contains unknown field '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                context,
                if !(*item).string.is_null() {
                    (*item).string
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            return r#false != 0;
        }
        item = (*item).next;
    }
    return r#true != 0;
}
unsafe extern "C" fn version_is_one(
    mut object: *const cJSON,
    mut context: *const ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    let mut version: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        object,
        b"version\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsNumber(version) == 0 || (*version).valuedouble != 1.0f64 {
        set_error(
            error,
            error_size,
            b"%s.version must be 1\0".as_ptr() as *const ::core::ffi::c_char,
            context,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_string_array(
    mut array: *const cJSON,
    mut field: *const ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    let mut count: ::core::ffi::c_int = if cJSON_IsArray(array) != 0 {
        cJSON_GetArraySize(array)
    } else {
        -1 as ::core::ffi::c_int
    };
    if count < 1 as ::core::ffi::c_int || count > 64 as ::core::ffi::c_int {
        set_error(
            error,
            error_size,
            b"%s must contain 1-64 values\0".as_ptr() as *const ::core::ffi::c_char,
            field,
        );
        return r#false != 0;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *const cJSON = cJSON_GetArrayItem(array, index);
        if cJSON_IsString(item) == 0 || !valid_text((*item).valuestring, 128 as size_t, r#true != 0)
        {
            set_error(
                error,
                error_size,
                b"%s contains an invalid value\0".as_ptr() as *const ::core::ffi::c_char,
                field,
            );
            return r#false != 0;
        }
        let mut previous: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while previous < index {
            let mut other: *const cJSON = cJSON_GetArrayItem(array, previous);
            if strcasecmp((*item).valuestring, (*other).valuestring) == 0 as ::core::ffi::c_int {
                set_error(
                    error,
                    error_size,
                    b"%s contains a duplicate value\0".as_ptr() as *const ::core::ffi::c_char,
                    field,
                );
                return r#false != 0;
            }
            previous += 1;
        }
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn route_has_type(
    mut route: *const event_route_t,
    mut r#type: *const ::core::ffi::c_char,
) -> bool {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*route).event_type_count {
        if strcmp(
            &raw const *(&raw const (*route).event_types as *const [::core::ffi::c_char; 96])
                .offset(index as isize) as *const ::core::ffi::c_char,
            r#type,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        index += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn route_types_are_family(
    mut route: *const event_route_t,
    mut family: *const ::core::ffi::c_char,
) -> bool {
    if route.is_null() || family.is_null() || (*route).event_type_count < 1 as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*route).event_type_count {
        let mut definition: *const event_type_definition_t = event_registry_find(
            &raw const *(&raw const (*route).event_types as *const [::core::ffi::c_char; 96])
                .offset(index as isize) as *const ::core::ffi::c_char,
        );
        if definition.is_null() || strcmp((*definition).family, family) != 0 as ::core::ffi::c_int {
            return r#false != 0;
        }
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn route_types_are_camera_subjects(mut route: *const event_route_t) -> bool {
    if route.is_null() || (*route).event_type_count < 1 as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*route).event_type_count {
        let mut definition: *const event_type_definition_t = event_registry_find(
            &raw const *(&raw const (*route).event_types as *const [::core::ffi::c_char; 96])
                .offset(index as isize) as *const ::core::ffi::c_char,
        );
        if definition.is_null()
            || (*definition).subject_kind.0 != event_subject_kind_t::EVENT_SUBJECT_CAMERA.0
        {
            return r#false != 0;
        }
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_health_severity(mut value: *const ::core::ffi::c_char) -> bool {
    return !value.is_null()
        && (strcmp(value, b"warning\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(value, b"error\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(value, b"critical\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn valid_health_condition_array(
    mut array: *const cJSON,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    let mut field: *const ::core::ffi::c_char =
        b"predicate.health.condition_codes_any\0".as_ptr() as *const ::core::ffi::c_char;
    if !valid_string_array(array, field, error, error_size) {
        return r#false != 0;
    }
    let mut count: ::core::ffi::c_int = cJSON_GetArraySize(array);
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut condition: system_health_condition_t =
            system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW;
        let mut item: *const cJSON = cJSON_GetArrayItem(array, index);
        if !system_health_condition_from_code((*item).valuestring, &raw mut condition) {
            set_error(
                error,
                error_size,
                b"%s contains unknown value '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                field,
                (*item).valuestring,
            );
            return r#false != 0;
        }
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_health_severity_array(
    mut array: *const cJSON,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    let mut field: *const ::core::ffi::c_char =
        b"predicate.health.severities_any\0".as_ptr() as *const ::core::ffi::c_char;
    if !valid_string_array(array, field, error, error_size) {
        return r#false != 0;
    }
    let mut count: ::core::ffi::c_int = cJSON_GetArraySize(array);
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *const cJSON = cJSON_GetArrayItem(array, index);
        if !valid_health_severity((*item).valuestring) {
            set_error(
                error,
                error_size,
                b"%s contains unknown value '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                field,
                (*item).valuestring,
            );
            return r#false != 0;
        }
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_predicate(
    mut route: *const event_route_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    let mut root: *mut cJSON =
        cJSON_Parse(&raw const (*route).predicate_json as *const ::core::ffi::c_char);
    let root_fields: [*const ::core::ffi::c_char; 3] = [
        b"version\0".as_ptr() as *const ::core::ffi::c_char,
        b"detection\0".as_ptr() as *const ::core::ffi::c_char,
        b"health\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if cJSON_IsObject(root) == 0
        || !validate_object_keys(
            root,
            &raw const root_fields as *const *const ::core::ffi::c_char,
            3 as size_t,
            b"predicate\0".as_ptr() as *const ::core::ffi::c_char,
            error,
            error_size,
        )
        || !version_is_one(
            root,
            b"predicate\0".as_ptr() as *const ::core::ffi::c_char,
            error,
            error_size,
        )
    {
        cJSON_Delete(root);
        if !error.is_null()
            && error_size > 0 as size_t
            && *error.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            set_error(
                error,
                error_size,
                b"predicate must be a JSON object\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return r#false != 0;
    }
    let mut detection: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"detection\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut valid: bool = r#true != 0;
    if !detection.is_null() {
        let detection_fields: [*const ::core::ffi::c_char; 3] = [
            b"labels_any\0".as_ptr() as *const ::core::ffi::c_char,
            b"min_confidence\0".as_ptr() as *const ::core::ffi::c_char,
            b"zone_ids_any\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        if cJSON_IsObject(detection) == 0
            || !route_has_type(
                route,
                b"io.lightnvr.detection.object.v1\0".as_ptr() as *const ::core::ffi::c_char,
            )
            || !validate_object_keys(
                detection,
                &raw const detection_fields as *const *const ::core::ffi::c_char,
                3 as size_t,
                b"predicate.detection\0".as_ptr() as *const ::core::ffi::c_char,
                error,
                error_size,
            )
        {
            if cJSON_IsObject(detection) != 0
                && !route_has_type(
                    route,
                    b"io.lightnvr.detection.object.v1\0".as_ptr() as *const ::core::ffi::c_char,
                )
            {
                set_error(
                    error,
                    error_size,
                    b"detection predicate requires the detection event type\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else if cJSON_IsObject(detection) == 0 {
                set_error(
                    error,
                    error_size,
                    b"predicate.detection must be an object\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            cJSON_Delete(root);
            return r#false != 0;
        }
        let mut labels: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            detection,
            b"labels_any\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut confidence: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            detection,
            b"min_confidence\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut zones: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            detection,
            b"zone_ids_any\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if labels.is_null() && confidence.is_null() && zones.is_null() {
            set_error(
                error,
                error_size,
                b"predicate.detection requires at least one filter\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            cJSON_Delete(root);
            return r#false != 0;
        }
        valid = (labels.is_null()
            || valid_string_array(
                labels,
                b"predicate.detection.labels_any\0".as_ptr() as *const ::core::ffi::c_char,
                error,
                error_size,
            ) as ::core::ffi::c_int
                != 0)
            && (zones.is_null()
                || valid_string_array(
                    zones,
                    b"predicate.detection.zone_ids_any\0".as_ptr() as *const ::core::ffi::c_char,
                    error,
                    error_size,
                ) as ::core::ffi::c_int
                    != 0);
        if valid as ::core::ffi::c_int != 0
            && !confidence.is_null()
            && (cJSON_IsNumber(confidence) == 0
                || (*confidence).valuedouble < 0.0f64
                || (*confidence).valuedouble > 1.0f64)
        {
            set_error(
                error,
                error_size,
                b"predicate.detection.min_confidence must be from 0 to 1\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            valid = r#false != 0;
        }
    }
    let mut health: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(root, b"health\0".as_ptr() as *const ::core::ffi::c_char);
    if valid as ::core::ffi::c_int != 0 && !health.is_null() {
        let health_fields: [*const ::core::ffi::c_char; 2] = [
            b"condition_codes_any\0".as_ptr() as *const ::core::ffi::c_char,
            b"severities_any\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        if cJSON_IsObject(health) == 0 {
            set_error(
                error,
                error_size,
                b"predicate.health must be an object\0".as_ptr() as *const ::core::ffi::c_char,
            );
            valid = r#false != 0;
        } else if !route_types_are_family(
            route,
            b"system_health\0".as_ptr() as *const ::core::ffi::c_char,
        ) {
            set_error(
                error,
                error_size,
                b"health predicate requires only system health event types\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            valid = r#false != 0;
        } else if !validate_object_keys(
            health,
            &raw const health_fields as *const *const ::core::ffi::c_char,
            2 as size_t,
            b"predicate.health\0".as_ptr() as *const ::core::ffi::c_char,
            error,
            error_size,
        ) {
            valid = r#false != 0;
        } else {
            let mut conditions: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                health,
                b"condition_codes_any\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut severities: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                health,
                b"severities_any\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if conditions.is_null() && severities.is_null() {
                set_error(
                    error,
                    error_size,
                    b"predicate.health requires at least one filter\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                valid = r#false != 0;
            } else {
                valid = (conditions.is_null()
                    || valid_health_condition_array(conditions, error, error_size)
                        as ::core::ffi::c_int
                        != 0)
                    && (severities.is_null()
                        || valid_health_severity_array(severities, error, error_size)
                            as ::core::ffi::c_int
                            != 0);
            }
        }
    }
    if valid as ::core::ffi::c_int != 0 && !detection.is_null() && !health.is_null() {
        set_error(
            error,
            error_size,
            b"predicate cannot combine detection and health filters\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        valid = r#false != 0;
    }
    cJSON_Delete(root);
    return valid;
}
unsafe extern "C" fn valid_timezone(mut timezone: *const ::core::ffi::c_char) -> bool {
    if !valid_text(timezone, 65 as size_t, r#true != 0)
        || *timezone.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        || !strstr(timezone, b"//\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        || *timezone.offset(strlen(timezone).wrapping_sub(1 as size_t) as isize)
            as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = timezone as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
            && *cursor as ::core::ffi::c_int != '/' as ::core::ffi::c_int
            && *cursor as ::core::ffi::c_int != '_' as ::core::ffi::c_int
            && *cursor as ::core::ffi::c_int != '-' as ::core::ffi::c_int
            && *cursor as ::core::ffi::c_int != '+' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn timezone_available(mut timezone: *const ::core::ffi::c_char) -> bool {
    if !valid_timezone(timezone) {
        return r#false != 0;
    }
    if strcmp(timezone, b"UTC\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strcmp(timezone, b"GMT\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    let mut path: [::core::ffi::c_char; 256] = [0; 256];
    let mut length: ::core::ffi::c_int = snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"/usr/share/zoneinfo/%s\0".as_ptr() as *const ::core::ffi::c_char,
        timezone,
    );
    if length < 0 as ::core::ffi::c_int
        || length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
    {
        return r#false != 0;
    }
    let mut metadata: stat = stat {
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
    return stat(&raw mut path as *mut ::core::ffi::c_char, &raw mut metadata)
        == 0 as ::core::ffi::c_int
        && metadata.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t;
}
unsafe extern "C" fn valid_clock_time(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null()
        || strlen(value) != 5 as size_t
        || *value.offset(2isize) as ::core::ffi::c_int != ':' as ::core::ffi::c_int
        || *(*__ctype_b_loc())
            .offset(*value.offset(0isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(*value.offset(1isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(*value.offset(3isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        || *(*__ctype_b_loc())
            .offset(*value.offset(4isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        return r#false != 0;
    }
    let mut hour: ::core::ffi::c_int = (*value.offset(0isize) as ::core::ffi::c_int
        - '0' as ::core::ffi::c_int)
        * 10 as ::core::ffi::c_int
        + *value.offset(1isize) as ::core::ffi::c_int
        - '0' as ::core::ffi::c_int;
    let mut minute: ::core::ffi::c_int = (*value.offset(3isize) as ::core::ffi::c_int
        - '0' as ::core::ffi::c_int)
        * 10 as ::core::ffi::c_int
        + *value.offset(4isize) as ::core::ffi::c_int
        - '0' as ::core::ffi::c_int;
    return hour >= 0 as ::core::ffi::c_int
        && hour <= 23 as ::core::ffi::c_int
        && minute >= 0 as ::core::ffi::c_int
        && minute <= 59 as ::core::ffi::c_int;
}
unsafe extern "C" fn valid_window(
    mut window: *const cJSON,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    let fields: [*const ::core::ffi::c_char; 3] = [
        b"days\0".as_ptr() as *const ::core::ffi::c_char,
        b"start\0".as_ptr() as *const ::core::ffi::c_char,
        b"end\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if cJSON_IsObject(window) == 0
        || !validate_object_keys(
            window,
            &raw const fields as *const *const ::core::ffi::c_char,
            3 as size_t,
            b"schedule window\0".as_ptr() as *const ::core::ffi::c_char,
            error,
            error_size,
        )
    {
        set_error(
            error,
            error_size,
            b"schedule windows must be objects\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut days: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(window, b"days\0".as_ptr() as *const ::core::ffi::c_char);
    let mut start: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(window, b"start\0".as_ptr() as *const ::core::ffi::c_char);
    let mut end: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(window, b"end\0".as_ptr() as *const ::core::ffi::c_char);
    let mut day_count: ::core::ffi::c_int = if cJSON_IsArray(days) != 0 {
        cJSON_GetArraySize(days)
    } else {
        -1 as ::core::ffi::c_int
    };
    if day_count < 1 as ::core::ffi::c_int
        || day_count > 7 as ::core::ffi::c_int
        || cJSON_IsString(start) == 0
        || cJSON_IsString(end) == 0
        || !valid_clock_time((*start).valuestring)
        || !valid_clock_time((*end).valuestring)
        || strcmp((*start).valuestring, (*end).valuestring) == 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"schedule window requires days 0-6 and distinct HH:MM bounds\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut seen: [bool; 7] = [r#false != 0, false, false, false, false, false, false];
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < day_count {
        let mut day: *const cJSON = cJSON_GetArrayItem(days, index);
        if cJSON_IsNumber(day) == 0
            || (*day).valuedouble != (*day).valueint as ::core::ffi::c_double
            || (*day).valueint < 0 as ::core::ffi::c_int
            || (*day).valueint > 6 as ::core::ffi::c_int
            || seen[(*day).valueint as usize] as ::core::ffi::c_int != 0
        {
            set_error(
                error,
                error_size,
                b"schedule window days must be unique integers from 0 to 6\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        seen[(*day).valueint as usize] = r#true != 0;
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_schedule(
    mut route: *const event_route_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    let mut root: *mut cJSON =
        cJSON_Parse(&raw const (*route).schedule_json as *const ::core::ffi::c_char);
    let fields: [*const ::core::ffi::c_char; 3] = [
        b"version\0".as_ptr() as *const ::core::ffi::c_char,
        b"timezone\0".as_ptr() as *const ::core::ffi::c_char,
        b"windows\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if cJSON_IsObject(root) == 0
        || !validate_object_keys(
            root,
            &raw const fields as *const *const ::core::ffi::c_char,
            3 as size_t,
            b"schedule\0".as_ptr() as *const ::core::ffi::c_char,
            error,
            error_size,
        )
        || !version_is_one(
            root,
            b"schedule\0".as_ptr() as *const ::core::ffi::c_char,
            error,
            error_size,
        )
    {
        cJSON_Delete(root);
        set_error(
            error,
            error_size,
            b"schedule must be a JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut timezone: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"timezone\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut windows: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(root, b"windows\0".as_ptr() as *const ::core::ffi::c_char);
    let mut count: ::core::ffi::c_int = if cJSON_IsArray(windows) != 0 {
        cJSON_GetArraySize(windows)
    } else {
        -1 as ::core::ffi::c_int
    };
    if cJSON_IsString(timezone) == 0
        || !timezone_available((*timezone).valuestring)
        || count < 0 as ::core::ffi::c_int
        || count > 64 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"schedule requires an available timezone and at most 64 windows\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        cJSON_Delete(root);
        return r#false != 0;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        if !valid_window(cJSON_GetArrayItem(windows, index), error, error_size) {
            cJSON_Delete(root);
            return r#false != 0;
        }
        index += 1;
    }
    cJSON_Delete(root);
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_validate(
    mut route: *const event_route_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> db_event_route_result_t {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if route.is_null()
        || !valid_text(
            &raw const (*route).name as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            r#true != 0,
        )
        || !valid_text(
            &raw const (*route).description as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            r#false != 0,
        )
    {
        set_error(
            error,
            error_size,
            b"route name or description is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
    }
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    if copy_trimmed_value(
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        &raw const (*route).name as *const ::core::ffi::c_char,
        0 as size_t,
    ) == 0 as size_t
    {
        set_error(
            error,
            error_size,
            b"route name cannot be blank\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
    }
    if strcmp(
        &raw const (*route).destination_key as *const ::core::ffi::c_char,
        EVENT_ROUTE_DEFAULT_DESTINATION.as_ptr(),
    ) != 0 as ::core::ffi::c_int
        && !db_event_destination_key_exists(
            &raw const (*route).destination_key as *const ::core::ffi::c_char,
        )
    {
        set_error(
            error,
            error_size,
            b"destination must be mqtt:default or an existing managed MQTT destination\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
    }
    if (*route).event_type_count < 1 as ::core::ffi::c_int
        || (*route).event_type_count > EVENT_ROUTE_MAX_TYPES
    {
        set_error(
            error,
            error_size,
            b"route requires 1-%d event types\0".as_ptr() as *const ::core::ffi::c_char,
            EVENT_ROUTE_MAX_TYPES,
        );
        return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*route).event_type_count {
        if event_registry_find(
            &raw const *(&raw const (*route).event_types as *const [::core::ffi::c_char; 96])
                .offset(index as isize) as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            set_error(
                error,
                error_size,
                b"route contains an unknown event type\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
        }
        let mut previous: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while previous < index {
            if strcmp(
                &raw const *(&raw const (*route).event_types as *const [::core::ffi::c_char; 96])
                    .offset(index as isize) as *const ::core::ffi::c_char,
                &raw const *(&raw const (*route).event_types as *const [::core::ffi::c_char; 96])
                    .offset(previous as isize) as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                set_error(
                    error,
                    error_size,
                    b"route contains a duplicate event type\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
            }
            previous += 1;
        }
        index += 1;
    }
    if strcmp(
        &raw const (*route).scope_type as *const ::core::ffi::c_char,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if (*route).selector_json[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            set_error(
                error,
                error_size,
                b"all scope cannot include a camera selector\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
        }
    } else if strcmp(
        &raw const (*route).scope_type as *const ::core::ffi::c_char,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !route_types_are_camera_subjects(route) {
            set_error(
                error,
                error_size,
                b"camera selector requires only camera event types\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
        }
        let mut json: *mut cJSON =
            cJSON_Parse(&raw const (*route).selector_json as *const ::core::ffi::c_char);
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
        cJSON_Delete(json);
        if selector.is_null() {
            set_error(
                error,
                error_size,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                if selector_error[0usize] as ::core::ffi::c_int != 0 {
                    &raw mut selector_error as *mut ::core::ffi::c_char
                } else {
                    b"camera selector is invalid\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
        }
        fleet_selector_free(selector);
    } else {
        set_error(
            error,
            error_size,
            b"camera scope must be all or selector\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
    }
    if !valid_predicate(route, error, error_size) || !valid_schedule(route, error, error_size) {
        return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
    }
    if (*route).debounce_seconds < 0 as ::core::ffi::c_int
        || (*route).debounce_seconds > 86400 as ::core::ffi::c_int
        || (*route).cooldown_seconds < 0 as ::core::ffi::c_int
        || (*route).cooldown_seconds > 604800 as ::core::ffi::c_int
        || (*route).grouping_window_seconds < 0 as ::core::ffi::c_int
        || (*route).grouping_window_seconds > 3600 as ::core::ffi::c_int
        || (*route).max_events_per_minute < 0 as ::core::ffi::c_int
        || (*route).max_events_per_minute > 60000 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"route suppression value is out of range\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
    }
    return db_event_route_result_t::DB_EVENT_ROUTE_OK;
}
unsafe extern "C" fn route_transaction_begin(mut db: *mut sqlite3) -> bool {
    return sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK;
}
unsafe extern "C" fn route_transaction_finish(mut db: *mut sqlite3, mut success: bool) -> bool {
    if success as ::core::ffi::c_int != 0
        && sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK
    {
        return r#true != 0;
    }
    sqlite3_exec(
        db,
        b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    return r#false != 0;
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
unsafe extern "C" fn populate_route(
    mut statement: *mut sqlite3_stmt,
    mut route: *mut event_route_t,
) {
    memset(
        route as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_route_t>(),
    );
    copy_column(
        &raw mut (*route).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*route).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*route).description as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        2 as ::core::ffi::c_int,
    );
    (*route).enabled =
        sqlite3_column_int(statement, 3 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    copy_column(
        &raw mut (*route).destination_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        4 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*route).scope_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        statement,
        5 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*route).selector_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        statement,
        6 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*route).predicate_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        statement,
        7 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*route).schedule_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        statement,
        8 as ::core::ffi::c_int,
    );
    (*route).debounce_seconds = sqlite3_column_int(statement, 9 as ::core::ffi::c_int);
    (*route).cooldown_seconds = sqlite3_column_int(statement, 10 as ::core::ffi::c_int);
    (*route).grouping_window_seconds = sqlite3_column_int(statement, 11 as ::core::ffi::c_int);
    (*route).max_events_per_minute = sqlite3_column_int(statement, 12 as ::core::ffi::c_int);
    (*route).revision = sqlite3_column_int64(statement, 13 as ::core::ffi::c_int) as int64_t;
    (*route).created_at = sqlite3_column_int64(statement, 14 as ::core::ffi::c_int) as int64_t;
    (*route).updated_at = sqlite3_column_int64(statement, 15 as ::core::ffi::c_int) as int64_t;
}
unsafe extern "C" fn load_types_locked(
    mut db: *mut sqlite3,
    mut route: *mut event_route_t,
) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT event_type FROM event_route_types WHERE route_uuid=? ORDER BY event_type;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        statement,
        1 as ::core::ffi::c_int,
        &raw mut (*route).uuid as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        result = sqlite3_step(statement);
        if !(result == SQLITE_ROW && count < EVENT_ROUTE_MAX_TYPES) {
            break;
        }
        copy_column(
            &raw mut *(&raw mut (*route).event_types as *mut [::core::ffi::c_char; 96])
                .offset(count as isize) as *mut ::core::ffi::c_char,
            EVENT_TYPE_MAX as size_t,
            statement,
            0 as ::core::ffi::c_int,
        );
        count += 1;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result != SQLITE_DONE {
        return -1 as ::core::ffi::c_int;
    }
    (*route).event_type_count = count;
    return if count > 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut route: *mut event_route_t,
) -> db_event_route_result_t {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT uuid,name,description,enabled,destination_key,scope_type,selector_json,predicate_json,schedule_json,debounce_seconds,cooldown_seconds,grouping_window_seconds,max_events_per_minute,revision,created_at,updated_at FROM event_routes WHERE uuid=? LIMIT 1;\0"
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
    if result == SQLITE_DONE {
        sqlite3_finalize(statement);
        return db_event_route_result_t::DB_EVENT_ROUTE_NOT_FOUND;
    }
    if result != SQLITE_ROW {
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        return db_event_route_result_t::DB_EVENT_ROUTE_ERROR;
    }
    populate_route(statement, route);
    sqlite3_finalize(statement);
    return db_event_route_result_t(if load_types_locked(db, route) == 0 as ::core::ffi::c_int {
        db_event_route_result_t::DB_EVENT_ROUTE_OK.0
    } else {
        db_event_route_result_t::DB_EVENT_ROUTE_ERROR.0
    });
}
unsafe extern "C" fn count_locked(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM event_routes;\0".as_ptr() as *const ::core::ffi::c_char,
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
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_count() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut count: ::core::ffi::c_int = count_locked(db);
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_list(
    mut routes: *mut event_route_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || mutex.is_null()
        || routes.is_null()
        || max_count <= 0 as ::core::ffi::c_int
        || max_count > EVENT_ROUTE_MAX_COUNT
    {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT uuid,name,description,enabled,destination_key,scope_type,selector_json,predicate_json,schedule_json,debounce_seconds,cooldown_seconds,grouping_window_seconds,max_events_per_minute,revision,created_at,updated_at FROM event_routes ORDER BY name COLLATE NOCASE,uuid;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result != SQLITE_OK {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while count < max_count && {
        result = sqlite3_step(statement);
        result == SQLITE_ROW
    } {
        let c2rust_fresh0 = count;
        count += 1;
        populate_route(statement, routes.offset(c2rust_fresh0 as isize));
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result != SQLITE_DONE && result != SQLITE_ROW {
        count = -1 as ::core::ffi::c_int;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while count >= 0 as ::core::ffi::c_int && index < count {
        if load_types_locked(db, routes.offset(index as isize)) != 0 as ::core::ffi::c_int {
            count = -1 as ::core::ffi::c_int;
        }
        index += 1;
    }
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_generation() -> uint64_t {
    return ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut ROUTE_GENERATION,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_get(
    mut uuid: *const ::core::ffi::c_char,
    mut route: *mut event_route_t,
) -> db_event_route_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() || route.is_null() || !valid_uuid(uuid) {
        return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_event_route_result_t = get_locked(db, uuid, route);
    pthread_mutex_unlock(mutex);
    return result;
}
unsafe extern "C" fn insert_types_locked(
    mut db: *mut sqlite3,
    mut route: *const event_route_t,
) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"INSERT INTO event_route_types(route_uuid,event_type) VALUES(?,?);\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while result == SQLITE_OK && index < (*route).event_type_count {
        sqlite3_reset(statement);
        sqlite3_clear_bindings(statement);
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw const (*route).uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw const *(&raw const (*route).event_types as *const [::core::ffi::c_char; 96])
                .offset(index as isize) as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
        if result == SQLITE_DONE {
            result = SQLITE_OK;
        }
        index += 1;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if result == SQLITE_OK {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn clear_suppression_locked(
    mut db: *mut sqlite3,
    mut route_uuid: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM event_route_suppression_state WHERE route_uuid=?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            route_uuid,
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
unsafe extern "C" fn bind_route_fields(
    mut statement: *mut sqlite3_stmt,
    mut route: *const event_route_t,
    mut normalized_name: *const ::core::ffi::c_char,
) {
    sqlite3_bind_text(
        statement,
        1 as ::core::ffi::c_int,
        normalized_name,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        2 as ::core::ffi::c_int,
        &raw const (*route).description as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_int(
        statement,
        3 as ::core::ffi::c_int,
        if (*route).enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_text(
        statement,
        4 as ::core::ffi::c_int,
        &raw const (*route).destination_key as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        5 as ::core::ffi::c_int,
        &raw const (*route).scope_type as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    if strcmp(
        &raw const (*route).scope_type as *const ::core::ffi::c_char,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            &raw const (*route).selector_json as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    } else {
        sqlite3_bind_null(statement, 6 as ::core::ffi::c_int);
    }
    sqlite3_bind_text(
        statement,
        7 as ::core::ffi::c_int,
        &raw const (*route).predicate_json as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        statement,
        8 as ::core::ffi::c_int,
        &raw const (*route).schedule_json as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_int(
        statement,
        9 as ::core::ffi::c_int,
        (*route).debounce_seconds,
    );
    sqlite3_bind_int(
        statement,
        10 as ::core::ffi::c_int,
        (*route).cooldown_seconds,
    );
    sqlite3_bind_int(
        statement,
        11 as ::core::ffi::c_int,
        (*route).grouping_window_seconds,
    );
    sqlite3_bind_int(
        statement,
        12 as ::core::ffi::c_int,
        (*route).max_events_per_minute,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_create(
    mut route: *mut event_route_t,
) -> db_event_route_result_t {
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
    if db_event_route_validate(
        route,
        &raw mut error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    )
    .0 != db_event_route_result_t::DB_EVENT_ROUTE_OK.0
    {
        return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_event_route_result_t::DB_EVENT_ROUTE_ERROR;
    }
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    copy_trimmed_value(
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        &raw mut (*route).name as *mut ::core::ffi::c_char,
        0 as size_t,
    );
    pthread_mutex_lock(mutex);
    if !route_transaction_begin(db) {
        pthread_mutex_unlock(mutex);
        return db_event_route_result_t::DB_EVENT_ROUTE_ERROR;
    }
    if count_locked(db) >= EVENT_ROUTE_MAX_COUNT {
        route_transaction_finish(db, r#false != 0);
        pthread_mutex_unlock(mutex);
        return db_event_route_result_t::DB_EVENT_ROUTE_LIMIT;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO event_routes(uuid,name,description,enabled,destination_key,scope_type,selector_json,predicate_json,schedule_json,debounce_seconds,cooldown_seconds,grouping_window_seconds,max_events_per_minute) VALUES(lower(hex(randomblob(4))||'-'||hex(randomblob(2))||'-4'||substr(hex(randomblob(2)),2)||'-'||substr('89ab',(abs(random())%4)+1,1)||substr(hex(randomblob(2)),2)||'-'||hex(randomblob(6))),?,?,?,?,?,?,?,?,?,?,?,?);\0"
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
        bind_route_fields(
            statement,
            route,
            &raw mut normalized_name as *mut ::core::ffi::c_char,
        );
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result != SQLITE_DONE {
        route_transaction_finish(db, r#false != 0);
        pthread_mutex_unlock(mutex);
        return db_event_route_result_t(
            if result & 0xff as ::core::ffi::c_int == SQLITE_CONSTRAINT {
                db_event_route_result_t::DB_EVENT_ROUTE_CONFLICT.0
            } else {
                db_event_route_result_t::DB_EVENT_ROUTE_ERROR.0
            },
        );
    }
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    result = sqlite3_prepare_v2(
        db,
        b"SELECT uuid FROM event_routes WHERE id=last_insert_rowid();\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW {
        copy_column(
            &raw mut (*route).uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            statement,
            0 as ::core::ffi::c_int,
        );
        result = SQLITE_OK;
    } else {
        result = SQLITE_ERROR;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    let mut ready_to_commit: bool =
        result == SQLITE_OK && insert_types_locked(db, route) == 0 as ::core::ffi::c_int;
    if !route_transaction_finish(db, ready_to_commit) {
        pthread_mutex_unlock(mutex);
        return db_event_route_result_t::DB_EVENT_ROUTE_ERROR;
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut ROUTE_GENERATION,
        1 as uint_fast64_t,
    );
    let mut route_result: db_event_route_result_t = get_locked(
        db,
        &raw mut (*route).uuid as *mut ::core::ffi::c_char,
        route,
    );
    pthread_mutex_unlock(mutex);
    return route_result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_update(
    mut route: *mut event_route_t,
    mut expected_revision: int64_t,
) -> db_event_route_result_t {
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
    if route.is_null()
        || !valid_uuid(&raw mut (*route).uuid as *mut ::core::ffi::c_char)
        || expected_revision < 1 as int64_t
        || db_event_route_validate(
            route,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        )
        .0 != db_event_route_result_t::DB_EVENT_ROUTE_OK.0
    {
        return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_event_route_result_t::DB_EVENT_ROUTE_ERROR;
    }
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    copy_trimmed_value(
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        &raw mut (*route).name as *mut ::core::ffi::c_char,
        0 as size_t,
    );
    pthread_mutex_lock(mutex);
    if !route_transaction_begin(db) {
        pthread_mutex_unlock(mutex);
        return db_event_route_result_t::DB_EVENT_ROUTE_ERROR;
    }
    let mut existing: event_route_t = event_route_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_key: [0; 128],
        scope_type: [0; 16],
        selector_json: [0; 8192],
        predicate_json: [0; 4096],
        schedule_json: [0; 8192],
        event_types: [[0; 96]; 32],
        event_type_count: 0,
        debounce_seconds: 0,
        cooldown_seconds: 0,
        grouping_window_seconds: 0,
        max_events_per_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut route_result: db_event_route_result_t = get_locked(
        db,
        &raw mut (*route).uuid as *mut ::core::ffi::c_char,
        &raw mut existing,
    );
    if route_result.0 != db_event_route_result_t::DB_EVENT_ROUTE_OK.0 {
        route_transaction_finish(db, r#false != 0);
        pthread_mutex_unlock(mutex);
        return route_result;
    }
    if existing.revision != expected_revision {
        route_transaction_finish(db, r#false != 0);
        pthread_mutex_unlock(mutex);
        return db_event_route_result_t::DB_EVENT_ROUTE_STALE;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE event_routes SET name=?,description=?,enabled=?,destination_key=?,scope_type=?,selector_json=?,predicate_json=?,schedule_json=?,debounce_seconds=?,cooldown_seconds=?,grouping_window_seconds=?,max_events_per_minute=?,revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=? AND revision=?;\0"
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
        bind_route_fields(
            statement,
            route,
            &raw mut normalized_name as *mut ::core::ffi::c_char,
        );
        sqlite3_bind_text(
            statement,
            13 as ::core::ffi::c_int,
            &raw mut (*route).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            14 as ::core::ffi::c_int,
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
    if result != SQLITE_DONE || changed != 1 as ::core::ffi::c_int {
        route_transaction_finish(db, r#false != 0);
        pthread_mutex_unlock(mutex);
        return db_event_route_result_t(
            if result & 0xff as ::core::ffi::c_int == SQLITE_CONSTRAINT {
                db_event_route_result_t::DB_EVENT_ROUTE_CONFLICT.0
            } else {
                db_event_route_result_t::DB_EVENT_ROUTE_ERROR.0
            },
        );
    }
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    result = sqlite3_prepare_v2(
        db,
        b"DELETE FROM event_route_types WHERE route_uuid=?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut (*route).uuid as *mut ::core::ffi::c_char,
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
    let mut ready_to_commit: bool = result == SQLITE_DONE
        && clear_suppression_locked(db, &raw mut (*route).uuid as *mut ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        && insert_types_locked(db, route) == 0 as ::core::ffi::c_int;
    if !route_transaction_finish(db, ready_to_commit) {
        pthread_mutex_unlock(mutex);
        return db_event_route_result_t::DB_EVENT_ROUTE_ERROR;
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut ROUTE_GENERATION,
        1 as uint_fast64_t,
    );
    route_result = get_locked(
        db,
        &raw mut (*route).uuid as *mut ::core::ffi::c_char,
        route,
    );
    pthread_mutex_unlock(mutex);
    return route_result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_event_route_delete(
    mut uuid: *const ::core::ffi::c_char,
    mut expected_revision: int64_t,
) -> db_event_route_result_t {
    if !valid_uuid(uuid) || expected_revision < 1 as int64_t {
        return db_event_route_result_t::DB_EVENT_ROUTE_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_event_route_result_t::DB_EVENT_ROUTE_ERROR;
    }
    pthread_mutex_lock(mutex);
    if !route_transaction_begin(db) {
        pthread_mutex_unlock(mutex);
        return db_event_route_result_t::DB_EVENT_ROUTE_ERROR;
    }
    let mut existing: event_route_t = event_route_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_key: [0; 128],
        scope_type: [0; 16],
        selector_json: [0; 8192],
        predicate_json: [0; 4096],
        schedule_json: [0; 8192],
        event_types: [[0; 96]; 32],
        event_type_count: 0,
        debounce_seconds: 0,
        cooldown_seconds: 0,
        grouping_window_seconds: 0,
        max_events_per_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_event_route_result_t = get_locked(db, uuid, &raw mut existing);
    if result.0 != db_event_route_result_t::DB_EVENT_ROUTE_OK.0 {
        route_transaction_finish(db, r#false != 0);
        pthread_mutex_unlock(mutex);
        return result;
    }
    if existing.revision != expected_revision {
        route_transaction_finish(db, r#false != 0);
        pthread_mutex_unlock(mutex);
        return db_event_route_result_t::DB_EVENT_ROUTE_STALE;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sqlite_result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM event_routes WHERE uuid=? AND revision=?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if sqlite_result == SQLITE_OK {
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
        sqlite_result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if sqlite_result == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    let mut committed: bool = route_transaction_finish(
        db,
        sqlite_result == SQLITE_DONE && changed == 1 as ::core::ffi::c_int,
    );
    if committed {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
            &raw mut ROUTE_GENERATION,
            1 as uint_fast64_t,
        );
    }
    pthread_mutex_unlock(mutex);
    return db_event_route_result_t(if committed as ::core::ffi::c_int != 0 {
        db_event_route_result_t::DB_EVENT_ROUTE_OK.0
    } else {
        db_event_route_result_t::DB_EVENT_ROUTE_ERROR.0
    });
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
