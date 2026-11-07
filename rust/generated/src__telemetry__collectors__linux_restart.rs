use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    unsafe fn fgetc(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type uint64_t = u64;
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
pub struct linux_restart_kind_t(pub ::core::ffi::c_uint);
impl linux_restart_kind_t {
    pub const LINUX_RESTART_FIRST_OBSERVATION: Self = Self(0);
    pub const LINUX_RESTART_SAME_RUN: Self = Self(1);
    pub const LINUX_RESTART_PROCESS_RESTART: Self = Self(2);
    pub const LINUX_RESTART_HOST_REBOOT: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_restart_evidence_t {
    pub capability: system_health_capability_t,
    pub boot_id: [::core::ffi::c_char; 37],
    pub run_id: [::core::ffi::c_char; 37],
    pub host_uptime_seconds: ::core::ffi::c_double,
    pub process_start_monotonic_ms: uint64_t,
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
pub const LINUX_RESTART_ID_LENGTH: ::core::ffi::c_uint = 37 as ::core::ffi::c_uint;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_restart_id_valid(mut id: *const ::core::ffi::c_char) -> bool {
    if id.is_null() || strlen(id) != 36 as size_t {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < 36 as size_t {
        let mut hyphen: bool =
            i == 8 as size_t || i == 13 as size_t || i == 18 as size_t || i == 23 as size_t;
        if hyphen as ::core::ffi::c_int != 0
            && *id.offset(i as isize) as ::core::ffi::c_int != '-' as ::core::ffi::c_int
            || !hyphen
                && *(*__ctype_b_loc()).offset(
                    *id.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
                ) as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISxdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    == 0
        {
            return r#false != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn capability_for_errno(
    mut error: ::core::ffi::c_int,
) -> system_health_capability_t {
    if error == EACCES || error == EPERM {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if error == ENOENT || error == ENOTDIR {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
}
unsafe extern "C" fn read_one_line(
    mut path: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
    mut capability: *mut system_health_capability_t,
) -> ::core::ffi::c_int {
    let mut file: *mut FILE = fopen(path, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    if file.is_null() {
        *capability = capability_for_errno(*__errno_location());
        return -1 as ::core::ffi::c_int;
    }
    if fgets(output, output_size as ::core::ffi::c_int, file).is_null() {
        let mut saved_errno: ::core::ffi::c_int = *__errno_location();
        fclose(file);
        *capability = capability_for_errno(saved_errno);
        return -1 as ::core::ffi::c_int;
    }
    let mut truncated: bool = r#false != 0;
    if strchr(output, '\n' as ::core::ffi::c_int).is_null() {
        let mut trailing: ::core::ffi::c_int = fgetc(file);
        if trailing == '\r' as ::core::ffi::c_int {
            trailing = fgetc(file);
        }
        truncated = trailing != '\n' as ::core::ffi::c_int && trailing != EOF;
        if ferror(file) != 0 {
            truncated = r#true != 0;
        }
    }
    fclose(file);
    if truncated {
        *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1 as ::core::ffi::c_int;
    }
    *output.offset(strcspn(output, b"\r\n\0".as_ptr() as *const ::core::ffi::c_char) as isize) =
        '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_restart_read_evidence(
    mut proc_root: *const ::core::ffi::c_char,
    mut run_id: *const ::core::ffi::c_char,
    mut process_start_monotonic_ms: uint64_t,
    mut evidence: *mut linux_restart_evidence_t,
) -> ::core::ffi::c_int {
    if proc_root.is_null()
        || run_id.is_null()
        || evidence.is_null()
        || !linux_restart_id_valid(run_id)
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        evidence as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_restart_evidence_t>(),
    );
    (*evidence).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    (*evidence).process_start_monotonic_ms = process_start_monotonic_ms;
    memcpy(
        &raw mut (*evidence).run_id as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        run_id as *const ::core::ffi::c_void,
        LINUX_RESTART_ID_LENGTH as size_t,
    );
    let mut path: [::core::ffi::c_char; 512] = [0; 512];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/sys/kernel/random/boot_id\0".as_ptr() as *const ::core::ffi::c_char,
        proc_root,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        || read_one_line(
            &raw mut path as *mut ::core::ffi::c_char,
            &raw mut (*evidence).boot_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            &raw mut (*evidence).capability,
        ) != 0 as ::core::ffi::c_int
        || !linux_restart_id_valid(&raw mut (*evidence).boot_id as *mut ::core::ffi::c_char)
    {
        if (*evidence).capability.0
            == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        {
            (*evidence).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        }
        return -1 as ::core::ffi::c_int;
    }
    written = snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/uptime\0".as_ptr() as *const ::core::ffi::c_char,
        proc_root,
    );
    let mut uptime: [::core::ffi::c_char; 128] = [0; 128];
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        || read_one_line(
            &raw mut path as *mut ::core::ffi::c_char,
            &raw mut uptime as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            &raw mut (*evidence).capability,
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    (*evidence).host_uptime_seconds =
        strtod(&raw mut uptime as *mut ::core::ffi::c_char, &raw mut end);
    if *__errno_location() != 0 as ::core::ffi::c_int
        || end == &raw mut uptime as *mut ::core::ffi::c_char
        || (*evidence).host_uptime_seconds.is_finite() as ::core::ffi::c_int == 0
        || (*evidence).host_uptime_seconds < 0.0f64
    {
        (*evidence).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1 as ::core::ffi::c_int;
    }
    while *end as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        end = end.offset(1);
    }
    let mut idle_end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut idle_seconds: ::core::ffi::c_double = strtod(end, &raw mut idle_end);
    if *__errno_location() != 0 as ::core::ffi::c_int
        || idle_end == end
        || idle_seconds.is_finite() as ::core::ffi::c_int == 0
        || idle_seconds < 0.0f64
    {
        (*evidence).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1 as ::core::ffi::c_int;
    }
    while *idle_end as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc())
            .offset(*idle_end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        idle_end = idle_end.offset(1);
    }
    if *idle_end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        (*evidence).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_restart_classify(
    mut previous: *const linux_restart_evidence_t,
    mut current: *const linux_restart_evidence_t,
) -> linux_restart_kind_t {
    if current.is_null()
        || (*current).capability.0
            != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        || previous.is_null()
        || (*previous).capability.0
            != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    {
        return linux_restart_kind_t::LINUX_RESTART_FIRST_OBSERVATION;
    }
    if strcmp(
        &raw const (*previous).boot_id as *const ::core::ffi::c_char,
        &raw const (*current).boot_id as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        return linux_restart_kind_t::LINUX_RESTART_HOST_REBOOT;
    }
    if strcmp(
        &raw const (*previous).run_id as *const ::core::ffi::c_char,
        &raw const (*current).run_id as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        return linux_restart_kind_t::LINUX_RESTART_PROCESS_RESTART;
    }
    return linux_restart_kind_t::LINUX_RESTART_SAME_RUN;
}
