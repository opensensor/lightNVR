unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn inet_ntoa(__in: in_addr) -> *mut ::core::ffi::c_char;
    unsafe fn inet_aton(
        __cp: *const ::core::ffi::c_char,
        __inp: *mut in_addr,
    ) -> ::core::ffi::c_int;
    unsafe fn gai_strerror(__ecode: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut ::core::ffi::c_char,
        __hostlen: socklen_t,
        __serv: *mut ::core::ffi::c_char,
        __servlen: socklen_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn getifaddrs(__ifap: *mut *mut ifaddrs) -> ::core::ffi::c_int;
    unsafe fn freeifaddrs(__ifa: *mut ifaddrs);
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __socklen_t = ::core::ffi::c_uint;
pub type socklen_t = __socklen_t;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
pub type uint32_t = u32;
pub type uint16_t = u16;
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
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut ::core::ffi::c_char,
    pub ifa_flags: ::core::ffi::c_uint,
    pub ifa_addr: *mut sockaddr,
    pub ifa_netmask: *mut sockaddr,
    pub ifa_ifu: C2Rust_Unnamed,
    pub ifa_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub ifu_broadaddr: *mut sockaddr,
    pub ifu_dstaddr: *mut sockaddr,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const IFF_UP: Self = Self(1);
    pub const IFF_BROADCAST: Self = Self(2);
    pub const IFF_DEBUG: Self = Self(4);
    pub const IFF_LOOPBACK: Self = Self(8);
    pub const IFF_POINTOPOINT: Self = Self(16);
    pub const IFF_NOTRAILERS: Self = Self(32);
    pub const IFF_RUNNING: Self = Self(64);
    pub const IFF_NOARP: Self = Self(128);
    pub const IFF_PROMISC: Self = Self(256);
    pub const IFF_ALLMULTI: Self = Self(512);
    pub const IFF_MASTER: Self = Self(1024);
    pub const IFF_SLAVE: Self = Self(2048);
    pub const IFF_MULTICAST: Self = Self(4096);
    pub const IFF_PORTSEL: Self = Self(8192);
    pub const IFF_AUTOMEDIA: Self = Self(16384);
    pub const IFF_DYNAMIC: Self = Self(32768);
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
pub const NI_MAXHOST: ::core::ffi::c_int = 1025 as ::core::ffi::c_int;
pub const NI_NUMERICHOST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_network(
    mut network: *const ::core::ffi::c_char,
    mut base_addr: *mut uint32_t,
    mut subnet_mask: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut network_copy: [::core::ffi::c_char; 64] = [0; 64];
    let mut slash: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut prefix_len: ::core::ffi::c_int = 0;
    let mut addr: in_addr = in_addr { s_addr: 0 };
    if network.is_null() || base_addr.is_null() || subnet_mask.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut network_copy as *mut ::core::ffi::c_char,
        network,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    slash = strchr(
        &raw mut network_copy as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if slash.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid network format: %s (expected format: x.x.x.x/y)\0".as_ptr()
                as *const ::core::ffi::c_char,
            network,
        );
        return -1 as ::core::ffi::c_int;
    }
    *slash = '\0' as ::core::ffi::c_char;
    prefix_len = strtol(
        slash.offset(1 as ::core::ffi::c_int as isize),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if prefix_len < 0 as ::core::ffi::c_int || prefix_len > 32 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid prefix length: %d (must be between 0 and 32)\0".as_ptr()
                as *const ::core::ffi::c_char,
            prefix_len,
        );
        return -1 as ::core::ffi::c_int;
    }
    if inet_aton(
        &raw mut network_copy as *mut ::core::ffi::c_char,
        &raw mut addr,
    ) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid IP address: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut network_copy as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    *base_addr = __bswap_32(addr.s_addr) as uint32_t;
    *subnet_mask = (if prefix_len == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        !(((1 as ::core::ffi::c_int) << 32 as ::core::ffi::c_int - prefix_len)
            - 1 as ::core::ffi::c_int)
    }) as uint32_t;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_local_networks(
    mut networks: *mut [::core::ffi::c_char; 64],
    mut max_networks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ifaddr: *mut ifaddrs = ::core::ptr::null_mut::<ifaddrs>();
    let mut ifa: *mut ifaddrs = ::core::ptr::null_mut::<ifaddrs>();
    let mut family: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut host: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting network interface detection\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if getifaddrs(&raw mut ifaddr) == -1 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get interface addresses: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    ifa = ifaddr;
    while !ifa.is_null() && count < max_networks {
        if !(*ifa).ifa_addr.is_null() {
            family = (*(*ifa).ifa_addr).sa_family as ::core::ffi::c_int;
            if !(*ifa).ifa_name.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Found interface: %s, family: %d\0".as_ptr() as *const ::core::ffi::c_char,
                    (*ifa).ifa_name,
                    family,
                );
            }
            if family == AF_INET {
                s = getnameinfo(
                    (*ifa).ifa_addr,
                    ::core::mem::size_of::<sockaddr_in>() as socklen_t,
                    &raw mut host as *mut ::core::ffi::c_char,
                    NI_MAXHOST as socklen_t,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    0 as socklen_t,
                    NI_NUMERICHOST,
                );
                if s != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to get IP address: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        gai_strerror(s),
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"IPv4 interface: %s, IP: %s, flags: 0x%x\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*ifa).ifa_name,
                        &raw mut host as *mut ::core::ffi::c_char,
                        (*ifa).ifa_flags,
                    );
                    if (*ifa).ifa_flags & C2Rust_Unnamed_0::IFF_LOOPBACK.0 != 0 {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Skipping loopback interface: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*ifa).ifa_name,
                        );
                    } else if (*ifa).ifa_flags & C2Rust_Unnamed_0::IFF_UP.0 == 0 {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Skipping interface that is not up: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*ifa).ifa_name,
                        );
                    } else if !(*ifa).ifa_name.is_null()
                        && (!strstr(
                            (*ifa).ifa_name,
                            b"docker\0".as_ptr() as *const ::core::ffi::c_char,
                        )
                        .is_null()
                            || !strstr(
                                (*ifa).ifa_name,
                                b"veth\0".as_ptr() as *const ::core::ffi::c_char,
                            )
                            .is_null()
                            || !strstr(
                                (*ifa).ifa_name,
                                b"br-\0".as_ptr() as *const ::core::ffi::c_char,
                            )
                            .is_null()
                            || !strstr(
                                (*ifa).ifa_name,
                                b"lxc\0".as_ptr() as *const ::core::ffi::c_char,
                            )
                            .is_null())
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Skipping Docker/virtual interface: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*ifa).ifa_name,
                        );
                    } else {
                        s = getnameinfo(
                            (*ifa).ifa_addr,
                            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
                            &raw mut host as *mut ::core::ffi::c_char,
                            NI_MAXHOST as socklen_t,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            0 as socklen_t,
                            NI_NUMERICHOST,
                        );
                        if s != 0 as ::core::ffi::c_int {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Failed to get IP address: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                gai_strerror(s),
                            );
                        } else {
                            let mut netmask: *mut sockaddr_in =
                                (*ifa).ifa_netmask as *mut sockaddr_in;
                            if !netmask.is_null() {
                                let mut mask: uint32_t = __bswap_32((*netmask).sin_addr.s_addr);
                                let mut prefix_len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while mask & 0x80000000 as uint32_t != 0 {
                                    prefix_len += 1;
                                    mask <<= 1 as ::core::ffi::c_int;
                                }
                                let mut addr: *mut sockaddr_in =
                                    (*ifa).ifa_addr as *mut sockaddr_in;
                                let mut ip: uint32_t = __bswap_32((*addr).sin_addr.s_addr);
                                let mut network: uint32_t =
                                    ip & __bswap_32((*netmask).sin_addr.s_addr);
                                let mut network_addr: in_addr = in_addr { s_addr: 0 };
                                network_addr.s_addr = __bswap_32(network) as in_addr_t;
                                snprintf(
                                    &raw mut *networks.offset(count as isize)
                                        as *mut ::core::ffi::c_char,
                                    64 as size_t,
                                    b"%s/%d\0".as_ptr() as *const ::core::ffi::c_char,
                                    inet_ntoa(network_addr),
                                    prefix_len,
                                );
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Detected network: %s (interface: %s)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut *networks.offset(count as isize)
                                        as *mut ::core::ffi::c_char,
                                    (*ifa).ifa_name,
                                );
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        ifa = (*ifa).ifa_next;
    }
    freeifaddrs(ifaddr);
    if count == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"No suitable network interfaces found\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return count;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
}
