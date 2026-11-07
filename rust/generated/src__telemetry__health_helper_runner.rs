unsafe extern "C" {
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...)
    -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn execve(
        __path: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
        __envp: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn _exit(__status: ::core::ffi::c_int) -> !;
    unsafe fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::core::ffi::c_int;
    unsafe fn fork() -> __pid_t;
    unsafe fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
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
}
pub type size_t = usize;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type clockid_t = __clockid_t;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct health_helper_outcome_t(pub ::core::ffi::c_uint);
impl health_helper_outcome_t {
    pub const HEALTH_HELPER_OK: Self = Self(0);
    pub const HEALTH_HELPER_EXITED: Self = Self(1);
    pub const HEALTH_HELPER_TIMED_OUT: Self = Self(2);
    pub const HEALTH_HELPER_BUSY: Self = Self(3);
    pub const HEALTH_HELPER_EXEC_ERROR: Self = Self(4);
    pub const HEALTH_HELPER_SYSTEM_ERROR: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct health_helper_request_t {
    pub program: *const ::core::ffi::c_char,
    pub argv: *const *mut ::core::ffi::c_char,
    pub timeout_ms: uint32_t,
    pub terminate_grace_ms: uint32_t,
    pub output_limit: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct health_helper_result_t {
    pub outcome: health_helper_outcome_t,
    pub exit_code: ::core::ffi::c_int,
    pub term_signal: ::core::ffi::c_int,
    pub latency_ms: uint32_t,
    pub output_truncated: bool,
    pub abandoned: bool,
    pub output_length: size_t,
    pub output: [::core::ffi::c_char; 4097],
}
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
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
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const HEALTH_HELPER_OUTPUT_MAX: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
pub const HEALTH_HELPER_ABANDONED_MAX: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGKILL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut runner_lock: pthread_mutex_t = pthread_mutex_t {
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
};
static mut abandoned_lock: pthread_mutex_t = pthread_mutex_t {
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
};
static mut abandoned_pids: [pid_t; 4] = [0; 4];
static mut abandoned_count: size_t = 0;
unsafe extern "C" fn monotonic_ms() -> uint64_t {
    let mut value: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_MONOTONIC, &raw mut value) != 0 as ::core::ffi::c_int {
        return 0 as uint64_t;
    }
    return (value.tv_sec as uint64_t)
        .wrapping_mul(1000 as uint64_t)
        .wrapping_add((value.tv_nsec as uint64_t).wrapping_div(1000000 as uint64_t));
}
unsafe extern "C" fn remember_abandoned(mut pid: pid_t) {
    pthread_mutex_lock(&raw mut abandoned_lock);
    if abandoned_count < HEALTH_HELPER_ABANDONED_MAX as size_t {
        let c2rust_fresh0 = abandoned_count;
        abandoned_count = abandoned_count.wrapping_add(1);
        abandoned_pids[c2rust_fresh0] = pid;
    }
    pthread_mutex_unlock(&raw mut abandoned_lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn health_helper_reap_abandoned() {
    pthread_mutex_lock(&raw mut abandoned_lock);
    let mut destination: size_t = 0 as size_t;
    let mut index: size_t = 0 as size_t;
    while index < abandoned_count {
        let mut pid: pid_t = abandoned_pids[index];
        let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut waited: pid_t = 0;
        loop {
            waited = waitpid(pid, &raw mut status, WNOHANG) as pid_t;
            if !(waited < 0 as ::core::ffi::c_int && *__errno_location() == EINTR) {
                break;
            }
        }
        if waited == 0 as ::core::ffi::c_int {
            let c2rust_fresh1 = destination;
            destination = destination.wrapping_add(1);
            abandoned_pids[c2rust_fresh1] = pid;
        }
        index = index.wrapping_add(1);
    }
    abandoned_count = destination;
    pthread_mutex_unlock(&raw mut abandoned_lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn health_helper_abandoned_count() -> uint32_t {
    health_helper_reap_abandoned();
    pthread_mutex_lock(&raw mut abandoned_lock);
    let mut count: uint32_t = abandoned_count as uint32_t;
    pthread_mutex_unlock(&raw mut abandoned_lock);
    return count;
}
unsafe extern "C" fn signal_helper(mut pid: pid_t, mut signal_number: ::core::ffi::c_int) {
    if kill(-pid, signal_number) != 0 as ::core::ffi::c_int {
        kill(pid, signal_number);
    }
}
unsafe extern "C" fn drain_output(
    mut fd: ::core::ffi::c_int,
    mut result: *mut health_helper_result_t,
    mut limit: size_t,
) {
    let mut buffer: [::core::ffi::c_char; 512] = [0; 512];
    loop {
        let mut count: ssize_t = read(
            fd,
            &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        );
        if count > 0 as ssize_t {
            let mut available: size_t = if limit > (*result).output_length {
                limit.wrapping_sub((*result).output_length)
            } else {
                0 as size_t
            };
            let mut copy: size_t = if (count as size_t) < available {
                count as size_t
            } else {
                available
            };
            if copy > 0 as size_t {
                memcpy(
                    (&raw mut (*result).output as *mut ::core::ffi::c_char)
                        .offset((*result).output_length as isize)
                        as *mut ::core::ffi::c_void,
                    &raw mut buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    copy,
                );
                (*result).output_length = (*result).output_length.wrapping_add(copy);
                (*result).output[(*result).output_length] = '\0' as ::core::ffi::c_char;
            }
            if copy < count as size_t {
                (*result).output_truncated = r#true != 0;
            }
        } else if !(count < 0 as ssize_t && *__errno_location() == EINTR) {
            break;
        }
    }
}
unsafe extern "C" fn wait_until(
    mut pid: pid_t,
    mut status: *mut ::core::ffi::c_int,
    mut deadline_ms: uint64_t,
    mut output_fd: ::core::ffi::c_int,
    mut result: *mut health_helper_result_t,
    mut output_limit: size_t,
    mut wait_error: *mut bool,
) -> bool {
    loop {
        drain_output(output_fd, result, output_limit);
        let mut waited: pid_t = waitpid(pid, status, WNOHANG);
        if waited == pid {
            drain_output(output_fd, result, output_limit);
            return r#true != 0;
        }
        if waited < 0 as ::core::ffi::c_int && *__errno_location() != EINTR {
            if !wait_error.is_null() {
                *wait_error = r#true != 0;
            }
            return r#false != 0;
        }
        let mut now: uint64_t = monotonic_ms();
        if now >= deadline_ms {
            return r#false != 0;
        }
        let mut remaining: uint64_t = deadline_ms.wrapping_sub(now);
        let mut wait_ms: ::core::ffi::c_int = if remaining > 20 as uint64_t {
            20 as ::core::ffi::c_int
        } else {
            remaining as ::core::ffi::c_int
        };
        let mut poll_fd: pollfd = pollfd {
            fd: output_fd,
            events: (POLLIN | POLLHUP) as ::core::ffi::c_short,
            revents: 0,
        };
        poll(&raw mut poll_fd, 1 as nfds_t, wait_ms);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn health_helper_run(
    mut request: *const health_helper_request_t,
    mut result: *mut health_helper_result_t,
) -> ::core::ffi::c_int {
    if request.is_null()
        || result.is_null()
        || (*request).program.is_null()
        || *(*request).program.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        || (*request).argv.is_null()
        || (*(*request).argv.offset(0isize)).is_null()
        || (*request).timeout_ms == 0 as uint32_t
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<health_helper_result_t>(),
    );
    (*result).exit_code = -1 as ::core::ffi::c_int;
    let mut output_limit: size_t = (*request).output_limit;
    if output_limit == 0 as size_t || output_limit > HEALTH_HELPER_OUTPUT_MAX as size_t {
        output_limit = HEALTH_HELPER_OUTPUT_MAX as size_t;
    }
    health_helper_reap_abandoned();
    if health_helper_abandoned_count() > 0 as uint32_t
        || pthread_mutex_trylock(&raw mut runner_lock) != 0 as ::core::ffi::c_int
    {
        (*result).outcome = health_helper_outcome_t::HEALTH_HELPER_BUSY;
        return 0 as ::core::ffi::c_int;
    }
    let mut descriptors: [::core::ffi::c_int; 2] = [0; 2];
    if pipe(&raw mut descriptors as *mut ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
        (*result).outcome = health_helper_outcome_t::HEALTH_HELPER_SYSTEM_ERROR;
        pthread_mutex_unlock(&raw mut runner_lock);
        return -1 as ::core::ffi::c_int;
    }
    fcntl(descriptors[0usize], F_SETFD, FD_CLOEXEC);
    fcntl(descriptors[1usize], F_SETFD, FD_CLOEXEC);
    let mut started: uint64_t = monotonic_ms();
    let mut pid: pid_t = fork();
    if pid == 0 as ::core::ffi::c_int {
        setpgid(0 as __pid_t, 0 as __pid_t);
        close(descriptors[0usize]);
        let mut null_fd: ::core::ffi::c_int = open(
            b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
            O_RDONLY,
        );
        if null_fd < 0 as ::core::ffi::c_int
            || dup2(null_fd, STDIN_FILENO) < 0 as ::core::ffi::c_int
        {
            _exit(126 as ::core::ffi::c_int);
        }
        if null_fd != STDIN_FILENO {
            close(null_fd);
        }
        if dup2(descriptors[1usize], STDOUT_FILENO) < 0 as ::core::ffi::c_int
            || dup2(descriptors[1usize], STDERR_FILENO) < 0 as ::core::ffi::c_int
        {
            _exit(126 as ::core::ffi::c_int);
        }
        close(descriptors[1usize]);
        let clean_environment: [*mut ::core::ffi::c_char; 4] = [
            b"PATH=/usr/sbin:/usr/bin:/sbin:/bin\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"LANG=C\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"LC_ALL=C\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        execve(
            (*request).program,
            (*request).argv,
            &raw const clean_environment as *const *mut ::core::ffi::c_char,
        );
        _exit(127 as ::core::ffi::c_int);
    }
    close(descriptors[1usize]);
    if pid < 0 as ::core::ffi::c_int {
        close(descriptors[0usize]);
        (*result).outcome = health_helper_outcome_t::HEALTH_HELPER_SYSTEM_ERROR;
        pthread_mutex_unlock(&raw mut runner_lock);
        return -1 as ::core::ffi::c_int;
    }
    setpgid(pid, pid);
    let mut flags: ::core::ffi::c_int =
        fcntl(descriptors[0usize], F_GETFL, 0 as ::core::ffi::c_int);
    if flags >= 0 as ::core::ffi::c_int {
        fcntl(descriptors[0usize], F_SETFL, flags | O_NONBLOCK);
    }
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut wait_error: bool = r#false != 0;
    let mut reaped: bool = wait_until(
        pid,
        &raw mut status,
        started.wrapping_add((*request).timeout_ms as uint64_t),
        descriptors[0usize],
        result,
        output_limit,
        &raw mut wait_error,
    );
    if wait_error {
        (*result).outcome = health_helper_outcome_t::HEALTH_HELPER_SYSTEM_ERROR;
        signal_helper(pid, SIGKILL);
        wait_error = r#false != 0;
        reaped = wait_until(
            pid,
            &raw mut status,
            monotonic_ms().wrapping_add(100 as uint64_t),
            descriptors[0usize],
            result,
            output_limit,
            &raw mut wait_error,
        );
        if !reaped {
            (*result).abandoned = r#true != 0;
            remember_abandoned(pid);
        }
    } else if !reaped {
        (*result).outcome = health_helper_outcome_t::HEALTH_HELPER_TIMED_OUT;
        signal_helper(pid, SIGTERM);
        let mut grace: uint32_t = (*request).terminate_grace_ms;
        if grace > 1000 as uint32_t {
            grace = 1000 as ::core::ffi::c_uint as uint32_t;
        }
        reaped = wait_until(
            pid,
            &raw mut status,
            monotonic_ms().wrapping_add(grace as uint64_t),
            descriptors[0usize],
            result,
            output_limit,
            &raw mut wait_error,
        );
        if !reaped {
            signal_helper(pid, SIGKILL);
            wait_error = r#false != 0;
            reaped = wait_until(
                pid,
                &raw mut status,
                monotonic_ms().wrapping_add(100 as uint64_t),
                descriptors[0usize],
                result,
                output_limit,
                &raw mut wait_error,
            );
        }
        if !reaped {
            (*result).abandoned = r#true != 0;
            remember_abandoned(pid);
        }
    } else if status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*result).exit_code = (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int;
        (*result).outcome = health_helper_outcome_t(
            (if (*result).exit_code == 0 as ::core::ffi::c_int {
                health_helper_outcome_t::HEALTH_HELPER_OK.0 as ::core::ffi::c_int
            } else {
                health_helper_outcome_t::HEALTH_HELPER_EXITED.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        if (*result).exit_code == 127 as ::core::ffi::c_int {
            (*result).outcome = health_helper_outcome_t::HEALTH_HELPER_EXEC_ERROR;
        }
    } else if ((status & 0x7f as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
        as ::core::ffi::c_schar as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        (*result).term_signal = status & 0x7f as ::core::ffi::c_int;
        (*result).outcome = health_helper_outcome_t::HEALTH_HELPER_EXITED;
    } else {
        (*result).outcome = health_helper_outcome_t::HEALTH_HELPER_SYSTEM_ERROR;
    }
    (*result).latency_ms = monotonic_ms().wrapping_sub(started) as uint32_t;
    close(descriptors[0usize]);
    pthread_mutex_unlock(&raw mut runner_lock);
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const WNOHANG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
