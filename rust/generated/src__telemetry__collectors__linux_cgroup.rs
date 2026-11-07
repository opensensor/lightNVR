use ::c2rust_bitfields;
use ::f128;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    unsafe fn system_health_observation_set_available(
        observation: *mut system_health_observation_t,
        value: ::core::ffi::c_double,
        unit: system_health_unit_t,
    );
    unsafe fn system_health_observation_set_unavailable(
        observation: *mut system_health_observation_t,
        capability_0: system_health_capability_t,
    );
    unsafe fn system_health_observation_sink_append(
        sink: *mut system_health_observation_sink_t,
        observation: *const system_health_observation_t,
    ) -> bool;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...)
    -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn openat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct linux_cgroup_version_t(pub ::core::ffi::c_uint);
impl linux_cgroup_version_t {
    pub const LINUX_CGROUP_UNKNOWN: Self = Self(0);
    pub const LINUX_CGROUP_V1: Self = Self(1);
    pub const LINUX_CGROUP_V2: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_cgroup_limit_t {
    pub unlimited: bool,
    pub value: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_cgroup_cpu_max_t {
    pub quota_usec: linux_cgroup_limit_t,
    pub period_usec: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_cgroup_cpu_stat_t {
    pub usage_usec: uint64_t,
    pub throttle_present: bool,
    pub periods: uint64_t,
    pub throttled_periods: uint64_t,
    pub throttled_usec: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_cgroup_memory_events_t {
    pub low: uint64_t,
    pub high: uint64_t,
    pub max: uint64_t,
    pub oom: uint64_t,
    pub oom_kill: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_cgroup_memberships_t {
    pub version: linux_cgroup_version_t,
    pub unified: [::core::ffi::c_char; 512],
    pub cpu: [::core::ffi::c_char; 512],
    pub cpuacct: [::core::ffi::c_char; 512],
    pub memory: [::core::ffi::c_char; 512],
    pub pids: [::core::ffi::c_char; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_cgroup_state_t {
    pub memberships: linux_cgroup_memberships_t,
    pub cpu_valid: bool,
    pub cpu_quota_valid: bool,
    pub cpu_max: linux_cgroup_cpu_max_t,
    pub oom_valid: bool,
    pub version: linux_cgroup_version_t,
    pub cpu: linux_cgroup_cpu_stat_t,
    pub oom_kills: uint64_t,
    pub sampled_monotonic_ms: uint64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct cg_read_result_t(pub ::core::ffi::c_uint);
impl cg_read_result_t {
    pub const CG_READ_OK: Self = Self(0);
    pub const CG_READ_UNSUPPORTED: Self = Self(1);
    pub const CG_READ_PERMISSION: Self = Self(2);
    pub const CG_READ_ERROR: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cgroup_sample_t {
    pub cpu_max_status: cg_read_result_t,
    pub cpu_max: linux_cgroup_cpu_max_t,
    pub cpu_stat_status: cg_read_result_t,
    pub cpu_stat: linux_cgroup_cpu_stat_t,
    pub memory_current_status: cg_read_result_t,
    pub memory_current: uint64_t,
    pub memory_max_status: cg_read_result_t,
    pub memory_max: linux_cgroup_limit_t,
    pub oom_status: cg_read_result_t,
    pub oom_kills: uint64_t,
    pub pids_current_status: cg_read_result_t,
    pub pids_current: uint64_t,
    pub pids_max_status: cg_read_result_t,
    pub pids_max: linux_cgroup_limit_t,
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
pub const EOVERFLOW: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const F_DUPFD_CLOEXEC: ::core::ffi::c_int = 1030 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_path_open_directory_at(
    mut parent: ::core::ffi::c_int,
    mut relative: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if relative.is_null()
        || *relative.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
    {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
    let mut directory: ::core::ffi::c_int = fcntl(parent, F_DUPFD_CLOEXEC, 0 as ::core::ffi::c_int);
    if directory < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    while *relative != 0 {
        let mut component: [::core::ffi::c_char; 256] = [0; 256];
        let mut length: size_t =
            strcspn(relative, b"/\0".as_ptr() as *const ::core::ffi::c_char) as size_t;
        if length == 0 as size_t || length >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>() {
            close(directory);
            *__errno_location() = EINVAL;
            return -1 as ::core::ffi::c_int;
        }
        memcpy(
            &raw mut component as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            relative as *const ::core::ffi::c_void,
            length,
        );
        component[length] = '\0' as ::core::ffi::c_char;
        if !strchr(
            &raw mut component as *mut ::core::ffi::c_char,
            '/' as ::core::ffi::c_int,
        )
        .is_null()
            || strcmp(
                &raw mut component as *mut ::core::ffi::c_char,
                b".\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut component as *mut ::core::ffi::c_char,
                b"..\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            close(directory);
            *__errno_location() = EINVAL;
            return -1 as ::core::ffi::c_int;
        }
        let mut next: ::core::ffi::c_int = openat(
            directory,
            &raw mut component as *mut ::core::ffi::c_char,
            O_RDONLY | O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW,
        );
        let mut saved_errno: ::core::ffi::c_int = *__errno_location();
        close(directory);
        if next < 0 as ::core::ffi::c_int {
            *__errno_location() = saved_errno;
            return -1 as ::core::ffi::c_int;
        }
        directory = next;
        relative = relative.offset(length as isize);
        if *relative as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            relative = relative.offset(1);
        }
    }
    return directory;
}
pub const CGROUP_READ_MAX: ::core::ffi::c_uint = 16384 as ::core::ffi::c_uint;
unsafe extern "C" fn same_memberships(
    mut a: *const linux_cgroup_memberships_t,
    mut b: *const linux_cgroup_memberships_t,
) -> bool {
    return (*a).version.0 == (*b).version.0
        && strcmp(
            &raw const (*a).unified as *const ::core::ffi::c_char,
            &raw const (*b).unified as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && strcmp(
            &raw const (*a).cpu as *const ::core::ffi::c_char,
            &raw const (*b).cpu as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && strcmp(
            &raw const (*a).cpuacct as *const ::core::ffi::c_char,
            &raw const (*b).cpuacct as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && strcmp(
            &raw const (*a).memory as *const ::core::ffi::c_char,
            &raw const (*b).memory as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && strcmp(
            &raw const (*a).pids as *const ::core::ffi::c_char,
            &raw const (*b).pids as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn skip_space(
    mut text: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    while !text.is_null()
        && *(*__ctype_b_loc()).offset(*text as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        text = text.offset(1);
    }
    return text;
}
unsafe extern "C" fn parse_u64(
    mut text: *const ::core::ffi::c_char,
    mut end_out: *mut *const ::core::ffi::c_char,
    mut out: *mut uint64_t,
) -> bool {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut value: ::core::ffi::c_ulonglong = 0;
    if text.is_null()
        || out.is_null()
        || *(*__ctype_b_loc()).offset(*text as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        return r#false != 0;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    value = strtoull(text, &raw mut end, 10 as ::core::ffi::c_int);
    if *__errno_location() == ERANGE || end == text as *mut ::core::ffi::c_char {
        return r#false != 0;
    }
    *out = value as uint64_t;
    if !end_out.is_null() {
        *end_out = end;
    }
    return r#true != 0;
}
unsafe extern "C" fn only_trailing_space(mut text: *const ::core::ffi::c_char) -> bool {
    text = skip_space(text);
    return !text.is_null() && *text as ::core::ffi::c_int == '\0' as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_cgroup_parse_limit(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_limit_t,
) -> ::core::ffi::c_int {
    let mut cursor: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut parsed: linux_cgroup_limit_t = linux_cgroup_limit_t {
        unlimited: false,
        value: 0,
    };
    if text.is_null() || out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    cursor = skip_space(text);
    memset(
        &raw mut parsed as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_cgroup_limit_t>(),
    );
    if strncmp(
        cursor,
        b"max\0".as_ptr() as *const ::core::ffi::c_char,
        3 as size_t,
    ) == 0 as ::core::ffi::c_int
        && only_trailing_space(cursor.offset(3 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_int
            != 0
    {
        parsed.unlimited = r#true != 0;
    } else if !parse_u64(cursor, &raw mut cursor, &raw mut parsed.value)
        || !only_trailing_space(cursor)
    {
        return -1 as ::core::ffi::c_int;
    }
    *out = parsed;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_cgroup_parse_cpu_max(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_cpu_max_t,
) -> ::core::ffi::c_int {
    let mut cursor: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut parsed: linux_cgroup_cpu_max_t = linux_cgroup_cpu_max_t {
        quota_usec: linux_cgroup_limit_t {
            unlimited: false,
            value: 0,
        },
        period_usec: 0,
    };
    if text.is_null() || out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    cursor = skip_space(text);
    memset(
        &raw mut parsed as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_cgroup_cpu_max_t>(),
    );
    if strncmp(
        cursor,
        b"max\0".as_ptr() as *const ::core::ffi::c_char,
        3 as size_t,
    ) == 0 as ::core::ffi::c_int
        && *(*__ctype_b_loc())
            .offset(*cursor.offset(3isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        parsed.quota_usec.unlimited = r#true != 0;
        cursor = cursor.offset(3 as ::core::ffi::c_int as isize);
    } else if !parse_u64(cursor, &raw mut cursor, &raw mut parsed.quota_usec.value) {
        return -1 as ::core::ffi::c_int;
    }
    cursor = skip_space(cursor);
    if !parse_u64(cursor, &raw mut cursor, &raw mut parsed.period_usec)
        || parsed.period_usec == 0 as uint64_t
        || !parsed.quota_usec.unlimited && parsed.quota_usec.value == 0 as uint64_t
        || !only_trailing_space(cursor)
    {
        return -1 as ::core::ffi::c_int;
    }
    *out = parsed;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn keyed_u64(
    mut text: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_char,
    mut out: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut line: *const ::core::ffi::c_char = text;
    let mut key_len: size_t = strlen(key);
    while !line.is_null() && *line as ::core::ffi::c_int != 0 {
        let mut next: *const ::core::ffi::c_char = strchr(line, '\n' as ::core::ffi::c_int);
        if strncmp(line, key, key_len) == 0 as ::core::ffi::c_int
            && *(*__ctype_b_loc()).offset(*line.offset(key_len as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            let mut cursor: *const ::core::ffi::c_char = skip_space(line.offset(key_len as isize));
            let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            if !parse_u64(cursor, &raw mut end, out) {
                return -1 as ::core::ffi::c_int;
            }
            while *end as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
                && *(*__ctype_b_loc())
                    .offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    != 0
            {
                end = end.offset(1);
            }
            if *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                && *end as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        line = if !next.is_null() {
            next.offset(1 as ::core::ffi::c_int as isize)
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
    }
    return 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_cgroup_parse_cpu_stat(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_cpu_stat_t,
) -> ::core::ffi::c_int {
    let mut parsed: linux_cgroup_cpu_stat_t = linux_cgroup_cpu_stat_t {
        usage_usec: 0,
        throttle_present: false,
        periods: 0,
        throttled_periods: 0,
        throttled_usec: 0,
    };
    let mut usage: ::core::ffi::c_int = 0;
    let mut periods: ::core::ffi::c_int = 0;
    let mut throttled: ::core::ffi::c_int = 0;
    let mut throttled_usec: ::core::ffi::c_int = 0;
    if text.is_null() || out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        &raw mut parsed as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_cgroup_cpu_stat_t>(),
    );
    usage = keyed_u64(
        text,
        b"usage_usec\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.usage_usec,
    );
    periods = keyed_u64(
        text,
        b"nr_periods\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.periods,
    );
    throttled = keyed_u64(
        text,
        b"nr_throttled\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.throttled_periods,
    );
    throttled_usec = keyed_u64(
        text,
        b"throttled_usec\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.throttled_usec,
    );
    if usage != 0 as ::core::ffi::c_int {
        return if usage > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    if periods < 0 as ::core::ffi::c_int
        || throttled < 0 as ::core::ffi::c_int
        || throttled_usec < 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    parsed.throttle_present = periods == 0 as ::core::ffi::c_int
        && throttled == 0 as ::core::ffi::c_int
        && throttled_usec == 0 as ::core::ffi::c_int;
    if (periods == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        != (throttled == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        || (periods == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            != (throttled_usec == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    *out = parsed;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_cgroup_parse_memory_events(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_memory_events_t,
) -> ::core::ffi::c_int {
    let mut parsed: linux_cgroup_memory_events_t = linux_cgroup_memory_events_t {
        low: 0,
        high: 0,
        max: 0,
        oom: 0,
        oom_kill: 0,
    };
    let mut oom_kill: ::core::ffi::c_int = 0;
    if text.is_null() || out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        &raw mut parsed as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_cgroup_memory_events_t>(),
    );
    if keyed_u64(
        text,
        b"low\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.low,
    ) < 0 as ::core::ffi::c_int
        || keyed_u64(
            text,
            b"high\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut parsed.high,
        ) < 0 as ::core::ffi::c_int
        || keyed_u64(
            text,
            b"max\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut parsed.max,
        ) < 0 as ::core::ffi::c_int
        || keyed_u64(
            text,
            b"oom\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut parsed.oom,
        ) < 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    oom_kill = keyed_u64(
        text,
        b"oom_kill\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.oom_kill,
    );
    if oom_kill != 0 as ::core::ffi::c_int {
        return if oom_kill > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    *out = parsed;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_open_error() -> cg_read_result_t {
    if *__errno_location() == ENOENT || *__errno_location() == ENOTDIR {
        return cg_read_result_t::CG_READ_UNSUPPORTED;
    }
    if *__errno_location() == EACCES || *__errno_location() == EPERM {
        return cg_read_result_t::CG_READ_PERMISSION;
    }
    return cg_read_result_t::CG_READ_ERROR;
}
unsafe extern "C" fn read_stream_bounded(
    mut file: *mut FILE,
    mut buffer: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> cg_read_result_t {
    let mut used: size_t = fread(
        buffer as *mut ::core::ffi::c_void,
        1 as size_t,
        capacity.wrapping_sub(1 as size_t),
        file,
    ) as size_t;
    let mut saved_errno: ::core::ffi::c_int = if ferror(file) != 0 {
        if *__errno_location() != 0 {
            *__errno_location()
        } else {
            EIO
        }
    } else {
        0 as ::core::ffi::c_int
    };
    if feof(file) == 0 && used == capacity.wrapping_sub(1 as size_t) {
        saved_errno = EOVERFLOW;
    }
    if fclose(file) != 0 as ::core::ffi::c_int && saved_errno == 0 as ::core::ffi::c_int {
        saved_errno = *__errno_location();
    }
    if saved_errno != 0 as ::core::ffi::c_int {
        return cg_read_result_t::CG_READ_ERROR;
    }
    *buffer.offset(used as isize) = '\0' as ::core::ffi::c_char;
    return cg_read_result_t::CG_READ_OK;
}
unsafe extern "C" fn read_bounded(
    mut path: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> cg_read_result_t {
    if path.is_null() || buffer.is_null() || capacity < 2 as size_t {
        return cg_read_result_t::CG_READ_ERROR;
    }
    let mut file: *mut FILE = fopen(path, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    if file.is_null() {
        return read_open_error();
    }
    return read_stream_bounded(file, buffer, capacity);
}
unsafe extern "C" fn safe_membership(mut path: *const ::core::ffi::c_char) -> bool {
    if path.is_null() || *path.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int {
        return r#false != 0;
    }
    if !strstr(path, b"//\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = path as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if (*cursor as ::core::ffi::c_uint) < 0x20 as ::core::ffi::c_uint
            || *cursor as ::core::ffi::c_uint == 0x7f as ::core::ffi::c_uint
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    let mut component: *const ::core::ffi::c_char = path;
    while !component.is_null() {
        while *component as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            component = component.offset(1);
        }
        let mut length: size_t =
            strcspn(component, b"/\0".as_ptr() as *const ::core::ffi::c_char) as size_t;
        if length == 1 as size_t
            && *component.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || length == 2 as size_t
                && *component.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && *component.offset(1isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        component = if *component.offset(length as isize) as ::core::ffi::c_int != 0 {
            component.offset(length as isize)
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
    }
    return strlen(path) < 512 as size_t;
}
unsafe extern "C" fn copy_membership(
    mut destination: *mut ::core::ffi::c_char,
    mut capacity: size_t,
    mut path: *const ::core::ffi::c_char,
) {
    snprintf(
        destination,
        capacity,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        path,
    );
}
unsafe extern "C" fn controller_list_has(
    mut list: *const ::core::ffi::c_char,
    mut controller: *const ::core::ffi::c_char,
) -> bool {
    let mut wanted: size_t = strlen(controller);
    let mut cursor: *const ::core::ffi::c_char = list;
    while !cursor.is_null() && *cursor as ::core::ffi::c_int != 0 {
        let mut end: *const ::core::ffi::c_char = strchr(cursor, ',' as ::core::ffi::c_int);
        let mut length: size_t = if !end.is_null() {
            end.offset_from(cursor) as size_t
        } else {
            strlen(cursor)
        };
        if length == wanted && strncmp(cursor, controller, wanted) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        cursor = if !end.is_null() {
            end.offset(1 as ::core::ffi::c_int as isize)
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
    }
    return r#false != 0;
}
unsafe extern "C" fn parse_memberships(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_memberships_t,
) -> ::core::ffi::c_int {
    let mut line: *const ::core::ffi::c_char = text;
    let mut parsed: linux_cgroup_memberships_t = linux_cgroup_memberships_t {
        version: linux_cgroup_version_t::LINUX_CGROUP_UNKNOWN,
        unified: [0; 512],
        cpu: [0; 512],
        cpuacct: [0; 512],
        memory: [0; 512],
        pids: [0; 512],
    };
    if text.is_null() || out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        &raw mut parsed as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_cgroup_memberships_t>(),
    );
    while !line.is_null() && *line as ::core::ffi::c_int != 0 {
        let mut next: *const ::core::ffi::c_char = strchr(line, '\n' as ::core::ffi::c_int);
        let mut first: *const ::core::ffi::c_char = strchr(line, ':' as ::core::ffi::c_int);
        let mut second: *const ::core::ffi::c_char = if !first.is_null() {
            strchr(
                first.offset(1 as ::core::ffi::c_int as isize),
                ':' as ::core::ffi::c_int,
            )
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        let mut controllers_length: size_t = 0;
        let mut path_length: size_t = 0;
        let mut controllers: [::core::ffi::c_char; 256] = [0; 256];
        let mut path: [::core::ffi::c_char; 512] = [0; 512];
        if first.is_null() || second.is_null() || !next.is_null() && second >= next {
            return -1 as ::core::ffi::c_int;
        }
        controllers_length = (second.offset_from(first) - 1isize) as size_t;
        path_length = if !next.is_null() {
            (next.offset_from(second) - 1isize) as size_t
        } else {
            strlen(second.offset(1 as ::core::ffi::c_int as isize))
        };
        if controllers_length >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            || path_length >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        {
            return -1 as ::core::ffi::c_int;
        }
        memcpy(
            &raw mut controllers as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            first.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            controllers_length,
        );
        controllers[controllers_length] = '\0' as ::core::ffi::c_char;
        memcpy(
            &raw mut path as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            second.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            path_length,
        );
        path[path_length] = '\0' as ::core::ffi::c_char;
        if !safe_membership(&raw mut path as *mut ::core::ffi::c_char) {
            return -1 as ::core::ffi::c_int;
        }
        if controllers_length == 0 as size_t {
            parsed.version = linux_cgroup_version_t::LINUX_CGROUP_V2;
            copy_membership(
                &raw mut parsed.unified as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                &raw mut path as *mut ::core::ffi::c_char,
            );
        } else if parsed.version.0 != linux_cgroup_version_t::LINUX_CGROUP_V2.0 {
            parsed.version = linux_cgroup_version_t::LINUX_CGROUP_V1;
            if controller_list_has(
                &raw mut controllers as *mut ::core::ffi::c_char,
                b"cpu\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                copy_membership(
                    &raw mut parsed.cpu as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    &raw mut path as *mut ::core::ffi::c_char,
                );
            }
            if controller_list_has(
                &raw mut controllers as *mut ::core::ffi::c_char,
                b"cpuacct\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                copy_membership(
                    &raw mut parsed.cpuacct as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    &raw mut path as *mut ::core::ffi::c_char,
                );
            }
            if controller_list_has(
                &raw mut controllers as *mut ::core::ffi::c_char,
                b"memory\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                copy_membership(
                    &raw mut parsed.memory as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    &raw mut path as *mut ::core::ffi::c_char,
                );
            }
            if controller_list_has(
                &raw mut controllers as *mut ::core::ffi::c_char,
                b"pids\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                copy_membership(
                    &raw mut parsed.pids as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    &raw mut path as *mut ::core::ffi::c_char,
                );
            }
        }
        line = if !next.is_null() {
            next.offset(1 as ::core::ffi::c_int as isize)
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
    }
    if parsed.version.0 == linux_cgroup_version_t::LINUX_CGROUP_UNKNOWN.0 {
        return -1 as ::core::ffi::c_int;
    }
    *out = parsed;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_proc_membership(
    mut proc_root: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_memberships_t,
) -> cg_read_result_t {
    let mut path: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut text: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut length: ::core::ffi::c_int = snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s/self/cgroup\0".as_ptr() as *const ::core::ffi::c_char,
        proc_root,
    );
    let mut result: cg_read_result_t = cg_read_result_t::CG_READ_OK;
    if length < 0 as ::core::ffi::c_int
        || length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
    {
        return cg_read_result_t::CG_READ_ERROR;
    }
    result = read_bounded(
        &raw mut path as *mut ::core::ffi::c_char,
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
    );
    if result.0 != cg_read_result_t::CG_READ_OK.0 {
        return result;
    }
    return cg_read_result_t(
        (if parse_memberships(&raw mut text as *mut ::core::ffi::c_char, out)
            == 0 as ::core::ffi::c_int
        {
            cg_read_result_t::CG_READ_OK.0 as ::core::ffi::c_int
        } else {
            cg_read_result_t::CG_READ_ERROR.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn read_cgroup(
    mut root: *const ::core::ffi::c_char,
    mut controller: *const ::core::ffi::c_char,
    mut membership: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut text: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> cg_read_result_t {
    if root.is_null()
        || !safe_membership(membership)
        || filename.is_null()
        || *filename == 0
        || !strchr(filename, '/' as ::core::ffi::c_int).is_null()
        || strcmp(filename, b".\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(filename, b"..\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || text.is_null()
        || capacity < 2 as size_t
    {
        return cg_read_result_t::CG_READ_ERROR;
    }
    let mut directory: ::core::ffi::c_int =
        open(root, O_RDONLY | O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW);
    if directory < 0 as ::core::ffi::c_int {
        return read_open_error();
    }
    if !controller.is_null() {
        let mut next: ::core::ffi::c_int = safe_path_open_directory_at(directory, controller);
        let mut saved_errno: ::core::ffi::c_int = *__errno_location();
        close(directory);
        *__errno_location() = saved_errno;
        if next < 0 as ::core::ffi::c_int {
            return read_open_error();
        }
        directory = next;
    }
    let mut member: ::core::ffi::c_int = safe_path_open_directory_at(
        directory,
        membership.offset(1 as ::core::ffi::c_int as isize),
    );
    let mut saved_errno_0: ::core::ffi::c_int = *__errno_location();
    close(directory);
    *__errno_location() = saved_errno_0;
    if member < 0 as ::core::ffi::c_int {
        return read_open_error();
    }
    let mut descriptor: ::core::ffi::c_int = openat(
        member,
        filename,
        O_RDONLY | O_CLOEXEC | O_NOFOLLOW | O_NONBLOCK,
    );
    saved_errno_0 = *__errno_location();
    close(member);
    *__errno_location() = saved_errno_0;
    if descriptor < 0 as ::core::ffi::c_int {
        return read_open_error();
    }
    let mut status: stat = stat {
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
    if fstat(descriptor, &raw mut status) != 0 as ::core::ffi::c_int
        || !(status.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
    {
        close(descriptor);
        return cg_read_result_t::CG_READ_ERROR;
    }
    let mut file: *mut FILE = fdopen(descriptor, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    if file.is_null() {
        close(descriptor);
        return cg_read_result_t::CG_READ_ERROR;
    }
    return read_stream_bounded(file, text, capacity);
}
unsafe extern "C" fn read_unified(
    mut root: *const ::core::ffi::c_char,
    mut membership: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut text: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> cg_read_result_t {
    return read_cgroup(
        root,
        ::core::ptr::null::<::core::ffi::c_char>(),
        membership,
        filename,
        text,
        capacity,
    );
}
unsafe extern "C" fn read_v1(
    mut root: *const ::core::ffi::c_char,
    mut membership: *const ::core::ffi::c_char,
    mut controller: *const ::core::ffi::c_char,
    mut combined_controller: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut text: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> cg_read_result_t {
    let mut controllers: [*const ::core::ffi::c_char; 3] = [
        controller,
        combined_controller,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut strongest: cg_read_result_t = cg_read_result_t::CG_READ_UNSUPPORTED;
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        let mut result: cg_read_result_t =
            read_cgroup(root, controllers[i], membership, filename, text, capacity);
        if result.0 == cg_read_result_t::CG_READ_OK.0 {
            return result;
        }
        if result.0 == cg_read_result_t::CG_READ_PERMISSION.0 {
            strongest = cg_read_result_t::CG_READ_PERMISSION;
        } else if result.0 == cg_read_result_t::CG_READ_ERROR.0
            && strongest.0 != cg_read_result_t::CG_READ_PERMISSION.0
        {
            strongest = cg_read_result_t::CG_READ_ERROR;
        }
        i = i.wrapping_add(1);
    }
    return strongest;
}
unsafe extern "C" fn parse_plain_u64(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut uint64_t,
) -> cg_read_result_t {
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !parse_u64(skip_space(text), &raw mut end, out) || !only_trailing_space(end) {
        return cg_read_result_t::CG_READ_ERROR;
    }
    return cg_read_result_t::CG_READ_OK;
}
unsafe extern "C" fn parse_limit_text(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_limit_t,
) -> cg_read_result_t {
    return cg_read_result_t(
        (if linux_cgroup_parse_limit(text, out) == 0 as ::core::ffi::c_int {
            cg_read_result_t::CG_READ_OK.0 as ::core::ffi::c_int
        } else {
            cg_read_result_t::CG_READ_ERROR.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn read_v2_value(
    mut root: *const ::core::ffi::c_char,
    mut membership: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut text: *mut ::core::ffi::c_char,
    mut out: *mut uint64_t,
) -> cg_read_result_t {
    let mut result: cg_read_result_t =
        read_unified(root, membership, filename, text, CGROUP_READ_MAX as size_t);
    return cg_read_result_t(if result.0 == cg_read_result_t::CG_READ_OK.0 {
        parse_plain_u64(text, out).0
    } else {
        result.0
    });
}
unsafe extern "C" fn read_v2_limit(
    mut root: *const ::core::ffi::c_char,
    mut membership: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut text: *mut ::core::ffi::c_char,
    mut out: *mut linux_cgroup_limit_t,
) -> cg_read_result_t {
    let mut result: cg_read_result_t =
        read_unified(root, membership, filename, text, CGROUP_READ_MAX as size_t);
    return cg_read_result_t(if result.0 == cg_read_result_t::CG_READ_OK.0 {
        parse_limit_text(text, out).0
    } else {
        result.0
    });
}
unsafe extern "C" fn parent_membership(mut membership: *mut ::core::ffi::c_char) -> bool {
    let mut slash: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut length: size_t = 0;
    if membership.is_null()
        || strcmp(membership, b"/\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    length = strlen(membership);
    while length > 1 as size_t
        && *membership.offset(length.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
    {
        length = length.wrapping_sub(1);
        *membership.offset(length as isize) = '\0' as ::core::ffi::c_char;
    }
    slash = strrchr(membership, '/' as ::core::ffi::c_int);
    if slash.is_null() {
        return r#false != 0;
    }
    if slash == membership {
        *membership.offset(1isize) = '\0' as ::core::ffi::c_char;
    } else {
        *slash = '\0' as ::core::ffi::c_char;
    }
    return r#true != 0;
}
unsafe extern "C" fn effective_v2_limit(
    mut root: *const ::core::ffi::c_char,
    mut membership: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_limit_t,
) -> cg_read_result_t {
    let mut current: [::core::ffi::c_char; 512] = [0; 512];
    let mut text: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut effective: linux_cgroup_limit_t = linux_cgroup_limit_t {
        unlimited: r#true != 0,
        value: 0,
    };
    let mut found: bool = r#false != 0;
    copy_membership(
        &raw mut current as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        membership,
    );
    loop {
        let mut candidate: linux_cgroup_limit_t = linux_cgroup_limit_t {
            unlimited: false,
            value: 0,
        };
        let mut result: cg_read_result_t = read_v2_limit(
            root,
            &raw mut current as *mut ::core::ffi::c_char,
            filename,
            &raw mut text as *mut ::core::ffi::c_char,
            &raw mut candidate,
        );
        if result.0 == cg_read_result_t::CG_READ_PERMISSION.0
            || result.0 == cg_read_result_t::CG_READ_ERROR.0
        {
            return result;
        }
        if result.0 == cg_read_result_t::CG_READ_OK.0 {
            found = r#true != 0;
            if !candidate.unlimited
                && (effective.unlimited as ::core::ffi::c_int != 0
                    || candidate.value < effective.value)
            {
                effective = candidate;
            }
        }
        if !parent_membership(&raw mut current as *mut ::core::ffi::c_char) {
            break;
        }
    }
    if !found {
        return cg_read_result_t::CG_READ_UNSUPPORTED;
    }
    *out = effective;
    return cg_read_result_t::CG_READ_OK;
}
unsafe extern "C" fn effective_v2_cpu_max(
    mut root: *const ::core::ffi::c_char,
    mut membership: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_cpu_max_t,
) -> cg_read_result_t {
    let mut current: [::core::ffi::c_char; 512] = [0; 512];
    let mut text: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut effective: linux_cgroup_cpu_max_t = linux_cgroup_cpu_max_t {
        quota_usec: linux_cgroup_limit_t {
            unlimited: false,
            value: 0,
        },
        period_usec: 0,
    };
    let mut found: bool = r#false != 0;
    memset(
        &raw mut effective as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_cgroup_cpu_max_t>(),
    );
    effective.quota_usec.unlimited = r#true != 0;
    copy_membership(
        &raw mut current as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        membership,
    );
    loop {
        let mut candidate: linux_cgroup_cpu_max_t = linux_cgroup_cpu_max_t {
            quota_usec: linux_cgroup_limit_t {
                unlimited: false,
                value: 0,
            },
            period_usec: 0,
        };
        let mut result: cg_read_result_t = read_unified(
            root,
            &raw mut current as *mut ::core::ffi::c_char,
            b"cpu.max\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut text as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
        );
        if result.0 == cg_read_result_t::CG_READ_PERMISSION.0
            || result.0 == cg_read_result_t::CG_READ_ERROR.0
        {
            return result;
        }
        if result.0 == cg_read_result_t::CG_READ_OK.0 {
            if linux_cgroup_parse_cpu_max(
                &raw mut text as *mut ::core::ffi::c_char,
                &raw mut candidate,
            ) != 0 as ::core::ffi::c_int
            {
                return cg_read_result_t::CG_READ_ERROR;
            }
            found = r#true != 0;
            if !candidate.quota_usec.unlimited
                && (effective.quota_usec.unlimited as ::core::ffi::c_int != 0
                    || ::f128::f128::new(candidate.quota_usec.value)
                        / ::f128::f128::new(candidate.period_usec)
                        < ::f128::f128::new(effective.quota_usec.value)
                            / ::f128::f128::new(effective.period_usec))
            {
                effective = candidate;
            }
        }
        if !parent_membership(&raw mut current as *mut ::core::ffi::c_char) {
            break;
        }
    }
    if !found {
        return cg_read_result_t::CG_READ_UNSUPPORTED;
    }
    *out = effective;
    return cg_read_result_t::CG_READ_OK;
}
unsafe extern "C" fn read_v1_value(
    mut root: *const ::core::ffi::c_char,
    mut membership: *const ::core::ffi::c_char,
    mut controller: *const ::core::ffi::c_char,
    mut combined: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut text: *mut ::core::ffi::c_char,
    mut out: *mut uint64_t,
) -> cg_read_result_t {
    let mut result: cg_read_result_t = read_v1(
        root,
        membership,
        controller,
        combined,
        filename,
        text,
        CGROUP_READ_MAX as size_t,
    );
    return cg_read_result_t(if result.0 == cg_read_result_t::CG_READ_OK.0 {
        parse_plain_u64(text, out).0
    } else {
        result.0
    });
}
unsafe extern "C" fn parse_v1_cpu_max(
    mut quota_text: *const ::core::ffi::c_char,
    mut period_text: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_cpu_max_t,
) -> cg_read_result_t {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut quota: ::core::ffi::c_longlong = 0;
    let mut period: uint64_t = 0;
    let mut parsed: linux_cgroup_cpu_max_t = linux_cgroup_cpu_max_t {
        quota_usec: linux_cgroup_limit_t {
            unlimited: false,
            value: 0,
        },
        period_usec: 0,
    };
    *__errno_location() = 0 as ::core::ffi::c_int;
    quota = strtoll(
        skip_space(quota_text),
        &raw mut end,
        10 as ::core::ffi::c_int,
    );
    if *__errno_location() == ERANGE
        || end == skip_space(quota_text) as *mut ::core::ffi::c_char
        || !only_trailing_space(end)
    {
        return cg_read_result_t::CG_READ_ERROR;
    }
    if parse_plain_u64(period_text, &raw mut period).0 != cg_read_result_t::CG_READ_OK.0
        || period == 0 as uint64_t
    {
        return cg_read_result_t::CG_READ_ERROR;
    }
    memset(
        &raw mut parsed as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_cgroup_cpu_max_t>(),
    );
    if quota == -1 as ::core::ffi::c_longlong {
        parsed.quota_usec.unlimited = r#true != 0;
    } else if quota < 0 as ::core::ffi::c_longlong {
        return cg_read_result_t::CG_READ_ERROR;
    } else if quota == 0 as ::core::ffi::c_longlong {
        return cg_read_result_t::CG_READ_ERROR;
    } else {
        parsed.quota_usec.value = quota as uint64_t;
    }
    parsed.period_usec = period;
    *out = parsed;
    return cg_read_result_t::CG_READ_OK;
}
unsafe extern "C" fn sample_v2(
    mut root: *const ::core::ffi::c_char,
    mut memberships: *const linux_cgroup_memberships_t,
    mut sample: *mut cgroup_sample_t,
) {
    let mut first: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut second: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut events: linux_cgroup_memory_events_t = linux_cgroup_memory_events_t {
        low: 0,
        high: 0,
        max: 0,
        oom: 0,
        oom_kill: 0,
    };
    (*sample).cpu_max_status = effective_v2_cpu_max(
        root,
        &raw const (*memberships).unified as *const ::core::ffi::c_char,
        &raw mut (*sample).cpu_max,
    );
    (*sample).cpu_stat_status = read_unified(
        root,
        &raw const (*memberships).unified as *const ::core::ffi::c_char,
        b"cpu.stat\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut first as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
    );
    if (*sample).cpu_stat_status.0 == cg_read_result_t::CG_READ_OK.0
        && linux_cgroup_parse_cpu_stat(
            &raw mut first as *mut ::core::ffi::c_char,
            &raw mut (*sample).cpu_stat,
        ) != 0 as ::core::ffi::c_int
    {
        (*sample).cpu_stat_status = cg_read_result_t::CG_READ_ERROR;
    }
    (*sample).memory_current_status = read_v2_value(
        root,
        &raw const (*memberships).unified as *const ::core::ffi::c_char,
        b"memory.current\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut first as *mut ::core::ffi::c_char,
        &raw mut (*sample).memory_current,
    );
    (*sample).memory_max_status = effective_v2_limit(
        root,
        &raw const (*memberships).unified as *const ::core::ffi::c_char,
        b"memory.max\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sample).memory_max,
    );
    (*sample).oom_status = read_unified(
        root,
        &raw const (*memberships).unified as *const ::core::ffi::c_char,
        b"memory.events\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut second as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
    );
    if (*sample).oom_status.0 == cg_read_result_t::CG_READ_OK.0 {
        let mut rc: ::core::ffi::c_int = linux_cgroup_parse_memory_events(
            &raw mut second as *mut ::core::ffi::c_char,
            &raw mut events,
        );
        if rc == 0 as ::core::ffi::c_int {
            (*sample).oom_kills = events.oom_kill;
        } else {
            (*sample).oom_status = cg_read_result_t(
                (if rc > 0 as ::core::ffi::c_int {
                    cg_read_result_t::CG_READ_UNSUPPORTED.0 as ::core::ffi::c_int
                } else {
                    cg_read_result_t::CG_READ_ERROR.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            );
        }
    }
    (*sample).pids_current_status = read_v2_value(
        root,
        &raw const (*memberships).unified as *const ::core::ffi::c_char,
        b"pids.current\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut first as *mut ::core::ffi::c_char,
        &raw mut (*sample).pids_current,
    );
    (*sample).pids_max_status = effective_v2_limit(
        root,
        &raw const (*memberships).unified as *const ::core::ffi::c_char,
        b"pids.max\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sample).pids_max,
    );
}
unsafe extern "C" fn read_v1_limit(
    mut root: *const ::core::ffi::c_char,
    mut membership: *const ::core::ffi::c_char,
    mut controller: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut text: *mut ::core::ffi::c_char,
    mut out: *mut linux_cgroup_limit_t,
) -> cg_read_result_t {
    let mut result: cg_read_result_t = read_v1(
        root,
        membership,
        controller,
        ::core::ptr::null::<::core::ffi::c_char>(),
        filename,
        text,
        CGROUP_READ_MAX as size_t,
    );
    if result.0 != cg_read_result_t::CG_READ_OK.0 {
        return result;
    }
    result = parse_limit_text(text, out);
    if result.0 == cg_read_result_t::CG_READ_OK.0
        && (*out).value >= (1 as uint64_t) << 60 as ::core::ffi::c_int
    {
        (*out).unlimited = r#true != 0;
        (*out).value = 0 as uint64_t;
    }
    return result;
}
unsafe extern "C" fn effective_v1_limit(
    mut root: *const ::core::ffi::c_char,
    mut membership: *const ::core::ffi::c_char,
    mut controller: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_limit_t,
) -> cg_read_result_t {
    let mut current: [::core::ffi::c_char; 512] = [0; 512];
    let mut text: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut effective: linux_cgroup_limit_t = linux_cgroup_limit_t {
        unlimited: r#true != 0,
        value: 0,
    };
    let mut found: bool = r#false != 0;
    copy_membership(
        &raw mut current as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        membership,
    );
    loop {
        let mut candidate: linux_cgroup_limit_t = linux_cgroup_limit_t {
            unlimited: false,
            value: 0,
        };
        let mut result: cg_read_result_t = read_v1_limit(
            root,
            &raw mut current as *mut ::core::ffi::c_char,
            controller,
            filename,
            &raw mut text as *mut ::core::ffi::c_char,
            &raw mut candidate,
        );
        if result.0 == cg_read_result_t::CG_READ_PERMISSION.0
            || result.0 == cg_read_result_t::CG_READ_ERROR.0
        {
            return result;
        }
        if result.0 == cg_read_result_t::CG_READ_OK.0 {
            found = r#true != 0;
            if !candidate.unlimited
                && (effective.unlimited as ::core::ffi::c_int != 0
                    || candidate.value < effective.value)
            {
                effective = candidate;
            }
        }
        if !parent_membership(&raw mut current as *mut ::core::ffi::c_char) {
            break;
        }
    }
    if !found {
        return cg_read_result_t::CG_READ_UNSUPPORTED;
    }
    *out = effective;
    return cg_read_result_t::CG_READ_OK;
}
unsafe extern "C" fn effective_v1_cpu_max(
    mut root: *const ::core::ffi::c_char,
    mut membership: *const ::core::ffi::c_char,
    mut out: *mut linux_cgroup_cpu_max_t,
) -> cg_read_result_t {
    let mut current: [::core::ffi::c_char; 512] = [0; 512];
    let mut quota_text: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut period_text: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut effective: linux_cgroup_cpu_max_t = linux_cgroup_cpu_max_t {
        quota_usec: linux_cgroup_limit_t {
            unlimited: false,
            value: 0,
        },
        period_usec: 0,
    };
    let mut found: bool = r#false != 0;
    memset(
        &raw mut effective as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_cgroup_cpu_max_t>(),
    );
    effective.quota_usec.unlimited = r#true != 0;
    copy_membership(
        &raw mut current as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        membership,
    );
    loop {
        let mut quota_status: cg_read_result_t = read_v1(
            root,
            &raw mut current as *mut ::core::ffi::c_char,
            b"cpu\0".as_ptr() as *const ::core::ffi::c_char,
            b"cpu,cpuacct\0".as_ptr() as *const ::core::ffi::c_char,
            b"cpu.cfs_quota_us\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut quota_text as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
        );
        let mut period_status: cg_read_result_t = read_v1(
            root,
            &raw mut current as *mut ::core::ffi::c_char,
            b"cpu\0".as_ptr() as *const ::core::ffi::c_char,
            b"cpu,cpuacct\0".as_ptr() as *const ::core::ffi::c_char,
            b"cpu.cfs_period_us\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut period_text as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
        );
        if quota_status.0 == cg_read_result_t::CG_READ_PERMISSION.0
            || period_status.0 == cg_read_result_t::CG_READ_PERMISSION.0
        {
            return cg_read_result_t::CG_READ_PERMISSION;
        }
        if quota_status.0 == cg_read_result_t::CG_READ_ERROR.0
            || period_status.0 == cg_read_result_t::CG_READ_ERROR.0
        {
            return cg_read_result_t::CG_READ_ERROR;
        }
        if quota_status.0 == cg_read_result_t::CG_READ_OK.0
            && period_status.0 == cg_read_result_t::CG_READ_OK.0
        {
            let mut candidate: linux_cgroup_cpu_max_t = linux_cgroup_cpu_max_t {
                quota_usec: linux_cgroup_limit_t {
                    unlimited: false,
                    value: 0,
                },
                period_usec: 0,
            };
            if parse_v1_cpu_max(
                &raw mut quota_text as *mut ::core::ffi::c_char,
                &raw mut period_text as *mut ::core::ffi::c_char,
                &raw mut candidate,
            )
            .0 != cg_read_result_t::CG_READ_OK.0
            {
                return cg_read_result_t::CG_READ_ERROR;
            }
            found = r#true != 0;
            if !candidate.quota_usec.unlimited
                && (effective.quota_usec.unlimited as ::core::ffi::c_int != 0
                    || ::f128::f128::new(candidate.quota_usec.value)
                        / ::f128::f128::new(candidate.period_usec)
                        < ::f128::f128::new(effective.quota_usec.value)
                            / ::f128::f128::new(effective.period_usec))
            {
                effective = candidate;
            }
        } else if quota_status.0 != period_status.0 {
            return cg_read_result_t::CG_READ_ERROR;
        }
        if !parent_membership(&raw mut current as *mut ::core::ffi::c_char) {
            break;
        }
    }
    if !found {
        return cg_read_result_t::CG_READ_UNSUPPORTED;
    }
    *out = effective;
    return cg_read_result_t::CG_READ_OK;
}
unsafe extern "C" fn sample_v1(
    mut root: *const ::core::ffi::c_char,
    mut memberships: *const linux_cgroup_memberships_t,
    mut sample: *mut cgroup_sample_t,
) {
    let mut first: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut second: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut cpu_usage_ns: uint64_t = 0;
    (*sample).cpu_max_status = effective_v1_cpu_max(
        root,
        &raw const (*memberships).cpu as *const ::core::ffi::c_char,
        &raw mut (*sample).cpu_max,
    );
    (*sample).cpu_stat_status = read_v1_value(
        root,
        &raw const (*memberships).cpuacct as *const ::core::ffi::c_char,
        b"cpuacct\0".as_ptr() as *const ::core::ffi::c_char,
        b"cpu,cpuacct\0".as_ptr() as *const ::core::ffi::c_char,
        b"cpuacct.usage\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut first as *mut ::core::ffi::c_char,
        &raw mut cpu_usage_ns,
    );
    if (*sample).cpu_stat_status.0 == cg_read_result_t::CG_READ_OK.0 {
        (*sample).cpu_stat.usage_usec = cpu_usage_ns.wrapping_div(1000 as uint64_t);
        let mut stat_result: cg_read_result_t = read_v1(
            root,
            &raw const (*memberships).cpu as *const ::core::ffi::c_char,
            b"cpu\0".as_ptr() as *const ::core::ffi::c_char,
            b"cpu,cpuacct\0".as_ptr() as *const ::core::ffi::c_char,
            b"cpu.stat\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut second as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
        );
        if stat_result.0 == cg_read_result_t::CG_READ_OK.0 {
            let mut periods: ::core::ffi::c_int = keyed_u64(
                &raw mut second as *mut ::core::ffi::c_char,
                b"nr_periods\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*sample).cpu_stat.periods,
            );
            let mut throttled: ::core::ffi::c_int = keyed_u64(
                &raw mut second as *mut ::core::ffi::c_char,
                b"nr_throttled\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*sample).cpu_stat.throttled_periods,
            );
            let mut time: ::core::ffi::c_int = keyed_u64(
                &raw mut second as *mut ::core::ffi::c_char,
                b"throttled_time\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*sample).cpu_stat.throttled_usec,
            );
            if periods < 0 as ::core::ffi::c_int
                || throttled < 0 as ::core::ffi::c_int
                || time < 0 as ::core::ffi::c_int
            {
                (*sample).cpu_stat_status = cg_read_result_t::CG_READ_ERROR;
            } else {
                (*sample).cpu_stat.throttle_present = periods == 0 as ::core::ffi::c_int
                    && throttled == 0 as ::core::ffi::c_int
                    && time == 0 as ::core::ffi::c_int;
                if (periods == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    != (throttled == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    || (periods == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        != (time == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                {
                    (*sample).cpu_stat_status = cg_read_result_t::CG_READ_ERROR;
                }
                (*sample).cpu_stat.throttled_usec = (*sample)
                    .cpu_stat
                    .throttled_usec
                    .wrapping_div(1000 as uint64_t);
            }
        } else if stat_result.0 == cg_read_result_t::CG_READ_PERMISSION.0
            || stat_result.0 == cg_read_result_t::CG_READ_ERROR.0
        {
            (*sample).cpu_stat_status = stat_result;
        }
    }
    (*sample).memory_current_status = read_v1_value(
        root,
        &raw const (*memberships).memory as *const ::core::ffi::c_char,
        b"memory\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"memory.usage_in_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut first as *mut ::core::ffi::c_char,
        &raw mut (*sample).memory_current,
    );
    (*sample).memory_max_status = effective_v1_limit(
        root,
        &raw const (*memberships).memory as *const ::core::ffi::c_char,
        b"memory\0".as_ptr() as *const ::core::ffi::c_char,
        b"memory.limit_in_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sample).memory_max,
    );
    (*sample).oom_status = cg_read_result_t::CG_READ_UNSUPPORTED;
    (*sample).oom_kills = 0 as uint64_t;
    if (*memberships).pids[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        (*sample).pids_current_status = read_v1_value(
            root,
            &raw const (*memberships).pids as *const ::core::ffi::c_char,
            b"pids\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"pids.current\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut first as *mut ::core::ffi::c_char,
            &raw mut (*sample).pids_current,
        );
        (*sample).pids_max_status = effective_v1_limit(
            root,
            &raw const (*memberships).pids as *const ::core::ffi::c_char,
            b"pids\0".as_ptr() as *const ::core::ffi::c_char,
            b"pids.max\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*sample).pids_max,
        );
    } else {
        (*sample).pids_current_status = cg_read_result_t::CG_READ_UNSUPPORTED;
        (*sample).pids_max_status = cg_read_result_t::CG_READ_UNSUPPORTED;
    };
}
unsafe extern "C" fn capability(mut result: cg_read_result_t) -> system_health_capability_t {
    match result {
        cg_read_result_t::CG_READ_OK => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        }
        cg_read_result_t::CG_READ_UNSUPPORTED => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        }
        cg_read_result_t::CG_READ_PERMISSION => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
        }
        cg_read_result_t::CG_READ_ERROR => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        }
        _ => {}
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
}
unsafe extern "C" fn init_observation(
    mut observation: *mut system_health_observation_t,
    mut context: *const system_health_collect_context_t,
    mut scope: system_health_scope_t,
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
        b"self\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*observation).scope = scope;
    (*observation).sampled_monotonic_ms = (*context).monotonic_ms;
    (*observation).observed_wall_time_ms = (*context).wall_time_ms;
}
unsafe extern "C" fn append_value(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut scope: system_health_scope_t,
    mut metric: *const ::core::ffi::c_char,
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
    init_observation(&raw mut observation, context, scope, metric);
    system_health_observation_set_available(&raw mut observation, value, unit);
    system_health_observation_sink_append(sink, &raw mut observation);
}
unsafe extern "C" fn append_unavailable(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut scope: system_health_scope_t,
    mut metric: *const ::core::ffi::c_char,
    mut cap: system_health_capability_t,
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
    init_observation(&raw mut observation, context, scope, metric);
    system_health_observation_set_unavailable(&raw mut observation, cap);
    system_health_observation_sink_append(sink, &raw mut observation);
}
unsafe extern "C" fn scope_prefix(mut scope: system_health_scope_t) -> *const ::core::ffi::c_char {
    return if scope.0 == system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER.0 {
        b"container\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"host\0".as_ptr() as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn metric_name(
    mut out: *mut ::core::ffi::c_char,
    mut capacity: size_t,
    mut scope: system_health_scope_t,
    mut suffix: *const ::core::ffi::c_char,
) {
    snprintf(
        out,
        capacity,
        b"%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
        scope_prefix(scope),
        suffix,
    );
}
unsafe extern "C" fn finite_limit(
    mut status: cg_read_result_t,
    mut limit: *const linux_cgroup_limit_t,
) -> bool {
    return status.0 == cg_read_result_t::CG_READ_OK.0
        && !(*limit).unlimited
        && (*limit).value > 0 as uint64_t;
}
unsafe extern "C" fn emit_cpu(
    mut state: *mut linux_cgroup_state_t,
    mut sample: *const cgroup_sample_t,
    mut version: linux_cgroup_version_t,
    mut scope: system_health_scope_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut metric: [::core::ffi::c_char; 64] = [0; 64];
    let mut quota_finite: bool = (*sample).cpu_max_status.0 == cg_read_result_t::CG_READ_OK.0
        && !(*sample).cpu_max.quota_usec.unlimited;
    metric_name(
        &raw mut metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        scope,
        b"cpu.quota_cores\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if quota_finite {
        append_value(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            (*sample).cpu_max.quota_usec.value as ::core::ffi::c_double
                / (*sample).cpu_max.period_usec as ::core::ffi::c_double,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
    } else {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t(
                if (*sample).cpu_max_status.0 == cg_read_result_t::CG_READ_OK.0 {
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                } else {
                    capability((*sample).cpu_max_status).0
                },
            ),
        );
    }
    metric_name(
        &raw mut metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        scope,
        b"cpu.usage_ratio\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if (*sample).cpu_stat_status.0 != cg_read_result_t::CG_READ_OK.0
        || (*sample).cpu_max_status.0 != cg_read_result_t::CG_READ_OK.0
    {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            capability(cg_read_result_t(
                if (*sample).cpu_stat_status.0 != cg_read_result_t::CG_READ_OK.0 {
                    (*sample).cpu_stat_status.0
                } else {
                    (*sample).cpu_max_status.0
                },
            )),
        );
    } else if !quota_finite {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        );
    } else if !(*state).cpu_valid
        || (*state).version.0 != version.0
        || !(*state).cpu_quota_valid
        || (*state).cpu_max.quota_usec.value != (*sample).cpu_max.quota_usec.value
        || (*state).cpu_max.period_usec != (*sample).cpu_max.period_usec
    {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE,
        );
    } else if (*context).monotonic_ms <= (*state).sampled_monotonic_ms
        || (*sample).cpu_stat.usage_usec < (*state).cpu.usage_usec
    {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
    } else {
        let mut elapsed_ms: uint64_t = (*context)
            .monotonic_ms
            .wrapping_sub((*state).sampled_monotonic_ms);
        let mut elapsed_usec: uint64_t = 0;
        if elapsed_ms > (UINT64_MAX as uint64_t).wrapping_div(1000 as uint64_t) {
            append_unavailable(
                sink,
                context,
                scope,
                &raw mut metric as *mut ::core::ffi::c_char,
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
            );
        } else {
            elapsed_usec = elapsed_ms.wrapping_mul(1000 as uint64_t);
            let mut quota_cores: ::core::ffi::c_double = (*sample).cpu_max.quota_usec.value
                as ::core::ffi::c_double
                / (*sample).cpu_max.period_usec as ::core::ffi::c_double;
            let mut ratio: ::core::ffi::c_double = if elapsed_usec == 0 as uint64_t {
                0.0f64
            } else {
                (*sample)
                    .cpu_stat
                    .usage_usec
                    .wrapping_sub((*state).cpu.usage_usec) as ::core::ffi::c_double
                    / (elapsed_usec as ::core::ffi::c_double * quota_cores)
            };
            append_value(
                sink,
                context,
                scope,
                &raw mut metric as *mut ::core::ffi::c_char,
                ratio,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
            );
        }
    }
    metric_name(
        &raw mut metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        scope,
        b"cpu.throttled_ratio\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if (*sample).cpu_stat_status.0 != cg_read_result_t::CG_READ_OK.0 {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            capability((*sample).cpu_stat_status),
        );
    } else if !(*sample).cpu_stat.throttle_present {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        );
    } else if !(*state).cpu_valid || (*state).version.0 != version.0 {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE,
        );
    } else if (*sample).cpu_stat.periods < (*state).cpu.periods
        || (*sample).cpu_stat.throttled_periods < (*state).cpu.throttled_periods
    {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
    } else {
        let mut periods: uint64_t = (*sample)
            .cpu_stat
            .periods
            .wrapping_sub((*state).cpu.periods);
        let mut throttled: uint64_t = (*sample)
            .cpu_stat
            .throttled_periods
            .wrapping_sub((*state).cpu.throttled_periods);
        append_value(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            if periods == 0 as uint64_t {
                0.0f64
            } else {
                throttled as ::core::ffi::c_double / periods as ::core::ffi::c_double
            },
            system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        );
    };
}
unsafe extern "C" fn emit_memory(
    mut state: *mut linux_cgroup_state_t,
    mut sample: *const cgroup_sample_t,
    mut version: linux_cgroup_version_t,
    mut scope: system_health_scope_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut metric: [::core::ffi::c_char; 64] = [0; 64];
    let mut limit_finite: bool =
        finite_limit((*sample).memory_max_status, &raw const (*sample).memory_max);
    let mut names: [*const ::core::ffi::c_char; 4] = [
        b"memory.current_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"memory.limit_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"memory.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"memory.available_ratio\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut combined_status: cg_read_result_t = cg_read_result_t(
        if (*sample).memory_current_status.0 != cg_read_result_t::CG_READ_OK.0 {
            (*sample).memory_current_status.0
        } else {
            (*sample).memory_max_status.0
        },
    );
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        metric_name(
            &raw mut metric as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            scope,
            names[i],
        );
        if i == 0 as size_t && (*sample).memory_current_status.0 == cg_read_result_t::CG_READ_OK.0 {
            append_value(
                sink,
                context,
                scope,
                &raw mut metric as *mut ::core::ffi::c_char,
                (*sample).memory_current as ::core::ffi::c_double,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
            );
        } else if i == 1 as size_t && limit_finite as ::core::ffi::c_int != 0 {
            append_value(
                sink,
                context,
                scope,
                &raw mut metric as *mut ::core::ffi::c_char,
                (*sample).memory_max.value as ::core::ffi::c_double,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
            );
        } else if (i == 2 as size_t || i == 3 as size_t)
            && limit_finite as ::core::ffi::c_int != 0
            && (*sample).memory_current_status.0 == cg_read_result_t::CG_READ_OK.0
        {
            let mut headroom: uint64_t = if (*sample).memory_current >= (*sample).memory_max.value {
                0 as uint64_t
            } else {
                (*sample)
                    .memory_max
                    .value
                    .wrapping_sub((*sample).memory_current)
            };
            let mut value: ::core::ffi::c_double = if i == 2 as size_t {
                headroom as ::core::ffi::c_double
            } else {
                headroom as ::core::ffi::c_double
                    / (*sample).memory_max.value as ::core::ffi::c_double
            };
            append_value(
                sink,
                context,
                scope,
                &raw mut metric as *mut ::core::ffi::c_char,
                value,
                system_health_unit_t(
                    (if i == 3 as size_t {
                        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO.0 as ::core::ffi::c_int
                    } else {
                        system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES.0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ),
            );
        } else if (i == 2 as size_t || i == 3 as size_t)
            && (*sample).memory_current_status.0 != cg_read_result_t::CG_READ_OK.0
        {
            append_unavailable(
                sink,
                context,
                scope,
                &raw mut metric as *mut ::core::ffi::c_char,
                capability((*sample).memory_current_status),
            );
        } else {
            append_unavailable(
                sink,
                context,
                scope,
                &raw mut metric as *mut ::core::ffi::c_char,
                system_health_capability_t(
                    if combined_status.0 == cg_read_result_t::CG_READ_OK.0 {
                        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                    } else {
                        capability(combined_status).0
                    },
                ),
            );
        }
        i = i.wrapping_add(1);
    }
    metric_name(
        &raw mut metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        scope,
        b"memory.oom_kills_delta\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if (*sample).oom_status.0 != cg_read_result_t::CG_READ_OK.0 {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            capability((*sample).oom_status),
        );
    } else if !(*state).oom_valid || (*state).version.0 != version.0 {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE,
        );
    } else if (*sample).oom_kills < (*state).oom_kills {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
    } else {
        append_value(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            (*sample).oom_kills.wrapping_sub((*state).oom_kills) as ::core::ffi::c_double,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
    };
}
unsafe extern "C" fn emit_pids(
    mut sample: *const cgroup_sample_t,
    mut scope: system_health_scope_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut metric: [::core::ffi::c_char; 64] = [0; 64];
    let mut limit_finite: bool =
        finite_limit((*sample).pids_max_status, &raw const (*sample).pids_max);
    metric_name(
        &raw mut metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        scope,
        b"pids.current\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if (*sample).pids_current_status.0 == cg_read_result_t::CG_READ_OK.0 {
        append_value(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            (*sample).pids_current as ::core::ffi::c_double,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
    } else {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            capability((*sample).pids_current_status),
        );
    }
    metric_name(
        &raw mut metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        scope,
        b"pids.limit\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if limit_finite {
        append_value(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            (*sample).pids_max.value as ::core::ffi::c_double,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
    } else {
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t(
                if (*sample).pids_max_status.0 == cg_read_result_t::CG_READ_OK.0 {
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                } else {
                    capability((*sample).pids_max_status).0
                },
            ),
        );
    }
    metric_name(
        &raw mut metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        scope,
        b"pids.available_ratio\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if limit_finite as ::core::ffi::c_int != 0
        && (*sample).pids_current_status.0 == cg_read_result_t::CG_READ_OK.0
    {
        let mut remaining: uint64_t = if (*sample).pids_current >= (*sample).pids_max.value {
            0 as uint64_t
        } else {
            (*sample)
                .pids_max
                .value
                .wrapping_sub((*sample).pids_current)
        };
        append_value(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            remaining as ::core::ffi::c_double / (*sample).pids_max.value as ::core::ffi::c_double,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        );
    } else {
        let mut status: cg_read_result_t = cg_read_result_t(
            if (*sample).pids_current_status.0 != cg_read_result_t::CG_READ_OK.0 {
                (*sample).pids_current_status.0
            } else {
                (*sample).pids_max_status.0
            },
        );
        append_unavailable(
            sink,
            context,
            scope,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t(if status.0 == cg_read_result_t::CG_READ_OK.0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
            } else {
                capability(status).0
            }),
        );
    };
}
unsafe extern "C" fn collect_cgroup(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    let mut state: *mut linux_cgroup_state_t = opaque as *mut linux_cgroup_state_t;
    let mut memberships: linux_cgroup_memberships_t = linux_cgroup_memberships_t {
        version: linux_cgroup_version_t::LINUX_CGROUP_UNKNOWN,
        unified: [0; 512],
        cpu: [0; 512],
        cpuacct: [0; 512],
        memory: [0; 512],
        pids: [0; 512],
    };
    let mut sample: cgroup_sample_t = cgroup_sample_t {
        cpu_max_status: cg_read_result_t::CG_READ_OK,
        cpu_max: linux_cgroup_cpu_max_t {
            quota_usec: linux_cgroup_limit_t {
                unlimited: false,
                value: 0,
            },
            period_usec: 0,
        },
        cpu_stat_status: cg_read_result_t::CG_READ_OK,
        cpu_stat: linux_cgroup_cpu_stat_t {
            usage_usec: 0,
            throttle_present: false,
            periods: 0,
            throttled_periods: 0,
            throttled_usec: 0,
        },
        memory_current_status: cg_read_result_t::CG_READ_OK,
        memory_current: 0,
        memory_max_status: cg_read_result_t::CG_READ_OK,
        memory_max: linux_cgroup_limit_t {
            unlimited: false,
            value: 0,
        },
        oom_status: cg_read_result_t::CG_READ_OK,
        oom_kills: 0,
        pids_current_status: cg_read_result_t::CG_READ_OK,
        pids_current: 0,
        pids_max_status: cg_read_result_t::CG_READ_OK,
        pids_max: linux_cgroup_limit_t {
            unlimited: false,
            value: 0,
        },
    };
    let mut membership_status: cg_read_result_t = cg_read_result_t::CG_READ_OK;
    let mut constrained: bool = false;
    let mut scope: system_health_scope_t = system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS;
    if state.is_null()
        || context.is_null()
        || sink.is_null()
        || (*context).proc_root.is_null()
        || (*context).cgroup_root.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        &raw mut sample as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cgroup_sample_t>(),
    );
    membership_status = read_proc_membership((*context).proc_root, &raw mut memberships);
    if membership_status.0 != cg_read_result_t::CG_READ_OK.0 {
        (*state).cpu_valid = r#false != 0;
        (*state).oom_valid = r#false != 0;
        let mut cap: system_health_capability_t = capability(membership_status);
        append_unavailable(
            sink,
            context,
            system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER,
            b"container.cgroup.discovery\0".as_ptr() as *const ::core::ffi::c_char,
            cap,
        );
        return 0 as ::core::ffi::c_int;
    }
    if !same_memberships(&raw mut (*state).memberships, &raw mut memberships) {
        (*state).cpu_valid = r#false != 0;
        (*state).oom_valid = r#false != 0;
    }
    if memberships.version.0 == linux_cgroup_version_t::LINUX_CGROUP_V2.0 {
        sample_v2(
            (*context).cgroup_root,
            &raw mut memberships,
            &raw mut sample,
        );
    } else {
        sample_v1(
            (*context).cgroup_root,
            &raw mut memberships,
            &raw mut sample,
        );
    }
    constrained = sample.cpu_max_status.0 == cg_read_result_t::CG_READ_OK.0
        && !sample.cpu_max.quota_usec.unlimited
        || finite_limit(sample.memory_max_status, &raw mut sample.memory_max) as ::core::ffi::c_int
            != 0
        || finite_limit(sample.pids_max_status, &raw mut sample.pids_max) as ::core::ffi::c_int
            != 0;
    scope = system_health_scope_t(
        (if constrained as ::core::ffi::c_int != 0 {
            system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER.0 as ::core::ffi::c_int
        } else {
            system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    emit_cpu(
        state,
        &raw mut sample,
        memberships.version,
        scope,
        context,
        sink,
    );
    emit_memory(
        state,
        &raw mut sample,
        memberships.version,
        scope,
        context,
        sink,
    );
    emit_pids(&raw mut sample, scope, context, sink);
    (*state).cpu_quota_valid = sample.cpu_stat_status.0 == cg_read_result_t::CG_READ_OK.0
        && sample.cpu_max_status.0 == cg_read_result_t::CG_READ_OK.0
        && !sample.cpu_max.quota_usec.unlimited;
    if (*state).cpu_quota_valid {
        (*state).cpu_max = sample.cpu_max;
    }
    if sample.cpu_stat_status.0 == cg_read_result_t::CG_READ_OK.0 {
        (*state).cpu = sample.cpu_stat;
        (*state).sampled_monotonic_ms = (*context).monotonic_ms;
    } else {
        (*state).cpu_valid = r#false != 0;
    }
    if sample.oom_status.0 == cg_read_result_t::CG_READ_OK.0 {
        (*state).oom_kills = sample.oom_kills;
    } else {
        (*state).oom_valid = r#false != 0;
    }
    (*state).version = memberships.version;
    (*state).memberships = memberships;
    if sample.cpu_stat_status.0 == cg_read_result_t::CG_READ_OK.0 {
        (*state).cpu_valid = r#true != 0;
    }
    if sample.oom_status.0 == cg_read_result_t::CG_READ_OK.0 {
        (*state).oom_valid = r#true != 0;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_cgroup_state_init(mut state: *mut linux_cgroup_state_t) {
    if !state.is_null() {
        memset(
            state as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<linux_cgroup_state_t>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_cgroup_collector_init(
    mut collector: *mut system_health_collector_t,
    mut state: *mut linux_cgroup_state_t,
) -> bool {
    if collector.is_null() || state.is_null() {
        return r#false != 0;
    }
    linux_cgroup_state_init(state);
    memset(
        collector as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_collector_t>(),
    );
    snprintf(
        &raw mut (*collector).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"linux_cgroup\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*collector).scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER;
    (*collector).tier = system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST;
    (*collector).interval_seconds = 5 as ::core::ffi::c_uint as uint32_t;
    (*collector).stale_after_seconds = 20 as ::core::ffi::c_uint as uint32_t;
    (*collector).state = state as *mut ::core::ffi::c_void;
    (*collector).collect = Some(
        collect_cgroup
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
