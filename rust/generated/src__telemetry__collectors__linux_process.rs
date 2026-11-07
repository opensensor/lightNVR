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
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn getrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *mut rlimit,
    ) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __rlim_t = ::core::ffi::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_process_value_t {
    pub value: uint64_t,
    pub capability: system_health_capability_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_process_sample_request_t {
    pub status_path: *const ::core::ffi::c_char,
    pub fd_directory_path: *const ::core::ffi::c_char,
    pub pids_max_path: *const ::core::ffi::c_char,
    pub fd_scan_limit: size_t,
    pub nofile_limit_supplied: bool,
    pub nofile_limit_unlimited: bool,
    pub nofile_limit: uint64_t,
    pub nproc_limit_supplied: bool,
    pub nproc_limit_unlimited: bool,
    pub nproc_limit: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_process_sample_t {
    pub rss_bytes: linux_process_value_t,
    pub thread_count: linux_process_value_t,
    pub open_fd_count: linux_process_value_t,
    pub effective_fd_limit: linux_process_value_t,
    pub effective_pid_limit: linux_process_value_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_process_collector_state_t {
    pub status_path: [::core::ffi::c_char; 512],
    pub fd_directory_path: [::core::ffi::c_char; 512],
    pub pids_max_path: [::core::ffi::c_char; 512],
    pub fd_scan_limit: size_t,
}
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct __rlimit_resource(pub ::core::ffi::c_uint);
impl __rlimit_resource {
    pub const RLIMIT_CPU: Self = Self(0);
    pub const RLIMIT_FSIZE: Self = Self(1);
    pub const RLIMIT_DATA: Self = Self(2);
    pub const RLIMIT_STACK: Self = Self(3);
    pub const RLIMIT_CORE: Self = Self(4);
    pub const __RLIMIT_RSS: Self = Self(5);
    pub const RLIMIT_NOFILE: Self = Self(7);
    pub const __RLIMIT_OFILE: Self = Self(7);
    pub const RLIMIT_AS: Self = Self(9);
    pub const __RLIMIT_NPROC: Self = Self(6);
    pub const __RLIMIT_MEMLOCK: Self = Self(8);
    pub const __RLIMIT_LOCKS: Self = Self(10);
    pub const __RLIMIT_SIGPENDING: Self = Self(11);
    pub const __RLIMIT_MSGQUEUE: Self = Self(12);
    pub const __RLIMIT_NICE: Self = Self(13);
    pub const __RLIMIT_RTPRIO: Self = Self(14);
    pub const __RLIMIT_RTTIME: Self = Self(15);
    pub const __RLIMIT_NLIMITS: Self = Self(16);
    pub const __RLIM_NLIMITS: Self = Self(16);
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
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const LINUX_PROCESS_DEFAULT_FD_SCAN_LIMIT: ::core::ffi::c_uint = 65536 as ::core::ffi::c_uint;
pub const RLIM_INFINITY: __rlim_t = -1 as ::core::ffi::c_int as __rlim_t;
pub const PROCESS_STATUS_MAX_BYTES: ::core::ffi::c_uint =
    (64 as ::core::ffi::c_uint).wrapping_mul(1024 as ::core::ffi::c_uint);
pub const PROCESS_SMALL_FILE_MAX_BYTES: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
unsafe extern "C" fn capability_for_errno(
    mut error_code: ::core::ffi::c_int,
) -> system_health_capability_t {
    if error_code == EACCES || error_code == EPERM {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
}
unsafe extern "C" fn value_unavailable(
    mut value: *mut linux_process_value_t,
    mut capability: system_health_capability_t,
) {
    (*value).value = 0 as uint64_t;
    (*value).capability = capability;
}
unsafe extern "C" fn value_available(mut value: *mut linux_process_value_t, mut number: uint64_t) {
    (*value).value = number;
    (*value).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
}
unsafe extern "C" fn parse_decimal(
    mut begin: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut value_out: *mut uint64_t,
    mut after_out: *mut *const ::core::ffi::c_char,
) -> bool {
    let mut cursor: *const ::core::ffi::c_char = begin;
    let mut value: uint64_t = 0 as uint64_t;
    let mut found: bool = r#false != 0;
    while cursor < end
        && *(*__ctype_b_loc())
            .offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        cursor = cursor.offset(1);
    }
    while cursor < end
        && *(*__ctype_b_loc())
            .offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        let mut digit: ::core::ffi::c_uint =
            (*cursor as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as ::core::ffi::c_uint;
        if value
            > (UINT64_MAX as uint64_t)
                .wrapping_sub(digit as uint64_t)
                .wrapping_div(10 as uint64_t)
        {
            return r#false != 0;
        }
        value = value
            .wrapping_mul(10 as uint64_t)
            .wrapping_add(digit as uint64_t);
        cursor = cursor.offset(1);
        found = r#true != 0;
    }
    if !found {
        return r#false != 0;
    }
    *value_out = value;
    if !after_out.is_null() {
        *after_out = cursor;
    }
    return r#true != 0;
}
unsafe extern "C" fn remaining_space(
    mut cursor: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> bool {
    while cursor < end {
        if *(*__ctype_b_loc())
            .offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_process_parse_status_text(
    mut text: *const ::core::ffi::c_char,
    mut text_length: size_t,
    mut sample: *mut linux_process_sample_t,
) -> ::core::ffi::c_int {
    if text.is_null() || sample.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    value_unavailable(
        &raw mut (*sample).rss_bytes,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
    );
    value_unavailable(
        &raw mut (*sample).thread_count,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
    );
    let mut offset: size_t = 0 as size_t;
    let mut rss_seen: bool = r#false != 0;
    let mut threads_seen: bool = r#false != 0;
    while offset < text_length {
        let mut line: *const ::core::ffi::c_char = text.offset(offset as isize);
        let mut line_end: *const ::core::ffi::c_char = memchr(
            line as *const ::core::ffi::c_void,
            '\n' as ::core::ffi::c_int,
            text_length.wrapping_sub(offset),
        ) as *const ::core::ffi::c_char;
        if line_end.is_null() {
            line_end = text.offset(text_length as isize);
        }
        if line_end.offset_from(line) as size_t >= 6 as size_t
            && memcmp(
                line as *const ::core::ffi::c_void,
                b"VmRSS:\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                6 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut kibibytes: uint64_t = 0 as uint64_t;
            let mut after: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut valid: bool = !rss_seen
                && parse_decimal(
                    line.offset(6 as ::core::ffi::c_int as isize),
                    line_end,
                    &raw mut kibibytes,
                    &raw mut after,
                ) as ::core::ffi::c_int
                    != 0;
            while valid as ::core::ffi::c_int != 0
                && after < line_end
                && *(*__ctype_b_loc())
                    .offset(*after as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    != 0
            {
                after = after.offset(1);
            }
            valid = valid as ::core::ffi::c_int != 0
                && line_end.offset_from(after) as size_t >= 2 as size_t
                && *after.offset(0isize) as ::core::ffi::c_int == 'k' as ::core::ffi::c_int
                && *after.offset(1isize) as ::core::ffi::c_int == 'B' as ::core::ffi::c_int
                && remaining_space(after.offset(2 as ::core::ffi::c_int as isize), line_end)
                    as ::core::ffi::c_int
                    != 0
                && kibibytes <= (UINT64_MAX as uint64_t).wrapping_div(1024 as uint64_t);
            if valid {
                value_available(
                    &raw mut (*sample).rss_bytes,
                    kibibytes.wrapping_mul(1024 as uint64_t),
                );
            } else {
                value_unavailable(
                    &raw mut (*sample).rss_bytes,
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
                );
            }
            rss_seen = r#true != 0;
        } else if line_end.offset_from(line) as size_t >= 8 as size_t
            && memcmp(
                line as *const ::core::ffi::c_void,
                b"Threads:\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                8 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut threads: uint64_t = 0 as uint64_t;
            let mut after_0: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            let mut valid_0: bool = !threads_seen
                && parse_decimal(
                    line.offset(8 as ::core::ffi::c_int as isize),
                    line_end,
                    &raw mut threads,
                    &raw mut after_0,
                ) as ::core::ffi::c_int
                    != 0
                && remaining_space(after_0, line_end) as ::core::ffi::c_int != 0;
            if valid_0 {
                value_available(&raw mut (*sample).thread_count, threads);
            } else {
                value_unavailable(
                    &raw mut (*sample).thread_count,
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
                );
            }
            threads_seen = r#true != 0;
        }
        offset = line_end.offset_from(text) as size_t;
        if offset < text_length {
            offset = offset.wrapping_add(1);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_status(
    mut path: *const ::core::ffi::c_char,
    mut sample: *mut linux_process_sample_t,
) -> ::core::ffi::c_int {
    if path.is_null() {
        value_unavailable(
            &raw mut (*sample).rss_bytes,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
        value_unavailable(
            &raw mut (*sample).thread_count,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut input: *mut FILE = fopen(path, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
    if input.is_null() {
        let mut capability: system_health_capability_t = capability_for_errno(*__errno_location());
        value_unavailable(&raw mut (*sample).rss_bytes, capability);
        value_unavailable(&raw mut (*sample).thread_count, capability);
        return -1 as ::core::ffi::c_int;
    }
    let mut contents: [::core::ffi::c_char; 65537] = [0; 65537];
    let mut length: size_t = fread(
        &raw mut contents as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        1 as size_t,
        PROCESS_STATUS_MAX_BYTES.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
        input,
    ) as size_t;
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    let mut failed: bool =
        ferror(input) != 0 as ::core::ffi::c_int || length > PROCESS_STATUS_MAX_BYTES as size_t;
    fclose(input);
    if failed {
        let mut capability_0: system_health_capability_t =
            system_health_capability_t(if length > PROCESS_STATUS_MAX_BYTES as size_t {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
            } else {
                capability_for_errno(saved_errno).0
            });
        value_unavailable(&raw mut (*sample).rss_bytes, capability_0);
        value_unavailable(&raw mut (*sample).thread_count, capability_0);
        return -1 as ::core::ffi::c_int;
    }
    return linux_process_parse_status_text(
        &raw mut contents as *mut ::core::ffi::c_char,
        length,
        sample,
    );
}
unsafe extern "C" fn count_open_fds(
    mut path: *const ::core::ffi::c_char,
    mut scan_limit: size_t,
    mut value: *mut linux_process_value_t,
) -> ::core::ffi::c_int {
    if path.is_null() {
        value_unavailable(
            value,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut directory: *mut DIR = opendir(path);
    if directory.is_null() {
        value_unavailable(value, capability_for_errno(*__errno_location()));
        return -1 as ::core::ffi::c_int;
    }
    let mut count: uint64_t = 0 as uint64_t;
    let mut exceeded: bool = r#false != 0;
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(directory);
        if entry.is_null() {
            break;
        }
        if strcmp(
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            b".\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                b"..\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            continue;
        }
        if count >= scan_limit as uint64_t {
            exceeded = r#true != 0;
            break;
        } else {
            count = count.wrapping_add(1);
        }
    }
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    closedir(directory);
    if exceeded as ::core::ffi::c_int != 0 || saved_errno != 0 as ::core::ffi::c_int {
        value_unavailable(
            value,
            system_health_capability_t(if saved_errno != 0 as ::core::ffi::c_int {
                capability_for_errno(saved_errno).0
            } else {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
            }),
        );
        return -1 as ::core::ffi::c_int;
    }
    value_available(value, count);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_pid_limit(
    mut path: *const ::core::ffi::c_char,
    mut value: *mut linux_process_value_t,
    mut unlimited: *mut bool,
) -> ::core::ffi::c_int {
    *unlimited = r#false != 0;
    if path.is_null() {
        value_unavailable(
            value,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut input: *mut FILE = fopen(path, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
    if input.is_null() {
        let mut capability: system_health_capability_t = system_health_capability_t(
            if *__errno_location() == ENOENT || *__errno_location() == ENOTDIR {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
            } else {
                capability_for_errno(*__errno_location()).0
            },
        );
        value_unavailable(value, capability);
        return -1 as ::core::ffi::c_int;
    }
    let mut contents: [::core::ffi::c_char; 129] = [0; 129];
    let mut length: size_t = fread(
        &raw mut contents as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        1 as size_t,
        PROCESS_SMALL_FILE_MAX_BYTES.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
        input,
    ) as size_t;
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    let mut failed: bool =
        ferror(input) != 0 as ::core::ffi::c_int || length > PROCESS_SMALL_FILE_MAX_BYTES as size_t;
    fclose(input);
    if failed {
        value_unavailable(
            value,
            system_health_capability_t(if length > PROCESS_SMALL_FILE_MAX_BYTES as size_t {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
            } else {
                capability_for_errno(saved_errno).0
            }),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut begin: *const ::core::ffi::c_char = &raw mut contents as *mut ::core::ffi::c_char;
    let mut end: *const ::core::ffi::c_char =
        (&raw mut contents as *mut ::core::ffi::c_char).offset(length as isize);
    while begin < end
        && *(*__ctype_b_loc()).offset(*begin as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        begin = begin.offset(1);
    }
    let mut trimmed_end: *const ::core::ffi::c_char = end;
    while trimmed_end > begin
        && *(*__ctype_b_loc()).offset(
            *trimmed_end.offset(-1isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
        ) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        trimmed_end = trimmed_end.offset(-1);
    }
    if trimmed_end.offset_from(begin) as size_t == 3 as size_t
        && memcmp(
            begin as *const ::core::ffi::c_void,
            b"max\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            3 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        *unlimited = r#true != 0;
        value_unavailable(
            value,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut parsed: uint64_t = 0 as uint64_t;
    let mut after: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !parse_decimal(begin, trimmed_end, &raw mut parsed, &raw mut after) || after != trimmed_end {
        value_unavailable(
            value,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
        return -1 as ::core::ffi::c_int;
    }
    value_available(value, parsed);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_rlimit_value(
    mut resource: ::core::ffi::c_int,
    mut supplied: bool,
    mut unlimited: bool,
    mut supplied_value: uint64_t,
    mut value: *mut linux_process_value_t,
    mut is_unlimited: *mut bool,
) {
    *is_unlimited = r#false != 0;
    if supplied {
        if unlimited {
            *is_unlimited = r#true != 0;
            value_unavailable(
                value,
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
            );
        } else {
            value_available(value, supplied_value);
        }
        return;
    }
    let mut limit: rlimit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    if getrlimit(
        __rlimit_resource(resource as ::core::ffi::c_uint),
        &raw mut limit,
    ) != 0 as ::core::ffi::c_int
    {
        value_unavailable(value, capability_for_errno(*__errno_location()));
    } else if limit.rlim_cur == RLIM_INFINITY {
        *is_unlimited = r#true != 0;
        value_unavailable(
            value,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        );
    } else {
        value_available(value, limit.rlim_cur as uint64_t);
    };
}
unsafe extern "C" fn choose_effective_pid_limit(
    mut nproc: *const linux_process_value_t,
    mut nproc_unlimited: bool,
    mut cgroup: *const linux_process_value_t,
    mut cgroup_unlimited: bool,
    mut effective: *mut linux_process_value_t,
) {
    if (*nproc).capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        && (*cgroup).capability.0
            == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    {
        value_available(
            effective,
            if (*nproc).value < (*cgroup).value {
                (*nproc).value
            } else {
                (*cgroup).value
            },
        );
        return;
    }
    if (*cgroup).capability.0
        == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED.0
        || (*cgroup).capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
    {
        value_unavailable(effective, (*cgroup).capability);
        return;
    }
    if (*nproc).capability.0
        == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED.0
        || (*nproc).capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
    {
        value_unavailable(effective, (*nproc).capability);
        return;
    }
    if (*nproc).capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 {
        value_available(effective, (*nproc).value);
        return;
    }
    if (*cgroup).capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 {
        value_available(effective, (*cgroup).value);
        return;
    }
    value_unavailable(
        effective,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_process_sample(
    mut request: *const linux_process_sample_request_t,
    mut sample: *mut linux_process_sample_t,
) -> ::core::ffi::c_int {
    if request.is_null() || sample.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        sample as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_process_sample_t>(),
    );
    read_status((*request).status_path, sample);
    count_open_fds(
        (*request).fd_directory_path,
        if (*request).fd_scan_limit != 0 as size_t {
            (*request).fd_scan_limit
        } else {
            LINUX_PROCESS_DEFAULT_FD_SCAN_LIMIT as size_t
        },
        &raw mut (*sample).open_fd_count,
    );
    let mut fd_unlimited: bool = r#false != 0;
    read_rlimit_value(
        __rlimit_resource::RLIMIT_NOFILE.0 as ::core::ffi::c_int,
        (*request).nofile_limit_supplied,
        (*request).nofile_limit_unlimited,
        (*request).nofile_limit,
        &raw mut (*sample).effective_fd_limit,
        &raw mut fd_unlimited,
    );
    let mut nproc: linux_process_value_t = linux_process_value_t {
        value: 0,
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
    };
    let mut nproc_unlimited: bool = r#false != 0;
    read_rlimit_value(
        __rlimit_resource::__RLIMIT_NPROC.0 as ::core::ffi::c_int,
        (*request).nproc_limit_supplied,
        (*request).nproc_limit_unlimited,
        (*request).nproc_limit,
        &raw mut nproc,
        &raw mut nproc_unlimited,
    );
    let mut cgroup: linux_process_value_t = linux_process_value_t {
        value: 0,
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
    };
    let mut cgroup_unlimited: bool = r#false != 0;
    read_pid_limit(
        (*request).pids_max_path,
        &raw mut cgroup,
        &raw mut cgroup_unlimited,
    );
    choose_effective_pid_limit(
        &raw mut nproc,
        nproc_unlimited,
        &raw mut cgroup,
        cgroup_unlimited,
        &raw mut (*sample).effective_pid_limit,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn make_path(
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
    mut root: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
) -> bool {
    let mut length: ::core::ffi::c_int = snprintf(
        output,
        output_size,
        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        root,
        suffix,
    );
    return length >= 0 as ::core::ffi::c_int && (length as size_t) < output_size;
}
unsafe extern "C" fn prepare_observation(
    mut observation: *mut system_health_observation_t,
    mut metric: *const ::core::ffi::c_char,
    mut context: *const system_health_collect_context_t,
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
        b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*observation).scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS;
    (*observation).sampled_monotonic_ms = (*context).monotonic_ms;
    (*observation).observed_wall_time_ms = (*context).wall_time_ms;
}
unsafe extern "C" fn emit_value(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut metric: *const ::core::ffi::c_char,
    mut value: *const linux_process_value_t,
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
    prepare_observation(&raw mut observation, metric, context);
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
unsafe extern "C" fn emit_ratio(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut metric: *const ::core::ffi::c_char,
    mut used: *const linux_process_value_t,
    mut limit: *const linux_process_value_t,
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
    prepare_observation(&raw mut observation, metric, context);
    if (*used).capability.0 != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 {
        system_health_observation_set_unavailable(&raw mut observation, (*used).capability);
    } else if (*limit).capability.0
        != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    {
        system_health_observation_set_unavailable(&raw mut observation, (*limit).capability);
    } else {
        let mut ratio: ::core::ffi::c_double = if (*limit).value == 0 as uint64_t {
            if (*used).value == 0 as uint64_t {
                0.0f64
            } else {
                1.0f64
            }
        } else {
            (*used).value as ::core::ffi::c_double / (*limit).value as ::core::ffi::c_double
        };
        system_health_observation_set_available(
            &raw mut observation,
            ratio,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        );
    }
    system_health_observation_sink_append(sink, &raw mut observation);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_process_collect(
    mut state: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    if context.is_null() || sink.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut configuration: *mut linux_process_collector_state_t =
        state as *mut linux_process_collector_state_t;
    let mut status_path: [::core::ffi::c_char; 512] = [0; 512];
    let mut fd_path: [::core::ffi::c_char; 512] = [0; 512];
    let mut pids_path: [::core::ffi::c_char; 512] = [0; 512];
    let mut proc_root: *const ::core::ffi::c_char = if !(*context).proc_root.is_null() {
        (*context).proc_root
    } else {
        b"/proc\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut cgroup_root: *const ::core::ffi::c_char = if !(*context).cgroup_root.is_null() {
        (*context).cgroup_root
    } else {
        b"/sys/fs/cgroup\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut status: *const ::core::ffi::c_char = if !configuration.is_null()
        && (*configuration).status_path[0usize] as ::core::ffi::c_int != 0
    {
        &raw mut (*configuration).status_path as *mut ::core::ffi::c_char
    } else if make_path(
        &raw mut status_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        proc_root,
        b"self/status\0".as_ptr() as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
    {
        &raw mut status_path as *mut ::core::ffi::c_char
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    let mut fds: *const ::core::ffi::c_char = if !configuration.is_null()
        && (*configuration).fd_directory_path[0usize] as ::core::ffi::c_int != 0
    {
        &raw mut (*configuration).fd_directory_path as *mut ::core::ffi::c_char
    } else if make_path(
        &raw mut fd_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        proc_root,
        b"self/fd\0".as_ptr() as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
    {
        &raw mut fd_path as *mut ::core::ffi::c_char
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    let mut pids: *const ::core::ffi::c_char = if !configuration.is_null()
        && (*configuration).pids_max_path[0usize] as ::core::ffi::c_int != 0
    {
        &raw mut (*configuration).pids_max_path as *mut ::core::ffi::c_char
    } else if make_path(
        &raw mut pids_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        cgroup_root,
        b"pids.max\0".as_ptr() as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
    {
        &raw mut pids_path as *mut ::core::ffi::c_char
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    let mut request: linux_process_sample_request_t = linux_process_sample_request_t {
        status_path: ::core::ptr::null::<::core::ffi::c_char>(),
        fd_directory_path: ::core::ptr::null::<::core::ffi::c_char>(),
        pids_max_path: ::core::ptr::null::<::core::ffi::c_char>(),
        fd_scan_limit: 0,
        nofile_limit_supplied: false,
        nofile_limit_unlimited: false,
        nofile_limit: 0,
        nproc_limit_supplied: false,
        nproc_limit_unlimited: false,
        nproc_limit: 0,
    };
    memset(
        &raw mut request as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_process_sample_request_t>(),
    );
    request.status_path = status;
    request.fd_directory_path = fds;
    request.pids_max_path = pids;
    request.fd_scan_limit = if !configuration.is_null() && (*configuration).fd_scan_limit != 0 {
        (*configuration).fd_scan_limit
    } else {
        LINUX_PROCESS_DEFAULT_FD_SCAN_LIMIT as size_t
    };
    let mut sample: linux_process_sample_t = linux_process_sample_t {
        rss_bytes: linux_process_value_t {
            value: 0,
            capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        },
        thread_count: linux_process_value_t {
            value: 0,
            capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        },
        open_fd_count: linux_process_value_t {
            value: 0,
            capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        },
        effective_fd_limit: linux_process_value_t {
            value: 0,
            capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        },
        effective_pid_limit: linux_process_value_t {
            value: 0,
            capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        },
    };
    linux_process_sample(&raw mut request, &raw mut sample);
    emit_value(
        sink,
        context,
        b"process.rss_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sample.rss_bytes,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
    );
    emit_value(
        sink,
        context,
        b"process.threads\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sample.thread_count,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
    emit_value(
        sink,
        context,
        b"process.open_fds\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sample.open_fd_count,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
    emit_value(
        sink,
        context,
        b"process.fd_limit\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sample.effective_fd_limit,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
    emit_ratio(
        sink,
        context,
        b"process.fd_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sample.open_fd_count,
        &raw mut sample.effective_fd_limit,
    );
    emit_value(
        sink,
        context,
        b"process.pid_limit\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sample.effective_pid_limit,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
    emit_ratio(
        sink,
        context,
        b"process.pid_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sample.thread_count,
        &raw mut sample.effective_pid_limit,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_process_collector_init(
    mut collector: *mut system_health_collector_t,
    mut state: *mut linux_process_collector_state_t,
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
        b"linux_process\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*collector).scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS;
    (*collector).tier = system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_NORMAL;
    (*collector).interval_seconds = 15 as ::core::ffi::c_uint as uint32_t;
    (*collector).stale_after_seconds = 45 as ::core::ffi::c_uint as uint32_t;
    (*collector).state = state as *mut ::core::ffi::c_void;
    (*collector).collect = Some(
        linux_process_collect
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
