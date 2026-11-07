unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    unsafe fn curl_slist_append(
        list: *mut curl_slist,
        data: *const ::core::ffi::c_char,
    ) -> *mut curl_slist;
    unsafe fn curl_slist_free_all(list: *mut curl_slist);
    unsafe fn curl_easy_strerror(error: CURLcode) -> *const ::core::ffi::c_char;
    unsafe fn curl_easy_init() -> *mut ::core::ffi::c_void;
    unsafe fn curl_easy_setopt(curl: *mut ::core::ffi::c_void, option: CURLoption, ...)
    -> CURLcode;
    unsafe fn curl_easy_perform(curl: *mut ::core::ffi::c_void) -> CURLcode;
    unsafe fn curl_easy_cleanup(curl: *mut ::core::ffi::c_void);
    unsafe fn curl_easy_getinfo(curl: *mut ::core::ffi::c_void, info: CURLINFO, ...) -> CURLcode;
    unsafe fn uv_close(handle: *mut uv_handle_t, close_cb: uv_close_cb);
    unsafe fn uv_async_init(
        _: *mut uv_loop_t,
        r#async: *mut uv_async_t,
        async_cb: uv_async_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_async_send(r#async: *mut uv_async_t) -> ::core::ffi::c_int;
    unsafe fn http_response_add_header(
        res: *mut http_response_t,
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_set_json_error(
        res: *mut http_response_t,
        status_code: ::core::ffi::c_int,
        error_message: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_init(res: *mut http_response_t);
    unsafe fn http_response_free(res: *mut http_response_t);
    unsafe fn libuv_send_response_ex(
        conn: *mut libuv_connection_t,
        response: *const http_response_t,
        action: write_complete_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_proxy_create_media_lease(
        req: *const http_request_t,
        res: *mut http_response_t,
        token: *mut ::core::ffi::c_char,
    ) -> bool;
    unsafe fn go2rtc_proxy_parse_source(
        req: *const http_request_t,
        name: *mut ::core::ffi::c_char,
        capacity: size_t,
    ) -> bool;
    unsafe fn go2rtc_proxy_build_url(
        req: *const http_request_t,
        url: *mut ::core::ffi::c_char,
        capacity: size_t,
    ) -> bool;
    unsafe fn go2rtc_proxy_authorize_request(
        req: *const http_request_t,
        res: *mut http_response_t,
    ) -> bool;
    unsafe fn go2rtc_proxy_capture_hls_sessions_for(
        stream_name: *const ::core::ffi::c_char,
        auth_fingerprint: *const ::core::ffi::c_char,
        manifest: *const ::core::ffi::c_void,
        manifest_size: size_t,
    );
    unsafe fn go2rtc_websocket_validate(
        req: *const http_request_t,
        res: *mut http_response_t,
    ) -> bool;
    unsafe fn go2rtc_websocket_start(
        conn: *mut libuv_connection_t,
        url: *const ::core::ffi::c_char,
        lease: *const ::core::ffi::c_char,
    );
    unsafe fn httpd_secure_cookies() -> bool;
    unsafe fn httpd_request_auth_fingerprint(
        req: *const http_request_t,
        fingerprint: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn log_set_thread_context(
        component: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    );
    unsafe fn safe_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn safe_free(ptr: *mut ::core::ffi::c_void);
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
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type clockid_t = __clockid_t;
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
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
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::core::ffi::c_uint,
    pub __writers: ::core::ffi::c_uint,
    pub __wrphase_futex: ::core::ffi::c_uint,
    pub __writers_futex: ::core::ffi::c_uint,
    pub __pad3: ::core::ffi::c_uint,
    pub __pad4: ::core::ffi::c_uint,
    pub __cur_writer: ::core::ffi::c_int,
    pub __shared: ::core::ffi::c_int,
    pub __pad1: ::core::ffi::c_ulong,
    pub __pad2: ::core::ffi::c_ulong,
    pub __flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const PTHREAD_CREATE_JOINABLE: Self = Self(0);
    pub const PTHREAD_CREATE_DETACHED: Self = Self(1);
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint64_t = u64;
pub type uintptr_t = usize;
pub type atomic_bool = bool;
pub type atomic_int = ::core::ffi::c_int;
pub type curl_off_t = ::core::ffi::c_long;
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
pub struct uv__queue {
    pub next: *mut uv__queue,
    pub prev: *mut uv__queue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_loop_s {
    pub data: *mut ::core::ffi::c_void,
    pub active_handles: ::core::ffi::c_uint,
    pub handle_queue: uv__queue,
    pub active_reqs: C2Rust_Unnamed_6,
    pub internal_fields: *mut ::core::ffi::c_void,
    pub stop_flag: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_ulong,
    pub backend_fd: ::core::ffi::c_int,
    pub pending_queue: uv__queue,
    pub watcher_queue: uv__queue,
    pub watchers: *mut *mut uv__io_t,
    pub nwatchers: ::core::ffi::c_uint,
    pub nfds: ::core::ffi::c_uint,
    pub wq: uv__queue,
    pub wq_mutex: uv_mutex_t,
    pub wq_async: uv_async_t,
    pub cloexec_lock: uv_rwlock_t,
    pub closing_handles: *mut uv_handle_t,
    pub process_handles: uv__queue,
    pub prepare_handles: uv__queue,
    pub check_handles: uv__queue,
    pub idle_handles: uv__queue,
    pub async_handles: uv__queue,
    pub async_unused: Option<unsafe extern "C" fn() -> ()>,
    pub async_io_watcher: uv__io_t,
    pub async_wfd: ::core::ffi::c_int,
    pub timer_heap: C2Rust_Unnamed_4,
    pub timer_counter: uint64_t,
    pub time: uint64_t,
    pub signal_pipefd: [::core::ffi::c_int; 2],
    pub signal_io_watcher: uv__io_t,
    pub child_watcher: uv_signal_t,
    pub emfile_fd: ::core::ffi::c_int,
    pub inotify_read_watcher: uv__io_t,
    pub inotify_watchers: *mut ::core::ffi::c_void,
    pub inotify_fd: ::core::ffi::c_int,
}
pub type uv__io_t = uv__io_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__io_s {
    pub bits: uintptr_t,
    pub pending_queue: uv__queue,
    pub watcher_queue: uv__queue,
    pub pevents: ::core::ffi::c_uint,
    pub events: ::core::ffi::c_uint,
    pub fd: ::core::ffi::c_int,
}
pub type uv_signal_t = uv_signal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_signal_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_3,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: ::core::ffi::c_int,
    pub tree_entry: C2Rust_Unnamed_1,
    pub caught_signals: ::core::ffi::c_uint,
    pub dispatched_signals: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub rbe_left: *mut uv_signal_s,
    pub rbe_right: *mut uv_signal_s,
    pub rbe_parent: *mut uv_signal_s,
    pub rbe_color: ::core::ffi::c_int,
}
pub type uv_signal_cb = Option<unsafe extern "C" fn(*mut uv_signal_t, ::core::ffi::c_int) -> ()>;
pub type uv_handle_t = uv_handle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_handle_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_2,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_close_cb = Option<unsafe extern "C" fn(*mut uv_handle_t) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct uv_handle_type(pub ::core::ffi::c_uint);
impl uv_handle_type {
    pub const UV_UNKNOWN_HANDLE: Self = Self(0);
    pub const UV_ASYNC: Self = Self(1);
    pub const UV_CHECK: Self = Self(2);
    pub const UV_FS_EVENT: Self = Self(3);
    pub const UV_FS_POLL: Self = Self(4);
    pub const UV_HANDLE: Self = Self(5);
    pub const UV_IDLE: Self = Self(6);
    pub const UV_NAMED_PIPE: Self = Self(7);
    pub const UV_POLL: Self = Self(8);
    pub const UV_PREPARE: Self = Self(9);
    pub const UV_PROCESS: Self = Self(10);
    pub const UV_STREAM: Self = Self(11);
    pub const UV_TCP: Self = Self(12);
    pub const UV_TIMER: Self = Self(13);
    pub const UV_TTY: Self = Self(14);
    pub const UV_UDP: Self = Self(15);
    pub const UV_SIGNAL: Self = Self(16);
    pub const UV_FILE: Self = Self(17);
    pub const UV_HANDLE_TYPE_MAX: Self = Self(18);
}
pub type uv_loop_t = uv_loop_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub min: *mut ::core::ffi::c_void,
    pub nelts: ::core::ffi::c_uint,
}
pub type uv_rwlock_t = pthread_rwlock_t;
pub type uv_async_t = uv_async_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_async_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_5,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub async_cb: uv_async_cb,
    pub queue: uv__queue,
    pub pending: ::core::ffi::c_int,
}
pub type uv_async_cb = Option<unsafe extern "C" fn(*mut uv_async_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_mutex_t = pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub unused: *mut ::core::ffi::c_void,
    pub count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_buf_t {
    pub base: *mut ::core::ffi::c_char,
    pub len: size_t,
}
pub type uv_thread_t = pthread_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__req_write_extra_s {
    pub nwritten: size_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct uv_req_type(pub ::core::ffi::c_uint);
impl uv_req_type {
    pub const UV_UNKNOWN_REQ: Self = Self(0);
    pub const UV_REQ: Self = Self(1);
    pub const UV_CONNECT: Self = Self(2);
    pub const UV_WRITE: Self = Self(3);
    pub const UV_SHUTDOWN: Self = Self(4);
    pub const UV_UDP_SEND: Self = Self(5);
    pub const UV_FS: Self = Self(6);
    pub const UV_WORK: Self = Self(7);
    pub const UV_GETADDRINFO: Self = Self(8);
    pub const UV_GETNAMEINFO: Self = Self(9);
    pub const UV_RANDOM: Self = Self(10);
    pub const UV_REQ_TYPE_MAX: Self = Self(11);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_stream_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_9,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub write_queue_size: size_t,
    pub alloc_cb: uv_alloc_cb,
    pub read_cb: uv_read_cb,
    pub connect_req: *mut uv_connect_t,
    pub shutdown_req: *mut uv_shutdown_t,
    pub io_watcher: uv__io_t,
    pub write_queue: uv__queue,
    pub write_completed_queue: uv__queue,
    pub connection_cb: uv_connection_cb,
    pub delayed_error: ::core::ffi::c_int,
    pub accepted_fd: ::core::ffi::c_int,
    pub queued_fds: *mut ::core::ffi::c_void,
}
pub type uv_connection_cb =
    Option<unsafe extern "C" fn(*mut uv_stream_t, ::core::ffi::c_int) -> ()>;
pub type uv_stream_t = uv_stream_s;
pub type uv_shutdown_t = uv_shutdown_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_shutdown_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_7,
    pub handle: *mut uv_stream_t,
    pub cb: uv_shutdown_cb,
}
pub type uv_shutdown_cb =
    Option<unsafe extern "C" fn(*mut uv_shutdown_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
}
pub type uv_connect_t = uv_connect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_connect_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_8,
    pub cb: uv_connect_cb,
    pub handle: *mut uv_stream_t,
    pub queue: uv__queue,
}
pub type uv_connect_cb = Option<unsafe extern "C" fn(*mut uv_connect_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
}
pub type uv_read_cb =
    Option<unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> ()>;
pub type uv_alloc_cb = Option<unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_tcp_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_10,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub write_queue_size: size_t,
    pub alloc_cb: uv_alloc_cb,
    pub read_cb: uv_read_cb,
    pub connect_req: *mut uv_connect_t,
    pub shutdown_req: *mut uv_shutdown_t,
    pub io_watcher: uv__io_t,
    pub write_queue: uv__queue,
    pub write_completed_queue: uv__queue,
    pub connection_cb: uv_connection_cb,
    pub delayed_error: ::core::ffi::c_int,
    pub accepted_fd: ::core::ffi::c_int,
    pub queued_fds: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_tcp_t = uv_tcp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llhttp__internal_s {
    pub _index: int32_t,
    pub _span_pos0: *mut ::core::ffi::c_void,
    pub _span_cb0: *mut ::core::ffi::c_void,
    pub error: int32_t,
    pub reason: *const ::core::ffi::c_char,
    pub error_pos: *const ::core::ffi::c_char,
    pub data: *mut ::core::ffi::c_void,
    pub _current: *mut ::core::ffi::c_void,
    pub content_length: uint64_t,
    pub r#type: uint8_t,
    pub method: uint8_t,
    pub http_major: uint8_t,
    pub http_minor: uint8_t,
    pub header_state: uint8_t,
    pub lenient_flags: uint16_t,
    pub upgrade: uint8_t,
    pub finish: uint8_t,
    pub flags: uint16_t,
    pub status_code: uint16_t,
    pub initial_message_completed: uint8_t,
    pub settings: *mut ::core::ffi::c_void,
}
pub type llhttp__internal_t = llhttp__internal_s;
pub type llhttp_t = llhttp__internal_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llhttp_settings_s {
    pub on_message_begin: llhttp_cb,
    pub on_protocol: llhttp_data_cb,
    pub on_url: llhttp_data_cb,
    pub on_status: llhttp_data_cb,
    pub on_method: llhttp_data_cb,
    pub on_version: llhttp_data_cb,
    pub on_header_field: llhttp_data_cb,
    pub on_header_value: llhttp_data_cb,
    pub on_chunk_extension_name: llhttp_data_cb,
    pub on_chunk_extension_value: llhttp_data_cb,
    pub on_headers_complete: llhttp_cb,
    pub on_body: llhttp_data_cb,
    pub on_message_complete: llhttp_cb,
    pub on_protocol_complete: llhttp_cb,
    pub on_url_complete: llhttp_cb,
    pub on_status_complete: llhttp_cb,
    pub on_method_complete: llhttp_cb,
    pub on_version_complete: llhttp_cb,
    pub on_header_field_complete: llhttp_cb,
    pub on_header_value_complete: llhttp_cb,
    pub on_chunk_extension_name_complete: llhttp_cb,
    pub on_chunk_extension_value_complete: llhttp_cb,
    pub on_chunk_header: llhttp_cb,
    pub on_chunk_complete: llhttp_cb,
    pub on_reset: llhttp_cb,
}
pub type llhttp_cb = Option<unsafe extern "C" fn(*mut llhttp_t) -> ::core::ffi::c_int>;
pub type llhttp_data_cb = Option<
    unsafe extern "C" fn(*mut llhttp_t, *const ::core::ffi::c_char, size_t) -> ::core::ffi::c_int,
>;
pub type llhttp_settings_t = llhttp_settings_s;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct http_method_t(pub ::core::ffi::c_uint);
impl http_method_t {
    pub const HTTP_METHOD_GET: Self = Self(0);
    pub const HTTP_METHOD_POST: Self = Self(1);
    pub const HTTP_METHOD_PUT: Self = Self(2);
    pub const HTTP_METHOD_DELETE: Self = Self(3);
    pub const HTTP_METHOD_OPTIONS: Self = Self(4);
    pub const HTTP_METHOD_HEAD: Self = Self(5);
    pub const HTTP_METHOD_PATCH: Self = Self(6);
    pub const HTTP_METHOD_UNKNOWN: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_header_t {
    pub name: [::core::ffi::c_char; 128],
    pub value: [::core::ffi::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_request_t {
    pub method: http_method_t,
    pub method_str: [::core::ffi::c_char; 16],
    pub path: [::core::ffi::c_char; 512],
    pub uri: [::core::ffi::c_char; 1536],
    pub query_string: [::core::ffi::c_char; 1024],
    pub content_type: [::core::ffi::c_char; 128],
    pub content_length: uint64_t,
    pub user_agent: [::core::ffi::c_char; 256],
    pub body: *mut ::core::ffi::c_void,
    pub body_len: size_t,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub client_ip: [::core::ffi::c_char; 64],
    pub request_id: [::core::ffi::c_char; 65],
    pub user_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_response_t {
    pub status_code: ::core::ffi::c_int,
    pub content_type: [::core::ffi::c_char; 128],
    pub body_length: size_t,
    pub body: *mut ::core::ffi::c_void,
    pub body_allocated: bool,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub user_data: *mut ::core::ffi::c_void,
}
pub type request_handler_t =
    Option<unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_server_config_t {
    pub port: ::core::ffi::c_int,
    pub bind_ip: *const ::core::ffi::c_char,
    pub web_root: *const ::core::ffi::c_char,
    pub auth_enabled: bool,
    pub username: [::core::ffi::c_char; 32],
    pub password: [::core::ffi::c_char; 32],
    pub cors_enabled: bool,
    pub allowed_origins: [::core::ffi::c_char; 256],
    pub allowed_methods: [::core::ffi::c_char; 256],
    pub allowed_headers: [::core::ffi::c_char; 256],
    pub ssl_enabled: bool,
    pub cert_path: [::core::ffi::c_char; 512],
    pub key_path: [::core::ffi::c_char; 512],
    pub max_connections: ::core::ffi::c_int,
    pub connection_timeout: ::core::ffi::c_int,
    pub daemon_mode: bool,
    pub pid_file: [::core::ffi::c_char; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libuv_server {
    pub r#loop: *mut uv_loop_t,
    pub listener: uv_tcp_t,
    pub stop_async: uv_async_t,
    pub config: http_server_config_t,
    pub running: bool,
    pub owns_loop: bool,
    pub shutting_down: bool,
    pub handlers: *mut C2Rust_Unnamed_11,
    pub handler_count: ::core::ffi::c_int,
    pub handler_capacity: ::core::ffi::c_int,
    pub tls_ctx: *mut ::core::ffi::c_void,
    pub thread: uv_thread_t,
    pub thread_running: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub path: [::core::ffi::c_char; 256],
    pub method: [::core::ffi::c_char; 16],
    pub handler: request_handler_t,
}
pub type libuv_server_t = libuv_server;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct write_complete_action_t(pub ::core::ffi::c_uint);
impl write_complete_action_t {
    pub const WRITE_ACTION_NONE: Self = Self(0);
    pub const WRITE_ACTION_KEEP_ALIVE: Self = Self(1);
    pub const WRITE_ACTION_CLOSE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libuv_connection {
    pub handle: uv_tcp_t,
    pub read_buf: uv_buf_t,
    pub recv_buffer: *mut ::core::ffi::c_char,
    pub recv_buffer_size: size_t,
    pub recv_buffer_used: size_t,
    pub parser: llhttp_t,
    pub settings: llhttp_settings_t,
    pub request: http_request_t,
    pub response: http_response_t,
    pub body_offset: size_t,
    pub body_present: bool,
    pub server: *mut libuv_server_t,
    pub tls_session: *mut ::core::ffi::c_void,
    pub headers_complete: bool,
    pub message_complete: bool,
    pub current_header_field: [::core::ffi::c_char; 128],
    pub current_header_field_len: size_t,
    pub keep_alive: bool,
    pub requests_handled: ::core::ffi::c_int,
    pub async_response_pending: bool,
    pub handler_on_worker: bool,
    pub close_completed: bool,
    pub go2rtc_websocket: *mut ::core::ffi::c_void,
    pub archive_stream: *mut ::core::ffi::c_void,
    pub deferred_file_serve: bool,
    pub deferred_file_path: [::core::ffi::c_char; 512],
    pub deferred_content_type: [::core::ffi::c_char; 128],
    pub deferred_extra_headers: [::core::ffi::c_char; 512],
    pub deferred_action: write_complete_action_t,
}
pub type libuv_connection_t = libuv_connection;
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
pub struct C2Rust_Unnamed_12 {
    pub r#loop: *mut uv_loop_t,
    pub async_handle: uv_async_t,
    pub done_mutex: pthread_mutex_t,
    pub workers_done: pthread_cond_t,
    pub done_head: *mut proxy_thread_ctx_t,
    pub done_tail: *mut proxy_thread_ctx_t,
    pub active_count: atomic_int,
    pub shutting_down: atomic_bool,
    pub close_pending: bool,
    pub initialized: bool,
}
pub type proxy_thread_ctx_t = proxy_thread_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct proxy_thread_ctx {
    pub request: http_request_t,
    pub url: [::core::ffi::c_char; 2048],
    pub media_lease: [::core::ffi::c_char; 37],
    pub media_policy_ack: bool,
    pub method: [::core::ffi::c_char; 16],
    pub body: *mut ::core::ffi::c_char,
    pub body_len: size_t,
    pub content_type: [::core::ffi::c_char; 256],
    pub capture_hls_session: bool,
    pub websocket_request: bool,
    pub capture_stream_name: [::core::ffi::c_char; 256],
    pub auth_fingerprint: [::core::ffi::c_char; 65],
    pub action: write_complete_action_t,
    pub conn: *mut libuv_connection_t,
    pub response_buffer: *mut ::core::ffi::c_char,
    pub response_size: size_t,
    pub response_content_type: [::core::ffi::c_char; 256],
    pub http_code: ::core::ffi::c_long,
    pub error: bool,
    pub authorization_denied: bool,
    pub auth_response: http_response_t,
    pub next: *mut proxy_thread_ctx,
}
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"go2rtcProxy\0") };
pub const MAX_CONCURRENT_PROXY_THREADS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut g_proxy_state: C2Rust_Unnamed_12 = C2Rust_Unnamed_12 {
    r#loop: ::core::ptr::null_mut::<uv_loop_t>(),
    async_handle: uv_async_t {
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        r#loop: ::core::ptr::null_mut::<uv_loop_t>(),
        r#type: uv_handle_type::UV_UNKNOWN_HANDLE,
        close_cb: None,
        handle_queue: uv__queue {
            next: ::core::ptr::null_mut::<uv__queue>(),
            prev: ::core::ptr::null_mut::<uv__queue>(),
        },
        u: C2Rust_Unnamed_5 { fd: 0 },
        next_closing: ::core::ptr::null_mut::<uv_handle_t>(),
        flags: 0,
        async_cb: None,
        queue: uv__queue {
            next: ::core::ptr::null_mut::<uv__queue>(),
            prev: ::core::ptr::null_mut::<uv__queue>(),
        },
        pending: 0,
    },
    done_mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __glibc_reserved: 0,
            __list: __pthread_list_t {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    workers_done: pthread_cond_t {
        __data: __pthread_cond_s {
            __wseq: __atomic_wide_counter { __value64: 0 },
            __g1_start: __atomic_wide_counter { __value64: 0 },
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
            __unused_initialized_1: 0,
            __unused_initialized_2: 0,
        },
    },
    done_head: ::core::ptr::null_mut::<proxy_thread_ctx_t>(),
    done_tail: ::core::ptr::null_mut::<proxy_thread_ctx_t>(),
    active_count: 0,
    shutting_down: false,
    close_pending: false,
    initialized: false,
};
unsafe extern "C" fn proxy_write_cb(
    mut contents: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> size_t {
    let mut total: size_t = size.wrapping_mul(nmemb);
    let mut ctx: *mut proxy_thread_ctx_t = userp as *mut proxy_thread_ctx_t;
    let mut new_buf: *mut ::core::ffi::c_char = realloc(
        (*ctx).response_buffer as *mut ::core::ffi::c_void,
        (*ctx).response_size.wrapping_add(total),
    ) as *mut ::core::ffi::c_char;
    if new_buf.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc proxy thread: Response buffer realloc failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        (*ctx).error = r#true != 0;
        return 0 as size_t;
    }
    (*ctx).response_buffer = new_buf;
    memcpy(
        (*ctx).response_buffer.offset((*ctx).response_size as isize) as *mut ::core::ffi::c_void,
        contents,
        total,
    );
    (*ctx).response_size = (*ctx).response_size.wrapping_add(total);
    return total;
}
unsafe extern "C" fn proxy_header_cb(
    mut buffer: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut nitems: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> size_t {
    let mut total: size_t = size.wrapping_mul(nitems);
    let mut ctx: *mut proxy_thread_ctx_t = userp as *mut proxy_thread_ctx_t;
    if total == ::core::mem::size_of::<[::core::ffi::c_char; 29]>().wrapping_sub(1usize)
        && strncasecmp(
            buffer,
            b"X-LightNVR-Media-Policy: 1\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            total,
        ) == 0 as ::core::ffi::c_int
    {
        (*ctx).media_policy_ack = r#true != 0;
    }
    if total > 14 as size_t
        && strncasecmp(
            buffer,
            b"Content-Type:\0".as_ptr() as *const ::core::ffi::c_char,
            13 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        let mut val: *const ::core::ffi::c_char = buffer.offset(13 as ::core::ffi::c_int as isize);
        while *val as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *val as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            val = val.offset(1);
        }
        let mut len: size_t = total.wrapping_sub(val.offset_from(buffer) as size_t);
        while len > 0 as size_t
            && (*val.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                == '\r' as ::core::ffi::c_int
                || *val.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                    == '\n' as ::core::ffi::c_int)
        {
            len = len.wrapping_sub(1);
        }
        if len >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>() {
            len =
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(1usize) as size_t;
        }
        memcpy(
            &raw mut (*ctx).response_content_type as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            val as *const ::core::ffi::c_void,
            len,
        );
        (*ctx).response_content_type[len] = '\0' as ::core::ffi::c_char;
    }
    return total;
}
unsafe extern "C" fn enqueue_done(mut ctx: *mut proxy_thread_ctx_t) {
    pthread_mutex_lock(&raw mut g_proxy_state.done_mutex);
    (*ctx).next = ::core::ptr::null_mut::<proxy_thread_ctx>();
    if !g_proxy_state.done_tail.is_null() {
        (*g_proxy_state.done_tail).next = ctx as *mut proxy_thread_ctx;
    } else {
        g_proxy_state.done_head = ctx;
    }
    g_proxy_state.done_tail = ctx;
    pthread_mutex_unlock(&raw mut g_proxy_state.done_mutex);
}
unsafe extern "C" fn dequeue_all_done() -> *mut proxy_thread_ctx_t {
    pthread_mutex_lock(&raw mut g_proxy_state.done_mutex);
    let mut head: *mut proxy_thread_ctx_t = g_proxy_state.done_head;
    g_proxy_state.done_head = ::core::ptr::null_mut::<proxy_thread_ctx_t>();
    g_proxy_state.done_tail = ::core::ptr::null_mut::<proxy_thread_ctx_t>();
    pthread_mutex_unlock(&raw mut g_proxy_state.done_mutex);
    return head;
}
unsafe extern "C" fn free_proxy_context(mut ctx: *mut proxy_thread_ctx_t) {
    if ctx.is_null() {
        return;
    }
    if !(*ctx).response_buffer.is_null() {
        free((*ctx).response_buffer as *mut ::core::ffi::c_void);
    }
    if !(*ctx).body.is_null() {
        free((*ctx).body as *mut ::core::ffi::c_void);
    }
    http_response_free(&raw mut (*ctx).auth_response);
    safe_free(ctx as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn proxy_progress_cb(
    mut unused: *mut ::core::ffi::c_void,
    mut download_total: curl_off_t,
    mut downloaded: curl_off_t,
    mut upload_total: curl_off_t,
    mut uploaded: curl_off_t,
) -> ::core::ffi::c_int {
    return if ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_proxy_state.shutting_down).load(::core::sync::atomic::Ordering::SeqCst) as ::core::ffi::c_int
        != 0
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn proxy_worker_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut snapshot_request: bool = false;
    let mut headers: *mut curl_slist = ::core::ptr::null_mut::<curl_slist>();
    log_set_thread_context(
        b"go2rtcProxy\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut ctx: *mut proxy_thread_ctx_t = arg as *mut proxy_thread_ctx_t;
    let mut auth_started: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut auth_finished: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut auth_started);
    if !go2rtc_proxy_authorize_request(&raw mut (*ctx).request, &raw mut (*ctx).auth_response) {
        (*ctx).authorization_denied = r#true != 0;
    }
    clock_gettime(CLOCK_MONOTONIC, &raw mut auth_finished);
    let mut auth_elapsed_ms: ::core::ffi::c_long = (auth_finished.tv_sec - auth_started.tv_sec)
        * 1000 as ::core::ffi::c_long
        + (auth_finished.tv_nsec - auth_started.tv_nsec) / 1000000 as ::core::ffi::c_long;
    if auth_elapsed_ms >= 250 as ::core::ffi::c_long {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc proxy authorization took %ld ms for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            auth_elapsed_ms,
            &raw mut (*ctx).request.path as *mut ::core::ffi::c_char,
        );
    }
    if !((*ctx).authorization_denied as ::core::ffi::c_int != 0
        || ::core::sync::atomic::AtomicBool::from_ptr(&raw mut g_proxy_state.shutting_down).load(::core::sync::atomic::Ordering::SeqCst) as ::core::ffi::c_int
            != 0)
    {
        if !go2rtc_proxy_build_url(
            &raw mut (*ctx).request,
            &raw mut (*ctx).url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        ) {
            http_response_set_json_error(
                &raw mut (*ctx).auth_response,
                400 as ::core::ffi::c_int,
                b"Invalid proxy query\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*ctx).authorization_denied = r#true != 0;
        } else if !go2rtc_proxy_create_media_lease(
            &raw mut (*ctx).request,
            &raw mut (*ctx).auth_response,
            &raw mut (*ctx).media_lease as *mut ::core::ffi::c_char,
        ) {
            (*ctx).authorization_denied = r#true != 0;
        } else if strcmp(
            &raw mut (*ctx).request.path as *mut ::core::ffi::c_char,
            b"/go2rtc/api/ws\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*ctx).websocket_request =
                go2rtc_websocket_validate(&raw mut (*ctx).request, &raw mut (*ctx).auth_response);
            (*ctx).authorization_denied = !(*ctx).websocket_request;
        } else {
            if strcmp(
                &raw mut (*ctx).request.path as *mut ::core::ffi::c_char,
                b"/go2rtc/api/stream.m3u8\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && go2rtc_proxy_parse_source(
                    &raw mut (*ctx).request,
                    &raw mut (*ctx).capture_stream_name as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                ) as ::core::ffi::c_int
                    != 0
                && httpd_request_auth_fingerprint(
                    &raw mut (*ctx).request,
                    &raw mut (*ctx).auth_fingerprint as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                (*ctx).capture_hls_session = r#true != 0;
            }
            curl = curl_easy_init();
            if curl.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"go2rtc proxy thread: curl_easy_init failed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*ctx).error = r#true != 0;
            } else {
                curl_easy_setopt(
                    curl,
                    CURLoption::CURLOPT_URL,
                    &raw mut (*ctx).url as *mut ::core::ffi::c_char,
                );
                curl_easy_setopt(
                    curl,
                    CURLoption::CURLOPT_PATH_AS_IS,
                    1 as ::core::ffi::c_long,
                );
                curl_easy_setopt(
                    curl,
                    CURLoption::CURLOPT_WRITEFUNCTION,
                    Some(
                        proxy_write_cb
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                size_t,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> size_t,
                    ),
                );
                curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, ctx);
                curl_easy_setopt(
                    curl,
                    CURLoption::CURLOPT_HEADERFUNCTION,
                    Some(
                        proxy_header_cb
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_char,
                                size_t,
                                size_t,
                                *mut ::core::ffi::c_void,
                            ) -> size_t,
                    ),
                );
                curl_easy_setopt(curl, CURLoption::CURLOPT_HEADERDATA, ctx);
                snapshot_request = strcmp(
                    &raw mut (*ctx).request.path as *mut ::core::ffi::c_char,
                    b"/go2rtc/api/frame.jpeg\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int;
                curl_easy_setopt(
                    curl,
                    CURLoption::CURLOPT_TIMEOUT,
                    if snapshot_request as ::core::ffi::c_int != 0 {
                        10 as ::core::ffi::c_long
                    } else {
                        30 as ::core::ffi::c_long
                    },
                );
                curl_easy_setopt(
                    curl,
                    CURLoption::CURLOPT_CONNECTTIMEOUT,
                    if snapshot_request as ::core::ffi::c_int != 0 {
                        2 as ::core::ffi::c_long
                    } else {
                        5 as ::core::ffi::c_long
                    },
                );
                curl_easy_setopt(curl, CURLoption::CURLOPT_NOSIGNAL, 1 as ::core::ffi::c_long);
                curl_easy_setopt(
                    curl,
                    CURLoption::CURLOPT_NOPROGRESS,
                    0 as ::core::ffi::c_long,
                );
                curl_easy_setopt(
                    curl,
                    CURLoption::CURLOPT_XFERINFOFUNCTION,
                    Some(
                        proxy_progress_cb
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                curl_off_t,
                                curl_off_t,
                                curl_off_t,
                                curl_off_t,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
                curl_easy_setopt(
                    curl,
                    CURLoption::CURLOPT_FOLLOWLOCATION,
                    0 as ::core::ffi::c_long,
                );
                headers = ::core::ptr::null_mut::<curl_slist>();
                if (*ctx).media_lease[0usize] != 0 {
                    let mut lease_header: [::core::ffi::c_char; 80] = [0; 80];
                    snprintf(
                        &raw mut lease_header as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
                        b"X-LightNVR-Media-Lease: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*ctx).media_lease as *mut ::core::ffi::c_char,
                    );
                    headers = curl_slist_append(
                        headers,
                        &raw mut lease_header as *mut ::core::ffi::c_char,
                    );
                    headers = curl_slist_append(
                        headers,
                        if httpd_secure_cookies() as ::core::ffi::c_int != 0 {
                            b"X-LightNVR-Media-TLS: 1\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"X-LightNVR-Media-TLS: 0\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                    curl_easy_setopt(curl, CURLoption::CURLOPT_HTTPHEADER, headers);
                }
                if strcmp(
                    &raw mut (*ctx).method as *mut ::core::ffi::c_char,
                    b"POST\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    curl_easy_setopt(curl, CURLoption::CURLOPT_POST, 1 as ::core::ffi::c_long);
                    if !(*ctx).body.is_null() && (*ctx).body_len > 0 as size_t {
                        curl_easy_setopt(curl, CURLoption::CURLOPT_POSTFIELDS, (*ctx).body);
                        curl_easy_setopt(
                            curl,
                            CURLoption::CURLOPT_POSTFIELDSIZE,
                            (*ctx).body_len as ::core::ffi::c_long,
                        );
                    }
                } else if strcmp(
                    &raw mut (*ctx).method as *mut ::core::ffi::c_char,
                    b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    curl_easy_setopt(
                        curl,
                        CURLoption::CURLOPT_CUSTOMREQUEST,
                        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if !(*ctx).body.is_null() && (*ctx).body_len > 0 as size_t {
                        curl_easy_setopt(curl, CURLoption::CURLOPT_POSTFIELDS, (*ctx).body);
                        curl_easy_setopt(
                            curl,
                            CURLoption::CURLOPT_POSTFIELDSIZE,
                            (*ctx).body_len as ::core::ffi::c_long,
                        );
                    }
                } else if strcmp(
                    &raw mut (*ctx).method as *mut ::core::ffi::c_char,
                    b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    curl_easy_setopt(
                        curl,
                        CURLoption::CURLOPT_CUSTOMREQUEST,
                        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                if (*ctx).content_type[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    let mut ct_header: [::core::ffi::c_char; 280] = [0; 280];
                    snprintf(
                        &raw mut ct_header as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 280]>(),
                        b"Content-Type: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*ctx).content_type as *mut ::core::ffi::c_char,
                    );
                    headers =
                        curl_slist_append(headers, &raw mut ct_header as *mut ::core::ffi::c_char);
                    curl_easy_setopt(curl, CURLoption::CURLOPT_HTTPHEADER, headers);
                }
                let mut cres: CURLcode = curl_easy_perform(curl);
                curl_easy_getinfo(
                    curl,
                    CURLINFO::CURLINFO_RESPONSE_CODE,
                    &raw mut (*ctx).http_code,
                );
                if !headers.is_null() {
                    curl_slist_free_all(headers);
                }
                curl_easy_cleanup(curl);
                if cres.0 != CURLcode::CURLE_OK.0
                    || (*ctx).error as ::core::ffi::c_int != 0
                    || (*ctx).media_lease[0usize] as ::core::ffi::c_int != 0
                        && (*ctx).http_code < 300 as ::core::ffi::c_long
                        && !(*ctx).media_policy_ack
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"go2rtc proxy thread: curl failed: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        curl_easy_strerror(cres),
                    );
                    (*ctx).error = r#true != 0;
                } else if (*ctx).capture_hls_session as ::core::ffi::c_int != 0
                    && (*ctx).http_code == 200 as ::core::ffi::c_long
                {
                    go2rtc_proxy_capture_hls_sessions_for(
                        &raw mut (*ctx).capture_stream_name as *mut ::core::ffi::c_char,
                        &raw mut (*ctx).auth_fingerprint as *mut ::core::ffi::c_char,
                        (*ctx).response_buffer as *const ::core::ffi::c_void,
                        (*ctx).response_size,
                    );
                }
            }
        }
    }
    enqueue_done(ctx);
    uv_async_send(&raw mut g_proxy_state.async_handle);
    pthread_mutex_lock(&raw mut g_proxy_state.done_mutex);
    ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_proxy_state.active_count,
        1 as ::core::ffi::c_int,
    );
    pthread_cond_signal(&raw mut g_proxy_state.workers_done);
    pthread_mutex_unlock(&raw mut g_proxy_state.done_mutex);
    return NULL;
}
unsafe extern "C" fn proxy_async_cb(mut handle: *mut uv_async_t) {
    let mut ctx: *mut proxy_thread_ctx_t = dequeue_all_done();
    while !ctx.is_null() {
        let mut next: *mut proxy_thread_ctx_t = (*ctx).next;
        let mut conn: *mut libuv_connection_t = (*ctx).conn;
        if g_proxy_state.shutting_down {
            free_proxy_context(ctx);
            ctx = next;
        } else if (*ctx).websocket_request {
            go2rtc_websocket_start(
                conn,
                &raw mut (*ctx).url as *mut ::core::ffi::c_char,
                &raw mut (*ctx).media_lease as *mut ::core::ffi::c_char,
            );
            free_proxy_context(ctx);
            ctx = next;
        } else {
            if (*ctx).authorization_denied {
                http_response_free(&raw mut (*conn).response);
                (*conn).response = (*ctx).auth_response;
                (*ctx).auth_response.body = NULL;
                (*ctx).auth_response.body_length = 0 as size_t;
                (*ctx).auth_response.body_allocated = r#false != 0;
            } else if (*ctx).error as ::core::ffi::c_int != 0
                || (*ctx).http_code == 0 as ::core::ffi::c_long
            {
                http_response_set_json_error(
                    &raw mut (*conn).response,
                    502 as ::core::ffi::c_int,
                    b"Proxy error: go2rtc is not responding\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                (*conn).response.status_code = (*ctx).http_code as ::core::ffi::c_int;
                if (*ctx).response_content_type[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                {
                    safe_strcpy(
                        &raw mut (*conn).response.content_type as *mut ::core::ffi::c_char,
                        &raw mut (*ctx).response_content_type as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                        0 as size_t,
                    );
                }
                http_response_add_header(
                    &raw mut (*conn).response,
                    b"Access-Control-Allow-Origin\0".as_ptr() as *const ::core::ffi::c_char,
                    b"*\0".as_ptr() as *const ::core::ffi::c_char,
                );
                http_response_add_header(
                    &raw mut (*conn).response,
                    b"Access-Control-Allow-Methods\0".as_ptr() as *const ::core::ffi::c_char,
                    b"GET, POST, PUT, DELETE, OPTIONS\0".as_ptr() as *const ::core::ffi::c_char,
                );
                http_response_add_header(
                    &raw mut (*conn).response,
                    b"Access-Control-Allow-Headers\0".as_ptr() as *const ::core::ffi::c_char,
                    b"Content-Type, Authorization\0".as_ptr() as *const ::core::ffi::c_char,
                );
                (*conn).response.body = (*ctx).response_buffer as *mut ::core::ffi::c_void;
                (*conn).response.body_length = (*ctx).response_size;
                (*conn).response.body_allocated = r#true != 0;
                (*ctx).response_buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"go2rtc proxy thread: %s -> %ld (%zu bytes)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*ctx).url as *mut ::core::ffi::c_char,
                    (*ctx).http_code,
                    (*ctx).response_size,
                );
            }
            (*conn).async_response_pending = r#false != 0;
            libuv_send_response_ex(conn, &raw mut (*conn).response, (*ctx).action);
            free_proxy_context(ctx);
            ctx = next;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_proxy_thread_init(
    mut r#loop: *mut uv_loop_t,
) -> ::core::ffi::c_int {
    if r#loop.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc_proxy_thread_init: NULL loop\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if g_proxy_state.close_pending {
        return -1 as ::core::ffi::c_int;
    }
    if g_proxy_state.initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc_proxy_thread_init: Already initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    memset(
        &raw mut g_proxy_state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<C2Rust_Unnamed_12>(),
    );
    g_proxy_state.r#loop = r#loop;
    *&raw mut g_proxy_state.active_count = 0 as ::core::ffi::c_int;
    *&raw mut g_proxy_state.shutting_down = r#false != 0;
    if pthread_mutex_init(
        &raw mut g_proxy_state.done_mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc_proxy_thread_init: Failed to initialize mutex\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if pthread_cond_init(
        &raw mut g_proxy_state.workers_done,
        ::core::ptr::null::<pthread_condattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_destroy(&raw mut g_proxy_state.done_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if uv_async_init(
        r#loop,
        &raw mut g_proxy_state.async_handle,
        Some(proxy_async_cb as unsafe extern "C" fn(*mut uv_async_t) -> ()),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc_proxy_thread_init: Failed to initialize uv_async\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_mutex_destroy(&raw mut g_proxy_state.done_mutex);
        pthread_cond_destroy(&raw mut g_proxy_state.workers_done);
        return -1 as ::core::ffi::c_int;
    }
    g_proxy_state.initialized = r#true != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"go2rtc_proxy_thread_init: Initialized (max %d concurrent proxy threads)\0".as_ptr()
            as *const ::core::ffi::c_char,
        32 as ::core::ffi::c_int,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_proxy_thread_submit(
    mut conn: *mut libuv_connection_t,
    mut action: write_complete_action_t,
) -> ::core::ffi::c_int {
    if !g_proxy_state.initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc_proxy_thread_submit: Not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if g_proxy_state.shutting_down {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc_proxy_thread_submit: Rejecting request during shutdown\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_proxy_state.active_count,
    ) >= MAX_CONCURRENT_PROXY_THREADS
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc_proxy_thread_submit: Max concurrent proxy threads (%d) reached\0".as_ptr()
                as *const ::core::ffi::c_char,
            32 as ::core::ffi::c_int,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut req: *const http_request_t = &raw mut (*conn).request;
    let mut ctx: *mut proxy_thread_ctx_t =
        safe_calloc(1 as size_t, ::core::mem::size_of::<proxy_thread_ctx_t>())
            as *mut proxy_thread_ctx_t;
    if ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc_proxy_thread_submit: Failed to allocate context\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    (*ctx).request = *req;
    (*ctx).request.user_data = NULL;
    (*ctx).request.body = NULL;
    (*ctx).request.body_len = 0 as size_t;
    http_response_init(&raw mut (*ctx).auth_response);
    safe_strcpy(
        &raw mut (*ctx).method as *mut ::core::ffi::c_char,
        &raw const (*req).method_str as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*ctx).content_type as *mut ::core::ffi::c_char,
        &raw const (*req).content_type as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    (*ctx).action = action;
    (*ctx).conn = conn;
    if !(*req).body.is_null() && (*req).body_len > 0 as size_t {
        (*ctx).body = malloc((*req).body_len) as *mut ::core::ffi::c_char;
        if (*ctx).body.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"go2rtc_proxy_thread_submit: Failed to copy request body\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            free_proxy_context(ctx);
            return -1 as ::core::ffi::c_int;
        }
        memcpy(
            (*ctx).body as *mut ::core::ffi::c_void,
            (*req).body,
            (*req).body_len,
        );
        (*ctx).body_len = (*req).body_len;
        (*ctx).request.body = (*ctx).body as *mut ::core::ffi::c_void;
        (*ctx).request.body_len = (*ctx).body_len;
    }
    (*conn).async_response_pending = r#true != 0;
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_proxy_state.active_count,
        1 as ::core::ffi::c_int,
    );
    let mut thread: pthread_t = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    pthread_attr_init(&raw mut attr);
    pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed_0::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
    );
    if pthread_create(
        &raw mut thread,
        &raw mut attr,
        Some(
            proxy_worker_thread
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        ctx as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc_proxy_thread_submit: pthread_create failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_attr_destroy(&raw mut attr);
        ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g_proxy_state.active_count,
            1 as ::core::ffi::c_int,
        );
        (*conn).async_response_pending = r#false != 0;
        free_proxy_context(ctx);
        return -1 as ::core::ffi::c_int;
    }
    pthread_attr_destroy(&raw mut attr);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"go2rtc_proxy_thread_submit: Submitted %s %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).method as *mut ::core::ffi::c_char,
        &raw mut (*ctx).request.path as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn proxy_async_closed(mut handle: *mut uv_handle_t) {
    g_proxy_state.close_pending = r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_proxy_thread_shutdown() {
    if !g_proxy_state.initialized {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"go2rtc_proxy_thread_shutdown: Shutting down\0".as_ptr() as *const ::core::ffi::c_char,
    );
    g_proxy_state.shutting_down = r#true != 0;
    pthread_mutex_lock(&raw mut g_proxy_state.done_mutex);
    while ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_proxy_state.active_count,
    ) > 0 as ::core::ffi::c_int
    {
        pthread_cond_wait(
            &raw mut g_proxy_state.workers_done,
            &raw mut g_proxy_state.done_mutex,
        );
    }
    pthread_mutex_unlock(&raw mut g_proxy_state.done_mutex);
    g_proxy_state.close_pending = r#true != 0;
    uv_close(
        &raw mut g_proxy_state.async_handle as *mut uv_handle_t,
        Some(proxy_async_closed as unsafe extern "C" fn(*mut uv_handle_t) -> ()),
    );
    pthread_cond_destroy(&raw mut g_proxy_state.workers_done);
    pthread_mutex_destroy(&raw mut g_proxy_state.done_mutex);
    let mut ctx: *mut proxy_thread_ctx_t = g_proxy_state.done_head;
    while !ctx.is_null() {
        let mut next: *mut proxy_thread_ctx_t = (*ctx).next;
        free_proxy_context(ctx);
        ctx = next;
    }
    g_proxy_state.done_tail = ::core::ptr::null_mut::<proxy_thread_ctx_t>();
    g_proxy_state.done_head = g_proxy_state.done_tail;
    g_proxy_state.initialized = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"go2rtc_proxy_thread_shutdown: Shutdown complete\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
