unsafe extern "C" {
    pub type __dirstream;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn fdopendir(__fd: ::core::ffi::c_int) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn dirname(__path: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fchmod(__fd: ::core::ffi::c_int, __mode: __mode_t) -> ::core::ffi::c_int;
    unsafe fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub const ELOOP: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sanitize_stream_name(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) {
    let mut i: size_t = 0 as size_t;
    while i < output_size.wrapping_sub(1 as size_t)
        && *input.offset(i as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        let mut c: ::core::ffi::c_uchar = *input.offset(i as isize) as ::core::ffi::c_uchar;
        if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalpha.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            || *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            || c as ::core::ffi::c_int == '-' as ::core::ffi::c_int
        {
            *output.offset(i as isize) = c as ::core::ffi::c_char;
        } else {
            *output.offset(i as isize) = '_' as ::core::ffi::c_char;
        }
        i = i.wrapping_add(1);
    }
    *output.offset(i as isize) = '\0' as ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_valid_stream_name(mut name: *const ::core::ffi::c_char) -> bool {
    if name.is_null() || *name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return r#false != 0;
    }
    if *name.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while *name.offset(i as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_uchar = *name.offset(i as isize) as ::core::ffi::c_uchar;
        if !(*(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalpha.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            || *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            || c as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '_' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '.' as ::core::ffi::c_int)
        {
            return r#false != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ensure_dir(mut path: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut st: stat = stat {
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
    if stat(path, &raw mut st) == 0 as ::core::ffi::c_int {
        if !(st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Regular file in the way of directory at %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                path,
            );
            *__errno_location() = ENOTDIR;
            return -ENOTDIR;
        }
    } else if mkdir(path, 0o755 as __mode_t) != 0 as ::core::ffi::c_int
        && *__errno_location() != EEXIST
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create directory %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mkdir_recursive(mut path: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = path;
    if *p as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        p = p.offset(1);
    }
    while *p != 0 {
        while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        let mut saved: ::core::ffi::c_char = *p;
        *p = '\0' as ::core::ffi::c_char;
        if ensure_dir(path) != 0 {
            return -1 as ::core::ffi::c_int;
        }
        *p = saved;
        if *p != 0 {
            p = p.offset(1);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ensure_path(mut path: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if path.is_null() || *path == 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut path_buf: [::core::ffi::c_char; 4096] = [0; 4096];
    safe_strcpy(
        &raw mut path_buf as *mut ::core::ffi::c_char,
        path,
        PATH_MAX as size_t,
        0 as size_t,
    );
    let mut dir: *mut ::core::ffi::c_char = dirname(&raw mut path_buf as *mut ::core::ffi::c_char);
    return _mkdir_recursive(dir);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mkdir_recursive(
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if path.is_null() || *path == 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut path_copy: [::core::ffi::c_char; 4096] = [0; 4096];
    safe_strcpy(
        &raw mut path_copy as *mut ::core::ffi::c_char,
        path,
        PATH_MAX as size_t,
        0 as size_t,
    );
    return _mkdir_recursive(&raw mut path_copy as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn _chmod_path(
    mut path: *const ::core::ffi::c_char,
    mut mode: mode_t,
    mut fd_out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = open(path, O_RDONLY | O_DIRECTORY | O_NOFOLLOW);
    let mut is_dir: bool = fd >= 0 as ::core::ffi::c_int;
    if !is_dir {
        if *__errno_location() == ENOTDIR {
            fd = open(path, O_RDONLY | O_NOFOLLOW);
        }
        if fd < 0 as ::core::ffi::c_int {
            if *__errno_location() == ELOOP {
                return 0 as ::core::ffi::c_int;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to open %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                path,
                strerror(*__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if fchmod(fd, mode) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to fchmod %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
        close(fd);
        return -1 as ::core::ffi::c_int;
    }
    if !is_dir || fd_out.is_null() {
        close(fd);
        fd = -1 as ::core::ffi::c_int;
    }
    if !fd_out.is_null() {
        *fd_out = fd;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chmod_path(
    mut path: *const ::core::ffi::c_char,
    mut mode: mode_t,
) -> ::core::ffi::c_int {
    if path.is_null() || *path == 0 {
        return -1 as ::core::ffi::c_int;
    }
    return _chmod_path(path, mode, ::core::ptr::null_mut::<::core::ffi::c_int>());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chmod_parent(
    mut path: *const ::core::ffi::c_char,
    mut mode: mode_t,
) -> ::core::ffi::c_int {
    if path.is_null() || *path == 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut path_buf: [::core::ffi::c_char; 4096] = [0; 4096];
    safe_strcpy(
        &raw mut path_buf as *mut ::core::ffi::c_char,
        path,
        PATH_MAX as size_t,
        0 as size_t,
    );
    let mut dir: *mut ::core::ffi::c_char = dirname(&raw mut path_buf as *mut ::core::ffi::c_char);
    return _chmod_path(dir, mode, ::core::ptr::null_mut::<::core::ffi::c_int>());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chmod_recursive(
    mut path: *const ::core::ffi::c_char,
    mut mode: mode_t,
) -> ::core::ffi::c_int {
    if path.is_null() || *path == 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut fd: ::core::ffi::c_int = 0;
    if _chmod_path(path, mode, &raw mut fd) != 0 {
        return -1 as ::core::ffi::c_int;
    }
    if fd < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut dir: *mut DIR = fdopendir(fd);
    if dir.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to fdopendir %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
        close(fd);
        return -1 as ::core::ffi::c_int;
    }
    let mut entry: *const dirent = ::core::ptr::null::<dirent>();
    let mut full_path: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut offset: size_t = snprintf(
        &raw mut full_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"%s/\0".as_ptr() as *const ::core::ffi::c_char,
        path,
    ) as size_t;
    if offset >= PATH_MAX as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Path too long to recur for chmod\0".as_ptr() as *const ::core::ffi::c_char,
        );
        closedir(dir);
        return -1 as ::core::ffi::c_int;
    }
    let mut path_ptr: *mut ::core::ffi::c_char =
        (&raw mut full_path as *mut ::core::ffi::c_char).offset(offset as isize);
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        if strcmp(
            &raw const (*entry).d_name as *const ::core::ffi::c_char,
            b".\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw const (*entry).d_name as *const ::core::ffi::c_char,
                b"..\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            continue;
        }
        safe_strcpy(
            path_ptr,
            &raw const (*entry).d_name as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>().wrapping_sub(offset),
            0 as size_t,
        );
        if chmod_recursive(&raw mut full_path as *mut ::core::ffi::c_char, mode)
            != 0 as ::core::ffi::c_int
        {
            result = -1 as ::core::ffi::c_int;
        }
    }
    closedir(dir);
    return result;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PATH_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
