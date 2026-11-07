unsafe extern "C" {
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
    unsafe fn linux_device_map_find(
        map: *const linux_device_map_t,
        physical_name: *const ::core::ffi::c_char,
    ) -> *const linux_device_map_entry_t;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn ioctl(
        __fd: ::core::ffi::c_int,
        __request: ::core::ffi::c_ulong,
        ...
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = usize;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type int64_t = i64;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type uintptr_t = usize;
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
pub type nvme_provider_open_fn = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, ::core::ffi::c_int) -> ::core::ffi::c_int,
>;
pub type nvme_provider_health_log_fn =
    Option<unsafe extern "C" fn(::core::ffi::c_int, *mut uint8_t, size_t) -> ::core::ffi::c_int>;
pub type nvme_provider_close_fn =
    Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nvme_provider_ops_t {
    pub open_device: nvme_provider_open_fn,
    pub read_health_log: nvme_provider_health_log_fn,
    pub close_device: nvme_provider_close_fn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nvme_provider_device_t {
    pub internal_name: [::core::ffi::c_char; 64],
    pub public_id: [::core::ffi::c_char; 64],
    pub previous_media_errors: uint64_t,
    pub previous_unsafe_shutdowns: uint64_t,
    pub previous_monotonic_ms: uint64_t,
    pub media_previous_valid: bool,
    pub unsafe_previous_valid: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nvme_provider_state_t {
    pub devices: [nvme_provider_device_t; 16],
    pub device_count: size_t,
    pub devices_dropped: size_t,
    pub discovery_capability: system_health_capability_t,
    pub installation_scope: [::core::ffi::c_char; 96],
    pub dev_root: [::core::ffi::c_char; 256],
    pub discovered_sys_root: [::core::ffi::c_char; 1024],
    pub ops: nvme_provider_ops_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nvme_passthru_cmd {
    pub opcode: __u8,
    pub flags: __u8,
    pub rsvd1: __u16,
    pub nsid: __u32,
    pub cdw2: __u32,
    pub cdw3: __u32,
    pub metadata: __u64,
    pub addr: __u64,
    pub metadata_len: __u32,
    pub data_len: __u32,
    pub cdw10: __u32,
    pub cdw11: __u32,
    pub cdw12: __u32,
    pub cdw13: __u32,
    pub cdw14: __u32,
    pub cdw15: __u32,
    pub timeout_ms: __u32,
    pub result: __u32,
}
pub type __u32 = ::core::ffi::c_uint;
pub type __u64 = ::core::ffi::c_ulonglong;
pub type __u16 = ::core::ffi::c_ushort;
pub type __u8 = ::core::ffi::c_uchar;
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
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const SYSTEM_HEALTH_MAX_DEVICES: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const LINUX_HARDWARE_INTERNAL_NAME_LENGTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const NVME_PROVIDER_MAX_DEVICES: ::core::ffi::c_uint = SYSTEM_HEALTH_MAX_DEVICES;
pub const NVME_PROVIDER_HEALTH_LOG_SIZE: ::core::ffi::c_uint = 512 as ::core::ffi::c_uint;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENODEV: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ENOTTY: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const _IOC_NRBITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const _IOC_TYPEBITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const _IOC_SIZEBITS: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const _IOC_NRSHIFT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _IOC_TYPESHIFT: ::core::ffi::c_int = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_SIZESHIFT: ::core::ffi::c_int = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT: ::core::ffi::c_int = _IOC_SIZESHIFT + _IOC_SIZEBITS;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const NVME_IOCTL_ADMIN_CMD: usize =
    ((2 as ::core::ffi::c_uint | 1 as ::core::ffi::c_uint) << _IOC_DIRSHIFT
        | (('N' as ::core::ffi::c_int) << _IOC_TYPESHIFT) as ::core::ffi::c_uint
        | ((0x41 as ::core::ffi::c_int) << _IOC_NRSHIFT) as ::core::ffi::c_uint) as usize
        | ::core::mem::size_of::<nvme_passthru_cmd>() << _IOC_SIZESHIFT;
pub const NVME_SCAN_MAX: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
unsafe extern "C" fn capability_from_errno(
    mut error_number: ::core::ffi::c_int,
) -> system_health_capability_t {
    if error_number == EACCES || error_number == EPERM {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if error_number == ENOENT
        || error_number == ENODEV
        || error_number == ENOTDIR
        || error_number == ENOTSUP
        || error_number == ENOTTY
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
}
unsafe extern "C" fn default_open(
    mut path: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return open(path, flags);
}
unsafe extern "C" fn default_close(mut descriptor: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return close(descriptor);
}
unsafe extern "C" fn default_health_log(
    mut descriptor: ::core::ffi::c_int,
    mut output: *mut uint8_t,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    if output.is_null() || output_size != NVME_PROVIDER_HEALTH_LOG_SIZE as size_t {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
    let mut command: nvme_passthru_cmd = nvme_passthru_cmd {
        opcode: 0,
        flags: 0,
        rsvd1: 0,
        nsid: 0,
        cdw2: 0,
        cdw3: 0,
        metadata: 0,
        addr: 0,
        metadata_len: 0,
        data_len: 0,
        cdw10: 0,
        cdw11: 0,
        cdw12: 0,
        cdw13: 0,
        cdw14: 0,
        cdw15: 0,
        timeout_ms: 0,
        result: 0,
    };
    memset(
        &raw mut command as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<nvme_passthru_cmd>(),
    );
    command.opcode = 0x2 as __u8;
    command.nsid = UINT32_MAX as __u32;
    command.addr = output.expose_provenance() as uintptr_t as uint64_t as __u64;
    command.data_len = output_size as uint32_t as __u32;
    command.cdw10 = ((output_size as uint32_t)
        .wrapping_div(4 as uint32_t)
        .wrapping_sub(1 as uint32_t)
        << 16 as ::core::ffi::c_uint
        | 0x2 as uint32_t) as __u32;
    return ioctl(
        descriptor,
        NVME_IOCTL_ADMIN_CMD as ::core::ffi::c_ulong,
        &raw mut command,
    );
}
unsafe extern "C" fn controller_name(mut name: *const ::core::ffi::c_char) -> bool {
    if name.is_null()
        || strncmp(
            name,
            b"nvme\0".as_ptr() as *const ::core::ffi::c_char,
            4 as size_t,
        ) != 0 as ::core::ffi::c_int
        || *(*__ctype_b_loc())
            .offset(*name.offset(4isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_char = name.offset(4 as ::core::ffi::c_uint as isize);
    while *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        cursor = cursor.offset(1);
    }
    return *cursor as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        && strlen(name) < LINUX_HARDWARE_INTERNAL_NAME_LENGTH as size_t;
}
unsafe extern "C" fn compare_names(
    mut left: *const ::core::ffi::c_void,
    mut right: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        left as *const ::core::ffi::c_char,
        right as *const ::core::ffi::c_char,
    );
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
pub unsafe extern "C" fn nvme_provider_state_init(
    mut state: *mut nvme_provider_state_t,
    mut installation_scope: *const ::core::ffi::c_char,
) {
    if state.is_null() {
        return;
    }
    memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<nvme_provider_state_t>(),
    );
    (*state).discovery_capability =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    snprintf(
        &raw mut (*state).dev_root as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        b"/dev\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !installation_scope.is_null() {
        snprintf(
            &raw mut (*state).installation_scope as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            installation_scope,
        );
    }
    (*state).ops.open_device = Some(
        default_open
            as unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ) as nvme_provider_open_fn;
    (*state).ops.read_health_log = Some(
        default_health_log
            as unsafe extern "C" fn(::core::ffi::c_int, *mut uint8_t, size_t) -> ::core::ffi::c_int,
    ) as nvme_provider_health_log_fn;
    (*state).ops.close_device =
        Some(default_close as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int)
            as nvme_provider_close_fn;
}
unsafe extern "C" fn old_device(
    mut devices: *const nvme_provider_device_t,
    mut count: size_t,
    mut name: *const ::core::ffi::c_char,
) -> *const nvme_provider_device_t {
    let mut index: size_t = 0 as size_t;
    while index < count {
        if strcmp(
            &raw const (*devices.offset(index as isize)).internal_name
                as *const ::core::ffi::c_char,
            name,
        ) == 0 as ::core::ffi::c_int
        {
            return devices.offset(index as isize);
        }
        index = index.wrapping_add(1);
    }
    return ::core::ptr::null::<nvme_provider_device_t>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nvme_provider_discover(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut inventory: *mut system_health_provider_inventory_t,
) -> ::core::ffi::c_int {
    let mut path: [::core::ffi::c_char; 1200] = [0; 1200];
    let mut written: ::core::ffi::c_int = 0;
    let mut directory: *mut DIR = ::core::ptr::null_mut::<DIR>();
    let mut names: [[::core::ffi::c_char; 64]; 64] = [[0; 64]; 64];
    let mut name_count: size_t = 0;
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
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
    let mut state: *mut nvme_provider_state_t = opaque as *mut nvme_provider_state_t;
    if state.is_null() || context.is_null() || (*context).sys_root.is_null() || inventory.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        inventory as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_provider_inventory_t>(),
    );
    let mut previous: [nvme_provider_device_t; 16] = [nvme_provider_device_t {
        internal_name: [0; 64],
        public_id: [0; 64],
        previous_media_errors: 0,
        previous_unsafe_shutdowns: 0,
        previous_monotonic_ms: 0,
        media_previous_valid: false,
        unsafe_previous_valid: false,
    }; 16];
    let mut previous_count: size_t = (*state).device_count;
    memcpy(
        &raw mut previous as *mut nvme_provider_device_t as *mut ::core::ffi::c_void,
        &raw mut (*state).devices as *mut nvme_provider_device_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[nvme_provider_device_t; 16]>(),
    );
    memset(
        &raw mut (*state).devices as *mut nvme_provider_device_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[nvme_provider_device_t; 16]>(),
    );
    (*state).device_count = 0 as size_t;
    (*state).devices_dropped = 0 as size_t;
    let mut root_length: ::core::ffi::c_int = snprintf(
        &raw mut (*state).discovered_sys_root as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*context).sys_root,
    );
    if root_length < 0 as ::core::ffi::c_int
        || root_length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
        || (*state).installation_scope[0usize] == 0
    {
        (*state).discovery_capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    } else {
        path = [0; 1200];
        written = snprintf(
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
            b"%s/class/nvme\0".as_ptr() as *const ::core::ffi::c_char,
            (*context).sys_root,
        );
        if written < 0 as ::core::ffi::c_int
            || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 1200]>()
        {
            (*state).discovery_capability =
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        } else {
            directory = opendir(&raw mut path as *mut ::core::ffi::c_char);
            if directory.is_null() {
                (*state).discovery_capability = capability_from_errno(*__errno_location());
            } else {
                names = [[0; 64]; 64];
                name_count = 0 as size_t;
                entry = ::core::ptr::null_mut::<dirent>();
                loop {
                    entry = readdir(directory);
                    if entry.is_null() {
                        break;
                    }
                    if !controller_name(&raw mut (*entry).d_name as *mut ::core::ffi::c_char) {
                        continue;
                    }
                    if name_count >= NVME_SCAN_MAX as size_t {
                        (*state).devices_dropped = (*state).devices_dropped.wrapping_add(1);
                        break;
                    } else {
                        let mut length: size_t =
                            strlen(&raw mut (*entry).d_name as *mut ::core::ffi::c_char);
                        memcpy(
                            &raw mut *(&raw mut names as *mut [::core::ffi::c_char; 64])
                                .offset(name_count as isize)
                                as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_void,
                            &raw mut (*entry).d_name as *mut ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            length.wrapping_add(1 as size_t),
                        );
                        name_count = name_count.wrapping_add(1);
                    }
                }
                closedir(directory);
                qsort(
                    &raw mut names as *mut [::core::ffi::c_char; 64] as *mut ::core::ffi::c_void,
                    name_count,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    Some(
                        compare_names
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_void,
                                *const ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
                map = linux_device_map_t {
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
                linux_device_map_build(
                    (*context).sys_root,
                    &raw mut (*state).installation_scope as *mut ::core::ffi::c_char,
                    &raw mut map,
                );
                let mut index: size_t = 0 as size_t;
                while index < name_count {
                    if (*state).device_count >= NVME_PROVIDER_MAX_DEVICES as size_t {
                        (*state).devices_dropped = (*state).devices_dropped.wrapping_add(1);
                    } else {
                        let c2rust_fresh0 = (*state).device_count;
                        (*state).device_count = (*state).device_count.wrapping_add(1);
                        let mut device: *mut nvme_provider_device_t = (&raw mut (*state).devices
                            as *mut nvme_provider_device_t)
                            .offset(c2rust_fresh0 as isize);
                        let mut internal_length: size_t = strlen(
                            &raw mut *(&raw mut names as *mut [::core::ffi::c_char; 64])
                                .offset(index as isize)
                                as *mut ::core::ffi::c_char,
                        );
                        memcpy(
                            &raw mut (*device).internal_name as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_void,
                            &raw mut *(&raw mut names as *mut [::core::ffi::c_char; 64])
                                .offset(index as isize)
                                as *mut ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            internal_length.wrapping_add(1 as size_t),
                        );
                        let mut mapped: *const linux_device_map_entry_t = if map.capability.0
                            == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
                        {
                            linux_device_map_find(
                                &raw mut map,
                                &raw mut *(&raw mut names as *mut [::core::ffi::c_char; 64])
                                    .offset(index as isize)
                                    as *mut ::core::ffi::c_char,
                            )
                        } else {
                            ::core::ptr::null::<linux_device_map_entry_t>()
                        };
                        if !mapped.is_null() {
                            snprintf(
                                &raw mut (*device).public_id as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw const (*mapped).public_id as *const ::core::ffi::c_char,
                            );
                        } else {
                            snprintf(
                                &raw mut (*device).public_id as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                                b"device.%016llx\0".as_ptr() as *const ::core::ffi::c_char,
                                scoped_hash(
                                    &raw mut (*state).installation_scope
                                        as *mut ::core::ffi::c_char,
                                    &raw mut *(&raw mut names as *mut [::core::ffi::c_char; 64])
                                        .offset(index as isize)
                                        as *mut ::core::ffi::c_char,
                                ) as ::core::ffi::c_ulonglong,
                            );
                        }
                        let mut old: *const nvme_provider_device_t = old_device(
                            &raw mut previous as *mut nvme_provider_device_t,
                            previous_count,
                            &raw mut (*device).internal_name as *mut ::core::ffi::c_char,
                        );
                        if !old.is_null() {
                            (*device).previous_media_errors = (*old).previous_media_errors;
                            (*device).previous_unsafe_shutdowns = (*old).previous_unsafe_shutdowns;
                            (*device).previous_monotonic_ms = (*old).previous_monotonic_ms;
                            (*device).media_previous_valid = (*old).media_previous_valid;
                            (*device).unsafe_previous_valid = (*old).unsafe_previous_valid;
                        }
                    }
                    index = index.wrapping_add(1);
                }
                (*state).devices_dropped = (*state).devices_dropped.wrapping_add(map.dropped);
                (*state).discovery_capability = system_health_capability_t(
                    (if (*state).device_count > 0 as size_t {
                        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
                            as ::core::ffi::c_int
                    } else {
                        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                            as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                );
            }
        }
    }
    let mut index_0: size_t = 0 as size_t;
    while index_0 < (*state).device_count {
        snprintf(
            &raw mut (*(&raw mut (*inventory).resources as *mut system_health_provider_resource_t)
                .offset(index_0 as isize))
            .id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut (*state).devices as *mut nvme_provider_device_t)
                .offset(index_0 as isize))
            .public_id as *mut ::core::ffi::c_char,
        );
        (*inventory).resources[index_0].scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_DEVICE;
        (*inventory).resources[index_0].capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        index_0 = index_0.wrapping_add(1);
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
            b"nvme\0".as_ptr() as *const ::core::ffi::c_char,
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
unsafe extern "C" fn little_u16(mut value: *const uint8_t) -> uint16_t {
    return (*value.offset(0isize) as uint16_t as ::core::ffi::c_int
        | ((*value.offset(1isize) as uint16_t as ::core::ffi::c_int) << 8 as ::core::ffi::c_uint)
            as uint16_t as ::core::ffi::c_int) as uint16_t;
}
unsafe extern "C" fn little_u128_as_u64(
    mut value: *const uint8_t,
    mut output: *mut uint64_t,
) -> bool {
    let mut index: size_t = 8 as size_t;
    while index < 16 as size_t {
        if *value.offset(index as isize) as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
            return r#false != 0;
        }
        index = index.wrapping_add(1);
    }
    let mut parsed: uint64_t = 0 as uint64_t;
    let mut index_0: size_t = 0 as size_t;
    while index_0 < 8 as size_t {
        parsed |=
            (*value.offset(index_0 as isize) as uint64_t) << index_0.wrapping_mul(8 as size_t);
        index_0 = index_0.wrapping_add(1);
    }
    *output = parsed;
    return r#true != 0;
}
unsafe extern "C" fn emit_unavailable_device(
    mut device: *const nvme_provider_device_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
    mut capability: system_health_capability_t,
) {
    static mut metrics: [*const ::core::ffi::c_char; 7] = [
        b"storage.device.available_spare_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.percentage_used_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.temperature_celsius\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.media_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.unsafe_shutdowns_delta\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.prefail\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage.device.critical\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut units: [system_health_unit_t; 7] = [
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_CELSIUS,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    ];
    let mut index: size_t = 0 as size_t;
    while index
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 7]>()
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
            units[index],
        );
        index = index.wrapping_add(1);
    }
}
unsafe extern "C" fn emit_delta(
    mut device: *mut nvme_provider_device_t,
    mut current: uint64_t,
    mut previous_valid: *mut bool,
    mut previous: *mut uint64_t,
    mut metric: *const ::core::ffi::c_char,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut valid: bool = *previous_valid as ::core::ffi::c_int != 0
        && (*context).monotonic_ms > (*device).previous_monotonic_ms
        && current >= *previous;
    let mut capability: system_health_capability_t = system_health_capability_t(
        (if !*previous_valid {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE.0 as ::core::ffi::c_int
        } else {
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    let mut delta: ::core::ffi::c_double = if valid as ::core::ffi::c_int != 0 {
        current.wrapping_sub(*previous) as ::core::ffi::c_double
    } else {
        0.0f64
    };
    *previous = current;
    *previous_valid = r#true != 0;
    emit(
        sink,
        context,
        metric,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        capability,
        valid,
        delta,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
}
unsafe extern "C" fn collect_device(
    mut state: *mut nvme_provider_state_t,
    mut device: *mut nvme_provider_device_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut path: [::core::ffi::c_char; 512] = [0; 512];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*state).dev_root as *mut ::core::ffi::c_char,
        &raw mut (*device).internal_name as *mut ::core::ffi::c_char,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        emit_unavailable_device(
            device,
            context,
            sink,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
        return;
    }
    let mut descriptor: ::core::ffi::c_int =
        (*state).ops.open_device.expect("non-null function pointer")(
            &raw mut path as *mut ::core::ffi::c_char,
            O_RDONLY | O_NONBLOCK | O_CLOEXEC,
        );
    if descriptor < 0 as ::core::ffi::c_int {
        emit_unavailable_device(
            device,
            context,
            sink,
            capability_from_errno(*__errno_location()),
        );
        (*device).media_previous_valid = r#false != 0;
        (*device).unsafe_previous_valid = r#false != 0;
        return;
    }
    let mut log: [uint8_t; 512] = [0; 512];
    memset(
        &raw mut log as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 512]>(),
    );
    let mut result: ::core::ffi::c_int = (*state)
        .ops
        .read_health_log
        .expect("non-null function pointer")(
        descriptor,
        &raw mut log as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 512]>(),
    );
    let mut saved: ::core::ffi::c_int = *__errno_location();
    (*state)
        .ops
        .close_device
        .expect("non-null function pointer")(descriptor);
    if result != 0 as ::core::ffi::c_int {
        emit_unavailable_device(device, context, sink, capability_from_errno(saved));
        (*device).media_previous_valid = r#false != 0;
        (*device).unsafe_previous_valid = r#false != 0;
        return;
    }
    let mut spare_valid: bool = log[3usize] as ::core::ffi::c_uint <= 100 as ::core::ffi::c_uint;
    let mut spare_ratio: ::core::ffi::c_double = log[3usize] as ::core::ffi::c_double / 100.0f64;
    let mut used_ratio: ::core::ffi::c_double = log[5usize] as ::core::ffi::c_double / 100.0f64;
    if used_ratio > 1.0f64 {
        used_ratio = 1.0f64;
    }
    emit(
        sink,
        context,
        b"storage.device.available_spare_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        system_health_capability_t(
            (if spare_valid as ::core::ffi::c_int != 0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
                    as ::core::ffi::c_int
            } else {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ),
        spare_valid,
        spare_ratio,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
    );
    emit(
        sink,
        context,
        b"storage.device.percentage_used_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        r#true != 0,
        used_ratio,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
    );
    let mut kelvin: uint16_t =
        little_u16((&raw mut log as *mut uint8_t).offset(1 as ::core::ffi::c_uint as isize));
    let mut temperature_valid: bool = kelvin as ::core::ffi::c_uint >= 173 as ::core::ffi::c_uint
        && kelvin as ::core::ffi::c_uint <= 573 as ::core::ffi::c_uint;
    emit(
        sink,
        context,
        b"storage.device.temperature_celsius\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        system_health_capability_t(
            (if temperature_valid as ::core::ffi::c_int != 0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
                    as ::core::ffi::c_int
            } else {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ),
        temperature_valid,
        kelvin as ::core::ffi::c_double - 273.15f64,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_CELSIUS,
    );
    let mut critical: bool = log[0usize] as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint;
    let mut prefail: bool = critical as ::core::ffi::c_int != 0
        || log[4usize] as ::core::ffi::c_uint > 0 as ::core::ffi::c_uint
            && (log[3usize] as ::core::ffi::c_int) < log[4usize] as ::core::ffi::c_int
        || log[5usize] as ::core::ffi::c_uint >= 90 as ::core::ffi::c_uint;
    emit(
        sink,
        context,
        b"storage.device.prefail\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device).public_id as *mut ::core::ffi::c_char,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        r#true != 0,
        if prefail as ::core::ffi::c_int != 0 {
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
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        r#true != 0,
        if critical as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    let mut current: uint64_t = 0 as uint64_t;
    if little_u128_as_u64(
        (&raw mut log as *mut uint8_t).offset(160 as ::core::ffi::c_uint as isize),
        &raw mut current,
    ) {
        emit_delta(
            device,
            current,
            &raw mut (*device).media_previous_valid,
            &raw mut (*device).previous_media_errors,
            b"storage.device.media_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
            context,
            sink,
        );
    } else {
        (*device).media_previous_valid = r#false != 0;
        emit(
            sink,
            context,
            b"storage.device.media_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*device).public_id as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
            r#false != 0,
            0.0f64,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
    }
    if little_u128_as_u64(
        (&raw mut log as *mut uint8_t).offset(144 as ::core::ffi::c_uint as isize),
        &raw mut current,
    ) {
        emit_delta(
            device,
            current,
            &raw mut (*device).unsafe_previous_valid,
            &raw mut (*device).previous_unsafe_shutdowns,
            b"storage.device.unsafe_shutdowns_delta\0".as_ptr() as *const ::core::ffi::c_char,
            context,
            sink,
        );
    } else {
        (*device).unsafe_previous_valid = r#false != 0;
        emit(
            sink,
            context,
            b"storage.device.unsafe_shutdowns_delta\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*device).public_id as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
            r#false != 0,
            0.0f64,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
    }
    (*device).previous_monotonic_ms = (*context).monotonic_ms;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nvme_provider_collect(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    let mut state: *mut nvme_provider_state_t = opaque as *mut nvme_provider_state_t;
    if state.is_null()
        || context.is_null()
        || sink.is_null()
        || (*state).ops.open_device.is_none()
        || (*state).ops.read_health_log.is_none()
        || (*state).ops.close_device.is_none()
    {
        return -1 as ::core::ffi::c_int;
    }
    emit(
        sink,
        context,
        b"hardware.provider.visible\0".as_ptr() as *const ::core::ffi::c_char,
        b"nvme\0".as_ptr() as *const ::core::ffi::c_char,
        (*state).discovery_capability,
        (*state).discovery_capability.0
            == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0,
        1.0f64,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    let mut index: size_t = 0 as size_t;
    while index < (*state).device_count {
        collect_device(
            state,
            (&raw mut (*state).devices as *mut nvme_provider_device_t).offset(index as isize),
            context,
            sink,
        );
        index = index.wrapping_add(1);
    }
    (*sink).dropped = (*sink).dropped.wrapping_add((*state).devices_dropped);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nvme_provider_init(
    mut provider: *mut system_health_provider_t,
    mut state: *mut nvme_provider_state_t,
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
        b"nvme\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*provider).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    (*provider).state = state as *mut ::core::ffi::c_void;
    (*provider).discover = Some(
        nvme_provider_discover
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_provider_inventory_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_provider_discover_fn;
    (*provider).collect = Some(
        nvme_provider_collect
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
