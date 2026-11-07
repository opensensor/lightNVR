unsafe extern "C" {
    unsafe fn health_helper_run(
        request: *const health_helper_request_t,
        result: *mut health_helper_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_observation_set_available(
        observation: *mut system_health_observation_t,
        value: ::core::ffi::c_double,
        unit: system_health_unit_t,
    );
    unsafe fn system_health_observation_set_unavailable(
        observation: *mut system_health_observation_t,
        capability: system_health_capability_t,
    );
    unsafe fn system_health_observation_sink_append(
        sink: *mut system_health_observation_sink_t,
        observation: *const system_health_observation_t,
    ) -> bool;
    unsafe fn linux_device_map_build(
        sys_root: *const ::core::ffi::c_char,
        installation_scope: *const ::core::ffi::c_char,
        map: *mut linux_device_map_t,
    ) -> ::core::ffi::c_int;
    unsafe fn linux_device_map_find(
        map: *const linux_device_map_t,
        physical_name_0: *const ::core::ffi::c_char,
    ) -> *const linux_device_map_entry_t;
    unsafe fn cJSON_ParseWithLengthOpts(
        value: *const ::core::ffi::c_char,
        buffer_length: size_t,
        return_parse_end: *mut *const ::core::ffi::c_char,
        require_null_terminated: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn __ctype_tolower_loc() -> *mut *const __int32_t;
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_request_tier(tier: system_health_sampling_tier_t) -> bool;
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type int64_t = i64;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type uint_fast64_t = ::core::ffi::c_ulong;
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
pub type atomic_bool = bool;
pub type atomic_uint_fast64_t = uint_fast64_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct health_helper_outcome_t(pub ::core::ffi::c_uint);
impl health_helper_outcome_t {
    pub const HEALTH_HELPER_OK: Self = Self(0);
    pub const HEALTH_HELPER_EXITED: Self = Self(1);
    pub const HEALTH_HELPER_TIMED_OUT: Self = Self(2);
    pub const HEALTH_HELPER_BUSY: Self = Self(3);
    pub const HEALTH_HELPER_EXEC_ERROR: Self = Self(4);
    pub const HEALTH_HELPER_SYSTEM_ERROR: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct health_helper_request_t {
    pub program: *const ::core::ffi::c_char,
    pub argv: *const *mut ::core::ffi::c_char,
    pub timeout_ms: uint32_t,
    pub terminate_grace_ms: uint32_t,
    pub output_limit: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct health_helper_result_t {
    pub outcome: health_helper_outcome_t,
    pub exit_code: ::core::ffi::c_int,
    pub term_signal: ::core::ffi::c_int,
    pub latency_ms: uint32_t,
    pub output_truncated: bool,
    pub abandoned: bool,
    pub output_length: size_t,
    pub output: [::core::ffi::c_char; 4097],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_scope_t(pub ::core::ffi::c_uint);
impl system_health_scope_t {
    pub const SYSTEM_HEALTH_SCOPE_PROCESS: Self = Self(0);
    pub const SYSTEM_HEALTH_SCOPE_CONTAINER: Self = Self(1);
    pub const SYSTEM_HEALTH_SCOPE_HOST: Self = Self(2);
    pub const SYSTEM_HEALTH_SCOPE_FILESYSTEM: Self = Self(3);
    pub const SYSTEM_HEALTH_SCOPE_DEVICE: Self = Self(4);
    pub const SYSTEM_HEALTH_SCOPE_COUNT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_capability_t(pub ::core::ffi::c_uint);
impl system_health_capability_t {
    pub const SYSTEM_HEALTH_CAPABILITY_AVAILABLE: Self = Self(0);
    pub const SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED: Self = Self(1);
    pub const SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED: Self = Self(2);
    pub const SYSTEM_HEALTH_CAPABILITY_STALE: Self = Self(3);
    pub const SYSTEM_HEALTH_CAPABILITY_ERROR: Self = Self(4);
    pub const SYSTEM_HEALTH_CAPABILITY_COUNT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_freshness_t(pub ::core::ffi::c_uint);
impl system_health_freshness_t {
    pub const SYSTEM_HEALTH_FRESHNESS_UNKNOWN: Self = Self(0);
    pub const SYSTEM_HEALTH_FRESHNESS_FRESH: Self = Self(1);
    pub const SYSTEM_HEALTH_FRESHNESS_STALE: Self = Self(2);
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_observation_t {
    pub metric: [::core::ffi::c_char; 64],
    pub resource_id: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub capability: system_health_capability_t,
    pub freshness: system_health_freshness_t,
    pub unit: system_health_unit_t,
    pub value_valid: bool,
    pub value: ::core::ffi::c_double,
    pub sampled_monotonic_ms: uint64_t,
    pub observed_wall_time_ms: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_sampling_tier_t(pub ::core::ffi::c_uint);
impl system_health_sampling_tier_t {
    pub const SYSTEM_HEALTH_TIER_FAST: Self = Self(0);
    pub const SYSTEM_HEALTH_TIER_NORMAL: Self = Self(1);
    pub const SYSTEM_HEALTH_TIER_SLOW: Self = Self(2);
    pub const SYSTEM_HEALTH_TIER_DEVICE: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_collect_context_t {
    pub monotonic_ms: uint64_t,
    pub wall_time_ms: int64_t,
    pub proc_root: *const ::core::ffi::c_char,
    pub sys_root: *const ::core::ffi::c_char,
    pub cgroup_root: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_observation_sink_t {
    pub items: *mut system_health_observation_t,
    pub capacity: size_t,
    pub count: size_t,
    pub dropped: size_t,
}
pub type system_health_collect_fn = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const system_health_collect_context_t,
        *mut system_health_observation_sink_t,
    ) -> ::core::ffi::c_int,
>;
pub type system_health_collector_destroy_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_provider_resource_t {
    pub id: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub capability: system_health_capability_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_provider_inventory_t {
    pub resources: [system_health_provider_resource_t; 16],
    pub count: size_t,
    pub dropped: size_t,
}
pub type system_health_provider_discover_fn = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const system_health_collect_context_t,
        *mut system_health_provider_inventory_t,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_provider_t {
    pub name: [::core::ffi::c_char; 32],
    pub capability: system_health_capability_t,
    pub state: *mut ::core::ffi::c_void,
    pub discover: system_health_provider_discover_fn,
    pub collect: system_health_collect_fn,
    pub destroy: system_health_collector_destroy_fn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_device_map_entry_t {
    pub sysfs_name: [::core::ffi::c_char; 64],
    pub public_id: [::core::ffi::c_char; 64],
    pub filesystem_device: uint64_t,
    pub target_mapped: bool,
    pub target_is_default: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_device_map_t {
    pub entries: [linux_device_map_entry_t; 16],
    pub count: size_t,
    pub dropped: size_t,
    pub capability: system_health_capability_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct smartctl_parse_status_t(pub ::core::ffi::c_uint);
impl smartctl_parse_status_t {
    pub const SMARTCTL_PARSE_OK: Self = Self(0);
    pub const SMARTCTL_PARSE_SLEEPING: Self = Self(1);
    pub const SMARTCTL_PARSE_PERMISSION_DENIED: Self = Self(2);
    pub const SMARTCTL_PARSE_UNSUPPORTED_VERSION: Self = Self(3);
    pub const SMARTCTL_PARSE_MALFORMED: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smartctl_u64_value_t {
    pub valid: bool,
    pub value: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smartctl_double_value_t {
    pub valid: bool,
    pub value: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smartctl_normalized_sample_t {
    pub status: smartctl_parse_status_t,
    pub capability: system_health_capability_t,
    pub exit_status: ::core::ffi::c_int,
    pub health_valid: bool,
    pub health_passed: bool,
    pub prefail: bool,
    pub critical: bool,
    pub temperature_celsius: smartctl_double_value_t,
    pub available_spare_ratio: smartctl_double_value_t,
    pub percentage_used_ratio: smartctl_double_value_t,
    pub reallocated_sectors: smartctl_u64_value_t,
    pub pending_sectors: smartctl_u64_value_t,
    pub uncorrectable_errors: smartctl_u64_value_t,
    pub interface_crc_errors: smartctl_u64_value_t,
    pub media_errors: smartctl_u64_value_t,
    pub unsafe_shutdowns: smartctl_u64_value_t,
}
pub type smartctl_helper_run_fn = Option<
    unsafe extern "C" fn(
        *const health_helper_request_t,
        *mut health_helper_result_t,
    ) -> ::core::ffi::c_int,
>;
pub type smartctl_wake_device_tier_fn = Option<unsafe extern "C" fn() -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smartctl_provider_device_t {
    pub device_path: [::core::ffi::c_char; 256],
    pub sysfs_name: [::core::ffi::c_char; 64],
    pub public_id: [::core::ffi::c_char; 64],
    pub previous_uncorrectable: uint64_t,
    pub previous_crc: uint64_t,
    pub previous_media: uint64_t,
    pub previous_unsafe_shutdowns: uint64_t,
    pub previous_monotonic_ms: uint64_t,
    pub previous_uncorrectable_valid: bool,
    pub previous_crc_valid: bool,
    pub previous_media_valid: bool,
    pub previous_unsafe_valid: bool,
    pub target_managed: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smartctl_provider_state_t {
    pub devices: [smartctl_provider_device_t; 16],
    pub device_count: size_t,
    pub devices_dropped: size_t,
    pub discover_target_devices: bool,
    pub installation_scope: [::core::ffi::c_char; 96],
    pub program: [::core::ffi::c_char; 256],
    pub timeout_ms: uint32_t,
    pub terminate_grace_ms: uint32_t,
    pub run_helper: smartctl_helper_run_fn,
    pub wake_device_tier: smartctl_wake_device_tier_fn,
    pub capability: system_health_capability_t,
    pub refresh_pending: atomic_bool,
    pub refresh_requests: atomic_uint_fast64_t,
    pub refresh_coalesced: atomic_uint_fast64_t,
    pub refresh_collections: atomic_uint_fast64_t,
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
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const HEALTH_HELPER_OUTPUT_MAX: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_MAX_DEVICES: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const LINUX_HARDWARE_INTERNAL_NAME_LENGTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const SMARTCTL_PROVIDER_MAX_DEVICES: ::core::ffi::c_uint = SYSTEM_HEALTH_MAX_DEVICES;
pub const SMARTCTL_PROVIDER_PATH_LENGTH: ::core::ffi::c_uint = 256 as ::core::ffi::c_uint;
pub const SMARTCTL_PROVIDER_JSON_NODES_MAX: ::core::ffi::c_uint = 512 as ::core::ffi::c_uint;
pub const SMARTCTL_PROVIDER_ATTRIBUTES_MAX: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const SMARTCTL_PROVIDER_TIMEOUT_MS: ::core::ffi::c_uint = 5000 as ::core::ffi::c_uint;
pub const SMARTCTL_PROVIDER_TERMINATE_GRACE_MS: ::core::ffi::c_uint = 250 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -128 as ::core::ffi::c_int && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SMARTCTL_EXIT_COMMAND_ERROR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SMARTCTL_EXIT_DEVICE_OR_POWER: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SMARTCTL_EXIT_SMART_COMMAND: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SMARTCTL_EXIT_HEALTH_FAILED: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SMARTCTL_EXIT_PREFAIL_ATTRIBUTE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SMARTCTL_SAFE_INTEGER_MAX: ::core::ffi::c_double = 9007199254740991.0f64;
unsafe extern "C" fn default_wake_device_tier() -> bool {
    return system_health_request_tier(system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_DEVICE);
}
unsafe extern "C" fn simple_device_name(mut name: *const ::core::ffi::c_char) -> bool {
    if name.is_null()
        || *name.offset(0isize) == 0
        || strcmp(name, b".\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strcmp(name, b"..\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strlen(name) >= LINUX_HARDWARE_INTERNAL_NAME_LENGTH as size_t
    {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = name as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
            && *cursor as ::core::ffi::c_int != '_' as ::core::ffi::c_int
            && *cursor as ::core::ffi::c_int != '-' as ::core::ffi::c_int
            && *cursor as ::core::ffi::c_int != '.' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn physical_name(
    mut name: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
) {
    let mut length: size_t = strlen(name);
    memcpy(
        output as *mut ::core::ffi::c_void,
        name as *const ::core::ffi::c_void,
        length.wrapping_add(1 as size_t),
    );
    if strncmp(
        output,
        b"nvme\0".as_ptr() as *const ::core::ffi::c_char,
        4 as size_t,
    ) == 0 as ::core::ffi::c_int
        && *(*__ctype_b_loc())
            .offset(*output.offset(4isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        let mut cursor: *mut ::core::ffi::c_char = output.offset(4 as ::core::ffi::c_uint as isize);
        while *(*__ctype_b_loc())
            .offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            cursor = cursor.offset(1);
        }
        if *cursor as ::core::ffi::c_int == 'n' as ::core::ffi::c_int
            && *(*__ctype_b_loc()).offset(
                *cursor.offset(1isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
            ) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            *cursor = '\0' as ::core::ffi::c_char;
        }
        return;
    }
    if strncmp(
        output,
        b"mmcblk\0".as_ptr() as *const ::core::ffi::c_char,
        6 as size_t,
    ) == 0 as ::core::ffi::c_int
        && *(*__ctype_b_loc())
            .offset(*output.offset(6isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        let mut cursor_0: *mut ::core::ffi::c_char =
            output.offset(6 as ::core::ffi::c_uint as isize);
        while *(*__ctype_b_loc())
            .offset(*cursor_0 as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            cursor_0 = cursor_0.offset(1);
        }
        if *cursor_0 as ::core::ffi::c_int == 'p' as ::core::ffi::c_int
            && *(*__ctype_b_loc()).offset(
                *cursor_0.offset(1isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
            ) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            *cursor_0 = '\0' as ::core::ffi::c_char;
        }
        return;
    }
    if *output.offset(0isize) as ::core::ffi::c_int == 's' as ::core::ffi::c_int
        && *output.offset(1isize) as ::core::ffi::c_int == 'd' as ::core::ffi::c_int
        && *(*__ctype_b_loc())
            .offset(*output.offset(2isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalpha.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        let mut cursor_1: *mut ::core::ffi::c_char =
            output.offset(2 as ::core::ffi::c_uint as isize);
        while *(*__ctype_b_loc())
            .offset(*cursor_1 as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalpha.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            cursor_1 = cursor_1.offset(1);
        }
        if *(*__ctype_b_loc())
            .offset(*cursor_1 as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            *cursor_1 = '\0' as ::core::ffi::c_char;
        }
    }
}
unsafe extern "C" fn scoped_hash(
    mut scope: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) -> uint64_t {
    let mut hash: uint64_t = 1469598103934665603 as uint64_t;
    let mut parts: [*const ::core::ffi::c_char; 3] = [
        scope,
        b"|linux-block|\0".as_ptr() as *const ::core::ffi::c_char,
        name,
    ];
    let mut part: size_t = 0 as size_t;
    while part
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        let mut cursor: *const ::core::ffi::c_uchar = parts[part] as *const ::core::ffi::c_uchar;
        while *cursor != 0 {
            hash ^= *cursor as uint64_t;
            hash = (hash as ::core::ffi::c_ulong)
                .wrapping_mul(1099511628211 as ::core::ffi::c_ulong) as uint64_t;
            cursor = cursor.offset(1);
        }
        part = part.wrapping_add(1);
    }
    return hash;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn smartctl_provider_state_init(
    mut state: *mut smartctl_provider_state_t,
    mut installation_scope: *const ::core::ffi::c_char,
) {
    if state.is_null() {
        return;
    }
    memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<smartctl_provider_state_t>(),
    );
    if !installation_scope.is_null() {
        snprintf(
            &raw mut (*state).installation_scope as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            installation_scope,
        );
    }
    snprintf(
        &raw mut (*state).program as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        b"/usr/sbin/smartctl\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*state).timeout_ms = SMARTCTL_PROVIDER_TIMEOUT_MS as uint32_t;
    (*state).terminate_grace_ms = SMARTCTL_PROVIDER_TERMINATE_GRACE_MS as uint32_t;
    (*state).run_helper = Some(
        health_helper_run
            as unsafe extern "C" fn(
                *const health_helper_request_t,
                *mut health_helper_result_t,
            ) -> ::core::ffi::c_int,
    ) as smartctl_helper_run_fn;
    (*state).wake_device_tier = Some(default_wake_device_tier as unsafe extern "C" fn() -> bool)
        as smartctl_wake_device_tier_fn;
    (*state).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    *&raw mut (*state).refresh_pending = r#false != 0;
    *&raw mut (*state).refresh_requests = 0 as uint_fast64_t;
    *&raw mut (*state).refresh_coalesced = 0 as uint_fast64_t;
    *&raw mut (*state).refresh_collections = 0 as uint_fast64_t;
}
unsafe extern "C" fn add_device(
    mut state: *mut smartctl_provider_state_t,
    mut device_path: *const ::core::ffi::c_char,
    mut target_managed: bool,
) -> bool {
    if state.is_null()
        || (*state).installation_scope[0usize] == 0
        || device_path.is_null()
        || strncmp(
            device_path,
            b"/dev/\0".as_ptr() as *const ::core::ffi::c_char,
            5 as size_t,
        ) != 0 as ::core::ffi::c_int
        || strlen(device_path) >= SMARTCTL_PROVIDER_PATH_LENGTH as size_t
    {
        return r#false != 0;
    }
    let mut name: *const ::core::ffi::c_char =
        device_path.offset(5 as ::core::ffi::c_uint as isize);
    if !simple_device_name(name) || !strchr(name, '/' as ::core::ffi::c_int).is_null() {
        return r#false != 0;
    }
    let mut physical: [::core::ffi::c_char; 64] = [0; 64];
    physical_name(name, &raw mut physical as *mut ::core::ffi::c_char);
    let mut index: size_t = 0 as size_t;
    while index < (*state).device_count {
        if strcmp(
            &raw mut (*(&raw mut (*state).devices as *mut smartctl_provider_device_t)
                .offset(index as isize))
            .sysfs_name as *mut ::core::ffi::c_char,
            &raw mut physical as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        index = index.wrapping_add(1);
    }
    if (*state).device_count >= SMARTCTL_PROVIDER_MAX_DEVICES as size_t {
        (*state).devices_dropped = (*state).devices_dropped.wrapping_add(1);
        return r#false != 0;
    }
    let c2rust_fresh0 = (*state).device_count;
    (*state).device_count = (*state).device_count.wrapping_add(1);
    let mut device: *mut smartctl_provider_device_t = (&raw mut (*state).devices
        as *mut smartctl_provider_device_t)
        .offset(c2rust_fresh0 as isize);
    snprintf(
        &raw mut (*device).device_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"/dev/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut physical as *mut ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*device).sysfs_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut physical as *mut ::core::ffi::c_char,
    );
    (*device).target_managed = target_managed;
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn smartctl_provider_add_device(
    mut state: *mut smartctl_provider_state_t,
    mut device_path: *const ::core::ffi::c_char,
) -> bool {
    return add_device(state, device_path, r#false != 0);
}
unsafe extern "C" fn number_u64(mut item: *const cJSON, mut output: *mut uint64_t) -> bool {
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || (*item).valuedouble < 0.0f64
        || (*item).valuedouble > SMARTCTL_SAFE_INTEGER_MAX
        || floor((*item).valuedouble) != (*item).valuedouble
    {
        return r#false != 0;
    }
    *output = (*item).valuedouble as uint64_t;
    return r#true != 0;
}
unsafe extern "C" fn number_int(
    mut item: *const cJSON,
    mut minimum: ::core::ffi::c_int,
    mut maximum: ::core::ffi::c_int,
    mut output: *mut ::core::ffi::c_int,
) -> bool {
    let mut value: uint64_t = 0;
    if !number_u64(item, &raw mut value)
        || value > maximum as uint64_t
        || value < minimum as uint64_t
    {
        return r#false != 0;
    }
    *output = value as ::core::ffi::c_int;
    return r#true != 0;
}
unsafe extern "C" fn object_number_u64(
    mut object: *const cJSON,
    mut name: *const ::core::ffi::c_char,
    mut output: *mut smartctl_u64_value_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(object, name);
    if item.is_null() {
        return r#true != 0;
    }
    let mut value: uint64_t = 0;
    if !number_u64(item, &raw mut value) {
        return r#false != 0;
    }
    (*output).valid = r#true != 0;
    (*output).value = value;
    return r#true != 0;
}
unsafe extern "C" fn object_percent(
    mut root: *const cJSON,
    mut object_name: *const ::core::ffi::c_char,
    mut value_name: *const ::core::ffi::c_char,
    mut output: *mut smartctl_double_value_t,
) -> bool {
    let mut object: *const cJSON = cJSON_GetObjectItemCaseSensitive(root, object_name);
    if object.is_null() {
        return r#true != 0;
    }
    if cJSON_IsObject(object) == 0 {
        return r#false != 0;
    }
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(object, value_name);
    if item.is_null() {
        return r#true != 0;
    }
    let mut percent: ::core::ffi::c_int = 0;
    if !number_int(
        item,
        0 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int,
        &raw mut percent,
    ) {
        return r#false != 0;
    }
    (*output).valid = r#true != 0;
    (*output).value = percent as ::core::ffi::c_double / 100.0f64;
    return r#true != 0;
}
unsafe extern "C" fn node_count_bounded(
    mut node: *const cJSON,
    mut depth: ::core::ffi::c_uint,
    mut count: *mut size_t,
) -> bool {
    if node.is_null() || depth > 32 as ::core::ffi::c_uint || {
        *count = (*count).wrapping_add(1);
        *count > SMARTCTL_PROVIDER_JSON_NODES_MAX as size_t
    } {
        return r#false != 0;
    }
    let mut child: *const cJSON = (*node).child;
    while !child.is_null() {
        if !node_count_bounded(child, depth.wrapping_add(1 as ::core::ffi::c_uint), count) {
            return r#false != 0;
        }
        child = (*child).next;
    }
    return r#true != 0;
}
unsafe extern "C" fn contains_case_insensitive(
    mut text: *const ::core::ffi::c_char,
    mut length: size_t,
    mut needle: *const ::core::ffi::c_char,
) -> bool {
    let mut needle_length: size_t = strlen(needle);
    if needle_length == 0 as size_t || needle_length > length {
        return r#false != 0;
    }
    let mut start: size_t = 0 as size_t;
    while start.wrapping_add(needle_length) <= length {
        let mut index: size_t = 0 as size_t;
        while index < needle_length
            && ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                    if 0 as ::core::ffi::c_int != 0 {
                        let mut __c: ::core::ffi::c_int = *text
                            .offset(start.wrapping_add(index) as isize)
                            as ::core::ffi::c_uchar
                            as ::core::ffi::c_int;
                        __res = (if __c < -128 as ::core::ffi::c_int
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = tolower(*text.offset(start.wrapping_add(index) as isize)
                            as ::core::ffi::c_uchar
                            as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*text.offset(start.wrapping_add(index) as isize)
                            as ::core::ffi::c_uchar
                            as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            }) == ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                    if 0 as ::core::ffi::c_int != 0 {
                        let mut __c: ::core::ffi::c_int = *needle.offset(index as isize)
                            as ::core::ffi::c_uchar
                            as ::core::ffi::c_int;
                        __res = (if __c < -128 as ::core::ffi::c_int
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = tolower(*needle.offset(index as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*needle.offset(index as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            })
        {
            index = index.wrapping_add(1);
        }
        if index == needle_length {
            return r#true != 0;
        }
        start = start.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn json_string_contains(
    mut item: *const cJSON,
    mut needle: *const ::core::ffi::c_char,
) -> bool {
    return cJSON_IsString(item) != 0
        && !(*item).valuestring.is_null()
        && contains_case_insensitive((*item).valuestring, strlen((*item).valuestring), needle)
            as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn classify_device_or_power(mut root: *const cJSON) -> smartctl_parse_status_t {
    let mut power: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"power_mode\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsObject(power) != 0 {
        let mut name: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            power,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if json_string_contains(name, b"standby\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
            || json_string_contains(name, b"sleep\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
            || json_string_contains(name, b"low power\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
            || json_string_contains(name, b"idle\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
        {
            return smartctl_parse_status_t::SMARTCTL_PARSE_SLEEPING;
        }
    }
    let mut smartctl: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"smartctl\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut messages: *const cJSON = if cJSON_IsObject(smartctl) != 0 {
        cJSON_GetObjectItemCaseSensitive(
            smartctl,
            b"messages\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if cJSON_IsArray(messages) != 0 && cJSON_GetArraySize(messages) <= 16 as ::core::ffi::c_int {
        let mut message: *const cJSON = ::core::ptr::null::<cJSON>();
        message = if !messages.is_null() {
            (*messages).child
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
        while !message.is_null() {
            let mut value: *const cJSON = if cJSON_IsObject(message) != 0 {
                cJSON_GetObjectItemCaseSensitive(
                    message,
                    b"string\0".as_ptr() as *const ::core::ffi::c_char,
                )
            } else {
                ::core::ptr::null_mut::<cJSON>()
            };
            if json_string_contains(
                value,
                b"permission denied\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
                || json_string_contains(
                    value,
                    b"operation not permitted\0".as_ptr() as *const ::core::ffi::c_char,
                ) as ::core::ffi::c_int
                    != 0
            {
                return smartctl_parse_status_t::SMARTCTL_PARSE_PERMISSION_DENIED;
            }
            if json_string_contains(value, b"standby\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
                || json_string_contains(value, b"sleeping\0".as_ptr() as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int
                    != 0
                || json_string_contains(
                    value,
                    b"low power\0".as_ptr() as *const ::core::ffi::c_char,
                ) as ::core::ffi::c_int
                    != 0
                || json_string_contains(value, b"idle\0".as_ptr() as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int
                    != 0
            {
                return smartctl_parse_status_t::SMARTCTL_PARSE_SLEEPING;
            }
            message = (*message).next;
        }
    }
    return smartctl_parse_status_t::SMARTCTL_PARSE_MALFORMED;
}
unsafe extern "C" fn parse_version(mut root: *const cJSON, mut supported: *mut bool) -> bool {
    let mut version: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"json_format_version\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsArray(version) == 0 || cJSON_GetArraySize(version) != 2 as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut major: ::core::ffi::c_int = 0;
    let mut minor: ::core::ffi::c_int = 0;
    if !number_int(
        cJSON_GetArrayItem(version, 0 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        255 as ::core::ffi::c_int,
        &raw mut major,
    ) || !number_int(
        cJSON_GetArrayItem(version, 1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        255 as ::core::ffi::c_int,
        &raw mut minor,
    ) {
        return r#false != 0;
    }
    *supported = major == 1 as ::core::ffi::c_int;
    return r#true != 0;
}
unsafe extern "C" fn parse_exit_status(
    mut root: *const cJSON,
    mut process_exit_status: ::core::ffi::c_int,
    mut exit_status: *mut ::core::ffi::c_int,
) -> bool {
    let mut smartctl: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"smartctl\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsObject(smartctl) == 0 {
        return r#false != 0;
    }
    let mut value: ::core::ffi::c_int = 0;
    if !number_int(
        cJSON_GetObjectItemCaseSensitive(
            smartctl,
            b"exit_status\0".as_ptr() as *const ::core::ffi::c_char,
        ),
        0 as ::core::ffi::c_int,
        255 as ::core::ffi::c_int,
        &raw mut value,
    ) || value != process_exit_status
    {
        return r#false != 0;
    }
    *exit_status = value;
    return r#true != 0;
}
unsafe extern "C" fn set_known_attribute(
    mut sample: *mut smartctl_normalized_sample_t,
    mut id: ::core::ffi::c_int,
    mut value: uint64_t,
    mut seen: *mut bool,
) -> bool {
    let mut destination: *mut smartctl_u64_value_t =
        ::core::ptr::null_mut::<smartctl_u64_value_t>();
    let mut slot: size_t = 0 as size_t;
    match id {
        5 => {
            destination = &raw mut (*sample).reallocated_sectors;
            slot = 0 as size_t;
        }
        197 => {
            destination = &raw mut (*sample).pending_sectors;
            slot = 1 as size_t;
        }
        198 => {
            destination = &raw mut (*sample).uncorrectable_errors;
            slot = 2 as size_t;
        }
        199 => {
            destination = &raw mut (*sample).interface_crc_errors;
            slot = 3 as size_t;
        }
        _ => return r#true != 0,
    }
    if *seen.offset(slot as isize) {
        return r#false != 0;
    }
    *seen.offset(slot as isize) = r#true != 0;
    (*destination).valid = r#true != 0;
    (*destination).value = value;
    return r#true != 0;
}
unsafe extern "C" fn parse_ata_attributes(
    mut root: *const cJSON,
    mut sample: *mut smartctl_normalized_sample_t,
) -> bool {
    let mut attributes: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"ata_smart_attributes\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if attributes.is_null() {
        return r#true != 0;
    }
    if cJSON_IsObject(attributes) == 0 {
        return r#false != 0;
    }
    let mut table: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        attributes,
        b"table\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if table.is_null() {
        return r#true != 0;
    }
    if cJSON_IsArray(table) == 0
        || cJSON_GetArraySize(table) < 0 as ::core::ffi::c_int
        || cJSON_GetArraySize(table) as size_t > SMARTCTL_PROVIDER_ATTRIBUTES_MAX as size_t
    {
        return r#false != 0;
    }
    let mut seen: [bool; 4] = [r#false != 0, false, false, false];
    let mut attribute: *const cJSON = ::core::ptr::null::<cJSON>();
    attribute = if !table.is_null() {
        (*table).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !attribute.is_null() {
        if cJSON_IsObject(attribute) == 0 {
            return r#false != 0;
        }
        let mut id: ::core::ffi::c_int = 0;
        if !number_int(
            cJSON_GetObjectItemCaseSensitive(
                attribute,
                b"id\0".as_ptr() as *const ::core::ffi::c_char,
            ),
            1 as ::core::ffi::c_int,
            255 as ::core::ffi::c_int,
            &raw mut id,
        ) {
            return r#false != 0;
        }
        if !(id != 5 as ::core::ffi::c_int
            && id != 197 as ::core::ffi::c_int
            && id != 198 as ::core::ffi::c_int
            && id != 199 as ::core::ffi::c_int)
        {
            let mut raw: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                attribute,
                b"raw\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut value: uint64_t = 0;
            if cJSON_IsObject(raw) == 0
                || !number_u64(
                    cJSON_GetObjectItemCaseSensitive(
                        raw,
                        b"value\0".as_ptr() as *const ::core::ffi::c_char,
                    ),
                    &raw mut value,
                )
                || !set_known_attribute(sample, id, value, &raw mut seen as *mut bool)
            {
                return r#false != 0;
            }
        }
        attribute = (*attribute).next;
    }
    return r#true != 0;
}
unsafe extern "C" fn add_u64(mut total: *mut uint64_t, mut addition: uint64_t) -> bool {
    if (UINT64_MAX as uint64_t).wrapping_sub(*total) < addition {
        return r#false != 0;
    }
    *total = (*total).wrapping_add(addition);
    return r#true != 0;
}
unsafe extern "C" fn parse_scsi(
    mut root: *const cJSON,
    mut sample: *mut smartctl_normalized_sample_t,
) -> bool {
    if !object_number_u64(
        root,
        b"scsi_grown_defect_list\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sample).reallocated_sectors,
    ) {
        return r#false != 0;
    }
    let mut log: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"scsi_error_counter_log\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if log.is_null() {
        return r#true != 0;
    }
    if cJSON_IsObject(log) == 0 {
        return r#false != 0;
    }
    static mut sections: [*const ::core::ffi::c_char; 3] = [
        b"read\0".as_ptr() as *const ::core::ffi::c_char,
        b"write\0".as_ptr() as *const ::core::ffi::c_char,
        b"verify\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut total: uint64_t = 0 as uint64_t;
    let mut found: bool = r#false != 0;
    let mut index: size_t = 0 as size_t;
    while index
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        let mut section: *const cJSON = cJSON_GetObjectItemCaseSensitive(log, sections[index]);
        if !section.is_null() {
            if cJSON_IsObject(section) == 0 {
                return r#false != 0;
            }
            let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                section,
                b"total_uncorrected_errors\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !item.is_null() {
                let mut value: uint64_t = 0;
                if !number_u64(item, &raw mut value) || !add_u64(&raw mut total, value) {
                    return r#false != 0;
                }
                found = r#true != 0;
            }
        }
        index = index.wrapping_add(1);
    }
    if found {
        (*sample).uncorrectable_errors.valid = r#true != 0;
        (*sample).uncorrectable_errors.value = total;
    }
    return r#true != 0;
}
unsafe extern "C" fn parse_nvme(
    mut root: *const cJSON,
    mut sample: *mut smartctl_normalized_sample_t,
) -> bool {
    let mut log: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"nvme_smart_health_information_log\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if log.is_null() {
        return r#true != 0;
    }
    if cJSON_IsObject(log) == 0 {
        return r#false != 0;
    }
    let mut critical_warning: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut critical: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        log,
        b"critical_warning\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !critical.is_null()
        && !number_int(
            critical,
            0 as ::core::ffi::c_int,
            255 as ::core::ffi::c_int,
            &raw mut critical_warning,
        )
    {
        return r#false != 0;
    }
    if critical_warning != 0 as ::core::ffi::c_int {
        (*sample).critical = r#true != 0;
    }
    let mut percent: ::core::ffi::c_int = 0;
    let mut spare: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        log,
        b"available_spare\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !spare.is_null() {
        if !number_int(
            spare,
            0 as ::core::ffi::c_int,
            100 as ::core::ffi::c_int,
            &raw mut percent,
        ) {
            return r#false != 0;
        }
        (*sample).available_spare_ratio.valid = r#true != 0;
        (*sample).available_spare_ratio.value = percent as ::core::ffi::c_double / 100.0f64;
    }
    let mut used: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        log,
        b"percentage_used\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !used.is_null() {
        if !number_int(
            used,
            0 as ::core::ffi::c_int,
            255 as ::core::ffi::c_int,
            &raw mut percent,
        ) {
            return r#false != 0;
        }
        (*sample).percentage_used_ratio.valid = r#true != 0;
        (*sample).percentage_used_ratio.value = if percent > 100 as ::core::ffi::c_int {
            1.0f64
        } else {
            percent as ::core::ffi::c_double / 100.0f64
        };
    }
    if !object_number_u64(
        log,
        b"media_errors\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sample).media_errors,
    ) || !object_number_u64(
        log,
        b"unsafe_shutdowns\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sample).unsafe_shutdowns,
    ) {
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn any_value(mut sample: *const smartctl_normalized_sample_t) -> bool {
    return (*sample).health_valid as ::core::ffi::c_int != 0
        || (*sample).temperature_celsius.valid as ::core::ffi::c_int != 0
        || (*sample).available_spare_ratio.valid as ::core::ffi::c_int != 0
        || (*sample).percentage_used_ratio.valid as ::core::ffi::c_int != 0
        || (*sample).reallocated_sectors.valid as ::core::ffi::c_int != 0
        || (*sample).pending_sectors.valid as ::core::ffi::c_int != 0
        || (*sample).uncorrectable_errors.valid as ::core::ffi::c_int != 0
        || (*sample).interface_crc_errors.valid as ::core::ffi::c_int != 0
        || (*sample).media_errors.valid as ::core::ffi::c_int != 0
        || (*sample).unsafe_shutdowns.valid as ::core::ffi::c_int != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn smartctl_provider_parse_json(
    mut json: *const ::core::ffi::c_char,
    mut length: size_t,
    mut process_exit_status: ::core::ffi::c_int,
    mut sample: *mut smartctl_normalized_sample_t,
) -> ::core::ffi::c_int {
    if json.is_null()
        || sample.is_null()
        || length == 0 as size_t
        || length > HEALTH_HELPER_OUTPUT_MAX as size_t
        || process_exit_status < 0 as ::core::ffi::c_int
        || process_exit_status > 255 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        sample as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<smartctl_normalized_sample_t>(),
    );
    (*sample).status = smartctl_parse_status_t::SMARTCTL_PARSE_MALFORMED;
    (*sample).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    let mut parse_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut root: *mut cJSON =
        cJSON_ParseWithLengthOpts(json, length, &raw mut parse_end, 0 as cJSON_bool);
    if root.is_null() || cJSON_IsObject(root) == 0 {
        cJSON_Delete(root);
        return -1 as ::core::ffi::c_int;
    }
    while parse_end < json.offset(length as isize)
        && *(*__ctype_b_loc())
            .offset(*parse_end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        parse_end = parse_end.offset(1);
    }
    let mut nodes: size_t = 0 as size_t;
    if parse_end != json.offset(length as isize)
        || !node_count_bounded(root, 0 as ::core::ffi::c_uint, &raw mut nodes)
    {
        cJSON_Delete(root);
        return -1 as ::core::ffi::c_int;
    }
    let mut supported: bool = r#false != 0;
    if !parse_version(root, &raw mut supported) {
        cJSON_Delete(root);
        return -1 as ::core::ffi::c_int;
    }
    if !supported {
        (*sample).status = smartctl_parse_status_t::SMARTCTL_PARSE_UNSUPPORTED_VERSION;
        (*sample).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        cJSON_Delete(root);
        return 0 as ::core::ffi::c_int;
    }
    if !parse_exit_status(root, process_exit_status, &raw mut (*sample).exit_status) {
        cJSON_Delete(root);
        return -1 as ::core::ffi::c_int;
    }
    if (*sample).exit_status & SMARTCTL_EXIT_DEVICE_OR_POWER != 0 as ::core::ffi::c_int {
        (*sample).status = classify_device_or_power(root);
        if (*sample).status.0 == smartctl_parse_status_t::SMARTCTL_PARSE_PERMISSION_DENIED.0 {
            (*sample).capability =
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
        } else if (*sample).status.0 == smartctl_parse_status_t::SMARTCTL_PARSE_SLEEPING.0 {
            (*sample).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE;
        } else {
            (*sample).status = smartctl_parse_status_t::SMARTCTL_PARSE_MALFORMED;
            (*sample).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        }
        cJSON_Delete(root);
        return 0 as ::core::ffi::c_int;
    }
    if (*sample).exit_status & (SMARTCTL_EXIT_COMMAND_ERROR | SMARTCTL_EXIT_SMART_COMMAND)
        != 0 as ::core::ffi::c_int
    {
        cJSON_Delete(root);
        return 0 as ::core::ffi::c_int;
    }
    let mut status: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"smart_status\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !status.is_null() {
        if cJSON_IsObject(status) == 0 {
            cJSON_Delete(root);
            return -1 as ::core::ffi::c_int;
        }
        let mut passed: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            status,
            b"passed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsBool(passed) == 0 {
            cJSON_Delete(root);
            return -1 as ::core::ffi::c_int;
        }
        (*sample).health_valid = r#true != 0;
        (*sample).health_passed = cJSON_IsTrue(passed) != 0;
    }
    let mut temperature: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"temperature\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !temperature.is_null() {
        let mut current: *const cJSON = if cJSON_IsObject(temperature) != 0 {
            cJSON_GetObjectItemCaseSensitive(
                temperature,
                b"current\0".as_ptr() as *const ::core::ffi::c_char,
            )
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
        if cJSON_IsNumber(current) == 0
            || (*current).valuedouble.is_finite() as ::core::ffi::c_int == 0
            || (*current).valuedouble < -100.0f64
            || (*current).valuedouble > 250.0f64
        {
            cJSON_Delete(root);
            return -1 as ::core::ffi::c_int;
        }
        (*sample).temperature_celsius.valid = r#true != 0;
        (*sample).temperature_celsius.value = (*current).valuedouble;
    }
    if !object_percent(
        root,
        b"spare_available\0".as_ptr() as *const ::core::ffi::c_char,
        b"current_percent\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sample).available_spare_ratio,
    ) || !object_percent(
        root,
        b"endurance_used\0".as_ptr() as *const ::core::ffi::c_char,
        b"current_percent\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sample).percentage_used_ratio,
    ) || !parse_ata_attributes(root, sample)
        || !parse_scsi(root, sample)
        || !parse_nvme(root, sample)
    {
        cJSON_Delete(root);
        return -1 as ::core::ffi::c_int;
    }
    if !(*sample).media_errors.valid
        && (*sample).uncorrectable_errors.valid as ::core::ffi::c_int != 0
    {
        (*sample).media_errors = (*sample).uncorrectable_errors;
    }
    if (*sample).exit_status & SMARTCTL_EXIT_HEALTH_FAILED != 0 as ::core::ffi::c_int
        || (*sample).health_valid as ::core::ffi::c_int != 0 && !(*sample).health_passed
    {
        (*sample).critical = r#true != 0;
    }
    (*sample).prefail = (*sample).critical as ::core::ffi::c_int != 0
        || (*sample).exit_status & SMARTCTL_EXIT_PREFAIL_ATTRIBUTE != 0 as ::core::ffi::c_int
        || (*sample).pending_sectors.valid as ::core::ffi::c_int != 0
            && (*sample).pending_sectors.value > 0 as uint64_t
        || (*sample).reallocated_sectors.valid as ::core::ffi::c_int != 0
            && (*sample).reallocated_sectors.value > 0 as uint64_t;
    if !(*sample).health_valid
        && (*sample).exit_status & (SMARTCTL_EXIT_HEALTH_FAILED | SMARTCTL_EXIT_PREFAIL_ATTRIBUTE)
            != 0 as ::core::ffi::c_int
    {
        (*sample).health_valid = r#true != 0;
    }
    (*sample).status = smartctl_parse_status_t::SMARTCTL_PARSE_OK;
    (*sample).capability = system_health_capability_t(
        (if any_value(sample) as ::core::ffi::c_int != 0 {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 as ::core::ffi::c_int
        } else {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    cJSON_Delete(root);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn smartctl_provider_request_refresh(
    mut state: *mut smartctl_provider_state_t,
) -> bool {
    if state.is_null() || (*state).wake_device_tier.is_none() {
        return r#false != 0;
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
        &raw mut (*state).refresh_requests,
        1 as uint_fast64_t,
    );
    let mut expected: bool = r#false != 0;
    let c2rust_result = match ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*state).refresh_pending).compare_exchange(*&raw mut expected, r#true != 0, ::core::sync::atomic::Ordering::AcqRel, ::core::sync::atomic::Ordering::Relaxed) { Ok(old) => (old, true), Err(old) => (old, false) };
    *&raw mut expected = c2rust_result.0;
    if !c2rust_result.1 {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
            &raw mut (*state).refresh_coalesced,
            1 as uint_fast64_t,
        );
        return r#true != 0;
    }
    if !(*state)
        .wake_device_tier
        .expect("non-null function pointer")()
    {
        ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*state).refresh_pending).store(r#false != 0, ::core::sync::atomic::Ordering::Release);
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn find_device(
    mut devices: *const smartctl_provider_device_t,
    mut count: size_t,
    mut sysfs_name: *const ::core::ffi::c_char,
) -> *const smartctl_provider_device_t {
    let mut index: size_t = 0 as size_t;
    while index < count {
        if strcmp(
            &raw const (*devices.offset(index as isize)).sysfs_name as *const ::core::ffi::c_char,
            sysfs_name,
        ) == 0 as ::core::ffi::c_int
        {
            return devices.offset(index as isize);
        }
        index = index.wrapping_add(1);
    }
    return ::core::ptr::null::<smartctl_provider_device_t>();
}
unsafe extern "C" fn restore_baselines(
    mut destination: *mut smartctl_provider_device_t,
    mut source: *const smartctl_provider_device_t,
) {
    if destination.is_null() || source.is_null() {
        return;
    }
    (*destination).previous_uncorrectable = (*source).previous_uncorrectable;
    (*destination).previous_crc = (*source).previous_crc;
    (*destination).previous_media = (*source).previous_media;
    (*destination).previous_unsafe_shutdowns = (*source).previous_unsafe_shutdowns;
    (*destination).previous_monotonic_ms = (*source).previous_monotonic_ms;
    (*destination).previous_uncorrectable_valid = (*source).previous_uncorrectable_valid;
    (*destination).previous_crc_valid = (*source).previous_crc_valid;
    (*destination).previous_media_valid = (*source).previous_media_valid;
    (*destination).previous_unsafe_valid = (*source).previous_unsafe_valid;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn smartctl_provider_discover(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut inventory: *mut system_health_provider_inventory_t,
) -> ::core::ffi::c_int {
    let mut state: *mut smartctl_provider_state_t = opaque as *mut smartctl_provider_state_t;
    if state.is_null() || context.is_null() || (*context).sys_root.is_null() || inventory.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        inventory as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_provider_inventory_t>(),
    );
    let mut map: linux_device_map_t = linux_device_map_t {
        entries: [linux_device_map_entry_t {
            sysfs_name: [0; 64],
            public_id: [0; 64],
            filesystem_device: 0,
            target_mapped: false,
            target_is_default: false,
        }; 16],
        count: 0,
        dropped: 0,
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
    };
    memset(
        &raw mut map as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_device_map_t>(),
    );
    if (*state).installation_scope[0usize] == 0
        || (*state).program[0usize] as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        || (*state).timeout_ms == 0 as uint32_t
        || (*state).run_helper.is_none()
    {
        (*state).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    } else {
        linux_device_map_build(
            (*context).sys_root,
            &raw mut (*state).installation_scope as *mut ::core::ffi::c_char,
            &raw mut map,
        );
        if (*state).discover_target_devices {
            let mut previous: [smartctl_provider_device_t; 16] = [smartctl_provider_device_t {
                device_path: [0; 256],
                sysfs_name: [0; 64],
                public_id: [0; 64],
                previous_uncorrectable: 0,
                previous_crc: 0,
                previous_media: 0,
                previous_unsafe_shutdowns: 0,
                previous_monotonic_ms: 0,
                previous_uncorrectable_valid: false,
                previous_crc_valid: false,
                previous_media_valid: false,
                previous_unsafe_valid: false,
                target_managed: false,
            }; 16];
            let mut previous_count: size_t = (*state).device_count;
            memcpy(
                &raw mut previous as *mut smartctl_provider_device_t as *mut ::core::ffi::c_void,
                &raw mut (*state).devices as *mut smartctl_provider_device_t
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[smartctl_provider_device_t; 16]>(),
            );
            let mut kept: size_t = 0 as size_t;
            let mut index: size_t = 0 as size_t;
            while index < previous_count {
                if !previous[index].target_managed {
                    let c2rust_fresh1 = kept;
                    kept = kept.wrapping_add(1);
                    (*state).devices[c2rust_fresh1] = previous[index];
                }
                index = index.wrapping_add(1);
            }
            memset(
                (&raw mut (*state).devices as *mut smartctl_provider_device_t).offset(kept as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<smartctl_provider_device_t>()
                    .wrapping_mul((SMARTCTL_PROVIDER_MAX_DEVICES as size_t).wrapping_sub(kept)),
            );
            (*state).device_count = kept;
            (*state).devices_dropped = 0 as size_t;
            if map.capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
            {
                let mut index_0: size_t = 0 as size_t;
                while index_0 < map.count {
                    let mut mapped: *const linux_device_map_entry_t = (&raw mut map.entries
                        as *mut linux_device_map_entry_t)
                        .offset(index_0 as isize);
                    if (*mapped).target_mapped {
                        let mut path: [::core::ffi::c_char; 256] = [0; 256];
                        let mut written: ::core::ffi::c_int = snprintf(
                            &raw mut path as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                            b"/dev/%s\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw const (*mapped).sysfs_name as *const ::core::ffi::c_char,
                        );
                        let mut before: size_t = (*state).device_count;
                        if !(written < 0 as ::core::ffi::c_int
                            || written as size_t
                                >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
                            || !add_device(
                                state,
                                &raw mut path as *mut ::core::ffi::c_char,
                                r#true != 0,
                            ))
                        {
                            if (*state).device_count != before {
                                let mut added: *mut smartctl_provider_device_t =
                                    (&raw mut (*state).devices as *mut smartctl_provider_device_t)
                                        .offset((*state).device_count.wrapping_sub(1 as size_t)
                                            as isize);
                                restore_baselines(
                                    added,
                                    find_device(
                                        &raw mut previous as *mut smartctl_provider_device_t,
                                        previous_count,
                                        &raw mut (*added).sysfs_name as *mut ::core::ffi::c_char,
                                    ),
                                );
                            }
                        }
                    }
                    index_0 = index_0.wrapping_add(1);
                }
            }
        }
        let mut index_1: size_t = 0 as size_t;
        while index_1 < (*state).device_count {
            let mut device: *mut smartctl_provider_device_t = (&raw mut (*state).devices
                as *mut smartctl_provider_device_t)
                .offset(index_1 as isize);
            let mut mapped_0: *const linux_device_map_entry_t = if map.capability.0
                == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
            {
                linux_device_map_find(
                    &raw mut map,
                    &raw mut (*device).sysfs_name as *mut ::core::ffi::c_char,
                )
            } else {
                ::core::ptr::null::<linux_device_map_entry_t>()
            };
            if !mapped_0.is_null() {
                snprintf(
                    &raw mut (*device).public_id as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*mapped_0).public_id as *const ::core::ffi::c_char,
                );
            } else {
                snprintf(
                    &raw mut (*device).public_id as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    b"device.%016llx\0".as_ptr() as *const ::core::ffi::c_char,
                    scoped_hash(
                        &raw mut (*state).installation_scope as *mut ::core::ffi::c_char,
                        &raw mut (*device).sysfs_name as *mut ::core::ffi::c_char,
                    ) as ::core::ffi::c_ulonglong,
                );
            }
            index_1 = index_1.wrapping_add(1);
        }
        if (*state).device_count > 0 as size_t {
            (*state).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        } else {
            (*state).capability = system_health_capability_t(
                if (*state).discover_target_devices as ::core::ffi::c_int != 0 {
                    if map.capability.0
                        == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
                    {
                        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                    } else {
                        map.capability.0
                    }
                } else {
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                },
            );
        }
    }
    let mut index_2: size_t = 0 as size_t;
    while index_2 < (*state).device_count {
        snprintf(
            &raw mut (*(&raw mut (*inventory).resources as *mut system_health_provider_resource_t)
                .offset(index_2 as isize))
            .id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut (*state).devices as *mut smartctl_provider_device_t)
                .offset(index_2 as isize))
            .public_id as *mut ::core::ffi::c_char,
        );
        (*inventory).resources[index_2].scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_DEVICE;
        (*inventory).resources[index_2].capability = (*state).capability;
        index_2 = index_2.wrapping_add(1);
    }
    (*inventory).count = (*state).device_count;
    (*inventory).dropped = (*state).devices_dropped;
    if (*inventory).count == 0 as size_t {
        snprintf(
            &raw mut (*(&raw mut (*inventory).resources as *mut system_health_provider_resource_t)
                .offset(0isize))
            .id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            b"smartctl\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*inventory).resources[0usize].scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_DEVICE;
        (*inventory).resources[0usize].capability = (*state).capability;
        (*inventory).count = 1 as size_t;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn emit(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut metric: *const ::core::ffi::c_char,
    mut resource: *const ::core::ffi::c_char,
    mut capability: system_health_capability_t,
    mut valid: bool,
    mut value: ::core::ffi::c_double,
    mut unit: system_health_unit_t,
) {
    let mut observation: system_health_observation_t = system_health_observation_t {
        metric: [0; 64],
        resource_id: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        freshness: system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_UNKNOWN,
        unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
        value_valid: false,
        value: 0.,
        sampled_monotonic_ms: 0,
        observed_wall_time_ms: 0,
    };
    memset(
        &raw mut observation as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_observation_t>(),
    );
    snprintf(
        &raw mut observation.metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        metric,
    );
    snprintf(
        &raw mut observation.resource_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        resource,
    );
    observation.scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_DEVICE;
    observation.sampled_monotonic_ms = (*context).monotonic_ms;
    observation.observed_wall_time_ms = (*context).wall_time_ms;
    if valid {
        system_health_observation_set_available(&raw mut observation, value, unit);
    } else {
        system_health_observation_set_unavailable(&raw mut observation, capability);
    }
    system_health_observation_sink_append(sink, &raw mut observation);
}
unsafe extern "C" fn reset_baselines(mut device: *mut smartctl_provider_device_t) {
    (*device).previous_uncorrectable_valid = r#false != 0;
    (*device).previous_crc_valid = r#false != 0;
    (*device).previous_media_valid = r#false != 0;
    (*device).previous_unsafe_valid = r#false != 0;
}
unsafe extern "C" fn emit_delta(
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
    mut metric: *const ::core::ffi::c_char,
    mut resource: *const ::core::ffi::c_char,
    mut current: smartctl_u64_value_t,
    mut previous: *mut uint64_t,
    mut previous_valid: *mut bool,
    mut previous_monotonic_ms: uint64_t,
    mut unavailable: system_health_capability_t,
) {
    if !current.valid {
        *previous_valid = r#false != 0;
        emit(
            sink,
            context,
            metric,
            resource,
            unavailable,
            r#false != 0,
            0.0f64,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
        return;
    }
    let mut valid: bool = *previous_valid as ::core::ffi::c_int != 0
        && (*context).monotonic_ms > previous_monotonic_ms
        && current.value >= *previous;
    let mut capability: system_health_capability_t = system_health_capability_t(
        (if !*previous_valid {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE.0 as ::core::ffi::c_int
        } else {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    let mut delta: ::core::ffi::c_double = if valid as ::core::ffi::c_int != 0 {
        current.value.wrapping_sub(*previous) as ::core::ffi::c_double
    } else {
        0.0f64
    };
    *previous = current.value;
    *previous_valid = r#true != 0;
    emit(
        sink,
        context,
        metric,
        resource,
        capability,
        valid,
        delta,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
}
unsafe extern "C" fn emit_unavailable_device(
    mut device: *const smartctl_provider_device_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
    mut capability: system_health_capability_t,
) {
    static mut metrics: [*const ::core::ffi::c_char; 11] = [
        b"storage.device.prefail\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.critical\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.temperature_celsius\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.available_spare_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.percentage_used_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.reallocated_sectors\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.pending_sectors\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.uncorrectable_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.interface_crc_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.media_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.unsafe_shutdowns_delta\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut index: size_t = 0 as size_t;
    while index
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 11]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        emit(
            sink,
            context,
            metrics[index],
            &raw const (*device).public_id as *const ::core::ffi::c_char,
            capability,
            r#false != 0,
            0.0f64,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
        );
        index = index.wrapping_add(1);
    }
}
unsafe extern "C" fn helper_failure_capability(
    mut result: *const health_helper_result_t,
) -> system_health_capability_t {
    if result.is_null() {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    match (*result).outcome {
        health_helper_outcome_t::HEALTH_HELPER_EXEC_ERROR => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        }
        health_helper_outcome_t::HEALTH_HELPER_BUSY => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE;
        }
        health_helper_outcome_t::HEALTH_HELPER_OK
        | health_helper_outcome_t::HEALTH_HELPER_EXITED => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        }
        health_helper_outcome_t::HEALTH_HELPER_TIMED_OUT
        | health_helper_outcome_t::HEALTH_HELPER_SYSTEM_ERROR => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        }
        _ => {}
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
}
unsafe extern "C" fn collect_device(
    mut state: *mut smartctl_provider_state_t,
    mut device: *mut smartctl_provider_device_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> system_health_capability_t {
    let mut arguments: [*mut ::core::ffi::c_char; 9] = [
        &raw mut (*state).program as *mut ::core::ffi::c_char,
        b"-H\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"-A\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"-j=c\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"-n\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"standby,2\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"--\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut (*device).device_path as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ];
    let mut request: health_helper_request_t = health_helper_request_t {
        program: &raw mut (*state).program as *mut ::core::ffi::c_char,
        argv: &raw mut arguments as *mut *mut ::core::ffi::c_char,
        timeout_ms: (*state).timeout_ms,
        terminate_grace_ms: (*state).terminate_grace_ms,
        output_limit: HEALTH_HELPER_OUTPUT_MAX as size_t,
    };
    let mut result: health_helper_result_t = health_helper_result_t {
        outcome: health_helper_outcome_t::HEALTH_HELPER_OK,
        exit_code: 0,
        term_signal: 0,
        latency_ms: 0,
        output_truncated: false,
        abandoned: false,
        output_length: 0,
        output: [0; 4097],
    };
    memset(
        &raw mut result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<health_helper_result_t>(),
    );
    let mut run_status: ::core::ffi::c_int =
        (*state).run_helper.expect("non-null function pointer")(&raw mut request, &raw mut result);
    if run_status != 0 as ::core::ffi::c_int
        || result.outcome.0 != health_helper_outcome_t::HEALTH_HELPER_OK.0
            && result.outcome.0 != health_helper_outcome_t::HEALTH_HELPER_EXITED.0
        || result.output_truncated as ::core::ffi::c_int != 0
    {
        let mut capability: system_health_capability_t =
            system_health_capability_t(if run_status != 0 as ::core::ffi::c_int {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
            } else if result.output_truncated as ::core::ffi::c_int != 0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
            } else {
                helper_failure_capability(&raw mut result).0
            });
        emit_unavailable_device(device, context, sink, capability);
        if capability.0 != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE.0 {
            reset_baselines(device);
        }
        return capability;
    }
    let mut sample: smartctl_normalized_sample_t = smartctl_normalized_sample_t {
        status: smartctl_parse_status_t::SMARTCTL_PARSE_OK,
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        exit_status: 0,
        health_valid: false,
        health_passed: false,
        prefail: false,
        critical: false,
        temperature_celsius: smartctl_double_value_t {
            valid: false,
            value: 0.,
        },
        available_spare_ratio: smartctl_double_value_t {
            valid: false,
            value: 0.,
        },
        percentage_used_ratio: smartctl_double_value_t {
            valid: false,
            value: 0.,
        },
        reallocated_sectors: smartctl_u64_value_t {
            valid: false,
            value: 0,
        },
        pending_sectors: smartctl_u64_value_t {
            valid: false,
            value: 0,
        },
        uncorrectable_errors: smartctl_u64_value_t {
            valid: false,
            value: 0,
        },
        interface_crc_errors: smartctl_u64_value_t {
            valid: false,
            value: 0,
        },
        media_errors: smartctl_u64_value_t {
            valid: false,
            value: 0,
        },
        unsafe_shutdowns: smartctl_u64_value_t {
            valid: false,
            value: 0,
        },
    };
    if smartctl_provider_parse_json(
        &raw mut result.output as *mut ::core::ffi::c_char,
        result.output_length,
        result.exit_code,
        &raw mut sample,
    ) != 0 as ::core::ffi::c_int
    {
        emit_unavailable_device(
            device,
            context,
            sink,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
        reset_baselines(device);
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    if sample.capability.0 != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 {
        emit_unavailable_device(device, context, sink, sample.capability);
        if sample.capability.0 != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE.0 {
            reset_baselines(device);
        }
        return sample.capability;
    }
    emit(
        sink,
        context,
        b"storage.device.prefail\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        system_health_capability_t(
            (if sample.health_valid as ::core::ffi::c_int != 0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
                    as ::core::ffi::c_int
            } else {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                    as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ),
        sample.health_valid,
        if sample.prefail as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    emit(
        sink,
        context,
        b"storage.device.critical\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        system_health_capability_t(
            (if sample.health_valid as ::core::ffi::c_int != 0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
                    as ::core::ffi::c_int
            } else {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                    as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ),
        sample.health_valid,
        if sample.critical as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    emit(
        sink,
        context,
        b"storage.device.temperature_celsius\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        sample.temperature_celsius.valid,
        sample.temperature_celsius.value,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_CELSIUS,
    );
    emit(
        sink,
        context,
        b"storage.device.available_spare_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        sample.available_spare_ratio.valid,
        sample.available_spare_ratio.value,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
    );
    emit(
        sink,
        context,
        b"storage.device.percentage_used_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        sample.percentage_used_ratio.valid,
        sample.percentage_used_ratio.value,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
    );
    emit(
        sink,
        context,
        b"storage.device.reallocated_sectors\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        sample.reallocated_sectors.valid,
        sample.reallocated_sectors.value as ::core::ffi::c_double,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
    emit(
        sink,
        context,
        b"storage.device.pending_sectors\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        sample.pending_sectors.valid,
        sample.pending_sectors.value as ::core::ffi::c_double,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
    emit_delta(
        context,
        sink,
        b"storage.device.uncorrectable_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        sample.uncorrectable_errors,
        &raw mut (*device).previous_uncorrectable,
        &raw mut (*device).previous_uncorrectable_valid,
        (*device).previous_monotonic_ms,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
    );
    emit_delta(
        context,
        sink,
        b"storage.device.interface_crc_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        sample.interface_crc_errors,
        &raw mut (*device).previous_crc,
        &raw mut (*device).previous_crc_valid,
        (*device).previous_monotonic_ms,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
    );
    emit_delta(
        context,
        sink,
        b"storage.device.media_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        sample.media_errors,
        &raw mut (*device).previous_media,
        &raw mut (*device).previous_media_valid,
        (*device).previous_monotonic_ms,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
    );
    emit_delta(
        context,
        sink,
        b"storage.device.unsafe_shutdowns_delta\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        sample.unsafe_shutdowns,
        &raw mut (*device).previous_unsafe_shutdowns,
        &raw mut (*device).previous_unsafe_valid,
        (*device).previous_monotonic_ms,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
    );
    (*device).previous_monotonic_ms = (*context).monotonic_ms;
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
}
unsafe extern "C" fn merge_provider_capability(
    mut current: system_health_capability_t,
    mut next: system_health_capability_t,
) -> system_health_capability_t {
    if current.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        || next.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    if current.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED.0
        || next.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED.0
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if current.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
        || next.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    if current.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE.0
        || next.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE.0
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE;
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn smartctl_provider_collect(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    let mut state: *mut smartctl_provider_state_t = opaque as *mut smartctl_provider_state_t;
    if state.is_null() || context.is_null() || sink.is_null() || (*state).run_helper.is_none() {
        return -1 as ::core::ffi::c_int;
    }
    let mut refresh: bool = ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*state).refresh_pending).swap(r#false != 0, ::core::sync::atomic::Ordering::AcqRel);
    if refresh {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::Relaxed }>(
            &raw mut (*state).refresh_collections,
            1 as uint_fast64_t,
        );
    }
    let mut capability: system_health_capability_t =
        system_health_capability_t(if (*state).device_count == 0 as size_t {
            (*state).capability.0
        } else {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
        });
    let mut index: size_t = 0 as size_t;
    while index < (*state).device_count {
        capability = merge_provider_capability(
            capability,
            collect_device(
                state,
                (&raw mut (*state).devices as *mut smartctl_provider_device_t)
                    .offset(index as isize),
                context,
                sink,
            ),
        );
        index = index.wrapping_add(1);
    }
    (*state).capability = capability;
    emit(
        sink,
        context,
        b"hardware.provider.visible\0".as_ptr() as *const ::core::ffi::c_char,
        b"smartctl\0".as_ptr() as *const ::core::ffi::c_char,
        capability,
        capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0,
        1.0f64,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    (*sink).dropped = (*sink).dropped.wrapping_add((*state).devices_dropped);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn smartctl_provider_init(
    mut provider: *mut system_health_provider_t,
    mut state: *mut smartctl_provider_state_t,
) {
    if provider.is_null() {
        return;
    }
    memset(
        provider as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_provider_t>(),
    );
    snprintf(
        &raw mut (*provider).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        b"smartctl\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*provider).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    (*provider).state = state as *mut ::core::ffi::c_void;
    (*provider).discover = Some(
        smartctl_provider_discover
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_provider_inventory_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_provider_discover_fn;
    (*provider).collect = Some(
        smartctl_provider_collect
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_observation_sink_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_collect_fn;
}
