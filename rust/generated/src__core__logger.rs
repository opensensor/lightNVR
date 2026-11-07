use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    unsafe fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn dirname(__path: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn closelog();
    unsafe fn openlog(
        __ident: *const ::core::ffi::c_char,
        __option: ::core::ffi::c_int,
        __facility: ::core::ffi::c_int,
    );
    unsafe fn syslog(__pri: ::core::ffi::c_int, __fmt: *const ::core::ffi::c_char, ...);
    unsafe fn init_json_logger(filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn shutdown_json_logger();
    unsafe fn write_json_log(
        level: log_level_t,
        timestamp: *const ::core::ffi::c_char,
        message: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn json_log_rotate(
        max_size: size_t,
        max_files: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mkdir_recursive(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
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
pub type __useconds_t = ::core::ffi::c_uint;
pub type __clockid_t = ::core::ffi::c_int;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __glibc_reserved: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const PTHREAD_MUTEX_TIMED_NP: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE_NP: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK_NP: Self = Self(2);
    pub const PTHREAD_MUTEX_ADAPTIVE_NP: Self = Self(3);
    pub const PTHREAD_MUTEX_NORMAL: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK: Self = Self(2);
    pub const PTHREAD_MUTEX_DEFAULT: Self = Self(0);
    pub const PTHREAD_MUTEX_FAST_NP: Self = Self(0);
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub log_file: *mut FILE,
    pub log_level: log_level_t,
    pub console_logging: ::core::ffi::c_int,
    pub log_filename: [::core::ffi::c_char; 256],
    pub mutex: pthread_mutex_t,
    pub syslog_enabled: ::core::ffi::c_int,
    pub syslog_ident: [::core::ffi::c_char; 64],
    pub shutdown: ::core::ffi::c_int,
    pub initialized: ::core::ffi::c_int,
}
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const LOG_ERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LOG_WARNING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LOG_INFO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LOG_DEBUG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LOG_PID: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LOG_CONS: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
static mut logger: C2Rust_Unnamed_0 = unsafe {
    C2Rust_Unnamed_0 {
        log_file: ::core::ptr::null_mut::<FILE>(),
        log_level: log_level_t::LOG_LEVEL_INFO,
        console_logging: 1 as ::core::ffi::c_int,
        log_filename: ::core::mem::transmute::<
            [u8; 256],
            [::core::ffi::c_char; 256],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0 as ::core::ffi::c_int,
                __count: 0 as ::core::ffi::c_uint,
                __owner: 0 as ::core::ffi::c_int,
                __nusers: 0 as ::core::ffi::c_uint,
                __kind: C2Rust_Unnamed::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
                __spins: 0 as ::core::ffi::c_short,
                __glibc_reserved: 0 as ::core::ffi::c_short,
                __list: __pthread_internal_list {
                    __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                    __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
                },
            },
        },
        syslog_enabled: 0 as ::core::ffi::c_int,
        syslog_ident: ::core::mem::transmute::<
            [u8; 64],
            [::core::ffi::c_char; 64],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        shutdown: 0 as ::core::ffi::c_int,
        initialized: 0 as ::core::ffi::c_int,
    }
};
#[thread_local]
static mut tls_log_component: [::core::ffi::c_char; 64] = [
    0 as ::core::ffi::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[thread_local]
static mut tls_log_stream: [::core::ffi::c_char; 128] = [
    0 as ::core::ffi::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_set_thread_context(
    mut component: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
) {
    safe_strcpy(
        &raw mut tls_log_component as *mut ::core::ffi::c_char,
        if !component.is_null() {
            component
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut tls_log_stream as *mut ::core::ffi::c_char,
        if !stream_name.is_null() {
            stream_name
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_clear_thread_context() {
    tls_log_component[0usize] = '\0' as ::core::ffi::c_char;
    tls_log_stream[0usize] = '\0' as ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_get_thread_component() -> *const ::core::ffi::c_char {
    return &raw mut tls_log_component as *mut ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_get_thread_stream() -> *const ::core::ffi::c_char {
    return &raw mut tls_log_stream as *mut ::core::ffi::c_char;
}
static mut log_level_strings: [*const ::core::ffi::c_char; 4] = [
    b"ERROR\0".as_ptr() as *const ::core::ffi::c_char,
    b"WARN\0".as_ptr() as *const ::core::ffi::c_char,
    b"INFO\0".as_ptr() as *const ::core::ffi::c_char,
    b"DEBUG\0".as_ptr() as *const ::core::ffi::c_char,
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_logger() -> ::core::ffi::c_int {
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const logger.initialized) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    ::core::ptr::write_volatile(&raw mut logger.shutdown, 0 as ::core::ffi::c_int);
    if pthread_mutex_init(
        &raw mut logger.mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"Failed to initialize logger mutex\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if logger.log_file.is_null()
        && logger.log_filename[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        logger.log_file = stderr;
    }
    if logger.log_filename[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && Some(
            init_json_logger
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
        )
        .is_some()
    {
        let mut json_log_filename: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut json_log_filename as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s.json\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut logger.log_filename as *mut ::core::ffi::c_char,
        );
        init_json_logger(&raw mut json_log_filename as *mut ::core::ffi::c_char);
    }
    ::core::ptr::write_volatile(&raw mut logger.initialized, 1 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_logger_available() -> ::core::ffi::c_int {
    return (::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const logger.initialized) != 0
        && ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const logger.shutdown) == 0)
        as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_logger() {
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const logger.shutdown) != 0
        || ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const logger.initialized) == 0
    {
        return;
    }
    ::core::ptr::write_volatile(&raw mut logger.shutdown, 1 as ::core::ffi::c_int);
    ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
    usleep(10000 as __useconds_t);
    pthread_mutex_lock(&raw mut logger.mutex);
    if !logger.log_file.is_null() && logger.log_file != stdout && logger.log_file != stderr {
        fclose(logger.log_file);
        logger.log_file = ::core::ptr::null_mut::<FILE>();
    }
    if logger.syslog_enabled != 0 {
        closelog();
        logger.syslog_enabled = 0 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut logger.mutex);
    ::core::ptr::write_volatile(&raw mut logger.shutdown, 2 as ::core::ffi::c_int);
    ::core::ptr::write_volatile(&raw mut logger.initialized, 0 as ::core::ffi::c_int);
    ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
    pthread_mutex_destroy(&raw mut logger.mutex);
    if Some(shutdown_json_logger as unsafe extern "C" fn() -> ()).is_some() {
        shutdown_json_logger();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_log_level(mut level: log_level_t) {
    if level.0 >= log_level_t::LOG_LEVEL_ERROR.0 && level.0 <= log_level_t::LOG_LEVEL_DEBUG.0 {
        pthread_mutex_lock(&raw mut logger.mutex);
        let mut old_level: log_level_t = logger.log_level;
        logger.log_level = level;
        pthread_mutex_unlock(&raw mut logger.mutex);
        if old_level.0 != log_level_t::LOG_LEVEL_ERROR.0
            || level.0 != log_level_t::LOG_LEVEL_ERROR.0
        {
            fprintf(
                stderr,
                b"[LOG LEVEL CHANGE] %s -> %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                log_level_strings[old_level.0 as usize],
                log_level_strings[level.0 as usize],
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_log_file(
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if filename.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut logger.mutex);
    if !logger.log_file.is_null() && logger.log_file != stdout && logger.log_file != stderr {
        fclose(logger.log_file);
        logger.log_file = ::core::ptr::null_mut::<FILE>();
    }
    let mut dir_path: *mut ::core::ffi::c_char = strdup(filename);
    if dir_path.is_null() {
        pthread_mutex_unlock(&raw mut logger.mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut dir: *const ::core::ffi::c_char = dirname(dir_path);
    if mkdir_recursive(dir) != 0 as ::core::ffi::c_int {
        free(dir_path as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut logger.mutex);
        return -1 as ::core::ffi::c_int;
    }
    free(dir_path as *mut ::core::ffi::c_void);
    let mut log_fd: ::core::ffi::c_int = open(
        filename,
        O_WRONLY | O_CREAT | O_APPEND,
        0o640 as ::core::ffi::c_int,
    );
    logger.log_file = if log_fd >= 0 as ::core::ffi::c_int {
        fdopen(log_fd, b"a\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<FILE>()
    };
    if logger.log_file.is_null() {
        if log_fd >= 0 as ::core::ffi::c_int {
            close(log_fd);
        }
        pthread_mutex_unlock(&raw mut logger.mutex);
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut logger.log_filename as *mut ::core::ffi::c_char,
        filename,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    pthread_mutex_unlock(&raw mut logger.mutex);
    if Some(
        init_json_logger as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    )
    .is_some()
    {
        let mut json_log_filename: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut json_log_filename as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s.json\0".as_ptr() as *const ::core::ffi::c_char,
            filename,
        );
        init_json_logger(&raw mut json_log_filename as *mut ::core::ffi::c_char);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_error(mut format: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    let mut args: ::core::ffi::VaList;
    args = c2rust_args.clone();
    log_message_v(log_level_t::LOG_LEVEL_ERROR, format, args.clone());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_warn(mut format: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    let mut args: ::core::ffi::VaList;
    args = c2rust_args.clone();
    log_message_v(log_level_t::LOG_LEVEL_WARN, format, args.clone());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_info(mut format: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    let mut args: ::core::ffi::VaList;
    args = c2rust_args.clone();
    log_message_v(log_level_t::LOG_LEVEL_INFO, format, args.clone());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_debug(mut format: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    let mut args: ::core::ffi::VaList;
    args = c2rust_args.clone();
    log_message_v(log_level_t::LOG_LEVEL_DEBUG, format, args.clone());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_message(
    mut level: log_level_t,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut args: ::core::ffi::VaList;
    args = c2rust_args.clone();
    log_message_v(level, format, args.clone());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sanitize_for_logging(
    mut str: *const ::core::ffi::c_char,
    mut max_len: size_t,
) -> *const ::core::ffi::c_char {
    static mut sanitized: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut i: size_t = 0;
    if str.is_null() {
        return b"(null)\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if max_len > ::core::mem::size_of::<[::core::ffi::c_char; 4096]>().wrapping_sub(1usize) {
        max_len =
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>().wrapping_sub(1usize) as size_t;
    }
    i = 0 as size_t;
    while i < max_len && *str.offset(i as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        if *str.offset(i as isize) as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
            && *str.offset(i as isize) as ::core::ffi::c_int <= 126 as ::core::ffi::c_int
            || *str.offset(i as isize) as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            || *str.offset(i as isize) as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
        {
            sanitized[i] = *str.offset(i as isize);
        } else {
            sanitized[i] = '?' as ::core::ffi::c_char;
        }
        i = i.wrapping_add(1);
    }
    sanitized[i] = '\0' as ::core::ffi::c_char;
    return &raw mut sanitized as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn build_ctx_prefix(
    mut buf: *mut ::core::ffi::c_char,
    mut bufsz: size_t,
    mut component: *const ::core::ffi::c_char,
    mut stream: *const ::core::ffi::c_char,
) {
    if !component.is_null() && *component.offset(0isize) as ::core::ffi::c_int != 0 {
        if !stream.is_null() && *stream.offset(0isize) as ::core::ffi::c_int != 0 {
            snprintf(
                buf,
                bufsz,
                b"[%s] [%s] \0".as_ptr() as *const ::core::ffi::c_char,
                component,
                stream,
            );
        } else {
            snprintf(
                buf,
                bufsz,
                b"[%s] \0".as_ptr() as *const ::core::ffi::c_char,
                component,
            );
        }
    } else {
        *buf.offset(0isize) = '\0' as ::core::ffi::c_char;
    };
}
unsafe extern "C" fn do_log_internal(
    mut level: log_level_t,
    mut component: *const ::core::ffi::c_char,
    mut stream: *const ::core::ffi::c_char,
    mut format: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) {
    if level.0 > logger.log_level.0 {
        return;
    }
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut now);
    let mut tm_buf: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    localtime_r(&raw mut now.tv_sec, &raw mut tm_buf);
    let mut timestamp: [::core::ffi::c_char; 32] = [0; 32];
    let mut offset: size_t = strftime(
        &raw mut timestamp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%Y-%m-%d %H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut tm_buf,
    );
    let mut msec: ::core::ffi::c_uint = (now.tv_nsec as ::core::ffi::c_ulong)
        .wrapping_div(1000000 as ::core::ffi::c_ulong)
        as ::core::ffi::c_uint;
    snprintf(
        (&raw mut timestamp as *mut ::core::ffi::c_char).offset(offset as isize),
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>().wrapping_sub(offset),
        b".%03u\0".as_ptr() as *const ::core::ffi::c_char,
        msec,
    );
    let mut message: [::core::ffi::c_char; 4096] = [0; 4096];
    vsnprintf(
        &raw mut message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        format,
        args.clone(),
    );
    let mut ctx_prefix: [::core::ffi::c_char; 224] = [
        0 as ::core::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    build_ctx_prefix(
        &raw mut ctx_prefix as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 224]>(),
        component,
        stream,
    );
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const logger.shutdown) != 0 {
        let mut console: *mut FILE = if level.0 == log_level_t::LOG_LEVEL_ERROR.0 {
            stderr
        } else {
            stdout
        };
        fprintf(
            console,
            b"[%s] [%s] %s%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut timestamp as *mut ::core::ffi::c_char,
            log_level_strings[level.0 as usize],
            &raw mut ctx_prefix as *mut ::core::ffi::c_char,
            &raw mut message as *mut ::core::ffi::c_char,
        );
        fflush(console);
        return;
    }
    pthread_mutex_lock(&raw mut logger.mutex);
    if !logger.log_file.is_null() && logger.log_file != stdout && logger.log_file != stderr {
        fprintf(
            logger.log_file,
            b"[%s] [%s] %s%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut timestamp as *mut ::core::ffi::c_char,
            log_level_strings[level.0 as usize],
            &raw mut ctx_prefix as *mut ::core::ffi::c_char,
            &raw mut message as *mut ::core::ffi::c_char,
        );
        fflush(logger.log_file);
    }
    let mut console_0: *mut FILE = if level.0 == log_level_t::LOG_LEVEL_ERROR.0 {
        stderr
    } else {
        stdout
    };
    fprintf(
        console_0,
        b"[%s] [%s] %s%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut timestamp as *mut ::core::ffi::c_char,
        log_level_strings[level.0 as usize],
        &raw mut ctx_prefix as *mut ::core::ffi::c_char,
        &raw mut message as *mut ::core::ffi::c_char,
    );
    fflush(console_0);
    if logger.syslog_enabled != 0 {
        let mut syslog_priority: ::core::ffi::c_int = 0;
        match level {
            log_level_t::LOG_LEVEL_ERROR => {
                syslog_priority = LOG_ERR;
            }
            log_level_t::LOG_LEVEL_WARN => {
                syslog_priority = LOG_WARNING;
            }
            log_level_t::LOG_LEVEL_INFO => {
                syslog_priority = LOG_INFO;
            }
            log_level_t::LOG_LEVEL_DEBUG => {
                syslog_priority = LOG_DEBUG;
            }
            _ => {
                syslog_priority = LOG_INFO;
            }
        }
        syslog(
            syslog_priority,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut message as *mut ::core::ffi::c_char,
        );
    }
    pthread_mutex_unlock(&raw mut logger.mutex);
    if Some(
        write_json_log
            as unsafe extern "C" fn(
                log_level_t,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
    )
    .is_some()
    {
        let mut iso_timestamp: [::core::ffi::c_char; 32] = [0; 32];
        strftime(
            &raw mut iso_timestamp as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%Y-%m-%dT%H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut tm_buf,
        );
        write_json_log(
            level,
            &raw mut iso_timestamp as *mut ::core::ffi::c_char,
            &raw mut message as *mut ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_message_v(
    mut level: log_level_t,
    mut format: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) {
    do_log_internal(
        level,
        &raw mut tls_log_component as *mut ::core::ffi::c_char,
        &raw mut tls_log_stream as *mut ::core::ffi::c_char,
        format,
        args.clone(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _log_message_ctx(
    mut level: log_level_t,
    mut component: *const ::core::ffi::c_char,
    mut stream: *const ::core::ffi::c_char,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut eff_comp: *const ::core::ffi::c_char =
        if tls_log_component[0usize] as ::core::ffi::c_int != 0 {
            &raw mut tls_log_component as *mut ::core::ffi::c_char
        } else if !component.is_null() && *component.offset(0isize) as ::core::ffi::c_int != 0 {
            component
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        };
    let mut eff_stream: *const ::core::ffi::c_char =
        if tls_log_stream[0usize] as ::core::ffi::c_int != 0 {
            &raw mut tls_log_stream as *mut ::core::ffi::c_char
        } else if !stream.is_null() && *stream.offset(0isize) as ::core::ffi::c_int != 0 {
            stream
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        };
    let mut args: ::core::ffi::VaList;
    args = c2rust_args.clone();
    do_log_internal(level, eff_comp, eff_stream, format, args.clone());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_log_level_string(
    mut level: log_level_t,
) -> *const ::core::ffi::c_char {
    if level.0 >= log_level_t::LOG_LEVEL_ERROR.0 && level.0 <= log_level_t::LOG_LEVEL_DEBUG.0 {
        return log_level_strings[level.0 as usize];
    }
    return b"UNKNOWN\0".as_ptr() as *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_log_level_string(
    mut log_level: *const ::core::ffi::c_char,
) -> log_level_t {
    let mut level_value: ::core::ffi::c_int = log_level_t::LOG_LEVEL_INFO.0 as ::core::ffi::c_int;
    if log_level.is_null() {
        return log_level_t(level_value as ::core::ffi::c_uint);
    }
    if strcasecmp(log_level, b"error\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        level_value = log_level_t::LOG_LEVEL_ERROR.0 as ::core::ffi::c_int;
    } else if strcasecmp(
        log_level,
        b"warning\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(log_level, b"warn\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        level_value = log_level_t::LOG_LEVEL_WARN.0 as ::core::ffi::c_int;
    } else if strcasecmp(log_level, b"info\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        level_value = log_level_t::LOG_LEVEL_INFO.0 as ::core::ffi::c_int;
    } else if strcasecmp(log_level, b"debug\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        level_value = log_level_t::LOG_LEVEL_DEBUG.0 as ::core::ffi::c_int;
    }
    return log_level_t(level_value as ::core::ffi::c_uint);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_rotate(
    mut max_size: size_t,
    mut max_files: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if logger.log_filename[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut logger.mutex);
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
    if stat(
        &raw mut logger.log_filename as *mut ::core::ffi::c_char,
        &raw mut st,
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(&raw mut logger.mutex);
        return -1 as ::core::ffi::c_int;
    }
    if (st.st_size as size_t) < max_size {
        pthread_mutex_unlock(&raw mut logger.mutex);
        return 0 as ::core::ffi::c_int;
    }
    if !logger.log_file.is_null() && logger.log_file != stdout && logger.log_file != stderr {
        fclose(logger.log_file);
        logger.log_file = ::core::ptr::null_mut::<FILE>();
    }
    let mut old_path: [::core::ffi::c_char; 512] = [0; 512];
    let mut new_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut old_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s.%d\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut logger.log_filename as *mut ::core::ffi::c_char,
        max_files,
    );
    unlink(&raw mut old_path as *mut ::core::ffi::c_char);
    let mut i: ::core::ffi::c_int = max_files - 1 as ::core::ffi::c_int;
    while i > 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut old_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s.%d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut logger.log_filename as *mut ::core::ffi::c_char,
            i,
        );
        snprintf(
            &raw mut new_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s.%d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut logger.log_filename as *mut ::core::ffi::c_char,
            i + 1 as ::core::ffi::c_int,
        );
        rename(
            &raw mut old_path as *mut ::core::ffi::c_char,
            &raw mut new_path as *mut ::core::ffi::c_char,
        );
        i -= 1;
    }
    snprintf(
        &raw mut new_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s.1\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut logger.log_filename as *mut ::core::ffi::c_char,
    );
    rename(
        &raw mut logger.log_filename as *mut ::core::ffi::c_char,
        &raw mut new_path as *mut ::core::ffi::c_char,
    );
    let mut rot_fd: ::core::ffi::c_int = open(
        &raw mut logger.log_filename as *mut ::core::ffi::c_char,
        O_WRONLY | O_CREAT | O_APPEND,
        0o640 as ::core::ffi::c_int,
    );
    logger.log_file = if rot_fd >= 0 as ::core::ffi::c_int {
        fdopen(rot_fd, b"a\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<FILE>()
    };
    if logger.log_file.is_null() {
        if rot_fd >= 0 as ::core::ffi::c_int {
            close(rot_fd);
        }
        pthread_mutex_unlock(&raw mut logger.mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut logger.mutex);
    if Some(
        json_log_rotate as unsafe extern "C" fn(size_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    )
    .is_some()
    {
        let mut json_log_filename: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut json_log_filename as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s.json\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut logger.log_filename as *mut ::core::ffi::c_char,
        );
        json_log_rotate(max_size, max_files);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn enable_syslog(
    mut ident: *const ::core::ffi::c_char,
    mut facility: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if ident.is_null() || *ident.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut logger.mutex);
    if logger.syslog_enabled != 0 {
        closelog();
    }
    safe_strcpy(
        &raw mut logger.syslog_ident as *mut ::core::ffi::c_char,
        ident,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    openlog(
        &raw mut logger.syslog_ident as *mut ::core::ffi::c_char,
        LOG_PID | LOG_CONS,
        facility,
    );
    logger.syslog_enabled = 1 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut logger.mutex);
    log_info(
        b"Syslog logging enabled (ident: %s, facility: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        ident,
        facility,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn disable_syslog() {
    let mut was_enabled: ::core::ffi::c_int = 0;
    pthread_mutex_lock(&raw mut logger.mutex);
    was_enabled = logger.syslog_enabled;
    if logger.syslog_enabled != 0 {
        closelog();
        logger.syslog_enabled = 0 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut logger.mutex);
    if was_enabled != 0 {
        log_info(b"Syslog logging disabled\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_syslog_enabled() -> ::core::ffi::c_int {
    let mut enabled: ::core::ffi::c_int = 0;
    pthread_mutex_lock(&raw mut logger.mutex);
    enabled = logger.syslog_enabled;
    pthread_mutex_unlock(&raw mut logger.mutex);
    return enabled;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
