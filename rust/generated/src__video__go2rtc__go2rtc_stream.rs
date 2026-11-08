use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fgetc(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
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
    unsafe fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn inet_pton(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn curl_easy_strerror(error: CURLcode) -> *const ::core::ffi::c_char;
    unsafe fn curl_easy_init() -> *mut ::core::ffi::c_void;
    unsafe fn curl_easy_setopt(curl: *mut ::core::ffi::c_void, option: CURLoption, ...)
    -> CURLcode;
    unsafe fn curl_easy_perform(curl: *mut ::core::ffi::c_void) -> CURLcode;
    unsafe fn curl_easy_cleanup(curl: *mut ::core::ffi::c_void);
    unsafe fn curl_easy_getinfo(curl: *mut ::core::ffi::c_void, info: CURLINFO, ...) -> CURLcode;
    unsafe fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...)
    -> ::core::ffi::c_int;
    unsafe fn getaddrinfo(
        __name: *const ::core::ffi::c_char,
        __service: *const ::core::ffi::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
    unsafe fn freeaddrinfo(__ai: *mut addrinfo);
    unsafe fn gai_strerror(__ecode: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    static mut g_config: config_t;
    unsafe fn go2rtc_process_init(
        binary_path: *const ::core::ffi::c_char,
        config_dir: *const ::core::ffi::c_char,
        api_port: ::core::ffi::c_int,
    ) -> bool;
    unsafe fn go2rtc_process_start(api_port: ::core::ffi::c_int) -> bool;
    unsafe fn go2rtc_process_stop() -> bool;
    unsafe fn go2rtc_process_is_running() -> bool;
    unsafe fn go2rtc_process_cleanup();
    unsafe fn go2rtc_process_get_rtsp_port() -> ::core::ffi::c_int;
    unsafe fn go2rtc_api_get_base_path() -> *const ::core::ffi::c_char;
    unsafe fn go2rtc_api_init(
        api_host: *const ::core::ffi::c_char,
        api_port: ::core::ffi::c_int,
    ) -> bool;
    unsafe fn go2rtc_api_add_stream(
        stream_id: *const ::core::ffi::c_char,
        stream_url: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn go2rtc_api_add_stream_multi(
        stream_id: *const ::core::ffi::c_char,
        sources: *mut *const ::core::ffi::c_char,
        num_sources: ::core::ffi::c_int,
    ) -> bool;
    unsafe fn go2rtc_api_remove_stream(stream_id: *const ::core::ffi::c_char) -> bool;
    unsafe fn go2rtc_api_delete_preload(stream_id: *const ::core::ffi::c_char) -> bool;
    unsafe fn go2rtc_api_get_webrtc_url(
        stream_id: *const ::core::ffi::c_char,
        buffer: *mut ::core::ffi::c_char,
        buffer_size: size_t,
    ) -> bool;
    unsafe fn go2rtc_api_get_server_info(rtsp_port: *mut ::core::ffi::c_int) -> bool;
    unsafe fn go2rtc_api_publish_stream_async(
        stream_id: *const ::core::ffi::c_char,
        destination: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn go2rtc_api_cleanup();
    unsafe fn go2rtc_integration_register_all_streams() -> bool;
    unsafe fn go2rtc_integration_is_initialized() -> bool;
    unsafe fn go2rtc_lifecycle_begin(
        operation: go2rtc_lifecycle_operation_t,
        coalesce: bool,
        intentional: bool,
        guard: *mut go2rtc_lifecycle_guard_t,
    ) -> bool;
    unsafe fn go2rtc_lifecycle_end(guard: *mut go2rtc_lifecycle_guard_t, result: bool);
    unsafe fn cleanup_dns_resolver();
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn url_apply_credentials(
        url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn url_redact_for_logging(
        url: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn simple_url_escape(
        input: *const ::core::ffi::c_char,
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    );
    unsafe fn publish_url_next(
        cursor: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
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
    unsafe fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn select(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::core::ffi::c_int;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = isize;
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub type CURL = ();
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
    pub go2rtc_external_ip: [::core::ffi::c_char; 256],
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
pub struct go2rtc_lifecycle_guard_t {
    pub acquired: bool,
    pub outermost: bool,
    pub coalesced: bool,
    pub result: bool,
    pub generation: uint64_t,
    pub restart_generation: uint64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct go2rtc_lifecycle_operation_t(pub ::core::ffi::c_uint);
impl go2rtc_lifecycle_operation_t {
    pub const GO2RTC_LIFECYCLE_CHECK: Self = Self(0);
    pub const GO2RTC_LIFECYCLE_PROCESS_START: Self = Self(1);
    pub const GO2RTC_LIFECYCLE_PROCESS_STOP: Self = Self(2);
    pub const GO2RTC_LIFECYCLE_FULL_START: Self = Self(3);
    pub const GO2RTC_LIFECYCLE_RESTART: Self = Self(4);
    pub const GO2RTC_LIFECYCLE_RECONFIGURE: Self = Self(5);
    pub const GO2RTC_LIFECYCLE_CLEANUP: Self = Self(6);
    pub const GO2RTC_LIFECYCLE_OPERATION_COUNT: Self = Self(7);
}
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GO2RTC_PREVIEW_MAX_WIDTH: ::core::ffi::c_int = 1920 as ::core::ffi::c_int;
pub const GO2RTC_PREVIEW_MAX_HEIGHT: ::core::ffi::c_int = 1080 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const DEFAULT_API_HOST: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"localhost\0") };
pub const URL_BUFFER_SIZE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
static mut g_initialized: bool = r#false != 0;
static mut g_api_port: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_config_dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut g_ready_cache_valid: bool = r#false != 0;
static mut g_ready_cache_value: bool = r#false != 0;
static mut g_ready_cache_time: time_t = 0 as time_t;
pub const READY_CACHE_TTL_SEC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_init(
    mut binary_path: *const ::core::ffi::c_char,
    mut config_dir: *const ::core::ffi::c_char,
    mut api_port: ::core::ffi::c_int,
) -> bool {
    if g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc stream integration already initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if config_dir.is_null() || api_port <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_stream_init\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    g_config_dir = strdup(config_dir);
    if g_config_dir.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for config directory\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !go2rtc_process_init(binary_path, config_dir, api_port) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize go2rtc process manager\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(g_config_dir as *mut ::core::ffi::c_void);
        g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return r#false != 0;
    }
    if !go2rtc_api_init(DEFAULT_API_HOST.as_ptr(), api_port) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize go2rtc API client\0".as_ptr() as *const ::core::ffi::c_char,
        );
        go2rtc_process_cleanup();
        free(g_config_dir as *mut ::core::ffi::c_void);
        g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return r#false != 0;
    }
    g_api_port = api_port;
    g_initialized = r#true != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc stream integration initialized with config dir: %s, API port: %d\0".as_ptr()
            as *const ::core::ffi::c_char,
        config_dir,
        api_port,
    );
    return r#true != 0;
}
unsafe extern "C" fn go2rtc_rtsp_fragment_key_supported(
    mut key: *const ::core::ffi::c_char,
    mut len: size_t,
) -> bool {
    static mut known: [*const ::core::ffi::c_char; 9] = [
        b"transport\0".as_ptr() as *const ::core::ffi::c_char,
        b"timeout\0".as_ptr() as *const ::core::ffi::c_char,
        b"backchannel\0".as_ptr() as *const ::core::ffi::c_char,
        b"media\0".as_ptr() as *const ::core::ffi::c_char,
        b"pkt_size\0".as_ptr() as *const ::core::ffi::c_char,
        b"log_level\0".as_ptr() as *const ::core::ffi::c_char,
        b"source\0".as_ptr() as *const ::core::ffi::c_char,
        b"mp4\0".as_ptr() as *const ::core::ffi::c_char,
        b"weak_timeout\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 9]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if strlen(known[i]) == len && strncmp(known[i], key, len) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn go2rtc_sanitize_rtsp_fragments(
    mut url: *mut ::core::ffi::c_char,
    mut url_size: size_t,
    mut stream_id: *const ::core::ffi::c_char,
    mut force_video_only: bool,
) -> bool {
    let mut hash: *mut ::core::ffi::c_char = strchr(url, '#' as ::core::ffi::c_int);
    if hash.is_null() {
        return r#false != 0;
    }
    let mut suppress_audio: bool = r#false != 0;
    let mut rebuilt: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut base_len: size_t = hash.offset_from(url) as size_t;
    if base_len >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() {
        return r#false != 0;
    }
    memcpy(
        &raw mut rebuilt as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        url as *const ::core::ffi::c_void,
        base_len,
    );
    rebuilt[base_len] = '\0' as ::core::ffi::c_char;
    let mut out: size_t = base_len;
    let mut p: *const ::core::ffi::c_char = hash.offset(1 as ::core::ffi::c_int as isize);
    while *p != 0 {
        let mut tok: *const ::core::ffi::c_char = p;
        let mut end: *const ::core::ffi::c_char = strchr(p, '#' as ::core::ffi::c_int);
        let mut tok_len: size_t = if !end.is_null() {
            end.offset_from(tok) as size_t
        } else {
            strlen(tok)
        };
        let mut eq: *const ::core::ffi::c_char = memchr(
            tok as *const ::core::ffi::c_void,
            '=' as ::core::ffi::c_int,
            tok_len,
        ) as *const ::core::ffi::c_char;
        let mut key_len: size_t = if !eq.is_null() {
            eq.offset_from(tok) as size_t
        } else {
            tok_len
        };
        if key_len == 7 as size_t
            && strncasecmp(
                tok,
                b"noaudio\0".as_ptr() as *const ::core::ffi::c_char,
                7 as size_t,
            ) == 0 as ::core::ffi::c_int
            || key_len == 8 as size_t
                && strncasecmp(
                    tok,
                    b"no-audio\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
        {
            suppress_audio = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s: '#noaudio' in source URL -> suppressing go2rtc audio producer\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_id,
            );
        } else if !(force_video_only as ::core::ffi::c_int != 0
            && key_len == 5 as size_t
            && strncasecmp(
                tok,
                b"media\0".as_ptr() as *const ::core::ffi::c_char,
                5 as size_t,
            ) == 0 as ::core::ffi::c_int)
        {
            if go2rtc_rtsp_fragment_key_supported(tok, key_len) {
                if out.wrapping_add(tok_len).wrapping_add(1 as size_t) < url_size
                    && out.wrapping_add(tok_len).wrapping_add(1 as size_t)
                        < ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()
                {
                    let c2rust_fresh5 = out;
                    out = out.wrapping_add(1);
                    rebuilt[c2rust_fresh5] = '#' as ::core::ffi::c_char;
                    memcpy(
                        (&raw mut rebuilt as *mut ::core::ffi::c_char).offset(out as isize)
                            as *mut ::core::ffi::c_void,
                        tok as *const ::core::ffi::c_void,
                        tok_len,
                    );
                    out = out.wrapping_add(tok_len);
                    rebuilt[out] = '\0' as ::core::ffi::c_char;
                }
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stream %s: dropping unsupported RTSP source fragment '#%.*s' (value redacted; go2rtc would ignore it)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    stream_id,
                    key_len as ::core::ffi::c_int,
                    tok,
                );
            }
        }
        if end.is_null() {
            break;
        }
        p = end.offset(1 as ::core::ffi::c_int as isize);
    }
    safe_strcpy(
        url,
        &raw mut rebuilt as *mut ::core::ffi::c_char,
        url_size,
        0 as size_t,
    );
    return suppress_audio;
}
unsafe extern "C" fn go2rtc_stream_register_locked(
    mut stream_id: *const ::core::ffi::c_char,
    mut stream_url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut backchannel_enabled: bool,
    mut protocol: stream_protocol_t,
    mut record_audio: bool,
    mut codec: *const ::core::ffi::c_char,
    mut live_audio_fallback: bool,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc stream integration not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if stream_id.is_null() || stream_url.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_stream_register\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc not running, starting service\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !go2rtc_stream_start_service() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to start go2rtc service\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        let mut retries: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        while retries > 0 as ::core::ffi::c_int && !go2rtc_stream_is_ready() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Waiting for go2rtc service to be ready (%d retries left)...\0".as_ptr()
                    as *const ::core::ffi::c_char,
                retries,
            );
            sleep(1 as ::core::ffi::c_uint);
            retries -= 1;
        }
        if !go2rtc_stream_is_ready() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc service failed to start in time\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
    }
    let mut pub_cfg: stream_config_t = stream_config_t {
        camera_uuid: [0; 37],
        location_uuid: [0; 37],
        name: [0; 256],
        url: [0; 512],
        enabled: false,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        priority: 0,
        record: false,
        segment_duration: 0,
        detection_based_recording: false,
        detection_model: [0; 512],
        detection_interval: 0,
        detection_threshold: 0.,
        pre_detection_buffer: 0,
        post_detection_buffer: 0,
        detection_api_url: [0; 512],
        detection_object_filter: [0; 16],
        detection_object_filter_list: [0; 256],
        streaming_enabled: false,
        protocol: stream_protocol_t::STREAM_PROTOCOL_TCP,
        record_audio: false,
        audio_voice_enhancement: false,
        onvif_username: [0; 64],
        onvif_password: [0; 64],
        onvif_profile: [0; 64],
        onvif_port: 0,
        onvif_discovery_enabled: false,
        is_onvif: false,
        backchannel_enabled: false,
        retention_days: 0,
        detection_retention_days: 0,
        max_storage_mb: 0,
        tier_critical_multiplier: 0.,
        tier_important_multiplier: 0.,
        tier_ephemeral_multiplier: 0.,
        storage_priority: 0,
        ptz_enabled: false,
        ptz_max_x: 0,
        ptz_max_y: 0,
        ptz_max_z: 0,
        ptz_has_home: false,
        record_on_schedule: false,
        recording_schedule: [0; 168],
        detection_record_on_schedule: false,
        detection_recording_schedule: [0; 168],
        tags: [0; 256],
        admin_url: [0; 512],
        privacy_mode: false,
        motion_trigger_source: [0; 256],
        go2rtc_source_override: [0; 8192],
        sub_stream_url: [0; 512],
        detection_url: [0; 512],
        publish_url: [0; 512],
        playback_transport: [0; 24],
        eptz_config: [0; 1024],
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    memset(
        &raw mut pub_cfg as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stream_config_t>(),
    );
    let mut have_config: bool =
        get_stream_config_by_name(stream_id, &raw mut pub_cfg) == 0 as ::core::ffi::c_int;
    let mut publish_enabled: bool = have_config as ::core::ffi::c_int != 0
        && pub_cfg.publish_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    if g_config.audio_disabled {
        backchannel_enabled = r#false != 0;
        record_audio = r#false != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s: enforcing instance-wide video-only audio policy\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_id,
        );
    }
    let mut modified_url: [::core::ffi::c_char; 2048] = [0; 2048];
    safe_strcpy(
        &raw mut modified_url as *mut ::core::ffi::c_char,
        stream_url,
        URL_BUFFER_SIZE as size_t,
        0 as size_t,
    );
    let mut credentialed_url: [::core::ffi::c_char; 2048] = [0; 2048];
    if url_apply_credentials(
        &raw mut modified_url as *mut ::core::ffi::c_char,
        username,
        password,
        &raw mut credentialed_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(
            &raw mut credentialed_url as *mut ::core::ffi::c_char,
            &raw mut modified_url as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut modified_url as *mut ::core::ffi::c_char,
                &raw mut credentialed_url as *mut ::core::ffi::c_char,
                URL_BUFFER_SIZE as size_t,
                0 as size_t,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Applied credentials to go2rtc source URL for registration\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    let mut is_rtsp: bool = strncmp(
        &raw mut modified_url as *mut ::core::ffi::c_char,
        b"rtsp://\0".as_ptr() as *const ::core::ffi::c_char,
        7 as size_t,
    ) == 0 as ::core::ffi::c_int
        || strncmp(
            &raw mut modified_url as *mut ::core::ffi::c_char,
            b"rtsps://\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int;
    let mut suppress_audio: bool = g_config.audio_disabled;
    if is_rtsp {
        suppress_audio = go2rtc_sanitize_rtsp_fragments(
            &raw mut modified_url as *mut ::core::ffi::c_char,
            URL_BUFFER_SIZE as size_t,
            stream_id,
            g_config.audio_disabled,
        ) as ::core::ffi::c_int
            != 0
            || suppress_audio as ::core::ffi::c_int != 0;
        let mut fragment_params: [::core::ffi::c_char; 256] = [
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
        ];
        let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut udp_transport: bool = !strstr(
            &raw mut modified_url as *mut ::core::ffi::c_char,
            b"#transport=udp\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null();
        if strstr(
            &raw mut modified_url as *mut ::core::ffi::c_char,
            b"#transport=\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            if protocol.0 == stream_protocol_t::STREAM_PROTOCOL_UDP.0 {
                offset += snprintf(
                    (&raw mut fragment_params as *mut ::core::ffi::c_char).offset(offset as isize),
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
                        .wrapping_sub(offset as size_t),
                    b"#transport=udp\0".as_ptr() as *const ::core::ffi::c_char,
                );
                udp_transport = r#true != 0;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Adding UDP transport parameter for stream\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                offset += snprintf(
                    (&raw mut fragment_params as *mut ::core::ffi::c_char).offset(offset as isize),
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
                        .wrapping_sub(offset as size_t),
                    b"#transport=tcp\0".as_ptr() as *const ::core::ffi::c_char,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Adding TCP transport parameter for stream\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        }
        if !udp_transport {
            offset += snprintf(
                (&raw mut fragment_params as *mut ::core::ffi::c_char).offset(offset as isize),
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(offset as size_t),
                b"#timeout=30\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if g_config.audio_disabled {
            offset += snprintf(
                (&raw mut fragment_params as *mut ::core::ffi::c_char).offset(offset as isize),
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(offset as size_t),
                b"#media=video\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if backchannel_enabled {
            snprintf(
                (&raw mut fragment_params as *mut ::core::ffi::c_char).offset(offset as isize),
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(offset as size_t),
                b"#backchannel=1\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut new_url: [::core::ffi::c_char; 2048] = [0; 2048];
        snprintf(
            &raw mut new_url as *mut ::core::ffi::c_char,
            URL_BUFFER_SIZE as size_t,
            b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut modified_url as *mut ::core::ffi::c_char,
            &raw mut fragment_params as *mut ::core::ffi::c_char,
        );
        safe_strcpy(
            &raw mut modified_url as *mut ::core::ffi::c_char,
            &raw mut new_url as *mut ::core::ffi::c_char,
            URL_BUFFER_SIZE as size_t,
            0 as size_t,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Non-RTSP source URL for stream %s, skipping RTSP fragment parameters\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_id,
        );
    }
    let mut safe_source_url: [::core::ffi::c_char; 2048] = [0; 2048];
    if url_redact_for_logging(
        &raw mut modified_url as *mut ::core::ffi::c_char,
        &raw mut safe_source_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
    ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut safe_source_url as *mut ::core::ffi::c_char,
            b"[invalid-url]\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            0 as size_t,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Prepared go2rtc source URL for stream registration of %s: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_id,
        &raw mut safe_source_url as *mut ::core::ffi::c_char,
    );
    let mut sources: [*const ::core::ffi::c_char; 4] =
        [::core::ptr::null::<::core::ffi::c_char>(); 4];
    let mut num_sources: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let c2rust_fresh0 = num_sources;
    num_sources += 1;
    sources[c2rust_fresh0 as usize] = &raw mut modified_url as *mut ::core::ffi::c_char;
    let mut ffmpeg_audio_source: [::core::ffi::c_char; 2048] = [0; 2048];
    if (record_audio as ::core::ffi::c_int != 0 || publish_enabled as ::core::ffi::c_int != 0)
        && !suppress_audio
    {
        snprintf(
            &raw mut ffmpeg_audio_source as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"ffmpeg:%s#audio=aac#audio=opus\0".as_ptr() as *const ::core::ffi::c_char,
            stream_id,
        );
        let c2rust_fresh1 = num_sources;
        num_sources += 1;
        sources[c2rust_fresh1 as usize] = &raw mut ffmpeg_audio_source as *mut ::core::ffi::c_char;
    } else if live_audio_fallback as ::core::ffi::c_int != 0 && !suppress_audio {
        snprintf(
            &raw mut ffmpeg_audio_source as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"ffmpeg:%s#audio=opus\0".as_ptr() as *const ::core::ffi::c_char,
            stream_id,
        );
        let c2rust_fresh2 = num_sources;
        num_sources += 1;
        sources[c2rust_fresh2 as usize] = &raw mut ffmpeg_audio_source as *mut ::core::ffi::c_char;
    }
    let mut is_h264: bool = !codec.is_null()
        && *codec.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && strcasecmp(codec, b"h264\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int;
    let mut ffmpeg_h264_source: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut ffmpeg_hardware_source: [::core::ffi::c_char; 2048] = [0; 2048];
    if !is_h264 {
        if g_config.hw_accel_enabled {
            snprintf(
                &raw mut ffmpeg_hardware_source as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
                b"ffmpeg:%s#video=h264#hardware\0".as_ptr() as *const ::core::ffi::c_char,
                stream_id,
            );
            if have_config as ::core::ffi::c_int != 0
                && pub_cfg.width > 0 as ::core::ffi::c_int
                && pub_cfg.height > 0 as ::core::ffi::c_int
            {
                let mut width: int64_t = pub_cfg.width as int64_t;
                let mut height: int64_t = pub_cfg.height as int64_t;
                if width > GO2RTC_PREVIEW_MAX_WIDTH as int64_t {
                    height = height * GO2RTC_PREVIEW_MAX_WIDTH as int64_t / width;
                    width = GO2RTC_PREVIEW_MAX_WIDTH as int64_t;
                }
                if height > GO2RTC_PREVIEW_MAX_HEIGHT as int64_t {
                    width = width * GO2RTC_PREVIEW_MAX_HEIGHT as int64_t / height;
                    height = GO2RTC_PREVIEW_MAX_HEIGHT as int64_t;
                }
                width = (if width < 2 as int64_t {
                    2 as ::core::ffi::c_longlong
                } else {
                    width as ::core::ffi::c_longlong & !(1 as ::core::ffi::c_longlong)
                }) as int64_t;
                height = (if height < 2 as int64_t {
                    2 as ::core::ffi::c_longlong
                } else {
                    height as ::core::ffi::c_longlong & !(1 as ::core::ffi::c_longlong)
                }) as int64_t;
                let mut used: size_t =
                    strlen(&raw mut ffmpeg_hardware_source as *mut ::core::ffi::c_char);
                snprintf(
                    (&raw mut ffmpeg_hardware_source as *mut ::core::ffi::c_char)
                        .offset(used as isize),
                    ::core::mem::size_of::<[::core::ffi::c_char; 2048]>().wrapping_sub(used),
                    b"#width=%d#height=%d\0".as_ptr() as *const ::core::ffi::c_char,
                    width as ::core::ffi::c_int,
                    height as ::core::ffi::c_int,
                );
            }
            let c2rust_fresh3 = num_sources;
            num_sources += 1;
            sources[c2rust_fresh3 as usize] =
                &raw mut ffmpeg_hardware_source as *mut ::core::ffi::c_char;
        }
        snprintf(
            &raw mut ffmpeg_h264_source as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"ffmpeg:%s#video=h264\0".as_ptr() as *const ::core::ffi::c_char,
            stream_id,
        );
        let c2rust_fresh4 = num_sources;
        num_sources += 1;
        sources[c2rust_fresh4 as usize] = &raw mut ffmpeg_h264_source as *mut ::core::ffi::c_char;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s codec=%s; adding ffmpeg H.264 fallback source for WebRTC (hw_accel=%s)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_id,
            if !codec.is_null() && *codec.offset(0isize) as ::core::ffi::c_int != 0 {
                codec
            } else {
                b"unknown\0".as_ptr() as *const ::core::ffi::c_char
            },
            if g_config.hw_accel_enabled as ::core::ffi::c_int != 0 {
                b"on\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"off\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s codec=h264; no transcoding fallback needed\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_id,
        );
    }
    let mut result: bool = false;
    if num_sources > 1 as ::core::ffi::c_int {
        result = go2rtc_api_add_stream_multi(
            stream_id,
            &raw mut sources as *mut *const ::core::ffi::c_char,
            num_sources,
        );
        if !result {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to register stream %s with go2rtc (%d sources); falling back to primary-only\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_id,
                num_sources,
            );
            result =
                go2rtc_api_add_stream(stream_id, &raw mut modified_url as *mut ::core::ffi::c_char);
        }
    } else {
        result =
            go2rtc_api_add_stream(stream_id, &raw mut modified_url as *mut ::core::ffi::c_char);
    }
    if result {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Successfully registered stream %s with go2rtc (%d source%s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_id,
            num_sources,
            if num_sources == 1 as ::core::ffi::c_int {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"s\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to register stream %s with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
            stream_id,
        );
    }
    if result {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Registered stream %s with go2rtc without preloading; startup paths will preload on demand\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_id,
        );
    }
    if result as ::core::ffi::c_int != 0 && publish_enabled as ::core::ffi::c_int != 0 {
        let mut destination: [::core::ffi::c_char; 512] = [0; 512];
        let mut cursor: *const ::core::ffi::c_char =
            &raw mut pub_cfg.publish_url as *mut ::core::ffi::c_char;
        loop {
            cursor = publish_url_next(
                cursor,
                &raw mut destination as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            );
            if cursor.is_null() {
                break;
            }
            if !go2rtc_api_publish_stream_async(
                stream_id,
                &raw mut destination as *mut ::core::ffi::c_char,
            ) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to schedule RTMP restream for %s; will retry on next registration\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    stream_id,
                );
            }
        }
    }
    return result;
}
unsafe extern "C" fn go2rtc_stream_register_with_lifecycle(
    mut stream_id: *const ::core::ffi::c_char,
    mut stream_url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut backchannel_enabled: bool,
    mut protocol: stream_protocol_t,
    mut record_audio: bool,
    mut codec: *const ::core::ffi::c_char,
    mut live_audio_fallback: bool,
) -> bool {
    let mut guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    if !go2rtc_lifecycle_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_RECONFIGURE,
        r#false != 0,
        r#true != 0,
        &raw mut guard,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to enter go2rtc lifecycle to register %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            if !stream_id.is_null() {
                stream_id
            } else {
                b"<null>\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return r#false != 0;
    }
    let mut result: bool = go2rtc_stream_register_locked(
        stream_id,
        stream_url,
        username,
        password,
        backchannel_enabled,
        protocol,
        record_audio,
        codec,
        live_audio_fallback,
    );
    go2rtc_lifecycle_end(&raw mut guard, result);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_register(
    mut stream_id: *const ::core::ffi::c_char,
    mut stream_url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut backchannel_enabled: bool,
    mut protocol: stream_protocol_t,
    mut record_audio: bool,
    mut codec: *const ::core::ffi::c_char,
) -> bool {
    return go2rtc_stream_register_with_lifecycle(
        stream_id,
        stream_url,
        username,
        password,
        backchannel_enabled,
        protocol,
        record_audio,
        codec,
        r#true != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_register_substream(
    mut stream_id: *const ::core::ffi::c_char,
    mut stream_url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut protocol: stream_protocol_t,
) -> bool {
    return go2rtc_stream_register_with_lifecycle(
        stream_id,
        stream_url,
        username,
        password,
        r#false != 0,
        protocol,
        r#false != 0,
        ::core::ptr::null::<::core::ffi::c_char>(),
        r#false != 0,
    );
}
unsafe extern "C" fn go2rtc_stream_unregister_locked(
    mut stream_id: *const ::core::ffi::c_char,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc stream integration not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if stream_id.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for go2rtc_stream_unregister\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service not running, cannot unregister stream\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    go2rtc_api_delete_preload(stream_id);
    let mut result: bool = go2rtc_api_remove_stream(stream_id);
    if result {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unregistered stream from go2rtc: %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_id,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to unregister stream from go2rtc: %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_id,
        );
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_unregister(
    mut stream_id: *const ::core::ffi::c_char,
) -> bool {
    let mut guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    if !go2rtc_lifecycle_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_RECONFIGURE,
        r#false != 0,
        r#true != 0,
        &raw mut guard,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to enter go2rtc lifecycle to unregister %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            if !stream_id.is_null() {
                stream_id
            } else {
                b"<null>\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return r#false != 0;
    }
    let mut result: bool = go2rtc_stream_unregister_locked(stream_id);
    go2rtc_lifecycle_end(&raw mut guard, result);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_get_webrtc_url(
    mut stream_id: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc stream integration not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if stream_id.is_null() || buffer.is_null() || buffer_size == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_stream_get_webrtc_url\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service not running, cannot get WebRTC URL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return go2rtc_api_get_webrtc_url(stream_id, buffer, buffer_size);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_get_rtsp_url(
    mut stream_id: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc stream integration not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if stream_id.is_null() || buffer.is_null() || buffer_size == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_stream_get_rtsp_url\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service not running, cannot get RTSP URL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut rtsp_port: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !go2rtc_api_get_server_info(&raw mut rtsp_port) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get RTSP port from go2rtc API, falling back to process manager\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        rtsp_port = go2rtc_process_get_rtsp_port();
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Retrieved RTSP port from go2rtc API: %d\0".as_ptr() as *const ::core::ffi::c_char,
            rtsp_port,
        );
    }
    let mut encoded_id: [::core::ffi::c_char; 768] = [0; 768];
    simple_url_escape(
        stream_id,
        &raw mut encoded_id as *mut ::core::ffi::c_char,
        (MAX_STREAM_NAME * 3 as ::core::ffi::c_int) as size_t,
    );
    snprintf(
        buffer,
        buffer_size,
        b"rtsp://localhost:%d/%s\0".as_ptr() as *const ::core::ffi::c_char,
        rtsp_port,
        &raw mut encoded_id as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Generated RTSP URL for stream %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_id,
        buffer,
    );
    return r#true != 0;
}
unsafe extern "C" fn discard_response(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut userdata: *mut ::core::ffi::c_void,
) -> size_t {
    return size.wrapping_mul(nmemb);
}
unsafe extern "C" fn is_port_open(
    mut host: *const ::core::ffi::c_char,
    mut port: ::core::ffi::c_int,
    mut timeout_ms: ::core::ffi::c_int,
) -> bool {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: ::core::ptr::null_mut::<sockaddr>(),
        ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ai_next: ::core::ptr::null_mut::<addrinfo>(),
    };
    let mut res: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    let mut rp: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    let mut fdset: fd_set = fd_set { fds_bits: [0; 16] };
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut result: bool = r#false != 0;
    memset(
        &raw mut hints as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<addrinfo>(),
    );
    hints.ai_family = AF_INET;
    hints.ai_socktype = __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int;
    let mut port_str: [::core::ffi::c_char; 16] = [0; 16];
    snprintf(
        &raw mut port_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        port,
    );
    let mut need_to_free_addrinfo: bool = r#false != 0;
    let mut status: ::core::ffi::c_int = getaddrinfo(
        host,
        &raw mut port_str as *mut ::core::ffi::c_char,
        &raw mut hints,
        &raw mut res,
    );
    if status != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"is_port_open: getaddrinfo failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            gai_strerror(status),
        );
        return r#false != 0;
    }
    need_to_free_addrinfo = r#true != 0;
    rp = res;
    's_186: while !rp.is_null() {
        let mut sockfd: ::core::ffi::c_int =
            socket((*rp).ai_family, (*rp).ai_socktype, (*rp).ai_protocol);
        's_46: {
            if sockfd >= 0 as ::core::ffi::c_int {
                let mut flags: ::core::ffi::c_int = fcntl(sockfd, F_GETFL, 0 as ::core::ffi::c_int);
                fcntl(sockfd, F_SETFL, flags | O_NONBLOCK);
                let mut connect_res: ::core::ffi::c_int = connect(
                    sockfd,
                    __CONST_SOCKADDR_ARG {
                        __sockaddr__: (*rp).ai_addr,
                    },
                    (*rp).ai_addrlen,
                );
                if connect_res < 0 as ::core::ffi::c_int {
                    if *__errno_location() == EINPROGRESS {
                        tv.tv_sec = (timeout_ms / 1000 as ::core::ffi::c_int) as __time_t;
                        tv.tv_usec = ((timeout_ms % 1000 as ::core::ffi::c_int)
                            as ::core::ffi::c_long
                            * 1000 as ::core::ffi::c_long)
                            as __suseconds_t;
                        let mut __i: ::core::ffi::c_uint = 0;
                        let mut __arr: *mut fd_set = &raw mut fdset;
                        __i = 0 as ::core::ffi::c_uint;
                        while (__i as usize)
                            < ::core::mem::size_of::<fd_set>()
                                .wrapping_div(::core::mem::size_of::<__fd_mask>())
                        {
                            (*__arr).fds_bits[__i as usize] = 0 as __fd_mask;
                            __i = __i.wrapping_add(1);
                        }
                        fdset.fds_bits[(sockfd / __NFDBITS) as usize] |=
                            ((1 as ::core::ffi::c_ulong) << sockfd % __NFDBITS) as __fd_mask;
                        let mut select_res: ::core::ffi::c_int = select(
                            sockfd + 1 as ::core::ffi::c_int,
                            ::core::ptr::null_mut::<fd_set>(),
                            &raw mut fdset,
                            ::core::ptr::null_mut::<fd_set>(),
                            &raw mut tv,
                        );
                        if select_res < 0 as ::core::ffi::c_int {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"is_port_open: select error: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                strerror(*__errno_location()),
                            );
                            close(sockfd);
                            break 's_46;
                        } else if select_res == 0 as ::core::ffi::c_int {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"is_port_open: connection timeout\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            close(sockfd);
                            break 's_46;
                        } else {
                            let mut so_error: ::core::ffi::c_int = 0;
                            let mut len: socklen_t =
                                ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t;
                            getsockopt(
                                sockfd,
                                SOL_SOCKET,
                                SO_ERROR,
                                &raw mut so_error as *mut ::core::ffi::c_void,
                                &raw mut len,
                            );
                            if so_error != 0 as ::core::ffi::c_int {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"is_port_open: connection failed: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    strerror(so_error),
                                );
                                close(sockfd);
                                break 's_46;
                            }
                        }
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"is_port_open: connection failed: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            strerror(*__errno_location()),
                        );
                        close(sockfd);
                        break 's_46;
                    }
                }
                close(sockfd);
                result = r#true != 0;
                break 's_186;
            }
        }
        rp = (*rp).ai_next;
    }
    if need_to_free_addrinfo as ::core::ffi::c_int != 0 && !res.is_null() {
        freeaddrinfo(res);
        res = ::core::ptr::null_mut::<addrinfo>();
    }
    cleanup_dns_resolver();
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_tcp_port_open(
    mut host: *const ::core::ffi::c_char,
    mut port: ::core::ffi::c_int,
    mut timeout_ms: ::core::ffi::c_int,
) -> bool {
    if host.is_null()
        || *host.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || port <= 0 as ::core::ffi::c_int
        || port > 65535 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    return is_port_open(host, port, timeout_ms);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_is_initialized() -> bool {
    return g_initialized;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_is_ready() -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if g_ready_cache_valid as ::core::ffi::c_int != 0
        && now - g_ready_cache_time < READY_CACHE_TTL_SEC as time_t
    {
        return g_ready_cache_value;
    }
    if g_api_port <= 0 as ::core::ffi::c_int || g_api_port > 65535 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: invalid API port: %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            g_api_port,
        );
        return r#false != 0;
    }
    let mut process_running: bool = r#false != 0;
    process_running = go2rtc_process_is_running();
    if !process_running {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: process not running\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut port_open: bool = r#false != 0;
    port_open = is_port_open(
        b"localhost\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_port,
        1000 as ::core::ffi::c_int,
    );
    if !port_open {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: port %d is not open\0".as_ptr() as *const ::core::ffi::c_char,
            g_api_port,
        );
        return r#false != 0;
    }
    let mut curl: *mut ::core::ffi::c_void = NULL;
    let mut res: CURLcode = CURLcode::CURLE_OK;
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: failed to initialize curl\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut url: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut url_result: ::core::ffi::c_int = snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        b"http://localhost:%d%s/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_port,
        go2rtc_api_get_base_path(),
    );
    if url_result < 0 as ::core::ffi::c_int
        || url_result >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: failed to format URL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        curl_easy_cleanup(curl);
        return r#false != 0;
    }
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            discard_response
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_TIMEOUT, 2 as ::core::ffi::c_long);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CONNECTTIMEOUT,
        2 as ::core::ffi::c_long,
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_NOSIGNAL, 1 as ::core::ffi::c_long);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_FAILONERROR,
        1 as ::core::ffi::c_long,
    );
    res = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: curl request failed: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            curl_easy_strerror(res),
        );
        let mut sockfd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut server_addr: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        let mut request: [::core::ffi::c_char; 256] = [
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
        ];
        let mut response: [::core::ffi::c_char; 1024] = [
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
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        sockfd = socket(
            AF_INET,
            __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if sockfd < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc_stream_is_ready: socket creation failed: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            curl_easy_cleanup(curl);
            curl = NULL as *mut ::core::ffi::c_void;
            return r#false != 0;
        }
        memset(
            &raw mut server_addr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_in>(),
        );
        server_addr.sin_family = AF_INET as sa_family_t;
        server_addr.sin_port = __bswap_16(g_api_port as __uint16_t) as in_port_t;
        if inet_pton(
            AF_INET,
            b"127.0.0.1\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut server_addr.sin_addr as *mut ::core::ffi::c_void,
        ) != 1 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc_stream_is_ready: invalid IP address\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL as *mut ::core::ffi::c_void;
            return r#false != 0;
        }
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        tv.tv_sec = 2 as __time_t;
        tv.tv_usec = 0 as __suseconds_t;
        if setsockopt(
            sockfd,
            SOL_SOCKET,
            SO_RCVTIMEO,
            &raw mut tv as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<timeval>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc_stream_is_ready: failed to set receive timeout: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        if setsockopt(
            sockfd,
            SOL_SOCKET,
            SO_SNDTIMEO,
            &raw mut tv as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<timeval>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc_stream_is_ready: failed to set send timeout: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        if connect(
            sockfd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut server_addr as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc_stream_is_ready: socket connect failed: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL as *mut ::core::ffi::c_void;
            return r#false != 0;
        }
        let mut req_result: ::core::ffi::c_int = snprintf(
            &raw mut request as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"GET %s/api/streams HTTP/1.1\r\nHost: localhost:%d\r\nConnection: close\r\n\r\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            go2rtc_api_get_base_path(),
            g_api_port,
        );
        if req_result < 0 as ::core::ffi::c_int
            || req_result
                >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc_stream_is_ready: failed to format HTTP request\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL as *mut ::core::ffi::c_void;
            return r#false != 0;
        }
        let mut sent: ssize_t = send(
            sockfd,
            &raw mut request as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(&raw mut request as *mut ::core::ffi::c_char),
            0 as ::core::ffi::c_int,
        );
        if sent < 0 as ssize_t {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc_stream_is_ready: socket send failed: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL as *mut ::core::ffi::c_void;
            return r#false != 0;
        }
        let mut bytes: ssize_t = recv(
            sockfd,
            &raw mut response as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>().wrapping_sub(1 as size_t),
            0 as ::core::ffi::c_int,
        );
        if bytes <= 0 as ssize_t {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc_stream_is_ready: socket recv failed: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL as *mut ::core::ffi::c_void;
            return r#false != 0;
        }
        if bytes
            < ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int as ssize_t
        {
            response[bytes as usize] = '\0' as ::core::ffi::c_char;
        } else {
            response[::core::mem::size_of::<[::core::ffi::c_char; 1024]>().wrapping_sub(1usize)] =
                '\0' as ::core::ffi::c_char;
        }
        if !strstr(
            &raw mut response as *mut ::core::ffi::c_char,
            b"HTTP/1.1 200\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
            || !strstr(
                &raw mut response as *mut ::core::ffi::c_char,
                b"HTTP/1.1 302\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc_stream_is_ready: socket HTTP request succeeded\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            close(sockfd);
            curl_easy_cleanup(curl);
            curl = NULL as *mut ::core::ffi::c_void;
            cleanup_dns_resolver();
            g_ready_cache_value = r#true != 0;
            g_ready_cache_time = time(::core::ptr::null_mut::<time_t>());
            g_ready_cache_valid = r#true != 0;
            return r#true != 0;
        }
        let mut truncated_response: [::core::ffi::c_char; 64] = [0; 64];
        safe_strcpy(
            &raw mut truncated_response as *mut ::core::ffi::c_char,
            &raw mut response as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: socket HTTP request failed: %s...\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut truncated_response as *mut ::core::ffi::c_char,
        );
        close(sockfd);
        curl_easy_cleanup(curl);
        curl = NULL as *mut ::core::ffi::c_void;
        cleanup_dns_resolver();
        g_ready_cache_value = r#false != 0;
        g_ready_cache_time = time(::core::ptr::null_mut::<time_t>());
        g_ready_cache_valid = r#true != 0;
        return r#false != 0;
    }
    let mut info_result: CURLcode =
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
    if info_result.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: failed to get HTTP response code: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            curl_easy_strerror(info_result),
        );
        curl_easy_cleanup(curl);
        curl = NULL as *mut ::core::ffi::c_void;
        return r#false != 0;
    }
    curl_easy_cleanup(curl);
    curl = NULL as *mut ::core::ffi::c_void;
    cleanup_dns_resolver();
    if http_code == 200 as ::core::ffi::c_long {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: API is responsive (HTTP %ld)\0".as_ptr()
                as *const ::core::ffi::c_char,
            http_code,
        );
        g_ready_cache_value = r#true != 0;
        g_ready_cache_time = time(::core::ptr::null_mut::<time_t>());
        g_ready_cache_valid = r#true != 0;
        return r#true != 0;
    } else if http_code > 0 as ::core::ffi::c_long {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: API returned HTTP %ld\0".as_ptr()
                as *const ::core::ffi::c_char,
            http_code,
        );
        g_ready_cache_value = r#true != 0;
        g_ready_cache_time = time(::core::ptr::null_mut::<time_t>());
        g_ready_cache_valid = r#true != 0;
        return r#true != 0;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_stream_is_ready: API is not responsive (HTTP %ld)\0".as_ptr()
                as *const ::core::ffi::c_char,
            http_code,
        );
        g_ready_cache_value = r#false != 0;
        g_ready_cache_time = time(::core::ptr::null_mut::<time_t>());
        g_ready_cache_valid = r#true != 0;
        return r#false != 0;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_get_api_port() -> ::core::ffi::c_int {
    return g_api_port;
}
unsafe extern "C" fn go2rtc_stream_start_service_locked() -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc stream integration not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !g_config.go2rtc_enabled {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service start suppressed because go2rtc is disabled\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if go2rtc_process_is_running() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service already running, checking if it's responsive\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        if go2rtc_stream_is_ready() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Existing go2rtc service is responsive and ready to use\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if go2rtc_integration_is_initialized() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Registering all existing streams with go2rtc\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                if !go2rtc_integration_register_all_streams() {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to register all streams with go2rtc\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"go2rtc integration module not initialized, skipping stream registration\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return r#true != 0;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Existing go2rtc service is not responding, will try to wait for it\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut retries: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        while retries > 0 as ::core::ffi::c_int {
            sleep(1 as ::core::ffi::c_uint);
            if go2rtc_stream_is_ready() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"go2rtc service is now ready\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if go2rtc_integration_is_initialized() {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Registering all existing streams with go2rtc\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    if !go2rtc_integration_register_all_streams() {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to register all streams with go2rtc\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"go2rtc integration module not initialized, skipping stream registration\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                return r#true != 0;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Waiting for go2rtc service to be ready (%d retries left)...\0".as_ptr()
                    as *const ::core::ffi::c_char,
                retries,
            );
            retries -= 1;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Existing go2rtc service is not responding to API requests\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Will continue using the existing service, but it may not work properly\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting go2rtc process with API port %d\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_port,
    );
    let mut result: bool = go2rtc_process_start(g_api_port);
    if result {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service started successfully\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut retries_0: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        while retries_0 > 0 as ::core::ffi::c_int {
            sleep(1 as ::core::ffi::c_uint);
            if go2rtc_stream_is_ready() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"go2rtc service is ready\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if go2rtc_integration_is_initialized() {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Registering all existing streams with go2rtc\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    if !go2rtc_integration_register_all_streams() {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to register all streams with go2rtc\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"go2rtc integration module not initialized, skipping stream registration\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                return r#true != 0;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Waiting for go2rtc service to be ready (%d retries left)...\0".as_ptr()
                    as *const ::core::ffi::c_char,
                retries_0,
            );
            retries_0 -= 1;
        }
        if !go2rtc_stream_is_ready() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc service started but is not responding to API requests\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            's_503: {
                if go2rtc_process_is_running() {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"go2rtc process is running but not responding, checking port\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    let mut port_in_use: bool = r#false != 0;
                    let mut hex_port: [::core::ffi::c_char; 8] = [0; 8];
                    snprintf(
                        &raw mut hex_port as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
                        b":%04X\0".as_ptr() as *const ::core::ffi::c_char,
                        g_api_port,
                    );
                    let mut tcp_files: [*const ::core::ffi::c_char; 3] = [
                        b"/proc/net/tcp\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/proc/net/tcp6\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    ];
                    let mut ti: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while !tcp_files[ti as usize].is_null() && !port_in_use {
                        let mut tcp_fp: *mut FILE = fopen(
                            tcp_files[ti as usize],
                            b"r\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !tcp_fp.is_null() {
                            let mut tcp_line: [::core::ffi::c_char; 256] = [0; 256];
                            fgets(
                                &raw mut tcp_line as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
                                    as ::core::ffi::c_int,
                                tcp_fp,
                            );
                            while !fgets(
                                &raw mut tcp_line as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
                                    as ::core::ffi::c_int,
                                tcp_fp,
                            )
                            .is_null()
                            {
                                if strstr(
                                    &raw mut tcp_line as *mut ::core::ffi::c_char,
                                    &raw mut hex_port as *mut ::core::ffi::c_char,
                                )
                                .is_null()
                                {
                                    continue;
                                }
                                port_in_use = r#true != 0;
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Port %d is in use (found in %s)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    g_api_port,
                                    tcp_files[ti as usize],
                                );
                                break;
                            }
                            fclose(tcp_fp);
                        }
                        ti += 1;
                    }
                    if !port_in_use {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"go2rtc process is running but not listening on port %d\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            g_api_port,
                        );
                    }
                    let mut log_path: [::core::ffi::c_char; 512] = [0; 512];
                    if g_config.log_file[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                    {
                        let mut log_dir: [::core::ffi::c_char; 512] = [0; 512];
                        safe_strcpy(
                            &raw mut log_dir as *mut ::core::ffi::c_char,
                            &raw mut g_config.log_file as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            0 as size_t,
                        );
                        let mut last_slash: *mut ::core::ffi::c_char = strrchr(
                            &raw mut log_dir as *mut ::core::ffi::c_char,
                            '/' as ::core::ffi::c_int,
                        );
                        if !last_slash.is_null() {
                            *last_slash.offset(1 as ::core::ffi::c_int as isize) =
                                '\0' as ::core::ffi::c_char;
                            snprintf(
                                &raw mut log_path as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                                b"%sgo2rtc.log\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut log_dir as *mut ::core::ffi::c_char,
                            );
                        } else if !g_config_dir.is_null() {
                            snprintf(
                                &raw mut log_path as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                                b"%s/go2rtc.log\0".as_ptr() as *const ::core::ffi::c_char,
                                g_config_dir,
                            );
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"No valid log directory found\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            break 's_503;
                        }
                    } else if !g_config_dir.is_null() {
                        snprintf(
                            &raw mut log_path as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            b"%s/go2rtc.log\0".as_ptr() as *const ::core::ffi::c_char,
                            g_config_dir,
                        );
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Config directory not available, cannot check go2rtc log\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        break 's_503;
                    }
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Checking go2rtc log file: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut log_path as *mut ::core::ffi::c_char,
                    );
                    let mut fp: *mut FILE = fopen(
                        &raw mut log_path as *mut ::core::ffi::c_char,
                        b"r\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if !fp.is_null() {
                        let mut log_line: [::core::ffi::c_char; 1024] = [0; 1024];
                        let mut lines: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        fseek(fp, 0 as ::core::ffi::c_long, SEEK_END);
                        let mut pos: ::core::ffi::c_long = ftell(fp);
                        while pos > 0 as ::core::ffi::c_long && lines < 10 as ::core::ffi::c_int {
                            pos -= 1;
                            fseek(fp, pos, SEEK_SET);
                            let mut c: ::core::ffi::c_int = fgetc(fp);
                            if c == '\n' as ::core::ffi::c_int && pos > 0 as ::core::ffi::c_long {
                                lines += 1;
                            }
                        }
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Last few lines of go2rtc log:\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        while !fgets(
                            &raw mut log_line as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                                as ::core::ffi::c_int,
                            fp,
                        )
                        .is_null()
                        {
                            let mut len: size_t =
                                strlen(&raw mut log_line as *mut ::core::ffi::c_char);
                            if len > 0 as size_t
                                && log_line[len.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
                                    == '\n' as ::core::ffi::c_int
                            {
                                log_line[len.wrapping_sub(1 as size_t)] =
                                    '\0' as ::core::ffi::c_char;
                            }
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"  %s\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut log_line as *mut ::core::ffi::c_char,
                            );
                        }
                        fclose(fp);
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Could not open go2rtc log file: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut log_path as *mut ::core::ffi::c_char,
                        );
                    }
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"go2rtc process is not running\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
            return r#false != 0;
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start go2rtc service\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_start_service() -> bool {
    let mut guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    if !go2rtc_lifecycle_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_PROCESS_START,
        r#true != 0,
        r#false != 0,
        &raw mut guard,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to enter go2rtc lifecycle for service start\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if guard.coalesced {
        return guard.result;
    }
    let mut result: bool = go2rtc_stream_start_service_locked();
    go2rtc_lifecycle_end(&raw mut guard, result);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_stop_service() -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc stream integration not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut result: bool = go2rtc_process_stop();
    if result {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Managed go2rtc process stopped successfully\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Managed go2rtc process may still be running\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_invalidate_ready_cache() {
    g_ready_cache_valid = r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_stream_cleanup() {
    if !g_initialized {
        return;
    }
    go2rtc_api_cleanup();
    go2rtc_process_cleanup();
    if !g_config_dir.is_null() {
        free(g_config_dir as *mut ::core::ffi::c_void);
        g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    g_initialized = r#false != 0;
    g_api_port = 0 as ::core::ffi::c_int;
    g_ready_cache_valid = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc stream integration cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const __NFDBITS: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_ERROR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SO_RCVTIMEO_OLD: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SO_SNDTIMEO_OLD: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SO_RCVTIMEO: ::core::ffi::c_int = SO_RCVTIMEO_OLD;
pub const SO_SNDTIMEO: ::core::ffi::c_int = SO_SNDTIMEO_OLD;
pub const EINPROGRESS: ::core::ffi::c_int = 115 as ::core::ffi::c_int;
