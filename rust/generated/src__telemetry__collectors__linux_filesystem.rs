use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn statvfs(
        __file: *const ::core::ffi::c_char,
        __buf: *mut statvfs,
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
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    unsafe fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub type __fsblkcnt_t = ::core::ffi::c_ulong;
pub type __fsfilcnt_t = ::core::ffi::c_ulong;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type uint32_t = u32;
pub type uint64_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
    pub f_bsize: ::core::ffi::c_ulong,
    pub f_frsize: ::core::ffi::c_ulong,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_favail: __fsfilcnt_t,
    pub f_fsid: ::core::ffi::c_ulong,
    pub f_flag: ::core::ffi::c_ulong,
    pub f_namemax: ::core::ffi::c_ulong,
    pub f_type: ::core::ffi::c_uint,
    pub __f_spare: [::core::ffi::c_int; 5],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const ST_RDONLY: Self = Self(1);
    pub const ST_NOSUID: Self = Self(2);
    pub const ST_NODEV: Self = Self(4);
    pub const ST_NOEXEC: Self = Self(8);
    pub const ST_SYNCHRONOUS: Self = Self(16);
    pub const ST_MANDLOCK: Self = Self(64);
    pub const ST_WRITE: Self = Self(128);
    pub const ST_APPEND: Self = Self(256);
    pub const ST_IMMUTABLE: Self = Self(512);
    pub const ST_NOATIME: Self = Self(1024);
    pub const ST_NODIRATIME: Self = Self(2048);
    pub const ST_RELATIME: Self = Self(4096);
    pub const ST_NOSYMFOLLOW: Self = Self(8192);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_value_t {
    pub value: uint64_t,
    pub capability: system_health_capability_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_flag_t {
    pub value: bool,
    pub capability: system_health_capability_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_resource_t {
    pub logical_id: [::core::ffi::c_char; 64],
    pub path: [::core::ffi::c_char; 1024],
    pub mount_required: bool,
    pub mount_guard_path: [::core::ffi::c_char; 1024],
    pub expected_device_key: [::core::ffi::c_char; 48],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_sample_t {
    pub logical_id: [::core::ffi::c_char; 64],
    pub device_key: [::core::ffi::c_char; 48],
    pub mount_present: linux_filesystem_flag_t,
    pub read_only: linux_filesystem_flag_t,
    pub mount_flags: linux_filesystem_value_t,
    pub capacity_bytes: linux_filesystem_value_t,
    pub available_bytes: linux_filesystem_value_t,
    pub capacity_inodes: linux_filesystem_value_t,
    pub available_inodes: linux_filesystem_value_t,
}
pub type linux_filesystem_stat_fn =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int>;
pub type linux_filesystem_statvfs_fn =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut statvfs) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_ops_t {
    pub stat_path: linux_filesystem_stat_fn,
    pub statvfs_path: linux_filesystem_statvfs_fn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_collector_state_t {
    pub resources: [linux_filesystem_resource_t; 18],
    pub resource_count: size_t,
    pub resources_dropped: size_t,
    pub mountinfo_path: [::core::ffi::c_char; 1024],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct linux_filesystem_probe_error_t(pub ::core::ffi::c_uint);
impl linux_filesystem_probe_error_t {
    pub const LINUX_FILESYSTEM_PROBE_ERROR_NONE: Self = Self(0);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_NOT_FOUND: Self = Self(1);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_PERMISSION: Self = Self(2);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY: Self = Self(3);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_NO_SPACE: Self = Self(4);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_IO: Self = Self(5);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_TIMED_OUT: Self = Self(6);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_BUSY: Self = Self(7);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_INVALID: Self = Self(8);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_OTHER: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const SYSTEM_HEALTH_MAX_FILESYSTEMS: ::core::ffi::c_uint = 18 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_ID_LENGTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const EPERM: ::core::ffi::c_int = 1;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const EIO: ::core::ffi::c_int = 5;
pub const EACCES: ::core::ffi::c_int = 13;
pub const EBUSY: ::core::ffi::c_int = 16;
pub const ENOTDIR: ::core::ffi::c_int = 20;
pub const EINVAL: ::core::ffi::c_int = 22;
pub const ENOSPC: ::core::ffi::c_int = 28;
pub const EROFS: ::core::ffi::c_int = 30;
pub const ENAMETOOLONG: ::core::ffi::c_int = 36;
pub const ENOSYS: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const EOVERFLOW: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110;
pub const EDQUOT: ::core::ffi::c_int = 122;
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> ::core::ffi::c_uint {
    let mut __major: ::core::ffi::c_uint = 0;
    __major = ((__dev & 0xfff00 as ::core::ffi::c_uint as __dev_t) >> 8 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    __major = (__major as __dev_t
        | (__dev & 0xfffff00000000000 as ::core::ffi::c_ulong) >> 32 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    return __major;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> ::core::ffi::c_uint {
    let mut __minor: ::core::ffi::c_uint = 0;
    __minor = ((__dev & 0xff as ::core::ffi::c_uint as __dev_t) >> 0 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    __minor = (__minor as __dev_t
        | (__dev & 0xffffff00000 as ::core::ffi::c_ulong) >> 12 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    return __minor;
}
pub const MOUNTINFO_LINE_MAX: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
pub const MOUNTINFO_ENTRY_MAX: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_filesystem_normalize_errno(
    mut error_code: ::core::ffi::c_int,
) -> linux_filesystem_probe_error_t {
    match error_code {
        0 => return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NONE,
        ENOENT | ENOTDIR => {
            return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NOT_FOUND;
        }
        EACCES | EPERM => {
            return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_PERMISSION;
        }
        EROFS => {
            return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY;
        }
        ENOSPC | EDQUOT => {
            return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NO_SPACE;
        }
        EIO => return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_IO,
        ETIMEDOUT => {
            return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_TIMED_OUT;
        }
        EBUSY => return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_BUSY,
        EINVAL | ENAMETOOLONG => {
            return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_INVALID;
        }
        _ => return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_OTHER,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_filesystem_capability_from_errno(
    mut error_code: ::core::ffi::c_int,
) -> system_health_capability_t {
    if error_code == EACCES || error_code == EPERM {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if error_code == ENOSYS || error_code == ENOTSUP {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_filesystem_logical_id_valid(
    mut logical_id: *const ::core::ffi::c_char,
) -> bool {
    if logical_id.is_null()
        || *logical_id.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut length: size_t = 0 as size_t;
    while *logical_id.offset(length as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut character: ::core::ffi::c_uchar =
            *logical_id.offset(length as isize) as ::core::ffi::c_uchar;
        if length >= SYSTEM_HEALTH_ID_LENGTH.wrapping_sub(1 as ::core::ffi::c_uint) as size_t {
            return r#false != 0;
        }
        if *(*__ctype_b_loc()).offset(character as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed_0::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
            && character as ::core::ffi::c_int != '_' as ::core::ffi::c_int
            && character as ::core::ffi::c_int != '-' as ::core::ffi::c_int
            && character as ::core::ffi::c_int != '.' as ::core::ffi::c_int
            && character as ::core::ffi::c_int != ':' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        length = length.wrapping_add(1);
    }
    return length > 0 as size_t;
}
unsafe extern "C" fn decode_mount_field(
    mut encoded: *const ::core::ffi::c_char,
    mut encoded_length: size_t,
    mut decoded: *mut ::core::ffi::c_char,
    mut decoded_size: size_t,
) -> bool {
    let mut output: size_t = 0 as size_t;
    let mut input: size_t = 0 as size_t;
    while input < encoded_length {
        let mut character: ::core::ffi::c_uchar =
            *encoded.offset(input as isize) as ::core::ffi::c_uchar;
        if character as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
            && input.wrapping_add(3 as size_t) < encoded_length
            && *encoded.offset(input.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                >= '0' as ::core::ffi::c_int
            && *encoded.offset(input.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                <= '7' as ::core::ffi::c_int
            && *encoded.offset(input.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int
                >= '0' as ::core::ffi::c_int
            && *encoded.offset(input.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int
                <= '7' as ::core::ffi::c_int
            && *encoded.offset(input.wrapping_add(3 as size_t) as isize) as ::core::ffi::c_int
                >= '0' as ::core::ffi::c_int
            && *encoded.offset(input.wrapping_add(3 as size_t) as isize) as ::core::ffi::c_int
                <= '7' as ::core::ffi::c_int
        {
            character = ((*encoded.offset(input.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_int
                - '0' as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int
                | (*encoded.offset(input.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int
                    - '0' as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int
                | *encoded.offset(input.wrapping_add(3 as size_t) as isize) as ::core::ffi::c_int
                    - '0' as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            input = input.wrapping_add(3 as size_t);
        }
        if output.wrapping_add(1 as size_t) >= decoded_size
            || character as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        let c2rust_fresh0 = output;
        output = output.wrapping_add(1);
        *decoded.offset(c2rust_fresh0 as isize) = character as ::core::ffi::c_char;
        input = input.wrapping_add(1);
    }
    if output >= decoded_size {
        return r#false != 0;
    }
    *decoded.offset(output as isize) = '\0' as ::core::ffi::c_char;
    return r#true != 0;
}
unsafe extern "C" fn mount_point_field(
    mut line: *const ::core::ffi::c_char,
    mut mount_point: *mut ::core::ffi::c_char,
    mut mount_point_size: size_t,
) -> bool {
    let mut cursor: *const ::core::ffi::c_char = line;
    let mut field: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
    while field <= 5 as ::core::ffi::c_uint {
        while *cursor as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            cursor = cursor.offset(1);
        }
        if *cursor as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        let mut begin: *const ::core::ffi::c_char = cursor;
        while *cursor as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && *cursor as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
            && *cursor as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
        {
            cursor = cursor.offset(1);
        }
        if field == 5 as ::core::ffi::c_uint {
            return decode_mount_field(
                begin,
                cursor.offset_from(begin) as size_t,
                mount_point,
                mount_point_size,
            );
        }
        field = field.wrapping_add(1);
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_filesystem_mountinfo_contains(
    mut mountinfo_path: *const ::core::ffi::c_char,
    mut mount_path: *const ::core::ffi::c_char,
    mut present: *mut bool,
) -> ::core::ffi::c_int {
    if mountinfo_path.is_null()
        || mount_path.is_null()
        || present.is_null()
        || *mount_path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
    *present = r#false != 0;
    let mut input: *mut FILE = fopen(
        mountinfo_path,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if input.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut line: [::core::ffi::c_char; 4098] = [0; 4098];
    let mut entries: size_t = 0 as size_t;
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4098]>() as ::core::ffi::c_int,
        input,
    )
    .is_null()
    {
        let mut length: size_t = strlen(&raw mut line as *mut ::core::ffi::c_char);
        if length > MOUNTINFO_LINE_MAX as size_t
            || length > 0 as size_t
                && line[length.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
                    != '\n' as ::core::ffi::c_int
                && feof(input) == 0
        {
            fclose(input);
            *__errno_location() = EOVERFLOW;
            return -1 as ::core::ffi::c_int;
        }
        entries = entries.wrapping_add(1);
        if entries > MOUNTINFO_ENTRY_MAX as size_t {
            fclose(input);
            *__errno_location() = EOVERFLOW;
            return -1 as ::core::ffi::c_int;
        }
        let mut decoded: [::core::ffi::c_char; 1024] = [0; 1024];
        if !mount_point_field(
            &raw mut line as *mut ::core::ffi::c_char,
            &raw mut decoded as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ) {
            fclose(input);
            *__errno_location() = EINVAL;
            return -1 as ::core::ffi::c_int;
        }
        if strcmp(&raw mut decoded as *mut ::core::ffi::c_char, mount_path)
            == 0 as ::core::ffi::c_int
        {
            *present = r#true != 0;
            fclose(input);
            return 0 as ::core::ffi::c_int;
        }
    }
    if ferror(input) != 0 {
        let mut saved_errno: ::core::ffi::c_int = *__errno_location();
        fclose(input);
        *__errno_location() = saved_errno;
        return -1 as ::core::ffi::c_int;
    }
    fclose(input);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn unavailable_value(
    mut value: *mut linux_filesystem_value_t,
    mut capability: system_health_capability_t,
) {
    (*value).value = 0 as uint64_t;
    (*value).capability = capability;
}
unsafe extern "C" fn unavailable_flag(
    mut flag: *mut linux_filesystem_flag_t,
    mut capability: system_health_capability_t,
) {
    (*flag).value = r#false != 0;
    (*flag).capability = capability;
}
unsafe extern "C" fn make_sample_unavailable(
    mut sample: *mut linux_filesystem_sample_t,
    mut capability: system_health_capability_t,
) {
    unavailable_flag(&raw mut (*sample).mount_present, capability);
    unavailable_flag(&raw mut (*sample).read_only, capability);
    unavailable_value(&raw mut (*sample).mount_flags, capability);
    unavailable_value(&raw mut (*sample).capacity_bytes, capability);
    unavailable_value(&raw mut (*sample).available_bytes, capability);
    unavailable_value(&raw mut (*sample).capacity_inodes, capability);
    unavailable_value(&raw mut (*sample).available_inodes, capability);
}
unsafe extern "C" fn make_capacity_unavailable(
    mut sample: *mut linux_filesystem_sample_t,
    mut capability: system_health_capability_t,
) {
    unavailable_flag(&raw mut (*sample).read_only, capability);
    unavailable_value(&raw mut (*sample).mount_flags, capability);
    unavailable_value(&raw mut (*sample).capacity_bytes, capability);
    unavailable_value(&raw mut (*sample).available_bytes, capability);
    unavailable_value(&raw mut (*sample).capacity_inodes, capability);
    unavailable_value(&raw mut (*sample).available_inodes, capability);
}
unsafe extern "C" fn checked_multiply(
    mut left: uint64_t,
    mut right: uint64_t,
    mut result: *mut uint64_t,
) -> bool {
    if left != 0 as uint64_t && right > (UINT64_MAX as uint64_t).wrapping_div(left) {
        return r#false != 0;
    }
    *result = left.wrapping_mul(right);
    return r#true != 0;
}
unsafe extern "C" fn default_stat_path(
    mut path: *const ::core::ffi::c_char,
    mut info: *mut stat,
) -> ::core::ffi::c_int {
    return stat(path, info);
}
unsafe extern "C" fn default_statvfs_path(
    mut path: *const ::core::ffi::c_char,
    mut info: *mut statvfs,
) -> ::core::ffi::c_int {
    return statvfs(path, info);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_filesystem_sample_with_ops(
    mut resource: *const linux_filesystem_resource_t,
    mut mountinfo_path: *const ::core::ffi::c_char,
    mut ops: *const linux_filesystem_ops_t,
    mut sample: *mut linux_filesystem_sample_t,
) -> ::core::ffi::c_int {
    if resource.is_null()
        || ops.is_null()
        || (*ops).stat_path.is_none()
        || (*ops).statvfs_path.is_none()
        || sample.is_null()
        || !linux_filesystem_logical_id_valid(
            &raw const (*resource).logical_id as *const ::core::ffi::c_char,
        )
        || (*resource).path[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        sample as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_filesystem_sample_t>(),
    );
    snprintf(
        &raw mut (*sample).logical_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).logical_id as *const ::core::ffi::c_char,
    );
    make_sample_unavailable(
        sample,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
    );
    if (*resource).mount_required {
        let mut guard: *const ::core::ffi::c_char =
            if (*resource).mount_guard_path[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*resource).mount_guard_path as *const ::core::ffi::c_char
            } else {
                &raw const (*resource).path as *const ::core::ffi::c_char
            };
        let mut present: bool = r#false != 0;
        if linux_filesystem_mountinfo_contains(mountinfo_path, guard, &raw mut present)
            != 0 as ::core::ffi::c_int
        {
            unavailable_flag(
                &raw mut (*sample).mount_present,
                linux_filesystem_capability_from_errno(*__errno_location()),
            );
            return 0 as ::core::ffi::c_int;
        }
        (*sample).mount_present.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        (*sample).mount_present.value = present;
        if !present {
            return 0 as ::core::ffi::c_int;
        }
    } else {
        (*sample).mount_present.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        (*sample).mount_present.value = r#true != 0;
    }
    let mut path_info: stat = stat {
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
    if (*ops).stat_path.expect("non-null function pointer")(
        &raw const (*resource).path as *const ::core::ffi::c_char,
        &raw mut path_info,
    ) != 0 as ::core::ffi::c_int
    {
        let mut capability: system_health_capability_t =
            linux_filesystem_capability_from_errno(*__errno_location());
        make_capacity_unavailable(sample, capability);
        if *__errno_location() == ENOENT || *__errno_location() == ENOTDIR {
            (*sample).mount_present.capability =
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
            (*sample).mount_present.value = r#false != 0;
        }
        return 0 as ::core::ffi::c_int;
    }
    if !(path_info.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t) {
        make_capacity_unavailable(
            sample,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
        (*sample).mount_present.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        (*sample).mount_present.value = r#false != 0;
        return 0 as ::core::ffi::c_int;
    }
    let mut key_length: ::core::ffi::c_int = snprintf(
        &raw mut (*sample).device_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 48]>(),
        b"linux-block-%u-%u\0".as_ptr() as *const ::core::ffi::c_char,
        gnu_dev_major(path_info.st_dev),
        gnu_dev_minor(path_info.st_dev),
    );
    if key_length < 0 as ::core::ffi::c_int
        || key_length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 48]>()
    {
        (*sample).device_key[0usize] = '\0' as ::core::ffi::c_char;
        make_capacity_unavailable(
            sample,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (*resource).expected_device_key[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && strcmp(
            &raw const (*resource).expected_device_key as *const ::core::ffi::c_char,
            &raw mut (*sample).device_key as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        (*sample).mount_present.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        (*sample).mount_present.value = r#false != 0;
        make_capacity_unavailable(
            sample,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut filesystem: statvfs = statvfs {
        f_bsize: 0,
        f_frsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_favail: 0,
        f_fsid: 0,
        f_flag: 0,
        f_namemax: 0,
        f_type: 0,
        __f_spare: [0; 5],
    };
    if (*ops).statvfs_path.expect("non-null function pointer")(
        &raw const (*resource).path as *const ::core::ffi::c_char,
        &raw mut filesystem,
    ) != 0 as ::core::ffi::c_int
    {
        make_capacity_unavailable(
            sample,
            linux_filesystem_capability_from_errno(*__errno_location()),
        );
        return 0 as ::core::ffi::c_int;
    }
    (*sample).read_only.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    (*sample).mount_flags.value = filesystem.f_flag as uint64_t;
    (*sample).mount_flags.capability =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    (*sample).read_only.value = filesystem.f_flag
        & C2Rust_Unnamed::ST_RDONLY.0 as ::core::ffi::c_int as ::core::ffi::c_ulong
        != 0 as ::core::ffi::c_ulong;
    let mut fragment_size: uint64_t = filesystem.f_frsize as uint64_t;
    let mut capacity: uint64_t = 0 as uint64_t;
    let mut available: uint64_t = 0 as uint64_t;
    if fragment_size == 0 as uint64_t && filesystem.f_blocks != 0 as __fsblkcnt_t
        || !checked_multiply(
            filesystem.f_blocks as uint64_t,
            fragment_size,
            &raw mut capacity,
        )
    {
        unavailable_value(
            &raw mut (*sample).capacity_bytes,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
    } else {
        (*sample).capacity_bytes.value = capacity;
        (*sample).capacity_bytes.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    if fragment_size == 0 as uint64_t && filesystem.f_bavail != 0 as __fsblkcnt_t
        || !checked_multiply(
            filesystem.f_bavail as uint64_t,
            fragment_size,
            &raw mut available,
        )
    {
        unavailable_value(
            &raw mut (*sample).available_bytes,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
    } else {
        (*sample).available_bytes.value = available;
        (*sample).available_bytes.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    if filesystem.f_files == 0 as __fsfilcnt_t {
        unavailable_value(
            &raw mut (*sample).capacity_inodes,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        );
        unavailable_value(
            &raw mut (*sample).available_inodes,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        );
    } else {
        (*sample).capacity_inodes.value = filesystem.f_files as uint64_t;
        (*sample).capacity_inodes.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        (*sample).available_inodes.value = filesystem.f_favail as uint64_t;
        (*sample).available_inodes.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_filesystem_sample(
    mut resource: *const linux_filesystem_resource_t,
    mut mountinfo_path: *const ::core::ffi::c_char,
    mut sample: *mut linux_filesystem_sample_t,
) -> ::core::ffi::c_int {
    let ops: linux_filesystem_ops_t = linux_filesystem_ops_t {
        stat_path: Some(
            default_stat_path
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut stat,
                ) -> ::core::ffi::c_int,
        ),
        statvfs_path: Some(
            default_statvfs_path
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut statvfs,
                ) -> ::core::ffi::c_int,
        ),
    };
    return linux_filesystem_sample_with_ops(resource, mountinfo_path, &raw const ops, sample);
}
unsafe extern "C" fn prepare_observation(
    mut observation: *mut system_health_observation_t,
    mut sample: *const linux_filesystem_sample_t,
    mut context: *const system_health_collect_context_t,
    mut metric: *const ::core::ffi::c_char,
) {
    memset(
        observation as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_observation_t>(),
    );
    snprintf(
        &raw mut (*observation).metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        metric,
    );
    snprintf(
        &raw mut (*observation).resource_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).logical_id as *const ::core::ffi::c_char,
    );
    (*observation).scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_FILESYSTEM;
    (*observation).sampled_monotonic_ms = (*context).monotonic_ms;
    (*observation).observed_wall_time_ms = (*context).wall_time_ms;
}
unsafe extern "C" fn emit_value(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut sample: *const linux_filesystem_sample_t,
    mut metric: *const ::core::ffi::c_char,
    mut value: *const linux_filesystem_value_t,
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
    prepare_observation(&raw mut observation, sample, context, metric);
    if (*value).capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 {
        system_health_observation_set_available(
            &raw mut observation,
            (*value).value as ::core::ffi::c_double,
            unit,
        );
    } else {
        system_health_observation_set_unavailable(&raw mut observation, (*value).capability);
    }
    system_health_observation_sink_append(sink, &raw mut observation);
}
unsafe extern "C" fn emit_flag(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut sample: *const linux_filesystem_sample_t,
    mut metric: *const ::core::ffi::c_char,
    mut flag: *const linux_filesystem_flag_t,
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
    prepare_observation(&raw mut observation, sample, context, metric);
    if (*flag).capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 {
        system_health_observation_set_available(
            &raw mut observation,
            if (*flag).value as ::core::ffi::c_int != 0 {
                1.0f64
            } else {
                0.0f64
            },
            system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
        );
    } else {
        system_health_observation_set_unavailable(&raw mut observation, (*flag).capability);
    }
    system_health_observation_sink_append(sink, &raw mut observation);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_filesystem_collect(
    mut state: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    if state.is_null() || context.is_null() || sink.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut configuration: *mut linux_filesystem_collector_state_t =
        state as *mut linux_filesystem_collector_state_t;
    let mut resource_count: size_t = (*configuration).resource_count;
    if resource_count > SYSTEM_HEALTH_MAX_FILESYSTEMS as size_t {
        resource_count = SYSTEM_HEALTH_MAX_FILESYSTEMS as size_t;
        (*configuration).resources_dropped = (*configuration)
            .resources_dropped
            .wrapping_add((*configuration).resource_count.wrapping_sub(resource_count));
    }
    let mut default_mountinfo: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut mountinfo: *const ::core::ffi::c_char =
        if (*configuration).mountinfo_path[0usize] as ::core::ffi::c_int != 0 {
            &raw mut (*configuration).mountinfo_path as *mut ::core::ffi::c_char
        } else {
            &raw mut default_mountinfo as *mut ::core::ffi::c_char
        };
    if (*configuration).mountinfo_path[0usize] == 0 {
        let mut proc_root: *const ::core::ffi::c_char = if !(*context).proc_root.is_null() {
            (*context).proc_root
        } else {
            b"/proc\0".as_ptr() as *const ::core::ffi::c_char
        };
        let mut length: ::core::ffi::c_int = snprintf(
            &raw mut default_mountinfo as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            proc_root,
            b"self/mountinfo\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if length < 0 as ::core::ffi::c_int
            || length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
        {
            mountinfo = ::core::ptr::null::<::core::ffi::c_char>();
        }
    }
    let mut index: size_t = 0 as size_t;
    while index < resource_count {
        let mut sample: linux_filesystem_sample_t = linux_filesystem_sample_t {
            logical_id: [0; 64],
            device_key: [0; 48],
            mount_present: linux_filesystem_flag_t {
                value: false,
                capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            },
            read_only: linux_filesystem_flag_t {
                value: false,
                capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            },
            mount_flags: linux_filesystem_value_t {
                value: 0,
                capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            },
            capacity_bytes: linux_filesystem_value_t {
                value: 0,
                capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            },
            available_bytes: linux_filesystem_value_t {
                value: 0,
                capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            },
            capacity_inodes: linux_filesystem_value_t {
                value: 0,
                capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            },
            available_inodes: linux_filesystem_value_t {
                value: 0,
                capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            },
        };
        if linux_filesystem_sample(
            (&raw mut (*configuration).resources as *mut linux_filesystem_resource_t)
                .offset(index as isize),
            mountinfo,
            &raw mut sample,
        ) == 0 as ::core::ffi::c_int
        {
            emit_flag(
                sink,
                context,
                &raw mut sample,
                b"filesystem.mount_present\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sample.mount_present,
            );
            emit_flag(
                sink,
                context,
                &raw mut sample,
                b"filesystem.read_only\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sample.read_only,
            );
            emit_value(
                sink,
                context,
                &raw mut sample,
                b"filesystem.flags\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sample.mount_flags,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
            );
            emit_value(
                sink,
                context,
                &raw mut sample,
                b"filesystem.capacity_bytes\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sample.capacity_bytes,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
            );
            emit_value(
                sink,
                context,
                &raw mut sample,
                b"filesystem.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sample.available_bytes,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
            );
            emit_value(
                sink,
                context,
                &raw mut sample,
                b"filesystem.capacity_inodes\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sample.capacity_inodes,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
            );
            emit_value(
                sink,
                context,
                &raw mut sample,
                b"filesystem.available_inodes\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut sample.available_inodes,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
            );
        }
        index = index.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_filesystem_collector_init(
    mut collector: *mut system_health_collector_t,
    mut state: *mut linux_filesystem_collector_state_t,
) {
    if collector.is_null() {
        return;
    }
    memset(
        collector as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_collector_t>(),
    );
    snprintf(
        &raw mut (*collector).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        b"linux_filesystem\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*collector).scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_FILESYSTEM;
    (*collector).tier = system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL;
    (*collector).interval_seconds = 30 as ::core::ffi::c_uint as uint32_t;
    (*collector).stale_after_seconds = 90 as ::core::ffi::c_uint as uint32_t;
    (*collector).state = state as *mut ::core::ffi::c_void;
    (*collector).collect = Some(
        linux_filesystem_collect
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
