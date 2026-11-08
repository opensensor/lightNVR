unsafe extern "C" {
    unsafe fn go2rtc_api_get_base_path() -> *const ::core::ffi::c_char;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
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
    unsafe fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn sem_init(
        __sem: *mut sem_t,
        __pshared: ::core::ffi::c_int,
        __value: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    unsafe fn sem_trywait(__sem: *mut sem_t) -> ::core::ffi::c_int;
    unsafe fn sem_post(__sem: *mut sem_t) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    static mut g_config: config_t;
    unsafe fn http_request_get_header(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    unsafe fn http_request_get_query_param(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
        value: *mut ::core::ffi::c_char,
        value_len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_add_header(
        res: *mut http_response_t,
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_set_json(
        res: *mut http_response_t,
        status_code: ::core::ffi::c_int,
        json_str: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_set_json_error(
        res: *mut http_response_t,
        status_code: ::core::ffi::c_int,
        error_message: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_init(res: *mut http_response_t);
    unsafe fn http_response_free(res: *mut http_response_t);
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn authorization_evaluate(
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_secure_cookies() -> bool;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_global_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_media_stream_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    unsafe fn safe_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn safe_free(ptr: *mut ::core::ffi::c_void);
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn audit_log_append(
        req: *const http_request_t,
        user: *const user_t,
        action: *const ::core::ffi::c_char,
        target_type: *const ::core::ffi::c_char,
        target_uuid: *const ::core::ffi::c_char,
        outcome: *const ::core::ffi::c_char,
        details: *const cJSON,
    );
    unsafe fn db_fleet_camera_find_by_name_readonly(
        stream_name: *const ::core::ffi::c_char,
        camera: *mut fleet_camera_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    unsafe fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type int32_t = i32;
pub type int64_t = i64;
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
pub type pthread_t = ::core::ffi::c_ulong;
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
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
pub type uint32_t = u32;
pub type uint16_t = u16;
pub type uint8_t = u8;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [::core::ffi::c_char; 32],
    pub __align: ::core::ffi::c_long,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub type uint64_t = u64;
pub type uintptr_t = usize;
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
pub struct streaming_proxy_ctx_t {
    pub buffer: *mut ::core::ffi::c_char,
    pub buffer_size: size_t,
    pub content_type: [::core::ffi::c_char; 256],
    pub http_code: ::core::ffi::c_long,
    pub media_policy_ack: bool,
    pub error_occurred: bool,
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
pub struct hls_session_auth_t {
    pub occupied: bool,
    pub session_id: [::core::ffi::c_char; 128],
    pub auth_fingerprint: [::core::ffi::c_char; 65],
    pub stream_name: [::core::ffi::c_char; 256],
    pub authorized_until: time_t,
    pub last_used: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct proxy_query_t {
    pub source: [::core::ffi::c_char; 256],
    pub session: [::core::ffi::c_char; 128],
    pub canonical: [::core::ffi::c_char; 2048],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct go2rtc_proxy_endpoint_t(pub ::core::ffi::c_uint);
impl go2rtc_proxy_endpoint_t {
    pub const GO2RTC_PROXY_NONE: Self = Self(0);
    pub const GO2RTC_PROXY_STREAMS: Self = Self(1);
    pub const GO2RTC_PROXY_RELOAD: Self = Self(2);
    pub const GO2RTC_PROXY_MANIFEST: Self = Self(3);
    pub const GO2RTC_PROXY_SNAPSHOT: Self = Self(4);
    pub const GO2RTC_PROXY_WEBRTC: Self = Self(5);
    pub const GO2RTC_PROXY_WEBSOCKET: Self = Self(6);
    pub const GO2RTC_PROXY_HLS: Self = Self(7);
}
pub type libuv_connection_t = libuv_connection;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct write_complete_action_t(pub ::core::ffi::c_uint);
impl write_complete_action_t {
    pub const WRITE_ACTION_NONE: Self = Self(0);
    pub const WRITE_ACTION_KEEP_ALIVE: Self = Self(1);
    pub const WRITE_ACTION_CLOSE: Self = Self(2);
}
pub type libuv_server_t = libuv_server;
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
    pub handlers: *mut C2Rust_Unnamed_1,
    pub handler_count: ::core::ffi::c_int,
    pub handler_capacity: ::core::ffi::c_int,
    pub tls_ctx: *mut ::core::ffi::c_void,
    pub thread: uv_thread_t,
    pub thread_running: bool,
}
pub type uv_thread_t = pthread_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub path: [::core::ffi::c_char; 256],
    pub method: [::core::ffi::c_char; 16],
    pub handler: request_handler_t,
}
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
pub type uv_async_t = uv_async_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_async_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_7,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub async_cb: uv_async_cb,
    pub queue: uv__queue,
    pub pending: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__queue {
    pub next: *mut uv__queue,
    pub prev: *mut uv__queue,
}
pub type uv_async_cb = Option<unsafe extern "C" fn(*mut uv_async_t) -> ()>;
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
    pub timer_heap: C2Rust_Unnamed_5,
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
    pub u: C2Rust_Unnamed_4,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: ::core::ffi::c_int,
    pub tree_entry: C2Rust_Unnamed_3,
    pub caught_signals: ::core::ffi::c_uint,
    pub dispatched_signals: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub rbe_left: *mut uv_signal_s,
    pub rbe_right: *mut uv_signal_s,
    pub rbe_parent: *mut uv_signal_s,
    pub rbe_color: ::core::ffi::c_int,
}
pub type uv_signal_cb = Option<unsafe extern "C" fn(*mut uv_signal_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub min: *mut ::core::ffi::c_void,
    pub nelts: ::core::ffi::c_uint,
}
pub type uv_rwlock_t = pthread_rwlock_t;
pub type uv_mutex_t = pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub unused: *mut ::core::ffi::c_void,
    pub count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_tcp_t = uv_tcp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_tcp_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_11,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_stream_s {
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
pub type uv_shutdown_t = uv_shutdown_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_shutdown_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_8,
    pub handle: *mut uv_stream_t,
    pub cb: uv_shutdown_cb,
}
pub type uv_shutdown_cb =
    Option<unsafe extern "C" fn(*mut uv_shutdown_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
}
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
pub type uv_connect_t = uv_connect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_connect_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_9,
    pub cb: uv_connect_cb,
    pub handle: *mut uv_stream_t,
    pub queue: uv__queue,
}
pub type uv_connect_cb = Option<unsafe extern "C" fn(*mut uv_connect_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
}
pub type uv_read_cb =
    Option<unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_buf_t {
    pub base: *mut ::core::ffi::c_char,
    pub len: size_t,
}
pub type uv_alloc_cb = Option<unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type llhttp_settings_t = llhttp_settings_s;
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
pub type llhttp_t = llhttp__internal_t;
pub type llhttp__internal_t = llhttp__internal_s;
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
pub type llhttp_data_cb = Option<
    unsafe extern "C" fn(*mut llhttp_t, *const ::core::ffi::c_char, size_t) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
    pub token: [::core::ffi::c_char; 37],
    pub source: [::core::ffi::c_char; 256],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub principal_id: int64_t,
    pub authentication_method: [::core::ffi::c_char; 24],
    pub request: *mut http_request_t,
    pub touched: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_t {
    pub id: int64_t,
    pub username: [::core::ffi::c_char; 64],
    pub email: [::core::ffi::c_char; 128],
    pub role: user_role_t,
    pub api_key: [::core::ffi::c_char; 64],
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub last_login: int64_t,
    pub is_active: bool,
    pub password_change_locked: bool,
    pub must_change_password: bool,
    pub totp_enabled: bool,
    pub allowed_login_cidrs: [::core::ffi::c_char; 1024],
    pub has_login_cidr_restriction: bool,
    pub authorization_mode: [::core::ffi::c_char; 16],
    pub authenticated_via_scoped_token: bool,
    pub api_token_uuid: [::core::ffi::c_char; 37],
    pub authentication_method: [::core::ffi::c_char; 24],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct user_role_t(pub ::core::ffi::c_uint);
impl user_role_t {
    pub const USER_ROLE_ADMIN: Self = Self(0);
    pub const USER_ROLE_USER: Self = Self(1);
    pub const USER_ROLE_VIEWER: Self = Self(2);
    pub const USER_ROLE_API: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 256],
    pub address: [::core::ffi::c_char; 512],
    pub legacy_tags: [::core::ffi::c_char; 256],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub location_name: [::core::ffi::c_char; 128],
    pub location_path: [::core::ffi::c_char; 1024],
    pub location_ancestor_uuids: [[::core::ffi::c_char; 37]; 32],
    pub location_depth: ::core::ffi::c_int,
    pub tags: [fleet_camera_tag_t; 64],
    pub tag_count: ::core::ffi::c_int,
    pub manufacturer: [::core::ffi::c_char; 128],
    pub model: [::core::ffi::c_char; 128],
    pub enabled: bool,
    pub streaming_enabled: bool,
    pub record: bool,
    pub detection_based_recording: bool,
    pub is_onvif: bool,
    pub ptz_enabled: bool,
    pub backchannel_enabled: bool,
    pub health: fleet_health_state_t,
    pub availability: fleet_availability_state_t,
    pub health_changed_at: int64_t,
    pub first_video_at: int64_t,
    pub last_frame_ts: int64_t,
    pub last_recording_at: int64_t,
    pub current_fps: ::core::ffi::c_double,
    pub recording_active: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct fleet_availability_state_t(pub ::core::ffi::c_uint);
impl fleet_availability_state_t {
    pub const FLEET_AVAILABILITY_LIVE: Self = Self(0);
    pub const FLEET_AVAILABILITY_OFFLINE: Self = Self(1);
    pub const FLEET_AVAILABILITY_NEVER_CONNECTED: Self = Self(2);
    pub const FLEET_AVAILABILITY_DISABLED: Self = Self(3);
    pub const FLEET_AVAILABILITY_COUNT: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct fleet_health_state_t(pub ::core::ffi::c_uint);
impl fleet_health_state_t {
    pub const FLEET_HEALTH_UNKNOWN: Self = Self(0);
    pub const FLEET_HEALTH_UP: Self = Self(1);
    pub const FLEET_HEALTH_DEGRADED: Self = Self(2);
    pub const FLEET_HEALTH_DOWN: Self = Self(3);
    pub const FLEET_HEALTH_DISABLED: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_tag_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 256],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_action_t(pub ::core::ffi::c_int);
impl authorization_action_t {
    pub const AUTHZ_ACTION_INVALID: Self = Self(-1);
    pub const AUTHZ_LIVE_VIEW: Self = Self(0);
    pub const AUTHZ_AUDIO_LISTEN: Self = Self(1);
    pub const AUTHZ_AUDIO_TALK: Self = Self(2);
    pub const AUTHZ_RECORDINGS_REPLAY: Self = Self(3);
    pub const AUTHZ_RECORDINGS_EXPORT: Self = Self(4);
    pub const AUTHZ_SNAPSHOT_CREATE: Self = Self(5);
    pub const AUTHZ_PTZ_CONTROL: Self = Self(6);
    pub const AUTHZ_EVIDENCE_PROTECT: Self = Self(7);
    pub const AUTHZ_RECORDING_DELETE: Self = Self(8);
    pub const AUTHZ_CAMERA_CONFIGURE: Self = Self(9);
    pub const AUTHZ_FLEET_EXECUTE_JOB: Self = Self(10);
    pub const AUTHZ_STORAGE_CONFIGURE: Self = Self(11);
    pub const AUTHZ_EVENTS_CONFIGURE: Self = Self(12);
    pub const AUTHZ_USERS_MANAGE: Self = Self(13);
    pub const AUTHZ_SYSTEM_ADMIN: Self = Self(14);
    pub const AUTHZ_LPR_READ: Self = Self(15);
    pub const AUTHZ_LPR_SEARCH: Self = Self(16);
    pub const AUTHZ_LPR_EXPORT: Self = Self(17);
    pub const AUTHZ_LPR_DELETE: Self = Self(18);
    pub const AUTHZ_OBSERVATIONS_ANNOTATE: Self = Self(19);
    pub const AUTHZ_ACTION_COUNT: Self = Self(20);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub r#type: ::core::ffi::c_int,
    pub valuestring: *mut ::core::ffi::c_char,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_t(pub ::core::ffi::c_uint);
impl authorization_decision_t {
    pub const AUTHZ_DECISION_DENY: Self = Self(0);
    pub const AUTHZ_DECISION_ALLOW: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_evaluation_t {
    pub decision: authorization_decision_t,
    pub source: authorization_decision_source_t,
    pub policy_version: int64_t,
    pub grant_uuid: [::core::ffi::c_char; 37],
    pub token_uuid: [::core::ffi::c_char; 37],
    pub role_uuid: [::core::ffi::c_char; 37],
    pub role_name: [::core::ffi::c_char; 128],
    pub explanation: [::core::ffi::c_char; 256],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_source_t(pub ::core::ffi::c_uint);
impl authorization_decision_source_t {
    pub const AUTHZ_SOURCE_NONE: Self = Self(0);
    pub const AUTHZ_SOURCE_LEGACY_ROLE: Self = Self(1);
    pub const AUTHZ_SOURCE_POLICY_GRANT: Self = Self(2);
}
pub const GO2RTC_BASE_PATH: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"/go2rtc\0") };
pub const PTHREAD_ONCE_INIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const LOG_COMPONENT: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"go2rtcAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_proxy_semaphore: *mut sem_t = ::core::ptr::null_mut::<sem_t>();
static mut g_proxy_init_once: pthread_once_t = PTHREAD_ONCE_INIT;
pub const HLS_SESSION_CACHE_CAPACITY: ::core::ffi::c_int =
    if (MAX_STREAMS * 2 as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
        64 as ::core::ffi::c_int
    } else if MAX_STREAMS * 2 as ::core::ffi::c_int > 1024 as ::core::ffi::c_int {
        1024 as ::core::ffi::c_int
    } else {
        MAX_STREAMS * 2 as ::core::ffi::c_int
    };
pub const HLS_SESSION_AUTH_TTL_SECONDS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const HLS_SESSION_ID_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
static mut g_hls_sessions: [hls_session_auth_t; 1024] = [hls_session_auth_t {
    occupied: false,
    session_id: [0; 128],
    auth_fingerprint: [0; 65],
    stream_name: [0; 256],
    authorized_until: 0,
    last_used: 0,
}; 1024];
static mut g_hls_sessions_mutex: pthread_mutex_t = pthread_mutex_t {
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
unsafe extern "C" fn init_proxy_semaphore() {
    let mut max_inflight: ::core::ffi::c_int = g_config.go2rtc_proxy_max_inflight;
    if max_inflight < 1 as ::core::ffi::c_int {
        max_inflight = 16 as ::core::ffi::c_int;
    }
    g_proxy_semaphore = safe_malloc(::core::mem::size_of::<sem_t>()) as *mut sem_t;
    if g_proxy_semaphore.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc proxy: Failed to allocate semaphore\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if sem_init(
        g_proxy_semaphore,
        0 as ::core::ffi::c_int,
        max_inflight as ::core::ffi::c_uint,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc proxy: Failed to initialize semaphore\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        safe_free(g_proxy_semaphore as *mut ::core::ffi::c_void);
        g_proxy_semaphore = ::core::ptr::null_mut::<sem_t>();
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"go2rtc proxy: Initialized with max_inflight=%d\0".as_ptr() as *const ::core::ffi::c_char,
        max_inflight,
    );
}
unsafe extern "C" fn buffered_write_cb(
    mut contents: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> size_t {
    let mut total: size_t = size.wrapping_mul(nmemb);
    let mut ctx: *mut streaming_proxy_ctx_t = userp as *mut streaming_proxy_ctx_t;
    let mut new_buf: *mut ::core::ffi::c_char = realloc(
        (*ctx).buffer as *mut ::core::ffi::c_void,
        (*ctx).buffer_size.wrapping_add(total),
    ) as *mut ::core::ffi::c_char;
    if new_buf.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc proxy: Failed to allocate buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*ctx).error_occurred = r#true != 0;
        return 0 as size_t;
    }
    (*ctx).buffer = new_buf;
    memcpy(
        (*ctx).buffer.offset((*ctx).buffer_size as isize) as *mut ::core::ffi::c_void,
        contents,
        total,
    );
    (*ctx).buffer_size = (*ctx).buffer_size.wrapping_add(total);
    return total;
}
unsafe extern "C" fn buffered_header_cb(
    mut buffer: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut nitems: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> size_t {
    let mut total: size_t = size.wrapping_mul(nitems);
    let mut ctx: *mut streaming_proxy_ctx_t = userp as *mut streaming_proxy_ctx_t;
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
            &raw mut (*ctx).content_type as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            val as *const ::core::ffi::c_void,
            len,
        );
        (*ctx).content_type[len] = '\0' as ::core::ffi::c_char;
    }
    return total;
}
unsafe extern "C" fn classify_proxy_path(
    mut path: *const ::core::ffi::c_char,
) -> go2rtc_proxy_endpoint_t {
    if path.is_null() {
        return go2rtc_proxy_endpoint_t::GO2RTC_PROXY_NONE;
    }
    if strcmp(
        path,
        b"/go2rtc/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return go2rtc_proxy_endpoint_t::GO2RTC_PROXY_STREAMS;
    }
    if strcmp(
        path,
        b"/go2rtc/api/reload\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return go2rtc_proxy_endpoint_t::GO2RTC_PROXY_RELOAD;
    }
    if strcmp(
        path,
        b"/go2rtc/api/stream.m3u8\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return go2rtc_proxy_endpoint_t::GO2RTC_PROXY_MANIFEST;
    }
    if strcmp(
        path,
        b"/go2rtc/api/frame.jpeg\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return go2rtc_proxy_endpoint_t::GO2RTC_PROXY_SNAPSHOT;
    }
    if strcmp(
        path,
        b"/go2rtc/api/webrtc\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return go2rtc_proxy_endpoint_t::GO2RTC_PROXY_WEBRTC;
    }
    if strcmp(
        path,
        b"/go2rtc/api/ws\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return go2rtc_proxy_endpoint_t::GO2RTC_PROXY_WEBSOCKET;
    }
    if strcmp(
        path,
        b"/go2rtc/api/hls/playlist.m3u8\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            path,
            b"/go2rtc/api/hls/init.mp4\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            path,
            b"/go2rtc/api/hls/segment.ts\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            path,
            b"/go2rtc/api/hls/segment.m4s\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return go2rtc_proxy_endpoint_t::GO2RTC_PROXY_HLS;
    }
    return go2rtc_proxy_endpoint_t::GO2RTC_PROXY_NONE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_proxy_path_matches(mut path: *const ::core::ffi::c_char) -> bool {
    return classify_proxy_path(path).0 != go2rtc_proxy_endpoint_t::GO2RTC_PROXY_NONE.0;
}
unsafe extern "C" fn audit_hls_session_denial(mut req: *const http_request_t) {
    let mut user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    let mut principal: *mut user_t = ::core::ptr::null_mut::<user_t>();
    memset(
        &raw mut user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    if httpd_check_action_access(req, &raw mut user) != 0 {
        principal = &raw mut user;
    }
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"authorization.media_session_denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            b"missing, expired, or credential-mismatched HLS session\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    audit_log_append(
        req,
        principal,
        b"live.view\0".as_ptr() as *const ::core::ffi::c_char,
        b"camera\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"denied\0".as_ptr() as *const ::core::ffi::c_char,
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn decode_query_value(
    mut begin: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> bool {
    let mut length: size_t = 0 as size_t;
    let mut p: *const ::core::ffi::c_uchar = begin as *const ::core::ffi::c_uchar;
    while p < end as *const ::core::ffi::c_uchar {
        let mut ch: ::core::ffi::c_uchar = *p;
        if ch as ::core::ffi::c_int == ';' as ::core::ffi::c_int
            || ch as ::core::ffi::c_int == '#' as ::core::ffi::c_int
            || *(*__ctype_b_loc()).offset(ch as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            return r#false != 0;
        }
        if ch as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
            if (p as *const ::core::ffi::c_char).offset(2 as ::core::ffi::c_int as isize) >= end
                || *(*__ctype_b_loc()).offset(*p.offset(1isize) as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed_0::_ISxdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    == 0
                || *(*__ctype_b_loc()).offset(*p.offset(2isize) as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed_0::_ISxdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    == 0
            {
                return r#false != 0;
            }
            let mut decoded: ::core::ffi::c_uint = 0;
            if sscanf(
                (p as *const ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize),
                b"%2x\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut decoded,
            ) != 1 as ::core::ffi::c_int
            {
                return r#false != 0;
            }
            ch = decoded as ::core::ffi::c_uchar;
            p = p.offset(2 as ::core::ffi::c_int as isize);
        } else if ch as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
            ch = ' ' as ::core::ffi::c_uchar;
        }
        if (ch as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int
            || ch as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int
            || length.wrapping_add(1 as size_t) >= capacity
        {
            return r#false != 0;
        }
        let c2rust_fresh4 = length;
        length = length.wrapping_add(1);
        *value.offset(c2rust_fresh4 as isize) = ch as ::core::ffi::c_char;
        p = p.offset(1);
    }
    *value.offset(length as isize) = '\0' as ::core::ffi::c_char;
    return r#true != 0;
}
unsafe extern "C" fn query_unreserved(mut ch: ::core::ffi::c_uchar) -> bool {
    return ch as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
        && ch as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
        || ch as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
            && ch as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
        || ch as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && ch as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        || ch as ::core::ffi::c_int == '-' as ::core::ffi::c_int
        || ch as ::core::ffi::c_int == '_' as ::core::ffi::c_int
        || ch as ::core::ffi::c_int == '.' as ::core::ffi::c_int
        || ch as ::core::ffi::c_int == '~' as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_proxy_query(
    mut req: *const http_request_t,
    mut parsed: *mut proxy_query_t,
) -> bool {
    static mut keys: [*const ::core::ffi::c_char; 13] = [
        b"src\0".as_ptr() as *const ::core::ffi::c_char,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        b"mp4\0".as_ptr() as *const ::core::ffi::c_char,
        b"video\0".as_ptr() as *const ::core::ffi::c_char,
        b"audio\0".as_ptr() as *const ::core::ffi::c_char,
        b"cache\0".as_ptr() as *const ::core::ffi::c_char,
        b"t\0".as_ptr() as *const ::core::ffi::c_char,
        b"n\0".as_ptr() as *const ::core::ffi::c_char,
        b"width\0".as_ptr() as *const ::core::ffi::c_char,
        b"height\0".as_ptr() as *const ::core::ffi::c_char,
        b"w\0".as_ptr() as *const ::core::ffi::c_char,
        b"h\0".as_ptr() as *const ::core::ffi::c_char,
        b"rotate\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut endpoint: go2rtc_proxy_endpoint_t =
        classify_proxy_path(&raw const (*req).path as *const ::core::ffi::c_char);
    memset(
        parsed as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<proxy_query_t>(),
    );
    if endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_NONE.0 {
        return r#false != 0;
    }
    if endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_RELOAD.0 {
        return (*req).query_string[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int;
    }
    if endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_STREAMS.0
        && (*req).query_string[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    let mut seen: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut used: size_t = 0 as size_t;
    let mut p: *const ::core::ffi::c_char =
        &raw const (*req).query_string as *const ::core::ffi::c_char;
    while *p != 0 {
        let mut end: *const ::core::ffi::c_char = strchr(p, '&' as ::core::ffi::c_int);
        if end.is_null() {
            end = p.offset(strlen(p) as isize);
        }
        let mut eq: *const ::core::ffi::c_char = memchr(
            p as *const ::core::ffi::c_void,
            '=' as ::core::ffi::c_int,
            end.offset_from(p) as size_t,
        ) as *const ::core::ffi::c_char;
        let mut key_end: *const ::core::ffi::c_char = if !eq.is_null() { eq } else { end };
        let mut key: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i: size_t = 0 as size_t;
        while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 13]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
        {
            if strlen(keys[i]) == key_end.offset_from(p) as size_t
                && memcmp(
                    p as *const ::core::ffi::c_void,
                    keys[i] as *const ::core::ffi::c_void,
                    key_end.offset_from(p) as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                key = i as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        if key < 0 as ::core::ffi::c_int || seen & (1 as ::core::ffi::c_uint) << key != 0 {
            return r#false != 0;
        }
        let mut allowed: bool = r#false != 0;
        if endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_HLS.0 {
            allowed = key == 1 as ::core::ffi::c_int || key == 7 as ::core::ffi::c_int;
        } else {
            allowed = key == 0 as ::core::ffi::c_int;
            if endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_MANIFEST.0 {
                allowed = allowed as ::core::ffi::c_int
                    | (key >= 2 as ::core::ffi::c_int && key <= 4 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                    != 0;
            }
            if endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_SNAPSHOT.0 {
                allowed = allowed as ::core::ffi::c_int
                    | (key == 5 as ::core::ffi::c_int
                        || key == 6 as ::core::ffi::c_int
                        || key >= 8 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                    != 0;
            }
        }
        if !allowed {
            return r#false != 0;
        }
        seen |= (1 as ::core::ffi::c_uint) << key;
        let mut value: [::core::ffi::c_char; 256] = [0; 256];
        let mut capacity: size_t = if key == 1 as ::core::ffi::c_int {
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
        } else {
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
        };
        if !decode_query_value(
            if !eq.is_null() {
                eq.offset(1 as ::core::ffi::c_int as isize)
            } else {
                end
            },
            end,
            &raw mut value as *mut ::core::ffi::c_char,
            capacity,
        ) {
            return r#false != 0;
        }
        if key == 0 as ::core::ffi::c_int {
            if *(&raw mut value as *mut ::core::ffi::c_char) == 0 {
                return r#false != 0;
            }
            strcpy(
                &raw mut (*parsed).source as *mut ::core::ffi::c_char,
                &raw mut value as *mut ::core::ffi::c_char,
            );
        } else if key == 1 as ::core::ffi::c_int {
            if *(&raw mut value as *mut ::core::ffi::c_char) == 0 {
                return r#false != 0;
            }
            let mut v: *const ::core::ffi::c_uchar =
                &raw mut value as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
            while *v != 0 {
                if !query_unreserved(*v) {
                    return r#false != 0;
                }
                v = v.offset(1);
            }
            strcpy(
                &raw mut (*parsed).session as *mut ::core::ffi::c_char,
                &raw mut value as *mut ::core::ffi::c_char,
            );
        }
        let mut key_length: size_t = strlen(keys[key as usize]);
        if used.wrapping_add(key_length).wrapping_add(2 as size_t)
            >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()
        {
            return r#false != 0;
        }
        if used != 0 {
            let c2rust_fresh1 = used;
            used = used.wrapping_add(1);
            (*parsed).canonical[c2rust_fresh1] = '&' as ::core::ffi::c_char;
        }
        memcpy(
            (&raw mut (*parsed).canonical as *mut ::core::ffi::c_char).offset(used as isize)
                as *mut ::core::ffi::c_void,
            keys[key as usize] as *const ::core::ffi::c_void,
            key_length,
        );
        used = used.wrapping_add(key_length);
        let c2rust_fresh2 = used;
        used = used.wrapping_add(1);
        (*parsed).canonical[c2rust_fresh2] = '=' as ::core::ffi::c_char;
        let mut v_0: *const ::core::ffi::c_uchar =
            &raw mut value as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
        while *v_0 != 0 {
            let mut needed: size_t = (if query_unreserved(*v_0) as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            }) as size_t;
            if used.wrapping_add(needed) >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() {
                return r#false != 0;
            }
            if needed == 1 as size_t {
                let c2rust_fresh3 = used;
                used = used.wrapping_add(1);
                (*parsed).canonical[c2rust_fresh3] = *v_0 as ::core::ffi::c_char;
            } else {
                snprintf(
                    (&raw mut (*parsed).canonical as *mut ::core::ffi::c_char)
                        .offset(used as isize),
                    4 as size_t,
                    b"%%%02X\0".as_ptr() as *const ::core::ffi::c_char,
                    *v_0 as ::core::ffi::c_int,
                );
                used = used.wrapping_add(3 as size_t);
            }
            v_0 = v_0.offset(1);
        }
        (*parsed).canonical[used] = '\0' as ::core::ffi::c_char;
        if *end == 0 {
            break;
        }
        p = end.offset(1 as ::core::ffi::c_int as isize);
        if *p == 0 {
            return r#false != 0;
        }
    }
    return if endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_HLS.0 {
        ((*parsed).session[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } else {
        ((*parsed).source[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_proxy_parse_source(
    mut req: *const http_request_t,
    mut name: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> bool {
    let mut query: proxy_query_t = proxy_query_t {
        source: [0; 256],
        session: [0; 128],
        canonical: [0; 2048],
    };
    if req.is_null()
        || name.is_null()
        || !parse_proxy_query(req, &raw mut query)
        || query.source[0usize] == 0
        || strlen(&raw mut query.source as *mut ::core::ffi::c_char) >= capacity
    {
        return r#false != 0;
    }
    strcpy(name, &raw mut query.source as *mut ::core::ffi::c_char);
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_proxy_build_url(
    mut req: *const http_request_t,
    mut url: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> bool {
    let mut query: proxy_query_t = proxy_query_t {
        source: [0; 256],
        session: [0; 128],
        canonical: [0; 2048],
    };
    if req.is_null() || url.is_null() || !parse_proxy_query(req, &raw mut query) {
        return r#false != 0;
    }
    let mut port: ::core::ffi::c_int = if g_config.go2rtc_api_port > 0 as ::core::ffi::c_int {
        g_config.go2rtc_api_port
    } else {
        1984 as ::core::ffi::c_int
    };
    let mut length: ::core::ffi::c_int = snprintf(
        url,
        capacity,
        b"http://127.0.0.1:%d%s%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        port,
        go2rtc_api_get_base_path(),
        (&raw const (*req).path as *const ::core::ffi::c_char)
            .offset(strlen(GO2RTC_BASE_PATH.as_ptr()) as isize),
        if query.canonical[0usize] as ::core::ffi::c_int != 0 {
            b"?\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        &raw mut query.canonical as *mut ::core::ffi::c_char,
    );
    return length >= 0 as ::core::ffi::c_int && (length as size_t) < capacity;
}
unsafe extern "C" fn authorize_source(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut action: authorization_action_t,
    mut source: *const ::core::ffi::c_char,
) -> bool {
    if httpd_authorize_media_stream_action(req, res, action, source) != 0 {
        return r#true != 0;
    }
    let mut length: size_t = strlen(source);
    if (*res).status_code != 404 as ::core::ffi::c_int
        || length <= 4 as size_t
        || strcmp(
            source
                .offset(length as isize)
                .offset(-(4 as ::core::ffi::c_int as isize)),
            b"_sub\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut parent: [::core::ffi::c_char; 256] = [0; 256];
    memcpy(
        &raw mut parent as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        source as *const ::core::ffi::c_void,
        length.wrapping_sub(4 as size_t),
    );
    parent[length.wrapping_sub(4 as size_t)] = '\0' as ::core::ffi::c_char;
    http_response_free(res);
    http_response_init(res);
    return httpd_authorize_media_stream_action(
        req,
        res,
        action,
        &raw mut parent as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int;
}
pub const MEDIA_LEASE_CAPACITY: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MEDIA_LEASE_IDLE_SECONDS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
static mut media_leases: [C2Rust_Unnamed_12; 256] = [C2Rust_Unnamed_12 {
    token: [0; 37],
    source: [0; 256],
    camera_uuid: [0; 37],
    principal_id: 0,
    authentication_method: [0; 24],
    request: ::core::ptr::null_mut::<http_request_t>(),
    touched: 0,
}; 256];
static mut media_lease_mutex: pthread_mutex_t = pthread_mutex_t {
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
unsafe extern "C" fn lease_camera(
    mut source: *const ::core::ffi::c_char,
    mut camera: *mut fleet_camera_t,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = db_fleet_camera_find_by_name_readonly(source, camera);
    let mut length: size_t = strlen(source);
    if result == 1 as ::core::ffi::c_int
        && length > 4 as size_t
        && strcmp(
            source
                .offset(length as isize)
                .offset(-(4 as ::core::ffi::c_int as isize)),
            b"_sub\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
    {
        let mut parent: [::core::ffi::c_char; 256] = [0; 256];
        memcpy(
            &raw mut parent as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            source as *const ::core::ffi::c_void,
            length.wrapping_sub(4 as size_t),
        );
        parent[length.wrapping_sub(4 as size_t)] = 0 as ::core::ffi::c_char;
        result = db_fleet_camera_find_by_name_readonly(
            &raw mut parent as *mut ::core::ffi::c_char,
            camera,
        );
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_proxy_create_media_lease(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut token: *mut ::core::ffi::c_char,
) -> bool {
    *token.offset(0isize) = 0 as ::core::ffi::c_char;
    if strcmp(
        &raw const (*req).path as *const ::core::ffi::c_char,
        b"/go2rtc/api/webrtc\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
        && strcmp(
            &raw const (*req).path as *const ::core::ffi::c_char,
            b"/go2rtc/api/ws\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        && strcmp(
            &raw const (*req).path as *const ::core::ffi::c_char,
            b"/go2rtc/api/stream.m3u8\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
    {
        return r#true != 0;
    }
    let mut source: [::core::ffi::c_char; 256] = [0; 256];
    let mut camera: fleet_camera_t = fleet_camera_t {
        camera_uuid: [0; 37],
        name: [0; 256],
        address: [0; 512],
        legacy_tags: [0; 256],
        location_uuid: [0; 37],
        location_name: [0; 128],
        location_path: [0; 1024],
        location_ancestor_uuids: [[0; 37]; 32],
        location_depth: 0,
        tags: [fleet_camera_tag_t {
            uuid: [0; 37],
            label: [0; 256],
        }; 64],
        tag_count: 0,
        manufacturer: [0; 128],
        model: [0; 128],
        enabled: false,
        streaming_enabled: false,
        record: false,
        detection_based_recording: false,
        is_onvif: false,
        ptz_enabled: false,
        backchannel_enabled: false,
        health: fleet_health_state_t::FLEET_HEALTH_UNKNOWN,
        availability: fleet_availability_state_t::FLEET_AVAILABILITY_LIVE,
        health_changed_at: 0,
        first_video_at: 0,
        last_frame_ts: 0,
        last_recording_at: 0,
        current_fps: 0.,
        recording_active: false,
    };
    if !go2rtc_proxy_parse_source(
        req,
        &raw mut source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) || lease_camera(&raw mut source as *mut ::core::ffi::c_char, &raw mut camera)
        != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Camera unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut principal: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    if httpd_check_action_access(req, &raw mut principal) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Media principal unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut saved: *mut http_request_t =
        calloc(1 as size_t, ::core::mem::size_of::<http_request_t>()) as *mut http_request_t;
    if saved.is_null()
        || lightnvr_uuid_generate_v4(token) != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(token)
    {
        free(saved as *mut ::core::ffi::c_void);
        *token.offset(0isize) = 0 as ::core::ffi::c_char;
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Media authority unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        &raw mut (*saved).client_ip as *mut ::core::ffi::c_char,
        &raw const (*req).client_ip as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*saved).user_agent as *mut ::core::ffi::c_char,
        &raw const (*req).user_agent as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*saved).path as *mut ::core::ffi::c_char,
        &raw const (*req).path as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    (*saved).method = (*req).method;
    safe_strcpy(
        &raw mut (*saved).method_str as *mut ::core::ffi::c_char,
        &raw const (*req).method_str as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    let mut names: [*const ::core::ffi::c_char; 3] = [
        b"Authorization\0".as_ptr() as *const ::core::ffi::c_char,
        b"Cookie\0".as_ptr() as *const ::core::ffi::c_char,
        b"X-API-Key\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        let mut value: *const ::core::ffi::c_char = http_request_get_header(req, names[i]);
        if !value.is_null() {
            let c2rust_fresh5 = (*saved).num_headers;
            (*saved).num_headers += 1;
            let mut header: *mut http_header_t =
                (&raw mut (*saved).headers as *mut http_header_t).offset(c2rust_fresh5 as isize);
            strcpy(
                &raw mut (*header).name as *mut ::core::ffi::c_char,
                names[i],
            );
            safe_strcpy(
                &raw mut (*header).value as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                0 as size_t,
            );
        }
        i = i.wrapping_add(1);
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_lock(&raw mut media_lease_mutex);
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < MEDIA_LEASE_CAPACITY {
        if !media_leases[i_0 as usize].request.is_null()
            && now - media_leases[i_0 as usize].touched > MEDIA_LEASE_IDLE_SECONDS as time_t
        {
            memset(
                media_leases[i_0 as usize].request as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<http_request_t>(),
            );
            free(media_leases[i_0 as usize].request as *mut ::core::ffi::c_void);
            memset(
                (&raw mut media_leases as *mut C2Rust_Unnamed_12).offset(i_0 as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<C2Rust_Unnamed_12>(),
            );
        }
        if media_leases[i_0 as usize].request.is_null() && slot < 0 as ::core::ffi::c_int {
            slot = i_0;
        }
        i_0 += 1;
    }
    if slot >= 0 as ::core::ffi::c_int {
        memcpy(
            &raw mut (*(&raw mut media_leases as *mut C2Rust_Unnamed_12).offset(slot as isize))
                .token as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            token as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        );
        safe_strcpy(
            &raw mut (*(&raw mut media_leases as *mut C2Rust_Unnamed_12).offset(slot as isize))
                .source as *mut ::core::ffi::c_char,
            &raw mut source as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*(&raw mut media_leases as *mut C2Rust_Unnamed_12).offset(slot as isize))
                .camera_uuid as *mut ::core::ffi::c_char,
            &raw mut camera.camera_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        media_leases[slot as usize].principal_id = principal.id;
        safe_strcpy(
            &raw mut (*(&raw mut media_leases as *mut C2Rust_Unnamed_12).offset(slot as isize))
                .authentication_method as *mut ::core::ffi::c_char,
            &raw mut principal.authentication_method as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            0 as size_t,
        );
        media_leases[slot as usize].request = saved;
        media_leases[slot as usize].touched = now;
    }
    pthread_mutex_unlock(&raw mut media_lease_mutex);
    if slot >= 0 as ::core::ffi::c_int {
        return r#true != 0;
    }
    memset(
        saved as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<http_request_t>(),
    );
    free(saved as *mut ::core::ffi::c_void);
    *token.offset(0isize) = 0 as ::core::ffi::c_char;
    http_response_set_json_error(
        res,
        503 as ::core::ffi::c_int,
        b"Media capacity reached\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_internal_media_lease(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    http_response_add_header(
        res,
        b"Cache-Control\0".as_ptr() as *const ::core::ffi::c_char,
        b"private, no-store\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut token: *const ::core::ffi::c_char = http_request_get_header(
        req,
        b"X-LightNVR-Media-Lease\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut source: [::core::ffi::c_char; 256] = [0; 256];
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if strcmp(
        &raw const (*req).client_ip as *const ::core::ffi::c_char,
        b"127.0.0.1\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
        && strcmp(
            &raw const (*req).client_ip as *const ::core::ffi::c_char,
            b"::1\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        || token.is_null()
        || strlen(token) != 36 as size_t
        || http_request_get_query_param(
            req,
            b"src\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut source as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        ) <= 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Media lease denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut saved: *mut http_request_t =
        malloc(::core::mem::size_of::<http_request_t>()) as *mut http_request_t;
    if saved.is_null() {
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Media authority unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut found: bool = r#false != 0;
    let mut principal_id: int64_t = 0 as int64_t;
    let mut authentication_method: [::core::ffi::c_char; 24] = [
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
    ];
    pthread_mutex_lock(&raw mut media_lease_mutex);
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MEDIA_LEASE_CAPACITY {
        if !media_leases[i as usize].request.is_null()
            && strcmp(
                token,
                &raw mut (*(&raw mut media_leases as *mut C2Rust_Unnamed_12).offset(i as isize))
                    .token as *mut ::core::ffi::c_char,
            ) == 0
            && strcmp(
                &raw mut source as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut media_leases as *mut C2Rust_Unnamed_12).offset(i as isize))
                    .source as *mut ::core::ffi::c_char,
            ) == 0
            && now - media_leases[i as usize].touched <= MEDIA_LEASE_IDLE_SECONDS as time_t
        {
            *saved = *media_leases[i as usize].request;
            strcpy(
                &raw mut uuid as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut media_leases as *mut C2Rust_Unnamed_12).offset(i as isize))
                    .camera_uuid as *mut ::core::ffi::c_char,
            );
            principal_id = media_leases[i as usize].principal_id;
            strcpy(
                &raw mut authentication_method as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut media_leases as *mut C2Rust_Unnamed_12).offset(i as isize))
                    .authentication_method as *mut ::core::ffi::c_char,
            );
            media_leases[i as usize].touched = now;
            found = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut media_lease_mutex);
    let mut camera: fleet_camera_t = fleet_camera_t {
        camera_uuid: [0; 37],
        name: [0; 256],
        address: [0; 512],
        legacy_tags: [0; 256],
        location_uuid: [0; 37],
        location_name: [0; 128],
        location_path: [0; 1024],
        location_ancestor_uuids: [[0; 37]; 32],
        location_depth: 0,
        tags: [fleet_camera_tag_t {
            uuid: [0; 37],
            label: [0; 256],
        }; 64],
        tag_count: 0,
        manufacturer: [0; 128],
        model: [0; 128],
        enabled: false,
        streaming_enabled: false,
        record: false,
        detection_based_recording: false,
        is_onvif: false,
        ptz_enabled: false,
        backchannel_enabled: false,
        health: fleet_health_state_t::FLEET_HEALTH_UNKNOWN,
        availability: fleet_availability_state_t::FLEET_AVAILABILITY_LIVE,
        health_changed_at: 0,
        first_video_at: 0,
        last_frame_ts: 0,
        last_recording_at: 0,
        current_fps: 0.,
        recording_active: false,
    };
    let mut view: bool = r#false != 0;
    let mut listen: bool = r#false != 0;
    let mut talk: bool = r#false != 0;
    if found as ::core::ffi::c_int != 0
        && lease_camera(&raw mut source as *mut ::core::ffi::c_char, &raw mut camera)
            == 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut uuid as *mut ::core::ffi::c_char,
            &raw mut camera.camera_uuid as *mut ::core::ffi::c_char,
        ) == 0
    {
        let mut principal: user_t = user_t {
            id: 0,
            username: [0; 64],
            email: [0; 128],
            role: user_role_t::USER_ROLE_ADMIN,
            api_key: [0; 64],
            created_at: 0,
            updated_at: 0,
            last_login: 0,
            is_active: false,
            password_change_locked: false,
            must_change_password: false,
            totp_enabled: false,
            allowed_login_cidrs: [0; 1024],
            has_login_cidr_restriction: false,
            authorization_mode: [0; 16],
            authenticated_via_scoped_token: false,
            api_token_uuid: [0; 37],
            authentication_method: [0; 24],
        };
        let mut evaluation: authorization_evaluation_t = authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        };
        if httpd_check_action_access(saved, &raw mut principal) != 0
            && principal.id == principal_id
            && strcmp(
                &raw mut principal.authentication_method as *mut ::core::ffi::c_char,
                &raw mut authentication_method as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            view = authorization_evaluate(
                &raw mut principal,
                authorization_action_t::AUTHZ_LIVE_VIEW,
                &raw mut camera,
                &raw mut evaluation,
            ) == 0 as ::core::ffi::c_int
                && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
            listen = view as ::core::ffi::c_int != 0
                && !g_config.audio_disabled
                && authorization_evaluate(
                    &raw mut principal,
                    authorization_action_t::AUTHZ_AUDIO_LISTEN,
                    &raw mut camera,
                    &raw mut evaluation,
                ) == 0 as ::core::ffi::c_int
                && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
            talk = view as ::core::ffi::c_int != 0
                && !g_config.audio_disabled
                && authorization_evaluate(
                    &raw mut principal,
                    authorization_action_t::AUTHZ_AUDIO_TALK,
                    &raw mut camera,
                    &raw mut evaluation,
                ) == 0 as ::core::ffi::c_int
                && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
        }
    }
    memset(
        saved as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<http_request_t>(),
    );
    free(saved as *mut ::core::ffi::c_void);
    let mut body: [::core::ffi::c_char; 80] = [0; 80];
    snprintf(
        &raw mut body as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
        b"{\"view\":%s,\"listen\":%s,\"talk\":%s}\0".as_ptr() as *const ::core::ffi::c_char,
        if view as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        if listen as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        if talk as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    http_response_set_json(
        res,
        if view as ::core::ffi::c_int != 0 {
            200 as ::core::ffi::c_int
        } else {
            403 as ::core::ffi::c_int
        },
        &raw mut body as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn authorize_hls_session_request(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut session_id: [::core::ffi::c_char; 128] = [
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
    let mut fingerprint: [::core::ffi::c_char; 65] = [
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
    ];
    let mut query: proxy_query_t = proxy_query_t {
        source: [0; 256],
        session: [0; 128],
        canonical: [0; 2048],
    };
    if !parse_proxy_query(req, &raw mut query) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid HLS query\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    strcpy(
        &raw mut session_id as *mut ::core::ffi::c_char,
        &raw mut query.session as *mut ::core::ffi::c_char,
    );
    if httpd_request_auth_fingerprint(req, &raw mut fingerprint as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        audit_hls_session_denial(req);
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized HLS session\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut stream_name: [::core::ffi::c_char; 256] = [
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
    let mut found: bool = r#false != 0;
    let mut current: bool = r#false != 0;
    pthread_mutex_lock(&raw mut g_hls_sessions_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < HLS_SESSION_CACHE_CAPACITY {
        let mut entry: *mut hls_session_auth_t =
            (&raw mut g_hls_sessions as *mut hls_session_auth_t).offset(i as isize);
        if !(*entry).occupied
            || strcmp(
                &raw mut (*entry).session_id as *mut ::core::ffi::c_char,
                &raw mut session_id as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut (*entry).auth_fingerprint as *mut ::core::ffi::c_char,
                &raw mut fingerprint as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            i += 1;
        } else {
            found = r#true != 0;
            current = (*entry).authorized_until >= now;
            (*entry).last_used = now;
            safe_strcpy(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                &raw mut (*entry).stream_name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
            break;
        }
    }
    pthread_mutex_unlock(&raw mut g_hls_sessions_mutex);
    if !found {
        audit_hls_session_denial(req);
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Forbidden HLS session\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if current {
        return r#true != 0;
    }
    if !authorize_source(
        req,
        res,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    ) {
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut g_hls_sessions_mutex);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < HLS_SESSION_CACHE_CAPACITY {
        let mut entry_0: *mut hls_session_auth_t =
            (&raw mut g_hls_sessions as *mut hls_session_auth_t).offset(i_0 as isize);
        if (*entry_0).occupied as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*entry_0).session_id as *mut ::core::ffi::c_char,
                &raw mut session_id as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            && strcmp(
                &raw mut (*entry_0).auth_fingerprint as *mut ::core::ffi::c_char,
                &raw mut fingerprint as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            (*entry_0).authorized_until = now + HLS_SESSION_AUTH_TTL_SECONDS as time_t;
            (*entry_0).last_used = now;
            break;
        } else {
            i_0 += 1;
        }
    }
    pthread_mutex_unlock(&raw mut g_hls_sessions_mutex);
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_proxy_authorize_request(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) -> bool {
    if req.is_null() || res.is_null() {
        return r#false != 0;
    }
    let mut endpoint: go2rtc_proxy_endpoint_t =
        classify_proxy_path(&raw const (*req).path as *const ::core::ffi::c_char);
    if endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_NONE.0 {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut allowed_method: *const ::core::ffi::c_char = if endpoint.0
        == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_RELOAD.0
        || endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_WEBRTC.0
    {
        b"POST\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"GET\0".as_ptr() as *const ::core::ffi::c_char
    };
    if strcmp(
        &raw const (*req).method_str as *const ::core::ffi::c_char,
        allowed_method,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            405 as ::core::ffi::c_int,
            b"Method not allowed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_add_header(
            res,
            b"Allow\0".as_ptr() as *const ::core::ffi::c_char,
            allowed_method,
        );
        return r#false != 0;
    }
    if endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_STREAMS.0
        || endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_RELOAD.0
    {
        return httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN)
            != 0 as ::core::ffi::c_int;
    }
    if endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_HLS.0 {
        return authorize_hls_session_request(req, res);
    }
    let mut stream_name: [::core::ffi::c_char; 256] = [
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
    if !go2rtc_proxy_parse_source(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid playback query: expected one src camera\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut action: authorization_action_t = authorization_action_t(
        if endpoint.0 == go2rtc_proxy_endpoint_t::GO2RTC_PROXY_SNAPSHOT.0 {
            authorization_action_t::AUTHZ_SNAPSHOT_CREATE.0
        } else {
            authorization_action_t::AUTHZ_LIVE_VIEW.0
        },
    );
    return authorize_source(
        req,
        res,
        action,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn cache_hls_session(
    mut session_id: *const ::core::ffi::c_char,
    mut fingerprint: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
) {
    if session_id.is_null()
        || fingerprint.is_null()
        || stream_name.is_null()
        || *session_id.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strlen(session_id) >= HLS_SESSION_ID_MAX as size_t
        || strlen(fingerprint) >= ::core::mem::size_of::<[::core::ffi::c_char; 65]>()
        || strlen(stream_name) >= MAX_STREAM_NAME as size_t
    {
        return;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_lock(&raw mut g_hls_sessions_mutex);
    let mut selected: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut first_free: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut least_recent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut oldest: time_t = g_hls_sessions[0usize].last_used;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < HLS_SESSION_CACHE_CAPACITY {
        let mut entry: *mut hls_session_auth_t =
            (&raw mut g_hls_sessions as *mut hls_session_auth_t).offset(i as isize);
        if (*entry).occupied as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*entry).session_id as *mut ::core::ffi::c_char,
                session_id,
            ) == 0 as ::core::ffi::c_int
            && strcmp(
                &raw mut (*entry).auth_fingerprint as *mut ::core::ffi::c_char,
                fingerprint,
            ) == 0 as ::core::ffi::c_int
        {
            selected = i;
            break;
        } else {
            if !(*entry).occupied {
                if first_free < 0 as ::core::ffi::c_int {
                    first_free = i;
                }
            } else if (*entry).last_used < oldest {
                least_recent = i;
                oldest = (*entry).last_used;
            }
            i += 1;
        }
    }
    if selected < 0 as ::core::ffi::c_int {
        selected = if first_free >= 0 as ::core::ffi::c_int {
            first_free
        } else {
            least_recent
        };
    }
    let mut entry_0: *mut hls_session_auth_t =
        (&raw mut g_hls_sessions as *mut hls_session_auth_t).offset(selected as isize);
    memset(
        entry_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<hls_session_auth_t>(),
    );
    (*entry_0).occupied = r#true != 0;
    safe_strcpy(
        &raw mut (*entry_0).session_id as *mut ::core::ffi::c_char,
        session_id,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*entry_0).auth_fingerprint as *mut ::core::ffi::c_char,
        fingerprint,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*entry_0).stream_name as *mut ::core::ffi::c_char,
        stream_name,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    (*entry_0).authorized_until = now + HLS_SESSION_AUTH_TTL_SECONDS as time_t;
    (*entry_0).last_used = now;
    pthread_mutex_unlock(&raw mut g_hls_sessions_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_proxy_capture_hls_sessions(
    mut req: *const http_request_t,
    mut manifest: *const ::core::ffi::c_void,
    mut manifest_size: size_t,
) {
    if req.is_null()
        || manifest.is_null()
        || manifest_size == 0 as size_t
        || classify_proxy_path(&raw const (*req).path as *const ::core::ffi::c_char).0
            != go2rtc_proxy_endpoint_t::GO2RTC_PROXY_MANIFEST.0
    {
        return;
    }
    let mut stream_name: [::core::ffi::c_char; 256] = [
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
    let mut fingerprint: [::core::ffi::c_char; 65] = [
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
    ];
    if !go2rtc_proxy_parse_source(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) || httpd_request_auth_fingerprint(req, &raw mut fingerprint as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return;
    }
    go2rtc_proxy_capture_hls_sessions_for(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut fingerprint as *mut ::core::ffi::c_char,
        manifest,
        manifest_size,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_proxy_capture_hls_sessions_for(
    mut stream_name: *const ::core::ffi::c_char,
    mut auth_fingerprint: *const ::core::ffi::c_char,
    mut manifest: *const ::core::ffi::c_void,
    mut manifest_size: size_t,
) {
    if stream_name.is_null()
        || auth_fingerprint.is_null()
        || manifest.is_null()
        || manifest_size == 0 as size_t
    {
        return;
    }
    let mut bytes: *const ::core::ffi::c_uchar = manifest as *const ::core::ffi::c_uchar;
    let mut i: size_t = 0 as size_t;
    while i.wrapping_add(3 as size_t) < manifest_size {
        if !(i == 0 as size_t
            || *bytes.offset(i.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                != '?' as ::core::ffi::c_int
                && *bytes.offset(i.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                    != '&' as ::core::ffi::c_int
            || *bytes.offset(i as isize) as ::core::ffi::c_int != 'i' as ::core::ffi::c_int
            || *bytes.offset(i.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                != 'd' as ::core::ffi::c_int
            || *bytes.offset(i.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int
                != '=' as ::core::ffi::c_int)
        {
            let mut session_id: [::core::ffi::c_char; 128] = [
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
            let mut written: size_t = 0 as size_t;
            let mut j: size_t = i.wrapping_add(3 as size_t);
            while j < manifest_size
                && query_unreserved(*bytes.offset(j as isize)) as ::core::ffi::c_int != 0
            {
                if written.wrapping_add(1 as size_t)
                    < ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                {
                    let c2rust_fresh0 = written;
                    written = written.wrapping_add(1);
                    session_id[c2rust_fresh0] = *bytes.offset(j as isize) as ::core::ffi::c_char;
                }
                j = j.wrapping_add(1);
            }
            if j.wrapping_sub(i.wrapping_add(3 as size_t))
                < ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
            {
                if written > 0 as size_t {
                    cache_hls_session(
                        &raw mut session_id as *mut ::core::ffi::c_char,
                        auth_fingerprint,
                        stream_name,
                    );
                }
            }
        }
        i = i.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_go2rtc_proxy(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if !go2rtc_proxy_authorize_request(req, res) {
        return;
    }
    pthread_once(
        &raw mut g_proxy_init_once,
        Some(init_proxy_semaphore as unsafe extern "C" fn() -> ()),
    );
    if g_proxy_semaphore.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc proxy: Semaphore not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Proxy service unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if sem_trywait(g_proxy_semaphore) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc proxy: Max concurrent requests reached, returning 503\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Service temporarily unavailable - too many concurrent requests\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut url: [::core::ffi::c_char; 2048] = [0; 2048];
    if !go2rtc_proxy_build_url(
        req,
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
    ) {
        sem_post(g_proxy_semaphore);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid proxy query\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if strcmp(
        &raw const (*req).path as *const ::core::ffi::c_char,
        b"/go2rtc/api/ws\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        sem_post(g_proxy_semaphore);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"WebSocket upgrade required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut media_lease: [::core::ffi::c_char; 37] = [0; 37];
    if !go2rtc_proxy_create_media_lease(req, res, &raw mut media_lease as *mut ::core::ffi::c_char)
    {
        sem_post(g_proxy_semaphore);
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"go2rtc proxy: %s %s -> %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*req).method_str as *const ::core::ffi::c_char,
        &raw const (*req).path as *const ::core::ffi::c_char,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    let mut conn: *mut libuv_connection_t = (*req).user_data as *mut libuv_connection_t;
    if conn.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc proxy: No connection handle available\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        sem_post(g_proxy_semaphore);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Internal server error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ctx: streaming_proxy_ctx_t = streaming_proxy_ctx_t {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buffer_size: 0 as size_t,
        content_type: [0; 256],
        http_code: 0 as ::core::ffi::c_long,
        media_policy_ack: false,
        error_occurred: r#false != 0,
    };
    ctx.content_type[0usize] = '\0' as ::core::ffi::c_char;
    let mut curl: *mut ::core::ffi::c_void = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc proxy: curl_easy_init failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sem_post(g_proxy_semaphore);
        http_response_set_json_error(
            res,
            502 as ::core::ffi::c_int,
            b"Proxy error: failed to initialize HTTP client\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
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
            buffered_write_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut ctx);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_HEADERFUNCTION,
        Some(
            buffered_header_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_HEADERDATA, &raw mut ctx);
    curl_easy_setopt(curl, CURLoption::CURLOPT_TIMEOUT, 30 as ::core::ffi::c_long);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CONNECTTIMEOUT,
        5 as ::core::ffi::c_long,
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_NOSIGNAL, 1 as ::core::ffi::c_long);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_FOLLOWLOCATION,
        0 as ::core::ffi::c_long,
    );
    let mut headers: *mut curl_slist = ::core::ptr::null_mut::<curl_slist>();
    if media_lease[0usize] != 0 {
        let mut header: [::core::ffi::c_char; 80] = [0; 80];
        snprintf(
            &raw mut header as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
            b"X-LightNVR-Media-Lease: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut media_lease as *mut ::core::ffi::c_char,
        );
        headers = curl_slist_append(headers, &raw mut header as *mut ::core::ffi::c_char);
        headers = curl_slist_append(
            headers,
            if httpd_secure_cookies() as ::core::ffi::c_int != 0 {
                b"X-LightNVR-Media-TLS: 1\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"X-LightNVR-Media-TLS: 0\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    if strcmp(
        &raw const (*req).method_str as *const ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        curl_easy_setopt(curl, CURLoption::CURLOPT_POST, 1 as ::core::ffi::c_long);
        if !(*req).body.is_null() && (*req).body_len > 0 as size_t {
            curl_easy_setopt(curl, CURLoption::CURLOPT_POSTFIELDS, (*req).body);
            curl_easy_setopt(
                curl,
                CURLoption::CURLOPT_POSTFIELDSIZE,
                (*req).body_len as ::core::ffi::c_long,
            );
        }
    } else if strcmp(
        &raw const (*req).method_str as *const ::core::ffi::c_char,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_CUSTOMREQUEST,
            b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !(*req).body.is_null() && (*req).body_len > 0 as size_t {
            curl_easy_setopt(curl, CURLoption::CURLOPT_POSTFIELDS, (*req).body);
            curl_easy_setopt(
                curl,
                CURLoption::CURLOPT_POSTFIELDSIZE,
                (*req).body_len as ::core::ffi::c_long,
            );
        }
    } else if strcmp(
        &raw const (*req).method_str as *const ::core::ffi::c_char,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_CUSTOMREQUEST,
            b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*req).content_type[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut ct_header: [::core::ffi::c_char; 256] = [0; 256];
        snprintf(
            &raw mut ct_header as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"Content-Type: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*req).content_type as *const ::core::ffi::c_char,
        );
        headers = curl_slist_append(headers, &raw mut ct_header as *mut ::core::ffi::c_char);
    }
    if !headers.is_null() {
        curl_easy_setopt(curl, CURLoption::CURLOPT_HTTPHEADER, headers);
    }
    let mut cres: CURLcode = curl_easy_perform(curl);
    curl_easy_getinfo(
        curl,
        CURLINFO::CURLINFO_RESPONSE_CODE,
        &raw mut ctx.http_code,
    );
    if !headers.is_null() {
        curl_slist_free_all(headers);
    }
    curl_easy_cleanup(curl);
    sem_post(g_proxy_semaphore);
    if cres.0 != CURLcode::CURLE_OK.0
        || ctx.error_occurred as ::core::ffi::c_int != 0
        || media_lease[0usize] as ::core::ffi::c_int != 0
            && ctx.http_code < 300 as ::core::ffi::c_long
            && !ctx.media_policy_ack
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc proxy: curl request failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            curl_easy_strerror(cres),
        );
        if !ctx.buffer.is_null() {
            free(ctx.buffer as *mut ::core::ffi::c_void);
        }
        http_response_set_json_error(
            res,
            502 as ::core::ffi::c_int,
            b"Proxy error: go2rtc is not responding\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    (*res).status_code = ctx.http_code as ::core::ffi::c_int;
    if ctx.content_type[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut (*res).content_type as *mut ::core::ffi::c_char,
            &raw mut ctx.content_type as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    }
    http_response_add_header(
        res,
        b"Access-Control-Allow-Origin\0".as_ptr() as *const ::core::ffi::c_char,
        b"*\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_add_header(
        res,
        b"Access-Control-Allow-Methods\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET, POST, PUT, DELETE, OPTIONS\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_add_header(
        res,
        b"Access-Control-Allow-Headers\0".as_ptr() as *const ::core::ffi::c_char,
        b"Content-Type, Authorization\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*res).body = ctx.buffer as *mut ::core::ffi::c_void;
    (*res).body_length = ctx.buffer_size;
    (*res).body_allocated = r#true != 0;
    if ctx.http_code == 200 as ::core::ffi::c_long {
        go2rtc_proxy_capture_hls_sessions(
            req,
            ctx.buffer as *const ::core::ffi::c_void,
            ctx.buffer_size,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"go2rtc proxy: %s %s -> %ld (%zu bytes, type: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*req).method_str as *const ::core::ffi::c_char,
        &raw const (*req).path as *const ::core::ffi::c_char,
        ctx.http_code,
        ctx.buffer_size,
        if ctx.content_type[0usize] as ::core::ffi::c_int != 0 {
            &raw mut ctx.content_type as *mut ::core::ffi::c_char
        } else {
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
