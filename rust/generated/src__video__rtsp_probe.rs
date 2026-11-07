unsafe extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    unsafe fn url_rtsp_endpoint(
        url: *const ::core::ffi::c_char,
        host: *mut ::core::ffi::c_char,
        host_size: size_t,
        port: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn connect(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn send(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    unsafe fn recv(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    unsafe fn getsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *mut ::core::ffi::c_void,
        __optlen: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn getaddrinfo(
        __name: *const ::core::ffi::c_char,
        __service: *const ::core::ffi::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
    unsafe fn freeaddrinfo(__ai: *mut addrinfo);
    unsafe fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
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
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: ::core::ffi::c_int,
    pub ai_family: ::core::ffi::c_int,
    pub ai_socktype: ::core::ffi::c_int,
    pub ai_protocol: ::core::ffi::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut ::core::ffi::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
pub type sa_family_t = ::core::ffi::c_ushort;
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = u16;
pub type uint8_t = u8;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct __socket_type(pub ::core::ffi::c_uint);
impl __socket_type {
    pub const SOCK_STREAM: Self = Self(1);
    pub const SOCK_DGRAM: Self = Self(2);
    pub const SOCK_RAW: Self = Self(3);
    pub const SOCK_RDM: Self = Self(4);
    pub const SOCK_SEQPACKET: Self = Self(5);
    pub const SOCK_DCCP: Self = Self(6);
    pub const SOCK_PACKET: Self = Self(10);
    pub const SOCK_CLOEXEC: Self = Self(524288);
    pub const SOCK_NONBLOCK: Self = Self(2048);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const MSG_OOB: Self = Self(1);
    pub const MSG_PEEK: Self = Self(2);
    pub const MSG_DONTROUTE: Self = Self(4);
    pub const MSG_TRYHARD: Self = Self(4);
    pub const MSG_CTRUNC: Self = Self(8);
    pub const MSG_PROXY: Self = Self(16);
    pub const MSG_TRUNC: Self = Self(32);
    pub const MSG_DONTWAIT: Self = Self(64);
    pub const MSG_EOR: Self = Self(128);
    pub const MSG_WAITALL: Self = Self(256);
    pub const MSG_FIN: Self = Self(512);
    pub const MSG_SYN: Self = Self(1024);
    pub const MSG_CONFIRM: Self = Self(2048);
    pub const MSG_RST: Self = Self(4096);
    pub const MSG_ERRQUEUE: Self = Self(8192);
    pub const MSG_NOSIGNAL: Self = Self(16384);
    pub const MSG_MORE: Self = Self(32768);
    pub const MSG_WAITFORONE: Self = Self(65536);
    pub const MSG_BATCH: Self = Self(262144);
    pub const MSG_SOCK_DEVMEM: Self = Self(33554432);
    pub const MSG_ZEROCOPY: Self = Self(67108864);
    pub const MSG_FASTOPEN: Self = Self(536870912);
    pub const MSG_CMSG_CLOEXEC: Self = Self(1073741824);
}
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_ERROR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const AI_NUMERICSERV: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn monotonic_ms() -> ::core::ffi::c_longlong {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_MONOTONIC, &raw mut now) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_longlong;
    }
    return now.tv_sec as ::core::ffi::c_longlong * 1000 as ::core::ffi::c_longlong
        + (now.tv_nsec / 1000000 as __syscall_slong_t) as ::core::ffi::c_longlong;
}
unsafe extern "C" fn ready(
    mut fd: ::core::ffi::c_int,
    mut events: ::core::ffi::c_short,
    mut deadline: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    loop {
        let mut now: ::core::ffi::c_longlong = monotonic_ms();
        if now < 0 as ::core::ffi::c_longlong || now >= deadline {
            return -1 as ::core::ffi::c_int;
        }
        let mut descriptor: pollfd = pollfd {
            fd: fd,
            events: events,
            revents: 0,
        };
        let mut rc: ::core::ffi::c_int = poll(
            &raw mut descriptor,
            1 as nfds_t,
            (deadline - now) as ::core::ffi::c_int,
        );
        if rc < 0 as ::core::ffi::c_int && *__errno_location() == EINTR {
            continue;
        }
        return if rc > 0 as ::core::ffi::c_int
            && descriptor.revents as ::core::ffi::c_int & events as ::core::ffi::c_int != 0
        {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rtsp_probe_server(
    mut url: *const ::core::ffi::c_char,
    mut host: *mut ::core::ffi::c_char,
    mut host_size: size_t,
    mut port: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut response: [::core::ffi::c_char; 256] = [0; 256];
    let mut used: size_t = 0;
    if url_rtsp_endpoint(url, host, host_size, port) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut service: [::core::ffi::c_char; 6] = [0; 6];
    snprintf(
        &raw mut service as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 6]>(),
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        *port,
    );
    let mut hints: addrinfo = addrinfo {
        ai_flags: AI_NUMERICSERV,
        ai_family: AF_UNSPEC,
        ai_socktype: __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: ::core::ptr::null_mut::<sockaddr>(),
        ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ai_next: ::core::ptr::null_mut::<addrinfo>(),
    };
    let mut addresses: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    if getaddrinfo(
        host,
        &raw mut service as *mut ::core::ffi::c_char,
        &raw mut hints,
        &raw mut addresses,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut started: ::core::ffi::c_longlong = monotonic_ms();
    if started < 0 as ::core::ffi::c_longlong {
        freeaddrinfo(addresses);
        return -1 as ::core::ffi::c_int;
    }
    let mut deadline: ::core::ffi::c_longlong = started + 2000 as ::core::ffi::c_longlong;
    let mut fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut a: *mut addrinfo = addresses;
    while !a.is_null() {
        fd = socket(
            (*a).ai_family,
            __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int
                | __socket_type::SOCK_CLOEXEC.0 as ::core::ffi::c_int
                | __socket_type::SOCK_NONBLOCK.0 as ::core::ffi::c_int,
            (*a).ai_protocol,
        );
        if fd >= 0 as ::core::ffi::c_int {
            let mut rc: ::core::ffi::c_int = connect(
                fd,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: (*a).ai_addr,
                },
                (*a).ai_addrlen,
            );
            if rc == 0 as ::core::ffi::c_int {
                break;
            }
            if *__errno_location() == EINPROGRESS
                && ready(fd, POLLOUT as ::core::ffi::c_short, deadline) == 0 as ::core::ffi::c_int
            {
                let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut size: socklen_t = ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t;
                if getsockopt(
                    fd,
                    SOL_SOCKET,
                    SO_ERROR,
                    &raw mut error as *mut ::core::ffi::c_void,
                    &raw mut size,
                ) == 0 as ::core::ffi::c_int
                    && error == 0
                {
                    break;
                }
            }
            close(fd);
            fd = -1 as ::core::ffi::c_int;
        }
        a = (*a).ai_next;
    }
    freeaddrinfo(addresses);
    if fd < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    static mut request: [::core::ffi::c_char; 54] = unsafe {
        ::core::mem::transmute::<[u8; 54], [::core::ffi::c_char; 54]>(
            *b"OPTIONS * RTSP/1.0\r\nCSeq: 1\r\nUser-Agent: LightNVR\r\n\r\n\0",
        )
    };
    let mut sent: size_t = 0 as size_t;
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    '_done: {
        while sent < ::core::mem::size_of::<[::core::ffi::c_char; 54]>().wrapping_sub(1usize) {
            if ready(fd, POLLOUT as ::core::ffi::c_short, deadline) != 0 as ::core::ffi::c_int {
                break '_done;
            }
            let mut n: ssize_t = send(
                fd,
                (&raw const request as *const ::core::ffi::c_char).offset(sent as isize)
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 54]>()
                    .wrapping_sub(1 as size_t)
                    .wrapping_sub(sent),
                C2Rust_Unnamed_0::MSG_NOSIGNAL.0 as ::core::ffi::c_int,
            );
            if n < 0 as ssize_t
                && (*__errno_location() == EINTR
                    || *__errno_location() == EAGAIN
                    || *__errno_location() == EWOULDBLOCK)
            {
                continue;
            }
            if n <= 0 as ssize_t {
                break '_done;
            }
            sent = sent.wrapping_add(n as size_t);
        }
        response = [0; 256];
        used = 0 as size_t;
        loop {
            if used >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(1usize) {
                break '_done;
            }
            if ready(fd, POLLIN as ::core::ffi::c_short, deadline) != 0 as ::core::ffi::c_int {
                break '_done;
            }
            let mut n_0: ssize_t = recv(
                fd,
                (&raw mut response as *mut ::core::ffi::c_char).offset(used as isize)
                    as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
                    .wrapping_sub(1 as size_t)
                    .wrapping_sub(used),
                0 as ::core::ffi::c_int,
            );
            if n_0 < 0 as ssize_t
                && (*__errno_location() == EINTR
                    || *__errno_location() == EAGAIN
                    || *__errno_location() == EWOULDBLOCK)
            {
                continue;
            }
            if n_0 <= 0 as ssize_t {
                break '_done;
            }
            used = used.wrapping_add(n_0 as size_t);
            response[used] = '\0' as ::core::ffi::c_char;
            if memchr(
                &raw mut response as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                '\n' as ::core::ffi::c_int,
                used,
            )
            .is_null()
            {
                continue;
            }
            if used >= 13 as size_t
                && memcmp(
                    &raw mut response as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    b"RTSP/1.0 \0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    9 as size_t,
                ) == 0
                && response[9usize] as ::core::ffi::c_int >= '1' as ::core::ffi::c_int
                && response[9usize] as ::core::ffi::c_int <= '5' as ::core::ffi::c_int
                && response[10usize] as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && response[10usize] as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                && response[11usize] as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && response[11usize] as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                && response[12usize] as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            {
                result = 0 as ::core::ffi::c_int;
            }
            break '_done;
        }
    }
    close(fd);
    return result;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EINPROGRESS: ::core::ffi::c_int = 115 as ::core::ffi::c_int;
