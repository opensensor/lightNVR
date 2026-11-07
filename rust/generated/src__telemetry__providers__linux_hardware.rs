use ::c2rust_bitfields;
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
    unsafe fn linux_device_map_build(
        sys_root: *const ::core::ffi::c_char,
        installation_scope: *const ::core::ffi::c_char,
        map: *mut linux_device_map_t,
    ) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn event_identity_get_source(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
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
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub struct linux_hardware_resource_kind_t(pub ::core::ffi::c_uint);
impl linux_hardware_resource_kind_t {
    pub const LINUX_HARDWARE_RESOURCE_FLASH: Self = Self(0);
    pub const LINUX_HARDWARE_RESOURCE_EDAC: Self = Self(1);
    pub const LINUX_HARDWARE_RESOURCE_FAN: Self = Self(2);
    pub const LINUX_HARDWARE_RESOURCE_BOARD: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_hardware_resource_t {
    pub kind: linux_hardware_resource_kind_t,
    pub internal_name: [::core::ffi::c_char; 64],
    pub public_id: [::core::ffi::c_char; 64],
    pub channel: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_hardware_counter_state_t {
    pub used: bool,
    pub public_id: [::core::ffi::c_char; 64],
    pub metric: [::core::ffi::c_char; 64],
    pub previous: uint64_t,
    pub previous_monotonic_ms: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_hardware_state_t {
    pub resources: [linux_hardware_resource_t; 16],
    pub resource_count: size_t,
    pub resources_dropped: size_t,
    pub discovery_capability: system_health_capability_t,
    pub installation_scope: [::core::ffi::c_char; 96],
    pub discovered_sys_root: [::core::ffi::c_char; 1024],
    pub fan_hot_celsius: ::core::ffi::c_double,
    pub counters: [linux_hardware_counter_state_t; 32],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hardware_integer_t {
    pub valid: bool,
    pub value: uint64_t,
    pub capability: system_health_capability_t,
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
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const UINT8_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const SYSTEM_HEALTH_MAX_DEVICES: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const LINUX_HARDWARE_MAX_RESOURCES: ::core::ffi::c_uint = SYSTEM_HEALTH_MAX_DEVICES;
pub const LINUX_HARDWARE_COUNTER_SLOTS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const HARDWARE_TEXT_MAX: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
pub const HARDWARE_SCAN_MAX: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
unsafe extern "C" fn capability_from_errno(
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
unsafe extern "C" fn merge_capability(
    mut left: system_health_capability_t,
    mut right: system_health_capability_t,
) -> system_health_capability_t {
    if left.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        || right.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    if left.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED.0
        || right.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED.0
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if left.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
        || right.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
}
unsafe extern "C" fn suffix_number(
    mut name: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
    mut number: *mut ::core::ffi::c_uint,
) -> bool {
    if name.is_null()
        || prefix.is_null()
        || strncmp(name, prefix, strlen(prefix)) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_char = name.offset(strlen(prefix) as isize);
    if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        == 0
    {
        return r#false != 0;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parsed: ::core::ffi::c_ulong = strtoul(cursor, &raw mut end, 10 as ::core::ffi::c_int);
    if *__errno_location() == ERANGE
        || end == cursor as *mut ::core::ffi::c_char
        || *end as ::core::ffi::c_int != 0
        || parsed > 999 as ::core::ffi::c_ulong
    {
        return r#false != 0;
    }
    if !number.is_null() {
        *number = parsed as ::core::ffi::c_uint;
    }
    return r#true != 0;
}
unsafe extern "C" fn format_path(
    mut output: *mut ::core::ffi::c_char,
    mut capacity: size_t,
    mut format: *const ::core::ffi::c_char,
    mut root: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut channel: ::core::ffi::c_uint,
) -> bool {
    let mut written: ::core::ffi::c_int = snprintf(output, capacity, format, root, name, channel);
    return written >= 0 as ::core::ffi::c_int && (written as size_t) < capacity;
}
unsafe extern "C" fn join_path(
    mut output: *mut ::core::ffi::c_char,
    mut capacity: size_t,
    mut left: *const ::core::ffi::c_char,
    mut right: *const ::core::ffi::c_char,
) -> bool {
    let mut left_length: size_t = strlen(left);
    let mut right_length: size_t = strlen(right);
    if left_length >= capacity
        || right_length >= capacity
        || left_length
            .wrapping_add(1 as size_t)
            .wrapping_add(right_length)
            >= capacity
    {
        return r#false != 0;
    }
    memcpy(
        output as *mut ::core::ffi::c_void,
        left as *const ::core::ffi::c_void,
        left_length,
    );
    *output.offset(left_length as isize) = '/' as ::core::ffi::c_char;
    memcpy(
        output
            .offset(left_length as isize)
            .offset(1 as ::core::ffi::c_uint as isize) as *mut ::core::ffi::c_void,
        right as *const ::core::ffi::c_void,
        right_length.wrapping_add(1 as size_t),
    );
    return r#true != 0;
}
unsafe extern "C" fn read_text(
    mut path: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut capability: *mut system_health_capability_t,
) -> bool {
    let mut file: *mut FILE = fopen(path, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    if file.is_null() {
        *capability = capability_from_errno(*__errno_location());
        return r#false != 0;
    }
    let mut length: size_t = fread(
        output as *mut ::core::ffi::c_void,
        1 as size_t,
        HARDWARE_TEXT_MAX.wrapping_sub(1 as ::core::ffi::c_uint) as size_t,
        file,
    ) as size_t;
    if ferror(file) != 0 {
        let mut saved: ::core::ffi::c_int = *__errno_location();
        fclose(file);
        *capability = capability_from_errno(saved);
        return r#false != 0;
    }
    if length == HARDWARE_TEXT_MAX.wrapping_sub(1 as ::core::ffi::c_uint) as size_t
        && fgetc(file) != EOF
    {
        fclose(file);
        *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
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
    while *(*__ctype_b_loc()).offset(
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
    if *output.offset(0isize) == 0 {
        *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return r#false != 0;
    }
    *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    return r#true != 0;
}
unsafe extern "C" fn read_integer(
    mut path: *const ::core::ffi::c_char,
    mut base: ::core::ffi::c_int,
) -> hardware_integer_t {
    let mut result: hardware_integer_t = hardware_integer_t {
        valid: false,
        value: 0,
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
    };
    let mut text: [::core::ffi::c_char; 128] = [0; 128];
    if !read_text(
        path,
        &raw mut text as *mut ::core::ffi::c_char,
        &raw mut result.capability,
    ) {
        return result;
    }
    if text[0usize] as ::core::ffi::c_int == '-' as ::core::ffi::c_int
        || text[0usize] as ::core::ffi::c_int == '+' as ::core::ffi::c_int
    {
        result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parsed: ::core::ffi::c_ulonglong = strtoull(
        &raw mut text as *mut ::core::ffi::c_char,
        &raw mut end,
        base,
    );
    if *__errno_location() == ERANGE || end == &raw mut text as *mut ::core::ffi::c_char {
        result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    while *(*__ctype_b_loc()).offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        end = end.offset(1);
    }
    if *end != 0 {
        result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    result.valid = r#true != 0;
    result.value = parsed as uint64_t;
    result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    return result;
}
unsafe extern "C" fn scoped_hash(
    mut scope: *const ::core::ffi::c_char,
    mut kind: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut channel: ::core::ffi::c_uint,
) -> uint64_t {
    let mut hash: uint64_t = 1469598103934665603 as uint64_t;
    let mut number: [::core::ffi::c_char; 16] = [0; 16];
    snprintf(
        &raw mut number as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        channel,
    );
    let mut parts: [*const ::core::ffi::c_char; 7] = [
        scope,
        b"|\0".as_ptr() as *const ::core::ffi::c_char,
        kind,
        b"|\0".as_ptr() as *const ::core::ffi::c_char,
        name,
        b"|\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut number as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
    ];
    let mut part: size_t = 0 as size_t;
    while part
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 7]>()
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
unsafe extern "C" fn add_resource(
    mut state: *mut linux_hardware_state_t,
    mut kind: linux_hardware_resource_kind_t,
    mut internal_name: *const ::core::ffi::c_char,
    mut public_id: *const ::core::ffi::c_char,
    mut channel: ::core::ffi::c_uint,
) -> bool {
    let mut index: size_t = 0 as size_t;
    while index < (*state).resource_count {
        let mut current: *mut linux_hardware_resource_t =
            (&raw mut (*state).resources as *mut linux_hardware_resource_t).offset(index as isize);
        if (*current).kind.0 == kind.0
            && (*current).channel == channel
            && strcmp(
                &raw mut (*current).internal_name as *mut ::core::ffi::c_char,
                internal_name,
            ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        index = index.wrapping_add(1);
    }
    if (*state).resource_count >= LINUX_HARDWARE_MAX_RESOURCES as size_t {
        (*state).resources_dropped = (*state).resources_dropped.wrapping_add(1);
        return r#false != 0;
    }
    let c2rust_fresh0 = (*state).resource_count;
    (*state).resource_count = (*state).resource_count.wrapping_add(1);
    let mut resource: *mut linux_hardware_resource_t = (&raw mut (*state).resources
        as *mut linux_hardware_resource_t)
        .offset(c2rust_fresh0 as isize);
    memset(
        resource as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_hardware_resource_t>(),
    );
    (*resource).kind = kind;
    (*resource).channel = channel;
    snprintf(
        &raw mut (*resource).internal_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        internal_name,
    );
    snprintf(
        &raw mut (*resource).public_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        public_id,
    );
    return r#true != 0;
}
unsafe extern "C" fn compare_resources(
    mut left: *const ::core::ffi::c_void,
    mut right: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const linux_hardware_resource_t = left as *const linux_hardware_resource_t;
    let mut b: *const linux_hardware_resource_t = right as *const linux_hardware_resource_t;
    let mut result: ::core::ffi::c_int = strcmp(
        &raw const (*a).public_id as *const ::core::ffi::c_char,
        &raw const (*b).public_id as *const ::core::ffi::c_char,
    );
    return if result != 0 as ::core::ffi::c_int {
        result
    } else {
        (*a).kind.0 as ::core::ffi::c_int - (*b).kind.0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn flash_files_present(
    mut root: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    static mut files: [*const ::core::ffi::c_char; 5] = [
        b"life_time\0".as_ptr() as *const ::core::ffi::c_char,
        b"life_time_estimation\0".as_ptr() as *const ::core::ffi::c_char,
        b"life_time_estimation_a\0".as_ptr() as *const ::core::ffi::c_char,
        b"pre_eol_info\0".as_ptr() as *const ::core::ffi::c_char,
        b"pre_eol\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut path: [::core::ffi::c_char; 1200] = [0; 1200];
    let mut info: stat = stat {
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
    let mut index: size_t = 0 as size_t;
    while index
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        let mut written: ::core::ffi::c_int = snprintf(
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
            b"%s/class/block/%s/device/%s\0".as_ptr() as *const ::core::ffi::c_char,
            root,
            name,
            files[index],
        );
        if written >= 0 as ::core::ffi::c_int
            && (written as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 1200]>()
            && stat(&raw mut path as *mut ::core::ffi::c_char, &raw mut info)
                == 0 as ::core::ffi::c_int
            && info.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
        {
            return r#true != 0;
        }
        index = index.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn discover_flash(
    mut state: *mut linux_hardware_state_t,
    mut root: *const ::core::ffi::c_char,
    mut capability: *mut system_health_capability_t,
) {
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
    if linux_device_map_build(
        root,
        &raw mut (*state).installation_scope as *mut ::core::ffi::c_char,
        &raw mut map,
    ) != 0 as ::core::ffi::c_int
    {
        *capability = merge_capability(
            *capability,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
        return;
    }
    if map.capability.0 != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 {
        *capability = merge_capability(*capability, map.capability);
        return;
    }
    (*state).resources_dropped = (*state).resources_dropped.wrapping_add(map.dropped);
    let mut index: size_t = 0 as size_t;
    while index < map.count {
        let mut entry: *const linux_device_map_entry_t =
            (&raw mut map.entries as *mut linux_device_map_entry_t).offset(index as isize);
        if flash_files_present(
            root,
            &raw const (*entry).sysfs_name as *const ::core::ffi::c_char,
        ) {
            add_resource(
                state,
                linux_hardware_resource_kind_t::LINUX_HARDWARE_RESOURCE_FLASH,
                &raw const (*entry).sysfs_name as *const ::core::ffi::c_char,
                &raw const (*entry).public_id as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_uint,
            );
        }
        index = index.wrapping_add(1);
    }
}
unsafe extern "C" fn discover_edac(
    mut state: *mut linux_hardware_state_t,
    mut root: *const ::core::ffi::c_char,
    mut capability: *mut system_health_capability_t,
) {
    let mut path: [::core::ffi::c_char; 1200] = [0; 1200];
    snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
        b"%s/devices/system/edac/mc\0".as_ptr() as *const ::core::ffi::c_char,
        root,
    );
    let mut directory: *mut DIR = opendir(&raw mut path as *mut ::core::ffi::c_char);
    if directory.is_null() {
        *capability = merge_capability(*capability, capability_from_errno(*__errno_location()));
        return;
    }
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut scanned: size_t = 0 as size_t;
    loop {
        entry = readdir(directory);
        if entry.is_null() {
            break;
        }
        scanned = scanned.wrapping_add(1);
        if scanned > HARDWARE_SCAN_MAX as size_t {
            (*state).resources_dropped = (*state).resources_dropped.wrapping_add(1);
            break;
        } else {
            let mut index: ::core::ffi::c_uint = 0;
            if !suffix_number(
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                b"mc\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut index,
            ) {
                continue;
            }
            let mut public_id: [::core::ffi::c_char; 64] = [0; 64];
            snprintf(
                &raw mut public_id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"edac.mc%u\0".as_ptr() as *const ::core::ffi::c_char,
                index,
            );
            add_resource(
                state,
                linux_hardware_resource_kind_t::LINUX_HARDWARE_RESOURCE_EDAC,
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                &raw mut public_id as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_uint,
            );
        }
    }
    closedir(directory);
}
unsafe extern "C" fn hwmon_has_board_flag(
    mut root: *const ::core::ffi::c_char,
    mut hwmon: *const ::core::ffi::c_char,
) -> bool {
    let mut path: [::core::ffi::c_char; 1200] = [0; 1200];
    snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
        b"%s/class/hwmon/%s\0".as_ptr() as *const ::core::ffi::c_char,
        root,
        hwmon,
    );
    let mut directory: *mut DIR = opendir(&raw mut path as *mut ::core::ffi::c_char);
    if directory.is_null() {
        return r#false != 0;
    }
    let mut found: bool = r#false != 0;
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut scanned: size_t = 0 as size_t;
    loop {
        entry = readdir(directory);
        if entry.is_null() {
            break;
        }
        scanned = scanned.wrapping_add(1);
        if scanned > HARDWARE_SCAN_MAX as size_t {
            break;
        }
        let mut name: *const ::core::ffi::c_char =
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char;
        if !((strncmp(
            name,
            b"in\0".as_ptr() as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                name,
                b"power\0".as_ptr() as *const ::core::ffi::c_char,
                5 as size_t,
            ) == 0 as ::core::ffi::c_int
            || strncmp(
                name,
                b"throttle\0".as_ptr() as *const ::core::ffi::c_char,
                8 as size_t,
            ) == 0 as ::core::ffi::c_int
            || strncmp(
                name,
                b"throttled\0".as_ptr() as *const ::core::ffi::c_char,
                9 as size_t,
            ) == 0 as ::core::ffi::c_int)
            && !strstr(name, b"alarm\0".as_ptr() as *const ::core::ffi::c_char).is_null())
        {
            continue;
        }
        found = r#true != 0;
        break;
    }
    closedir(directory);
    return found;
}
unsafe extern "C" fn discover_hwmon(
    mut state: *mut linux_hardware_state_t,
    mut root: *const ::core::ffi::c_char,
    mut capability: *mut system_health_capability_t,
) {
    let mut path: [::core::ffi::c_char; 1200] = [0; 1200];
    snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
        b"%s/class/hwmon\0".as_ptr() as *const ::core::ffi::c_char,
        root,
    );
    let mut directory: *mut DIR = opendir(&raw mut path as *mut ::core::ffi::c_char);
    if directory.is_null() {
        *capability = merge_capability(*capability, capability_from_errno(*__errno_location()));
        return;
    }
    let mut board: bool = r#false != 0;
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut hwmons_scanned: size_t = 0 as size_t;
    loop {
        entry = readdir(directory);
        if entry.is_null() {
            break;
        }
        hwmons_scanned = hwmons_scanned.wrapping_add(1);
        if hwmons_scanned > HARDWARE_SCAN_MAX as size_t {
            (*state).resources_dropped = (*state).resources_dropped.wrapping_add(1);
            break;
        } else {
            if !suffix_number(
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                b"hwmon\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_uint>(),
            ) {
                continue;
            }
            let mut directory_path: [::core::ffi::c_char; 1200] = [0; 1200];
            snprintf(
                &raw mut directory_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
                b"%s/class/hwmon/%s\0".as_ptr() as *const ::core::ffi::c_char,
                root,
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            );
            let mut channels: *mut DIR =
                opendir(&raw mut directory_path as *mut ::core::ffi::c_char);
            if channels.is_null() {
                *capability =
                    merge_capability(*capability, capability_from_errno(*__errno_location()));
            } else {
                let mut channel: *mut dirent = ::core::ptr::null_mut::<dirent>();
                let mut channels_scanned: size_t = 0 as size_t;
                loop {
                    channel = readdir(channels);
                    if channel.is_null() {
                        break;
                    }
                    channels_scanned = channels_scanned.wrapping_add(1);
                    if channels_scanned > HARDWARE_SCAN_MAX as size_t {
                        (*state).resources_dropped = (*state).resources_dropped.wrapping_add(1);
                        break;
                    } else {
                        let mut fan: ::core::ffi::c_uint = 0;
                        if sscanf(
                            &raw mut (*channel).d_name as *mut ::core::ffi::c_char,
                            b"fan%u_input\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut fan,
                        ) != 1 as ::core::ffi::c_int
                            || fan == 0 as ::core::ffi::c_uint
                            || fan > 999 as ::core::ffi::c_uint
                        {
                            continue;
                        }
                        let mut expected: [::core::ffi::c_char; 32] = [0; 32];
                        snprintf(
                            &raw mut expected as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                            b"fan%u_input\0".as_ptr() as *const ::core::ffi::c_char,
                            fan,
                        );
                        if strcmp(
                            &raw mut expected as *mut ::core::ffi::c_char,
                            &raw mut (*channel).d_name as *mut ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int
                        {
                            continue;
                        }
                        let mut public_id: [::core::ffi::c_char; 64] = [0; 64];
                        snprintf(
                            &raw mut public_id as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                            b"fan.%016llx\0".as_ptr() as *const ::core::ffi::c_char,
                            scoped_hash(
                                &raw mut (*state).installation_scope as *mut ::core::ffi::c_char,
                                b"fan\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                                fan,
                            ) as ::core::ffi::c_ulonglong,
                        );
                        add_resource(
                            state,
                            linux_hardware_resource_kind_t::LINUX_HARDWARE_RESOURCE_FAN,
                            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                            &raw mut public_id as *mut ::core::ffi::c_char,
                            fan,
                        );
                    }
                }
                closedir(channels);
                board = board as ::core::ffi::c_int != 0
                    || hwmon_has_board_flag(
                        root,
                        &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                    ) as ::core::ffi::c_int
                        != 0;
            }
        }
    }
    closedir(directory);
    static mut known_flags: [*const ::core::ffi::c_char; 3] = [
        b"/devices/platform/soc/soc:firmware/get_throttled\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/devices/platform/firmware/get_throttled\0".as_ptr() as *const ::core::ffi::c_char,
        b"/firmware/get_throttled\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut info: stat = stat {
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
    let mut known_path: [::core::ffi::c_char; 1200] = [0; 1200];
    let mut index: size_t = 0 as size_t;
    while index
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        snprintf(
            &raw mut known_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
            b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
            root,
            known_flags[index],
        );
        if stat(
            &raw mut known_path as *mut ::core::ffi::c_char,
            &raw mut info,
        ) == 0 as ::core::ffi::c_int
        {
            board = r#true != 0;
        }
        index = index.wrapping_add(1);
    }
    if board {
        add_resource(
            state,
            linux_hardware_resource_kind_t::LINUX_HARDWARE_RESOURCE_BOARD,
            b"board\0".as_ptr() as *const ::core::ffi::c_char,
            b"board\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_uint,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_hardware_state_init(
    mut state: *mut linux_hardware_state_t,
    mut installation_scope: *const ::core::ffi::c_char,
) {
    if state.is_null() {
        return;
    }
    memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_hardware_state_t>(),
    );
    (*state).fan_hot_celsius = 80.0f64;
    (*state).discovery_capability =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    if !installation_scope.is_null()
        && *installation_scope.offset(0isize) as ::core::ffi::c_int != 0
    {
        snprintf(
            &raw mut (*state).installation_scope as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            installation_scope,
        );
    } else {
        event_identity_get_source(
            &raw mut (*state).installation_scope as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        );
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_hardware_discover(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut inventory: *mut system_health_provider_inventory_t,
) -> ::core::ffi::c_int {
    let mut state: *mut linux_hardware_state_t = opaque as *mut linux_hardware_state_t;
    if state.is_null() || context.is_null() || (*context).sys_root.is_null() || inventory.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        inventory as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_provider_inventory_t>(),
    );
    (*state).resource_count = 0 as size_t;
    (*state).resources_dropped = 0 as size_t;
    let mut root_length: ::core::ffi::c_int = snprintf(
        &raw mut (*state).discovered_sys_root as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*context).sys_root,
    );
    let mut capability: system_health_capability_t =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    if root_length < 0 as ::core::ffi::c_int
        || root_length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
        || (*state).installation_scope[0usize] == 0
    {
        (*state).discovery_capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    } else {
        discover_flash(state, (*context).sys_root, &raw mut capability);
        discover_edac(state, (*context).sys_root, &raw mut capability);
        discover_hwmon(state, (*context).sys_root, &raw mut capability);
        if (*state).resource_count > 0 as size_t {
            capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        }
        (*state).discovery_capability = capability;
    }
    qsort(
        &raw mut (*state).resources as *mut linux_hardware_resource_t as *mut ::core::ffi::c_void,
        (*state).resource_count,
        ::core::mem::size_of::<linux_hardware_resource_t>(),
        Some(
            compare_resources
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut counter: size_t = 0 as size_t;
    while counter < LINUX_HARDWARE_COUNTER_SLOTS as size_t {
        let mut slot: *mut linux_hardware_counter_state_t = (&raw mut (*state).counters
            as *mut linux_hardware_counter_state_t)
            .offset(counter as isize);
        if (*slot).used {
            let mut present: bool = r#false != 0;
            let mut resource: size_t = 0 as size_t;
            while resource < (*state).resource_count {
                if strcmp(
                    &raw mut (*slot).public_id as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut (*state).resources as *mut linux_hardware_resource_t)
                        .offset(resource as isize))
                    .public_id as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    present = r#true != 0;
                    break;
                } else {
                    resource = resource.wrapping_add(1);
                }
            }
            if !present {
                (*slot).previous_monotonic_ms = 0 as uint64_t;
            }
        }
        counter = counter.wrapping_add(1);
    }
    let mut index: size_t = 0 as size_t;
    while index < (*state).resource_count {
        snprintf(
            &raw mut (*(&raw mut (*inventory).resources as *mut system_health_provider_resource_t)
                .offset(index as isize))
            .id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut (*state).resources as *mut linux_hardware_resource_t)
                .offset(index as isize))
            .public_id as *mut ::core::ffi::c_char,
        );
        (*inventory).resources[index].scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_DEVICE;
        (*inventory).resources[index].capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        index = index.wrapping_add(1);
    }
    (*inventory).count = (*state).resource_count;
    (*inventory).dropped = (*state).resources_dropped;
    if (*inventory).count == 0 as size_t {
        snprintf(
            &raw mut (*(&raw mut (*inventory).resources as *mut system_health_provider_resource_t)
                .offset(0isize))
            .id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            b"linux_hardware\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*inventory).resources[0usize].scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_DEVICE;
        (*inventory).resources[0usize].capability = (*state).discovery_capability;
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
unsafe extern "C" fn read_first(
    mut root: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut files: *const *const ::core::ffi::c_char,
    mut count: size_t,
    mut output: *mut ::core::ffi::c_char,
    mut capability: *mut system_health_capability_t,
) -> bool {
    *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    let mut path: [::core::ffi::c_char; 1200] = [0; 1200];
    let mut index: size_t = 0 as size_t;
    while index < count {
        snprintf(
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
            b"%s/class/block/%s/device/%s\0".as_ptr() as *const ::core::ffi::c_char,
            root,
            name,
            *files.offset(index as isize),
        );
        let mut current: system_health_capability_t =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        if read_text(
            &raw mut path as *mut ::core::ffi::c_char,
            output,
            &raw mut current,
        ) {
            return r#true != 0;
        }
        *capability = merge_capability(*capability, current);
        index = index.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn parse_byte(
    mut cursor: *mut *const ::core::ffi::c_char,
    mut output: *mut uint8_t,
) -> bool {
    while *(*__ctype_b_loc())
        .offset(**cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        *cursor = (*cursor).offset(1);
    }
    if **cursor == 0
        || **cursor as ::core::ffi::c_int == '-' as ::core::ffi::c_int
        || **cursor as ::core::ffi::c_int == '+' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut value: ::core::ffi::c_ulong = strtoul(*cursor, &raw mut end, 0 as ::core::ffi::c_int);
    if *__errno_location() == ERANGE
        || end == *cursor as *mut ::core::ffi::c_char
        || value > UINT8_MAX as ::core::ffi::c_ulong
    {
        return r#false != 0;
    }
    *cursor = end;
    *output = value as uint8_t;
    return r#true != 0;
}
unsafe extern "C" fn only_whitespace(mut cursor: *const ::core::ffi::c_char) -> bool {
    while *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        cursor = cursor.offset(1);
    }
    return *cursor as ::core::ffi::c_int == '\0' as ::core::ffi::c_int;
}
unsafe extern "C" fn collect_flash(
    mut resource: *const linux_hardware_resource_t,
    mut root: *const ::core::ffi::c_char,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    static mut life_files: [*const ::core::ffi::c_char; 3] = [
        b"life_time\0".as_ptr() as *const ::core::ffi::c_char,
        b"life_time_estimation\0".as_ptr() as *const ::core::ffi::c_char,
        b"life_time_estimation_a\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut pre_files: [*const ::core::ffi::c_char; 2] = [
        b"pre_eol_info\0".as_ptr() as *const ::core::ffi::c_char,
        b"pre_eol\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut text: [::core::ffi::c_char; 128] = [0; 128];
    let mut life_capability: system_health_capability_t =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    let mut life_valid: bool = r#false != 0;
    let mut life_a: uint8_t = 0 as uint8_t;
    let mut life_b: uint8_t = 0 as uint8_t;
    if read_first(
        root,
        &raw const (*resource).internal_name as *const ::core::ffi::c_char,
        &raw const life_files as *const *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
        &raw mut text as *mut ::core::ffi::c_char,
        &raw mut life_capability,
    ) {
        let mut cursor: *const ::core::ffi::c_char = &raw mut text as *mut ::core::ffi::c_char;
        life_valid = parse_byte(&raw mut cursor, &raw mut life_a);
        let mut second: uint8_t = 0;
        if life_valid as ::core::ffi::c_int != 0
            && parse_byte(&raw mut cursor, &raw mut second) as ::core::ffi::c_int != 0
        {
            life_b = second;
        }
        if life_valid as ::core::ffi::c_int != 0 && !only_whitespace(cursor) {
            life_valid = r#false != 0;
        }
        if !life_valid {
            life_capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        }
    }
    let mut b_path: [::core::ffi::c_char; 1200] = [0; 1200];
    snprintf(
        &raw mut b_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
        b"%s/class/block/%s/device/life_time_estimation_b\0".as_ptr() as *const ::core::ffi::c_char,
        root,
        &raw const (*resource).internal_name as *const ::core::ffi::c_char,
    );
    let mut separate_b: hardware_integer_t = read_integer(
        &raw mut b_path as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if separate_b.valid as ::core::ffi::c_int != 0 && separate_b.value <= UINT8_MAX as uint64_t {
        life_b = separate_b.value as uint8_t;
        life_valid = life_valid as ::core::ffi::c_int != 0
            || life_b as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint;
        life_capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    let mut life: uint8_t = (if life_a as ::core::ffi::c_int > life_b as ::core::ffi::c_int {
        life_a as ::core::ffi::c_int
    } else {
        life_b as ::core::ffi::c_int
    }) as uint8_t;
    let mut life_defined: bool = life_valid as ::core::ffi::c_int != 0
        && life as ::core::ffi::c_uint >= 1 as ::core::ffi::c_uint
        && life as ::core::ffi::c_uint <= 11 as ::core::ffi::c_uint;
    emit(
        sink,
        context,
        b"storage.device.life_used_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).public_id as *const ::core::ffi::c_char,
        system_health_capability_t(if life_valid as ::core::ffi::c_int != 0 && !life_defined {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
        } else {
            life_capability.0
        }),
        life_defined,
        if life as ::core::ffi::c_uint >= 10 as ::core::ffi::c_uint {
            1.0f64
        } else {
            life as ::core::ffi::c_double / 10.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
    );
    let mut pre_capability: system_health_capability_t =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    let mut pre_valid: bool = r#false != 0;
    let mut pre_eol: uint8_t = 0 as uint8_t;
    if read_first(
        root,
        &raw const (*resource).internal_name as *const ::core::ffi::c_char,
        &raw const pre_files as *const *const ::core::ffi::c_char,
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
        &raw mut text as *mut ::core::ffi::c_char,
        &raw mut pre_capability,
    ) {
        let mut cursor_0: *const ::core::ffi::c_char = &raw mut text as *mut ::core::ffi::c_char;
        pre_valid = parse_byte(&raw mut cursor_0, &raw mut pre_eol) as ::core::ffi::c_int != 0
            && only_whitespace(cursor_0) as ::core::ffi::c_int != 0
            && pre_eol as ::core::ffi::c_uint >= 1 as ::core::ffi::c_uint
            && pre_eol as ::core::ffi::c_uint <= 3 as ::core::ffi::c_uint;
        if !pre_valid {
            pre_capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        }
    }
    emit(
        sink,
        context,
        b"storage.device.pre_eol\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).public_id as *const ::core::ffi::c_char,
        pre_capability,
        pre_valid,
        pre_eol as ::core::ffi::c_double,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
    let mut prefail_valid: bool =
        life_defined as ::core::ffi::c_int != 0 || pre_valid as ::core::ffi::c_int != 0;
    let mut prefail: bool = life_defined as ::core::ffi::c_int != 0
        && life as ::core::ffi::c_uint >= 10 as ::core::ffi::c_uint
        || pre_valid as ::core::ffi::c_int != 0
            && pre_eol as ::core::ffi::c_uint >= 2 as ::core::ffi::c_uint;
    let mut prefail_capability: system_health_capability_t = merge_capability(
        system_health_capability_t(if life_defined as ::core::ffi::c_int != 0 {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        } else {
            life_capability.0
        }),
        system_health_capability_t(if pre_valid as ::core::ffi::c_int != 0 {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        } else {
            pre_capability.0
        }),
    );
    emit(
        sink,
        context,
        b"storage.device.prefail\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).public_id as *const ::core::ffi::c_char,
        prefail_capability,
        prefail_valid,
        if prefail as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    let mut critical: bool = life_defined as ::core::ffi::c_int != 0
        && life as ::core::ffi::c_uint >= 11 as ::core::ffi::c_uint
        || pre_valid as ::core::ffi::c_int != 0
            && pre_eol as ::core::ffi::c_uint >= 3 as ::core::ffi::c_uint;
    emit(
        sink,
        context,
        b"storage.device.critical\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).public_id as *const ::core::ffi::c_char,
        prefail_capability,
        prefail_valid,
        if critical as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
}
unsafe extern "C" fn counter_slot(
    mut state: *mut linux_hardware_state_t,
    mut resource: *const ::core::ffi::c_char,
    mut metric: *const ::core::ffi::c_char,
) -> *mut linux_hardware_counter_state_t {
    let mut empty: *mut linux_hardware_counter_state_t =
        ::core::ptr::null_mut::<linux_hardware_counter_state_t>();
    let mut index: size_t = 0 as size_t;
    while index < LINUX_HARDWARE_COUNTER_SLOTS as size_t {
        let mut slot: *mut linux_hardware_counter_state_t = (&raw mut (*state).counters
            as *mut linux_hardware_counter_state_t)
            .offset(index as isize);
        if (*slot).used as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*slot).public_id as *mut ::core::ffi::c_char,
                resource,
            ) == 0 as ::core::ffi::c_int
            && strcmp(&raw mut (*slot).metric as *mut ::core::ffi::c_char, metric)
                == 0 as ::core::ffi::c_int
        {
            return slot;
        }
        if !(*slot).used && empty.is_null() {
            empty = slot;
        }
        index = index.wrapping_add(1);
    }
    if empty.is_null() {
        return ::core::ptr::null_mut::<linux_hardware_counter_state_t>();
    }
    (*empty).used = r#true != 0;
    snprintf(
        &raw mut (*empty).public_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        resource,
    );
    snprintf(
        &raw mut (*empty).metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        metric,
    );
    return empty;
}
unsafe extern "C" fn emit_counter(
    mut state: *mut linux_hardware_state_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
    mut metric: *const ::core::ffi::c_char,
    mut resource: *const ::core::ffi::c_char,
    mut reading: hardware_integer_t,
) {
    let mut slot: *mut linux_hardware_counter_state_t = counter_slot(state, resource, metric);
    if slot.is_null() {
        emit(
            sink,
            context,
            metric,
            resource,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
            r#false != 0,
            0.0f64,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
        return;
    }
    if !reading.valid {
        (*slot).previous_monotonic_ms = 0 as uint64_t;
        emit(
            sink,
            context,
            metric,
            resource,
            reading.capability,
            r#false != 0,
            0.0f64,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
        return;
    }
    let mut valid: bool = (*slot).previous_monotonic_ms > 0 as uint64_t
        && (*context).monotonic_ms > (*slot).previous_monotonic_ms
        && reading.value >= (*slot).previous;
    let mut capability: system_health_capability_t = system_health_capability_t(
        (if (*slot).previous_monotonic_ms == 0 as uint64_t {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE.0 as ::core::ffi::c_int
        } else {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    let mut delta: ::core::ffi::c_double = if valid as ::core::ffi::c_int != 0 {
        reading.value.wrapping_sub((*slot).previous) as ::core::ffi::c_double
    } else {
        0.0f64
    };
    (*slot).previous = reading.value;
    (*slot).previous_monotonic_ms = (*context).monotonic_ms;
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
unsafe extern "C" fn collect_edac(
    mut state: *mut linux_hardware_state_t,
    mut resource: *const linux_hardware_resource_t,
    mut root: *const ::core::ffi::c_char,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut path: [::core::ffi::c_char; 1200] = [0; 1200];
    snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
        b"%s/devices/system/edac/mc/%s/ce_count\0".as_ptr() as *const ::core::ffi::c_char,
        root,
        &raw const (*resource).internal_name as *const ::core::ffi::c_char,
    );
    let mut corrected: hardware_integer_t = read_integer(
        &raw mut path as *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    );
    snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
        b"%s/devices/system/edac/mc/%s/ue_count\0".as_ptr() as *const ::core::ffi::c_char,
        root,
        &raw const (*resource).internal_name as *const ::core::ffi::c_char,
    );
    let mut uncorrectable: hardware_integer_t = read_integer(
        &raw mut path as *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    );
    emit_counter(
        state,
        context,
        sink,
        b"hardware.ecc.corrected_delta\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).public_id as *const ::core::ffi::c_char,
        corrected,
    );
    emit_counter(
        state,
        context,
        sink,
        b"hardware.ecc.uncorrectable_delta\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).public_id as *const ::core::ffi::c_char,
        uncorrectable,
    );
}
unsafe extern "C" fn collect_fan(
    mut state: *const linux_hardware_state_t,
    mut resource: *const linux_hardware_resource_t,
    mut root: *const ::core::ffi::c_char,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut path: [::core::ffi::c_char; 1200] = [0; 1200];
    format_path(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
        b"%s/class/hwmon/%s/fan%u_input\0".as_ptr() as *const ::core::ffi::c_char,
        root,
        &raw const (*resource).internal_name as *const ::core::ffi::c_char,
        (*resource).channel,
    );
    let mut rpm: hardware_integer_t = read_integer(
        &raw mut path as *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    );
    format_path(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
        b"%s/class/hwmon/%s/fan%u_min\0".as_ptr() as *const ::core::ffi::c_char,
        root,
        &raw const (*resource).internal_name as *const ::core::ffi::c_char,
        (*resource).channel,
    );
    let mut minimum: hardware_integer_t = read_integer(
        &raw mut path as *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    );
    emit(
        sink,
        context,
        b"hardware.fan.rpm\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).public_id as *const ::core::ffi::c_char,
        rpm.capability,
        rpm.valid,
        rpm.value as ::core::ffi::c_double,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
    emit(
        sink,
        context,
        b"hardware.fan.minimum_rpm\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).public_id as *const ::core::ffi::c_char,
        minimum.capability,
        minimum.valid,
        minimum.value as ::core::ffi::c_double,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
    let mut temperature_valid: bool = r#false != 0;
    let mut temperature: ::core::ffi::c_double = 0.0f64;
    let mut hot_limit: ::core::ffi::c_double = (*state).fan_hot_celsius;
    let mut channel: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
    while channel <= 32 as ::core::ffi::c_uint {
        format_path(
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
            b"%s/class/hwmon/%s/temp%u_input\0".as_ptr() as *const ::core::ffi::c_char,
            root,
            &raw const (*resource).internal_name as *const ::core::ffi::c_char,
            channel,
        );
        let mut input: hardware_integer_t = read_integer(
            &raw mut path as *mut ::core::ffi::c_char,
            10 as ::core::ffi::c_int,
        );
        if input.valid {
            let mut current: ::core::ffi::c_double =
                input.value as ::core::ffi::c_double / 1000.0f64;
            if !temperature_valid || current > temperature {
                temperature = current;
            }
            temperature_valid = r#true != 0;
            format_path(
                &raw mut path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
                b"%s/class/hwmon/%s/temp%u_max\0".as_ptr() as *const ::core::ffi::c_char,
                root,
                &raw const (*resource).internal_name as *const ::core::ffi::c_char,
                channel,
            );
            let mut maximum: hardware_integer_t = read_integer(
                &raw mut path as *mut ::core::ffi::c_char,
                10 as ::core::ffi::c_int,
            );
            if maximum.valid as ::core::ffi::c_int != 0 && maximum.value > 0 as uint64_t {
                let mut candidate: ::core::ffi::c_double =
                    maximum.value as ::core::ffi::c_double / 1000.0f64;
                if candidate < hot_limit {
                    hot_limit = candidate;
                }
            }
        }
        channel = channel.wrapping_add(1);
    }
    let mut hot: bool = temperature_valid as ::core::ffi::c_int != 0 && temperature >= hot_limit;
    let mut stopped: bool = rpm.valid as ::core::ffi::c_int != 0
        && (rpm.value == 0 as uint64_t
            || minimum.valid as ::core::ffi::c_int != 0
                && minimum.value > 0 as uint64_t
                && rpm.value < minimum.value);
    emit(
        sink,
        context,
        b"hardware.fan.hot\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).public_id as *const ::core::ffi::c_char,
        system_health_capability_t(
            (if temperature_valid as ::core::ffi::c_int != 0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
                    as ::core::ffi::c_int
            } else {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                    as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ),
        temperature_valid,
        if hot as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    let mut failed_capability: system_health_capability_t =
        system_health_capability_t(if rpm.valid as ::core::ffi::c_int != 0 {
            (if temperature_valid as ::core::ffi::c_int != 0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
                    as ::core::ffi::c_int
            } else {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                    as ::core::ffi::c_int
            }) as ::core::ffi::c_uint
        } else {
            rpm.capability.0
        });
    emit(
        sink,
        context,
        b"hardware.fan.failed\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).public_id as *const ::core::ffi::c_char,
        failed_capability,
        rpm.valid as ::core::ffi::c_int != 0 && temperature_valid as ::core::ffi::c_int != 0,
        if stopped as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
}
unsafe extern "C" fn collect_board(
    mut resource: *const linux_hardware_resource_t,
    mut root: *const ::core::ffi::c_char,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut capability: system_health_capability_t =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    let mut valid: bool = r#false != 0;
    let mut power: bool = r#false != 0;
    let mut throttled: bool = r#false != 0;
    static mut known_flags: [*const ::core::ffi::c_char; 3] = [
        b"/devices/platform/soc/soc:firmware/get_throttled\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/devices/platform/firmware/get_throttled\0".as_ptr() as *const ::core::ffi::c_char,
        b"/firmware/get_throttled\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut path: [::core::ffi::c_char; 1200] = [0; 1200];
    let mut index: size_t = 0 as size_t;
    while index
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        snprintf(
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
            b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
            root,
            known_flags[index],
        );
        let mut value: hardware_integer_t = read_integer(
            &raw mut path as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        capability = merge_capability(capability, value.capability);
        if value.valid {
            valid = r#true != 0;
            power = power as ::core::ffi::c_int != 0
                || value.value & (1 as uint64_t | (1 as uint64_t) << 16 as ::core::ffi::c_int) != 0;
            throttled = throttled as ::core::ffi::c_int != 0
                || value.value
                    & ((1 as uint64_t) << 2 as ::core::ffi::c_int
                        | (1 as uint64_t) << 18 as ::core::ffi::c_int)
                    != 0;
        }
        index = index.wrapping_add(1);
    }
    let mut hwmon_root: [::core::ffi::c_char; 1200] = [0; 1200];
    snprintf(
        &raw mut hwmon_root as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
        b"%s/class/hwmon\0".as_ptr() as *const ::core::ffi::c_char,
        root,
    );
    let mut hwmons: *mut DIR = opendir(&raw mut hwmon_root as *mut ::core::ffi::c_char);
    if !hwmons.is_null() {
        let mut hwmon: *mut dirent = ::core::ptr::null_mut::<dirent>();
        let mut hwmons_scanned: size_t = 0 as size_t;
        loop {
            hwmon = readdir(hwmons);
            if hwmon.is_null() {
                break;
            }
            hwmons_scanned = hwmons_scanned.wrapping_add(1);
            if hwmons_scanned > HARDWARE_SCAN_MAX as size_t {
                break;
            }
            let mut ignored: ::core::ffi::c_uint = 0;
            if !suffix_number(
                &raw mut (*hwmon).d_name as *mut ::core::ffi::c_char,
                b"hwmon\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut ignored,
            ) {
                continue;
            }
            let mut directory_path: [::core::ffi::c_char; 1200] = [0; 1200];
            if !join_path(
                &raw mut directory_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
                &raw mut hwmon_root as *mut ::core::ffi::c_char,
                &raw mut (*hwmon).d_name as *mut ::core::ffi::c_char,
            ) {
                continue;
            }
            let mut entries: *mut DIR =
                opendir(&raw mut directory_path as *mut ::core::ffi::c_char);
            if entries.is_null() {
                continue;
            }
            let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
            let mut entries_scanned: size_t = 0 as size_t;
            loop {
                entry = readdir(entries);
                if entry.is_null() {
                    break;
                }
                entries_scanned = entries_scanned.wrapping_add(1);
                if entries_scanned > HARDWARE_SCAN_MAX as size_t {
                    break;
                }
                let mut name: *const ::core::ffi::c_char =
                    &raw mut (*entry).d_name as *mut ::core::ffi::c_char;
                let mut power_alarm: bool = (strncmp(
                    name,
                    b"in\0".as_ptr() as *const ::core::ffi::c_char,
                    2 as size_t,
                ) == 0 as ::core::ffi::c_int
                    || strncmp(
                        name,
                        b"power\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as size_t,
                    ) == 0 as ::core::ffi::c_int)
                    && !strstr(name, b"alarm\0".as_ptr() as *const ::core::ffi::c_char).is_null();
                let mut throttle_alarm: bool = (strncmp(
                    name,
                    b"throttle\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
                    || strncmp(
                        name,
                        b"throttled\0".as_ptr() as *const ::core::ffi::c_char,
                        9 as size_t,
                    ) == 0 as ::core::ffi::c_int)
                    && !strstr(name, b"alarm\0".as_ptr() as *const ::core::ffi::c_char).is_null();
                if !power_alarm && !throttle_alarm {
                    continue;
                }
                if !join_path(
                    &raw mut path as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
                    &raw mut directory_path as *mut ::core::ffi::c_char,
                    name,
                ) {
                    continue;
                }
                let mut value_0: hardware_integer_t = read_integer(
                    &raw mut path as *mut ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                capability = merge_capability(capability, value_0.capability);
                if !value_0.valid {
                    continue;
                }
                valid = r#true != 0;
                if power_alarm {
                    power = power as ::core::ffi::c_int != 0 || value_0.value != 0 as uint64_t;
                }
                if throttle_alarm {
                    throttled =
                        throttled as ::core::ffi::c_int != 0 || value_0.value != 0 as uint64_t;
                }
            }
            closedir(entries);
        }
        closedir(hwmons);
    }
    emit(
        sink,
        context,
        b"hardware.power.unstable\0".as_ptr() as *const ::core::ffi::c_char,
        b"board\0".as_ptr() as *const ::core::ffi::c_char,
        capability,
        valid,
        if power as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    emit(
        sink,
        context,
        b"hardware.throttled\0".as_ptr() as *const ::core::ffi::c_char,
        b"board\0".as_ptr() as *const ::core::ffi::c_char,
        capability,
        valid,
        if throttled as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_hardware_collect(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    let mut state: *mut linux_hardware_state_t = opaque as *mut linux_hardware_state_t;
    if state.is_null() || context.is_null() || (*context).sys_root.is_null() || sink.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    emit(
        sink,
        context,
        b"hardware.provider.visible\0".as_ptr() as *const ::core::ffi::c_char,
        b"linux_hardware\0".as_ptr() as *const ::core::ffi::c_char,
        (*state).discovery_capability,
        (*state).discovery_capability.0
            == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0,
        1.0f64,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    let mut index: size_t = 0 as size_t;
    while index < (*state).resource_count {
        let mut resource: *const linux_hardware_resource_t =
            (&raw mut (*state).resources as *mut linux_hardware_resource_t).offset(index as isize);
        match (*resource).kind {
            linux_hardware_resource_kind_t::LINUX_HARDWARE_RESOURCE_FLASH => {
                collect_flash(resource, (*context).sys_root, context, sink);
            }
            linux_hardware_resource_kind_t::LINUX_HARDWARE_RESOURCE_EDAC => {
                collect_edac(state, resource, (*context).sys_root, context, sink);
            }
            linux_hardware_resource_kind_t::LINUX_HARDWARE_RESOURCE_FAN => {
                collect_fan(state, resource, (*context).sys_root, context, sink);
            }
            linux_hardware_resource_kind_t::LINUX_HARDWARE_RESOURCE_BOARD => {
                collect_board(resource, (*context).sys_root, context, sink);
            }
            _ => {}
        }
        index = index.wrapping_add(1);
    }
    (*sink).dropped = (*sink).dropped.wrapping_add((*state).resources_dropped);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_hardware_provider_init(
    mut provider: *mut system_health_provider_t,
    mut state: *mut linux_hardware_state_t,
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
        b"linux_hardware\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*provider).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    (*provider).state = state as *mut ::core::ffi::c_void;
    (*provider).discover = Some(
        linux_hardware_discover
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_provider_inventory_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_provider_discover_fn;
    (*provider).collect = Some(
        linux_hardware_collect
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_observation_sink_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_collect_fn;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
