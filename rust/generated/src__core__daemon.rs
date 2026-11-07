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
    unsafe fn getpid() -> __pid_t;
    unsafe fn setsid() -> __pid_t;
    unsafe fn fork() -> __pid_t;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn ftruncate(__fd: ::core::ffi::c_int, __length: __off_t) -> ::core::ffi::c_int;
    unsafe fn lockf(
        __fd: ::core::ffi::c_int,
        __cmd: ::core::ffi::c_int,
        __len: __off_t,
    ) -> ::core::ffi::c_int;
    unsafe fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    unsafe fn fchmod(__fd: ::core::ffi::c_int, __mode: __mode_t) -> ::core::ffi::c_int;
    unsafe fn umask(__mask: __mode_t) -> __mode_t;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
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
pub type __uid_t = ::core::ffi::c_uint;
pub type __mode_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_TLOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SA_RESTART: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[inline]
unsafe extern "C" fn signal_safe_write(mut msg: *const ::core::ffi::c_char) {
    if msg.is_null() {
        return;
    }
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    let mut remaining: size_t = strlen(msg);
    while remaining > 0 as size_t {
        let mut written: ssize_t =
            write(STDERR_FILENO, msg as *const ::core::ffi::c_void, remaining);
        if written > 0 as ssize_t {
            msg = msg.offset(written as isize);
            remaining = remaining.wrapping_sub(written as size_t);
        } else if !(written < 0 as ssize_t && *__errno_location() == EINTR) {
            break;
        }
    }
    *__errno_location() = saved_errno;
}
static mut pid_file_path: [::core::ffi::c_char; 512] = unsafe {
    ::core::mem::transmute::<
        [u8; 512],
        [::core::ffi::c_char; 512],
    >(
        *b"/run/lightnvr.pid\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
unsafe extern "C" fn notify_startup_status(
    mut fd: ::core::ffi::c_int,
    mut status: ::core::ffi::c_uchar,
) {
    let mut n: ssize_t = 0;
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
    if check_running_daemon(&raw mut pid_file_path as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Daemon is already running\0".as_ptr() as *const ::core::ffi::c_char,
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
    umask(0 as __mode_t);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Reset file creation mask\0".as_ptr() as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Keeping all file descriptors open for web server compatibility\0".as_ptr()
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
            signal_safe_write(
                b"[DAEMON] Received shutdown signal, initiating shutdown...\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            running = r#false != 0;
            request_background_abort();
        }
        SIGHUP => {
            signal_safe_write(
                b"[DAEMON] Received SIGHUP signal\n\0".as_ptr() as *const ::core::ffi::c_char
            );
        }
        SIGALRM => {
            signal_safe_write(
                b"[DAEMON] Stray SIGALRM caught at top level (harmless, ignored)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        _ => {}
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_pid_file(
    mut pid_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if ensure_path(pid_file) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create PID file directory %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut fd: ::core::ffi::c_int = open(pid_file, O_RDWR | O_CREAT, 0o644 as ::core::ffi::c_int);
    if fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
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
        return -1 as ::core::ffi::c_int;
    }
    let mut pid_str: [::core::ffi::c_char; 16] = [0; 16];
    sprintf(
        &raw mut pid_str as *mut ::core::ffi::c_char,
        b"%d\n\0".as_ptr() as *const ::core::ffi::c_char,
        getpid(),
    );
    if write(
        fd,
        &raw mut pid_str as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(&raw mut pid_str as *mut ::core::ffi::c_char),
    ) as size_t
        != strlen(&raw mut pid_str as *mut ::core::ffi::c_char)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to write to PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
        close(fd);
        unlink(pid_file);
        return -1 as ::core::ffi::c_int;
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
    return fd;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_pid_file(
    mut fd: ::core::ffi::c_int,
    mut pid_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if unlink(pid_file) != 0 as ::core::ffi::c_int {
        if *__errno_location() == ENOENT {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"PID file %s already removed\0".as_ptr() as *const ::core::ffi::c_char,
                pid_file,
            );
            if fd >= 0 as ::core::ffi::c_int {
                close(fd);
            }
            return 0 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to remove PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
        if fd >= 0 as ::core::ffi::c_int {
            close(fd);
        }
        return -1 as ::core::ffi::c_int;
    }
    if fd >= 0 as ::core::ffi::c_int {
        close(fd);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Removed PID file %s\0".as_ptr() as *const ::core::ffi::c_char,
        pid_file,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn check_running_daemon(
    mut pid_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = open(pid_file, O_RDWR);
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() == ENOENT {
            return 0 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    loop {
        if lockf(fd, F_TLOCK, 0 as __off_t) == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Found stale PID file %s (not locked), removing it\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pid_file,
            );
            remove_pid_file(fd, pid_file);
            return 0 as ::core::ffi::c_int;
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
            return -1 as ::core::ffi::c_int;
        }
    }
    let mut pid_str: [::core::ffi::c_char; 16] = [0; 16];
    let mut bytes_read: ssize_t = read(
        fd,
        &raw mut pid_str as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>().wrapping_sub(1 as size_t),
    );
    close(fd);
    if bytes_read <= 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to read PID from file %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
        );
        return -1 as ::core::ffi::c_int;
    }
    pid_str[bytes_read as usize] = '\0' as ::core::ffi::c_char;
    let mut end_ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pid_val: ::core::ffi::c_long = strtol(
        &raw mut pid_str as *mut ::core::ffi::c_char,
        &raw mut end_ptr,
        10 as ::core::ffi::c_int,
    );
    if end_ptr == &raw mut pid_str as *mut ::core::ffi::c_char
        || pid_val <= 0 as ::core::ffi::c_long
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to parse PID from file %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid_file,
        );
        return -1 as ::core::ffi::c_int;
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
        return 1 as ::core::ffi::c_int;
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
        return -1 as ::core::ffi::c_int;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to check process status: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    };
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
    let mut fd: ::core::ffi::c_int = open(&raw mut file_path as *mut ::core::ffi::c_char, O_RDWR);
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() == ENOENT {
            return 0 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut lock_ret: ::core::ffi::c_int = 0;
    loop {
        lock_ret = lockf(fd, F_TLOCK, 0 as __off_t);
        if !(lock_ret < 0 as ::core::ffi::c_int && *__errno_location() == EINTR) {
            break;
        }
    }
    if lock_ret == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Found stale PID file %s (not locked), removing it\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        );
        remove_pid_file(fd, &raw mut file_path as *mut ::core::ffi::c_char);
        return 0 as ::core::ffi::c_int;
    }
    if *__errno_location() != EACCES && *__errno_location() != EAGAIN {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to lock PID file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(fd);
        return -1 as ::core::ffi::c_int;
    }
    let mut pid_str: [::core::ffi::c_char; 16] = [0; 16];
    let mut bytes_read: ssize_t = read(
        fd,
        &raw mut pid_str as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>().wrapping_sub(1 as size_t),
    );
    close(fd);
    if bytes_read <= 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to read PID from file %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pid_str[bytes_read as usize] = '\0' as ::core::ffi::c_char;
    let mut end_ptr2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pid_val2: ::core::ffi::c_long = strtol(
        &raw mut pid_str as *mut ::core::ffi::c_char,
        &raw mut end_ptr2,
        10 as ::core::ffi::c_int,
    );
    if end_ptr2 == &raw mut pid_str as *mut ::core::ffi::c_char
        || pid_val2 <= 0 as ::core::ffi::c_long
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to parse PID from file %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut pid: pid_t = pid_val2 as pid_t;
    if kill(pid, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Daemon is running with PID %d\0".as_ptr() as *const ::core::ffi::c_char,
            pid,
        );
        return 1 as ::core::ffi::c_int;
    } else if *__errno_location() == EPERM {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Daemon is running with PID %d (no permission to signal)\0".as_ptr()
                as *const ::core::ffi::c_char,
            pid,
        );
        return 1 as ::core::ffi::c_int;
    } else if *__errno_location() == ESRCH {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"PID file %s is locked but process %d is not running; leaving file in place\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
            pid,
        );
        return -1 as ::core::ffi::c_int;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to check process status: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
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
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ESRCH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
