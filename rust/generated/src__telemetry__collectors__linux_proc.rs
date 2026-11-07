use ::c2rust_bitfields;
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
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
}
pub type size_t = usize;
pub type __uint64_t = u64;
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
pub struct linux_proc_cpu_times_t {
    pub user: uint64_t,
    pub nice: uint64_t,
    pub system: uint64_t,
    pub idle: uint64_t,
    pub iowait: uint64_t,
    pub irq: uint64_t,
    pub softirq: uint64_t,
    pub steal: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_proc_loadavg_t {
    pub one: ::core::ffi::c_double,
    pub five: ::core::ffi::c_double,
    pub fifteen: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_proc_memory_t {
    pub total_bytes: uint64_t,
    pub available_bytes: uint64_t,
    pub swap_total_bytes: uint64_t,
    pub swap_free_bytes: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_proc_vmstat_t {
    pub major_faults: uint64_t,
    pub swap_in_pages: uint64_t,
    pub swap_out_pages: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_proc_pressure_t {
    pub some_present: bool,
    pub full_present: bool,
    pub some_avg10_ratio: ::core::ffi::c_double,
    pub full_avg10_ratio: ::core::ffi::c_double,
    pub some_total_usec: uint64_t,
    pub full_total_usec: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_proc_state_t {
    pub cpu_valid: bool,
    pub cpu: linux_proc_cpu_times_t,
    pub cpu_monotonic_ms: uint64_t,
    pub vmstat_valid: bool,
    pub vmstat: linux_proc_vmstat_t,
    pub vmstat_monotonic_ms: uint64_t,
    pub pressure_valid: [bool; 3],
    pub pressure: [linux_proc_pressure_t; 3],
    pub pressure_monotonic_ms: [uint64_t; 3],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct read_result_t(pub ::core::ffi::c_uint);
impl read_result_t {
    pub const READ_OK: Self = Self(0);
    pub const READ_UNSUPPORTED: Self = Self(1);
    pub const READ_PERMISSION: Self = Self(2);
    pub const READ_ERROR: Self = Self(3);
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
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EOVERFLOW: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
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
unsafe extern "C" fn parse_double_value(
    mut text: *const ::core::ffi::c_char,
    mut end_out: *mut *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_double,
) -> bool {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut value: ::core::ffi::c_double = 0.;
    if text.is_null() || out.is_null() {
        return r#false != 0;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    value = strtod(text, &raw mut end);
    if *__errno_location() == ERANGE
        || end == text as *mut ::core::ffi::c_char
        || value < 0.0f64
        || value.is_finite() as ::core::ffi::c_int == 0
    {
        return r#false != 0;
    }
    *out = value;
    if !end_out.is_null() {
        *end_out = end;
    }
    return r#true != 0;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_proc_parse_cpu_stat(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut linux_proc_cpu_times_t,
) -> ::core::ffi::c_int {
    let mut fields: [uint64_t; 8] = [0; 8];
    let mut cursor: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if text.is_null()
        || out.is_null()
        || strncmp(
            text,
            b"cpu\0".as_ptr() as *const ::core::ffi::c_char,
            3 as size_t,
        ) != 0 as ::core::ffi::c_int
        || *(*__ctype_b_loc())
            .offset(*text.offset(3isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    cursor = text.offset(3 as ::core::ffi::c_int as isize);
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[uint64_t; 8]>()
        .wrapping_div(::core::mem::size_of::<uint64_t>())
    {
        cursor = skip_space(cursor);
        if !parse_u64(
            cursor,
            &raw mut cursor,
            (&raw mut fields as *mut uint64_t).offset(i as isize),
        ) {
            return -1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    (*out).user = fields[0usize];
    (*out).nice = fields[1usize];
    (*out).system = fields[2usize];
    (*out).idle = fields[3usize];
    (*out).iowait = fields[4usize];
    (*out).irq = fields[5usize];
    (*out).softirq = fields[6usize];
    (*out).steal = fields[7usize];
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_proc_parse_loadavg(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut linux_proc_loadavg_t,
) -> ::core::ffi::c_int {
    let mut cursor: *const ::core::ffi::c_char = text;
    if text.is_null()
        || out.is_null()
        || !parse_double_value(cursor, &raw mut cursor, &raw mut (*out).one)
    {
        return -1 as ::core::ffi::c_int;
    }
    cursor = skip_space(cursor);
    if !parse_double_value(cursor, &raw mut cursor, &raw mut (*out).five) {
        return -1 as ::core::ffi::c_int;
    }
    cursor = skip_space(cursor);
    if !parse_double_value(
        cursor,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        &raw mut (*out).fifteen,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn meminfo_value(
    mut text: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_char,
    mut bytes_out: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut key_len: size_t = strlen(key);
    let mut line: *const ::core::ffi::c_char = text;
    while !line.is_null() && *line as ::core::ffi::c_int != 0 {
        let mut next: *const ::core::ffi::c_char = strchr(line, '\n' as ::core::ffi::c_int);
        if strncmp(line, key, key_len) == 0 as ::core::ffi::c_int
            && *line.offset(key_len as isize) as ::core::ffi::c_int == ':' as ::core::ffi::c_int
        {
            let mut kib: uint64_t = 0;
            let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut value: *const ::core::ffi::c_char = skip_space(
                line.offset(key_len as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
            if !parse_u64(value, &raw mut end, &raw mut kib)
                || kib > (UINT64_MAX as uint64_t).wrapping_div(1024 as uint64_t)
            {
                return -1 as ::core::ffi::c_int;
            }
            end = skip_space(end);
            if strncmp(
                end,
                b"kB\0".as_ptr() as *const ::core::ffi::c_char,
                2 as size_t,
            ) != 0 as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            end = end.offset(2 as ::core::ffi::c_int as isize);
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
            *bytes_out = kib.wrapping_mul(1024 as uint64_t);
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
pub unsafe extern "C" fn linux_proc_parse_meminfo(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut linux_proc_memory_t,
) -> ::core::ffi::c_int {
    let mut parsed: linux_proc_memory_t = linux_proc_memory_t {
        total_bytes: 0,
        available_bytes: 0,
        swap_total_bytes: 0,
        swap_free_bytes: 0,
    };
    let mut total: ::core::ffi::c_int = 0;
    let mut available: ::core::ffi::c_int = 0;
    let mut swap_total: ::core::ffi::c_int = 0;
    let mut swap_free: ::core::ffi::c_int = 0;
    if text.is_null() || out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        &raw mut parsed as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_proc_memory_t>(),
    );
    total = meminfo_value(
        text,
        b"MemTotal\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.total_bytes,
    );
    available = meminfo_value(
        text,
        b"MemAvailable\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.available_bytes,
    );
    swap_total = meminfo_value(
        text,
        b"SwapTotal\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.swap_total_bytes,
    );
    swap_free = meminfo_value(
        text,
        b"SwapFree\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.swap_free_bytes,
    );
    if total < 0 as ::core::ffi::c_int
        || available < 0 as ::core::ffi::c_int
        || swap_total < 0 as ::core::ffi::c_int
        || swap_free < 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if total > 0 as ::core::ffi::c_int
        || available > 0 as ::core::ffi::c_int
        || swap_total > 0 as ::core::ffi::c_int
        || swap_free > 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    if parsed.total_bytes == 0 as uint64_t
        || parsed.available_bytes > parsed.total_bytes
        || parsed.swap_free_bytes > parsed.swap_total_bytes
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
    let mut key_len: size_t = strlen(key);
    let mut line: *const ::core::ffi::c_char = text;
    while !line.is_null() && *line as ::core::ffi::c_int != 0 {
        let mut next: *const ::core::ffi::c_char = strchr(line, '\n' as ::core::ffi::c_int);
        if strncmp(line, key, key_len) == 0 as ::core::ffi::c_int
            && *(*__ctype_b_loc()).offset(*line.offset(key_len as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut value: *const ::core::ffi::c_char = skip_space(line.offset(key_len as isize));
            if !parse_u64(value, &raw mut end, out) {
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
pub unsafe extern "C" fn linux_proc_parse_vmstat(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut linux_proc_vmstat_t,
) -> ::core::ffi::c_int {
    let mut parsed: linux_proc_vmstat_t = linux_proc_vmstat_t {
        major_faults: 0,
        swap_in_pages: 0,
        swap_out_pages: 0,
    };
    let mut faults: ::core::ffi::c_int = 0;
    let mut swap_in: ::core::ffi::c_int = 0;
    let mut swap_out: ::core::ffi::c_int = 0;
    if text.is_null() || out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    faults = keyed_u64(
        text,
        b"pgmajfault\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.major_faults,
    );
    swap_in = keyed_u64(
        text,
        b"pswpin\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.swap_in_pages,
    );
    swap_out = keyed_u64(
        text,
        b"pswpout\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed.swap_out_pages,
    );
    if faults < 0 as ::core::ffi::c_int
        || swap_in < 0 as ::core::ffi::c_int
        || swap_out < 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if faults > 0 as ::core::ffi::c_int
        || swap_in > 0 as ::core::ffi::c_int
        || swap_out > 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    *out = parsed;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_pressure_line(
    mut line: *const ::core::ffi::c_char,
    mut kind: *const ::core::ffi::c_char,
    mut ratio_out: *mut ::core::ffi::c_double,
    mut total_out: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut bounded_line: [::core::ffi::c_char; 512] = [0; 512];
    let mut line_end: *const ::core::ffi::c_char = strchr(line, '\n' as ::core::ffi::c_int);
    let mut line_length: size_t = if !line_end.is_null() {
        line_end.offset_from(line) as size_t
    } else {
        strlen(line)
    };
    let mut kind_len: size_t = strlen(kind);
    let mut avg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut total: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut value_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut percent: ::core::ffi::c_double = 0.;
    if line_length >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>() {
        return -1 as ::core::ffi::c_int;
    }
    memcpy(
        &raw mut bounded_line as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        line as *const ::core::ffi::c_void,
        line_length,
    );
    bounded_line[line_length] = '\0' as ::core::ffi::c_char;
    line = &raw mut bounded_line as *mut ::core::ffi::c_char;
    if strncmp(line, kind, kind_len) != 0 as ::core::ffi::c_int
        || *(*__ctype_b_loc()).offset(
            *line.offset(kind_len as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
        ) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        return 1 as ::core::ffi::c_int;
    }
    avg = strstr(
        line.offset(kind_len as isize),
        b"avg10=\0".as_ptr() as *const ::core::ffi::c_char,
    );
    total = strstr(
        line.offset(kind_len as isize),
        b"total=\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if avg.is_null()
        || total.is_null()
        || !parse_double_value(
            avg.offset(6 as ::core::ffi::c_int as isize),
            &raw mut value_end,
            &raw mut percent,
        )
        || *(*__ctype_b_loc())
            .offset(*value_end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        || percent > 100.0f64
        || !parse_u64(
            total.offset(6 as ::core::ffi::c_int as isize),
            &raw mut value_end,
            total_out,
        )
    {
        return -1 as ::core::ffi::c_int;
    }
    while *(*__ctype_b_loc())
        .offset(*value_end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        value_end = value_end.offset(1);
    }
    if *value_end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    *ratio_out = percent / 100.0f64;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_proc_parse_pressure(
    mut text: *const ::core::ffi::c_char,
    mut out: *mut linux_proc_pressure_t,
) -> ::core::ffi::c_int {
    let mut line: *const ::core::ffi::c_char = text;
    let mut parsed: linux_proc_pressure_t = linux_proc_pressure_t {
        some_present: false,
        full_present: false,
        some_avg10_ratio: 0.,
        full_avg10_ratio: 0.,
        some_total_usec: 0,
        full_total_usec: 0,
    };
    if text.is_null() || out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        &raw mut parsed as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_proc_pressure_t>(),
    );
    while !line.is_null() && *line as ::core::ffi::c_int != 0 {
        let mut next: *const ::core::ffi::c_char = strchr(line, '\n' as ::core::ffi::c_int);
        let mut rc: ::core::ffi::c_int = parse_pressure_line(
            line,
            b"some\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut parsed.some_avg10_ratio,
            &raw mut parsed.some_total_usec,
        );
        if rc < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if rc == 0 as ::core::ffi::c_int {
            parsed.some_present = r#true != 0;
        }
        rc = parse_pressure_line(
            line,
            b"full\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut parsed.full_avg10_ratio,
            &raw mut parsed.full_total_usec,
        );
        if rc < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if rc == 0 as ::core::ffi::c_int {
            parsed.full_present = r#true != 0;
        }
        line = if !next.is_null() {
            next.offset(1 as ::core::ffi::c_int as isize)
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
    }
    if !parsed.some_present {
        return -1 as ::core::ffi::c_int;
    }
    *out = parsed;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_bounded(
    mut path: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> read_result_t {
    let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut used: size_t = 0;
    let mut saved_errno: ::core::ffi::c_int = 0;
    if path.is_null() || buffer.is_null() || capacity < 2 as size_t {
        return read_result_t::READ_ERROR;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    file = fopen(path, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if file.is_null() {
        if *__errno_location() == ENOENT || *__errno_location() == ENOTDIR {
            return read_result_t::READ_UNSUPPORTED;
        }
        if *__errno_location() == EACCES || *__errno_location() == EPERM {
            return read_result_t::READ_PERMISSION;
        }
        return read_result_t::READ_ERROR;
    }
    used = fread(
        buffer as *mut ::core::ffi::c_void,
        1 as size_t,
        capacity.wrapping_sub(1 as size_t),
        file,
    ) as size_t;
    saved_errno = if ferror(file) != 0 {
        *__errno_location()
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
        return read_result_t::READ_ERROR;
    }
    *buffer.offset(used as isize) = '\0' as ::core::ffi::c_char;
    return read_result_t::READ_OK;
}
unsafe extern "C" fn read_capability(mut result: read_result_t) -> system_health_capability_t {
    match result {
        read_result_t::READ_UNSUPPORTED => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        }
        read_result_t::READ_PERMISSION => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
        }
        read_result_t::READ_ERROR => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        }
        read_result_t::READ_OK => {
            return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        }
        _ => {}
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
}
unsafe extern "C" fn read_proc_file(
    mut root: *const ::core::ffi::c_char,
    mut relative: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> read_result_t {
    let mut path: [::core::ffi::c_char; 512] = [0; 512];
    let mut length: ::core::ffi::c_int = 0;
    if root.is_null() || relative.is_null() {
        return read_result_t::READ_ERROR;
    }
    length = snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        root,
        relative,
    );
    if length < 0 as ::core::ffi::c_int
        || length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        return read_result_t::READ_ERROR;
    }
    return read_bounded(&raw mut path as *mut ::core::ffi::c_char, buffer, capacity);
}
unsafe extern "C" fn init_observation(
    mut observation: *mut system_health_observation_t,
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
        b"host\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*observation).scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST;
    (*observation).sampled_monotonic_ms = (*context).monotonic_ms;
    (*observation).observed_wall_time_ms = (*context).wall_time_ms;
}
unsafe extern "C" fn append_value(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
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
    init_observation(&raw mut observation, context, metric);
    system_health_observation_set_available(&raw mut observation, value, unit);
    system_health_observation_sink_append(sink, &raw mut observation);
}
unsafe extern "C" fn append_unavailable(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut metric: *const ::core::ffi::c_char,
    mut capability: system_health_capability_t,
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
    init_observation(&raw mut observation, context, metric);
    system_health_observation_set_unavailable(&raw mut observation, capability);
    system_health_observation_sink_append(sink, &raw mut observation);
}
unsafe extern "C" fn add_checked(
    mut left: uint64_t,
    mut right: uint64_t,
    mut out: *mut uint64_t,
) -> bool {
    if (UINT64_MAX as uint64_t).wrapping_sub(left) < right {
        return r#false != 0;
    }
    *out = left.wrapping_add(right);
    return r#true != 0;
}
unsafe extern "C" fn cpu_totals(
    mut cpu: *const linux_proc_cpu_times_t,
    mut busy: *mut uint64_t,
    mut idle: *mut uint64_t,
    mut total: *mut uint64_t,
) -> bool {
    let mut busy_value: uint64_t = 0 as uint64_t;
    let mut idle_value: uint64_t = 0;
    let busy_fields: [uint64_t; 6] = [
        (*cpu).user,
        (*cpu).nice,
        (*cpu).system,
        (*cpu).irq,
        (*cpu).softirq,
        (*cpu).steal,
    ];
    if !add_checked((*cpu).idle, (*cpu).iowait, &raw mut idle_value) {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[uint64_t; 6]>()
        .wrapping_div(::core::mem::size_of::<uint64_t>())
    {
        if !add_checked(busy_value, busy_fields[i], &raw mut busy_value) {
            return r#false != 0;
        }
        i = i.wrapping_add(1);
    }
    if !add_checked(busy_value, idle_value, total) {
        return r#false != 0;
    }
    *busy = busy_value;
    *idle = idle_value;
    return r#true != 0;
}
unsafe extern "C" fn collect_cpu(
    mut state: *mut linux_proc_state_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut text: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut current: linux_proc_cpu_times_t = linux_proc_cpu_times_t {
        user: 0,
        nice: 0,
        system: 0,
        idle: 0,
        iowait: 0,
        irq: 0,
        softirq: 0,
        steal: 0,
    };
    let mut read_result: read_result_t = read_proc_file(
        (*context).proc_root,
        b"stat\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
    );
    if read_result.0 != read_result_t::READ_OK.0
        || linux_proc_parse_cpu_stat(&raw mut text as *mut ::core::ffi::c_char, &raw mut current)
            != 0 as ::core::ffi::c_int
    {
        append_unavailable(
            sink,
            context,
            b"host.cpu.busy_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_capability_t(if read_result.0 == read_result_t::READ_OK.0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
            } else {
                read_capability(read_result).0
            }),
        );
        return;
    }
    if !(*state).cpu_valid {
        append_unavailable(
            sink,
            context,
            b"host.cpu.busy_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE,
        );
    } else {
        let mut current_busy: uint64_t = 0;
        let mut current_idle: uint64_t = 0;
        let mut current_total: uint64_t = 0;
        let mut previous_busy: uint64_t = 0;
        let mut previous_idle: uint64_t = 0;
        let mut previous_total: uint64_t = 0;
        let mut valid: bool = (*context).monotonic_ms > (*state).cpu_monotonic_ms
            && cpu_totals(
                &raw mut current,
                &raw mut current_busy,
                &raw mut current_idle,
                &raw mut current_total,
            ) as ::core::ffi::c_int
                != 0
            && cpu_totals(
                &raw mut (*state).cpu,
                &raw mut previous_busy,
                &raw mut previous_idle,
                &raw mut previous_total,
            ) as ::core::ffi::c_int
                != 0
            && current_busy >= previous_busy
            && current_idle >= previous_idle
            && current_total > previous_total;
        if valid {
            let mut ratio: ::core::ffi::c_double = current_busy.wrapping_sub(previous_busy)
                as ::core::ffi::c_double
                / current_total.wrapping_sub(previous_total) as ::core::ffi::c_double;
            append_value(
                sink,
                context,
                b"host.cpu.busy_ratio\0".as_ptr() as *const ::core::ffi::c_char,
                ratio,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
            );
        } else {
            append_unavailable(
                sink,
                context,
                b"host.cpu.busy_ratio\0".as_ptr() as *const ::core::ffi::c_char,
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
            );
        }
    }
    (*state).cpu = current;
    (*state).cpu_monotonic_ms = (*context).monotonic_ms;
    (*state).cpu_valid = r#true != 0;
}
unsafe extern "C" fn collect_load(
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut text: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut load: linux_proc_loadavg_t = linux_proc_loadavg_t {
        one: 0.,
        five: 0.,
        fifteen: 0.,
    };
    let mut result: read_result_t = read_proc_file(
        (*context).proc_root,
        b"loadavg\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
    );
    if result.0 != read_result_t::READ_OK.0
        || linux_proc_parse_loadavg(&raw mut text as *mut ::core::ffi::c_char, &raw mut load)
            != 0 as ::core::ffi::c_int
    {
        let mut capability: system_health_capability_t =
            system_health_capability_t(if result.0 == read_result_t::READ_OK.0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
            } else {
                read_capability(result).0
            });
        append_unavailable(
            sink,
            context,
            b"host.load.1\0".as_ptr() as *const ::core::ffi::c_char,
            capability,
        );
        append_unavailable(
            sink,
            context,
            b"host.load.5\0".as_ptr() as *const ::core::ffi::c_char,
            capability,
        );
        append_unavailable(
            sink,
            context,
            b"host.load.15\0".as_ptr() as *const ::core::ffi::c_char,
            capability,
        );
        return;
    }
    append_value(
        sink,
        context,
        b"host.load.1\0".as_ptr() as *const ::core::ffi::c_char,
        load.one,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
    append_value(
        sink,
        context,
        b"host.load.5\0".as_ptr() as *const ::core::ffi::c_char,
        load.five,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
    append_value(
        sink,
        context,
        b"host.load.15\0".as_ptr() as *const ::core::ffi::c_char,
        load.fifteen,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
    );
}
unsafe extern "C" fn collect_memory(
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut text: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut memory: linux_proc_memory_t = linux_proc_memory_t {
        total_bytes: 0,
        available_bytes: 0,
        swap_total_bytes: 0,
        swap_free_bytes: 0,
    };
    let mut result: read_result_t = read_proc_file(
        (*context).proc_root,
        b"meminfo\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
    );
    let mut parse_result: ::core::ffi::c_int = if result.0 == read_result_t::READ_OK.0 {
        linux_proc_parse_meminfo(&raw mut text as *mut ::core::ffi::c_char, &raw mut memory)
    } else {
        -1 as ::core::ffi::c_int
    };
    if result.0 != read_result_t::READ_OK.0 || parse_result != 0 as ::core::ffi::c_int {
        let mut capability: system_health_capability_t =
            system_health_capability_t(if result.0 != read_result_t::READ_OK.0 {
                read_capability(result).0
            } else {
                (if parse_result > 0 as ::core::ffi::c_int {
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                        as ::core::ffi::c_int
                } else {
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
                        as ::core::ffi::c_int
                }) as ::core::ffi::c_uint
            });
        append_unavailable(
            sink,
            context,
            b"host.memory.total_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            capability,
        );
        append_unavailable(
            sink,
            context,
            b"host.memory.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            capability,
        );
        append_unavailable(
            sink,
            context,
            b"host.memory.available_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            capability,
        );
        append_unavailable(
            sink,
            context,
            b"host.swap.used_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            capability,
        );
        return;
    }
    append_value(
        sink,
        context,
        b"host.memory.total_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        memory.total_bytes as ::core::ffi::c_double,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
    );
    append_value(
        sink,
        context,
        b"host.memory.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        memory.available_bytes as ::core::ffi::c_double,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
    );
    append_value(
        sink,
        context,
        b"host.memory.available_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        if memory.total_bytes == 0 as uint64_t {
            0.0f64
        } else {
            memory.available_bytes as ::core::ffi::c_double
                / memory.total_bytes as ::core::ffi::c_double
        },
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
    );
    append_value(
        sink,
        context,
        b"host.swap.used_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        memory.swap_total_bytes.wrapping_sub(memory.swap_free_bytes) as ::core::ffi::c_double,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
    );
}
unsafe extern "C" fn append_counter_delta(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut metric: *const ::core::ffi::c_char,
    mut previous_valid: bool,
    mut previous: uint64_t,
    mut current: uint64_t,
    mut time_valid: bool,
) {
    if !previous_valid {
        append_unavailable(
            sink,
            context,
            metric,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE,
        );
    } else if !time_valid || current < previous {
        append_unavailable(
            sink,
            context,
            metric,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
    } else {
        append_value(
            sink,
            context,
            metric,
            current.wrapping_sub(previous) as ::core::ffi::c_double,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
    };
}
unsafe extern "C" fn collect_vmstat(
    mut state: *mut linux_proc_state_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut text: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut current: linux_proc_vmstat_t = linux_proc_vmstat_t {
        major_faults: 0,
        swap_in_pages: 0,
        swap_out_pages: 0,
    };
    let mut result: read_result_t = read_proc_file(
        (*context).proc_root,
        b"vmstat\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
    );
    let mut parse_result: ::core::ffi::c_int = if result.0 == read_result_t::READ_OK.0 {
        linux_proc_parse_vmstat(&raw mut text as *mut ::core::ffi::c_char, &raw mut current)
    } else {
        -1 as ::core::ffi::c_int
    };
    if result.0 != read_result_t::READ_OK.0 || parse_result != 0 as ::core::ffi::c_int {
        let mut capability: system_health_capability_t =
            system_health_capability_t(if result.0 != read_result_t::READ_OK.0 {
                read_capability(result).0
            } else {
                (if parse_result > 0 as ::core::ffi::c_int {
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                        as ::core::ffi::c_int
                } else {
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
                        as ::core::ffi::c_int
                }) as ::core::ffi::c_uint
            });
        append_unavailable(
            sink,
            context,
            b"host.vm.major_faults_delta\0".as_ptr() as *const ::core::ffi::c_char,
            capability,
        );
        append_unavailable(
            sink,
            context,
            b"host.vm.swap_in_pages_delta\0".as_ptr() as *const ::core::ffi::c_char,
            capability,
        );
        append_unavailable(
            sink,
            context,
            b"host.vm.swap_out_pages_delta\0".as_ptr() as *const ::core::ffi::c_char,
            capability,
        );
        return;
    }
    append_counter_delta(
        sink,
        context,
        b"host.vm.major_faults_delta\0".as_ptr() as *const ::core::ffi::c_char,
        (*state).vmstat_valid,
        (*state).vmstat.major_faults,
        current.major_faults,
        (*context).monotonic_ms > (*state).vmstat_monotonic_ms,
    );
    append_counter_delta(
        sink,
        context,
        b"host.vm.swap_in_pages_delta\0".as_ptr() as *const ::core::ffi::c_char,
        (*state).vmstat_valid,
        (*state).vmstat.swap_in_pages,
        current.swap_in_pages,
        (*context).monotonic_ms > (*state).vmstat_monotonic_ms,
    );
    append_counter_delta(
        sink,
        context,
        b"host.vm.swap_out_pages_delta\0".as_ptr() as *const ::core::ffi::c_char,
        (*state).vmstat_valid,
        (*state).vmstat.swap_out_pages,
        current.swap_out_pages,
        (*context).monotonic_ms > (*state).vmstat_monotonic_ms,
    );
    (*state).vmstat = current;
    (*state).vmstat_monotonic_ms = (*context).monotonic_ms;
    (*state).vmstat_valid = r#true != 0;
}
unsafe extern "C" fn collect_pressure_one(
    mut state: *mut linux_proc_state_t,
    mut index: size_t,
    mut name: *const ::core::ffi::c_char,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) {
    let mut text: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut relative: [::core::ffi::c_char; 64] = [0; 64];
    let mut metric: [::core::ffi::c_char; 64] = [0; 64];
    let mut current: linux_proc_pressure_t = linux_proc_pressure_t {
        some_present: false,
        full_present: false,
        some_avg10_ratio: 0.,
        full_avg10_ratio: 0.,
        some_total_usec: 0,
        full_total_usec: 0,
    };
    let mut result: read_result_t = read_result_t::READ_OK;
    snprintf(
        &raw mut relative as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"pressure/%s\0".as_ptr() as *const ::core::ffi::c_char,
        name,
    );
    result = read_proc_file(
        (*context).proc_root,
        &raw mut relative as *mut ::core::ffi::c_char,
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16384]>(),
    );
    if result.0 != read_result_t::READ_OK.0
        || linux_proc_parse_pressure(&raw mut text as *mut ::core::ffi::c_char, &raw mut current)
            != 0 as ::core::ffi::c_int
    {
        let mut capability: system_health_capability_t =
            system_health_capability_t(if result.0 == read_result_t::READ_OK.0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
            } else {
                read_capability(result).0
            });
        snprintf(
            &raw mut metric as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"host.pressure.%s.some_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        append_unavailable(
            sink,
            context,
            &raw mut metric as *mut ::core::ffi::c_char,
            capability,
        );
        snprintf(
            &raw mut metric as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"host.pressure.%s.some_seconds_delta\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        append_unavailable(
            sink,
            context,
            &raw mut metric as *mut ::core::ffi::c_char,
            capability,
        );
        if strcmp(name, b"cpu\0".as_ptr() as *const ::core::ffi::c_char) != 0 as ::core::ffi::c_int
        {
            snprintf(
                &raw mut metric as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"host.pressure.%s.full_ratio\0".as_ptr() as *const ::core::ffi::c_char,
                name,
            );
            append_unavailable(
                sink,
                context,
                &raw mut metric as *mut ::core::ffi::c_char,
                capability,
            );
        }
        return;
    }
    snprintf(
        &raw mut metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"host.pressure.%s.some_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        name,
    );
    append_value(
        sink,
        context,
        &raw mut metric as *mut ::core::ffi::c_char,
        current.some_avg10_ratio,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
    );
    snprintf(
        &raw mut metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"host.pressure.%s.some_seconds_delta\0".as_ptr() as *const ::core::ffi::c_char,
        name,
    );
    if !(*state).pressure_valid[index] {
        append_unavailable(
            sink,
            context,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE,
        );
    } else if (*context).monotonic_ms <= (*state).pressure_monotonic_ms[index]
        || current.some_total_usec < (*state).pressure[index].some_total_usec
    {
        append_unavailable(
            sink,
            context,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        );
    } else {
        append_value(
            sink,
            context,
            &raw mut metric as *mut ::core::ffi::c_char,
            current
                .some_total_usec
                .wrapping_sub((*state).pressure[index].some_total_usec)
                as ::core::ffi::c_double
                / 1000000.0f64,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_SECONDS,
        );
    }
    if current.full_present {
        snprintf(
            &raw mut metric as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"host.pressure.%s.full_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        append_value(
            sink,
            context,
            &raw mut metric as *mut ::core::ffi::c_char,
            current.full_avg10_ratio,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_RATIO,
        );
    } else if strcmp(name, b"cpu\0".as_ptr() as *const ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut metric as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"host.pressure.%s.full_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        append_unavailable(
            sink,
            context,
            &raw mut metric as *mut ::core::ffi::c_char,
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        );
    }
    (*state).pressure[index] = current;
    (*state).pressure_monotonic_ms[index] = (*context).monotonic_ms;
    (*state).pressure_valid[index] = r#true != 0;
}
unsafe extern "C" fn collect_proc(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    let mut state: *mut linux_proc_state_t = opaque as *mut linux_proc_state_t;
    if state.is_null() || context.is_null() || sink.is_null() || (*context).proc_root.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    collect_cpu(state, context, sink);
    collect_load(context, sink);
    collect_memory(context, sink);
    collect_vmstat(state, context, sink);
    collect_pressure_one(
        state,
        0 as size_t,
        b"cpu\0".as_ptr() as *const ::core::ffi::c_char,
        context,
        sink,
    );
    collect_pressure_one(
        state,
        1 as size_t,
        b"memory\0".as_ptr() as *const ::core::ffi::c_char,
        context,
        sink,
    );
    collect_pressure_one(
        state,
        2 as size_t,
        b"io\0".as_ptr() as *const ::core::ffi::c_char,
        context,
        sink,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_proc_state_init(mut state: *mut linux_proc_state_t) {
    if !state.is_null() {
        memset(
            state as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<linux_proc_state_t>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_proc_collector_init(
    mut collector: *mut system_health_collector_t,
    mut state: *mut linux_proc_state_t,
) -> bool {
    if collector.is_null() || state.is_null() {
        return r#false != 0;
    }
    linux_proc_state_init(state);
    memset(
        collector as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_collector_t>(),
    );
    snprintf(
        &raw mut (*collector).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"linux_proc\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*collector).scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST;
    (*collector).tier = system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST;
    (*collector).interval_seconds = 5 as ::core::ffi::c_uint as uint32_t;
    (*collector).stale_after_seconds = 20 as ::core::ffi::c_uint as uint32_t;
    (*collector).state = state as *mut ::core::ffi::c_void;
    (*collector).collect = Some(
        collect_proc
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_observation_sink_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_collect_fn;
    return r#true != 0;
}
