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
    unsafe fn fgetc(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn realpath(
        __name: *const ::core::ffi::c_char,
        __resolved: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
pub struct linux_thermal_sensor_state_t {
    pub id: [::core::ffi::c_char; 64],
    pub limit_metric: [::core::ffi::c_char; 64],
    pub present: bool,
    pub seen: bool,
    pub missing_cycles: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_thermal_state_t {
    pub sensors: [linux_thermal_sensor_state_t; 64],
    pub sensor_count: size_t,
    pub resources_dropped_total: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_thermal_sensor_sample_t {
    pub id: [::core::ffi::c_char; 64],
    pub temperature_capability: system_health_capability_t,
    pub temperature_valid: bool,
    pub temperature_celsius: ::core::ffi::c_double,
    pub limit_capability: system_health_capability_t,
    pub limit_valid: bool,
    pub limit_is_critical: bool,
    pub limit_celsius: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_thermal_result_t {
    pub sensors: [linux_thermal_sensor_sample_t; 32],
    pub sensor_count: size_t,
    pub resources_dropped: size_t,
    pub capability: system_health_capability_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thermal_numeric_t {
    pub valid: bool,
    pub capability: system_health_capability_t,
    pub value: ::core::ffi::c_double,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct thermal_source_t(pub ::core::ffi::c_uint);
impl thermal_source_t {
    pub const THERMAL_SOURCE_ZONE: Self = Self(0);
    pub const THERMAL_SOURCE_HWMON: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thermal_candidate_t {
    pub source: thermal_source_t,
    pub entry: [::core::ffi::c_char; 64],
    pub channel: ::core::ffi::c_uint,
    pub id: [::core::ffi::c_char; 64],
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
pub const SYSTEM_HEALTH_MAX_SENSORS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_ID_LENGTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const LINUX_THERMAL_STATE_CAPACITY: ::core::ffi::c_uint =
    SYSTEM_HEALTH_MAX_SENSORS.wrapping_mul(2 as ::core::ffi::c_uint);
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
pub const THERMAL_CANDIDATE_CAPACITY: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
pub const THERMAL_TEXT_LENGTH: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
pub const THERMAL_TRIP_LIMIT: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
unsafe extern "C" fn format_path(
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
    mut format: *const ::core::ffi::c_char,
    mut root: *const ::core::ffi::c_char,
    mut entry: *const ::core::ffi::c_char,
    mut index: ::core::ffi::c_uint,
) -> bool {
    if output.is_null() || output_size == 0 as size_t || format.is_null() || root.is_null() {
        return r#false != 0;
    }
    let mut written: ::core::ffi::c_int = snprintf(
        output,
        output_size,
        format,
        root,
        if !entry.is_null() {
            entry
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        index,
    );
    return written >= 0 as ::core::ffi::c_int && (written as size_t) < output_size;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_thermal_capability_from_errno(
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
unsafe extern "C" fn read_text(
    mut path: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut capability: *mut system_health_capability_t,
) -> bool {
    if path.is_null() || output.is_null() || capability.is_null() {
        return r#false != 0;
    }
    *output.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut file: *mut FILE = fopen(path, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    if file.is_null() {
        *capability = linux_thermal_capability_from_errno(*__errno_location());
        return r#false != 0;
    }
    let mut length: size_t = fread(
        output as *mut ::core::ffi::c_void,
        1 as size_t,
        THERMAL_TEXT_LENGTH.wrapping_sub(1 as ::core::ffi::c_uint) as size_t,
        file,
    ) as size_t;
    if ferror(file) != 0 {
        let mut saved_errno: ::core::ffi::c_int = *__errno_location();
        fclose(file);
        *capability = linux_thermal_capability_from_errno(saved_errno);
        *output.offset(0isize) = '\0' as ::core::ffi::c_char;
        return r#false != 0;
    }
    if length == THERMAL_TEXT_LENGTH.wrapping_sub(1 as ::core::ffi::c_uint) as size_t
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
unsafe extern "C" fn read_millidegrees(mut path: *const ::core::ffi::c_char) -> thermal_numeric_t {
    let mut result: thermal_numeric_t = thermal_numeric_t {
        valid: r#false != 0,
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
        value: 0.0f64,
    };
    let mut text: [::core::ffi::c_char; 128] = [0; 128];
    if !read_text(
        path,
        &raw mut text as *mut ::core::ffi::c_char,
        &raw mut result.capability,
    ) {
        return result;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut raw: ::core::ffi::c_longlong = strtoll(
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
    if raw < -273150 as ::core::ffi::c_longlong || raw > 2000000 as ::core::ffi::c_longlong {
        result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    result.valid = r#true != 0;
    result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    result.value = raw as ::core::ffi::c_double / 1000.0f64;
    return result;
}
unsafe extern "C" fn normalize_component(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) {
    if output.is_null() || output_size == 0 as size_t {
        return;
    }
    let mut used: size_t = 0 as size_t;
    let mut separator: bool = r#false != 0;
    if !input.is_null() {
        let mut cursor: *const ::core::ffi::c_uchar = input as *const ::core::ffi::c_uchar;
        while *cursor as ::core::ffi::c_int != 0 && used.wrapping_add(1 as size_t) < output_size {
            let mut character: ::core::ffi::c_uchar = *cursor;
            if *(*__ctype_b_loc()).offset(character as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            {
                let c2rust_fresh1 = used;
                used = used.wrapping_add(1);
                *output.offset(c2rust_fresh1 as isize) = ({
                    let mut __res: ::core::ffi::c_int = 0;
                    if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                        if 0 as ::core::ffi::c_int != 0 {
                            let mut __c: ::core::ffi::c_int = character as ::core::ffi::c_int;
                            __res = (if __c < -128 as ::core::ffi::c_int
                                || __c > 255 as ::core::ffi::c_int
                            {
                                __c as __int32_t
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            }) as ::core::ffi::c_int;
                        } else {
                            __res = tolower(character as ::core::ffi::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(character as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int;
                    }
                    __res
                }) as ::core::ffi::c_char;
                separator = r#false != 0;
            } else if !separator && used > 0 as size_t {
                let c2rust_fresh2 = used;
                used = used.wrapping_add(1);
                *output.offset(c2rust_fresh2 as isize) = '_' as ::core::ffi::c_char;
                separator = r#true != 0;
            }
            cursor = cursor.offset(1);
        }
    }
    while used > 0 as size_t
        && *output.offset(used.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == '_' as ::core::ffi::c_int
    {
        used = used.wrapping_sub(1);
    }
    if used == 0 as size_t && output_size > 1 as size_t {
        memcpy(
            output as *mut ::core::ffi::c_void,
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            7 as size_t,
        );
        used = 7 as size_t;
    }
    *output.offset(used as isize) = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn fnv1a_update(
    mut hash: uint32_t,
    mut text: *const ::core::ffi::c_char,
) -> uint32_t {
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
unsafe extern "C" fn hwmon_identity_hash(
    mut sys_root: *const ::core::ffi::c_char,
    mut directory: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut label: *const ::core::ffi::c_char,
    mut channel: ::core::ffi::c_uint,
) -> uint32_t {
    let mut identity: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut material: *const ::core::ffi::c_char = directory;
    if !realpath(directory, &raw mut identity as *mut ::core::ffi::c_char).is_null() {
        material = &raw mut identity as *mut ::core::ffi::c_char;
        let mut root_length: size_t = if !sys_root.is_null() {
            strlen(sys_root)
        } else {
            0 as size_t
        };
        if root_length > 0 as size_t
            && strncmp(
                &raw mut identity as *mut ::core::ffi::c_char,
                sys_root,
                root_length,
            ) == 0 as ::core::ffi::c_int
        {
            material = (&raw mut identity as *mut ::core::ffi::c_char).offset(root_length as isize);
        }
        let mut hwmon_tail: *mut ::core::ffi::c_char = strstr(
            material as *mut ::core::ffi::c_char,
            b"/hwmon/hwmon\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !hwmon_tail.is_null() {
            *hwmon_tail = '\0' as ::core::ffi::c_char;
        }
    }
    let mut channel_text: [::core::ffi::c_char; 16] = [0; 16];
    snprintf(
        &raw mut channel_text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        channel,
    );
    let mut hash: uint32_t = 2166136261 as uint32_t;
    hash = fnv1a_update(hash, material);
    hash = fnv1a_update(hash, b"|\0".as_ptr() as *const ::core::ffi::c_char);
    hash = fnv1a_update(hash, name);
    hash = fnv1a_update(hash, b"|\0".as_ptr() as *const ::core::ffi::c_char);
    hash = fnv1a_update(hash, label);
    hash = fnv1a_update(hash, b"|\0".as_ptr() as *const ::core::ffi::c_char);
    return fnv1a_update(hash, &raw mut channel_text as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn suffix_is_digits(
    mut name: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> bool {
    if name.is_null() || prefix.is_null() {
        return r#false != 0;
    }
    let mut prefix_length: size_t = strlen(prefix);
    if strncmp(name, prefix, prefix_length) != 0 as ::core::ffi::c_int
        || *name.offset(prefix_length as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar =
        name.offset(prefix_length as isize) as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn candidate_consider(
    mut candidates: *mut thermal_candidate_t,
    mut candidate_count: *mut size_t,
    mut total_candidates: *mut size_t,
    mut candidate: *const thermal_candidate_t,
) {
    if candidates.is_null()
        || candidate_count.is_null()
        || total_candidates.is_null()
        || candidate.is_null()
    {
        return;
    }
    *total_candidates = (*total_candidates).wrapping_add(1);
    let mut position: size_t = *candidate_count;
    if *candidate_count < THERMAL_CANDIDATE_CAPACITY as size_t {
        *candidate_count = (*candidate_count).wrapping_add(1);
    } else {
        if strcmp(
            &raw const (*candidate).id as *const ::core::ffi::c_char,
            &raw mut (*candidates
                .offset(THERMAL_CANDIDATE_CAPACITY.wrapping_sub(1 as ::core::ffi::c_uint) as isize))
            .id as *mut ::core::ffi::c_char,
        ) >= 0 as ::core::ffi::c_int
        {
            return;
        }
        position = THERMAL_CANDIDATE_CAPACITY.wrapping_sub(1 as ::core::ffi::c_uint) as size_t;
    }
    while position > 0 as size_t
        && strcmp(
            &raw const (*candidate).id as *const ::core::ffi::c_char,
            &raw mut (*candidates.offset(position.wrapping_sub(1 as size_t) as isize)).id
                as *mut ::core::ffi::c_char,
        ) < 0 as ::core::ffi::c_int
    {
        if position < THERMAL_CANDIDATE_CAPACITY as size_t {
            *candidates.offset(position as isize) =
                *candidates.offset(position.wrapping_sub(1 as size_t) as isize);
        }
        position = position.wrapping_sub(1);
    }
    *candidates.offset(position as isize) = *candidate;
}
unsafe extern "C" fn build_zone_id(
    mut entry: *const ::core::ffi::c_char,
    mut r#type: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
) {
    let mut normalized: [::core::ffi::c_char; 32] = [0; 32];
    normalize_component(
        r#type,
        &raw mut normalized as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    );
    let mut zone: *const ::core::ffi::c_char =
        entry.offset(strlen(b"thermal_zone\0".as_ptr() as *const ::core::ffi::c_char) as isize);
    snprintf(
        output,
        SYSTEM_HEALTH_ID_LENGTH as size_t,
        b"thermal.%.28s.zone%.12s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut normalized as *mut ::core::ffi::c_char,
        zone,
    );
}
unsafe extern "C" fn build_hwmon_id(
    mut sys_root: *const ::core::ffi::c_char,
    mut directory: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut label: *const ::core::ffi::c_char,
    mut channel: ::core::ffi::c_uint,
    mut output: *mut ::core::ffi::c_char,
) {
    let mut normalized_name: [::core::ffi::c_char; 18] = [0; 18];
    let mut normalized_label: [::core::ffi::c_char; 24] = [0; 24];
    normalize_component(
        name,
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 18]>(),
    );
    normalize_component(
        label,
        &raw mut normalized_label as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
    );
    let mut identity: uint32_t = hwmon_identity_hash(sys_root, directory, name, label, channel);
    snprintf(
        output,
        SYSTEM_HEALTH_ID_LENGTH as size_t,
        b"hwmon.%.16s.%.22s.%08x\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        &raw mut normalized_label as *mut ::core::ffi::c_char,
        identity,
    );
}
unsafe extern "C" fn merge_capability(
    mut current: system_health_capability_t,
    mut incoming: system_health_capability_t,
) -> system_health_capability_t {
    if current.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        || incoming.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    if current.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED.0
        || incoming.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED.0
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if current.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
        || incoming.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
}
unsafe extern "C" fn enumerate_zones(
    mut sys_root: *const ::core::ffi::c_char,
    mut candidates: *mut thermal_candidate_t,
    mut candidate_count: *mut size_t,
    mut total_candidates: *mut size_t,
) -> system_health_capability_t {
    let mut class_path: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut class_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"%s/class/thermal\0".as_ptr() as *const ::core::ffi::c_char,
        sys_root,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    let mut directory: *mut DIR = opendir(&raw mut class_path as *mut ::core::ffi::c_char);
    if directory.is_null() {
        return linux_thermal_capability_from_errno(*__errno_location());
    }
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(directory);
        if entry.is_null() {
            break;
        }
        if !suffix_is_digits(
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            b"thermal_zone\0".as_ptr() as *const ::core::ffi::c_char,
        ) {
            continue;
        }
        let mut candidate: thermal_candidate_t = thermal_candidate_t {
            source: thermal_source_t::THERMAL_SOURCE_ZONE,
            entry: [0; 64],
            channel: 0,
            id: [0; 64],
        };
        memset(
            &raw mut candidate as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<thermal_candidate_t>(),
        );
        candidate.source = thermal_source_t::THERMAL_SOURCE_ZONE;
        snprintf(
            &raw mut candidate.entry as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
        );
        let mut type_path: [::core::ffi::c_char; 4096] = [0; 4096];
        let mut r#type: [::core::ffi::c_char; 128] = ::core::mem::transmute::<
            [u8; 128],
            [::core::ffi::c_char; 128],
        >(
            *b"unknown\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut ignored: system_health_capability_t =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        if format_path(
            &raw mut type_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            b"%s/class/thermal/%s/type\0".as_ptr() as *const ::core::ffi::c_char,
            sys_root,
            &raw mut candidate.entry as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_uint,
        ) {
            read_text(
                &raw mut type_path as *mut ::core::ffi::c_char,
                &raw mut r#type as *mut ::core::ffi::c_char,
                &raw mut ignored,
            );
        }
        build_zone_id(
            &raw mut candidate.entry as *mut ::core::ffi::c_char,
            &raw mut r#type as *mut ::core::ffi::c_char,
            &raw mut candidate.id as *mut ::core::ffi::c_char,
        );
        candidate_consider(
            candidates,
            candidate_count,
            total_candidates,
            &raw mut candidate,
        );
    }
    closedir(directory);
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
}
unsafe extern "C" fn parse_temp_input_name(
    mut name: *const ::core::ffi::c_char,
    mut channel: *mut ::core::ffi::c_uint,
) -> bool {
    if name.is_null()
        || strncmp(
            name,
            b"temp\0".as_ptr() as *const ::core::ffi::c_char,
            4 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_char = name.offset(4 as ::core::ffi::c_int as isize);
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
        || parsed == 0 as ::core::ffi::c_ulong
        || parsed > 999 as ::core::ffi::c_ulong
        || strcmp(end, b"_input\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    *channel = parsed as ::core::ffi::c_uint;
    return r#true != 0;
}
unsafe extern "C" fn enumerate_hwmon(
    mut sys_root: *const ::core::ffi::c_char,
    mut candidates: *mut thermal_candidate_t,
    mut candidate_count: *mut size_t,
    mut total_candidates: *mut size_t,
) -> system_health_capability_t {
    let mut class_path: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut class_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"%s/class/hwmon\0".as_ptr() as *const ::core::ffi::c_char,
        sys_root,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
    {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    let mut directory: *mut DIR = opendir(&raw mut class_path as *mut ::core::ffi::c_char);
    if directory.is_null() {
        return linux_thermal_capability_from_errno(*__errno_location());
    }
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(directory);
        if entry.is_null() {
            break;
        }
        if !suffix_is_digits(
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            b"hwmon\0".as_ptr() as *const ::core::ffi::c_char,
        ) {
            continue;
        }
        let mut hwmon_path: [::core::ffi::c_char; 4096] = [0; 4096];
        written = snprintf(
            &raw mut hwmon_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut class_path as *mut ::core::ffi::c_char,
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
        );
        if written < 0 as ::core::ffi::c_int
            || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
        {
            continue;
        }
        let mut name_path: [::core::ffi::c_char; 4096] = [0; 4096];
        let mut name: [::core::ffi::c_char; 128] = ::core::mem::transmute::<
            [u8; 128],
            [::core::ffi::c_char; 128],
        >(
            *b"hwmon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut ignored: system_health_capability_t =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        written = snprintf(
            &raw mut name_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            b"%s/name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut hwmon_path as *mut ::core::ffi::c_char,
        );
        if written >= 0 as ::core::ffi::c_int
            && (written as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
        {
            read_text(
                &raw mut name_path as *mut ::core::ffi::c_char,
                &raw mut name as *mut ::core::ffi::c_char,
                &raw mut ignored,
            );
        }
        let mut hwmon_directory: *mut DIR =
            opendir(&raw mut hwmon_path as *mut ::core::ffi::c_char);
        if hwmon_directory.is_null() {
            continue;
        }
        let mut channel_entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
        loop {
            channel_entry = readdir(hwmon_directory);
            if channel_entry.is_null() {
                break;
            }
            let mut channel: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            if !parse_temp_input_name(
                &raw mut (*channel_entry).d_name as *mut ::core::ffi::c_char,
                &raw mut channel,
            ) {
                continue;
            }
            let mut label_path: [::core::ffi::c_char; 4096] = [0; 4096];
            let mut label: [::core::ffi::c_char; 128] = [0; 128];
            snprintf(
                &raw mut label as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                b"temp%u\0".as_ptr() as *const ::core::ffi::c_char,
                channel,
            );
            written = snprintf(
                &raw mut label_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                b"%s/temp%u_label\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut hwmon_path as *mut ::core::ffi::c_char,
                channel,
            );
            if written >= 0 as ::core::ffi::c_int
                && (written as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
            {
                let mut read_label: [::core::ffi::c_char; 128] = [0; 128];
                if read_text(
                    &raw mut label_path as *mut ::core::ffi::c_char,
                    &raw mut read_label as *mut ::core::ffi::c_char,
                    &raw mut ignored,
                ) {
                    snprintf(
                        &raw mut label as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut read_label as *mut ::core::ffi::c_char,
                    );
                }
            }
            let mut candidate: thermal_candidate_t = thermal_candidate_t {
                source: thermal_source_t::THERMAL_SOURCE_ZONE,
                entry: [0; 64],
                channel: 0,
                id: [0; 64],
            };
            memset(
                &raw mut candidate as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<thermal_candidate_t>(),
            );
            candidate.source = thermal_source_t::THERMAL_SOURCE_HWMON;
            candidate.channel = channel;
            snprintf(
                &raw mut candidate.entry as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            );
            build_hwmon_id(
                sys_root,
                &raw mut hwmon_path as *mut ::core::ffi::c_char,
                &raw mut name as *mut ::core::ffi::c_char,
                &raw mut label as *mut ::core::ffi::c_char,
                channel,
                &raw mut candidate.id as *mut ::core::ffi::c_char,
            );
            candidate_consider(
                candidates,
                candidate_count,
                total_candidates,
                &raw mut candidate,
            );
        }
        closedir(hwmon_directory);
    }
    closedir(directory);
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
}
unsafe extern "C" fn zone_critical_limit(
    mut sys_root: *const ::core::ffi::c_char,
    mut entry: *const ::core::ffi::c_char,
) -> thermal_numeric_t {
    let mut best: thermal_numeric_t = thermal_numeric_t {
        valid: r#false != 0,
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        value: 0.0f64,
    };
    let mut index: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while index < THERMAL_TRIP_LIMIT {
        let mut type_path: [::core::ffi::c_char; 4096] = [0; 4096];
        if !format_path(
            &raw mut type_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            b"%s/class/thermal/%s/trip_point_%u_type\0".as_ptr() as *const ::core::ffi::c_char,
            sys_root,
            entry,
            index,
        ) {
            best.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        } else {
            let mut r#type: [::core::ffi::c_char; 128] = [0; 128];
            let mut type_capability: system_health_capability_t =
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
            if !read_text(
                &raw mut type_path as *mut ::core::ffi::c_char,
                &raw mut r#type as *mut ::core::ffi::c_char,
                &raw mut type_capability,
            ) {
                if type_capability.0
                    != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                {
                    best.capability = merge_capability(best.capability, type_capability);
                }
            } else if strcasecmp(
                &raw mut r#type as *mut ::core::ffi::c_char,
                b"critical\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut temp_path: [::core::ffi::c_char; 4096] = [0; 4096];
                if !format_path(
                    &raw mut temp_path as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                    b"%s/class/thermal/%s/trip_point_%u_temp\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    sys_root,
                    entry,
                    index,
                ) {
                    best.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
                } else {
                    let mut candidate: thermal_numeric_t =
                        read_millidegrees(&raw mut temp_path as *mut ::core::ffi::c_char);
                    if candidate.valid as ::core::ffi::c_int != 0
                        && (!best.valid || candidate.value < best.value)
                    {
                        best = candidate;
                    } else if !candidate.valid && !best.valid {
                        best.capability = merge_capability(best.capability, candidate.capability);
                    }
                }
            }
        }
        index = index.wrapping_add(1);
    }
    return best;
}
unsafe extern "C" fn hwmon_limit(
    mut sys_root: *const ::core::ffi::c_char,
    mut candidate: *const thermal_candidate_t,
    mut is_critical: *mut bool,
) -> thermal_numeric_t {
    let mut result: thermal_numeric_t = thermal_numeric_t {
        valid: r#false != 0,
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        value: 0.0f64,
    };
    *is_critical = r#false != 0;
    let mut path: [::core::ffi::c_char; 4096] = [0; 4096];
    if !format_path(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"%s/class/hwmon/%s/temp%u_crit\0".as_ptr() as *const ::core::ffi::c_char,
        sys_root,
        &raw const (*candidate).entry as *const ::core::ffi::c_char,
        (*candidate).channel,
    ) {
        result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    result = read_millidegrees(&raw mut path as *mut ::core::ffi::c_char);
    if result.valid {
        *is_critical = r#true != 0;
        return result;
    }
    if result.capability.0 != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0 {
        return result;
    }
    if !format_path(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"%s/class/hwmon/%s/temp%u_max\0".as_ptr() as *const ::core::ffi::c_char,
        sys_root,
        &raw const (*candidate).entry as *const ::core::ffi::c_char,
        (*candidate).channel,
    ) {
        result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    return read_millidegrees(&raw mut path as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn track_sensor(
    mut state: *mut linux_thermal_state_t,
    mut id: *const ::core::ffi::c_char,
) -> *mut linux_thermal_sensor_state_t {
    let mut index: size_t = 0 as size_t;
    while index < (*state).sensor_count {
        if strcmp(
            &raw mut (*(&raw mut (*state).sensors as *mut linux_thermal_sensor_state_t)
                .offset(index as isize))
            .id as *mut ::core::ffi::c_char,
            id,
        ) == 0 as ::core::ffi::c_int
        {
            return (&raw mut (*state).sensors as *mut linux_thermal_sensor_state_t)
                .offset(index as isize);
        }
        index = index.wrapping_add(1);
    }
    let mut slot: size_t = (*state).sensor_count;
    if slot >= LINUX_THERMAL_STATE_CAPACITY as size_t {
        let mut index_0: size_t = 0 as size_t;
        while index_0 < (*state).sensor_count {
            if !(*state).sensors[index_0].present
                && (*state).sensors[index_0].missing_cycles as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
            {
                slot = index_0;
                break;
            } else {
                index_0 = index_0.wrapping_add(1);
            }
        }
    } else {
        (*state).sensor_count = (*state).sensor_count.wrapping_add(1);
    }
    if slot >= LINUX_THERMAL_STATE_CAPACITY as size_t {
        return ::core::ptr::null_mut::<linux_thermal_sensor_state_t>();
    }
    memset(
        (&raw mut (*state).sensors as *mut linux_thermal_sensor_state_t).offset(slot as isize)
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_thermal_sensor_state_t>(),
    );
    snprintf(
        &raw mut (*(&raw mut (*state).sensors as *mut linux_thermal_sensor_state_t)
            .offset(slot as isize))
        .id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        id,
    );
    return (&raw mut (*state).sensors as *mut linux_thermal_sensor_state_t).offset(slot as isize);
}
unsafe extern "C" fn append_observation(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
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
    observation.scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST;
    observation.sampled_monotonic_ms = (*context).monotonic_ms;
    observation.observed_wall_time_ms = (*context).wall_time_ms;
    if value_valid {
        system_health_observation_set_available(&raw mut observation, value, unit);
    } else {
        system_health_observation_set_unavailable(&raw mut observation, capability);
    }
    system_health_observation_sink_append(sink, &raw mut observation);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_thermal_state_init(mut state: *mut linux_thermal_state_t) {
    if !state.is_null() {
        memset(
            state as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<linux_thermal_state_t>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_thermal_collect(
    mut state: *mut linux_thermal_state_t,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
    mut result: *mut linux_thermal_result_t,
) -> ::core::ffi::c_int {
    if state.is_null() || context.is_null() || (*context).sys_root.is_null() || sink.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if !result.is_null() {
        memset(
            result as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<linux_thermal_result_t>(),
        );
    }
    let mut index: size_t = 0 as size_t;
    while index < (*state).sensor_count {
        (*state).sensors[index].seen = r#false != 0;
        index = index.wrapping_add(1);
    }
    let mut candidates: [thermal_candidate_t; 128] = [thermal_candidate_t {
        source: thermal_source_t::THERMAL_SOURCE_ZONE,
        entry: [0; 64],
        channel: 0,
        id: [0; 64],
    }; 128];
    let mut candidate_count: size_t = 0 as size_t;
    let mut total_candidates: size_t = 0 as size_t;
    let mut zone_capability: system_health_capability_t = enumerate_zones(
        (*context).sys_root,
        &raw mut candidates as *mut thermal_candidate_t,
        &raw mut candidate_count,
        &raw mut total_candidates,
    );
    let mut hwmon_capability: system_health_capability_t = enumerate_hwmon(
        (*context).sys_root,
        &raw mut candidates as *mut thermal_candidate_t,
        &raw mut candidate_count,
        &raw mut total_candidates,
    );
    let mut overall: system_health_capability_t =
        merge_capability(zone_capability, hwmon_capability);
    if total_candidates == 0 as size_t
        && overall.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    {
        overall = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }
    let mut export_count: size_t = candidate_count;
    if export_count > SYSTEM_HEALTH_MAX_SENSORS as size_t {
        export_count = SYSTEM_HEALTH_MAX_SENSORS as size_t;
    }
    let mut resources_dropped: size_t = if total_candidates > export_count {
        total_candidates.wrapping_sub(export_count)
    } else {
        0 as size_t
    };
    (*state).resources_dropped_total = ((*state).resources_dropped_total as ::core::ffi::c_ulong)
        .wrapping_add(resources_dropped as ::core::ffi::c_ulong)
        as uint64_t;
    if !result.is_null() {
        (*result).capability = overall;
        (*result).resources_dropped = resources_dropped;
    }
    append_observation(
        sink,
        context,
        b"thermal.collector_available\0".as_ptr() as *const ::core::ffi::c_char,
        b"thermal\0".as_ptr() as *const ::core::ffi::c_char,
        overall,
        overall.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0,
        1.0f64,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
    );
    let mut index_0: size_t = 0 as size_t;
    while index_0 < export_count {
        let mut candidate: *const thermal_candidate_t =
            (&raw mut candidates as *mut thermal_candidate_t).offset(index_0 as isize);
        let mut tracked: *mut linux_thermal_sensor_state_t = track_sensor(
            state,
            &raw const (*candidate).id as *const ::core::ffi::c_char,
        );
        if tracked.is_null() {
            (*state).resources_dropped_total = (*state).resources_dropped_total.wrapping_add(1);
            if !result.is_null() {
                (*result).resources_dropped = (*result).resources_dropped.wrapping_add(1);
            }
        } else {
            (*tracked).seen = r#true != 0;
            (*tracked).present = r#true != 0;
            (*tracked).missing_cycles = 0 as uint8_t;
            let mut temperature_path: [::core::ffi::c_char; 4096] = [0; 4096];
            let mut path_ok: bool = false;
            if (*candidate).source.0 == thermal_source_t::THERMAL_SOURCE_ZONE.0 {
                path_ok = format_path(
                    &raw mut temperature_path as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                    b"%s/class/thermal/%s/temp\0".as_ptr() as *const ::core::ffi::c_char,
                    (*context).sys_root,
                    &raw const (*candidate).entry as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_uint,
                );
            } else {
                path_ok = format_path(
                    &raw mut temperature_path as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                    b"%s/class/hwmon/%s/temp%u_input\0".as_ptr() as *const ::core::ffi::c_char,
                    (*context).sys_root,
                    &raw const (*candidate).entry as *const ::core::ffi::c_char,
                    (*candidate).channel,
                );
            }
            let mut temperature: thermal_numeric_t = if path_ok as ::core::ffi::c_int != 0 {
                read_millidegrees(&raw mut temperature_path as *mut ::core::ffi::c_char)
            } else {
                thermal_numeric_t {
                    valid: r#false != 0,
                    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
                    value: 0.0f64,
                }
            };
            let mut limit_is_critical: bool =
                (*candidate).source.0 == thermal_source_t::THERMAL_SOURCE_ZONE.0;
            let mut limit: thermal_numeric_t =
                if (*candidate).source.0 == thermal_source_t::THERMAL_SOURCE_ZONE.0 {
                    zone_critical_limit(
                        (*context).sys_root,
                        &raw const (*candidate).entry as *const ::core::ffi::c_char,
                    )
                } else {
                    hwmon_limit((*context).sys_root, candidate, &raw mut limit_is_critical)
                };
            let mut limit_metric: *const ::core::ffi::c_char =
                if limit_is_critical as ::core::ffi::c_int != 0 {
                    b"thermal.critical_celsius\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"thermal.maximum_celsius\0".as_ptr() as *const ::core::ffi::c_char
                };
            if !limit.valid && !limit_is_critical {
                limit_metric = b"thermal.critical_celsius\0".as_ptr() as *const ::core::ffi::c_char;
            }
            snprintf(
                &raw mut (*tracked).limit_metric as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                limit_metric,
            );
            append_observation(
                sink,
                context,
                b"thermal.temperature_celsius\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*candidate).id as *const ::core::ffi::c_char,
                temperature.capability,
                temperature.valid,
                temperature.value,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_CELSIUS,
            );
            append_observation(
                sink,
                context,
                limit_metric,
                &raw const (*candidate).id as *const ::core::ffi::c_char,
                limit.capability,
                limit.valid,
                limit.value,
                system_health_unit_t::SYSTEM_HEALTH_UNIT_CELSIUS,
            );
            if !result.is_null() && (*result).sensor_count < SYSTEM_HEALTH_MAX_SENSORS as size_t {
                let c2rust_fresh0 = (*result).sensor_count;
                (*result).sensor_count = (*result).sensor_count.wrapping_add(1);
                let mut sample: *mut linux_thermal_sensor_sample_t = (&raw mut (*result).sensors
                    as *mut linux_thermal_sensor_sample_t)
                    .offset(c2rust_fresh0 as isize);
                memset(
                    sample as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<linux_thermal_sensor_sample_t>(),
                );
                snprintf(
                    &raw mut (*sample).id as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*candidate).id as *const ::core::ffi::c_char,
                );
                (*sample).temperature_capability = temperature.capability;
                (*sample).temperature_valid = temperature.valid;
                (*sample).temperature_celsius = temperature.value;
                (*sample).limit_capability = limit.capability;
                (*sample).limit_valid = limit.valid;
                (*sample).limit_is_critical = limit_is_critical as ::core::ffi::c_int != 0
                    && limit.valid as ::core::ffi::c_int != 0;
                (*sample).limit_celsius = limit.value;
            }
        }
        index_0 = index_0.wrapping_add(1);
    }
    let mut index_1: size_t = 0 as size_t;
    while index_1 < (*state).sensor_count {
        let mut tracked_0: *mut linux_thermal_sensor_state_t = (&raw mut (*state).sensors
            as *mut linux_thermal_sensor_state_t)
            .offset(index_1 as isize);
        if !(*tracked_0).seen {
            if (*tracked_0).present {
                (*tracked_0).present = r#false != 0;
                (*tracked_0).missing_cycles = 1 as uint8_t;
                append_observation(
                    sink,
                    context,
                    b"thermal.temperature_celsius\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*tracked_0).id as *mut ::core::ffi::c_char,
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE,
                    r#false != 0,
                    0.0f64,
                    system_health_unit_t::SYSTEM_HEALTH_UNIT_CELSIUS,
                );
                append_observation(
                    sink,
                    context,
                    if (*tracked_0).limit_metric[0usize] as ::core::ffi::c_int != 0 {
                        &raw mut (*tracked_0).limit_metric as *mut ::core::ffi::c_char
                    } else {
                        b"thermal.critical_celsius\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    &raw mut (*tracked_0).id as *mut ::core::ffi::c_char,
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE,
                    r#false != 0,
                    0.0f64,
                    system_health_unit_t::SYSTEM_HEALTH_UNIT_CELSIUS,
                );
            } else if ((*tracked_0).missing_cycles as ::core::ffi::c_int) < UINT8_MAX {
                (*tracked_0).missing_cycles = (*tracked_0).missing_cycles.wrapping_add(1);
            }
        }
        index_1 = index_1.wrapping_add(1);
    }
    if resources_dropped > 0 as size_t {
        append_observation(
            sink,
            context,
            b"thermal.sensors_dropped\0".as_ptr() as *const ::core::ffi::c_char,
            b"thermal\0".as_ptr() as *const ::core::ffi::c_char,
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
    return linux_thermal_collect(
        state as *mut linux_thermal_state_t,
        context,
        sink,
        ::core::ptr::null_mut::<linux_thermal_result_t>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_thermal_collector_init(
    mut collector: *mut system_health_collector_t,
    mut state: *mut linux_thermal_state_t,
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
        b"linux_thermal\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*collector).scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST;
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
