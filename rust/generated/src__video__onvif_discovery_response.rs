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
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn bind(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn sendto(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __addr_len: socklen_t,
    ) -> ssize_t;
    unsafe fn recvfrom(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    unsafe fn getsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *mut ::core::ffi::c_void,
        __optlen: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn inet_addr(__cp: *const ::core::ffi::c_char) -> in_addr_t;
    unsafe fn inet_ntop(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_void,
        __buf: *mut ::core::ffi::c_char,
        __len: socklen_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
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
    unsafe fn copy_trimmed_value(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
        input: *const ::core::ffi::c_char,
        input_size: size_t,
    ) -> size_t;
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
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn select(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = isize;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
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
    pub __in6_u: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const IPPROTO_IP: Self = Self(0);
    pub const IPPROTO_ICMP: Self = Self(1);
    pub const IPPROTO_IGMP: Self = Self(2);
    pub const IPPROTO_IPIP: Self = Self(4);
    pub const IPPROTO_TCP: Self = Self(6);
    pub const IPPROTO_EGP: Self = Self(8);
    pub const IPPROTO_PUP: Self = Self(12);
    pub const IPPROTO_UDP: Self = Self(17);
    pub const IPPROTO_IDP: Self = Self(22);
    pub const IPPROTO_TP: Self = Self(29);
    pub const IPPROTO_DCCP: Self = Self(33);
    pub const IPPROTO_IPV6: Self = Self(41);
    pub const IPPROTO_RSVP: Self = Self(46);
    pub const IPPROTO_GRE: Self = Self(47);
    pub const IPPROTO_ESP: Self = Self(50);
    pub const IPPROTO_AH: Self = Self(51);
    pub const IPPROTO_MTP: Self = Self(92);
    pub const IPPROTO_BEETPH: Self = Self(94);
    pub const IPPROTO_ENCAP: Self = Self(98);
    pub const IPPROTO_PIM: Self = Self(103);
    pub const IPPROTO_COMP: Self = Self(108);
    pub const IPPROTO_L2TP: Self = Self(115);
    pub const IPPROTO_SCTP: Self = Self(132);
    pub const IPPROTO_UDPLITE: Self = Self(136);
    pub const IPPROTO_MPLS: Self = Self(137);
    pub const IPPROTO_ETHERNET: Self = Self(143);
    pub const IPPROTO_RAW: Self = Self(255);
    pub const IPPROTO_SMC: Self = Self(256);
    pub const IPPROTO_MPTCP: Self = Self(262);
    pub const IPPROTO_MAX: Self = Self(263);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_device_info_t {
    pub inventory_uuid: [::core::ffi::c_char; 37],
    pub endpoint: [::core::ffi::c_char; 512],
    pub device_service: [::core::ffi::c_char; 512],
    pub media_service: [::core::ffi::c_char; 512],
    pub ptz_service: [::core::ffi::c_char; 512],
    pub imaging_service: [::core::ffi::c_char; 512],
    pub manufacturer: [::core::ffi::c_char; 64],
    pub model: [::core::ffi::c_char; 64],
    pub firmware_version: [::core::ffi::c_char; 32],
    pub serial_number: [::core::ffi::c_char; 64],
    pub hardware_id: [::core::ffi::c_char; 64],
    pub ip_address: [::core::ffi::c_char; 64],
    pub mac_address: [::core::ffi::c_char; 32],
    pub discovery_time: time_t,
    pub first_seen_at: time_t,
    pub last_seen_at: time_t,
    pub last_scan_network: [::core::ffi::c_char; 64],
    pub claim_state: [::core::ffi::c_char; 16],
    pub claimed_camera_uuid: [::core::ffi::c_char; 37],
    pub duplicate_suspected: bool,
    pub online: bool,
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
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const IP_MULTICAST_TTL: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const IP_MULTICAST_LOOP: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const IP_ADD_MEMBERSHIP: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const INET_ADDRSTRLEN: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAX_URL_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn extract_xml_content(
    mut xml: *const ::core::ffi::c_char,
    mut tag_start: *const ::core::ffi::c_char,
    mut tag_end: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) -> *mut ::core::ffi::c_char {
    let mut start: *const ::core::ffi::c_char = strstr(xml, tag_start);
    if start.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    start = start.offset(strlen(tag_start) as isize);
    let mut end: *const ::core::ffi::c_char = strstr(start, tag_end);
    if end.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut len: size_t = end.offset_from(start) as size_t;
    copy_trimmed_value(buffer, buffer_size, start, len);
    return buffer;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_device_info(
    mut response: *const ::core::ffi::c_char,
    mut device_info: *mut onvif_device_info_t,
) -> ::core::ffi::c_int {
    memset(
        device_info as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<onvif_device_info_t>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Parsing ONVIF discovery response (%zu bytes)\0".as_ptr() as *const ::core::ffi::c_char,
        strlen(response),
    );
    if !strstr(response, b"Probe\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        && strstr(
            response,
            b"ProbeMatch\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Ignoring probe message (not a device response)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if strstr(
        response,
        b"NetworkVideoTransmitter\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .is_null()
        && strstr(response, b"Device\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        && strstr(response, b"ONVIF\0".as_ptr() as *const ::core::ffi::c_char).is_null()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Not an ONVIF response (missing required keywords)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut xaddrs: [::core::ffi::c_char; 512] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    if extract_xml_content(
        response,
        b"<d:XAddrs>\0".as_ptr() as *const ::core::ffi::c_char,
        b"</d:XAddrs>\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut xaddrs as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    )
    .is_null()
    {
        if extract_xml_content(
            response,
            b"<XAddrs>\0".as_ptr() as *const ::core::ffi::c_char,
            b"</XAddrs>\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut xaddrs as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        )
        .is_null()
        {
            let mut xaddr_tag: *const ::core::ffi::c_char = strstr(
                response,
                b"XAddrs>\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !xaddr_tag.is_null() {
                let mut start: *const ::core::ffi::c_char =
                    strchr(xaddr_tag, '>' as ::core::ffi::c_int);
                if !start.is_null() {
                    start = start.offset(1);
                    let mut end: *const ::core::ffi::c_char =
                        strstr(start, b"</\0".as_ptr() as *const ::core::ffi::c_char);
                    if !end.is_null() {
                        let mut len: size_t = end.offset_from(start) as size_t;
                        copy_trimmed_value(
                            &raw mut xaddrs as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            start,
                            len,
                        );
                    }
                }
            }
        }
    }
    if strlen(&raw mut xaddrs as *mut ::core::ffi::c_char) == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to find XAddrs in response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Found XAddrs: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut xaddrs as *mut ::core::ffi::c_char,
    );
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut url: *const ::core::ffi::c_char = strtok_r(
        &raw mut xaddrs as *mut ::core::ffi::c_char,
        b" \t\n\r\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    if !url.is_null() {
        safe_strcpy(
            &raw mut (*device_info).device_service as *mut ::core::ffi::c_char,
            url,
            MAX_URL_LENGTH as size_t,
            0 as size_t,
        );
        let mut endpoint_reference: *const ::core::ffi::c_char = strstr(
            response,
            b"EndpointReference\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut address_tag: *const ::core::ffi::c_char = if !endpoint_reference.is_null() {
            strstr(
                endpoint_reference,
                b"Address>\0".as_ptr() as *const ::core::ffi::c_char,
            )
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        let mut endpoint_reference_address: [::core::ffi::c_char; 512] = [
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
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        if !address_tag.is_null() {
            let mut address_start: *const ::core::ffi::c_char =
                strchr(address_tag, '>' as ::core::ffi::c_int);
            if !address_start.is_null() {
                address_start = address_start.offset(1);
                let mut address_end: *const ::core::ffi::c_char = strstr(
                    address_start,
                    b"</\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if !address_end.is_null() {
                    copy_trimmed_value(
                        &raw mut endpoint_reference_address as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                        address_start,
                        address_end.offset_from(address_start) as size_t,
                    );
                }
            }
        }
        safe_strcpy(
            &raw mut (*device_info).endpoint as *mut ::core::ffi::c_char,
            if endpoint_reference_address[0usize] as ::core::ffi::c_int != 0 {
                &raw mut endpoint_reference_address as *mut ::core::ffi::c_char
            } else {
                url
            },
            MAX_URL_LENGTH as size_t,
            0 as size_t,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Found device service URL: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*device_info).device_service as *mut ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No valid URL found in XAddrs\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut http: *const ::core::ffi::c_char = strstr(
        &raw mut (*device_info).device_service as *mut ::core::ffi::c_char,
        b"http://\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !http.is_null() {
        let mut ip_start: *const ::core::ffi::c_char =
            http.offset(7 as ::core::ffi::c_int as isize);
        let mut ip_end: *const ::core::ffi::c_char = strchr(ip_start, ':' as ::core::ffi::c_int);
        if ip_end.is_null() {
            ip_end = strchr(ip_start, '/' as ::core::ffi::c_int);
        }
        if !ip_end.is_null() {
            let mut len_0: size_t = ip_end.offset_from(ip_start) as size_t;
            if len_0 >= ::core::mem::size_of::<[::core::ffi::c_char; 64]>() {
                len_0 = ::core::mem::size_of::<[::core::ffi::c_char; 64]>().wrapping_sub(1usize)
                    as size_t;
            }
            safe_strcpy(
                &raw mut (*device_info).ip_address as *mut ::core::ffi::c_char,
                ip_start,
                64 as size_t,
                len_0,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Extracted IP address: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*device_info).ip_address as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut types: [::core::ffi::c_char; 128] = [
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
    if extract_xml_content(
        response,
        b"<d:Types>\0".as_ptr() as *const ::core::ffi::c_char,
        b"</d:Types>\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut types as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    )
    .is_null()
    {
        extract_xml_content(
            response,
            b"<Types>\0".as_ptr() as *const ::core::ffi::c_char,
            b"</Types>\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut types as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        );
    }
    if strlen(&raw mut types as *mut ::core::ffi::c_char) > 0 as size_t {
        if !strstr(
            &raw mut types as *mut ::core::ffi::c_char,
            b"NetworkVideoTransmitter\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            safe_strcpy(
                &raw mut (*device_info).model as *mut ::core::ffi::c_char,
                b"NetworkVideoTransmitter\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
    }
    (*device_info).discovery_time = time(::core::ptr::null_mut::<time_t>());
    (*device_info).online = r#true != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Successfully parsed device info: %s (%s)\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*device_info).device_service as *mut ::core::ffi::c_char,
        &raw mut (*device_info).ip_address as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn receive_discovery_responses(
    mut devices: *mut onvif_device_info_t,
    mut max_devices: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sock: ::core::ffi::c_int = 0;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut addr_len: socklen_t = ::core::mem::size_of::<sockaddr_in>() as socklen_t;
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buffer_size: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
    let mut ret: ssize_t = 0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    buffer = malloc(buffer_size as size_t) as *mut ::core::ffi::c_char;
    if buffer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for receive buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut readfds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut timeout: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Setting up socket to receive discovery responses\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
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
        free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut reuse: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if setsockopt(
        sock,
        SOL_SOCKET,
        SO_REUSEADDR,
        &raw mut reuse as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set socket options (SO_REUSEADDR): %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(sock);
        free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    if setsockopt(
        sock,
        SOL_SOCKET,
        SO_REUSEPORT,
        &raw mut reuse as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set SO_REUSEPORT option: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
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
            b"Failed to set socket options (SO_BROADCAST): %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(sock);
        free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut rcvbuf: ::core::ffi::c_int = 256 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
    if setsockopt(
        sock,
        SOL_SOCKET,
        SO_RCVBUF,
        &raw mut rcvbuf as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to increase receive buffer size: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    let mut actual_rcvbuf: ::core::ffi::c_int = 0;
    let mut optlen: socklen_t = ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t;
    if getsockopt(
        sock,
        SOL_SOCKET,
        SO_RCVBUF,
        &raw mut actual_rcvbuf as *mut ::core::ffi::c_void,
        &raw mut optlen,
    ) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Socket receive buffer size: %d bytes\0".as_ptr() as *const ::core::ffi::c_char,
            actual_rcvbuf,
        );
    }
    let mut mreq_buf: [::core::ffi::c_char; 8] = [0; 8];
    let mut imr_multiaddr: *mut in_addr =
        &raw mut mreq_buf as *mut ::core::ffi::c_char as *mut in_addr;
    let mut imr_interface: *mut in_addr = (&raw mut mreq_buf as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<in_addr>() as isize)
        as *mut in_addr;
    (*imr_multiaddr).s_addr =
        inet_addr(b"239.255.255.250\0".as_ptr() as *const ::core::ffi::c_char);
    (*imr_interface).s_addr = __bswap_32(0 as ::core::ffi::c_int as __uint32_t) as in_addr_t;
    if setsockopt(
        sock,
        C2Rust_Unnamed_1::IPPROTO_IP.0 as ::core::ffi::c_int,
        IP_ADD_MEMBERSHIP,
        &raw mut mreq_buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to join multicast group: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    let mut ttl: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    if setsockopt(
        sock,
        C2Rust_Unnamed_1::IPPROTO_IP.0 as ::core::ffi::c_int,
        IP_MULTICAST_TTL,
        &raw mut ttl as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set multicast TTL: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    let mut loopback: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if setsockopt(
        sock,
        C2Rust_Unnamed_1::IPPROTO_IP.0 as ::core::ffi::c_int,
        IP_MULTICAST_LOOP,
        &raw mut loopback as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to enable multicast loopback: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    memset(
        &raw mut addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_in>(),
    );
    addr.sin_family = AF_INET as sa_family_t;
    addr.sin_port = __bswap_16(3702 as __uint16_t) as in_port_t;
    addr.sin_addr.s_addr = __bswap_32(0 as ::core::ffi::c_int as __uint32_t) as in_addr_t;
    let mut bind_attempts: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_bind_attempts: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut bind_result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut original_port: ::core::ffi::c_int = __bswap_16(addr.sin_port) as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Waiting 2 seconds before binding to allow TIME_WAIT sockets to clear\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    sleep(2 as ::core::ffi::c_uint);
    while bind_attempts < max_bind_attempts {
        bind_result = bind(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut addr as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        );
        if bind_result == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Successfully bound to port %d\0".as_ptr() as *const ::core::ffi::c_char,
                __bswap_16(addr.sin_port) as ::core::ffi::c_int,
            );
            break;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to bind to port %d: %s (attempt %d/%d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                __bswap_16(addr.sin_port) as ::core::ffi::c_int,
                strerror(*__errno_location()),
                bind_attempts + 1 as ::core::ffi::c_int,
                max_bind_attempts,
            );
            if *__errno_location() != EADDRINUSE {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Trying a different port...\0".as_ptr() as *const ::core::ffi::c_char,
                );
                addr.sin_port = __bswap_16(
                    (original_port + bind_attempts + 1 as ::core::ffi::c_int) as __uint16_t,
                ) as in_port_t;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Address in use, waiting 1 second before retry\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                sleep(1 as ::core::ffi::c_uint);
                if bind_attempts >= 2 as ::core::ffi::c_int {
                    addr.sin_port = __bswap_16(
                        (original_port + 10000 as ::core::ffi::c_int + bind_attempts) as __uint16_t,
                    ) as in_port_t;
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Trying alternative port %d\0".as_ptr() as *const ::core::ffi::c_char,
                        __bswap_16(addr.sin_port) as ::core::ffi::c_int,
                    );
                }
            }
            bind_attempts += 1;
        }
    }
    if bind_result < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to bind socket after %d attempts: %s\0".as_ptr() as *const ::core::ffi::c_char,
            max_bind_attempts,
            strerror(*__errno_location()),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Continuing with discovery process, but may not receive responses\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    usleep(100000 as ::core::ffi::c_int as __useconds_t);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Waiting for discovery responses (timeout: 10 seconds, attempts: 5)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    timeout.tv_sec = 10 as __time_t;
    timeout.tv_usec = 0 as __suseconds_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 5 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Waiting for responses, attempt %d/5\0".as_ptr() as *const ::core::ffi::c_char,
            i + 1 as ::core::ffi::c_int,
        );
        let mut __i: ::core::ffi::c_uint = 0;
        let mut __arr: *mut fd_set = &raw mut readfds;
        __i = 0 as ::core::ffi::c_uint;
        while (__i as usize)
            < ::core::mem::size_of::<fd_set>().wrapping_div(::core::mem::size_of::<__fd_mask>())
        {
            (*__arr).fds_bits[__i as usize] = 0 as __fd_mask;
            __i = __i.wrapping_add(1);
        }
        readfds.fds_bits[(sock / __NFDBITS) as usize] |=
            ((1 as ::core::ffi::c_ulong) << sock % __NFDBITS) as __fd_mask;
        ret = select(
            sock + 1 as ::core::ffi::c_int,
            &raw mut readfds,
            ::core::ptr::null_mut::<fd_set>(),
            ::core::ptr::null_mut::<fd_set>(),
            &raw mut timeout,
        ) as ssize_t;
        if ret < 0 as ssize_t {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Select failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            close(sock);
            free(buffer as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        } else {
            if ret == 0 as ssize_t {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Timeout waiting for responses, no data available\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Sending additional discovery probe to broadcast address\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                let mut broadcast_addr: sockaddr_in = sockaddr_in {
                    sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr { s_addr: 0 },
                    sin_zero: [0; 8],
                };
                memset(
                    &raw mut broadcast_addr as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<sockaddr_in>(),
                );
                broadcast_addr.sin_family = AF_INET as sa_family_t;
                broadcast_addr.sin_port = __bswap_16(3702 as __uint16_t) as in_port_t;
                broadcast_addr.sin_addr.s_addr =
                    inet_addr(b"255.255.255.255\0".as_ptr() as *const ::core::ffi::c_char);
                let mut uuid: [::core::ffi::c_char; 64] = [0; 64];
                let mut message: [::core::ffi::c_char; 1024] = [0; 1024];
                unsafe extern "C" {
                    static mut ONVIF_DISCOVERY_MSG: *const ::core::ffi::c_char;
                }
                unsafe extern "C" {
                    static mut ONVIF_DISCOVERY_MSG_ALT: *const ::core::ffi::c_char;
                }
                unsafe extern "C" {
                    #[link_name = "generate_uuid"]
                    unsafe fn generate_uuid_0(uuid_0: *mut ::core::ffi::c_char, size: size_t);
                }
                generate_uuid_0(
                    &raw mut uuid as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                );
                let mut message_len: ::core::ffi::c_int = snprintf(
                    &raw mut message as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                    ONVIF_DISCOVERY_MSG,
                    &raw mut uuid as *mut ::core::ffi::c_char,
                );
                sendto(
                    sock,
                    &raw mut message as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    message_len as size_t,
                    0 as ::core::ffi::c_int,
                    __CONST_SOCKADDR_ARG {
                        __sockaddr__: &raw mut broadcast_addr as *mut sockaddr,
                    },
                    ::core::mem::size_of::<sockaddr_in>() as socklen_t,
                );
                usleep(50000 as __useconds_t);
                generate_uuid_0(
                    &raw mut uuid as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                );
                message_len = snprintf(
                    &raw mut message as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                    ONVIF_DISCOVERY_MSG_ALT,
                    &raw mut uuid as *mut ::core::ffi::c_char,
                );
                sendto(
                    sock,
                    &raw mut message as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    message_len as size_t,
                    0 as ::core::ffi::c_int,
                    __CONST_SOCKADDR_ARG {
                        __sockaddr__: &raw mut broadcast_addr as *mut sockaddr,
                    },
                    ::core::mem::size_of::<sockaddr_in>() as socklen_t,
                );
            } else {
                loop {
                    ret = recvfrom(
                        sock,
                        buffer as *mut ::core::ffi::c_void,
                        (buffer_size - 1 as ::core::ffi::c_int) as size_t,
                        C2Rust_Unnamed::MSG_DONTWAIT.0 as ::core::ffi::c_int,
                        __SOCKADDR_ARG {
                            __sockaddr__: &raw mut addr as *mut sockaddr,
                        },
                        &raw mut addr_len,
                    );
                    if ret < 0 as ssize_t {
                        if *__errno_location() == EAGAIN || *__errno_location() == EWOULDBLOCK {
                            break;
                        }
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to receive data: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            strerror(*__errno_location()),
                        );
                        break;
                    } else {
                        let mut ip_str: [::core::ffi::c_char; 16] = [0; 16];
                        inet_ntop(
                            AF_INET,
                            &raw mut addr.sin_addr as *const ::core::ffi::c_void,
                            &raw mut ip_str as *mut ::core::ffi::c_char,
                            INET_ADDRSTRLEN as socklen_t,
                        );
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Received %d bytes from %s:%d\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            ret as ::core::ffi::c_int,
                            &raw mut ip_str as *mut ::core::ffi::c_char,
                            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
                        );
                        *buffer.offset(ret) = '\0' as ::core::ffi::c_char;
                        let mut debug_buffer: [::core::ffi::c_char; 501] = [0; 501];
                        safe_strcpy(
                            &raw mut debug_buffer as *mut ::core::ffi::c_char,
                            buffer,
                            501 as size_t,
                            0 as size_t,
                        );
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Response (first 500 chars): %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut debug_buffer as *mut ::core::ffi::c_char,
                        );
                        if count < max_devices {
                            if parse_device_info(buffer, devices.offset(count as isize))
                                == 0 as ::core::ffi::c_int
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Discovered ONVIF device: %s (%s)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut (*devices.offset(count as isize)).device_service
                                        as *mut ::core::ffi::c_char,
                                    &raw mut (*devices.offset(count as isize)).ip_address
                                        as *mut ::core::ffi::c_char,
                                );
                                let mut duplicate: bool = r#false != 0;
                                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while j < count {
                                    if strcmp(
                                        &raw mut (*devices.offset(j as isize)).ip_address
                                            as *mut ::core::ffi::c_char,
                                        &raw mut (*devices.offset(count as isize)).ip_address
                                            as *mut ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        duplicate = r#true != 0;
                                        break;
                                    } else {
                                        j += 1;
                                    }
                                }
                                if !duplicate {
                                    count += 1;
                                } else {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_DEBUG,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Skipping duplicate device: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        &raw mut (*devices.offset(count as isize)).ip_address
                                            as *mut ::core::ffi::c_char,
                                    );
                                }
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_DEBUG,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Failed to parse device info from response\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        }
                        if count >= max_devices {
                            break;
                        }
                    }
                }
                timeout.tv_sec = 2 as __time_t;
                timeout.tv_usec = 0 as __suseconds_t;
            }
            i += 1;
        }
    }
    close(sock);
    free(buffer as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Discovery response collection completed, found %d devices\0".as_ptr()
            as *const ::core::ffi::c_char,
        count,
    );
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn receive_extended_discovery_responses(
    mut devices: *mut onvif_device_info_t,
    mut max_devices: ::core::ffi::c_int,
    mut timeout_sec: ::core::ffi::c_int,
    mut max_attempts: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sock: ::core::ffi::c_int = 0;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut addr_len: socklen_t = ::core::mem::size_of::<sockaddr_in>() as socklen_t;
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buffer_size: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
    let mut ret: ssize_t = 0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    buffer = malloc(buffer_size as size_t) as *mut ::core::ffi::c_char;
    if buffer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for receive buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut readfds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut timeout: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Setting up socket to receive discovery responses (extended timeouts)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
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
        free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut reuse: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if setsockopt(
        sock,
        SOL_SOCKET,
        SO_REUSEADDR,
        &raw mut reuse as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set socket options (SO_REUSEADDR): %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(sock);
        free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    if setsockopt(
        sock,
        SOL_SOCKET,
        SO_REUSEPORT,
        &raw mut reuse as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set SO_REUSEPORT option: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
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
            b"Failed to set socket options (SO_BROADCAST): %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(sock);
        free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut mreq_buf: [::core::ffi::c_char; 8] = [0; 8];
    let mut imr_multiaddr: *mut in_addr =
        &raw mut mreq_buf as *mut ::core::ffi::c_char as *mut in_addr;
    let mut imr_interface: *mut in_addr = (&raw mut mreq_buf as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<in_addr>() as isize)
        as *mut in_addr;
    (*imr_multiaddr).s_addr =
        inet_addr(b"239.255.255.250\0".as_ptr() as *const ::core::ffi::c_char);
    (*imr_interface).s_addr = __bswap_32(0 as ::core::ffi::c_int as __uint32_t) as in_addr_t;
    if setsockopt(
        sock,
        C2Rust_Unnamed_1::IPPROTO_IP.0 as ::core::ffi::c_int,
        IP_ADD_MEMBERSHIP,
        &raw mut mreq_buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to join multicast group: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
    }
    memset(
        &raw mut addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_in>(),
    );
    addr.sin_family = AF_INET as sa_family_t;
    addr.sin_port = __bswap_16(3702 as __uint16_t) as in_port_t;
    addr.sin_addr.s_addr = __bswap_32(0 as ::core::ffi::c_int as __uint32_t) as in_addr_t;
    let mut bind_attempts: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_bind_attempts: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut bind_result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut original_port: ::core::ffi::c_int = __bswap_16(addr.sin_port) as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Waiting 2 seconds before binding to allow TIME_WAIT sockets to clear\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    sleep(2 as ::core::ffi::c_uint);
    while bind_attempts < max_bind_attempts {
        bind_result = bind(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut addr as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        );
        if bind_result == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Successfully bound to port %d\0".as_ptr() as *const ::core::ffi::c_char,
                __bswap_16(addr.sin_port) as ::core::ffi::c_int,
            );
            break;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to bind to port %d: %s (attempt %d/%d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                __bswap_16(addr.sin_port) as ::core::ffi::c_int,
                strerror(*__errno_location()),
                bind_attempts + 1 as ::core::ffi::c_int,
                max_bind_attempts,
            );
            if *__errno_location() != EADDRINUSE {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Trying a different port...\0".as_ptr() as *const ::core::ffi::c_char,
                );
                addr.sin_port = __bswap_16(
                    (original_port + bind_attempts + 1 as ::core::ffi::c_int) as __uint16_t,
                ) as in_port_t;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Address in use, waiting 1 second before retry\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                sleep(1 as ::core::ffi::c_uint);
                if bind_attempts >= 2 as ::core::ffi::c_int {
                    addr.sin_port = __bswap_16(
                        (original_port + 10000 as ::core::ffi::c_int + bind_attempts) as __uint16_t,
                    ) as in_port_t;
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Trying alternative port %d\0".as_ptr() as *const ::core::ffi::c_char,
                        __bswap_16(addr.sin_port) as ::core::ffi::c_int,
                    );
                }
            }
            bind_attempts += 1;
        }
    }
    if bind_result < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to bind socket after %d attempts: %s\0".as_ptr() as *const ::core::ffi::c_char,
            max_bind_attempts,
            strerror(*__errno_location()),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Continuing with discovery process, but may not receive responses\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    usleep(100000 as ::core::ffi::c_int as __useconds_t);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Waiting for discovery responses (timeout: %d seconds, attempts: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        timeout_sec,
        max_attempts,
    );
    let mut attempt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while attempt < max_attempts {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Waiting for responses, attempt %d/%d\0".as_ptr() as *const ::core::ffi::c_char,
            attempt + 1 as ::core::ffi::c_int,
            max_attempts,
        );
        let mut __i: ::core::ffi::c_uint = 0;
        let mut __arr: *mut fd_set = &raw mut readfds;
        __i = 0 as ::core::ffi::c_uint;
        while (__i as usize)
            < ::core::mem::size_of::<fd_set>().wrapping_div(::core::mem::size_of::<__fd_mask>())
        {
            (*__arr).fds_bits[__i as usize] = 0 as __fd_mask;
            __i = __i.wrapping_add(1);
        }
        readfds.fds_bits[(sock / __NFDBITS) as usize] |=
            ((1 as ::core::ffi::c_ulong) << sock % __NFDBITS) as __fd_mask;
        timeout.tv_sec = timeout_sec as __time_t;
        timeout.tv_usec = 0 as __suseconds_t;
        ret = select(
            sock + 1 as ::core::ffi::c_int,
            &raw mut readfds,
            ::core::ptr::null_mut::<fd_set>(),
            ::core::ptr::null_mut::<fd_set>(),
            &raw mut timeout,
        ) as ssize_t;
        if ret < 0 as ssize_t {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Select failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            close(sock);
            free(buffer as *mut ::core::ffi::c_void);
            return count;
        } else {
            if ret == 0 as ssize_t {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Timeout waiting for responses, no data available\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                while count < max_devices {
                    ret = recvfrom(
                        sock,
                        buffer as *mut ::core::ffi::c_void,
                        (buffer_size - 1 as ::core::ffi::c_int) as size_t,
                        C2Rust_Unnamed::MSG_DONTWAIT.0 as ::core::ffi::c_int,
                        __SOCKADDR_ARG {
                            __sockaddr__: &raw mut addr as *mut sockaddr,
                        },
                        &raw mut addr_len,
                    );
                    if ret < 0 as ssize_t {
                        if *__errno_location() == EAGAIN || *__errno_location() == EWOULDBLOCK {
                            break;
                        }
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to receive data: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            strerror(*__errno_location()),
                        );
                        break;
                    } else {
                        let mut ip_str: [::core::ffi::c_char; 16] = [0; 16];
                        inet_ntop(
                            AF_INET,
                            &raw mut addr.sin_addr as *const ::core::ffi::c_void,
                            &raw mut ip_str as *mut ::core::ffi::c_char,
                            INET_ADDRSTRLEN as socklen_t,
                        );
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Received %d bytes from %s:%d\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            ret as ::core::ffi::c_int,
                            &raw mut ip_str as *mut ::core::ffi::c_char,
                            __bswap_16(addr.sin_port) as ::core::ffi::c_int,
                        );
                        *buffer.offset(ret) = '\0' as ::core::ffi::c_char;
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_DEBUG,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Received ONVIF discovery response payload (%d bytes)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            ret as ::core::ffi::c_int,
                        );
                        if parse_device_info(buffer, devices.offset(count as isize))
                            == 0 as ::core::ffi::c_int
                        {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Discovered ONVIF device: %s (%s)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*devices.offset(count as isize)).device_service
                                    as *mut ::core::ffi::c_char,
                                &raw mut (*devices.offset(count as isize)).ip_address
                                    as *mut ::core::ffi::c_char,
                            );
                            let mut duplicate: bool = r#false != 0;
                            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i < count {
                                if strcmp(
                                    &raw mut (*devices.offset(i as isize)).ip_address
                                        as *mut ::core::ffi::c_char,
                                    &raw mut (*devices.offset(count as isize)).ip_address
                                        as *mut ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    duplicate = r#true != 0;
                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                            if !duplicate {
                                count += 1;
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_DEBUG,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Skipping duplicate device: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut (*devices.offset(count as isize)).ip_address
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_DEBUG,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Failed to parse device info from response\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        if count >= max_devices {
                            break;
                        }
                    }
                }
            }
            attempt += 1;
        }
    }
    close(sock);
    free(buffer as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Discovery response collection completed, found %d devices\0".as_ptr()
            as *const ::core::ffi::c_char,
        count,
    );
    return count;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
}
pub const __NFDBITS: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_REUSEADDR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SO_BROADCAST: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SO_RCVBUF: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SO_REUSEPORT: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EADDRINUSE: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
