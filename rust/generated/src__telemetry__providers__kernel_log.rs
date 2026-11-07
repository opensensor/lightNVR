unsafe extern "C" {
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
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
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
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type ssize_t = isize;
pub type int64_t = i64;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const KERNEL_LOG_FILESYSTEM_REMOUNT: Self = Self(0);
    pub const KERNEL_LOG_BLOCK_IO_ERROR: Self = Self(1);
    pub const KERNEL_LOG_MACHINE_CHECK: Self = Self(2);
    pub const KERNEL_LOG_THERMAL_SHUTDOWN: Self = Self(3);
    pub const KERNEL_LOG_OOM_KILL: Self = Self(4);
    pub const KERNEL_LOG_CATEGORY_COUNT: Self = Self(5);
}
pub type kernel_log_open_fn = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, ::core::ffi::c_int) -> ::core::ffi::c_int,
>;
pub type kernel_log_read_fn =
    Option<unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void, size_t) -> ssize_t>;
pub type kernel_log_close_fn =
    Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernel_log_ops_t {
    pub open_log: kernel_log_open_fn,
    pub read_log: kernel_log_read_fn,
    pub close_log: kernel_log_close_fn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernel_log_state_t {
    pub descriptor: ::core::ffi::c_int,
    pub path: [::core::ffi::c_char; 256],
    pub ops: kernel_log_ops_t,
    pub capability: system_health_capability_t,
    pub last_sequence: uint64_t,
    pub sequence_valid: bool,
    pub recent_hashes: [uint64_t; 64],
    pub recent_count: size_t,
    pub recent_next: size_t,
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
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const KERNEL_LOG_READ_BUFFER: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
pub const KERNEL_LOG_READS_PER_CYCLE: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const KERNEL_LOG_DEDUPE_SLOTS: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -128 as ::core::ffi::c_int && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENODEV: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
static mut category_metrics: [*const ::core::ffi::c_char; 5] = [
    b"kernel.filesystem_remount_delta\0".as_ptr() as *const ::core::ffi::c_char,
    b"kernel.block_io_error_delta\0".as_ptr() as *const ::core::ffi::c_char,
    b"kernel.machine_check_delta\0".as_ptr() as *const ::core::ffi::c_char,
    b"kernel.thermal_shutdown_delta\0".as_ptr() as *const ::core::ffi::c_char,
    b"kernel.oom_kill_delta\0".as_ptr() as *const ::core::ffi::c_char,
];
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
unsafe extern "C" fn default_read(
    mut descriptor: ::core::ffi::c_int,
    mut buffer: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> ssize_t {
    return read(descriptor, buffer, size);
}
unsafe extern "C" fn default_close(mut descriptor: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return close(descriptor);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_log_state_init(
    mut state: *mut kernel_log_state_t,
    mut path: *const ::core::ffi::c_char,
) {
    if state.is_null() {
        return;
    }
    memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<kernel_log_state_t>(),
    );
    (*state).descriptor = -1 as ::core::ffi::c_int;
    (*state).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    snprintf(
        &raw mut (*state).path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        if !path.is_null() && *path.offset(0isize) as ::core::ffi::c_int != 0 {
            path
        } else {
            b"/dev/kmsg\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    (*state).ops.open_log = Some(
        default_open
            as unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ) as kernel_log_open_fn;
    (*state).ops.read_log = Some(
        default_read
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
                size_t,
            ) -> ssize_t,
    ) as kernel_log_read_fn;
    (*state).ops.close_log =
        Some(default_close as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int)
            as kernel_log_close_fn;
}
unsafe extern "C" fn contains(
    mut text: *const ::core::ffi::c_char,
    mut needle: *const ::core::ffi::c_char,
) -> bool {
    return !strstr(text, needle).is_null();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_log_classify_line(
    mut line: *const ::core::ffi::c_char,
    mut length: size_t,
    mut matches: *mut bool,
    mut sequence: *mut uint64_t,
    mut sequence_valid: *mut bool,
) -> ::core::ffi::c_int {
    if line.is_null()
        || matches.is_null()
        || sequence.is_null()
        || sequence_valid.is_null()
        || length == 0 as size_t
        || length >= KERNEL_LOG_READ_BUFFER as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        matches as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<bool>().wrapping_mul(
            C2Rust_Unnamed::KERNEL_LOG_CATEGORY_COUNT.0 as ::core::ffi::c_int as size_t,
        ),
    );
    *sequence = 0 as uint64_t;
    *sequence_valid = r#false != 0;
    let mut end: *const ::core::ffi::c_char = line.offset(length as isize);
    let mut first_comma: *const ::core::ffi::c_char = memchr(
        line as *const ::core::ffi::c_void,
        ',' as ::core::ffi::c_int,
        length,
    ) as *const ::core::ffi::c_char;
    if !first_comma.is_null() {
        let mut cursor: *const ::core::ffi::c_char =
            first_comma.offset(1 as ::core::ffi::c_int as isize);
        if cursor < end
            && *(*__ctype_b_loc())
                .offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            let mut parsed: uint64_t = 0 as uint64_t;
            let mut valid: bool = r#true != 0;
            while cursor < end
                && *(*__ctype_b_loc())
                    .offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed_0::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    != 0
            {
                let mut digit: ::core::ffi::c_uint = (*cursor as ::core::ffi::c_int
                    - '0' as ::core::ffi::c_int)
                    as ::core::ffi::c_uint;
                if parsed
                    > (UINT64_MAX as uint64_t)
                        .wrapping_sub(digit as uint64_t)
                        .wrapping_div(10 as uint64_t)
                {
                    valid = r#false != 0;
                    break;
                } else {
                    parsed = parsed
                        .wrapping_mul(10 as uint64_t)
                        .wrapping_add(digit as uint64_t);
                    cursor = cursor.offset(1);
                }
            }
            if valid as ::core::ffi::c_int != 0
                && cursor < end
                && *cursor as ::core::ffi::c_int == ',' as ::core::ffi::c_int
            {
                *sequence = parsed;
                *sequence_valid = r#true != 0;
            }
        }
    }
    let mut message: *const ::core::ffi::c_char = memchr(
        line as *const ::core::ffi::c_void,
        ';' as ::core::ffi::c_int,
        length,
    ) as *const ::core::ffi::c_char;
    if !message.is_null() {
        message = message.offset(1);
    } else {
        message = line;
    }
    let mut message_length: size_t = end.offset_from(message) as size_t;
    let mut normalized: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut index: size_t = 0 as size_t;
    while index < message_length {
        normalized[index] = ({
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                if 0 as ::core::ffi::c_int != 0 {
                    let mut __c: ::core::ffi::c_int = *message.offset(index as isize)
                        as ::core::ffi::c_uchar
                        as ::core::ffi::c_int;
                    __res = (if __c < -128 as ::core::ffi::c_int || __c > 255 as ::core::ffi::c_int
                    {
                        __c as __int32_t
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    }) as ::core::ffi::c_int;
                } else {
                    __res = tolower(*message.offset(index as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*message.offset(index as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
            }
            __res
        }) as ::core::ffi::c_char;
        index = index.wrapping_add(1);
    }
    normalized[message_length] = '\0' as ::core::ffi::c_char;
    *matches
        .offset(C2Rust_Unnamed::KERNEL_LOG_FILESYSTEM_REMOUNT.0 as ::core::ffi::c_int as isize) =
        contains(
            &raw mut normalized as *mut ::core::ffi::c_char,
            b"remount\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            && (contains(
                &raw mut normalized as *mut ::core::ffi::c_char,
                b"read-only\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
                || contains(
                    &raw mut normalized as *mut ::core::ffi::c_char,
                    b"readonly\0".as_ptr() as *const ::core::ffi::c_char,
                ) as ::core::ffi::c_int
                    != 0)
            || contains(
                &raw mut normalized as *mut ::core::ffi::c_char,
                b"filesystem has been set read-only\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0;
    *matches.offset(C2Rust_Unnamed::KERNEL_LOG_BLOCK_IO_ERROR.0 as ::core::ffi::c_int as isize) =
        contains(
            &raw mut normalized as *mut ::core::ffi::c_char,
            b"i/o error\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            || contains(
                &raw mut normalized as *mut ::core::ffi::c_char,
                b"blk_update_request\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || contains(
                &raw mut normalized as *mut ::core::ffi::c_char,
                b"critical medium error\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0;
    *matches.offset(C2Rust_Unnamed::KERNEL_LOG_MACHINE_CHECK.0 as ::core::ffi::c_int as isize) =
        contains(
            &raw mut normalized as *mut ::core::ffi::c_char,
            b"machine check\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            || contains(
                &raw mut normalized as *mut ::core::ffi::c_char,
                b"mce:\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || contains(
                &raw mut normalized as *mut ::core::ffi::c_char,
                b"hardware error\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0;
    *matches.offset(C2Rust_Unnamed::KERNEL_LOG_THERMAL_SHUTDOWN.0 as ::core::ffi::c_int as isize) =
        contains(
            &raw mut normalized as *mut ::core::ffi::c_char,
            b"thermal shutdown\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            || contains(
                &raw mut normalized as *mut ::core::ffi::c_char,
                b"critical temperature reached\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || contains(
                &raw mut normalized as *mut ::core::ffi::c_char,
                b"overheat\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
                && contains(
                    &raw mut normalized as *mut ::core::ffi::c_char,
                    b"shutdown\0".as_ptr() as *const ::core::ffi::c_char,
                ) as ::core::ffi::c_int
                    != 0;
    *matches.offset(C2Rust_Unnamed::KERNEL_LOG_OOM_KILL.0 as ::core::ffi::c_int as isize) = contains(
        &raw mut normalized as *mut ::core::ffi::c_char,
        b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
    )
        as ::core::ffi::c_int
        != 0
        || contains(
            &raw mut normalized as *mut ::core::ffi::c_char,
            b"oom-killer\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        || contains(
            &raw mut normalized as *mut ::core::ffi::c_char,
            b"oom_reaper\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        || contains(
            &raw mut normalized as *mut ::core::ffi::c_char,
            b"killed process\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_log_discover(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut inventory: *mut system_health_provider_inventory_t,
) -> ::core::ffi::c_int {
    let mut state: *mut kernel_log_state_t = opaque as *mut kernel_log_state_t;
    if state.is_null()
        || inventory.is_null()
        || (*state).ops.open_log.is_none()
        || (*state).ops.read_log.is_none()
        || (*state).ops.close_log.is_none()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        inventory as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_provider_inventory_t>(),
    );
    if (*state).descriptor < 0 as ::core::ffi::c_int {
        (*state).descriptor = (*state).ops.open_log.expect("non-null function pointer")(
            &raw mut (*state).path as *mut ::core::ffi::c_char,
            O_RDONLY | O_NONBLOCK | O_CLOEXEC,
        );
        (*state).capability =
            system_health_capability_t(if (*state).descriptor >= 0 as ::core::ffi::c_int {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
            } else {
                capability_from_errno(*__errno_location()).0
            });
    }
    snprintf(
        &raw mut (*(&raw mut (*inventory).resources as *mut system_health_provider_resource_t)
            .offset(0isize))
        .id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        b"kernel\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*inventory).resources[0usize].scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST;
    (*inventory).resources[0usize].capability = (*state).capability;
    (*inventory).count = 1 as size_t;
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
    observation.scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST;
    observation.sampled_monotonic_ms = (*context).monotonic_ms;
    observation.observed_wall_time_ms = (*context).wall_time_ms;
    if valid {
        system_health_observation_set_available(&raw mut observation, value, unit);
    } else {
        system_health_observation_set_unavailable(&raw mut observation, capability);
    }
    system_health_observation_sink_append(sink, &raw mut observation);
}
unsafe extern "C" fn line_hash(
    mut line: *const ::core::ffi::c_char,
    mut length: size_t,
) -> uint64_t {
    let mut message: *const ::core::ffi::c_char = memchr(
        line as *const ::core::ffi::c_void,
        ';' as ::core::ffi::c_int,
        length,
    ) as *const ::core::ffi::c_char;
    if !message.is_null() {
        message = message.offset(1);
        length = line.offset(length as isize).offset_from(message) as size_t;
        line = message;
    }
    let mut hash: uint64_t = 1469598103934665603 as uint64_t;
    let mut index: size_t = 0 as size_t;
    while index < length {
        hash ^= *line.offset(index as isize) as ::core::ffi::c_uchar as uint64_t;
        hash = (hash as ::core::ffi::c_ulong).wrapping_mul(1099511628211 as ::core::ffi::c_ulong)
            as uint64_t;
        index = index.wrapping_add(1);
    }
    return if hash == 0 as uint64_t {
        1 as uint64_t
    } else {
        hash
    };
}
unsafe extern "C" fn hash_seen(mut state: *mut kernel_log_state_t, mut hash: uint64_t) -> bool {
    let mut index: size_t = 0 as size_t;
    while index < (*state).recent_count {
        if (*state).recent_hashes[index] == hash {
            return r#true != 0;
        }
        index = index.wrapping_add(1);
    }
    (*state).recent_hashes[(*state).recent_next] = hash;
    (*state).recent_next = (*state)
        .recent_next
        .wrapping_add(1 as size_t)
        .wrapping_rem(KERNEL_LOG_DEDUPE_SLOTS as size_t);
    if (*state).recent_count < KERNEL_LOG_DEDUPE_SLOTS as size_t {
        (*state).recent_count = (*state).recent_count.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn process_line(
    mut state: *mut kernel_log_state_t,
    mut line: *const ::core::ffi::c_char,
    mut length: size_t,
    mut counts: *mut uint64_t,
) {
    while length > 0 as size_t
        && (*line.offset(length.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == '\n' as ::core::ffi::c_int
            || *line.offset(length.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                == '\r' as ::core::ffi::c_int)
    {
        length = length.wrapping_sub(1);
    }
    if length == 0 as size_t || length >= KERNEL_LOG_READ_BUFFER as size_t {
        return;
    }
    let mut matches: [bool; 5] = [false; 5];
    let mut sequence: uint64_t = 0 as uint64_t;
    let mut sequence_valid: bool = r#false != 0;
    if kernel_log_classify_line(
        line,
        length,
        &raw mut matches as *mut bool,
        &raw mut sequence,
        &raw mut sequence_valid,
    ) != 0 as ::core::ffi::c_int
    {
        return;
    }
    if sequence_valid {
        if (*state).sequence_valid as ::core::ffi::c_int != 0 && sequence <= (*state).last_sequence
        {
            return;
        }
        (*state).last_sequence = sequence;
        (*state).sequence_valid = r#true != 0;
    } else if hash_seen(state, line_hash(line, length)) {
        return;
    }
    let mut category: size_t = 0 as size_t;
    while category < C2Rust_Unnamed::KERNEL_LOG_CATEGORY_COUNT.0 as ::core::ffi::c_int as size_t {
        if matches[category] as ::core::ffi::c_int != 0
            && *counts.offset(category as isize) < UINT64_MAX as uint64_t
        {
            *counts.offset(category as isize) = (*counts.offset(category as isize)).wrapping_add(1);
        }
        category = category.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_log_collect(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    let mut state: *mut kernel_log_state_t = opaque as *mut kernel_log_state_t;
    if state.is_null()
        || context.is_null()
        || sink.is_null()
        || (*state).ops.read_log.is_none()
        || (*state).ops.close_log.is_none()
    {
        return -1 as ::core::ffi::c_int;
    }
    emit(
        sink,
        context,
        b"hardware.provider.visible\0".as_ptr() as *const ::core::ffi::c_char,
        b"kernel_log\0".as_ptr() as *const ::core::ffi::c_char,
        (*state).capability,
        (*state).capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0,
        1.0f64,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    if (*state).descriptor < 0 as ::core::ffi::c_int
        || (*state).capability.0 != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    {
        let mut category: size_t = 0 as size_t;
        while category < C2Rust_Unnamed::KERNEL_LOG_CATEGORY_COUNT.0 as ::core::ffi::c_int as size_t
        {
            emit(
                sink,
                context,
                category_metrics[category],
                b"kernel\0".as_ptr() as *const ::core::ffi::c_char,
                (*state).capability,
                r#false != 0,
                0.0f64,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
            );
            category = category.wrapping_add(1);
        }
        return 0 as ::core::ffi::c_int;
    }
    let mut counts: [uint64_t; 5] = [0 as uint64_t, 0, 0, 0, 0];
    let mut buffer: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut attempt: size_t = 0 as size_t;
    while attempt < KERNEL_LOG_READS_PER_CYCLE as size_t {
        let mut length: ssize_t = (*state).ops.read_log.expect("non-null function pointer")(
            (*state).descriptor,
            &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>().wrapping_sub(1 as size_t),
        );
        if length < 0 as ssize_t {
            if *__errno_location() == EAGAIN || *__errno_location() == EWOULDBLOCK {
                break;
            }
            (*state).capability = capability_from_errno(*__errno_location());
            (*state).ops.close_log.expect("non-null function pointer")((*state).descriptor);
            (*state).descriptor = -1 as ::core::ffi::c_int;
            break;
        } else {
            if length == 0 as ssize_t {
                break;
            }
            buffer[length as usize] = '\0' as ::core::ffi::c_char;
            let mut start: size_t = 0 as size_t;
            let mut index: size_t = 0 as size_t;
            while index <= length as size_t {
                if index == length as size_t
                    || buffer[index] as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                {
                    process_line(
                        state,
                        (&raw mut buffer as *mut ::core::ffi::c_char).offset(start as isize),
                        index.wrapping_sub(start),
                        &raw mut counts as *mut uint64_t,
                    );
                    start = index.wrapping_add(1 as size_t);
                }
                index = index.wrapping_add(1);
            }
            attempt = attempt.wrapping_add(1);
        }
    }
    let mut valid: bool =
        (*state).capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0;
    let mut category_0: size_t = 0 as size_t;
    while category_0 < C2Rust_Unnamed::KERNEL_LOG_CATEGORY_COUNT.0 as ::core::ffi::c_int as size_t {
        emit(
            sink,
            context,
            category_metrics[category_0],
            b"kernel\0".as_ptr() as *const ::core::ffi::c_char,
            (*state).capability,
            valid,
            counts[category_0] as ::core::ffi::c_double,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        );
        category_0 = category_0.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_log_destroy(mut opaque: *mut ::core::ffi::c_void) {
    let mut state: *mut kernel_log_state_t = opaque as *mut kernel_log_state_t;
    if state.is_null() {
        return;
    }
    if (*state).descriptor >= 0 as ::core::ffi::c_int && (*state).ops.close_log.is_some() {
        (*state).ops.close_log.expect("non-null function pointer")((*state).descriptor);
    }
    (*state).descriptor = -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_log_provider_init(
    mut provider: *mut system_health_provider_t,
    mut state: *mut kernel_log_state_t,
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
        b"kernel_log\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*provider).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    (*provider).state = state as *mut ::core::ffi::c_void;
    (*provider).discover = Some(
        kernel_log_discover
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_provider_inventory_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_provider_discover_fn;
    (*provider).collect = Some(
        kernel_log_collect
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_observation_sink_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_collect_fn;
    (*provider).destroy =
        Some(kernel_log_destroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
            as system_health_collector_destroy_fn;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
