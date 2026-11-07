use ::c2rust_bitfields;
use ::f128;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
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
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn __ctype_tolower_loc() -> *mut *const __int32_t;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fgetc(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type __uint64_t = u64;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int64_t = i64;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_counter_t {
    pub delta_valid: bool,
    pub reset_detected: bool,
    pub current: uint64_t,
    pub previous: uint64_t,
    pub delta: uint64_t,
    pub interval_ms: uint64_t,
    pub sampled_monotonic_ms: uint64_t,
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
pub struct system_health_collector_t {
    pub name: [::core::ffi::c_char; 32],
    pub scope: system_health_scope_t,
    pub tier: system_health_sampling_tier_t,
    pub interval_seconds: uint32_t,
    pub stale_after_seconds: uint32_t,
    pub state: *mut ::core::ffi::c_void,
    pub collect: system_health_collect_fn,
    pub destroy: system_health_collector_destroy_fn,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct linux_network_counter_kind_t(pub ::core::ffi::c_uint);
impl linux_network_counter_kind_t {
    pub const LINUX_NETWORK_RX_BYTES: Self = Self(0);
    pub const LINUX_NETWORK_RX_PACKETS: Self = Self(1);
    pub const LINUX_NETWORK_RX_ERRORS: Self = Self(2);
    pub const LINUX_NETWORK_RX_DROPS: Self = Self(3);
    pub const LINUX_NETWORK_TX_BYTES: Self = Self(4);
    pub const LINUX_NETWORK_TX_PACKETS: Self = Self(5);
    pub const LINUX_NETWORK_TX_ERRORS: Self = Self(6);
    pub const LINUX_NETWORK_TX_DROPS: Self = Self(7);
    pub const LINUX_NETWORK_COUNTER_COUNT: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_network_interface_state_t {
    pub internal_name: [::core::ffi::c_char; 64],
    pub id: [::core::ffi::c_char; 64],
    pub previous: [uint64_t; 8],
    pub exported_total: [uint64_t; 8],
    pub previous_valid: [bool; 8],
    pub previous_monotonic_ms: uint64_t,
    pub carrier_valid: bool,
    pub carrier: bool,
    pub carrier_flaps: uint64_t,
    pub present: bool,
    pub seen: bool,
    pub missing_cycles: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_network_state_t {
    pub interfaces: [linux_network_interface_state_t; 32],
    pub interface_count: size_t,
    pub scope: system_health_scope_t,
    pub primary_override: [::core::ffi::c_char; 64],
    pub resources_dropped_total: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_network_interface_sample_t {
    pub id: [::core::ffi::c_char; 64],
    pub primary: bool,
    pub carrier_capability: system_health_capability_t,
    pub carrier_valid: bool,
    pub carrier: bool,
    pub carrier_flaps: uint64_t,
    pub carrier_flap_delta: uint64_t,
    pub counters: [system_health_counter_t; 8],
    pub error_drop_ratio_valid: bool,
    pub error_drop_ratio: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_network_result_t {
    pub interfaces: [linux_network_interface_sample_t; 16],
    pub interface_count: size_t,
    pub resources_dropped: size_t,
    pub capability: system_health_capability_t,
    pub primary_capability: system_health_capability_t,
    pub primary_id: [::core::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct network_integer_t {
    pub valid: bool,
    pub capability: system_health_capability_t,
    pub value: uint64_t,
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
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
pub const UINT8_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const SYSTEM_HEALTH_MAX_INTERFACES: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_ID_LENGTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const LINUX_NETWORK_INTERNAL_NAME_LENGTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const LINUX_NETWORK_STATE_CAPACITY: ::core::ffi::c_uint =
    SYSTEM_HEALTH_MAX_INTERFACES.wrapping_mul(2 as ::core::ffi::c_uint);
#[inline]
unsafe extern "C" fn tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -128 as ::core::ffi::c_int && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const NETWORK_DIRECTORY_CAPACITY: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
pub const NETWORK_TEXT_LENGTH: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
static mut counter_files: [*const ::core::ffi::c_char; 8] = [
    b"rx_bytes\0".as_ptr() as *const ::core::ffi::c_char,
    b"rx_packets\0".as_ptr() as *const ::core::ffi::c_char,
    b"rx_errors\0".as_ptr() as *const ::core::ffi::c_char,
    b"rx_dropped\0".as_ptr() as *const ::core::ffi::c_char,
    b"tx_bytes\0".as_ptr() as *const ::core::ffi::c_char,
    b"tx_packets\0".as_ptr() as *const ::core::ffi::c_char,
    b"tx_errors\0".as_ptr() as *const ::core::ffi::c_char,
    b"tx_dropped\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut counter_metrics: [*const ::core::ffi::c_char; 8] = [
    b"network.rx_bytes_total\0".as_ptr() as *const ::core::ffi::c_char,
    b"network.rx_packets_total\0".as_ptr() as *const ::core::ffi::c_char,
    b"network.rx_errors_total\0".as_ptr() as *const ::core::ffi::c_char,
    b"network.rx_drops_total\0".as_ptr() as *const ::core::ffi::c_char,
    b"network.tx_bytes_total\0".as_ptr() as *const ::core::ffi::c_char,
    b"network.tx_packets_total\0".as_ptr() as *const ::core::ffi::c_char,
    b"network.tx_errors_total\0".as_ptr() as *const ::core::ffi::c_char,
    b"network.tx_drops_total\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut counter_units: [system_health_unit_t; 8] = [
    system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
    system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
    system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_network_capability_from_errno(
    mut error_number: ::core::ffi::c_int,
) -> system_health_capability_t {
    if error_number == EACCES || error_number == EPERM {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if error_number == ENOENT || error_number == ENOTDIR {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
}
unsafe extern "C" fn valid_interface_name(mut name: *const ::core::ffi::c_char) -> bool {
    if name.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strlen(name);
    if length == 0 as size_t
        || length >= LINUX_NETWORK_INTERNAL_NAME_LENGTH as size_t
        || strcmp(name, b".\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || !strstr(name, b"..\0".as_ptr() as *const ::core::ffi::c_char).is_null()
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
unsafe extern "C" fn fnv1a(mut text: *const ::core::ffi::c_char) -> uint32_t {
    let mut hash: uint32_t = 2166136261 as uint32_t;
    if text.is_null() {
        return hash;
    }
    let mut cursor: *const ::core::ffi::c_uchar = text as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        hash ^= *cursor as uint32_t;
        hash =
            (hash as ::core::ffi::c_uint).wrapping_mul(16777619 as ::core::ffi::c_uint) as uint32_t;
        cursor = cursor.offset(1);
    }
    return hash;
}
unsafe extern "C" fn make_public_id(
    mut internal_name: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
) {
    let mut normalized: [::core::ffi::c_char; 36] = [0; 36];
    let mut used: size_t = 0 as size_t;
    let mut separator: bool = r#false != 0;
    let mut cursor: *const ::core::ffi::c_uchar = internal_name as *const ::core::ffi::c_uchar;
    while *cursor as ::core::ffi::c_int != 0
        && used.wrapping_add(1 as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 36]>()
    {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            let c2rust_fresh1 = used;
            used = used.wrapping_add(1);
            normalized[c2rust_fresh1] = ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                    if 0 as ::core::ffi::c_int != 0 {
                        let mut __c: ::core::ffi::c_int = *cursor as ::core::ffi::c_int;
                        __res = (if __c < -128 as ::core::ffi::c_int
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = tolower(*cursor as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(*cursor as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            }) as ::core::ffi::c_char;
            separator = r#false != 0;
        } else if !separator && used > 0 as size_t {
            let c2rust_fresh2 = used;
            used = used.wrapping_add(1);
            normalized[c2rust_fresh2] = '_' as ::core::ffi::c_char;
            separator = r#true != 0;
        }
        cursor = cursor.offset(1);
    }
    while used > 0 as size_t
        && normalized[used.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
            == '_' as ::core::ffi::c_int
    {
        used = used.wrapping_sub(1);
    }
    if used == 0 as size_t {
        let c2rust_fresh3 = used;
        used = used.wrapping_add(1);
        normalized[c2rust_fresh3] = 'i' as ::core::ffi::c_char;
    }
    normalized[used] = '\0' as ::core::ffi::c_char;
    snprintf(
        output,
        SYSTEM_HEALTH_ID_LENGTH as size_t,
        b"net.%.34s.%08x\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut normalized as *mut ::core::ffi::c_char,
        fnv1a(internal_name),
    );
}
unsafe extern "C" fn read_text(
    mut path: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut capability: *mut system_health_capability_t,
) -> bool {
    *output.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut file: *mut FILE = fopen(path, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    if file.is_null() {
        *capability = linux_network_capability_from_errno(*__errno_location());
        return r#false != 0;
    }
    let mut length: size_t = fread(
        output as *mut ::core::ffi::c_void,
        1 as size_t,
        NETWORK_TEXT_LENGTH.wrapping_sub(1 as ::core::ffi::c_uint) as size_t,
        file,
    ) as size_t;
    if ferror(file) != 0 {
        let mut saved_errno: ::core::ffi::c_int = *__errno_location();
        fclose(file);
        *capability = linux_network_capability_from_errno(saved_errno);
        *output.offset(0isize) = '\0' as ::core::ffi::c_char;
        return r#false != 0;
    }
    if length == NETWORK_TEXT_LENGTH.wrapping_sub(1 as ::core::ffi::c_uint) as size_t
        && fgetc(file) != EOF
    {
        fclose(file);
        *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        *output.offset(0isize) = '\0' as ::core::ffi::c_char;
        return r#false != 0;
    }
    fclose(file);
    *output.offset(length as isize) = '\0' as ::core::ffi::c_char;
    while length > 0 as size_t
        && *(*__ctype_b_loc())
            .offset(*output.offset(length.wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        length = length.wrapping_sub(1);
        *output.offset(length as isize) = '\0' as ::core::ffi::c_char;
    }
    let mut start: size_t = 0 as size_t;
    while *output.offset(start as isize) as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(
            *output.offset(start as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
        ) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        start = start.wrapping_add(1);
    }
    if start > 0 as size_t {
        memmove(
            output as *mut ::core::ffi::c_void,
            output.offset(start as isize) as *const ::core::ffi::c_void,
            strlen(output.offset(start as isize)).wrapping_add(1 as size_t),
        );
    }
    if *output.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return r#false != 0;
    }
    *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    return r#true != 0;
}
unsafe extern "C" fn read_u64(mut path: *const ::core::ffi::c_char) -> network_integer_t {
    let mut result: network_integer_t = network_integer_t {
        valid: r#false != 0,
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        value: 0 as uint64_t,
    };
    let mut text: [::core::ffi::c_char; 128] = [0; 128];
    if !read_text(
        path,
        &raw mut text as *mut ::core::ffi::c_char,
        &raw mut result.capability,
    ) {
        return result;
    }
    if text[0usize] as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parsed: ::core::ffi::c_ulonglong = strtoull(
        &raw mut text as *mut ::core::ffi::c_char,
        &raw mut end,
        10 as ::core::ffi::c_int,
    );
    if *__errno_location() == ERANGE
        || end == &raw mut text as *mut ::core::ffi::c_char
        || end.is_null()
        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    result.valid = r#true != 0;
    result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    result.value = parsed as uint64_t;
    return result;
}
unsafe extern "C" fn parse_hex_u32(
    mut text: *const ::core::ffi::c_char,
    mut output: *mut uint32_t,
) -> bool {
    if text.is_null()
        || output.is_null()
        || *text.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strlen(text) > 8 as size_t
        || *text.offset(0isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parsed: ::core::ffi::c_ulong = strtoul(text, &raw mut end, 16 as ::core::ffi::c_int);
    if *__errno_location() == ERANGE
        || end == text as *mut ::core::ffi::c_char
        || end.is_null()
        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || parsed > UINT32_MAX as ::core::ffi::c_ulong
    {
        return r#false != 0;
    }
    *output = parsed as uint32_t;
    return r#true != 0;
}
unsafe extern "C" fn parse_decimal_u32(
    mut text: *const ::core::ffi::c_char,
    mut output: *mut uint32_t,
) -> bool {
    if text.is_null()
        || output.is_null()
        || *text.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || *text.offset(0isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parsed: ::core::ffi::c_ulong = strtoul(text, &raw mut end, 10 as ::core::ffi::c_int);
    if *__errno_location() == ERANGE
        || end == text as *mut ::core::ffi::c_char
        || end.is_null()
        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || parsed > UINT32_MAX as ::core::ffi::c_ulong
    {
        return r#false != 0;
    }
    *output = parsed as uint32_t;
    return r#true != 0;
}
unsafe extern "C" fn prefix_bits(mut mask: uint32_t) -> ::core::ffi::c_uint {
    let mut bits: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while mask != 0 as uint32_t {
        bits = bits.wrapping_add((mask & 1 as uint32_t) as ::core::ffi::c_uint);
        mask >>= 1 as ::core::ffi::c_uint;
    }
    return bits;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_network_select_primary(
    mut proc_root: *const ::core::ffi::c_char,
    mut interface_name: *mut ::core::ffi::c_char,
    mut capability: *mut system_health_capability_t,
) -> ::core::ffi::c_int {
    if proc_root.is_null() || interface_name.is_null() || capability.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *interface_name.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut path: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"%s/net/route\0".as_ptr() as *const ::core::ffi::c_char,
        proc_root,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
    {
        *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1 as ::core::ffi::c_int;
    }
    let mut file: *mut FILE = fopen(
        &raw mut path as *mut ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if file.is_null() {
        *capability = linux_network_capability_from_errno(*__errno_location());
        return -1 as ::core::ffi::c_int;
    }
    let mut line: [::core::ffi::c_char; 512] = [0; 512];
    if fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int,
        file,
    )
    .is_null()
    {
        fclose(file);
        *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1 as ::core::ffi::c_int;
    }
    let mut found: bool = r#false != 0;
    let mut saw_well_formed: bool = r#false != 0;
    let mut best_prefix: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut best_metric: uint32_t = UINT32_MAX as uint32_t;
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int,
        file,
    )
    .is_null()
    {
        if strchr(
            &raw mut line as *mut ::core::ffi::c_char,
            '\n' as ::core::ffi::c_int,
        )
        .is_null()
            && feof(file) == 0
        {
            let mut character: ::core::ffi::c_int = 0;
            loop {
                character = fgetc(file);
                if !(character != '\n' as ::core::ffi::c_int && character != EOF) {
                    break;
                }
            }
        } else {
            let mut name: [::core::ffi::c_char; 64] = [0; 64];
            let mut destination_text: [::core::ffi::c_char; 16] = [0; 16];
            let mut gateway_text: [::core::ffi::c_char; 16] = [0; 16];
            let mut flags_text: [::core::ffi::c_char; 16] = [0; 16];
            let mut mask_text: [::core::ffi::c_char; 16] = [0; 16];
            let mut ref_text: [::core::ffi::c_char; 16] = [0; 16];
            let mut use_text: [::core::ffi::c_char; 16] = [0; 16];
            let mut metric_text: [::core::ffi::c_char; 16] = [0; 16];
            let mut mtu_text: [::core::ffi::c_char; 16] = [0; 16];
            let mut window_text: [::core::ffi::c_char; 16] = [0; 16];
            let mut irtt_text: [::core::ffi::c_char; 16] = [0; 16];
            let mut fields: ::core::ffi::c_int = sscanf(
                &raw mut line as *mut ::core::ffi::c_char,
                b"%63s %15s %15s %15s %15s %15s %15s %15s %15s %15s %15s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut name as *mut ::core::ffi::c_char,
                &raw mut destination_text as *mut ::core::ffi::c_char,
                &raw mut gateway_text as *mut ::core::ffi::c_char,
                &raw mut flags_text as *mut ::core::ffi::c_char,
                &raw mut ref_text as *mut ::core::ffi::c_char,
                &raw mut use_text as *mut ::core::ffi::c_char,
                &raw mut metric_text as *mut ::core::ffi::c_char,
                &raw mut mask_text as *mut ::core::ffi::c_char,
                &raw mut mtu_text as *mut ::core::ffi::c_char,
                &raw mut window_text as *mut ::core::ffi::c_char,
                &raw mut irtt_text as *mut ::core::ffi::c_char,
            );
            if fields < 8 as ::core::ffi::c_int
                || !valid_interface_name(&raw mut name as *mut ::core::ffi::c_char)
            {
                continue;
            }
            let mut destination: uint32_t = 0;
            let mut gateway: uint32_t = 0;
            let mut flags: uint32_t = 0;
            let mut mask: uint32_t = 0;
            let mut metric: uint32_t = 0;
            if !parse_hex_u32(
                &raw mut destination_text as *mut ::core::ffi::c_char,
                &raw mut destination,
            ) || !parse_hex_u32(
                &raw mut gateway_text as *mut ::core::ffi::c_char,
                &raw mut gateway,
            ) || !parse_hex_u32(
                &raw mut flags_text as *mut ::core::ffi::c_char,
                &raw mut flags,
            ) || !parse_hex_u32(
                &raw mut mask_text as *mut ::core::ffi::c_char,
                &raw mut mask,
            ) || !parse_decimal_u32(
                &raw mut metric_text as *mut ::core::ffi::c_char,
                &raw mut metric,
            ) {
                continue;
            }
            saw_well_formed = r#true != 0;
            if destination != 0 as uint32_t
                || flags & 0x1 as uint32_t == 0 as uint32_t
                || flags & 0x200 as uint32_t != 0 as uint32_t
            {
                continue;
            }
            let mut prefix: ::core::ffi::c_uint = prefix_bits(mask);
            if !found
                || prefix > best_prefix
                || prefix == best_prefix && metric < best_metric
                || prefix == best_prefix
                    && metric == best_metric
                    && strcmp(&raw mut name as *mut ::core::ffi::c_char, interface_name)
                        < 0 as ::core::ffi::c_int
            {
                snprintf(
                    interface_name,
                    LINUX_NETWORK_INTERNAL_NAME_LENGTH as size_t,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut name as *mut ::core::ffi::c_char,
                );
                best_prefix = prefix;
                best_metric = metric;
                found = r#true != 0;
            }
        }
    }
    let mut read_error: bool = ferror(file) != 0;
    fclose(file);
    if read_error {
        *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        *interface_name.offset(0isize) = '\0' as ::core::ffi::c_char;
        return -1 as ::core::ffi::c_int;
    }
    if !found {
        *capability = system_health_capability_t(
            (if saw_well_formed as ::core::ffi::c_int != 0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                    as ::core::ffi::c_int
            } else {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        return -1 as ::core::ffi::c_int;
    }
    *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_network_state_init(
    mut state: *mut linux_network_state_t,
    mut scope: system_health_scope_t,
) {
    if state.is_null() {
        return;
    }
    memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_network_state_t>(),
    );
    (*state).scope = system_health_scope_t(
        (if scope.0 == system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST.0 {
            system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST.0 as ::core::ffi::c_int
        } else {
            system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_network_set_primary_override(
    mut state: *mut linux_network_state_t,
    mut interface_name: *const ::core::ffi::c_char,
) -> bool {
    if state.is_null() || interface_name.is_null() {
        return r#false != 0;
    }
    if *interface_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        (*state).primary_override[0usize] = '\0' as ::core::ffi::c_char;
        return r#true != 0;
    }
    if !valid_interface_name(interface_name) {
        return r#false != 0;
    }
    snprintf(
        &raw mut (*state).primary_override as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        interface_name,
    );
    return r#true != 0;
}
unsafe extern "C" fn name_consider(
    mut names: *mut [::core::ffi::c_char; 64],
    mut stored: *mut size_t,
    mut total: *mut size_t,
    mut name: *const ::core::ffi::c_char,
) {
    *total = (*total).wrapping_add(1);
    let mut position: size_t = *stored;
    if *stored < NETWORK_DIRECTORY_CAPACITY as size_t {
        *stored = (*stored).wrapping_add(1);
    } else {
        if strcmp(
            name,
            &raw mut *names
                .offset(NETWORK_DIRECTORY_CAPACITY.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
                as *mut ::core::ffi::c_char,
        ) >= 0 as ::core::ffi::c_int
        {
            return;
        }
        position = NETWORK_DIRECTORY_CAPACITY.wrapping_sub(1 as ::core::ffi::c_uint) as size_t;
    }
    while position > 0 as size_t
        && strcmp(
            name,
            &raw mut *names.offset(position.wrapping_sub(1 as size_t) as isize)
                as *mut ::core::ffi::c_char,
        ) < 0 as ::core::ffi::c_int
    {
        if position < NETWORK_DIRECTORY_CAPACITY as size_t {
            memmove(
                &raw mut *names.offset(position as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                &raw mut *names.offset(position.wrapping_sub(1 as size_t) as isize)
                    as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                LINUX_NETWORK_INTERNAL_NAME_LENGTH as size_t,
            );
        }
        position = position.wrapping_sub(1);
    }
    snprintf(
        &raw mut *names.offset(position as isize) as *mut ::core::ffi::c_char,
        LINUX_NETWORK_INTERNAL_NAME_LENGTH as size_t,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        name,
    );
}
unsafe extern "C" fn name_in_list(
    mut names: *mut [::core::ffi::c_char; 64],
    mut count: size_t,
    mut name: *const ::core::ffi::c_char,
    mut index_out: *mut size_t,
) -> bool {
    let mut index: size_t = 0 as size_t;
    while index < count {
        let mut order: ::core::ffi::c_int = strcmp(
            &raw mut *names.offset(index as isize) as *mut ::core::ffi::c_char,
            name,
        );
        if order == 0 as ::core::ffi::c_int {
            if !index_out.is_null() {
                *index_out = index;
            }
            return r#true != 0;
        }
        if order > 0 as ::core::ffi::c_int {
            break;
        }
        index = index.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn track_interface(
    mut state: *mut linux_network_state_t,
    mut internal_name: *const ::core::ffi::c_char,
) -> *mut linux_network_interface_state_t {
    let mut index: size_t = 0 as size_t;
    while index < (*state).interface_count {
        if strcmp(
            &raw mut (*(&raw mut (*state).interfaces as *mut linux_network_interface_state_t)
                .offset(index as isize))
            .internal_name as *mut ::core::ffi::c_char,
            internal_name,
        ) == 0 as ::core::ffi::c_int
        {
            return (&raw mut (*state).interfaces as *mut linux_network_interface_state_t)
                .offset(index as isize);
        }
        index = index.wrapping_add(1);
    }
    let mut slot: size_t = (*state).interface_count;
    if slot >= LINUX_NETWORK_STATE_CAPACITY as size_t {
        let mut index_0: size_t = 0 as size_t;
        while index_0 < (*state).interface_count {
            if !(*state).interfaces[index_0].present
                && (*state).interfaces[index_0].missing_cycles as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
            {
                slot = index_0;
                break;
            } else {
                index_0 = index_0.wrapping_add(1);
            }
        }
    } else {
        (*state).interface_count = (*state).interface_count.wrapping_add(1);
    }
    if slot >= LINUX_NETWORK_STATE_CAPACITY as size_t {
        return ::core::ptr::null_mut::<linux_network_interface_state_t>();
    }
    memset(
        (&raw mut (*state).interfaces as *mut linux_network_interface_state_t).offset(slot as isize)
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_network_interface_state_t>(),
    );
    snprintf(
        &raw mut (*(&raw mut (*state).interfaces as *mut linux_network_interface_state_t)
            .offset(slot as isize))
        .internal_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        internal_name,
    );
    make_public_id(
        internal_name,
        &raw mut (*(&raw mut (*state).interfaces as *mut linux_network_interface_state_t)
            .offset(slot as isize))
        .id as *mut ::core::ffi::c_char,
    );
    return (&raw mut (*state).interfaces as *mut linux_network_interface_state_t)
        .offset(slot as isize);
}
unsafe extern "C" fn append_observation(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut scope: system_health_scope_t,
    mut metric: *const ::core::ffi::c_char,
    mut resource_id: *const ::core::ffi::c_char,
    mut capability: system_health_capability_t,
    mut value_valid: bool,
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
        resource_id,
    );
    observation.scope = scope;
    observation.sampled_monotonic_ms = (*context).monotonic_ms;
    observation.observed_wall_time_ms = (*context).wall_time_ms;
    if value_valid {
        system_health_observation_set_available(&raw mut observation, value, unit);
    } else {
        system_health_observation_set_unavailable(&raw mut observation, capability);
    }
    system_health_observation_sink_append(sink, &raw mut observation);
}
unsafe extern "C" fn update_counter(
    mut tracked: *mut linux_network_interface_state_t,
    mut kind: linux_network_counter_kind_t,
    mut reading: network_integer_t,
    mut monotonic_ms: uint64_t,
) -> system_health_counter_t {
    let mut counter: system_health_counter_t = system_health_counter_t {
        delta_valid: false,
        reset_detected: false,
        current: 0,
        previous: 0,
        delta: 0,
        interval_ms: 0,
        sampled_monotonic_ms: 0,
    };
    memset(
        &raw mut counter as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_counter_t>(),
    );
    counter.sampled_monotonic_ms = monotonic_ms;
    if !reading.valid {
        (*tracked).previous_valid[kind.0 as usize] = r#false != 0;
        return counter;
    }
    counter.current = reading.value;
    if (*tracked).previous_valid[kind.0 as usize] {
        counter.previous = (*tracked).previous[kind.0 as usize];
        if reading.value < (*tracked).previous[kind.0 as usize] {
            counter.reset_detected = r#true != 0;
        } else if monotonic_ms > (*tracked).previous_monotonic_ms {
            counter.delta_valid = r#true != 0;
            counter.delta = reading
                .value
                .wrapping_sub((*tracked).previous[kind.0 as usize]);
            counter.interval_ms = monotonic_ms.wrapping_sub((*tracked).previous_monotonic_ms);
        }
    }
    (*tracked).previous[kind.0 as usize] = reading.value;
    (*tracked).previous_valid[kind.0 as usize] = r#true != 0;
    return counter;
}
unsafe extern "C" fn saturating_add_u64(mut left: uint64_t, mut right: uint64_t) -> uint64_t {
    return if (UINT64_MAX as uint64_t).wrapping_sub(left) < right {
        UINT64_MAX as uint64_t
    } else {
        left.wrapping_add(right)
    };
}
unsafe extern "C" fn all_rate_counters_valid(
    mut sample: *const linux_network_interface_sample_t,
) -> bool {
    let kinds: [linux_network_counter_kind_t; 6] = [
        linux_network_counter_kind_t::LINUX_NETWORK_RX_PACKETS,
        linux_network_counter_kind_t::LINUX_NETWORK_TX_PACKETS,
        linux_network_counter_kind_t::LINUX_NETWORK_RX_ERRORS,
        linux_network_counter_kind_t::LINUX_NETWORK_TX_ERRORS,
        linux_network_counter_kind_t::LINUX_NETWORK_RX_DROPS,
        linux_network_counter_kind_t::LINUX_NETWORK_TX_DROPS,
    ];
    let mut index: size_t = 0 as size_t;
    while index
        < ::core::mem::size_of::<[linux_network_counter_kind_t; 6]>()
            .wrapping_div(::core::mem::size_of::<linux_network_counter_kind_t>())
    {
        if !(*sample).counters[kinds[index].0 as usize].delta_valid {
            return r#false != 0;
        }
        index = index.wrapping_add(1);
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_network_collect(
    mut state: *mut linux_network_state_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
    mut result: *mut linux_network_result_t,
) -> ::core::ffi::c_int {
    if state.is_null()
        || context.is_null()
        || (*context).sys_root.is_null()
        || (*context).proc_root.is_null()
        || sink.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    if !result.is_null() {
        memset(
            result as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<linux_network_result_t>(),
        );
    }
    let mut index: size_t = 0 as size_t;
    while index < (*state).interface_count {
        (*state).interfaces[index].seen = r#false != 0;
        index = index.wrapping_add(1);
    }
    let mut primary_name: [::core::ffi::c_char; 64] = ::core::mem::transmute::<
        [u8; 64],
        [::core::ffi::c_char; 64],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut primary_capability: system_health_capability_t =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    if (*state).primary_override[0usize] != 0 {
        snprintf(
            &raw mut primary_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*state).primary_override as *mut ::core::ffi::c_char,
        );
        primary_capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    } else {
        linux_network_select_primary(
            (*context).proc_root,
            &raw mut primary_name as *mut ::core::ffi::c_char,
            &raw mut primary_capability,
        );
    }
    let mut class_path: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut class_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"%s/class/net\0".as_ptr() as *const ::core::ffi::c_char,
        (*context).sys_root,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut directory: *mut DIR = opendir(&raw mut class_path as *mut ::core::ffi::c_char);
    let mut overall: system_health_capability_t =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    let mut names: [[::core::ffi::c_char; 64]; 128] = [[0; 64]; 128];
    let mut stored_names: size_t = 0 as size_t;
    let mut total_names: size_t = 0 as size_t;
    if directory.is_null() {
        overall = linux_network_capability_from_errno(*__errno_location());
    } else {
        let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
        loop {
            entry = readdir(directory);
            if entry.is_null() {
                break;
            }
            if !valid_interface_name(&raw mut (*entry).d_name as *mut ::core::ffi::c_char) {
                continue;
            }
            name_consider(
                &raw mut names as *mut [::core::ffi::c_char; 64],
                &raw mut stored_names,
                &raw mut total_names,
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            );
        }
        closedir(directory);
        if total_names == 0 as size_t {
            overall = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        }
    }
    let mut primary_index: size_t = 0 as size_t;
    let mut primary_present: bool = primary_capability.0
        == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        && name_in_list(
            &raw mut names as *mut [::core::ffi::c_char; 64],
            stored_names,
            &raw mut primary_name as *mut ::core::ffi::c_char,
            &raw mut primary_index,
        ) as ::core::ffi::c_int
            != 0;
    if primary_capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        && !primary_present
    {
        primary_capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE;
        primary_name[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut selected: [[::core::ffi::c_char; 64]; 16] = [[0; 64]; 16];
    let mut selected_count: size_t = if stored_names < SYSTEM_HEALTH_MAX_INTERFACES as size_t {
        stored_names
    } else {
        SYSTEM_HEALTH_MAX_INTERFACES as size_t
    };
    let mut index_0: size_t = 0 as size_t;
    while index_0 < selected_count {
        snprintf(
            &raw mut *(&raw mut selected as *mut [::core::ffi::c_char; 64]).offset(index_0 as isize)
                as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut *(&raw mut names as *mut [::core::ffi::c_char; 64]).offset(index_0 as isize)
                as *mut ::core::ffi::c_char,
        );
        index_0 = index_0.wrapping_add(1);
    }
    if primary_present as ::core::ffi::c_int != 0
        && primary_index >= selected_count
        && selected_count > 0 as size_t
    {
        snprintf(
            &raw mut *(&raw mut selected as *mut [::core::ffi::c_char; 64])
                .offset(selected_count.wrapping_sub(1 as size_t) as isize)
                as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut *(&raw mut names as *mut [::core::ffi::c_char; 64])
                .offset(primary_index as isize) as *mut ::core::ffi::c_char,
        );
        let mut index_1: size_t = selected_count.wrapping_sub(1 as size_t);
        while index_1 > 0 as size_t
            && strcmp(
                &raw mut *(&raw mut selected as *mut [::core::ffi::c_char; 64])
                    .offset(index_1 as isize) as *mut ::core::ffi::c_char,
                &raw mut *(&raw mut selected as *mut [::core::ffi::c_char; 64])
                    .offset(index_1.wrapping_sub(1 as size_t) as isize)
                    as *mut ::core::ffi::c_char,
            ) < 0 as ::core::ffi::c_int
        {
            let mut swap: [::core::ffi::c_char; 64] = [0; 64];
            snprintf(
                &raw mut swap as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut *(&raw mut selected as *mut [::core::ffi::c_char; 64])
                    .offset(index_1 as isize) as *mut ::core::ffi::c_char,
            );
            memmove(
                &raw mut *(&raw mut selected as *mut [::core::ffi::c_char; 64])
                    .offset(index_1 as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                &raw mut *(&raw mut selected as *mut [::core::ffi::c_char; 64])
                    .offset(index_1.wrapping_sub(1 as size_t) as isize)
                    as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            );
            snprintf(
                &raw mut *(&raw mut selected as *mut [::core::ffi::c_char; 64])
                    .offset(index_1.wrapping_sub(1 as size_t) as isize)
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut swap as *mut ::core::ffi::c_char,
            );
            index_1 = index_1.wrapping_sub(1);
        }
    }
    let mut resources_dropped: size_t = if total_names > selected_count {
        total_names.wrapping_sub(selected_count)
    } else {
        0 as size_t
    };
    (*state).resources_dropped_total = ((*state).resources_dropped_total as ::core::ffi::c_ulong)
        .wrapping_add(resources_dropped as ::core::ffi::c_ulong)
        as uint64_t;
    if !result.is_null() {
        (*result).capability = overall;
        (*result).primary_capability = primary_capability;
        (*result).resources_dropped = resources_dropped;
    }
    append_observation(
        sink,
        context,
        (*state).scope,
        b"network.collector_available\0".as_ptr() as *const ::core::ffi::c_char,
        b"network\0".as_ptr() as *const ::core::ffi::c_char,
        overall,
        overall.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0,
        1.0f64,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    append_observation(
        sink,
        context,
        (*state).scope,
        b"network.primary_available\0".as_ptr() as *const ::core::ffi::c_char,
        b"network\0".as_ptr() as *const ::core::ffi::c_char,
        primary_capability,
        primary_capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0,
        1.0f64,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    let mut interface_index: size_t = 0 as size_t;
    while interface_index < selected_count {
        let mut internal_name: *const ::core::ffi::c_char =
            &raw mut *(&raw mut selected as *mut [::core::ffi::c_char; 64])
                .offset(interface_index as isize) as *mut ::core::ffi::c_char;
        let mut tracked: *mut linux_network_interface_state_t =
            track_interface(state, internal_name);
        if tracked.is_null() {
            (*state).resources_dropped_total = (*state).resources_dropped_total.wrapping_add(1);
            if !result.is_null() {
                (*result).resources_dropped = (*result).resources_dropped.wrapping_add(1);
            }
        } else {
            let mut was_present: bool = (*tracked).present;
            (*tracked).seen = r#true != 0;
            (*tracked).present = r#true != 0;
            (*tracked).missing_cycles = 0 as uint8_t;
            if !was_present {
                memset(
                    &raw mut (*tracked).previous_valid as *mut bool as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<[bool; 8]>(),
                );
                (*tracked).carrier_valid = r#false != 0;
            }
            let mut sample: linux_network_interface_sample_t = linux_network_interface_sample_t {
                id: [0; 64],
                primary: false,
                carrier_capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                carrier_valid: false,
                carrier: false,
                carrier_flaps: 0,
                carrier_flap_delta: 0,
                counters: [system_health_counter_t {
                    delta_valid: false,
                    reset_detected: false,
                    current: 0,
                    previous: 0,
                    delta: 0,
                    interval_ms: 0,
                    sampled_monotonic_ms: 0,
                }; 8],
                error_drop_ratio_valid: false,
                error_drop_ratio: 0.,
            };
            memset(
                &raw mut sample as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<linux_network_interface_sample_t>(),
            );
            snprintf(
                &raw mut sample.id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*tracked).id as *mut ::core::ffi::c_char,
            );
            sample.primary = primary_capability.0
                == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
                && strcmp(
                    internal_name,
                    &raw mut primary_name as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int;
            let mut path: [::core::ffi::c_char; 4096] = [0; 4096];
            written = snprintf(
                &raw mut path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                b"%s/%s/carrier\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut class_path as *mut ::core::ffi::c_char,
                internal_name,
            );
            let mut carrier: network_integer_t = if written >= 0 as ::core::ffi::c_int
                && (written as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
            {
                read_u64(&raw mut path as *mut ::core::ffi::c_char)
            } else {
                network_integer_t {
                    valid: r#false != 0,
                    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
                    value: 0 as uint64_t,
                }
            };
            if carrier.valid as ::core::ffi::c_int != 0 && carrier.value > 1 as uint64_t {
                carrier.valid = r#false != 0;
                carrier.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
            }
            sample.carrier_capability = carrier.capability;
            sample.carrier_valid = carrier.valid;
            sample.carrier =
                carrier.valid as ::core::ffi::c_int != 0 && carrier.value == 1 as uint64_t;
            if carrier.valid {
                if (*tracked).carrier_valid as ::core::ffi::c_int != 0
                    && (*tracked).carrier as ::core::ffi::c_int
                        != sample.carrier as ::core::ffi::c_int
                {
                    (*tracked).carrier_flaps = (*tracked).carrier_flaps.wrapping_add(1);
                    sample.carrier_flap_delta = 1 as uint64_t;
                }
                (*tracked).carrier = sample.carrier;
                (*tracked).carrier_valid = r#true != 0;
            } else {
                (*tracked).carrier_valid = r#false != 0;
            }
            sample.carrier_flaps = (*tracked).carrier_flaps;
            append_observation(
                sink,
                context,
                (*state).scope,
                b"network.carrier\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*tracked).id as *mut ::core::ffi::c_char,
                carrier.capability,
                carrier.valid,
                if sample.carrier as ::core::ffi::c_int != 0 {
                    1.0f64
                } else {
                    0.0f64
                },
                system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
            );
            let mut any_reset: bool = r#false != 0;
            let mut counter_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while counter_index
                < linux_network_counter_kind_t::LINUX_NETWORK_COUNTER_COUNT.0 as ::core::ffi::c_int
            {
                written = snprintf(
                    &raw mut path as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                    b"%s/%s/statistics/%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut class_path as *mut ::core::ffi::c_char,
                    internal_name,
                    counter_files[counter_index as usize],
                );
                let mut reading: network_integer_t = if written >= 0 as ::core::ffi::c_int
                    && (written as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
                {
                    read_u64(&raw mut path as *mut ::core::ffi::c_char)
                } else {
                    network_integer_t {
                        valid: r#false != 0,
                        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
                        value: 0 as uint64_t,
                    }
                };
                let mut had_previous: bool = (*tracked).previous_valid[counter_index as usize];
                sample.counters[counter_index as usize] = update_counter(
                    tracked,
                    linux_network_counter_kind_t(counter_index as ::core::ffi::c_uint),
                    reading,
                    (*context).monotonic_ms,
                );
                if reading.valid {
                    let mut increment: uint64_t = 0 as uint64_t;
                    if !had_previous
                        || sample.counters[counter_index as usize].reset_detected
                            as ::core::ffi::c_int
                            != 0
                    {
                        increment = reading.value;
                    } else if sample.counters[counter_index as usize].delta_valid {
                        increment = sample.counters[counter_index as usize].delta;
                    }
                    (*tracked).exported_total[counter_index as usize] = saturating_add_u64(
                        (*tracked).exported_total[counter_index as usize],
                        increment,
                    );
                }
                any_reset = any_reset as ::core::ffi::c_int != 0
                    || sample.counters[counter_index as usize].reset_detected as ::core::ffi::c_int
                        != 0;
                append_observation(
                    sink,
                    context,
                    (*state).scope,
                    counter_metrics[counter_index as usize],
                    &raw mut (*tracked).id as *mut ::core::ffi::c_char,
                    reading.capability,
                    reading.valid,
                    (*tracked).exported_total[counter_index as usize] as ::core::ffi::c_double,
                    counter_units[counter_index as usize],
                );
                counter_index += 1;
            }
            (*tracked).previous_monotonic_ms = (*context).monotonic_ms;
            if all_rate_counters_valid(&raw mut sample) {
                let mut packets: ::f128::f128 = ::f128::f128::new(
                    sample.counters[linux_network_counter_kind_t::LINUX_NETWORK_RX_PACKETS.0
                        as ::core::ffi::c_int as usize]
                        .delta,
                ) + ::f128::f128::new(
                    sample.counters[linux_network_counter_kind_t::LINUX_NETWORK_TX_PACKETS.0
                        as ::core::ffi::c_int as usize]
                        .delta,
                );
                let mut bad: ::f128::f128 = ::f128::f128::new(
                    sample.counters[linux_network_counter_kind_t::LINUX_NETWORK_RX_ERRORS.0
                        as ::core::ffi::c_int as usize]
                        .delta,
                ) + ::f128::f128::new(
                    sample.counters[linux_network_counter_kind_t::LINUX_NETWORK_TX_ERRORS.0
                        as ::core::ffi::c_int as usize]
                        .delta,
                ) + ::f128::f128::new(
                    sample.counters[linux_network_counter_kind_t::LINUX_NETWORK_RX_DROPS.0
                        as ::core::ffi::c_int as usize]
                        .delta,
                ) + ::f128::f128::new(
                    sample.counters[linux_network_counter_kind_t::LINUX_NETWORK_TX_DROPS.0
                        as ::core::ffi::c_int as usize]
                        .delta,
                );
                if packets > ::f128::f128::new(0.0) {
                    sample.error_drop_ratio_valid = r#true != 0;
                    sample.error_drop_ratio = Into::<::core::ffi::c_double>::into(bad / packets);
                }
            }
            append_observation(
                sink,
                context,
                (*state).scope,
                b"network.error_drop_ratio\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*tracked).id as *mut ::core::ffi::c_char,
                system_health_capability_t(
                    (if any_reset as ::core::ffi::c_int != 0 {
                        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
                            as ::core::ffi::c_int
                    } else {
                        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE.0
                            as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ),
                sample.error_drop_ratio_valid,
                sample.error_drop_ratio,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
            );
            append_observation(
                sink,
                context,
                (*state).scope,
                b"network.primary\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*tracked).id as *mut ::core::ffi::c_char,
                primary_capability,
                primary_capability.0
                    == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0,
                if sample.primary as ::core::ffi::c_int != 0 {
                    1.0f64
                } else {
                    0.0f64
                },
                system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
            );
            append_observation(
                sink,
                context,
                (*state).scope,
                b"network.counter_reset\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*tracked).id as *mut ::core::ffi::c_char,
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                r#true != 0,
                if any_reset as ::core::ffi::c_int != 0 {
                    1.0f64
                } else {
                    0.0f64
                },
                system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
            );
            if sample.primary as ::core::ffi::c_int != 0 && !result.is_null() {
                snprintf(
                    &raw mut (*result).primary_id as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*tracked).id as *mut ::core::ffi::c_char,
                );
            }
            if !result.is_null()
                && (*result).interface_count < SYSTEM_HEALTH_MAX_INTERFACES as size_t
            {
                let c2rust_fresh0 = (*result).interface_count;
                (*result).interface_count = (*result).interface_count.wrapping_add(1);
                (*result).interfaces[c2rust_fresh0] = sample;
            }
        }
        interface_index = interface_index.wrapping_add(1);
    }
    let mut index_2: size_t = 0 as size_t;
    while index_2 < (*state).interface_count {
        let mut tracked_0: *mut linux_network_interface_state_t = (&raw mut (*state).interfaces
            as *mut linux_network_interface_state_t)
            .offset(index_2 as isize);
        if !(*tracked_0).seen {
            if (*tracked_0).present {
                (*tracked_0).present = r#false != 0;
                (*tracked_0).missing_cycles = 1 as uint8_t;
                memset(
                    &raw mut (*tracked_0).previous_valid as *mut bool as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<[bool; 8]>(),
                );
                (*tracked_0).carrier_valid = r#false != 0;
                append_observation(
                    sink,
                    context,
                    (*state).scope,
                    b"network.carrier\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*tracked_0).id as *mut ::core::ffi::c_char,
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE,
                    r#false != 0,
                    0.0f64,
                    system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
                );
            } else if ((*tracked_0).missing_cycles as ::core::ffi::c_int) < UINT8_MAX {
                (*tracked_0).missing_cycles = (*tracked_0).missing_cycles.wrapping_add(1);
            }
        }
        index_2 = index_2.wrapping_add(1);
    }
    if resources_dropped > 0 as size_t {
        append_observation(
            sink,
            context,
            (*state).scope,
            b"network.interfaces_dropped\0".as_ptr() as *const ::core::ffi::c_char,
            b"network\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            r#true != 0,
            resources_dropped as ::core::ffi::c_double,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn collect_adapter(
    mut state: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    return linux_network_collect(
        state as *mut linux_network_state_t,
        context,
        sink,
        ::core::ptr::null_mut::<linux_network_result_t>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_network_collector_init(
    mut collector: *mut system_health_collector_t,
    mut state: *mut linux_network_state_t,
    mut interval_seconds: uint32_t,
    mut stale_after_seconds: uint32_t,
) -> bool {
    if collector.is_null()
        || state.is_null()
        || interval_seconds == 0 as uint32_t
        || stale_after_seconds < interval_seconds
    {
        return r#false != 0;
    }
    memset(
        collector as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_collector_t>(),
    );
    snprintf(
        &raw mut (*collector).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"linux_network\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*collector).scope = (*state).scope;
    (*collector).tier = system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL;
    (*collector).interval_seconds = interval_seconds;
    (*collector).stale_after_seconds = stale_after_seconds;
    (*collector).state = state as *mut ::core::ffi::c_void;
    (*collector).collect = Some(
        collect_adapter
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_observation_sink_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_collect_fn;
    return r#true != 0;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
