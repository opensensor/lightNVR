use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
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
    unsafe fn fscanf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_count() -> ::core::ffi::c_int;
    unsafe fn db_storage_target_list(
        targets: *mut storage_target_t,
        max_count: ::core::ffi::c_int,
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
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
pub type __ino_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int64_t = i64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_target_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub target_type: [::core::ffi::c_char; 24],
    pub root_path: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub is_default: bool,
    pub storage_class: [::core::ffi::c_char; 16],
    pub reserve_bytes: uint64_t,
    pub high_watermark_pct: ::core::ffi::c_double,
    pub low_watermark_pct: ::core::ffi::c_double,
    pub health_status: [::core::ffi::c_char; 16],
    pub capacity_bytes: uint64_t,
    pub available_bytes: uint64_t,
    pub filesystem_device: uint64_t,
    pub last_probe_at: int64_t,
    pub last_success_at: int64_t,
    pub last_error: [::core::ffi::c_char; 256],
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub recording_count: uint64_t,
    pub recording_bytes: uint64_t,
    pub mount_required: bool,
    pub mount_guard_path: [::core::ffi::c_char; 512],
    pub migration_bandwidth_bps: uint64_t,
    pub archival_window_start_minute: ::core::ffi::c_int,
    pub archival_window_end_minute: ::core::ffi::c_int,
    pub replica_count: uint64_t,
    pub replica_bytes: uint64_t,
    pub endpoint: [::core::ffi::c_char; 512],
    pub region: [::core::ffi::c_char; 64],
    pub bucket: [::core::ffi::c_char; 128],
    pub credential_ref: [::core::ffi::c_char; 128],
    pub archive_budget_bytes: uint64_t,
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
pub const SYSTEM_HEALTH_MAX_DEVICES: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const LINUX_HARDWARE_INTERNAL_NAME_LENGTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const LINUX_HARDWARE_DEVICE_MAP_MAX: ::core::ffi::c_uint = SYSTEM_HEALTH_MAX_DEVICES;
#[inline]
unsafe extern "C" fn gnu_dev_makedev(
    mut __major: ::core::ffi::c_uint,
    mut __minor: ::core::ffi::c_uint,
) -> __dev_t {
    let mut __dev: __dev_t = 0;
    __dev = ((__major & 0xfff as ::core::ffi::c_uint) as __dev_t) << 8 as ::core::ffi::c_int;
    __dev |= ((__major & 0xfffff000 as ::core::ffi::c_uint) as __dev_t) << 32 as ::core::ffi::c_int;
    __dev |= ((__minor & 0xff as ::core::ffi::c_uint) as __dev_t) << 0 as ::core::ffi::c_int;
    __dev |= ((__minor & 0xffffff00 as ::core::ffi::c_uint) as __dev_t) << 12 as ::core::ffi::c_int;
    return __dev;
}
pub const STORAGE_TARGET_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const DEVICE_MAP_SCAN_MAX: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
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
unsafe extern "C" fn internal_name_valid(mut name: *const ::core::ffi::c_char) -> bool {
    if name.is_null()
        || *name.offset(0isize) == 0
        || strcmp(name, b".\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strcmp(name, b"..\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strlen(name) >= LINUX_HARDWARE_INTERNAL_NAME_LENGTH as size_t
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
unsafe extern "C" fn compare_names(
    mut left: *const ::core::ffi::c_void,
    mut right: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        left as *const ::core::ffi::c_char,
        right as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn read_device_number(
    mut sys_root: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut device: *mut uint64_t,
    mut capability: *mut system_health_capability_t,
) -> bool {
    let mut path: [::core::ffi::c_char; 1200] = [0; 1200];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
        b"%s/class/block/%s/dev\0".as_ptr() as *const ::core::ffi::c_char,
        sys_root,
        name,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 1200]>()
    {
        *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return r#false != 0;
    }
    let mut file: *mut FILE = fopen(
        &raw mut path as *mut ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if file.is_null() {
        *capability = capability_from_errno(*__errno_location());
        return r#false != 0;
    }
    let mut major_number: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut minor_number: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut trailing: ::core::ffi::c_char = '\0' as ::core::ffi::c_char;
    let mut matched: ::core::ffi::c_int = fscanf(
        file,
        b"%u:%u %c\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut major_number,
        &raw mut minor_number,
        &raw mut trailing,
    );
    fclose(file);
    if matched != 2 as ::core::ffi::c_int {
        *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return r#false != 0;
    }
    *device = gnu_dev_makedev(major_number, minor_number) as uint64_t;
    *capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    return r#true != 0;
}
unsafe extern "C" fn physical_name(
    mut name: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
) {
    let mut length: size_t = strnlen(name, LINUX_HARDWARE_INTERNAL_NAME_LENGTH as size_t);
    if length >= LINUX_HARDWARE_INTERNAL_NAME_LENGTH as size_t {
        *output.offset(0isize) = '\0' as ::core::ffi::c_char;
        return;
    }
    memcpy(
        output as *mut ::core::ffi::c_void,
        name as *const ::core::ffi::c_void,
        length.wrapping_add(1 as size_t),
    );
    if strncmp(
        name,
        b"nvme\0".as_ptr() as *const ::core::ffi::c_char,
        4 as size_t,
    ) == 0 as ::core::ffi::c_int
        && *(*__ctype_b_loc())
            .offset(*name.offset(4isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        let mut cursor: *mut ::core::ffi::c_char = output.offset(4 as ::core::ffi::c_int as isize);
        while *(*__ctype_b_loc())
            .offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            cursor = cursor.offset(1);
        }
        if *cursor as ::core::ffi::c_int == 'n' as ::core::ffi::c_int
            && *(*__ctype_b_loc()).offset(
                *cursor.offset(1isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
            ) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            *cursor = '\0' as ::core::ffi::c_char;
        }
        return;
    }
    if strncmp(
        name,
        b"mmcblk\0".as_ptr() as *const ::core::ffi::c_char,
        6 as size_t,
    ) == 0 as ::core::ffi::c_int
        && *(*__ctype_b_loc())
            .offset(*name.offset(6isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        let mut cursor_0: *mut ::core::ffi::c_char =
            output.offset(6 as ::core::ffi::c_int as isize);
        while *(*__ctype_b_loc())
            .offset(*cursor_0 as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            cursor_0 = cursor_0.offset(1);
        }
        if *cursor_0 as ::core::ffi::c_int == 'p' as ::core::ffi::c_int
            && *(*__ctype_b_loc()).offset(
                *cursor_0.offset(1isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
            ) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            *cursor_0 = '\0' as ::core::ffi::c_char;
        }
        return;
    }
    if *name.offset(0isize) as ::core::ffi::c_int == 's' as ::core::ffi::c_int
        && *name.offset(1isize) as ::core::ffi::c_int == 'd' as ::core::ffi::c_int
        && *(*__ctype_b_loc())
            .offset(*name.offset(2isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalpha.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        let mut cursor_1: *mut ::core::ffi::c_char =
            output.offset(2 as ::core::ffi::c_int as isize);
        while *(*__ctype_b_loc())
            .offset(*cursor_1 as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalpha.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            cursor_1 = cursor_1.offset(1);
        }
        if *(*__ctype_b_loc())
            .offset(*cursor_1 as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            *cursor_1 = '\0' as ::core::ffi::c_char;
        }
    }
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
unsafe extern "C" fn target_preferred(
    mut candidate: *const storage_target_t,
    mut current: *const linux_device_map_entry_t,
) -> bool {
    if !(*current).target_mapped {
        return r#true != 0;
    }
    if (*candidate).is_default as ::core::ffi::c_int
        != (*current).target_is_default as ::core::ffi::c_int
    {
        return (*candidate).is_default;
    }
    let mut current_uuid: *const ::core::ffi::c_char = strchr(
        &raw const (*current).public_id as *const ::core::ffi::c_char,
        ':' as ::core::ffi::c_int,
    );
    return !current_uuid.is_null()
        && strcmp(
            &raw const (*candidate).uuid as *const ::core::ffi::c_char,
            current_uuid.offset(1 as ::core::ffi::c_int as isize),
        ) < 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_device_map_find(
    mut map: *const linux_device_map_t,
    mut physical_name_value: *const ::core::ffi::c_char,
) -> *const linux_device_map_entry_t {
    if map.is_null() || physical_name_value.is_null() {
        return ::core::ptr::null::<linux_device_map_entry_t>();
    }
    let mut index: size_t = 0 as size_t;
    while index < (*map).count {
        if strcmp(
            &raw const (*(&raw const (*map).entries as *const linux_device_map_entry_t)
                .offset(index as isize))
            .sysfs_name as *const ::core::ffi::c_char,
            physical_name_value,
        ) == 0 as ::core::ffi::c_int
        {
            return (&raw const (*map).entries as *const linux_device_map_entry_t)
                .offset(index as isize);
        }
        index = index.wrapping_add(1);
    }
    return ::core::ptr::null::<linux_device_map_entry_t>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_device_map_build(
    mut sys_root: *const ::core::ffi::c_char,
    mut installation_scope: *const ::core::ffi::c_char,
    mut map: *mut linux_device_map_t,
) -> ::core::ffi::c_int {
    if sys_root.is_null()
        || *sys_root.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        || installation_scope.is_null()
        || *installation_scope.offset(0isize) == 0
        || map.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        map as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_device_map_t>(),
    );
    let mut class_path: [::core::ffi::c_char; 1200] = [0; 1200];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut class_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>(),
        b"%s/class/block\0".as_ptr() as *const ::core::ffi::c_char,
        sys_root,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 1200]>()
    {
        (*map).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1 as ::core::ffi::c_int;
    }
    let mut directory: *mut DIR = opendir(&raw mut class_path as *mut ::core::ffi::c_char);
    if directory.is_null() {
        (*map).capability = capability_from_errno(*__errno_location());
        return 0 as ::core::ffi::c_int;
    }
    let mut names: [[::core::ffi::c_char; 64]; 128] = [[0; 64]; 128];
    let mut name_count: size_t = 0 as size_t;
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(directory);
        if entry.is_null() {
            break;
        }
        if !internal_name_valid(&raw mut (*entry).d_name as *mut ::core::ffi::c_char) {
            continue;
        }
        if name_count >= DEVICE_MAP_SCAN_MAX as size_t {
            (*map).dropped = (*map).dropped.wrapping_add(1);
            break;
        } else {
            let mut length: size_t = strlen(&raw mut (*entry).d_name as *mut ::core::ffi::c_char);
            memcpy(
                &raw mut *(&raw mut names as *mut [::core::ffi::c_char; 64])
                    .offset(name_count as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
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
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut target_total: ::core::ffi::c_int = db_storage_target_count();
    let mut targets: *mut storage_target_t = ::core::ptr::null_mut::<storage_target_t>();
    let mut target_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if target_total > 0 as ::core::ffi::c_int && target_total <= STORAGE_TARGET_MAX_COUNT {
        targets = calloc(
            target_total as size_t,
            ::core::mem::size_of::<storage_target_t>(),
        ) as *mut storage_target_t;
        if !targets.is_null() {
            target_count = db_storage_target_list(targets, target_total);
            if target_count < 0 as ::core::ffi::c_int {
                target_count = 0 as ::core::ffi::c_int;
            }
        }
    }
    let mut scan_capability: system_health_capability_t =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    let mut name_index: size_t = 0 as size_t;
    while name_index < name_count {
        let mut device: uint64_t = 0 as uint64_t;
        let mut device_capability: system_health_capability_t =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        's_130: {
            if !read_device_number(
                sys_root,
                &raw mut *(&raw mut names as *mut [::core::ffi::c_char; 64])
                    .offset(name_index as isize) as *mut ::core::ffi::c_char,
                &raw mut device,
                &raw mut device_capability,
            ) {
                scan_capability = merge_capability(scan_capability, device_capability);
            } else {
                scan_capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
                let mut physical: [::core::ffi::c_char; 64] = [0; 64];
                physical_name(
                    &raw mut *(&raw mut names as *mut [::core::ffi::c_char; 64])
                        .offset(name_index as isize)
                        as *mut ::core::ffi::c_char,
                    &raw mut physical as *mut ::core::ffi::c_char,
                );
                let mut map_index: size_t = 0 as size_t;
                while map_index < (*map).count
                    && strcmp(
                        &raw mut (*(&raw mut (*map).entries as *mut linux_device_map_entry_t)
                            .offset(map_index as isize))
                        .sysfs_name as *mut ::core::ffi::c_char,
                        &raw mut physical as *mut ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                {
                    map_index = map_index.wrapping_add(1);
                }
                if map_index == (*map).count {
                    if (*map).count >= LINUX_HARDWARE_DEVICE_MAP_MAX as size_t {
                        (*map).dropped = (*map).dropped.wrapping_add(1);
                        break 's_130;
                    } else {
                        let c2rust_fresh0 = (*map).count;
                        (*map).count = (*map).count.wrapping_add(1);
                        let mut mapped: *mut linux_device_map_entry_t = (&raw mut (*map).entries
                            as *mut linux_device_map_entry_t)
                            .offset(c2rust_fresh0 as isize);
                        snprintf(
                            &raw mut (*mapped).sysfs_name as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut physical as *mut ::core::ffi::c_char,
                        );
                        (*mapped).filesystem_device = device;
                    }
                }
                let mut mapped_0: *mut linux_device_map_entry_t = (&raw mut (*map).entries
                    as *mut linux_device_map_entry_t)
                    .offset(map_index as isize);
                let mut target_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while target_index < target_count {
                    let mut target: *const storage_target_t = targets.offset(target_index as isize);
                    if !((*target).filesystem_device != device
                        || !target_preferred(target, mapped_0))
                    {
                        snprintf(
                            &raw mut (*mapped_0).public_id as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                            b"target:%s\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw const (*target).uuid as *const ::core::ffi::c_char,
                        );
                        (*mapped_0).target_mapped = r#true != 0;
                        (*mapped_0).target_is_default = (*target).is_default;
                        (*mapped_0).filesystem_device = device;
                    }
                    target_index += 1;
                }
            }
        }
        name_index = name_index.wrapping_add(1);
    }
    free(targets as *mut ::core::ffi::c_void);
    let mut index: size_t = 0 as size_t;
    while index < (*map).count {
        let mut mapped_1: *mut linux_device_map_entry_t =
            (&raw mut (*map).entries as *mut linux_device_map_entry_t).offset(index as isize);
        if !(*mapped_1).target_mapped {
            snprintf(
                &raw mut (*mapped_1).public_id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"device.%016llx\0".as_ptr() as *const ::core::ffi::c_char,
                scoped_hash(
                    installation_scope,
                    &raw mut (*mapped_1).sysfs_name as *mut ::core::ffi::c_char,
                ) as ::core::ffi::c_ulonglong,
            );
        }
        index = index.wrapping_add(1);
    }
    (*map).capability = system_health_capability_t(if (*map).count > 0 as size_t {
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    } else {
        scan_capability.0
    });
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
