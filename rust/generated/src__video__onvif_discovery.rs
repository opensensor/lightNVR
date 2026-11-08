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
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn getpid() -> __pid_t;
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
    unsafe fn connect(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
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
    unsafe fn inet_ntoa(__in: in_addr) -> *mut ::core::ffi::c_char;
    unsafe fn inet_pton(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn inet_aton(
        __cp: *const ::core::ffi::c_char,
        __inp: *mut in_addr,
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...)
    -> ::core::ffi::c_int;
    unsafe fn curl_slist_append(
        list: *mut curl_slist,
        data: *const ::core::ffi::c_char,
    ) -> *mut curl_slist;
    unsafe fn curl_slist_free_all(list: *mut curl_slist);
    unsafe fn curl_easy_init() -> *mut ::core::ffi::c_void;
    unsafe fn curl_easy_setopt(curl: *mut ::core::ffi::c_void, option: CURLoption, ...)
    -> CURLcode;
    unsafe fn curl_easy_perform(curl: *mut ::core::ffi::c_void) -> CURLcode;
    unsafe fn curl_easy_cleanup(curl: *mut ::core::ffi::c_void);
    unsafe fn curl_easy_getinfo(curl: *mut ::core::ffi::c_void, info: CURLINFO, ...) -> CURLcode;
    static mut g_config: config_t;
    unsafe fn parse_network(
        network: *const ::core::ffi::c_char,
        base_addr: *mut uint32_t,
        subnet_mask: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn detect_local_networks(
        networks: *mut [::core::ffi::c_char; 64],
        max_networks: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn send_all_discovery_probes(
        sock: ::core::ffi::c_int,
        ip_addr: *const ::core::ffi::c_char,
        dest_addr: *mut sockaddr_in,
    ) -> ::core::ffi::c_int;
    unsafe fn receive_discovery_responses(
        devices: *mut onvif_device_info_t,
        max_devices: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn receive_extended_discovery_responses(
        devices: *mut onvif_device_info_t,
        max_devices: ::core::ffi::c_int,
        timeout_sec: ::core::ffi::c_int,
        max_attempts: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn curl_init_global() -> ::core::ffi::c_int;
    unsafe fn db_onvif_inventory_record_scan(
        scan_network: *const ::core::ffi::c_char,
        devices: *mut onvif_device_info_t,
        count: ::core::ffi::c_int,
        observed_at: int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_onvif_inventory_list(
        devices: *mut onvif_device_info_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn select(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::core::ffi::c_int;
    unsafe fn srand(__seed: ::core::ffi::c_uint);
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type time_t = __time_t;
pub type int64_t = i64;
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
pub type uint64_t = u64;
pub type CURL = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_slist {
    pub data: *mut ::core::ffi::c_char,
    pub next: *mut curl_slist,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CURLcode(pub ::core::ffi::c_uint);
impl CURLcode {
    pub const CURLE_OK: Self = Self(0);
    pub const CURLE_UNSUPPORTED_PROTOCOL: Self = Self(1);
    pub const CURLE_FAILED_INIT: Self = Self(2);
    pub const CURLE_URL_MALFORMAT: Self = Self(3);
    pub const CURLE_NOT_BUILT_IN: Self = Self(4);
    pub const CURLE_COULDNT_RESOLVE_PROXY: Self = Self(5);
    pub const CURLE_COULDNT_RESOLVE_HOST: Self = Self(6);
    pub const CURLE_COULDNT_CONNECT: Self = Self(7);
    pub const CURLE_WEIRD_SERVER_REPLY: Self = Self(8);
    pub const CURLE_REMOTE_ACCESS_DENIED: Self = Self(9);
    pub const CURLE_FTP_ACCEPT_FAILED: Self = Self(10);
    pub const CURLE_FTP_WEIRD_PASS_REPLY: Self = Self(11);
    pub const CURLE_FTP_ACCEPT_TIMEOUT: Self = Self(12);
    pub const CURLE_FTP_WEIRD_PASV_REPLY: Self = Self(13);
    pub const CURLE_FTP_WEIRD_227_FORMAT: Self = Self(14);
    pub const CURLE_FTP_CANT_GET_HOST: Self = Self(15);
    pub const CURLE_HTTP2: Self = Self(16);
    pub const CURLE_FTP_COULDNT_SET_TYPE: Self = Self(17);
    pub const CURLE_PARTIAL_FILE: Self = Self(18);
    pub const CURLE_FTP_COULDNT_RETR_FILE: Self = Self(19);
    pub const CURLE_OBSOLETE20: Self = Self(20);
    pub const CURLE_QUOTE_ERROR: Self = Self(21);
    pub const CURLE_HTTP_RETURNED_ERROR: Self = Self(22);
    pub const CURLE_WRITE_ERROR: Self = Self(23);
    pub const CURLE_OBSOLETE24: Self = Self(24);
    pub const CURLE_UPLOAD_FAILED: Self = Self(25);
    pub const CURLE_READ_ERROR: Self = Self(26);
    pub const CURLE_OUT_OF_MEMORY: Self = Self(27);
    pub const CURLE_OPERATION_TIMEDOUT: Self = Self(28);
    pub const CURLE_OBSOLETE29: Self = Self(29);
    pub const CURLE_FTP_PORT_FAILED: Self = Self(30);
    pub const CURLE_FTP_COULDNT_USE_REST: Self = Self(31);
    pub const CURLE_OBSOLETE32: Self = Self(32);
    pub const CURLE_RANGE_ERROR: Self = Self(33);
    pub const CURLE_OBSOLETE34: Self = Self(34);
    pub const CURLE_SSL_CONNECT_ERROR: Self = Self(35);
    pub const CURLE_BAD_DOWNLOAD_RESUME: Self = Self(36);
    pub const CURLE_FILE_COULDNT_READ_FILE: Self = Self(37);
    pub const CURLE_LDAP_CANNOT_BIND: Self = Self(38);
    pub const CURLE_LDAP_SEARCH_FAILED: Self = Self(39);
    pub const CURLE_OBSOLETE40: Self = Self(40);
    pub const CURLE_OBSOLETE41: Self = Self(41);
    pub const CURLE_ABORTED_BY_CALLBACK: Self = Self(42);
    pub const CURLE_BAD_FUNCTION_ARGUMENT: Self = Self(43);
    pub const CURLE_OBSOLETE44: Self = Self(44);
    pub const CURLE_INTERFACE_FAILED: Self = Self(45);
    pub const CURLE_OBSOLETE46: Self = Self(46);
    pub const CURLE_TOO_MANY_REDIRECTS: Self = Self(47);
    pub const CURLE_UNKNOWN_OPTION: Self = Self(48);
    pub const CURLE_SETOPT_OPTION_SYNTAX: Self = Self(49);
    pub const CURLE_OBSOLETE50: Self = Self(50);
    pub const CURLE_OBSOLETE51: Self = Self(51);
    pub const CURLE_GOT_NOTHING: Self = Self(52);
    pub const CURLE_SSL_ENGINE_NOTFOUND: Self = Self(53);
    pub const CURLE_SSL_ENGINE_SETFAILED: Self = Self(54);
    pub const CURLE_SEND_ERROR: Self = Self(55);
    pub const CURLE_RECV_ERROR: Self = Self(56);
    pub const CURLE_OBSOLETE57: Self = Self(57);
    pub const CURLE_SSL_CERTPROBLEM: Self = Self(58);
    pub const CURLE_SSL_CIPHER: Self = Self(59);
    pub const CURLE_PEER_FAILED_VERIFICATION: Self = Self(60);
    pub const CURLE_BAD_CONTENT_ENCODING: Self = Self(61);
    pub const CURLE_OBSOLETE62: Self = Self(62);
    pub const CURLE_FILESIZE_EXCEEDED: Self = Self(63);
    pub const CURLE_USE_SSL_FAILED: Self = Self(64);
    pub const CURLE_SEND_FAIL_REWIND: Self = Self(65);
    pub const CURLE_SSL_ENGINE_INITFAILED: Self = Self(66);
    pub const CURLE_LOGIN_DENIED: Self = Self(67);
    pub const CURLE_TFTP_NOTFOUND: Self = Self(68);
    pub const CURLE_TFTP_PERM: Self = Self(69);
    pub const CURLE_REMOTE_DISK_FULL: Self = Self(70);
    pub const CURLE_TFTP_ILLEGAL: Self = Self(71);
    pub const CURLE_TFTP_UNKNOWNID: Self = Self(72);
    pub const CURLE_REMOTE_FILE_EXISTS: Self = Self(73);
    pub const CURLE_TFTP_NOSUCHUSER: Self = Self(74);
    pub const CURLE_OBSOLETE75: Self = Self(75);
    pub const CURLE_OBSOLETE76: Self = Self(76);
    pub const CURLE_SSL_CACERT_BADFILE: Self = Self(77);
    pub const CURLE_REMOTE_FILE_NOT_FOUND: Self = Self(78);
    pub const CURLE_SSH: Self = Self(79);
    pub const CURLE_SSL_SHUTDOWN_FAILED: Self = Self(80);
    pub const CURLE_AGAIN: Self = Self(81);
    pub const CURLE_SSL_CRL_BADFILE: Self = Self(82);
    pub const CURLE_SSL_ISSUER_ERROR: Self = Self(83);
    pub const CURLE_FTP_PRET_FAILED: Self = Self(84);
    pub const CURLE_RTSP_CSEQ_ERROR: Self = Self(85);
    pub const CURLE_RTSP_SESSION_ERROR: Self = Self(86);
    pub const CURLE_FTP_BAD_FILE_LIST: Self = Self(87);
    pub const CURLE_CHUNK_FAILED: Self = Self(88);
    pub const CURLE_NO_CONNECTION_AVAILABLE: Self = Self(89);
    pub const CURLE_SSL_PINNEDPUBKEYNOTMATCH: Self = Self(90);
    pub const CURLE_SSL_INVALIDCERTSTATUS: Self = Self(91);
    pub const CURLE_HTTP2_STREAM: Self = Self(92);
    pub const CURLE_RECURSIVE_API_CALL: Self = Self(93);
    pub const CURLE_AUTH_ERROR: Self = Self(94);
    pub const CURLE_HTTP3: Self = Self(95);
    pub const CURLE_QUIC_CONNECT_ERROR: Self = Self(96);
    pub const CURLE_PROXY: Self = Self(97);
    pub const CURLE_SSL_CLIENTCERT: Self = Self(98);
    pub const CURLE_UNRECOVERABLE_POLL: Self = Self(99);
    pub const CURLE_TOO_LARGE: Self = Self(100);
    pub const CURLE_ECH_REQUIRED: Self = Self(101);
    pub const CURL_LAST: Self = Self(102);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CURLoption(pub ::core::ffi::c_uint);
impl CURLoption {
    pub const CURLOPT_WRITEDATA: Self = Self(10001);
    pub const CURLOPT_URL: Self = Self(10002);
    pub const CURLOPT_PORT: Self = Self(3);
    pub const CURLOPT_PROXY: Self = Self(10004);
    pub const CURLOPT_USERPWD: Self = Self(10005);
    pub const CURLOPT_PROXYUSERPWD: Self = Self(10006);
    pub const CURLOPT_RANGE: Self = Self(10007);
    pub const CURLOPT_READDATA: Self = Self(10009);
    pub const CURLOPT_ERRORBUFFER: Self = Self(10010);
    pub const CURLOPT_WRITEFUNCTION: Self = Self(20011);
    pub const CURLOPT_READFUNCTION: Self = Self(20012);
    pub const CURLOPT_TIMEOUT: Self = Self(13);
    pub const CURLOPT_INFILESIZE: Self = Self(14);
    pub const CURLOPT_POSTFIELDS: Self = Self(10015);
    pub const CURLOPT_REFERER: Self = Self(10016);
    pub const CURLOPT_FTPPORT: Self = Self(10017);
    pub const CURLOPT_USERAGENT: Self = Self(10018);
    pub const CURLOPT_LOW_SPEED_LIMIT: Self = Self(19);
    pub const CURLOPT_LOW_SPEED_TIME: Self = Self(20);
    pub const CURLOPT_RESUME_FROM: Self = Self(21);
    pub const CURLOPT_COOKIE: Self = Self(10022);
    pub const CURLOPT_HTTPHEADER: Self = Self(10023);
    pub const CURLOPT_HTTPPOST: Self = Self(10024);
    pub const CURLOPT_SSLCERT: Self = Self(10025);
    pub const CURLOPT_KEYPASSWD: Self = Self(10026);
    pub const CURLOPT_CRLF: Self = Self(27);
    pub const CURLOPT_QUOTE: Self = Self(10028);
    pub const CURLOPT_HEADERDATA: Self = Self(10029);
    pub const CURLOPT_COOKIEFILE: Self = Self(10031);
    pub const CURLOPT_SSLVERSION: Self = Self(32);
    pub const CURLOPT_TIMECONDITION: Self = Self(33);
    pub const CURLOPT_TIMEVALUE: Self = Self(34);
    pub const CURLOPT_CUSTOMREQUEST: Self = Self(10036);
    pub const CURLOPT_STDERR: Self = Self(10037);
    pub const CURLOPT_POSTQUOTE: Self = Self(10039);
    pub const CURLOPT_VERBOSE: Self = Self(41);
    pub const CURLOPT_HEADER: Self = Self(42);
    pub const CURLOPT_NOPROGRESS: Self = Self(43);
    pub const CURLOPT_NOBODY: Self = Self(44);
    pub const CURLOPT_FAILONERROR: Self = Self(45);
    pub const CURLOPT_UPLOAD: Self = Self(46);
    pub const CURLOPT_POST: Self = Self(47);
    pub const CURLOPT_DIRLISTONLY: Self = Self(48);
    pub const CURLOPT_APPEND: Self = Self(50);
    pub const CURLOPT_NETRC: Self = Self(51);
    pub const CURLOPT_FOLLOWLOCATION: Self = Self(52);
    pub const CURLOPT_TRANSFERTEXT: Self = Self(53);
    pub const CURLOPT_PUT: Self = Self(54);
    pub const CURLOPT_PROGRESSFUNCTION: Self = Self(20056);
    pub const CURLOPT_XFERINFODATA: Self = Self(10057);
    pub const CURLOPT_AUTOREFERER: Self = Self(58);
    pub const CURLOPT_PROXYPORT: Self = Self(59);
    pub const CURLOPT_POSTFIELDSIZE: Self = Self(60);
    pub const CURLOPT_HTTPPROXYTUNNEL: Self = Self(61);
    pub const CURLOPT_INTERFACE: Self = Self(10062);
    pub const CURLOPT_KRBLEVEL: Self = Self(10063);
    pub const CURLOPT_SSL_VERIFYPEER: Self = Self(64);
    pub const CURLOPT_CAINFO: Self = Self(10065);
    pub const CURLOPT_MAXREDIRS: Self = Self(68);
    pub const CURLOPT_FILETIME: Self = Self(69);
    pub const CURLOPT_TELNETOPTIONS: Self = Self(10070);
    pub const CURLOPT_MAXCONNECTS: Self = Self(71);
    pub const CURLOPT_FRESH_CONNECT: Self = Self(74);
    pub const CURLOPT_FORBID_REUSE: Self = Self(75);
    pub const CURLOPT_RANDOM_FILE: Self = Self(10076);
    pub const CURLOPT_EGDSOCKET: Self = Self(10077);
    pub const CURLOPT_CONNECTTIMEOUT: Self = Self(78);
    pub const CURLOPT_HEADERFUNCTION: Self = Self(20079);
    pub const CURLOPT_HTTPGET: Self = Self(80);
    pub const CURLOPT_SSL_VERIFYHOST: Self = Self(81);
    pub const CURLOPT_COOKIEJAR: Self = Self(10082);
    pub const CURLOPT_SSL_CIPHER_LIST: Self = Self(10083);
    pub const CURLOPT_HTTP_VERSION: Self = Self(84);
    pub const CURLOPT_FTP_USE_EPSV: Self = Self(85);
    pub const CURLOPT_SSLCERTTYPE: Self = Self(10086);
    pub const CURLOPT_SSLKEY: Self = Self(10087);
    pub const CURLOPT_SSLKEYTYPE: Self = Self(10088);
    pub const CURLOPT_SSLENGINE: Self = Self(10089);
    pub const CURLOPT_SSLENGINE_DEFAULT: Self = Self(90);
    pub const CURLOPT_DNS_USE_GLOBAL_CACHE: Self = Self(91);
    pub const CURLOPT_DNS_CACHE_TIMEOUT: Self = Self(92);
    pub const CURLOPT_PREQUOTE: Self = Self(10093);
    pub const CURLOPT_DEBUGFUNCTION: Self = Self(20094);
    pub const CURLOPT_DEBUGDATA: Self = Self(10095);
    pub const CURLOPT_COOKIESESSION: Self = Self(96);
    pub const CURLOPT_CAPATH: Self = Self(10097);
    pub const CURLOPT_BUFFERSIZE: Self = Self(98);
    pub const CURLOPT_NOSIGNAL: Self = Self(99);
    pub const CURLOPT_SHARE: Self = Self(10100);
    pub const CURLOPT_PROXYTYPE: Self = Self(101);
    pub const CURLOPT_ACCEPT_ENCODING: Self = Self(10102);
    pub const CURLOPT_PRIVATE: Self = Self(10103);
    pub const CURLOPT_HTTP200ALIASES: Self = Self(10104);
    pub const CURLOPT_UNRESTRICTED_AUTH: Self = Self(105);
    pub const CURLOPT_FTP_USE_EPRT: Self = Self(106);
    pub const CURLOPT_HTTPAUTH: Self = Self(107);
    pub const CURLOPT_SSL_CTX_FUNCTION: Self = Self(20108);
    pub const CURLOPT_SSL_CTX_DATA: Self = Self(10109);
    pub const CURLOPT_FTP_CREATE_MISSING_DIRS: Self = Self(110);
    pub const CURLOPT_PROXYAUTH: Self = Self(111);
    pub const CURLOPT_SERVER_RESPONSE_TIMEOUT: Self = Self(112);
    pub const CURLOPT_IPRESOLVE: Self = Self(113);
    pub const CURLOPT_MAXFILESIZE: Self = Self(114);
    pub const CURLOPT_INFILESIZE_LARGE: Self = Self(30115);
    pub const CURLOPT_RESUME_FROM_LARGE: Self = Self(30116);
    pub const CURLOPT_MAXFILESIZE_LARGE: Self = Self(30117);
    pub const CURLOPT_NETRC_FILE: Self = Self(10118);
    pub const CURLOPT_USE_SSL: Self = Self(119);
    pub const CURLOPT_POSTFIELDSIZE_LARGE: Self = Self(30120);
    pub const CURLOPT_TCP_NODELAY: Self = Self(121);
    pub const CURLOPT_FTPSSLAUTH: Self = Self(129);
    pub const CURLOPT_IOCTLFUNCTION: Self = Self(20130);
    pub const CURLOPT_IOCTLDATA: Self = Self(10131);
    pub const CURLOPT_FTP_ACCOUNT: Self = Self(10134);
    pub const CURLOPT_COOKIELIST: Self = Self(10135);
    pub const CURLOPT_IGNORE_CONTENT_LENGTH: Self = Self(136);
    pub const CURLOPT_FTP_SKIP_PASV_IP: Self = Self(137);
    pub const CURLOPT_FTP_FILEMETHOD: Self = Self(138);
    pub const CURLOPT_LOCALPORT: Self = Self(139);
    pub const CURLOPT_LOCALPORTRANGE: Self = Self(140);
    pub const CURLOPT_CONNECT_ONLY: Self = Self(141);
    pub const CURLOPT_CONV_FROM_NETWORK_FUNCTION: Self = Self(20142);
    pub const CURLOPT_CONV_TO_NETWORK_FUNCTION: Self = Self(20143);
    pub const CURLOPT_CONV_FROM_UTF8_FUNCTION: Self = Self(20144);
    pub const CURLOPT_MAX_SEND_SPEED_LARGE: Self = Self(30145);
    pub const CURLOPT_MAX_RECV_SPEED_LARGE: Self = Self(30146);
    pub const CURLOPT_FTP_ALTERNATIVE_TO_USER: Self = Self(10147);
    pub const CURLOPT_SOCKOPTFUNCTION: Self = Self(20148);
    pub const CURLOPT_SOCKOPTDATA: Self = Self(10149);
    pub const CURLOPT_SSL_SESSIONID_CACHE: Self = Self(150);
    pub const CURLOPT_SSH_AUTH_TYPES: Self = Self(151);
    pub const CURLOPT_SSH_PUBLIC_KEYFILE: Self = Self(10152);
    pub const CURLOPT_SSH_PRIVATE_KEYFILE: Self = Self(10153);
    pub const CURLOPT_FTP_SSL_CCC: Self = Self(154);
    pub const CURLOPT_TIMEOUT_MS: Self = Self(155);
    pub const CURLOPT_CONNECTTIMEOUT_MS: Self = Self(156);
    pub const CURLOPT_HTTP_TRANSFER_DECODING: Self = Self(157);
    pub const CURLOPT_HTTP_CONTENT_DECODING: Self = Self(158);
    pub const CURLOPT_NEW_FILE_PERMS: Self = Self(159);
    pub const CURLOPT_NEW_DIRECTORY_PERMS: Self = Self(160);
    pub const CURLOPT_POSTREDIR: Self = Self(161);
    pub const CURLOPT_SSH_HOST_PUBLIC_KEY_MD5: Self = Self(10162);
    pub const CURLOPT_OPENSOCKETFUNCTION: Self = Self(20163);
    pub const CURLOPT_OPENSOCKETDATA: Self = Self(10164);
    pub const CURLOPT_COPYPOSTFIELDS: Self = Self(10165);
    pub const CURLOPT_PROXY_TRANSFER_MODE: Self = Self(166);
    pub const CURLOPT_SEEKFUNCTION: Self = Self(20167);
    pub const CURLOPT_SEEKDATA: Self = Self(10168);
    pub const CURLOPT_CRLFILE: Self = Self(10169);
    pub const CURLOPT_ISSUERCERT: Self = Self(10170);
    pub const CURLOPT_ADDRESS_SCOPE: Self = Self(171);
    pub const CURLOPT_CERTINFO: Self = Self(172);
    pub const CURLOPT_USERNAME: Self = Self(10173);
    pub const CURLOPT_PASSWORD: Self = Self(10174);
    pub const CURLOPT_PROXYUSERNAME: Self = Self(10175);
    pub const CURLOPT_PROXYPASSWORD: Self = Self(10176);
    pub const CURLOPT_NOPROXY: Self = Self(10177);
    pub const CURLOPT_TFTP_BLKSIZE: Self = Self(178);
    pub const CURLOPT_SOCKS5_GSSAPI_SERVICE: Self = Self(10179);
    pub const CURLOPT_SOCKS5_GSSAPI_NEC: Self = Self(180);
    pub const CURLOPT_PROTOCOLS: Self = Self(181);
    pub const CURLOPT_REDIR_PROTOCOLS: Self = Self(182);
    pub const CURLOPT_SSH_KNOWNHOSTS: Self = Self(10183);
    pub const CURLOPT_SSH_KEYFUNCTION: Self = Self(20184);
    pub const CURLOPT_SSH_KEYDATA: Self = Self(10185);
    pub const CURLOPT_MAIL_FROM: Self = Self(10186);
    pub const CURLOPT_MAIL_RCPT: Self = Self(10187);
    pub const CURLOPT_FTP_USE_PRET: Self = Self(188);
    pub const CURLOPT_RTSP_REQUEST: Self = Self(189);
    pub const CURLOPT_RTSP_SESSION_ID: Self = Self(10190);
    pub const CURLOPT_RTSP_STREAM_URI: Self = Self(10191);
    pub const CURLOPT_RTSP_TRANSPORT: Self = Self(10192);
    pub const CURLOPT_RTSP_CLIENT_CSEQ: Self = Self(193);
    pub const CURLOPT_RTSP_SERVER_CSEQ: Self = Self(194);
    pub const CURLOPT_INTERLEAVEDATA: Self = Self(10195);
    pub const CURLOPT_INTERLEAVEFUNCTION: Self = Self(20196);
    pub const CURLOPT_WILDCARDMATCH: Self = Self(197);
    pub const CURLOPT_CHUNK_BGN_FUNCTION: Self = Self(20198);
    pub const CURLOPT_CHUNK_END_FUNCTION: Self = Self(20199);
    pub const CURLOPT_FNMATCH_FUNCTION: Self = Self(20200);
    pub const CURLOPT_CHUNK_DATA: Self = Self(10201);
    pub const CURLOPT_FNMATCH_DATA: Self = Self(10202);
    pub const CURLOPT_RESOLVE: Self = Self(10203);
    pub const CURLOPT_TLSAUTH_USERNAME: Self = Self(10204);
    pub const CURLOPT_TLSAUTH_PASSWORD: Self = Self(10205);
    pub const CURLOPT_TLSAUTH_TYPE: Self = Self(10206);
    pub const CURLOPT_TRANSFER_ENCODING: Self = Self(207);
    pub const CURLOPT_CLOSESOCKETFUNCTION: Self = Self(20208);
    pub const CURLOPT_CLOSESOCKETDATA: Self = Self(10209);
    pub const CURLOPT_GSSAPI_DELEGATION: Self = Self(210);
    pub const CURLOPT_DNS_SERVERS: Self = Self(10211);
    pub const CURLOPT_ACCEPTTIMEOUT_MS: Self = Self(212);
    pub const CURLOPT_TCP_KEEPALIVE: Self = Self(213);
    pub const CURLOPT_TCP_KEEPIDLE: Self = Self(214);
    pub const CURLOPT_TCP_KEEPINTVL: Self = Self(215);
    pub const CURLOPT_SSL_OPTIONS: Self = Self(216);
    pub const CURLOPT_MAIL_AUTH: Self = Self(10217);
    pub const CURLOPT_SASL_IR: Self = Self(218);
    pub const CURLOPT_XFERINFOFUNCTION: Self = Self(20219);
    pub const CURLOPT_XOAUTH2_BEARER: Self = Self(10220);
    pub const CURLOPT_DNS_INTERFACE: Self = Self(10221);
    pub const CURLOPT_DNS_LOCAL_IP4: Self = Self(10222);
    pub const CURLOPT_DNS_LOCAL_IP6: Self = Self(10223);
    pub const CURLOPT_LOGIN_OPTIONS: Self = Self(10224);
    pub const CURLOPT_SSL_ENABLE_NPN: Self = Self(225);
    pub const CURLOPT_SSL_ENABLE_ALPN: Self = Self(226);
    pub const CURLOPT_EXPECT_100_TIMEOUT_MS: Self = Self(227);
    pub const CURLOPT_PROXYHEADER: Self = Self(10228);
    pub const CURLOPT_HEADEROPT: Self = Self(229);
    pub const CURLOPT_PINNEDPUBLICKEY: Self = Self(10230);
    pub const CURLOPT_UNIX_SOCKET_PATH: Self = Self(10231);
    pub const CURLOPT_SSL_VERIFYSTATUS: Self = Self(232);
    pub const CURLOPT_SSL_FALSESTART: Self = Self(233);
    pub const CURLOPT_PATH_AS_IS: Self = Self(234);
    pub const CURLOPT_PROXY_SERVICE_NAME: Self = Self(10235);
    pub const CURLOPT_SERVICE_NAME: Self = Self(10236);
    pub const CURLOPT_PIPEWAIT: Self = Self(237);
    pub const CURLOPT_DEFAULT_PROTOCOL: Self = Self(10238);
    pub const CURLOPT_STREAM_WEIGHT: Self = Self(239);
    pub const CURLOPT_STREAM_DEPENDS: Self = Self(10240);
    pub const CURLOPT_STREAM_DEPENDS_E: Self = Self(10241);
    pub const CURLOPT_TFTP_NO_OPTIONS: Self = Self(242);
    pub const CURLOPT_CONNECT_TO: Self = Self(10243);
    pub const CURLOPT_TCP_FASTOPEN: Self = Self(244);
    pub const CURLOPT_KEEP_SENDING_ON_ERROR: Self = Self(245);
    pub const CURLOPT_PROXY_CAINFO: Self = Self(10246);
    pub const CURLOPT_PROXY_CAPATH: Self = Self(10247);
    pub const CURLOPT_PROXY_SSL_VERIFYPEER: Self = Self(248);
    pub const CURLOPT_PROXY_SSL_VERIFYHOST: Self = Self(249);
    pub const CURLOPT_PROXY_SSLVERSION: Self = Self(250);
    pub const CURLOPT_PROXY_TLSAUTH_USERNAME: Self = Self(10251);
    pub const CURLOPT_PROXY_TLSAUTH_PASSWORD: Self = Self(10252);
    pub const CURLOPT_PROXY_TLSAUTH_TYPE: Self = Self(10253);
    pub const CURLOPT_PROXY_SSLCERT: Self = Self(10254);
    pub const CURLOPT_PROXY_SSLCERTTYPE: Self = Self(10255);
    pub const CURLOPT_PROXY_SSLKEY: Self = Self(10256);
    pub const CURLOPT_PROXY_SSLKEYTYPE: Self = Self(10257);
    pub const CURLOPT_PROXY_KEYPASSWD: Self = Self(10258);
    pub const CURLOPT_PROXY_SSL_CIPHER_LIST: Self = Self(10259);
    pub const CURLOPT_PROXY_CRLFILE: Self = Self(10260);
    pub const CURLOPT_PROXY_SSL_OPTIONS: Self = Self(261);
    pub const CURLOPT_PRE_PROXY: Self = Self(10262);
    pub const CURLOPT_PROXY_PINNEDPUBLICKEY: Self = Self(10263);
    pub const CURLOPT_ABSTRACT_UNIX_SOCKET: Self = Self(10264);
    pub const CURLOPT_SUPPRESS_CONNECT_HEADERS: Self = Self(265);
    pub const CURLOPT_REQUEST_TARGET: Self = Self(10266);
    pub const CURLOPT_SOCKS5_AUTH: Self = Self(267);
    pub const CURLOPT_SSH_COMPRESSION: Self = Self(268);
    pub const CURLOPT_MIMEPOST: Self = Self(10269);
    pub const CURLOPT_TIMEVALUE_LARGE: Self = Self(30270);
    pub const CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS: Self = Self(271);
    pub const CURLOPT_RESOLVER_START_FUNCTION: Self = Self(20272);
    pub const CURLOPT_RESOLVER_START_DATA: Self = Self(10273);
    pub const CURLOPT_HAPROXYPROTOCOL: Self = Self(274);
    pub const CURLOPT_DNS_SHUFFLE_ADDRESSES: Self = Self(275);
    pub const CURLOPT_TLS13_CIPHERS: Self = Self(10276);
    pub const CURLOPT_PROXY_TLS13_CIPHERS: Self = Self(10277);
    pub const CURLOPT_DISALLOW_USERNAME_IN_URL: Self = Self(278);
    pub const CURLOPT_DOH_URL: Self = Self(10279);
    pub const CURLOPT_UPLOAD_BUFFERSIZE: Self = Self(280);
    pub const CURLOPT_UPKEEP_INTERVAL_MS: Self = Self(281);
    pub const CURLOPT_CURLU: Self = Self(10282);
    pub const CURLOPT_TRAILERFUNCTION: Self = Self(20283);
    pub const CURLOPT_TRAILERDATA: Self = Self(10284);
    pub const CURLOPT_HTTP09_ALLOWED: Self = Self(285);
    pub const CURLOPT_ALTSVC_CTRL: Self = Self(286);
    pub const CURLOPT_ALTSVC: Self = Self(10287);
    pub const CURLOPT_MAXAGE_CONN: Self = Self(288);
    pub const CURLOPT_SASL_AUTHZID: Self = Self(10289);
    pub const CURLOPT_MAIL_RCPT_ALLOWFAILS: Self = Self(290);
    pub const CURLOPT_SSLCERT_BLOB: Self = Self(40291);
    pub const CURLOPT_SSLKEY_BLOB: Self = Self(40292);
    pub const CURLOPT_PROXY_SSLCERT_BLOB: Self = Self(40293);
    pub const CURLOPT_PROXY_SSLKEY_BLOB: Self = Self(40294);
    pub const CURLOPT_ISSUERCERT_BLOB: Self = Self(40295);
    pub const CURLOPT_PROXY_ISSUERCERT: Self = Self(10296);
    pub const CURLOPT_PROXY_ISSUERCERT_BLOB: Self = Self(40297);
    pub const CURLOPT_SSL_EC_CURVES: Self = Self(10298);
    pub const CURLOPT_HSTS_CTRL: Self = Self(299);
    pub const CURLOPT_HSTS: Self = Self(10300);
    pub const CURLOPT_HSTSREADFUNCTION: Self = Self(20301);
    pub const CURLOPT_HSTSREADDATA: Self = Self(10302);
    pub const CURLOPT_HSTSWRITEFUNCTION: Self = Self(20303);
    pub const CURLOPT_HSTSWRITEDATA: Self = Self(10304);
    pub const CURLOPT_AWS_SIGV4: Self = Self(10305);
    pub const CURLOPT_DOH_SSL_VERIFYPEER: Self = Self(306);
    pub const CURLOPT_DOH_SSL_VERIFYHOST: Self = Self(307);
    pub const CURLOPT_DOH_SSL_VERIFYSTATUS: Self = Self(308);
    pub const CURLOPT_CAINFO_BLOB: Self = Self(40309);
    pub const CURLOPT_PROXY_CAINFO_BLOB: Self = Self(40310);
    pub const CURLOPT_SSH_HOST_PUBLIC_KEY_SHA256: Self = Self(10311);
    pub const CURLOPT_PREREQFUNCTION: Self = Self(20312);
    pub const CURLOPT_PREREQDATA: Self = Self(10313);
    pub const CURLOPT_MAXLIFETIME_CONN: Self = Self(314);
    pub const CURLOPT_MIME_OPTIONS: Self = Self(315);
    pub const CURLOPT_SSH_HOSTKEYFUNCTION: Self = Self(20316);
    pub const CURLOPT_SSH_HOSTKEYDATA: Self = Self(10317);
    pub const CURLOPT_PROTOCOLS_STR: Self = Self(10318);
    pub const CURLOPT_REDIR_PROTOCOLS_STR: Self = Self(10319);
    pub const CURLOPT_WS_OPTIONS: Self = Self(320);
    pub const CURLOPT_CA_CACHE_TIMEOUT: Self = Self(321);
    pub const CURLOPT_QUICK_EXIT: Self = Self(322);
    pub const CURLOPT_HAPROXY_CLIENT_IP: Self = Self(10323);
    pub const CURLOPT_SERVER_RESPONSE_TIMEOUT_MS: Self = Self(324);
    pub const CURLOPT_ECH: Self = Self(10325);
    pub const CURLOPT_TCP_KEEPCNT: Self = Self(326);
    pub const CURLOPT_UPLOAD_FLAGS: Self = Self(327);
    pub const CURLOPT_SSL_SIGNATURE_ALGORITHMS: Self = Self(10328);
    pub const CURLOPT_HTTPSIG_ALGORITHM: Self = Self(329);
    pub const CURLOPT_HTTPSIG_KEY: Self = Self(10330);
    pub const CURLOPT_HTTPSIG_KEYID: Self = Self(10331);
    pub const CURLOPT_HTTPSIG_HEADERS: Self = Self(10332);
    pub const CURLOPT_LASTENTRY: Self = Self(10333);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CURLINFO(pub ::core::ffi::c_uint);
impl CURLINFO {
    pub const CURLINFO_NONE: Self = Self(0);
    pub const CURLINFO_EFFECTIVE_URL: Self = Self(1048577);
    pub const CURLINFO_RESPONSE_CODE: Self = Self(2097154);
    pub const CURLINFO_TOTAL_TIME: Self = Self(3145731);
    pub const CURLINFO_NAMELOOKUP_TIME: Self = Self(3145732);
    pub const CURLINFO_CONNECT_TIME: Self = Self(3145733);
    pub const CURLINFO_PRETRANSFER_TIME: Self = Self(3145734);
    pub const CURLINFO_SIZE_UPLOAD: Self = Self(3145735);
    pub const CURLINFO_SIZE_UPLOAD_T: Self = Self(6291463);
    pub const CURLINFO_SIZE_DOWNLOAD: Self = Self(3145736);
    pub const CURLINFO_SIZE_DOWNLOAD_T: Self = Self(6291464);
    pub const CURLINFO_SPEED_DOWNLOAD: Self = Self(3145737);
    pub const CURLINFO_SPEED_DOWNLOAD_T: Self = Self(6291465);
    pub const CURLINFO_SPEED_UPLOAD: Self = Self(3145738);
    pub const CURLINFO_SPEED_UPLOAD_T: Self = Self(6291466);
    pub const CURLINFO_HEADER_SIZE: Self = Self(2097163);
    pub const CURLINFO_REQUEST_SIZE: Self = Self(2097164);
    pub const CURLINFO_SSL_VERIFYRESULT: Self = Self(2097165);
    pub const CURLINFO_FILETIME: Self = Self(2097166);
    pub const CURLINFO_FILETIME_T: Self = Self(6291470);
    pub const CURLINFO_CONTENT_LENGTH_DOWNLOAD: Self = Self(3145743);
    pub const CURLINFO_CONTENT_LENGTH_DOWNLOAD_T: Self = Self(6291471);
    pub const CURLINFO_CONTENT_LENGTH_UPLOAD: Self = Self(3145744);
    pub const CURLINFO_CONTENT_LENGTH_UPLOAD_T: Self = Self(6291472);
    pub const CURLINFO_STARTTRANSFER_TIME: Self = Self(3145745);
    pub const CURLINFO_CONTENT_TYPE: Self = Self(1048594);
    pub const CURLINFO_REDIRECT_TIME: Self = Self(3145747);
    pub const CURLINFO_REDIRECT_COUNT: Self = Self(2097172);
    pub const CURLINFO_PRIVATE: Self = Self(1048597);
    pub const CURLINFO_HTTP_CONNECTCODE: Self = Self(2097174);
    pub const CURLINFO_HTTPAUTH_AVAIL: Self = Self(2097175);
    pub const CURLINFO_PROXYAUTH_AVAIL: Self = Self(2097176);
    pub const CURLINFO_OS_ERRNO: Self = Self(2097177);
    pub const CURLINFO_NUM_CONNECTS: Self = Self(2097178);
    pub const CURLINFO_SSL_ENGINES: Self = Self(4194331);
    pub const CURLINFO_COOKIELIST: Self = Self(4194332);
    pub const CURLINFO_LASTSOCKET: Self = Self(2097181);
    pub const CURLINFO_FTP_ENTRY_PATH: Self = Self(1048606);
    pub const CURLINFO_REDIRECT_URL: Self = Self(1048607);
    pub const CURLINFO_PRIMARY_IP: Self = Self(1048608);
    pub const CURLINFO_APPCONNECT_TIME: Self = Self(3145761);
    pub const CURLINFO_CERTINFO: Self = Self(4194338);
    pub const CURLINFO_CONDITION_UNMET: Self = Self(2097187);
    pub const CURLINFO_RTSP_SESSION_ID: Self = Self(1048612);
    pub const CURLINFO_RTSP_CLIENT_CSEQ: Self = Self(2097189);
    pub const CURLINFO_RTSP_SERVER_CSEQ: Self = Self(2097190);
    pub const CURLINFO_RTSP_CSEQ_RECV: Self = Self(2097191);
    pub const CURLINFO_PRIMARY_PORT: Self = Self(2097192);
    pub const CURLINFO_LOCAL_IP: Self = Self(1048617);
    pub const CURLINFO_LOCAL_PORT: Self = Self(2097194);
    pub const CURLINFO_TLS_SESSION: Self = Self(4194347);
    pub const CURLINFO_ACTIVESOCKET: Self = Self(5242924);
    pub const CURLINFO_TLS_SSL_PTR: Self = Self(4194349);
    pub const CURLINFO_HTTP_VERSION: Self = Self(2097198);
    pub const CURLINFO_PROXY_SSL_VERIFYRESULT: Self = Self(2097199);
    pub const CURLINFO_PROTOCOL: Self = Self(2097200);
    pub const CURLINFO_SCHEME: Self = Self(1048625);
    pub const CURLINFO_TOTAL_TIME_T: Self = Self(6291506);
    pub const CURLINFO_NAMELOOKUP_TIME_T: Self = Self(6291507);
    pub const CURLINFO_CONNECT_TIME_T: Self = Self(6291508);
    pub const CURLINFO_PRETRANSFER_TIME_T: Self = Self(6291509);
    pub const CURLINFO_STARTTRANSFER_TIME_T: Self = Self(6291510);
    pub const CURLINFO_REDIRECT_TIME_T: Self = Self(6291511);
    pub const CURLINFO_APPCONNECT_TIME_T: Self = Self(6291512);
    pub const CURLINFO_RETRY_AFTER: Self = Self(6291513);
    pub const CURLINFO_EFFECTIVE_METHOD: Self = Self(1048634);
    pub const CURLINFO_PROXY_ERROR: Self = Self(2097211);
    pub const CURLINFO_REFERER: Self = Self(1048636);
    pub const CURLINFO_CAINFO: Self = Self(1048637);
    pub const CURLINFO_CAPATH: Self = Self(1048638);
    pub const CURLINFO_XFER_ID: Self = Self(6291519);
    pub const CURLINFO_CONN_ID: Self = Self(6291520);
    pub const CURLINFO_QUEUE_TIME_T: Self = Self(6291521);
    pub const CURLINFO_USED_PROXY: Self = Self(2097218);
    pub const CURLINFO_POSTTRANSFER_TIME_T: Self = Self(6291523);
    pub const CURLINFO_EARLYDATA_SENT_T: Self = Self(6291524);
    pub const CURLINFO_HTTPAUTH_USED: Self = Self(2097221);
    pub const CURLINFO_PROXYAUTH_USED: Self = Self(2097222);
    pub const CURLINFO_SIZE_DELIVERED: Self = Self(6291527);
    pub const CURLINFO_LASTONE: Self = Self(71);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_policy_settings_t {
    pub enabled: bool,
    pub profile: [::core::ffi::c_char; 16],
    pub fast_interval_seconds: uint32_t,
    pub normal_interval_seconds: uint32_t,
    pub slow_interval_seconds: uint32_t,
    pub device_interval_seconds: uint32_t,
    pub write_probe_enabled: bool,
    pub hardware_provider: [::core::ffi::c_char; 16],
    pub presence_interval_seconds: uint32_t,
    pub incident_retention_days: uint32_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_config_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 256],
    pub url: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_int,
    pub codec: [::core::ffi::c_char; 16],
    pub priority: ::core::ffi::c_int,
    pub record: bool,
    pub segment_duration: ::core::ffi::c_int,
    pub detection_based_recording: bool,
    pub detection_model: [::core::ffi::c_char; 512],
    pub detection_interval: ::core::ffi::c_int,
    pub detection_threshold: ::core::ffi::c_float,
    pub pre_detection_buffer: ::core::ffi::c_int,
    pub post_detection_buffer: ::core::ffi::c_int,
    pub detection_api_url: [::core::ffi::c_char; 512],
    pub detection_object_filter: [::core::ffi::c_char; 16],
    pub detection_object_filter_list: [::core::ffi::c_char; 256],
    pub streaming_enabled: bool,
    pub protocol: stream_protocol_t,
    pub record_audio: bool,
    pub audio_voice_enhancement: bool,
    pub onvif_username: [::core::ffi::c_char; 64],
    pub onvif_password: [::core::ffi::c_char; 64],
    pub onvif_profile: [::core::ffi::c_char; 64],
    pub onvif_port: ::core::ffi::c_int,
    pub onvif_discovery_enabled: bool,
    pub is_onvif: bool,
    pub backchannel_enabled: bool,
    pub retention_days: ::core::ffi::c_int,
    pub detection_retention_days: ::core::ffi::c_int,
    pub max_storage_mb: ::core::ffi::c_int,
    pub tier_critical_multiplier: ::core::ffi::c_double,
    pub tier_important_multiplier: ::core::ffi::c_double,
    pub tier_ephemeral_multiplier: ::core::ffi::c_double,
    pub storage_priority: ::core::ffi::c_int,
    pub ptz_enabled: bool,
    pub ptz_max_x: ::core::ffi::c_int,
    pub ptz_max_y: ::core::ffi::c_int,
    pub ptz_max_z: ::core::ffi::c_int,
    pub ptz_has_home: bool,
    pub record_on_schedule: bool,
    pub recording_schedule: [uint8_t; 168],
    pub detection_record_on_schedule: bool,
    pub detection_recording_schedule: [uint8_t; 168],
    pub tags: [::core::ffi::c_char; 256],
    pub admin_url: [::core::ffi::c_char; 512],
    pub privacy_mode: bool,
    pub motion_trigger_source: [::core::ffi::c_char; 256],
    pub go2rtc_source_override: [::core::ffi::c_char; 8192],
    pub sub_stream_url: [::core::ffi::c_char; 512],
    pub detection_url: [::core::ffi::c_char; 512],
    pub publish_url: [::core::ffi::c_char; 512],
    pub playback_transport: [::core::ffi::c_char; 24],
    pub eptz_config: [::core::ffi::c_char; 1024],
    pub observation_evidence_enabled: bool,
    pub observation_labels: [::core::ffi::c_char; 512],
    pub observation_max_age_days: ::core::ffi::c_int,
    pub observation_max_bytes: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_engine_config_t {
    pub enabled: bool,
    pub num_threads: ::core::ffi::c_int,
    pub delegate: [::core::ffi::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_t {
    pub pid_file: [::core::ffi::c_char; 512],
    pub log_file: [::core::ffi::c_char; 512],
    pub log_level: ::core::ffi::c_int,
    pub syslog_enabled: bool,
    pub syslog_ident: [::core::ffi::c_char; 64],
    pub syslog_facility: ::core::ffi::c_int,
    pub storage_path: [::core::ffi::c_char; 512],
    pub storage_path_hls: [::core::ffi::c_char; 512],
    pub max_storage_size: uint64_t,
    pub retention_days: ::core::ffi::c_int,
    pub auto_delete_oldest: bool,
    pub mp4_faststart: bool,
    pub storage_min_free_pct: ::core::ffi::c_int,
    pub storage_pressure_warning_pct: ::core::ffi::c_double,
    pub storage_pressure_critical_pct: ::core::ffi::c_double,
    pub storage_pressure_emergency_pct: ::core::ffi::c_double,
    pub generate_thumbnails: bool,
    pub thumbnails_per_recording: ::core::ffi::c_int,
    pub record_mp4_directly: bool,
    pub mp4_storage_path: [::core::ffi::c_char; 512],
    pub mp4_directory_format: [::core::ffi::c_char; 32],
    pub mp4_segment_duration: ::core::ffi::c_int,
    pub mp4_retention_days: ::core::ffi::c_int,
    pub models_path: [::core::ffi::c_char; 512],
    pub api_detection_url: [::core::ffi::c_char; 512],
    pub api_detection_backend: [::core::ffi::c_char; 32],
    pub api_detection_format: [::core::ffi::c_char; 32],
    pub api_detection_detector_name: [::core::ffi::c_char; 64],
    pub default_detection_threshold: ::core::ffi::c_int,
    pub default_pre_detection_buffer: ::core::ffi::c_int,
    pub default_post_detection_buffer: ::core::ffi::c_int,
    pub default_playback_transport: [::core::ffi::c_char; 24],
    pub detection_grace_period: ::core::ffi::c_int,
    pub db_path: [::core::ffi::c_char; 512],
    pub db_backup_interval_minutes: ::core::ffi::c_int,
    pub db_backup_retention_count: ::core::ffi::c_int,
    pub db_post_backup_script: [::core::ffi::c_char; 512],
    pub db_startup_check: ::core::ffi::c_int,
    pub db_backup_verify: ::core::ffi::c_int,
    pub web_thread_pool_size: ::core::ffi::c_int,
    pub web_port: ::core::ffi::c_int,
    pub web_bind_ip: [::core::ffi::c_char; 32],
    pub web_root: [::core::ffi::c_char; 512],
    pub web_ssl_enabled: bool,
    pub web_ssl_cert_path: [::core::ffi::c_char; 512],
    pub web_ssl_key_path: [::core::ffi::c_char; 512],
    pub web_auth_enabled: bool,
    pub web_username: [::core::ffi::c_char; 32],
    pub web_password: [::core::ffi::c_char; 32],
    pub audio_disabled: bool,
    pub auto_disabled: bool,
    pub webrtc_disabled: bool,
    pub hls_disabled: bool,
    pub mse_disabled: bool,
    pub auth_timeout_hours: ::core::ffi::c_int,
    pub auth_absolute_timeout_hours: ::core::ffi::c_int,
    pub trusted_device_days: ::core::ffi::c_int,
    pub trusted_proxy_cidrs: [::core::ffi::c_char; 1024],
    pub demo_mode: bool,
    pub force_mfa_on_login: bool,
    pub login_rate_limit_enabled: bool,
    pub login_rate_limit_max_attempts: ::core::ffi::c_int,
    pub login_rate_limit_window_seconds: ::core::ffi::c_int,
    pub web_compression_enabled: bool,
    pub web_use_minified_assets: bool,
    pub web_cache_max_age_html: ::core::ffi::c_int,
    pub web_cache_max_age_css: ::core::ffi::c_int,
    pub web_cache_max_age_js: ::core::ffi::c_int,
    pub web_cache_max_age_images: ::core::ffi::c_int,
    pub web_cache_max_age_fonts: ::core::ffi::c_int,
    pub web_cache_max_age_default: ::core::ffi::c_int,
    pub onvif_discovery_enabled: bool,
    pub onvif_discovery_interval: ::core::ffi::c_int,
    pub onvif_discovery_network: [::core::ffi::c_char; 64],
    pub max_streams: ::core::ffi::c_int,
    pub streams: *mut stream_config_t,
    pub buffer_size: ::core::ffi::c_int,
    pub use_swap: bool,
    pub swap_file: [::core::ffi::c_char; 512],
    pub swap_size: uint64_t,
    pub memory_constrained: bool,
    pub hw_accel_enabled: bool,
    pub hw_accel_device: [::core::ffi::c_char; 32],
    pub go2rtc_enabled: bool,
    pub go2rtc_binary_path: [::core::ffi::c_char; 512],
    pub go2rtc_config_dir: [::core::ffi::c_char; 512],
    pub go2rtc_api_port: ::core::ffi::c_int,
    pub go2rtc_rtsp_port: ::core::ffi::c_int,
    pub go2rtc_force_native_hls: bool,
    pub go2rtc_proxy_max_inflight: ::core::ffi::c_int,
    pub go2rtc_webrtc_enabled: bool,
    pub go2rtc_webrtc_listen_port: ::core::ffi::c_int,
    pub go2rtc_stun_enabled: bool,
    pub go2rtc_stun_server: [::core::ffi::c_char; 256],
    pub go2rtc_external_ip: [::core::ffi::c_char; 64],
    pub go2rtc_ice_servers: [::core::ffi::c_char; 512],
    pub webrtc_connection_timeout_ms: ::core::ffi::c_int,
    pub webrtc_ice_recovery_timeout_ms: ::core::ffi::c_int,
    pub turn_enabled: bool,
    pub turn_server_url: [::core::ffi::c_char; 256],
    pub turn_username: [::core::ffi::c_char; 64],
    pub turn_password: [::core::ffi::c_char; 64],
    pub mqtt_enabled: bool,
    pub mqtt_broker_host: [::core::ffi::c_char; 256],
    pub mqtt_broker_port: ::core::ffi::c_int,
    pub mqtt_username: [::core::ffi::c_char; 64],
    pub mqtt_password: [::core::ffi::c_char; 128],
    pub mqtt_client_id: [::core::ffi::c_char; 64],
    pub mqtt_topic_prefix: [::core::ffi::c_char; 128],
    pub mqtt_tls_enabled: bool,
    pub mqtt_keepalive: ::core::ffi::c_int,
    pub mqtt_qos: ::core::ffi::c_int,
    pub mqtt_retain: bool,
    pub mqtt_ha_discovery: bool,
    pub mqtt_ha_discovery_prefix: [::core::ffi::c_char; 128],
    pub mqtt_ha_snapshot_interval: ::core::ffi::c_int,
    pub health: system_health_policy_settings_t,
    pub detection_engine: detection_engine_config_t,
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
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_DETECTED_NETWORKS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MAX_DISCOVERED_DEVICES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut g_discovered_devices: [onvif_device_info_t; 32] = [onvif_device_info_t {
    inventory_uuid: [0; 37],
    endpoint: [0; 512],
    device_service: [0; 512],
    media_service: [0; 512],
    ptz_service: [0; 512],
    imaging_service: [0; 512],
    manufacturer: [0; 64],
    model: [0; 64],
    firmware_version: [0; 32],
    serial_number: [0; 64],
    hardware_id: [0; 64],
    ip_address: [0; 64],
    mac_address: [0; 32],
    discovery_time: 0,
    first_seen_at: 0,
    last_seen_at: 0,
    last_scan_network: [0; 64],
    claim_state: [0; 16],
    claimed_camera_uuid: [0; 37],
    duplicate_suspected: false,
    online: false,
}; 32];
static mut g_discovered_device_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut g_discovery_mutex: pthread_mutex_t = pthread_mutex_t {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_onvif_discovery() -> ::core::ffi::c_int {
    srand(
        time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_uint
            ^ (getpid() as ::core::ffi::c_uint) << 16 as ::core::ffi::c_int,
    );
    memset(
        &raw mut g_discovered_devices as *mut onvif_device_info_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[onvif_device_info_t; 32]>(),
    );
    g_discovered_device_count = db_onvif_inventory_list(
        &raw mut g_discovered_devices as *mut onvif_device_info_t,
        MAX_DISCOVERED_DEVICES,
    );
    if g_discovered_device_count < 0 as ::core::ffi::c_int {
        g_discovered_device_count = 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"ONVIF discovery module initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_onvif_discovery() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"ONVIF discovery module shutdown\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_discovery_mutex() -> *mut pthread_mutex_t {
    return &raw mut g_discovery_mutex;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_current_discovery_network() -> *const ::core::ffi::c_char {
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_onvif_discovery(
    mut network: *const ::core::ffi::c_char,
    mut interval: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"start_onvif_discovery is deprecated, use discover_onvif_devices directly\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_onvif_discovery() -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"stop_onvif_discovery is deprecated, use discover_onvif_devices directly\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_discovered_onvif_devices(
    mut devices: *mut onvif_device_info_t,
    mut max_devices: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0;
    if devices.is_null() || max_devices <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut persisted: [onvif_device_info_t; 32] = [onvif_device_info_t {
        inventory_uuid: [0; 37],
        endpoint: [0; 512],
        device_service: [0; 512],
        media_service: [0; 512],
        ptz_service: [0; 512],
        imaging_service: [0; 512],
        manufacturer: [0; 64],
        model: [0; 64],
        firmware_version: [0; 32],
        serial_number: [0; 64],
        hardware_id: [0; 64],
        ip_address: [0; 64],
        mac_address: [0; 32],
        discovery_time: 0,
        first_seen_at: 0,
        last_seen_at: 0,
        last_scan_network: [0; 64],
        claim_state: [0; 16],
        claimed_camera_uuid: [0; 37],
        duplicate_suspected: false,
        online: false,
    }; 32];
    let mut persisted_count: ::core::ffi::c_int = db_onvif_inventory_list(
        &raw mut persisted as *mut onvif_device_info_t,
        MAX_DISCOVERED_DEVICES,
    );
    pthread_mutex_lock(&raw mut g_discovery_mutex);
    if persisted_count >= 0 as ::core::ffi::c_int {
        memcpy(
            &raw mut g_discovered_devices as *mut onvif_device_info_t as *mut ::core::ffi::c_void,
            &raw mut persisted as *mut onvif_device_info_t as *const ::core::ffi::c_void,
            (persisted_count as size_t).wrapping_mul(::core::mem::size_of::<onvif_device_info_t>()),
        );
        g_discovered_device_count = persisted_count;
    }
    count = if g_discovered_device_count < max_devices {
        g_discovered_device_count
    } else {
        max_devices
    };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        memcpy(
            devices.offset(i as isize) as *mut ::core::ffi::c_void,
            (&raw mut g_discovered_devices as *mut onvif_device_info_t).offset(i as isize)
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<onvif_device_info_t>(),
        );
        i += 1;
    }
    pthread_mutex_unlock(&raw mut g_discovery_mutex);
    return count;
}
unsafe extern "C" fn is_port_open(
    mut ip_addr: *const ::core::ffi::c_char,
    mut port: ::core::ffi::c_int,
    mut timeout_ms: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sock: ::core::ffi::c_int = 0;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut fdset: fd_set = fd_set { fds_bits: [0; 16] };
    let mut res: ::core::ffi::c_int = 0;
    sock = socket(
        AF_INET,
        __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if sock < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut arg: ::core::ffi::c_long = fcntl(sock, F_GETFL, NULL) as ::core::ffi::c_long;
    arg |= O_NONBLOCK as ::core::ffi::c_long;
    fcntl(sock, F_SETFL, arg);
    memset(
        &raw mut addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_in>(),
    );
    addr.sin_family = AF_INET as sa_family_t;
    addr.sin_port = __bswap_16(port as __uint16_t) as in_port_t;
    if inet_aton(ip_addr, &raw mut addr.sin_addr) == 0 as ::core::ffi::c_int {
        close(sock);
        return 0 as ::core::ffi::c_int;
    }
    res = connect(
        sock,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut addr as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_in>() as socklen_t,
    );
    if res < 0 as ::core::ffi::c_int {
        if *__errno_location() == EINPROGRESS {
            tv.tv_sec = (timeout_ms / 1000 as ::core::ffi::c_int) as __time_t;
            tv.tv_usec = ((timeout_ms % 1000 as ::core::ffi::c_int) as ::core::ffi::c_long
                * 1000 as ::core::ffi::c_long) as __suseconds_t;
            let mut __i: ::core::ffi::c_uint = 0;
            let mut __arr: *mut fd_set = &raw mut fdset;
            __i = 0 as ::core::ffi::c_uint;
            while (__i as usize)
                < ::core::mem::size_of::<fd_set>().wrapping_div(::core::mem::size_of::<__fd_mask>())
            {
                (*__arr).fds_bits[__i as usize] = 0 as __fd_mask;
                __i = __i.wrapping_add(1);
            }
            fdset.fds_bits[(sock / __NFDBITS) as usize] |=
                ((1 as ::core::ffi::c_ulong) << sock % __NFDBITS) as __fd_mask;
            res = select(
                sock + 1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<fd_set>(),
                &raw mut fdset,
                ::core::ptr::null_mut::<fd_set>(),
                &raw mut tv,
            );
            if res > 0 as ::core::ffi::c_int {
                let mut so_error: ::core::ffi::c_int = 0;
                let mut len: socklen_t = ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t;
                getsockopt(
                    sock,
                    SOL_SOCKET,
                    SO_ERROR,
                    &raw mut so_error as *mut ::core::ffi::c_void,
                    &raw mut len,
                );
                if so_error == 0 as ::core::ffi::c_int {
                    close(sock);
                    return 1 as ::core::ffi::c_int;
                }
            }
        }
    } else {
        close(sock);
        return 1 as ::core::ffi::c_int;
    }
    close(sock);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn discover_onvif_devices(
    mut network: *const ::core::ffi::c_char,
    mut devices: *mut onvif_device_info_t,
    mut max_devices: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut base_addr: uint32_t = 0;
    let mut subnet_mask: uint32_t = 0;
    let mut ip_addr: [::core::ffi::c_char; 16] = [0; 16];
    let mut addr: in_addr = in_addr { s_addr: 0 };
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut selected_network: [::core::ffi::c_char; 64] = [0; 64];
    let mut discovery_sock: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut broadcast_enabled: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if devices.is_null() || max_devices <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for discover_onvif_devices\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if network.is_null()
        || strlen(network) == 0 as size_t
        || strcmp(network, b"auto\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        let mut env_network: *const ::core::ffi::c_char =
            getenv(b"LIGHTNVR_ONVIF_NETWORK\0".as_ptr() as *const ::core::ffi::c_char);
        if !env_network.is_null()
            && strlen(env_network) > 0 as size_t
            && strcmp(
                env_network,
                b"auto\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Using ONVIF discovery network from environment variable: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                env_network,
            );
            safe_strcpy(
                &raw mut selected_network as *mut ::core::ffi::c_char,
                env_network,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            network = &raw mut selected_network as *mut ::core::ffi::c_char;
        } else if g_config.onvif_discovery_network[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
            && strcmp(
                &raw mut g_config.onvif_discovery_network as *mut ::core::ffi::c_char,
                b"auto\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Using ONVIF discovery network from config file: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut g_config.onvif_discovery_network as *mut ::core::ffi::c_char,
            );
            safe_strcpy(
                &raw mut selected_network as *mut ::core::ffi::c_char,
                &raw mut g_config.onvif_discovery_network as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            network = &raw mut selected_network as *mut ::core::ffi::c_char;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Auto-detecting networks for ONVIF discovery\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            let mut detected_networks: [[::core::ffi::c_char; 64]; 10] = [[0; 64]; 10];
            let mut network_count: ::core::ffi::c_int = detect_local_networks(
                &raw mut detected_networks as *mut [::core::ffi::c_char; 64],
                MAX_DETECTED_NETWORKS,
            );
            if network_count <= 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to auto-detect networks for ONVIF discovery\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            safe_strcpy(
                &raw mut selected_network as *mut ::core::ffi::c_char,
                &raw mut *(&raw mut detected_networks as *mut [::core::ffi::c_char; 64])
                    .offset(0isize) as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Auto-detected network for ONVIF discovery: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut selected_network as *mut ::core::ffi::c_char,
            );
            network = &raw mut selected_network as *mut ::core::ffi::c_char;
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting ONVIF discovery on network %s\0".as_ptr() as *const ::core::ffi::c_char,
        network,
    );
    if parse_network(network, &raw mut base_addr, &raw mut subnet_mask) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to parse network: %s\0".as_ptr() as *const ::core::ffi::c_char,
            network,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut network_addr: uint32_t = base_addr & subnet_mask;
    let mut broadcast: uint32_t = network_addr | !subnet_mask;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Scanning network for open ONVIF ports (3702 and 80)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut candidate_ips: [[::core::ffi::c_char; 16]; 256] = [[0; 16]; 256];
    let mut candidate_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ip: uint32_t = network_addr.wrapping_add(1 as uint32_t);
    while ip < broadcast && candidate_count < MAX_CANDIDATE_IPS {
        if !(ip == network_addr.wrapping_add(1 as uint32_t)
            || ip == broadcast.wrapping_sub(1 as uint32_t))
        {
            addr.s_addr = __bswap_32(ip) as in_addr_t;
            safe_strcpy(
                &raw mut ip_addr as *mut ::core::ffi::c_char,
                inet_ntoa(addr),
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            if is_port_open(
                &raw mut ip_addr as *mut ::core::ffi::c_char,
                3702 as ::core::ffi::c_int,
                25 as ::core::ffi::c_int,
            ) != 0
                || is_port_open(
                    &raw mut ip_addr as *mut ::core::ffi::c_char,
                    80 as ::core::ffi::c_int,
                    25 as ::core::ffi::c_int,
                ) != 0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Found potential ONVIF device at %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut ip_addr as *mut ::core::ffi::c_char,
                );
                safe_strcpy(
                    &raw mut *(&raw mut candidate_ips as *mut [::core::ffi::c_char; 16])
                        .offset(candidate_count as isize)
                        as *mut ::core::ffi::c_char,
                    &raw mut ip_addr as *mut ::core::ffi::c_char,
                    16 as size_t,
                    0 as size_t,
                );
                candidate_count += 1;
            }
        }
        ip = ip.wrapping_add(1);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Found %d potential ONVIF devices with open ports\0".as_ptr()
            as *const ::core::ffi::c_char,
        candidate_count,
    );
    if candidate_count == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"No devices with open ports found, trying broadcast and multicast\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        discovery_sock = socket(
            AF_INET,
            __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if discovery_sock < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to create discovery socket: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
        if setsockopt(
            discovery_sock,
            SOL_SOCKET,
            SO_BROADCAST,
            &raw mut broadcast_enabled as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to set SO_BROADCAST option: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            close(discovery_sock);
            return -1 as ::core::ffi::c_int;
        }
        let mut bind_addr: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        memset(
            &raw mut bind_addr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_in>(),
        );
        bind_addr.sin_family = AF_INET as sa_family_t;
        bind_addr.sin_addr.s_addr = __bswap_32(0 as ::core::ffi::c_int as __uint32_t) as in_addr_t;
        bind_addr.sin_port = __bswap_16(0 as __uint16_t) as in_port_t;
        if bind(
            discovery_sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut bind_addr as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to bind discovery socket: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        let mut dest_addr: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        memset(
            &raw mut dest_addr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_in>(),
        );
        dest_addr.sin_family = AF_INET as sa_family_t;
        dest_addr.sin_port = __bswap_16(3702 as __uint16_t) as in_port_t;
        addr.s_addr = __bswap_32(broadcast) as in_addr_t;
        safe_strcpy(
            &raw mut ip_addr as *mut ::core::ffi::c_char,
            inet_ntoa(addr),
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sending discovery probes to broadcast address: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut ip_addr as *mut ::core::ffi::c_char,
        );
        dest_addr.sin_addr.s_addr = __bswap_32(broadcast) as in_addr_t;
        send_all_discovery_probes(
            discovery_sock,
            &raw mut ip_addr as *mut ::core::ffi::c_char,
            &raw mut dest_addr,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sending discovery probes to ONVIF multicast address: 239.255.255.250\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        inet_pton(
            AF_INET,
            b"239.255.255.250\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut dest_addr.sin_addr as *mut ::core::ffi::c_void,
        );
        send_all_discovery_probes(
            discovery_sock,
            b"239.255.255.250\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut dest_addr,
        );
        close(discovery_sock);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Waiting for discovery responses...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        count = receive_discovery_responses(devices, max_devices);
        if count == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"No devices found with standard timeout, trying with shorter timeout\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            count = receive_extended_discovery_responses(
                devices,
                max_devices,
                1 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
            );
        }
    } else {
        discovery_sock = socket(
            AF_INET,
            __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if discovery_sock < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to create discovery socket: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
        if setsockopt(
            discovery_sock,
            SOL_SOCKET,
            SO_BROADCAST,
            &raw mut broadcast_enabled as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to set SO_BROADCAST option: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            close(discovery_sock);
            return -1 as ::core::ffi::c_int;
        }
        let mut bind_addr_0: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        memset(
            &raw mut bind_addr_0 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_in>(),
        );
        bind_addr_0.sin_family = AF_INET as sa_family_t;
        bind_addr_0.sin_addr.s_addr =
            __bswap_32(0 as ::core::ffi::c_int as __uint32_t) as in_addr_t;
        bind_addr_0.sin_port = __bswap_16(0 as __uint16_t) as in_port_t;
        if bind(
            discovery_sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut bind_addr_0 as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to bind discovery socket: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        let mut dest_addr_0: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        memset(
            &raw mut dest_addr_0 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_in>(),
        );
        dest_addr_0.sin_family = AF_INET as sa_family_t;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Sending discovery probes to %d candidate IPs\0".as_ptr()
                as *const ::core::ffi::c_char,
            candidate_count,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < candidate_count {
            dest_addr_0.sin_port = __bswap_16(3702 as __uint16_t) as in_port_t;
            if inet_aton(
                &raw mut *(&raw mut candidate_ips as *mut [::core::ffi::c_char; 16])
                    .offset(i as isize) as *mut ::core::ffi::c_char,
                &raw mut dest_addr_0.sin_addr,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Sending discovery probe to %s:3702\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut *(&raw mut candidate_ips as *mut [::core::ffi::c_char; 16])
                        .offset(i as isize) as *mut ::core::ffi::c_char,
                );
                send_all_discovery_probes(
                    discovery_sock,
                    &raw mut *(&raw mut candidate_ips as *mut [::core::ffi::c_char; 16])
                        .offset(i as isize) as *mut ::core::ffi::c_char,
                    &raw mut dest_addr_0,
                );
            }
            dest_addr_0.sin_port = __bswap_16(80 as __uint16_t) as in_port_t;
            if inet_aton(
                &raw mut *(&raw mut candidate_ips as *mut [::core::ffi::c_char; 16])
                    .offset(i as isize) as *mut ::core::ffi::c_char,
                &raw mut dest_addr_0.sin_addr,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Sending discovery probe to %s:80\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut *(&raw mut candidate_ips as *mut [::core::ffi::c_char; 16])
                        .offset(i as isize) as *mut ::core::ffi::c_char,
                );
                send_all_discovery_probes(
                    discovery_sock,
                    &raw mut *(&raw mut candidate_ips as *mut [::core::ffi::c_char; 16])
                        .offset(i as isize) as *mut ::core::ffi::c_char,
                    &raw mut dest_addr_0,
                );
            }
            i += 1;
        }
        close(discovery_sock);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Waiting for discovery responses...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        count = receive_discovery_responses(devices, max_devices);
        if count == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"No devices found with standard timeout, trying with shorter timeout\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            count = receive_extended_discovery_responses(
                devices,
                max_devices,
                1 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
            );
        }
    }
    if count == 0 as ::core::ffi::c_int && candidate_count > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"No devices found with WS-Discovery, trying direct HTTP probing\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        count = try_direct_http_discovery(
            &raw mut candidate_ips as *mut [::core::ffi::c_char; 16],
            candidate_count,
            devices,
            max_devices,
        );
    }
    if count < 0 as ::core::ffi::c_int {
        count = 0 as ::core::ffi::c_int;
    }
    let mut observed_at: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    if db_onvif_inventory_record_scan(network, devices, count, observed_at)
        < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to persist ONVIF discovery inventory\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut persisted: [onvif_device_info_t; 32] = [onvif_device_info_t {
        inventory_uuid: [0; 37],
        endpoint: [0; 512],
        device_service: [0; 512],
        media_service: [0; 512],
        ptz_service: [0; 512],
        imaging_service: [0; 512],
        manufacturer: [0; 64],
        model: [0; 64],
        firmware_version: [0; 32],
        serial_number: [0; 64],
        hardware_id: [0; 64],
        ip_address: [0; 64],
        mac_address: [0; 32],
        discovery_time: 0,
        first_seen_at: 0,
        last_seen_at: 0,
        last_scan_network: [0; 64],
        claim_state: [0; 16],
        claimed_camera_uuid: [0; 37],
        duplicate_suspected: false,
        online: false,
    }; 32];
    let mut persisted_count: ::core::ffi::c_int = db_onvif_inventory_list(
        &raw mut persisted as *mut onvif_device_info_t,
        MAX_DISCOVERED_DEVICES,
    );
    pthread_mutex_lock(&raw mut g_discovery_mutex);
    g_discovered_device_count = 0 as ::core::ffi::c_int;
    if persisted_count >= 0 as ::core::ffi::c_int {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < persisted_count {
            g_discovered_devices[i_0 as usize] = persisted[i_0 as usize];
            g_discovered_device_count += 1;
            i_0 += 1;
        }
    } else {
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < count && i_1 < MAX_DISCOVERED_DEVICES {
            g_discovered_devices[i_1 as usize] = *devices.offset(i_1 as isize);
            g_discovered_device_count += 1;
            i_1 += 1;
        }
    }
    let mut return_count: ::core::ffi::c_int = if g_discovered_device_count < max_devices {
        g_discovered_device_count
    } else {
        max_devices
    };
    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_2 < return_count {
        *devices.offset(i_2 as isize) = g_discovered_devices[i_2 as usize];
        i_2 += 1;
    }
    pthread_mutex_unlock(&raw mut g_discovery_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"ONVIF discovery completed, observed %d devices; %d staged\0".as_ptr()
            as *const ::core::ffi::c_char,
        count,
        return_count,
    );
    return return_count;
}
pub const MAX_CANDIDATE_IPS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn try_direct_http_discovery(
    mut candidate_ips: *mut [::core::ffi::c_char; 16],
    mut candidate_count: ::core::ffi::c_int,
    mut devices: *mut onvif_device_info_t,
    mut max_devices: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut res: CURLcode = CURLcode::CURLE_OK;
    if curl_init_global() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize curl global for direct HTTP discovery\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL for direct HTTP discovery\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut soap_request: *const ::core::ffi::c_char = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\">  <s:Body xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">    <GetSystemDateAndTime xmlns=\"http://www.onvif.org/ver10/device/wsdl\"/>  </s:Body></s:Envelope>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut onvif_paths: [*const ::core::ffi::c_char; 9] = [
        b"/onvif/device_service\0".as_ptr() as *const ::core::ffi::c_char,
        b"/onvif/services\0".as_ptr() as *const ::core::ffi::c_char,
        b"/onvif/service\0".as_ptr() as *const ::core::ffi::c_char,
        b"/onvif/devices\0".as_ptr() as *const ::core::ffi::c_char,
        b"/onvif/device\0".as_ptr() as *const ::core::ffi::c_char,
        b"/device_service\0".as_ptr() as *const ::core::ffi::c_char,
        b"/services\0".as_ptr() as *const ::core::ffi::c_char,
        b"/service\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    curl_easy_setopt(curl, CURLoption::CURLOPT_TIMEOUT, 2 as ::core::ffi::c_long);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CONNECTTIMEOUT,
        1 as ::core::ffi::c_long,
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_NOSIGNAL, 1 as ::core::ffi::c_long);
    curl_easy_setopt(curl, CURLoption::CURLOPT_POST, 1 as ::core::ffi::c_long);
    curl_easy_setopt(curl, CURLoption::CURLOPT_POSTFIELDS, soap_request);
    curl_easy_setopt(curl, CURLoption::CURLOPT_NOBODY, 0 as ::core::ffi::c_long);
    let mut headers: *mut curl_slist = ::core::ptr::null_mut::<curl_slist>();
    headers = curl_slist_append(
        headers,
        b"Content-Type: application/soap+xml; charset=utf-8\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLoption::CURLOPT_VERBOSE, 0 as ::core::ffi::c_long);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            onvif_curl_write_callback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting direct HTTP/HTTPS probing for %d candidate IPs\0".as_ptr()
            as *const ::core::ffi::c_char,
        candidate_count,
    );
    let mut schemes: [*const ::core::ffi::c_char; 3] = [
        b"http\0".as_ptr() as *const ::core::ffi::c_char,
        b"https\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < candidate_count && count < max_devices {
        let mut ip: *const ::core::ffi::c_char =
            &raw mut *candidate_ips.offset(i as isize) as *mut ::core::ffi::c_char;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Probing IP %s for ONVIF services\0".as_ptr() as *const ::core::ffi::c_char,
            ip,
        );
        let mut device_found: bool = r#false != 0;
        let mut s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !schemes[s as usize].is_null() && !device_found && count < max_devices {
            let mut scheme: *const ::core::ffi::c_char = schemes[s as usize];
            let mut is_https: bool = *schemes[s as usize].offset(4isize) as ::core::ffi::c_int
                == 's' as ::core::ffi::c_int;
            curl_easy_setopt(
                curl,
                CURLoption::CURLOPT_SSL_VERIFYPEER,
                if is_https as ::core::ffi::c_int != 0 {
                    0 as ::core::ffi::c_long
                } else {
                    1 as ::core::ffi::c_long
                },
            );
            curl_easy_setopt(
                curl,
                CURLoption::CURLOPT_SSL_VERIFYHOST,
                if is_https as ::core::ffi::c_int != 0 {
                    0 as ::core::ffi::c_long
                } else {
                    2 as ::core::ffi::c_long
                },
            );
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while !onvif_paths[j as usize].is_null() && !device_found && count < max_devices {
                let mut url: [::core::ffi::c_char; 256] = [0; 256];
                snprintf(
                    &raw mut url as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    b"%s://%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                    scheme,
                    ip,
                    onvif_paths[j as usize],
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Trying URL: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut url as *mut ::core::ffi::c_char,
                );
                curl_easy_setopt(
                    curl,
                    CURLoption::CURLOPT_URL,
                    &raw mut url as *mut ::core::ffi::c_char,
                );
                res = curl_easy_perform(curl);
                if res.0 == CURLcode::CURLE_OK.0 {
                    let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                    curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
                    if http_code >= 200 as ::core::ffi::c_long
                        && http_code < 300 as ::core::ffi::c_long
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Found ONVIF device at %s\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut url as *mut ::core::ffi::c_char,
                        );
                        memset(
                            devices.offset(count as isize) as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<onvif_device_info_t>(),
                        );
                        safe_strcpy(
                            &raw mut (*devices.offset(count as isize)).ip_address
                                as *mut ::core::ffi::c_char,
                            ip,
                            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                            0 as size_t,
                        );
                        safe_strcpy(
                            &raw mut (*devices.offset(count as isize)).device_service
                                as *mut ::core::ffi::c_char,
                            &raw mut url as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            0 as size_t,
                        );
                        safe_strcpy(
                            &raw mut (*devices.offset(count as isize)).endpoint
                                as *mut ::core::ffi::c_char,
                            &raw mut url as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            0 as size_t,
                        );
                        snprintf(
                            &raw mut (*devices.offset(count as isize)).model
                                as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                            b"Unknown (%s discovery)\0".as_ptr() as *const ::core::ffi::c_char,
                            if is_https as ::core::ffi::c_int != 0 {
                                b"HTTPS\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"HTTP\0".as_ptr() as *const ::core::ffi::c_char
                            },
                        );
                        (*devices.offset(count as isize)).discovery_time =
                            time(::core::ptr::null_mut::<time_t>());
                        (*devices.offset(count as isize)).online = r#true != 0;
                        count += 1;
                        device_found = r#true != 0;
                    }
                }
                j += 1;
            }
            s += 1;
        }
        i += 1;
    }
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Direct HTTP/HTTPS probing completed, found %d devices\0".as_ptr()
            as *const ::core::ffi::c_char,
        count,
    );
    return count;
}
unsafe extern "C" fn onvif_curl_write_callback(
    mut contents: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> size_t {
    return size.wrapping_mul(nmemb);
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
pub const SO_ERROR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SO_BROADCAST: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EINPROGRESS: ::core::ffi::c_int = 115 as ::core::ffi::c_int;
