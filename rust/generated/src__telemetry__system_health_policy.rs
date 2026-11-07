unsafe extern "C" {
    unsafe fn system_health_unit_name(unit: system_health_unit_t) -> *const ::core::ffi::c_char;
    unsafe fn system_health_condition_code(
        condition: system_health_condition_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn system_health_condition_from_code(
        code: *const ::core::ffi::c_char,
        condition_out: *mut system_health_condition_t,
    ) -> bool;
    unsafe fn cJSON_ParseWithOpts(
        value: *const ::core::ffi::c_char,
        return_parse_end: *mut *const ::core::ffi::c_char,
        require_null_terminated: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
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
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
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
    unsafe fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    unsafe fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    unsafe fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
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
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub type pthread_once_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_unit_t(pub ::core::ffi::c_uint);
impl system_health_unit_t {
    pub const SYSTEM_HEALTH_UNIT_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_UNIT_RATIO: Self = Self(1);
    pub const SYSTEM_HEALTH_UNIT_BYTES: Self = Self(2);
    pub const SYSTEM_HEALTH_UNIT_COUNT: Self = Self(3);
    pub const SYSTEM_HEALTH_UNIT_SECONDS: Self = Self(4);
    pub const SYSTEM_HEALTH_UNIT_CELSIUS: Self = Self(5);
    pub const SYSTEM_HEALTH_UNIT_HERTZ: Self = Self(6);
    pub const SYSTEM_HEALTH_UNIT_BOOLEAN: Self = Self(7);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_profile_t(pub ::core::ffi::c_uint);
impl system_health_profile_t {
    pub const SYSTEM_HEALTH_PROFILE_BALANCED: Self = Self(0);
    pub const SYSTEM_HEALTH_PROFILE_CONSERVATIVE: Self = Self(1);
    pub const SYSTEM_HEALTH_PROFILE_DISABLED: Self = Self(2);
    pub const SYSTEM_HEALTH_PROFILE_CUSTOM: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_threshold_direction_t(pub ::core::ffi::c_uint);
impl system_health_threshold_direction_t {
    pub const SYSTEM_HEALTH_THRESHOLD_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE: Self = Self(1);
    pub const SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_policy_settings_t {
    pub enabled: bool,
    pub profile: [::core::ffi::c_char; 16],
    pub fast_interval_seconds: uint32_t,
    pub normal_interval_seconds: uint32_t,
    pub slow_interval_seconds: uint32_t,
    pub device_interval_seconds: uint32_t,
    pub write_probe_enabled: bool,
    pub hardware_provider: [::core::ffi::c_char; 16],
    pub presence_interval_seconds: uint32_t,
    pub incident_retention_days: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_condition_policy_t {
    pub condition: system_health_condition_t,
    pub profile: system_health_profile_t,
    pub enabled: bool,
    pub overridden: bool,
    pub direction: system_health_threshold_direction_t,
    pub unit: system_health_unit_t,
    pub warning_threshold: ::core::ffi::c_double,
    pub critical_threshold: ::core::ffi::c_double,
    pub recovery_threshold: ::core::ffi::c_double,
    pub warning_for_seconds: uint32_t,
    pub critical_for_seconds: uint32_t,
    pub recovery_for_seconds: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_policy_t {
    pub generation: uint64_t,
    pub settings: system_health_policy_settings_t,
    pub conditions: [system_health_condition_policy_t; 27],
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const PTHREAD_RWLOCK_PREFER_READER_NP: Self = Self(0);
    pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: Self = Self(1);
    pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: Self = Self(2);
    pub const PTHREAD_RWLOCK_DEFAULT_NP: Self = Self(0);
}
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const SYSTEM_HEALTH_POLICY_ERROR_LENGTH: ::core::ffi::c_uint = 256 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_OVERRIDE_JSON_MAX: ::core::ffi::c_uint = 16384 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_FAST_INTERVAL_MIN: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_FAST_INTERVAL_MAX: ::core::ffi::c_uint = 60 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_NORMAL_INTERVAL_MIN: ::core::ffi::c_uint = 15 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_NORMAL_INTERVAL_MAX: ::core::ffi::c_uint = 600 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_SLOW_INTERVAL_MIN: ::core::ffi::c_uint = 60 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_SLOW_INTERVAL_MAX: ::core::ffi::c_uint = 3600 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_DEVICE_INTERVAL_MIN: ::core::ffi::c_uint = 300 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_DEVICE_INTERVAL_MAX: ::core::ffi::c_uint = 86400 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_PRESENCE_INTERVAL_MIN: ::core::ffi::c_uint = 15 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_PRESENCE_INTERVAL_MAX: ::core::ffi::c_uint = 3600 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_RETENTION_DAYS_MIN: ::core::ffi::c_uint = 7 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_RETENTION_DAYS_MAX: ::core::ffi::c_uint = 3650 as ::core::ffi::c_uint;
pub const PTHREAD_ONCE_INIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POLICY_DWELL_MAX_SECONDS: ::core::ffi::c_uint = 604800 as ::core::ffi::c_uint;
static mut active_policy_lock: pthread_rwlock_t = pthread_rwlock_t {
    __data: __pthread_rwlock_arch_t {
        __readers: 0 as ::core::ffi::c_uint,
        __writers: 0 as ::core::ffi::c_uint,
        __wrphase_futex: 0 as ::core::ffi::c_uint,
        __writers_futex: 0 as ::core::ffi::c_uint,
        __pad3: 0 as ::core::ffi::c_uint,
        __pad4: 0 as ::core::ffi::c_uint,
        __cur_writer: 0 as ::core::ffi::c_int,
        __shared: 0 as ::core::ffi::c_int,
        __pad1: 0 as ::core::ffi::c_ulong,
        __pad2: 0 as ::core::ffi::c_ulong,
        __flags: C2Rust_Unnamed::PTHREAD_RWLOCK_DEFAULT_NP.0,
    },
};
static mut active_policy_once: pthread_once_t = PTHREAD_ONCE_INIT;
static mut active_policy: system_health_policy_t = system_health_policy_t {
    generation: 0,
    settings: system_health_policy_settings_t {
        enabled: false,
        profile: [0; 16],
        fast_interval_seconds: 0,
        normal_interval_seconds: 0,
        slow_interval_seconds: 0,
        device_interval_seconds: 0,
        write_probe_enabled: false,
        hardware_provider: [0; 16],
        presence_interval_seconds: 0,
        incident_retention_days: 0,
    },
    conditions: [system_health_condition_policy_t {
        condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
        profile: system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED,
        enabled: false,
        overridden: false,
        direction: system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE,
        unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
        warning_threshold: 0.,
        critical_threshold: 0.,
        recovery_threshold: 0.,
        warning_for_seconds: 0,
        critical_for_seconds: 0,
        recovery_for_seconds: 0,
    }; 27],
};
unsafe extern "C" fn fail(
    mut error: *mut ::core::ffi::c_char,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    if !error.is_null() {
        let mut arguments: ::core::ffi::VaList;
        arguments = c2rust_args.clone();
        vsnprintf(
            error,
            SYSTEM_HEALTH_POLICY_ERROR_LENGTH as size_t,
            format,
            arguments.clone(),
        );
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn clear_error(mut error: *mut ::core::ffi::c_char) {
    if !error.is_null() {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_policy_settings_defaults(
    mut settings: *mut system_health_policy_settings_t,
) {
    if settings.is_null() {
        return;
    }
    memset(
        settings as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_policy_settings_t>(),
    );
    (*settings).enabled = r#true != 0;
    snprintf(
        &raw mut (*settings).profile as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        b"balanced\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*settings).fast_interval_seconds = 10 as ::core::ffi::c_uint as uint32_t;
    (*settings).normal_interval_seconds = 60 as ::core::ffi::c_uint as uint32_t;
    (*settings).slow_interval_seconds = 300 as ::core::ffi::c_uint as uint32_t;
    (*settings).device_interval_seconds = 900 as ::core::ffi::c_uint as uint32_t;
    (*settings).write_probe_enabled = r#true != 0;
    snprintf(
        &raw mut (*settings).hardware_provider as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        b"auto\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*settings).presence_interval_seconds = 60 as ::core::ffi::c_uint as uint32_t;
    (*settings).incident_retention_days = 90 as ::core::ffi::c_uint as uint32_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_policy_profile_from_name(
    mut name: *const ::core::ffi::c_char,
    mut profile: *mut system_health_profile_t,
) -> bool {
    if name.is_null() || profile.is_null() {
        return r#false != 0;
    }
    if strcmp(name, b"balanced\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        *profile = system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED;
    } else if strcmp(
        name,
        b"conservative\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        *profile = system_health_profile_t::SYSTEM_HEALTH_PROFILE_CONSERVATIVE;
    } else if strcmp(name, b"disabled\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        *profile = system_health_profile_t::SYSTEM_HEALTH_PROFILE_DISABLED;
    } else if strcmp(name, b"custom\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        *profile = system_health_profile_t::SYSTEM_HEALTH_PROFILE_CUSTOM;
    } else {
        return r#false != 0;
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_policy_profile_name(
    mut profile: system_health_profile_t,
) -> *const ::core::ffi::c_char {
    match profile {
        system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED => {
            return b"balanced\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_profile_t::SYSTEM_HEALTH_PROFILE_CONSERVATIVE => {
            return b"conservative\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_profile_t::SYSTEM_HEALTH_PROFILE_DISABLED => {
            return b"disabled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_profile_t::SYSTEM_HEALTH_PROFILE_CUSTOM => {
            return b"custom\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_policy_validate_settings(
    mut settings: *const system_health_policy_settings_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    clear_error(error);
    if settings.is_null() {
        return fail(
            error,
            b"health settings are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if memchr(
        &raw const (*settings).profile as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        '\0' as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
    )
    .is_null()
        || memchr(
            &raw const (*settings).hardware_provider as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            '\0' as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        )
        .is_null()
    {
        return fail(
            error,
            b"health profile and provider must be bounded strings\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut profile: system_health_profile_t =
        system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED;
    if !system_health_policy_profile_from_name(
        &raw const (*settings).profile as *const ::core::ffi::c_char,
        &raw mut profile,
    ) || profile.0 == system_health_profile_t::SYSTEM_HEALTH_PROFILE_CUSTOM.0
    {
        return fail(
            error,
            b"health profile must be balanced, conservative, or disabled\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        &raw const (*settings).hardware_provider as *const ::core::ffi::c_char,
        b"auto\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw const (*settings).hardware_provider as *const ::core::ffi::c_char,
            b"smartctl\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw const (*settings).hardware_provider as *const ::core::ffi::c_char,
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return fail(
            error,
            b"hardware provider must be auto, smartctl, or disabled\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if (*settings).fast_interval_seconds < SYSTEM_HEALTH_FAST_INTERVAL_MIN as uint32_t
        || (*settings).fast_interval_seconds > SYSTEM_HEALTH_FAST_INTERVAL_MAX as uint32_t
    {
        return fail(
            error,
            b"fast interval must be between %u and %u seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            SYSTEM_HEALTH_FAST_INTERVAL_MIN,
            SYSTEM_HEALTH_FAST_INTERVAL_MAX,
        );
    }
    if (*settings).normal_interval_seconds < SYSTEM_HEALTH_NORMAL_INTERVAL_MIN as uint32_t
        || (*settings).normal_interval_seconds > SYSTEM_HEALTH_NORMAL_INTERVAL_MAX as uint32_t
    {
        return fail(
            error,
            b"normal interval must be between %u and %u seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            SYSTEM_HEALTH_NORMAL_INTERVAL_MIN,
            SYSTEM_HEALTH_NORMAL_INTERVAL_MAX,
        );
    }
    if (*settings).slow_interval_seconds < SYSTEM_HEALTH_SLOW_INTERVAL_MIN as uint32_t
        || (*settings).slow_interval_seconds > SYSTEM_HEALTH_SLOW_INTERVAL_MAX as uint32_t
    {
        return fail(
            error,
            b"slow interval must be between %u and %u seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            SYSTEM_HEALTH_SLOW_INTERVAL_MIN,
            SYSTEM_HEALTH_SLOW_INTERVAL_MAX,
        );
    }
    if (*settings).device_interval_seconds < SYSTEM_HEALTH_DEVICE_INTERVAL_MIN as uint32_t
        || (*settings).device_interval_seconds > SYSTEM_HEALTH_DEVICE_INTERVAL_MAX as uint32_t
    {
        return fail(
            error,
            b"device interval must be between %u and %u seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            SYSTEM_HEALTH_DEVICE_INTERVAL_MIN,
            SYSTEM_HEALTH_DEVICE_INTERVAL_MAX,
        );
    }
    if (*settings).normal_interval_seconds < (*settings).fast_interval_seconds
        || (*settings).slow_interval_seconds < (*settings).normal_interval_seconds
        || (*settings).device_interval_seconds < (*settings).slow_interval_seconds
    {
        return fail(
            error,
            b"health intervals must be ordered fast <= normal <= slow <= device\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if (*settings).presence_interval_seconds < SYSTEM_HEALTH_PRESENCE_INTERVAL_MIN as uint32_t
        || (*settings).presence_interval_seconds > SYSTEM_HEALTH_PRESENCE_INTERVAL_MAX as uint32_t
    {
        return fail(
            error,
            b"presence interval must be between %u and %u seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            SYSTEM_HEALTH_PRESENCE_INTERVAL_MIN,
            SYSTEM_HEALTH_PRESENCE_INTERVAL_MAX,
        );
    }
    if (*settings).incident_retention_days < SYSTEM_HEALTH_RETENTION_DAYS_MIN as uint32_t
        || (*settings).incident_retention_days > SYSTEM_HEALTH_RETENTION_DAYS_MAX as uint32_t
    {
        return fail(
            error,
            b"incident retention must be between %u and %u days\0".as_ptr()
                as *const ::core::ffi::c_char,
            SYSTEM_HEALTH_RETENTION_DAYS_MIN,
            SYSTEM_HEALTH_RETENTION_DAYS_MAX,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn set_threshold(
    mut policy: *mut system_health_policy_t,
    mut condition: system_health_condition_t,
    mut direction: system_health_threshold_direction_t,
    mut unit: system_health_unit_t,
    mut warning: ::core::ffi::c_double,
    mut critical: ::core::ffi::c_double,
    mut recovery: ::core::ffi::c_double,
    mut warning_for: uint32_t,
    mut critical_for: uint32_t,
    mut recovery_for: uint32_t,
) {
    let mut rule: *mut system_health_condition_policy_t = (&raw mut (*policy).conditions
        as *mut system_health_condition_policy_t)
        .offset(condition.0 as isize);
    (*rule).direction = direction;
    (*rule).unit = unit;
    (*rule).warning_threshold = warning;
    (*rule).critical_threshold = critical;
    (*rule).recovery_threshold = recovery;
    (*rule).warning_for_seconds = warning_for;
    (*rule).critical_for_seconds = critical_for;
    (*rule).recovery_for_seconds = recovery_for;
}
unsafe extern "C" fn fill_balanced(mut policy: *mut system_health_policy_t) {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int {
        let mut rule: *mut system_health_condition_policy_t = (&raw mut (*policy).conditions
            as *mut system_health_condition_policy_t)
            .offset(index as isize);
        memset(
            rule as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<system_health_condition_policy_t>(),
        );
        (*rule).condition = system_health_condition_t(index as ::core::ffi::c_uint);
        (*rule).profile = system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED;
        (*rule).enabled = r#true != 0;
        (*rule).unit = system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE;
        index += 1;
    }
    set_threshold(
        policy,
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        0.15f64,
        0.08f64,
        0.20f64,
        120 as uint32_t,
        30 as uint32_t,
        300 as uint32_t,
    );
    set_threshold(
        policy,
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_CPU_SATURATION,
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        0.90f64,
        0.95f64,
        0.75f64,
        300 as uint32_t,
        300 as uint32_t,
        300 as uint32_t,
    );
    set_threshold(
        policy,
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_CPU_THROTTLED,
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        0.10f64,
        0.30f64,
        0.05f64,
        300 as uint32_t,
        300 as uint32_t,
        300 as uint32_t,
    );
    set_threshold(
        policy,
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_IO_PRESSURE,
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_SECONDS,
        0.50f64,
        2.0f64,
        0.20f64,
        300 as uint32_t,
        0 as uint32_t,
        600 as uint32_t,
    );
    set_threshold(
        policy,
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW,
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        0.15f64,
        0.05f64,
        0.20f64,
        300 as uint32_t,
        60 as uint32_t,
        300 as uint32_t,
    );
    set_threshold(
        policy,
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW,
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        0.10f64,
        0.05f64,
        0.15f64,
        300 as uint32_t,
        60 as uint32_t,
        300 as uint32_t,
    );
    set_threshold(
        policy,
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE,
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        0.01f64,
        0.05f64,
        0.005f64,
        300 as uint32_t,
        300 as uint32_t,
        600 as uint32_t,
    );
    set_threshold(
        policy,
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION,
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        0.80f64,
        0.90f64,
        0.70f64,
        300 as uint32_t,
        60 as uint32_t,
        300 as uint32_t,
    );
    set_threshold(
        policy,
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION,
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        0.80f64,
        0.90f64,
        0.70f64,
        300 as uint32_t,
        60 as uint32_t,
        300 as uint32_t,
    );
    (*policy).conditions[system_health_condition_t::SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN.0
        as ::core::ffi::c_int as usize]
        .warning_for_seconds = 30 as ::core::ffi::c_uint as uint32_t;
    (*policy).conditions[system_health_condition_t::SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN.0
        as ::core::ffi::c_int as usize]
        .critical_for_seconds = 300 as ::core::ffi::c_uint as uint32_t;
    (*policy).conditions[system_health_condition_t::SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN.0
        as ::core::ffi::c_int as usize]
        .recovery_for_seconds = 120 as ::core::ffi::c_uint as uint32_t;
    (*policy)
        .conditions[system_health_condition_t::SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED
            .0 as ::core::ffi::c_int as usize]
        .warning_for_seconds = 600 as ::core::ffi::c_uint as uint32_t;
    (*policy)
        .conditions[system_health_condition_t::SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED
            .0 as ::core::ffi::c_int as usize]
        .recovery_for_seconds = 300 as ::core::ffi::c_uint as uint32_t;
    (*policy)
        .conditions[system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED
            .0 as ::core::ffi::c_int as usize]
        .recovery_for_seconds = 300 as ::core::ffi::c_uint as uint32_t;
    (*policy)
        .conditions[system_health_condition_t::SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY
            .0 as ::core::ffi::c_int as usize]
        .recovery_for_seconds = 300 as ::core::ffi::c_uint as uint32_t;
}
unsafe extern "C" fn apply_conservative(mut policy: *mut system_health_policy_t) {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int {
        let mut rule: *mut system_health_condition_policy_t = (&raw mut (*policy).conditions
            as *mut system_health_condition_policy_t)
            .offset(index as isize);
        (*rule).profile = system_health_profile_t::SYSTEM_HEALTH_PROFILE_CONSERVATIVE;
        if (*rule).direction.0
            == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE.0
        {
            (*rule).warning_threshold *= 1.25f64;
            (*rule).critical_threshold *= 1.25f64;
            (*rule).recovery_threshold *= 1.25f64;
        } else if (*rule).direction.0
            == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE.0
            && (*rule).unit.0 == system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO.0
        {
            (*rule).warning_threshold *= 0.90f64;
            (*rule).critical_threshold *= 0.90f64;
            (*rule).recovery_threshold *= 0.90f64;
        }
        if (*rule).warning_for_seconds > 30 as uint32_t {
            (*rule).warning_for_seconds = ((*rule).warning_for_seconds as ::core::ffi::c_uint)
                .wrapping_div(2 as ::core::ffi::c_uint)
                as uint32_t;
        }
        if (*rule).critical_for_seconds > 30 as uint32_t {
            (*rule).critical_for_seconds = ((*rule).critical_for_seconds as ::core::ffi::c_uint)
                .wrapping_div(2 as ::core::ffi::c_uint)
                as uint32_t;
        }
        index += 1;
    }
}
unsafe extern "C" fn fill_for_profile(
    mut profile: system_health_profile_t,
    mut policy: *mut system_health_policy_t,
) {
    fill_balanced(policy);
    if profile.0 == system_health_profile_t::SYSTEM_HEALTH_PROFILE_CONSERVATIVE.0 {
        apply_conservative(policy);
    } else if profile.0 == system_health_profile_t::SYSTEM_HEALTH_PROFILE_DISABLED.0 {
        let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while index
            < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int
        {
            (*policy).conditions[index as usize].profile =
                system_health_profile_t::SYSTEM_HEALTH_PROFILE_DISABLED;
            (*policy).conditions[index as usize].enabled = r#false != 0;
            index += 1;
        }
    }
}
unsafe extern "C" fn valid_number(mut number: ::core::ffi::c_double) -> bool {
    return number.is_finite() as ::core::ffi::c_int != 0
        && number >= -1000000000000.0f64
        && number <= 1000000000000.0f64;
}
unsafe extern "C" fn validate_condition(
    mut rule: *const system_health_condition_policy_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut code: *const ::core::ffi::c_char = system_health_condition_code((*rule).condition);
    if code.is_null() {
        return fail(
            error,
            b"policy contains an unknown condition\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*rule).warning_for_seconds > POLICY_DWELL_MAX_SECONDS as uint32_t
        || (*rule).critical_for_seconds > POLICY_DWELL_MAX_SECONDS as uint32_t
        || (*rule).recovery_for_seconds > POLICY_DWELL_MAX_SECONDS as uint32_t
    {
        return fail(
            error,
            b"%s dwell exceeds seven days\0".as_ptr() as *const ::core::ffi::c_char,
            code,
        );
    }
    if (*rule).direction.0 == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE.0
        || !(*rule).enabled
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*rule).warning_for_seconds == 0 as uint32_t || (*rule).recovery_for_seconds == 0 as uint32_t
    {
        return fail(
            error,
            b"%s warning and recovery dwell must be nonzero\0".as_ptr()
                as *const ::core::ffi::c_char,
            code,
        );
    }
    if !valid_number((*rule).warning_threshold)
        || !valid_number((*rule).critical_threshold)
        || !valid_number((*rule).recovery_threshold)
    {
        return fail(
            error,
            b"%s has an invalid numeric threshold\0".as_ptr() as *const ::core::ffi::c_char,
            code,
        );
    }
    if (*rule).unit.0 == system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO.0
        && ((*rule).warning_threshold < 0.0f64
            || (*rule).warning_threshold > 1.0f64
            || (*rule).critical_threshold < 0.0f64
            || (*rule).critical_threshold > 1.0f64
            || (*rule).recovery_threshold < 0.0f64
            || (*rule).recovery_threshold > 1.0f64)
    {
        return fail(
            error,
            b"%s ratio thresholds must be between zero and one\0".as_ptr()
                as *const ::core::ffi::c_char,
            code,
        );
    }
    if (*rule).direction.0
        == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE.0
        && !((*rule).critical_threshold < (*rule).warning_threshold
            && (*rule).warning_threshold < (*rule).recovery_threshold)
    {
        return fail(
            error,
            b"%s requires critical < warning < recovery\0".as_ptr() as *const ::core::ffi::c_char,
            code,
        );
    }
    if (*rule).direction.0
        == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE.0
        && !((*rule).recovery_threshold < (*rule).warning_threshold
            && (*rule).warning_threshold < (*rule).critical_threshold)
    {
        return fail(
            error,
            b"%s requires recovery < warning < critical\0".as_ptr() as *const ::core::ffi::c_char,
            code,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn object_keys_exact(
    mut object: *const cJSON,
    mut allowed: *const *const ::core::ffi::c_char,
    mut allowed_count: size_t,
    mut expected_count: size_t,
) -> bool {
    let mut count: size_t = 0 as size_t;
    let mut child: *const cJSON = (*object).child;
    while !child.is_null() {
        let mut matched: bool = r#false != 0;
        let mut index: size_t = 0 as size_t;
        while index < allowed_count {
            if !(*child).string.is_null()
                && strcmp((*child).string, *allowed.offset(index as isize))
                    == 0 as ::core::ffi::c_int
            {
                if cJSON_GetObjectItemCaseSensitive(object, *allowed.offset(index as isize))
                    != child as *mut cJSON
                {
                    return r#false != 0;
                }
                matched = r#true != 0;
                break;
            } else {
                index = index.wrapping_add(1);
            }
        }
        if !matched {
            return r#false != 0;
        }
        count = count.wrapping_add(1);
        child = (*child).next;
    }
    return count == expected_count;
}
unsafe extern "C" fn json_u32(mut value: *const cJSON, mut output: *mut uint32_t) -> bool {
    if cJSON_IsNumber(value) == 0
        || (*value).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || (*value).valuedouble < 0.0f64
        || (*value).valuedouble > POLICY_DWELL_MAX_SECONDS as ::core::ffi::c_double
    {
        return r#false != 0;
    }
    let mut integer: uint32_t = (*value).valuedouble as uint32_t;
    if integer as ::core::ffi::c_double != (*value).valuedouble {
        return r#false != 0;
    }
    *output = integer;
    return r#true != 0;
}
unsafe extern "C" fn unit_from_name(
    mut name: *const ::core::ffi::c_char,
    mut unit: *mut system_health_unit_t,
) -> bool {
    if name.is_null() || unit.is_null() {
        return r#false != 0;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index <= system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN.0 as ::core::ffi::c_int {
        let mut candidate: system_health_unit_t =
            system_health_unit_t(index as ::core::ffi::c_uint);
        if strcmp(name, system_health_unit_name(candidate)) == 0 as ::core::ffi::c_int {
            *unit = candidate;
            return r#true != 0;
        }
        index += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn apply_profile_override(
    mut policy: *mut system_health_policy_t,
    mut condition: system_health_condition_t,
    mut profile: system_health_profile_t,
) -> ::core::ffi::c_int {
    let mut defaults: system_health_policy_t = system_health_policy_t {
        generation: 0,
        settings: system_health_policy_settings_t {
            enabled: false,
            profile: [0; 16],
            fast_interval_seconds: 0,
            normal_interval_seconds: 0,
            slow_interval_seconds: 0,
            device_interval_seconds: 0,
            write_probe_enabled: false,
            hardware_provider: [0; 16],
            presence_interval_seconds: 0,
            incident_retention_days: 0,
        },
        conditions: [system_health_condition_policy_t {
            condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
            profile: system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED,
            enabled: false,
            overridden: false,
            direction: system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE,
            unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
            warning_threshold: 0.,
            critical_threshold: 0.,
            recovery_threshold: 0.,
            warning_for_seconds: 0,
            critical_for_seconds: 0,
            recovery_for_seconds: 0,
        }; 27],
    };
    memset(
        &raw mut defaults as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_policy_t>(),
    );
    fill_for_profile(profile, &raw mut defaults);
    (*policy).conditions[condition.0 as usize] = defaults.conditions[condition.0 as usize];
    (*policy).conditions[condition.0 as usize].overridden = r#true != 0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn apply_override_item(
    mut item: *const cJSON,
    mut seen: *mut bool,
    mut policy: *mut system_health_policy_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut profile_keys: [*const ::core::ffi::c_char; 2] = [
        b"code\0".as_ptr() as *const ::core::ffi::c_char,
        b"profile\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut custom_keys: [*const ::core::ffi::c_char; 8] = [
        b"code\0".as_ptr() as *const ::core::ffi::c_char,
        b"unit\0".as_ptr() as *const ::core::ffi::c_char,
        b"warning\0".as_ptr() as *const ::core::ffi::c_char,
        b"critical\0".as_ptr() as *const ::core::ffi::c_char,
        b"recovery\0".as_ptr() as *const ::core::ffi::c_char,
        b"warning_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"critical_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"recovery_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if cJSON_IsObject(item) == 0 {
        return fail(
            error,
            b"each health override must be an object\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut code_json: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(item, b"code\0".as_ptr() as *const ::core::ffi::c_char);
    if cJSON_IsString(code_json) == 0 || (*code_json).valuestring.is_null() {
        return fail(
            error,
            b"each health override requires a condition code\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut condition: system_health_condition_t =
        system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW;
    if !system_health_condition_from_code((*code_json).valuestring, &raw mut condition) {
        return fail(
            error,
            b"unknown health condition code: %s\0".as_ptr() as *const ::core::ffi::c_char,
            (*code_json).valuestring,
        );
    }
    if *seen.offset(condition.0 as isize) {
        return fail(
            error,
            b"duplicate health condition override: %s\0".as_ptr() as *const ::core::ffi::c_char,
            (*code_json).valuestring,
        );
    }
    *seen.offset(condition.0 as isize) = r#true != 0;
    let mut profile_json: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(item, b"profile\0".as_ptr() as *const ::core::ffi::c_char);
    if !profile_json.is_null() {
        if !object_keys_exact(
            item,
            &raw const profile_keys as *const *const ::core::ffi::c_char,
            2 as size_t,
            2 as size_t,
        ) || cJSON_IsString(profile_json) == 0
            || (*profile_json).valuestring.is_null()
        {
            return fail(
                error,
                b"%s profile override must contain only code and profile\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*code_json).valuestring,
            );
        }
        let mut profile: system_health_profile_t =
            system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED;
        if !system_health_policy_profile_from_name((*profile_json).valuestring, &raw mut profile)
            || profile.0 == system_health_profile_t::SYSTEM_HEALTH_PROFILE_CUSTOM.0
        {
            return fail(
                error,
                b"%s has an invalid override profile\0".as_ptr() as *const ::core::ffi::c_char,
                (*code_json).valuestring,
            );
        }
        return apply_profile_override(policy, condition, profile);
    }
    if !object_keys_exact(
        item,
        &raw const custom_keys as *const *const ::core::ffi::c_char,
        8 as size_t,
        8 as size_t,
    ) {
        return fail(
            error,
            b"%s custom override must be complete and contain no extra fields\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*code_json).valuestring,
        );
    }
    let mut rule: *mut system_health_condition_policy_t = (&raw mut (*policy).conditions
        as *mut system_health_condition_policy_t)
        .offset(condition.0 as isize);
    if (*rule).direction.0 == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE.0 {
        return fail(
            error,
            b"%s does not accept numeric threshold overrides\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*code_json).valuestring,
        );
    }
    let mut unit_json: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(item, b"unit\0".as_ptr() as *const ::core::ffi::c_char);
    let mut unit: system_health_unit_t = system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE;
    if cJSON_IsString(unit_json) == 0
        || (*unit_json).valuestring.is_null()
        || !unit_from_name((*unit_json).valuestring, &raw mut unit)
        || unit.0 != (*rule).unit.0
    {
        return fail(
            error,
            b"%s override unit must be %s\0".as_ptr() as *const ::core::ffi::c_char,
            (*code_json).valuestring,
            system_health_unit_name((*rule).unit),
        );
    }
    let mut warning: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(item, b"warning\0".as_ptr() as *const ::core::ffi::c_char);
    let mut critical: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        item,
        b"critical\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut recovery: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        item,
        b"recovery\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsNumber(warning) == 0
        || cJSON_IsNumber(critical) == 0
        || cJSON_IsNumber(recovery) == 0
    {
        return fail(
            error,
            b"%s thresholds must be numbers\0".as_ptr() as *const ::core::ffi::c_char,
            (*code_json).valuestring,
        );
    }
    (*rule).warning_threshold = (*warning).valuedouble;
    (*rule).critical_threshold = (*critical).valuedouble;
    (*rule).recovery_threshold = (*recovery).valuedouble;
    if !json_u32(
        cJSON_GetObjectItemCaseSensitive(
            item,
            b"warning_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        ),
        &raw mut (*rule).warning_for_seconds,
    ) || !json_u32(
        cJSON_GetObjectItemCaseSensitive(
            item,
            b"critical_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        ),
        &raw mut (*rule).critical_for_seconds,
    ) || !json_u32(
        cJSON_GetObjectItemCaseSensitive(
            item,
            b"recovery_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        ),
        &raw mut (*rule).recovery_for_seconds,
    ) {
        return fail(
            error,
            b"%s dwell values must be whole seconds up to seven days\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*code_json).valuestring,
        );
    }
    (*rule).profile = system_health_profile_t::SYSTEM_HEALTH_PROFILE_CUSTOM;
    (*rule).enabled = r#true != 0;
    (*rule).overridden = r#true != 0;
    return validate_condition(rule, error);
}
unsafe extern "C" fn override_json_for_policy(
    mut policy: *const system_health_policy_t,
) -> *mut cJSON {
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut conditions: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || conditions.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(conditions);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddNumberToObject(
        root,
        b"version\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"conditions\0".as_ptr() as *const ::core::ffi::c_char,
        conditions,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int {
        let mut rule: *const system_health_condition_policy_t = (&raw const (*policy).conditions
            as *const system_health_condition_policy_t)
            .offset(index as isize);
        if (*rule).overridden {
            let mut item: *mut cJSON = cJSON_CreateObject();
            if item.is_null() {
                cJSON_Delete(root);
                return ::core::ptr::null_mut::<cJSON>();
            }
            cJSON_AddStringToObject(
                item,
                b"code\0".as_ptr() as *const ::core::ffi::c_char,
                system_health_condition_code((*rule).condition),
            );
            if (*rule).profile.0 != system_health_profile_t::SYSTEM_HEALTH_PROFILE_CUSTOM.0 {
                cJSON_AddStringToObject(
                    item,
                    b"profile\0".as_ptr() as *const ::core::ffi::c_char,
                    system_health_policy_profile_name((*rule).profile),
                );
            } else {
                cJSON_AddStringToObject(
                    item,
                    b"unit\0".as_ptr() as *const ::core::ffi::c_char,
                    system_health_unit_name((*rule).unit),
                );
                cJSON_AddNumberToObject(
                    item,
                    b"warning\0".as_ptr() as *const ::core::ffi::c_char,
                    (*rule).warning_threshold,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"critical\0".as_ptr() as *const ::core::ffi::c_char,
                    (*rule).critical_threshold,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"recovery\0".as_ptr() as *const ::core::ffi::c_char,
                    (*rule).recovery_threshold,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"warning_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
                    (*rule).warning_for_seconds as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"critical_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
                    (*rule).critical_for_seconds as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"recovery_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
                    (*rule).recovery_for_seconds as ::core::ffi::c_double,
                );
            }
            cJSON_AddItemToArray(conditions, item);
        }
        index += 1;
    }
    return root;
}
unsafe extern "C" fn print_json(
    mut root: *mut cJSON,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    if output.is_null() || output_size == 0 as size_t {
        return if !root.is_null() {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    let mut printed: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    if printed.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut length: size_t = strlen(printed);
    if length >= output_size {
        free(printed as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    memcpy(
        output as *mut ::core::ffi::c_void,
        printed as *const ::core::ffi::c_void,
        length.wrapping_add(1 as size_t),
    );
    free(printed as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_policy_build(
    mut settings: *const system_health_policy_settings_t,
    mut override_json: *const ::core::ffi::c_char,
    mut policy: *mut system_health_policy_t,
    mut canonical_json: *mut ::core::ffi::c_char,
    mut canonical_json_size: size_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    clear_error(error);
    if policy.is_null() {
        return fail(
            error,
            b"health policy output is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !canonical_json.is_null() && canonical_json_size > 0 as size_t {
        *canonical_json.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if system_health_policy_validate_settings(settings, error) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut profile: system_health_profile_t =
        system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED;
    system_health_policy_profile_from_name(
        &raw const (*settings).profile as *const ::core::ffi::c_char,
        &raw mut profile,
    );
    memset(
        policy as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_policy_t>(),
    );
    (*policy).settings = *settings;
    fill_for_profile(profile, policy);
    if !(*settings).enabled {
        let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while index
            < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int
        {
            (*policy).conditions[index as usize].enabled = r#false != 0;
            index += 1;
        }
    }
    if !override_json.is_null()
        && *override_json.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        let mut length: size_t = strlen(override_json);
        if length > SYSTEM_HEALTH_OVERRIDE_JSON_MAX as size_t {
            return fail(
                error,
                b"health overrides exceed %u bytes\0".as_ptr() as *const ::core::ffi::c_char,
                SYSTEM_HEALTH_OVERRIDE_JSON_MAX,
            );
        }
        let mut parse_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut root: *mut cJSON = cJSON_ParseWithOpts(override_json, &raw mut parse_end, r#true);
        static mut root_keys: [*const ::core::ffi::c_char; 2] = [
            b"version\0".as_ptr() as *const ::core::ffi::c_char,
            b"conditions\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        if root.is_null()
            || cJSON_IsObject(root) == 0
            || !object_keys_exact(
                root,
                &raw const root_keys as *const *const ::core::ffi::c_char,
                2 as size_t,
                2 as size_t,
            )
        {
            cJSON_Delete(root);
            return fail(
                error,
                b"health overrides must be a versioned conditions object\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        let mut version: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            root,
            b"version\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut conditions: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            root,
            b"conditions\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsNumber(version) == 0
            || (*version).valuedouble != 1.0f64
            || cJSON_IsArray(conditions) == 0
        {
            cJSON_Delete(root);
            return fail(
                error,
                b"health override version must be 1 with a conditions array\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        let mut count: ::core::ffi::c_int = cJSON_GetArraySize(conditions);
        if count < 0 as ::core::ffi::c_int
            || count
                > system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int
        {
            cJSON_Delete(root);
            return fail(
                error,
                b"too many health condition overrides\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut seen: [bool; 27] = [
            r#false != 0,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
        ];
        let mut index_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while index_0 < count {
            if apply_override_item(
                cJSON_GetArrayItem(conditions, index_0),
                &raw mut seen as *mut bool,
                policy,
                error,
            ) != 0 as ::core::ffi::c_int
            {
                cJSON_Delete(root);
                return -1 as ::core::ffi::c_int;
            }
            index_0 += 1;
        }
        cJSON_Delete(root);
    }
    let mut index_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index_1 < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int
    {
        if !(*settings).enabled {
            (*policy).conditions[index_1 as usize].enabled = r#false != 0;
        }
        if validate_condition(
            (&raw mut (*policy).conditions as *mut system_health_condition_policy_t)
                .offset(index_1 as isize),
            error,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        index_1 += 1;
    }
    let mut canonical: *mut cJSON = override_json_for_policy(policy);
    if canonical.is_null() {
        return fail(
            error,
            b"could not allocate canonical health overrides\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut print_result: ::core::ffi::c_int =
        print_json(canonical, canonical_json, canonical_json_size);
    cJSON_Delete(canonical);
    if print_result != 0 as ::core::ffi::c_int {
        return fail(
            error,
            b"canonical health overrides do not fit output buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn initialize_active_policy() {
    let mut settings: system_health_policy_settings_t = system_health_policy_settings_t {
        enabled: false,
        profile: [0; 16],
        fast_interval_seconds: 0,
        normal_interval_seconds: 0,
        slow_interval_seconds: 0,
        device_interval_seconds: 0,
        write_probe_enabled: false,
        hardware_provider: [0; 16],
        presence_interval_seconds: 0,
        incident_retention_days: 0,
    };
    system_health_policy_settings_defaults(&raw mut settings);
    let mut error: [::core::ffi::c_char; 256] = [0; 256];
    if system_health_policy_build(
        &raw mut settings,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut active_policy,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
        &raw mut error as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        memset(
            &raw mut active_policy as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<system_health_policy_t>(),
        );
    }
    active_policy.generation = 1 as uint64_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_policy_replace(
    mut candidate: *const system_health_policy_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    clear_error(error);
    if candidate.is_null() {
        return fail(
            error,
            b"health policy candidate is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if system_health_policy_validate_settings(&raw const (*candidate).settings, error)
        != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int {
        if (*candidate).conditions[index as usize].condition.0
            != system_health_condition_t(index as ::core::ffi::c_uint).0
        {
            return fail(
                error,
                b"invalid complete health policy at condition %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                index,
            );
        }
        if validate_condition(
            (&raw const (*candidate).conditions as *const system_health_condition_policy_t)
                .offset(index as isize),
            error,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        index += 1;
    }
    pthread_once(
        &raw mut active_policy_once,
        Some(initialize_active_policy as unsafe extern "C" fn() -> ()),
    );
    pthread_rwlock_wrlock(&raw mut active_policy_lock);
    let mut generation: uint64_t = if active_policy.generation == UINT64_MAX as uint64_t {
        1 as uint64_t
    } else {
        active_policy.generation.wrapping_add(1 as uint64_t)
    };
    active_policy = *candidate;
    active_policy.generation = generation;
    pthread_rwlock_unlock(&raw mut active_policy_lock);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_policy_snapshot(
    mut snapshot: *mut system_health_policy_t,
) -> ::core::ffi::c_int {
    if snapshot.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_once(
        &raw mut active_policy_once,
        Some(initialize_active_policy as unsafe extern "C" fn() -> ()),
    );
    pthread_rwlock_rdlock(&raw mut active_policy_lock);
    *snapshot = active_policy;
    pthread_rwlock_unlock(&raw mut active_policy_lock);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_health_policy_serialize(
    mut policy: *const system_health_policy_t,
    mut json: *mut ::core::ffi::c_char,
    mut json_size: size_t,
) -> ::core::ffi::c_int {
    if policy.is_null() || json.is_null() || json_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut conditions: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || conditions.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(conditions);
        return -1 as ::core::ffi::c_int;
    }
    cJSON_AddBoolToObject(
        root,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).settings.enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        root,
        b"profile\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*policy).settings.profile as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        root,
        b"generation\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).generation as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"conditions\0".as_ptr() as *const ::core::ffi::c_char,
        conditions,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int {
        let mut rule: *const system_health_condition_policy_t = (&raw const (*policy).conditions
            as *const system_health_condition_policy_t)
            .offset(index as isize);
        let mut item: *mut cJSON = cJSON_CreateObject();
        if item.is_null() {
            cJSON_Delete(root);
            return -1 as ::core::ffi::c_int;
        }
        cJSON_AddStringToObject(
            item,
            b"code\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_condition_code((*rule).condition),
        );
        cJSON_AddBoolToObject(
            item,
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
            (*rule).enabled as cJSON_bool,
        );
        cJSON_AddStringToObject(
            item,
            b"profile\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_policy_profile_name((*rule).profile),
        );
        cJSON_AddStringToObject(
            item,
            b"unit\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_unit_name((*rule).unit),
        );
        if (*rule).direction.0
            != system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE.0
        {
            cJSON_AddNumberToObject(
                item,
                b"warning\0".as_ptr() as *const ::core::ffi::c_char,
                (*rule).warning_threshold,
            );
            cJSON_AddNumberToObject(
                item,
                b"critical\0".as_ptr() as *const ::core::ffi::c_char,
                (*rule).critical_threshold,
            );
            cJSON_AddNumberToObject(
                item,
                b"recovery\0".as_ptr() as *const ::core::ffi::c_char,
                (*rule).recovery_threshold,
            );
        }
        cJSON_AddNumberToObject(
            item,
            b"warning_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            (*rule).warning_for_seconds as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"critical_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            (*rule).critical_for_seconds as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"recovery_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            (*rule).recovery_for_seconds as ::core::ffi::c_double,
        );
        cJSON_AddItemToArray(conditions, item);
        index += 1;
    }
    let mut result: ::core::ffi::c_int = print_json(root, json, json_size);
    cJSON_Delete(root);
    return result;
}
