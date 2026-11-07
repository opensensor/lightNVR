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
    unsafe fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sendto(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __addr_len: socklen_t,
    ) -> ssize_t;
    unsafe fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    static mut ONVIF_DISCOVERY_MSG: *const ::core::ffi::c_char;
    static mut ONVIF_DISCOVERY_MSG_ALT: *const ::core::ffi::c_char;
    static mut ONVIF_DISCOVERY_MSG_WITH_SCOPE: *const ::core::ffi::c_char;
    unsafe fn generate_uuid(uuid: *mut ::core::ffi::c_char, size: size_t);
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn inet_aton(
        __cp: *const ::core::ffi::c_char,
        __inp: *mut in_addr,
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = isize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type socklen_t = __socklen_t;
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
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn send_discovery_probe(
    mut ip_addr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sock: ::core::ffi::c_int = 0;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut uuid: [::core::ffi::c_char; 64] = [0; 64];
    let mut message: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut message_len: ::core::ffi::c_int = 0;
    let mut ret: ssize_t = 0;
    let mut success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    sock = socket(
        AF_INET,
        __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if sock < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create socket: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut broadcast: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if setsockopt(
        sock,
        SOL_SOCKET,
        SO_BROADCAST,
        &raw mut broadcast as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set socket options: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(sock);
        return -1 as ::core::ffi::c_int;
    }
    memset(
        &raw mut addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_in>(),
    );
    addr.sin_family = AF_INET as sa_family_t;
    addr.sin_port = __bswap_16(3702 as __uint16_t) as in_port_t;
    if inet_aton(ip_addr, &raw mut addr.sin_addr) == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid IP address: %s\0".as_ptr() as *const ::core::ffi::c_char,
            ip_addr,
        );
        close(sock);
        return -1 as ::core::ffi::c_int;
    }
    generate_uuid(
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    message_len = snprintf(
        &raw mut message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ONVIF_DISCOVERY_MSG,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    ret = sendto(
        sock,
        &raw mut message as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        message_len as size_t,
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut addr as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_in>() as socklen_t,
    );
    if ret < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to send standard discovery probe to %s:%d: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ip_addr,
            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sent standard discovery probe to %s:%d\0".as_ptr() as *const ::core::ffi::c_char,
            ip_addr,
            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
        );
        success = 1 as ::core::ffi::c_int;
    }
    usleep(50000 as __useconds_t);
    generate_uuid(
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    message_len = snprintf(
        &raw mut message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ONVIF_DISCOVERY_MSG_ALT,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    ret = sendto(
        sock,
        &raw mut message as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        message_len as size_t,
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut addr as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_in>() as socklen_t,
    );
    if ret < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to send alternative discovery probe to %s:%d: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ip_addr,
            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sent alternative discovery probe to %s:%d\0".as_ptr() as *const ::core::ffi::c_char,
            ip_addr,
            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
        );
        success = 1 as ::core::ffi::c_int;
    }
    usleep(50000 as __useconds_t);
    generate_uuid(
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    message_len = snprintf(
        &raw mut message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ONVIF_DISCOVERY_MSG_WITH_SCOPE,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    ret = sendto(
        sock,
        &raw mut message as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        message_len as size_t,
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut addr as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_in>() as socklen_t,
    );
    if ret < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to send scoped discovery probe to %s:%d: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ip_addr,
            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sent scoped discovery probe to %s:%d\0".as_ptr() as *const ::core::ffi::c_char,
            ip_addr,
            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
        );
        success = 1 as ::core::ffi::c_int;
    }
    addr.sin_port = __bswap_16(80 as __uint16_t) as in_port_t;
    generate_uuid(
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    message_len = snprintf(
        &raw mut message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ONVIF_DISCOVERY_MSG,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    ret = sendto(
        sock,
        &raw mut message as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        message_len as size_t,
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut addr as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_in>() as socklen_t,
    );
    if ret < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to send standard discovery probe to %s:%d: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ip_addr,
            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sent standard discovery probe to %s:%d\0".as_ptr() as *const ::core::ffi::c_char,
            ip_addr,
            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
        );
        success = 1 as ::core::ffi::c_int;
    }
    usleep(50000 as __useconds_t);
    generate_uuid(
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    message_len = snprintf(
        &raw mut message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ONVIF_DISCOVERY_MSG_ALT,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    ret = sendto(
        sock,
        &raw mut message as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        message_len as size_t,
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut addr as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_in>() as socklen_t,
    );
    if ret < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to send alternative discovery probe to %s:%d: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ip_addr,
            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
            strerror(*__errno_location()),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sent alternative discovery probe to %s:%d\0".as_ptr() as *const ::core::ffi::c_char,
            ip_addr,
            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
        );
        success = 1 as ::core::ffi::c_int;
    }
    close(sock);
    return if success != 0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn send_all_discovery_probes(
    mut sock: ::core::ffi::c_int,
    mut ip_addr: *const ::core::ffi::c_char,
    mut dest_addr: *mut sockaddr_in,
) -> ::core::ffi::c_int {
    let mut uuid: [::core::ffi::c_char; 64] = [0; 64];
    let mut message: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut message_len: ::core::ffi::c_int = 0;
    let mut ret: ssize_t = 0;
    generate_uuid(
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    message_len = snprintf(
        &raw mut message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ONVIF_DISCOVERY_MSG,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    ret = sendto(
        sock,
        &raw mut message as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        message_len as size_t,
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: dest_addr as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_in>() as socklen_t,
    );
    if ret < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to send standard discovery probe to %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ip_addr,
            strerror(*__errno_location()),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sent standard discovery probe to %s\0".as_ptr() as *const ::core::ffi::c_char,
            ip_addr,
        );
    }
    usleep(50000 as __useconds_t);
    generate_uuid(
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    message_len = snprintf(
        &raw mut message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ONVIF_DISCOVERY_MSG_ALT,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    ret = sendto(
        sock,
        &raw mut message as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        message_len as size_t,
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: dest_addr as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_in>() as socklen_t,
    );
    if ret < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to send alternative discovery probe to %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ip_addr,
            strerror(*__errno_location()),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sent alternative discovery probe to %s\0".as_ptr() as *const ::core::ffi::c_char,
            ip_addr,
        );
    }
    usleep(50000 as __useconds_t);
    generate_uuid(
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    message_len = snprintf(
        &raw mut message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ONVIF_DISCOVERY_MSG_WITH_SCOPE,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    ret = sendto(
        sock,
        &raw mut message as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        message_len as size_t,
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: dest_addr as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_in>() as socklen_t,
    );
    if ret < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to send scoped discovery probe to %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ip_addr,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sent scoped discovery probe to %s\0".as_ptr() as *const ::core::ffi::c_char,
            ip_addr,
        );
    }
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_BROADCAST: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
