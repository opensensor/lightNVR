use ::libc;
unsafe extern "C" {
    unsafe fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn getcwd(__buf: *mut ::core::ffi::c_char, __size: size_t) -> *mut ::core::ffi::c_char;
    unsafe fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn getpid() -> __pid_t;
    unsafe fn setsid() -> __pid_t;
    unsafe fn fork() -> __pid_t;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn ftruncate(__fd: ::core::ffi::c_int, __length: __off_t) -> ::core::ffi::c_int;
    unsafe fn lockf(
        __fd: ::core::ffi::c_int,
        __cmd: ::core::ffi::c_int,
        __len: __off_t,
    ) -> ::core::ffi::c_int;
    unsafe fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sigismember(
        __set: *const sigset_t,
        __signo: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    unsafe fn sigpending(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigtimedwait(
        __set: *const sigset_t,
        __info: *mut siginfo_t,
        __timeout: *const timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_sigmask(
        __how: ::core::ffi::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fchmod(__fd: ::core::ffi::c_int, __mode: __mode_t) -> ::core::ffi::c_int;
    unsafe fn umask(__mask: __mode_t) -> __mode_t;
    unsafe fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...)
    -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_atfork(
        __prepare: Option<unsafe extern "C" fn() -> ()>,
        __parent: Option<unsafe extern "C" fn() -> ()>,
        __child: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn ensure_path(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn request_background_abort();
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    static mut running: bool;
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn exit(__status: ::core::ffi::c_int) -> !;
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type pthread_once_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub __pad0: ::core::ffi::c_int,
    pub _sifields: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub _pad: [::core::ffi::c_int; 28],
    pub _kill: C2Rust_Unnamed_8,
    pub _timer: C2Rust_Unnamed_7,
    pub _rt: C2Rust_Unnamed_6,
    pub _sigchld: C2Rust_Unnamed_5,
    pub _sigfault: C2Rust_Unnamed_2,
    pub _sigpoll: C2Rust_Unnamed_1,
    pub _sigsys: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub _addr_bnd: C2Rust_Unnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::core::ffi::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub si_tid: ::core::ffi::c_int,
    pub si_overrun: ::core::ffi::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2Rust_Unnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::core::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut siginfo_t, *mut ::core::ffi::c_void) -> (),
    >,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: ::core::ffi::c_short,
    pub l_whence: ::core::ffi::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_TLOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const SA_RESTART: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SIG_BLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ESRCH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EPIPE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const F_GETLK64: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_GETLK: ::core::ffi::c_int = F_GETLK64;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const F_DUPFD_CLOEXEC: ::core::ffi::c_int = 1030 as ::core::ffi::c_int;
pub const F_WRLCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_UNLCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTHREAD_ONCE_INIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut pid_file_path: [::core::ffi::c_char; 512] = unsafe {
    ::core::mem::transmute::<
        [u8; 512],
        [::core::ffi::c_char; 512],
    >(
        *b"/run/lightnvr.pid\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut owned_pid_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_11::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut owned_pid_fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
static mut owned_pid_process: pid_t = 0 as pid_t;
static mut owned_pid_atfork_once: pthread_once_t = PTHREAD_ONCE_INIT;
unsafe extern "C" fn owned_pid_atfork_prepare() {
    pthread_mutex_lock(&raw mut owned_pid_mutex);
}
unsafe extern "C" fn owned_pid_atfork_parent() {
    pthread_mutex_unlock(&raw mut owned_pid_mutex);
}
unsafe extern "C" fn owned_pid_atfork_child() {
    if owned_pid_fd >= 0 as ::core::ffi::c_int {
        close(owned_pid_fd);
    }
    owned_pid_fd = -1 as ::core::ffi::c_int;
    owned_pid_process = 0 as ::core::ffi::c_int as pid_t;
    pthread_mutex_unlock(&raw mut owned_pid_mutex);
}
unsafe extern "C" fn register_owned_pid_atfork() {
    pthread_atfork(
        Some(owned_pid_atfork_prepare as unsafe extern "C" fn() -> ()),
        Some(owned_pid_atfork_parent as unsafe extern "C" fn() -> ()),
        Some(owned_pid_atfork_child as unsafe extern "C" fn() -> ()),
    );
}
unsafe extern "C" fn notify_startup_status(
    mut fd: ::core::ffi::c_int,
    mut status: ::core::ffi::c_uchar,
) {
    let mut n: ssize_t = 0;
    let mut block_set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut old_set: sigset_t = __sigset_t { __val: [0; 16] };
    if sigemptyset(&raw mut block_set) != 0 as ::core::ffi::c_int
        || sigaddset(&raw mut block_set, SIGPIPE) != 0 as ::core::ffi::c_int
        || pthread_sigmask(SIG_BLOCK, &raw mut block_set, &raw mut old_set)
            != 0 as ::core::ffi::c_int
    {
        close(fd);
        return;
    }
    let mut pending_before: sigset_t = __sigset_t { __val: [0; 16] };
    let mut had_pending_sigpipe: bool = sigpending(&raw mut pending_before)
        == 0 as ::core::ffi::c_int
        && sigismember(&raw mut pending_before, SIGPIPE) == 1 as ::core::ffi::c_int;
    loop {
        n = write(
            fd,
            &raw mut status as *const ::core::ffi::c_void,
            1 as size_t,
        );
        if !(n < 0 as ssize_t && *__errno_location() == EINTR) {
            break;
        }
    }
    let mut write_error: ::core::ffi::c_int = *__errno_location();
    if n < 0 as ssize_t && write_error == EPIPE && !had_pending_sigpipe {
        let mut no_wait: timespec = timespec {
            tv_sec: 0 as __time_t,
            tv_nsec: 0 as __syscall_slong_t,
        };
        let mut sigwait_ret: ::core::ffi::c_int = 0;
        loop {
            sigwait_ret = sigtimedwait(
                &raw mut block_set,
                ::core::ptr::null_mut::<siginfo_t>(),
                &raw mut no_wait,
            );
            if !(sigwait_ret < 0 as ::core::ffi::c_int && *__errno_location() == EINTR) {
                break;
            }
        }
    }
    pthread_sigmask(
        SIG_SETMASK,
        &raw mut old_set,
        ::core::ptr::null_mut::<__sigset_t>(),
    );
    *__errno_location() = write_error;
    if n != 1 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to report daemon startup status to parent: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            if n < 0 as ssize_t {
                strerror(*__errno_location())
            } else {
                b"short write\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    close(fd);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_daemon(
    mut pid_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !pid_file.is_null() {
        safe_strcpy(
            &raw mut pid_file_path as *mut ::core::ffi::c_char,
            pid_file,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    let mut daemon_check: ::core::ffi::c_int =
        check_running_daemon(&raw mut pid_file_path as *mut ::core::ffi::c_char);
    if daemon_check > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Daemon is already running\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    } else if daemon_check < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to verify daemon state from PID file: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut pid_file_path as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting daemon mode with PID file: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut pid_file_path as *mut ::core::ffi::c_char,
    );
    let mut startup_cwd: *mut ::core::ffi::c_char =
        getcwd(::core::ptr::null_mut::<::core::ffi::c_char>(), 0 as size_t);
    if !startup_cwd.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Current working directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            startup_cwd,
        );
        free(startup_cwd as *mut ::core::ffi::c_void);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Current working directory unavailable: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    let mut status_pipe: [::core::ffi::c_int; 2] = [0; 2];
    if pipe(&raw mut status_pipe as *mut ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create startup status pipe: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_once(
        &raw mut owned_pid_atfork_once,
        Some(register_owned_pid_atfork as unsafe extern "C" fn() -> ()),
    );
    let mut pid: pid_t = fork();
    if pid < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to fork daemon process: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(status_pipe[0usize]);
        close(status_pipe[1usize]);
        return -1 as ::core::ffi::c_int;
    }
    if pid > 0 as ::core::ffi::c_int {
        close(status_pipe[1usize]);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Parent process waiting for child startup, child PID: %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            pid,
        );
        let mut child_status: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
        let mut n: ssize_t = 0;
        loop {
            n = read(
                status_pipe[0usize],
                &raw mut child_status as *mut ::core::ffi::c_void,
                1 as size_t,
            );
            if !(n < 0 as ssize_t && *__errno_location() == EINTR) {
                break;
            }
        }
        close(status_pipe[0usize]);
        if n == 1 as ssize_t && child_status as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Child daemon initialization succeeded, child PID: %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pid,
            );
            exit(EXIT_SUCCESS);
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Child daemon initialization failed, child PID: %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            pid,
        );
        exit(EXIT_FAILURE);
    }
    close(status_pipe[0usize]);
    if status_pipe[1usize] <= STDERR_FILENO {
        let mut status_fd: ::core::ffi::c_int = fcntl(
            status_pipe[1usize],
            F_DUPFD_CLOEXEC,
            STDERR_FILENO + 1 as ::core::ffi::c_int,
        );
        if status_fd < 0 as ::core::ffi::c_int {
            notify_startup_status(status_pipe[1usize], 0 as ::core::ffi::c_uchar);
            return -1 as ::core::ffi::c_int;
        }
        close(status_pipe[1usize]);
        status_pipe[1usize] = status_fd;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Child process starting daemon initialization, PID: %d\0".as_ptr()
            as *const ::core::ffi::c_char,
        getpid(),
    );
    let mut sid: pid_t = setsid();
    if sid < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create session: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        notify_startup_status(status_pipe[1usize], 0 as ::core::ffi::c_uchar);
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Created new session with SID: %d\0".as_ptr() as *const ::core::ffi::c_char,
        sid,
    );
    let mut cwd: *mut ::core::ffi::c_char =
        getcwd(::core::ptr::null_mut::<::core::ffi::c_char>(), 0 as size_t);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Keeping current working directory for SQLite compatibility: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        if !cwd.is_null() {
            cwd
        } else {
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if !cwd.is_null() {
        free(cwd as *mut ::core::ffi::c_void);
    }
    umask(0o27 as __mode_t);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Set daemon file creation mask to 0027\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut null_fd: ::core::ffi::c_int = open(
        b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
        O_RDWR,
    );
    if null_fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open /dev/null for stdio redirection: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        notify_startup_status(status_pipe[1usize], 0 as ::core::ffi::c_uchar);
        return -1 as ::core::ffi::c_int;
    }
    if dup2(null_fd, STDIN_FILENO) < 0 as ::core::ffi::c_int
        || dup2(null_fd, STDOUT_FILENO) < 0 as ::core::ffi::c_int
        || dup2(null_fd, STDERR_FILENO) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to redirect standard file descriptors to /dev/null: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        if null_fd > STDERR_FILENO {
            close(null_fd);
        }
        notify_startup_status(status_pipe[1usize], 0 as ::core::ffi::c_uchar);
        return -1 as ::core::ffi::c_int;
    }
    if null_fd > STDERR_FILENO {
        close(null_fd);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Redirected standard file descriptors to /dev/null; preserved other descriptors\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    memset(
        &raw mut sa as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sigaction>(),
    );
    sa.__sigaction_handler.sa_handler =
        Some(daemon_signal_handler as unsafe extern "C" fn(::core::ffi::c_int) -> ())
            as __sighandler_t;
    sigemptyset(&raw mut sa.sa_mask);
    sa.sa_flags = SA_RESTART;
    if sigaction(SIGTERM, &raw mut sa, ::core::ptr::null_mut::<sigaction>())
        != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set SIGTERM handler: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    if sigaction(SIGINT, &raw mut sa, ::core::ptr::null_mut::<sigaction>())
        != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set SIGINT handler: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    if sigaction(SIGHUP, &raw mut sa, ::core::ptr::null_mut::<sigaction>())
        != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set SIGHUP handler: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    if sigaction(SIGALRM, &raw mut sa, ::core::ptr::null_mut::<sigaction>())
        != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set SIGALRM handler: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Signal handlers configured\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut sa_pipe: sigaction = sigaction {
        __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    memset(
        &raw mut sa_pipe as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sigaction>(),
    );
    sa_pipe.__sigaction_handler.sa_handler = ::core::mem::transmute::<
        ::libc::intptr_t,
        __sighandler_t,
    >(1 as ::core::ffi::c_int as ::libc::intptr_t);
    sigemptyset(&raw mut sa_pipe.sa_mask);
    if sigaction(
        SIGPIPE,
        &raw mut sa_pipe,
        ::core::ptr::null_mut::<sigaction>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to ignore SIGPIPE: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"SIGPIPE ignored process-wide\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Writing PID file...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut fd: ::core::ffi::c_int =
        write_pid_file(&raw mut pid_file_path as *mut ::core::ffi::c_char);
    if fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to write PID file, daemon initialization failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        notify_startup_status(status_pipe[1usize], 0 as ::core::ffi::c_uchar);
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Daemon started successfully with PID %d\0".as_ptr() as *const ::core::ffi::c_char,
        getpid(),
    );
    notify_startup_status(status_pipe[1usize], 1 as ::core::ffi::c_uchar);
    usleep(100000 as ::core::ffi::c_int as __useconds_t);
    return fd;
}
unsafe extern "C" fn daemon_signal_handler(mut sig: ::core::ffi::c_int) {
    match sig {
        SIGTERM | SIGINT => {
            running = r#false != 0;
            request_background_abort();
        }
        SIGHUP | SIGALRM | _ => {}
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_pid_file(
    mut pid_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    pthread_once(
        &raw mut owned_pid_atfork_once,
        Some(register_owned_pid_atfork as unsafe extern "C" fn() -> ()),
    );
    pthread_mutex_lock(&raw mut owned_pid_mutex);
    if owned_pid_fd >= 0 as ::core::ffi::c_int && owned_pid_process == getpid() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"PID file already owned by this process\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut owned_pid_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if ensure_path(pid_file) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create PID file directory %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
        pthread_mutex_unlock(&raw mut owned_pid_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut fd: ::core::ffi::c_int = open(
        pid_file,
        O_RDWR | O_CREAT | O_NOFOLLOW | O_CLOEXEC | O_NONBLOCK,
        0o644 as ::core::ffi::c_int,
    );
    if fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
        pthread_mutex_unlock(&raw mut owned_pid_mutex);
        return -1 as ::core::ffi::c_int;
    }
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
    if fstat(fd, &raw mut st) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to stat PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
        close(fd);
        pthread_mutex_unlock(&raw mut owned_pid_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) || st.st_nlink != 1 as __nlink_t
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unsafe PID file %s: expected regular file with a single link\0".as_ptr()
                as *const ::core::ffi::c_char,
            pid_file,
        );
        close(fd);
        pthread_mutex_unlock(&raw mut owned_pid_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if lockf(fd, F_TLOCK, 0 as __off_t) < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to lock PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
        close(fd);
        pthread_mutex_unlock(&raw mut owned_pid_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if ftruncate(fd, 0 as __off_t) < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to truncate PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
        close(fd);
        pthread_mutex_unlock(&raw mut owned_pid_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut pid_str: [::core::ffi::c_char; 16] = [0; 16];
    sprintf(
        &raw mut pid_str as *mut ::core::ffi::c_char,
        b"%d\n\0".as_ptr() as *const ::core::ffi::c_char,
        getpid(),
    );
    let mut pid_len: size_t = strlen(&raw mut pid_str as *mut ::core::ffi::c_char);
    let mut total_written: size_t = 0 as size_t;
    while total_written < pid_len {
        let mut n: ssize_t = write(
            fd,
            (&raw mut pid_str as *mut ::core::ffi::c_char).offset(total_written as isize)
                as *const ::core::ffi::c_void,
            pid_len.wrapping_sub(total_written),
        );
        if n > 0 as ssize_t {
            total_written = total_written.wrapping_add(n as size_t);
        } else {
            if n < 0 as ssize_t && *__errno_location() == EINTR {
                continue;
            }
            if n == 0 as ssize_t {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to write to PID file %s: short write\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pid_file,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to write to PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    pid_file,
                    strerror(*__errno_location()),
                );
            }
            close(fd);
            pthread_mutex_unlock(&raw mut owned_pid_mutex);
            return -1 as ::core::ffi::c_int;
        }
    }
    fsync(fd);
    if fchmod(fd, 0o644 as __mode_t) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set permissions on PID file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Wrote PID %d to file %s\0".as_ptr() as *const ::core::ffi::c_char,
        getpid(),
        pid_file,
    );
    owned_pid_fd = fd;
    owned_pid_process = getpid() as pid_t;
    pthread_mutex_unlock(&raw mut owned_pid_mutex);
    return fd;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_pid_file(
    mut fd: ::core::ffi::c_int,
    mut pid_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if fd >= 0 as ::core::ffi::c_int {
        pthread_mutex_lock(&raw mut owned_pid_mutex);
        if fd != owned_pid_fd || owned_pid_process != getpid() {
            pthread_mutex_unlock(&raw mut owned_pid_mutex);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"PID file descriptor %d for %s is not owned (already released or replaced); skipping close\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                fd,
                pid_file,
            );
            return -1 as ::core::ffi::c_int;
        }
        owned_pid_fd = -1 as ::core::ffi::c_int;
        owned_pid_process = 0 as ::core::ffi::c_int as pid_t;
        let mut close_result: ::core::ffi::c_int = close(fd);
        let mut close_error: ::core::ffi::c_int = *__errno_location();
        pthread_mutex_unlock(&raw mut owned_pid_mutex);
        if close_result != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to close PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                pid_file,
                strerror(close_error),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Released PID file lock for %s (file left in place)\0".as_ptr()
            as *const ::core::ffi::c_char,
        pid_file,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn finish_running_probe(mut result: ::core::ffi::c_int) -> ::core::ffi::c_int {
    pthread_mutex_unlock(&raw mut owned_pid_mutex);
    return result;
}
unsafe extern "C" fn check_running_daemon(
    mut pid_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut owned_pid_mutex);
    if owned_pid_fd >= 0 as ::core::ffi::c_int && owned_pid_process == getpid() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Refusing PID startup probe for %s: current process already owns PID file lock\0"
                .as_ptr() as *const ::core::ffi::c_char,
            pid_file,
        );
        return finish_running_probe(-1 as ::core::ffi::c_int);
    }
    let mut fd: ::core::ffi::c_int = open(pid_file, O_RDWR | O_NOFOLLOW | O_CLOEXEC | O_NONBLOCK);
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() == ENOENT {
            return finish_running_probe(0 as ::core::ffi::c_int);
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
        return finish_running_probe(-1 as ::core::ffi::c_int);
    }
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
    if fstat(fd, &raw mut st) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to stat PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
        close(fd);
        return finish_running_probe(-1 as ::core::ffi::c_int);
    }
    if !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) || st.st_nlink != 1 as __nlink_t
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"PID file %s is not a safe regular single-link file\0".as_ptr()
                as *const ::core::ffi::c_char,
            pid_file,
        );
        close(fd);
        return finish_running_probe(-1 as ::core::ffi::c_int);
    }
    loop {
        if lockf(fd, F_TLOCK, 0 as __off_t) == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Found stale PID file %s (not locked)\0".as_ptr() as *const ::core::ffi::c_char,
                pid_file,
            );
            close(fd);
            return finish_running_probe(0 as ::core::ffi::c_int);
        }
        if *__errno_location() != EINTR {
            if *__errno_location() == EACCES || *__errno_location() == EAGAIN {
                break;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to lock PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                pid_file,
                strerror(*__errno_location()),
            );
            close(fd);
            return finish_running_probe(-1 as ::core::ffi::c_int);
        }
    }
    let mut pid_str: [::core::ffi::c_char; 16] = [0; 16];
    let mut bytes_read: ssize_t = 0;
    loop {
        bytes_read = read(
            fd,
            &raw mut pid_str as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>().wrapping_sub(1 as size_t),
        );
        if !(bytes_read < 0 as ssize_t && *__errno_location() == EINTR) {
            break;
        }
    }
    let mut extra: ::core::ffi::c_char = 0;
    let mut extra_read: ssize_t = -1 as ssize_t;
    if bytes_read > 0 as ssize_t {
        loop {
            extra_read = read(fd, &raw mut extra as *mut ::core::ffi::c_void, 1 as size_t);
            if !(extra_read < 0 as ssize_t && *__errno_location() == EINTR) {
                break;
            }
        }
    }
    close(fd);
    if bytes_read <= 0 as ssize_t
        || extra_read != 0 as ssize_t
        || !memchr(
            &raw mut pid_str as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            '\0' as ::core::ffi::c_int,
            bytes_read as size_t,
        )
        .is_null()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid PID file contents: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
        );
        return finish_running_probe(-1 as ::core::ffi::c_int);
    }
    pid_str[bytes_read as usize] = '\0' as ::core::ffi::c_char;
    let mut end_ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut pid_val: ::core::ffi::c_long = strtol(
        &raw mut pid_str as *mut ::core::ffi::c_char,
        &raw mut end_ptr,
        10 as ::core::ffi::c_int,
    );
    while *end_ptr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && *(*__ctype_b_loc())
            .offset(*end_ptr as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed_10::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        end_ptr = end_ptr.offset(1);
    }
    if end_ptr == &raw mut pid_str as *mut ::core::ffi::c_char
        || *end_ptr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || *__errno_location() == ERANGE
        || pid_val <= 0 as ::core::ffi::c_long
        || pid_val as pid_t as ::core::ffi::c_long != pid_val
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to parse PID from file %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
        );
        return finish_running_probe(-1 as ::core::ffi::c_int);
    }
    let mut pid: pid_t = pid_val as pid_t;
    if kill(pid, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Daemon is already running with PID %d\0".as_ptr() as *const ::core::ffi::c_char,
            pid,
        );
        return finish_running_probe(1 as ::core::ffi::c_int);
    } else if *__errno_location() == ESRCH {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"PID file %s is lock-held but process %d is not running; refusing to remove due to ambiguous lock ownership\0"
                .as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            pid,
        );
        return finish_running_probe(-1 as ::core::ffi::c_int);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to check process status: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return finish_running_probe(-1 as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn finish_daemon_status(mut result: ::core::ffi::c_int) -> ::core::ffi::c_int {
    pthread_mutex_unlock(&raw mut owned_pid_mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn daemon_status(
    mut pid_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut file_path: [::core::ffi::c_char; 512] = [0; 512];
    if !pid_file.is_null() {
        safe_strcpy(
            &raw mut file_path as *mut ::core::ffi::c_char,
            pid_file,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    } else {
        safe_strcpy(
            &raw mut file_path as *mut ::core::ffi::c_char,
            &raw mut pid_file_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    pthread_mutex_lock(&raw mut owned_pid_mutex);
    if owned_pid_fd >= 0 as ::core::ffi::c_int && owned_pid_process == getpid() {
        let mut held: stat = stat {
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
        let mut current: stat = stat {
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
        let mut owns_path: bool = fstat(owned_pid_fd, &raw mut held) == 0 as ::core::ffi::c_int
            && lstat(
                &raw mut file_path as *mut ::core::ffi::c_char,
                &raw mut current,
            ) == 0 as ::core::ffi::c_int
            && current.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
            && held.st_dev == current.st_dev
            && held.st_ino == current.st_ino;
        if owns_path {
            pthread_mutex_unlock(&raw mut owned_pid_mutex);
            return 1 as ::core::ffi::c_int;
        }
        pthread_mutex_unlock(&raw mut owned_pid_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut fd: ::core::ffi::c_int = open(
        &raw mut file_path as *mut ::core::ffi::c_char,
        O_RDONLY | O_NOFOLLOW | O_CLOEXEC | O_NONBLOCK,
    );
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() == ENOENT {
            return finish_daemon_status(0 as ::core::ffi::c_int);
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return finish_daemon_status(-1 as ::core::ffi::c_int);
    }
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
    if fstat(fd, &raw mut st) < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to stat PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(fd);
        return finish_daemon_status(-1 as ::core::ffi::c_int);
    }
    if !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) || st.st_nlink != 1 as __nlink_t
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"PID file %s is not a regular single-link file\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        );
        close(fd);
        return finish_daemon_status(-1 as ::core::ffi::c_int);
    }
    let mut fl: flock = flock {
        l_type: F_WRLCK as ::core::ffi::c_short,
        l_whence: SEEK_SET as ::core::ffi::c_short,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    let mut lock_ret: ::core::ffi::c_int = 0;
    loop {
        lock_ret = fcntl(fd, F_GETLK, &raw mut fl);
        if !(lock_ret < 0 as ::core::ffi::c_int && *__errno_location() == EINTR) {
            break;
        }
    }
    if lock_ret < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to query PID file lock %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(fd);
        return finish_daemon_status(-1 as ::core::ffi::c_int);
    }
    let mut locked_by_other: bool = fl.l_type as ::core::ffi::c_int != F_UNLCK;
    let mut pid_str: [::core::ffi::c_char; 16] = [0; 16];
    let mut bytes_read: ssize_t = 0;
    loop {
        bytes_read = read(
            fd,
            &raw mut pid_str as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>().wrapping_sub(1 as size_t),
        );
        if !(bytes_read < 0 as ssize_t && *__errno_location() == EINTR) {
            break;
        }
    }
    if bytes_read <= 0 as ssize_t {
        close(fd);
        if !locked_by_other && bytes_read == 0 as ssize_t {
            return finish_daemon_status(0 as ::core::ffi::c_int);
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to read PID from file %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        );
        return finish_daemon_status(-1 as ::core::ffi::c_int);
    }
    let mut i: ssize_t = 0 as ssize_t;
    while i < bytes_read {
        if pid_str[i as usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            close(fd);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"PID file %s contains embedded NUL bytes\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut file_path as *mut ::core::ffi::c_char,
            );
            return finish_daemon_status(-1 as ::core::ffi::c_int);
        }
        i += 1;
    }
    let mut extra: ::core::ffi::c_char = 0;
    let mut extra_read: ssize_t = 0;
    loop {
        extra_read = read(fd, &raw mut extra as *mut ::core::ffi::c_void, 1 as size_t);
        if !(extra_read < 0 as ssize_t && *__errno_location() == EINTR) {
            break;
        }
    }
    close(fd);
    if extra_read < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to validate PID file %s length: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return finish_daemon_status(-1 as ::core::ffi::c_int);
    }
    if extra_read > 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"PID file %s contains trailing data\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        );
        return finish_daemon_status(-1 as ::core::ffi::c_int);
    }
    pid_str[bytes_read as usize] = '\0' as ::core::ffi::c_char;
    let mut end_ptr2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut pid_val2: ::core::ffi::c_long = strtol(
        &raw mut pid_str as *mut ::core::ffi::c_char,
        &raw mut end_ptr2,
        10 as ::core::ffi::c_int,
    );
    while *end_ptr2 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
        || *end_ptr2 as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
        || *end_ptr2 as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *end_ptr2 as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        end_ptr2 = end_ptr2.offset(1);
    }
    if end_ptr2 == &raw mut pid_str as *mut ::core::ffi::c_char
        || *end_ptr2 as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || *__errno_location() == ERANGE
        || pid_val2 <= 0 as ::core::ffi::c_long
        || pid_val2 as pid_t as ::core::ffi::c_long != pid_val2
    {
        if !locked_by_other {
            return finish_daemon_status(0 as ::core::ffi::c_int);
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to parse PID from file %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        );
        return finish_daemon_status(-1 as ::core::ffi::c_int);
    }
    let mut pid: pid_t = pid_val2 as pid_t;
    if !locked_by_other {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"PID file %s is not locked; treating it as stale\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        );
        return finish_daemon_status(0 as ::core::ffi::c_int);
    }
    if fl.l_pid > 0 as ::core::ffi::c_int && pid != fl.l_pid {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"PID file %s is locked by PID %d but contains PID %d; startup may be in progress\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
            fl.l_pid,
            pid,
        );
        return finish_daemon_status(-1 as ::core::ffi::c_int);
    }
    if kill(pid, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Daemon is running with PID %d\0".as_ptr() as *const ::core::ffi::c_char,
            pid,
        );
        return finish_daemon_status(1 as ::core::ffi::c_int);
    } else if *__errno_location() == EPERM {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Daemon is running with PID %d (no permission to signal)\0".as_ptr()
                as *const ::core::ffi::c_char,
            pid,
        );
        return finish_daemon_status(1 as ::core::ffi::c_int);
    } else if *__errno_location() == ESRCH {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"PID file %s contains PID %d but process is not running\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
            pid,
        );
        return finish_daemon_status(0 as ::core::ffi::c_int);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to check process status: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return finish_daemon_status(-1 as ::core::ffi::c_int);
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SIGALRM: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
