unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn curl_easy_escape(
        curl: *mut ::core::ffi::c_void,
        string: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn curl_free(p: *mut ::core::ffi::c_void);
    unsafe fn curl_easy_strerror(error: CURLcode) -> *const ::core::ffi::c_char;
    unsafe fn curl_easy_init() -> *mut ::core::ffi::c_void;
    unsafe fn curl_easy_setopt(curl: *mut ::core::ffi::c_void, option: CURLoption, ...)
    -> CURLcode;
    unsafe fn curl_easy_perform(curl: *mut ::core::ffi::c_void) -> CURLcode;
    unsafe fn curl_easy_cleanup(curl: *mut ::core::ffi::c_void);
    unsafe fn curl_easy_getinfo(curl: *mut ::core::ffi::c_void, info: CURLINFO, ...) -> CURLcode;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_HasObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> cJSON_bool;
    unsafe fn cJSON_GetStringValue(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_detach(__th: pthread_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_api_get_base_path() -> *const ::core::ffi::c_char;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn simple_url_escape(
        input: *const ::core::ffi::c_char,
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    );
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub struct go2rtc_stream_activity_t {
    pub stream_name: *const ::core::ffi::c_char,
    pub alternate_stream_name: *const ::core::ffi::c_char,
    pub video_active: bool,
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
pub struct response_buffer_t {
    pub buffer: [::core::ffi::c_char; 65536],
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryStruct {
    pub memory: *mut ::core::ffi::c_char,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct publish_job_t {
    pub stream_id: *mut ::core::ffi::c_char,
    pub destination: *mut ::core::ffi::c_char,
}
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g1_start: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g_size: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __g1_orig_size: 0 as ::core::ffi::c_uint,
        __wrefs: 0 as ::core::ffi::c_uint,
        __g_signals: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __unused_initialized_1: 0 as ::core::ffi::c_uint,
        __unused_initialized_2: 0 as ::core::ffi::c_uint,
    },
};
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut g_api_host: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut g_api_port: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_initialized: bool = r#false != 0;
static mut g_publish_jobs_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_0::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut g_publish_jobs_done: pthread_cond_t = PTHREAD_COND_INITIALIZER;
static mut g_active_publish_jobs: size_t = 0 as size_t;
static mut g_accept_publish_jobs: bool = r#false != 0;
pub const HTTP_RESPONSE_SIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const URL_BUFFER_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const GO2RTC_API_READ_TIMEOUT_SECONDS: ::core::ffi::c_long = 5 as ::core::ffi::c_long;
pub const GO2RTC_API_MUTATION_TIMEOUT_SECONDS: ::core::ffi::c_long = 10 as ::core::ffi::c_long;
unsafe extern "C" fn configure_curl_timeouts(
    mut curl: *mut ::core::ffi::c_void,
    mut timeout_seconds: ::core::ffi::c_long,
) {
    curl_easy_setopt(curl, CURLoption::CURLOPT_TIMEOUT, timeout_seconds);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CONNECTTIMEOUT,
        2 as ::core::ffi::c_long,
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_NOSIGNAL, 1 as ::core::ffi::c_long);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_init(
    mut api_host: *const ::core::ffi::c_char,
    mut api_port: ::core::ffi::c_int,
) -> bool {
    if g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client already initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if api_host.is_null() || api_port <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_api_init\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    g_api_host = strdup(api_host);
    if g_api_host.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate go2rtc API host\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    g_api_port = api_port;
    g_initialized = r#true != 0;
    pthread_mutex_lock(&raw mut g_publish_jobs_mutex);
    g_accept_publish_jobs = r#true != 0;
    pthread_mutex_unlock(&raw mut g_publish_jobs_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc API client initialized with host: %s, port: %d\0".as_ptr()
            as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
    );
    return r#true != 0;
}
unsafe extern "C" fn WriteMemoryCallback(
    mut contents: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> size_t {
    let mut realsize: size_t = size.wrapping_mul(nmemb);
    let mut mem: *mut MemoryStruct = userp as *mut MemoryStruct;
    let mut ptr: *mut ::core::ffi::c_char = realloc(
        (*mem).memory as *mut ::core::ffi::c_void,
        (*mem).size.wrapping_add(realsize).wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    if ptr.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Not enough memory for CURL response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as size_t;
    }
    (*mem).memory = ptr;
    memcpy(
        (*mem).memory.offset((*mem).size as isize) as *mut ::core::ffi::c_void,
        contents,
        realsize,
    );
    (*mem).size = (*mem).size.wrapping_add(realsize);
    *(*mem).memory.offset((*mem).size as isize) = 0 as ::core::ffi::c_char;
    return realsize;
}
unsafe extern "C" fn PerRequestWriteCallback(
    mut contents: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> size_t {
    let mut realsize: size_t = size.wrapping_mul(nmemb);
    let mut resp: *mut response_buffer_t = userp as *mut response_buffer_t;
    if (*resp)
        .size
        .wrapping_add(realsize)
        .wrapping_add(1 as size_t)
        > HTTP_RESPONSE_SIZE as size_t
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"CURL response buffer full, truncating\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if (*resp).size.wrapping_add(1 as size_t) >= HTTP_RESPONSE_SIZE as size_t {
            return 0 as size_t;
        }
        realsize = (HTTP_RESPONSE_SIZE as size_t)
            .wrapping_sub((*resp).size)
            .wrapping_sub(1 as size_t);
    }
    memcpy(
        (&raw mut (*resp).buffer as *mut ::core::ffi::c_char).offset((*resp).size as isize)
            as *mut ::core::ffi::c_void,
        contents,
        realsize,
    );
    (*resp).size = (*resp).size.wrapping_add(realsize);
    (*resp).buffer[(*resp).size] = 0 as ::core::ffi::c_char;
    return realsize;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_add_stream(
    mut stream_id: *const ::core::ffi::c_char,
    mut stream_url: *const ::core::ffi::c_char,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if stream_id.is_null() || stream_url.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_api_add_stream\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut res: CURLcode = CURLcode::CURLE_OK;
    let mut url: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut success: bool = r#false != 0;
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut resp: response_buffer_t = response_buffer_t {
        buffer: [0; 65536],
        size: 0 as size_t,
    };
    resp.buffer[0usize] = '\0' as ::core::ffi::c_char;
    let mut encoded_url: [::core::ffi::c_char; 3072] = [0; 3072];
    simple_url_escape(
        stream_url,
        &raw mut encoded_url as *mut ::core::ffi::c_char,
        (URL_BUFFER_SIZE * 3 as ::core::ffi::c_int) as size_t,
    );
    let mut encoded_stream_id: [::core::ffi::c_char; 768] = [0; 768];
    simple_url_escape(
        stream_id,
        &raw mut encoded_stream_id as *mut ::core::ffi::c_char,
        (MAX_STREAM_NAME * 3 as ::core::ffi::c_int) as size_t,
    );
    snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"http://%s:%d%s/api/streams?src=%s&name=%s\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
        &raw mut encoded_url as *mut ::core::ffi::c_char,
        &raw mut encoded_stream_id as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Adding go2rtc stream source for %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_id,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CUSTOMREQUEST,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            PerRequestWriteCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut resp);
    configure_curl_timeouts(curl, GO2RTC_API_MUTATION_TIMEOUT_SECONDS);
    res = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"CURL request failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            curl_easy_strerror(res),
        );
    } else {
        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
        if http_code == 200 as ::core::ffi::c_long {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Added stream to go2rtc: %s\0".as_ptr() as *const ::core::ffi::c_char,
                stream_id,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Response: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut resp.buffer as *mut ::core::ffi::c_char,
            );
            success = r#true != 0;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to add stream to go2rtc (status %ld): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                http_code,
                &raw mut resp.buffer as *mut ::core::ffi::c_char,
            );
        }
    }
    curl_easy_cleanup(curl);
    return success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_add_stream_multi(
    mut stream_id: *const ::core::ffi::c_char,
    mut sources: *mut *const ::core::ffi::c_char,
    mut num_sources: ::core::ffi::c_int,
) -> bool {
    let mut encoded_stream_id: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if stream_id.is_null() || sources.is_null() || num_sources <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_api_add_stream_multi\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut res: CURLcode = CURLcode::CURLE_OK;
    let mut success: bool = r#false != 0;
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut resp: response_buffer_t = response_buffer_t {
        buffer: [0; 65536],
        size: 0 as size_t,
    };
    resp.buffer[0usize] = '\0' as ::core::ffi::c_char;
    let mut url: *mut ::core::ffi::c_char = malloc(
        (URL_BUFFER_SIZE as size_t).wrapping_mul((num_sources + 1 as ::core::ffi::c_int) as size_t),
    ) as *mut ::core::ffi::c_char;
    if url.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        curl_easy_cleanup(curl);
        return r#false != 0;
    }
    let mut url_buf_size: size_t =
        (URL_BUFFER_SIZE as size_t).wrapping_mul((num_sources + 1 as ::core::ffi::c_int) as size_t);
    let mut written: ::core::ffi::c_int = snprintf(
        url,
        url_buf_size,
        b"http://%s:%d%s/api/streams?\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
    );
    if written < 0 as ::core::ffi::c_int || written as size_t >= url_buf_size {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"URL buffer overflow building go2rtc request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(url as *mut ::core::ffi::c_void);
        curl_easy_cleanup(curl);
        return r#false != 0;
    }
    let mut offset: size_t = written as size_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    '_request_too_long: {
        while i < num_sources {
            if (*sources.offset(i as isize)).is_null() {
                break '_request_too_long;
            }
            let mut encoded_url: *mut ::core::ffi::c_char =
                curl_easy_escape(curl, *sources.offset(i as isize), 0 as ::core::ffi::c_int);
            if encoded_url.is_null() {
                break '_request_too_long;
            }
            if strlen(encoded_url) >= (URL_BUFFER_SIZE * 3 as ::core::ffi::c_int) as size_t {
                curl_free(encoded_url as *mut ::core::ffi::c_void);
                break '_request_too_long;
            } else {
                written = snprintf(
                    url.offset(offset as isize),
                    url_buf_size.wrapping_sub(offset),
                    b"%ssrc=%s\0".as_ptr() as *const ::core::ffi::c_char,
                    if i > 0 as ::core::ffi::c_int {
                        b"&\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    encoded_url,
                );
                curl_free(encoded_url as *mut ::core::ffi::c_void);
                if written < 0 as ::core::ffi::c_int
                    || written as size_t >= url_buf_size.wrapping_sub(offset)
                {
                    break '_request_too_long;
                }
                offset = offset.wrapping_add(written as size_t);
                i += 1;
            }
        }
        encoded_stream_id = curl_easy_escape(curl, stream_id, 0 as ::core::ffi::c_int);
        if !encoded_stream_id.is_null() {
            if strlen(encoded_stream_id) >= (MAX_STREAM_NAME * 3 as ::core::ffi::c_int) as size_t {
                curl_free(encoded_stream_id as *mut ::core::ffi::c_void);
            } else {
                written = snprintf(
                    url.offset(offset as isize),
                    url_buf_size.wrapping_sub(offset),
                    b"&name=%s\0".as_ptr() as *const ::core::ffi::c_char,
                    encoded_stream_id,
                );
                curl_free(encoded_stream_id as *mut ::core::ffi::c_void);
                if !(written < 0 as ::core::ffi::c_int
                    || written as size_t >= url_buf_size.wrapping_sub(offset))
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Adding go2rtc stream with %d source(s) for %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        num_sources,
                        stream_id,
                    );
                    curl_easy_setopt(curl, CURLoption::CURLOPT_URL, url);
                    curl_easy_setopt(
                        curl,
                        CURLoption::CURLOPT_CUSTOMREQUEST,
                        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    curl_easy_setopt(
                        curl,
                        CURLoption::CURLOPT_WRITEFUNCTION,
                        Some(
                            PerRequestWriteCallback
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    size_t,
                                    size_t,
                                    *mut ::core::ffi::c_void,
                                ) -> size_t,
                        ),
                    );
                    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut resp);
                    configure_curl_timeouts(curl, GO2RTC_API_MUTATION_TIMEOUT_SECONDS);
                    res = curl_easy_perform(curl);
                    if res.0 != CURLcode::CURLE_OK.0 {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"CURL request failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            curl_easy_strerror(res),
                        );
                    } else {
                        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                        curl_easy_getinfo(
                            curl,
                            CURLINFO::CURLINFO_RESPONSE_CODE,
                            &raw mut http_code,
                        );
                        if http_code == 200 as ::core::ffi::c_long {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Added stream to go2rtc with %d sources: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                num_sources,
                                stream_id,
                            );
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Response: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut resp.buffer as *mut ::core::ffi::c_char,
                            );
                            success = r#true != 0;
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Failed to add stream to go2rtc (status %ld): %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                http_code,
                                &raw mut resp.buffer as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                    free(url as *mut ::core::ffi::c_void);
                    curl_easy_cleanup(curl);
                    return success;
                }
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc request for stream %s does not fit its URL buffer\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_id,
    );
    free(url as *mut ::core::ffi::c_void);
    curl_easy_cleanup(curl);
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_remove_stream(
    mut stream_id: *const ::core::ffi::c_char,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if stream_id.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for go2rtc_api_remove_stream\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut res: CURLcode = CURLcode::CURLE_OK;
    let mut url: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut success: bool = r#false != 0;
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut resp: response_buffer_t = response_buffer_t {
        buffer: [0; 65536],
        size: 0 as size_t,
    };
    resp.buffer[0usize] = '\0' as ::core::ffi::c_char;
    let mut encoded_id: [::core::ffi::c_char; 3072] = [0; 3072];
    simple_url_escape(
        stream_id,
        &raw mut encoded_id as *mut ::core::ffi::c_char,
        (URL_BUFFER_SIZE * 3 as ::core::ffi::c_int) as size_t,
    );
    snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"http://%s:%d%s/api/streams?src=%s\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
        &raw mut encoded_id as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"DELETE URL: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CUSTOMREQUEST,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            PerRequestWriteCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut resp);
    configure_curl_timeouts(curl, GO2RTC_API_MUTATION_TIMEOUT_SECONDS);
    res = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"CURL request failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            curl_easy_strerror(res),
        );
        if res.0 == CURLcode::CURLE_OPERATION_TIMEDOUT.0 {
            let mut listed: ::core::ffi::c_int = stream_listed_in_go2rtc(stream_id);
            if listed == 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"DELETE for stream %s timed out but go2rtc no longer lists it; treating as removed\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    stream_id,
                );
                success = r#true != 0;
            } else if listed == 1 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"DELETE for stream %s timed out and go2rtc still lists it\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_id,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"DELETE for stream %s timed out and go2rtc could not be queried\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_id,
                );
            }
        }
    } else {
        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
        if http_code == 200 as ::core::ffi::c_long {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Removed stream from go2rtc: %s\0".as_ptr() as *const ::core::ffi::c_char,
                stream_id,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Response: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut resp.buffer as *mut ::core::ffi::c_char,
            );
            success = r#true != 0;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to remove stream from go2rtc (status %ld): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                http_code,
                &raw mut resp.buffer as *mut ::core::ffi::c_char,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Trying old method as fallback\0".as_ptr() as *const ::core::ffi::c_char,
            );
            resp.size = 0 as size_t;
            resp.buffer[0usize] = '\0' as ::core::ffi::c_char;
            snprintf(
                &raw mut url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                b"http://%s:%d%s/api/streams/%s\0".as_ptr() as *const ::core::ffi::c_char,
                g_api_host,
                g_api_port,
                go2rtc_api_get_base_path(),
                &raw mut encoded_id as *mut ::core::ffi::c_char,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Fallback DELETE URL: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut url as *mut ::core::ffi::c_char,
            );
            curl_easy_setopt(
                curl,
                CURLoption::CURLOPT_URL,
                &raw mut url as *mut ::core::ffi::c_char,
            );
            res = curl_easy_perform(curl);
            if res.0 != CURLcode::CURLE_OK.0 {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"CURL fallback request failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    curl_easy_strerror(res),
                );
            } else {
                curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
                if http_code == 200 as ::core::ffi::c_long {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Removed stream from go2rtc using old method: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        stream_id,
                    );
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Response: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut resp.buffer as *mut ::core::ffi::c_char,
                    );
                    success = r#true != 0;
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to remove stream from go2rtc using old method (status %ld): %s\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        http_code,
                        &raw mut resp.buffer as *mut ::core::ffi::c_char,
                    );
                }
            }
        }
    }
    curl_easy_cleanup(curl);
    return success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_publish_stream(
    mut stream_id: *const ::core::ffi::c_char,
    mut destination: *const ::core::ffi::c_char,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if stream_id.is_null()
        || destination.is_null()
        || *destination.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_api_publish_stream\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut res: CURLcode = CURLcode::CURLE_OK;
    let mut url: [::core::ffi::c_char; 3968] = [0; 3968];
    let mut success: bool = r#false != 0;
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut resp: response_buffer_t = response_buffer_t {
        buffer: [0; 65536],
        size: 0 as size_t,
    };
    resp.buffer[0usize] = '\0' as ::core::ffi::c_char;
    let mut encoded_stream_id: [::core::ffi::c_char; 768] = [0; 768];
    simple_url_escape(
        stream_id,
        &raw mut encoded_stream_id as *mut ::core::ffi::c_char,
        (MAX_STREAM_NAME * 3 as ::core::ffi::c_int) as size_t,
    );
    let mut encoded_dst: [::core::ffi::c_char; 3072] = [0; 3072];
    simple_url_escape(
        destination,
        &raw mut encoded_dst as *mut ::core::ffi::c_char,
        (URL_BUFFER_SIZE * 3 as ::core::ffi::c_int) as size_t,
    );
    snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 3968]>(),
        b"http://%s:%d%s/api/streams?src=%s&dst=%s\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
        &raw mut encoded_stream_id as *mut ::core::ffi::c_char,
        &raw mut encoded_dst as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Publishing (restreaming) go2rtc stream %s to configured RTMP destination\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_id,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CUSTOMREQUEST,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            PerRequestWriteCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut resp);
    configure_curl_timeouts(curl, GO2RTC_API_MUTATION_TIMEOUT_SECONDS);
    res = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"CURL publish request failed for stream %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_id,
            curl_easy_strerror(res),
        );
    } else {
        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
        if http_code == 200 as ::core::ffi::c_long {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc accepted publish request for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_id,
            );
            success = r#true != 0;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc rejected publish request for stream %s (status %ld): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_id,
                http_code,
                &raw mut resp.buffer as *mut ::core::ffi::c_char,
            );
        }
    }
    curl_easy_cleanup(curl);
    return success;
}
unsafe extern "C" fn publish_stream_worker(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut job: *mut publish_job_t = arg as *mut publish_job_t;
    if !go2rtc_api_publish_stream((*job).stream_id, (*job).destination) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start background RTMP restream for %s; will retry on next registration\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*job).stream_id,
        );
    }
    free((*job).stream_id as *mut ::core::ffi::c_void);
    free((*job).destination as *mut ::core::ffi::c_void);
    free(job as *mut ::core::ffi::c_void);
    pthread_mutex_lock(&raw mut g_publish_jobs_mutex);
    g_active_publish_jobs = g_active_publish_jobs.wrapping_sub(1);
    if g_active_publish_jobs == 0 as size_t {
        pthread_cond_broadcast(&raw mut g_publish_jobs_done);
    }
    pthread_mutex_unlock(&raw mut g_publish_jobs_mutex);
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_publish_stream_async(
    mut stream_id: *const ::core::ffi::c_char,
    mut destination: *const ::core::ffi::c_char,
) -> bool {
    if stream_id.is_null()
        || destination.is_null()
        || *destination.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_api_publish_stream_async\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut job: *mut publish_job_t =
        calloc(1 as size_t, ::core::mem::size_of::<publish_job_t>()) as *mut publish_job_t;
    if job.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate background RTMP publish job\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    (*job).stream_id = strdup(stream_id);
    (*job).destination = strdup(destination);
    if (*job).stream_id.is_null() || (*job).destination.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to copy background RTMP publish job parameters\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free((*job).stream_id as *mut ::core::ffi::c_void);
        free((*job).destination as *mut ::core::ffi::c_void);
        free(job as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut g_publish_jobs_mutex);
    if !g_initialized || !g_accept_publish_jobs {
        pthread_mutex_unlock(&raw mut g_publish_jobs_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client is not accepting RTMP publish jobs\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free((*job).stream_id as *mut ::core::ffi::c_void);
        free((*job).destination as *mut ::core::ffi::c_void);
        free(job as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    let mut thread: pthread_t = 0;
    g_active_publish_jobs = g_active_publish_jobs.wrapping_add(1);
    let mut create_result: ::core::ffi::c_int = pthread_create(
        &raw mut thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            publish_stream_worker
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        job as *mut ::core::ffi::c_void,
    );
    if create_result != 0 as ::core::ffi::c_int {
        g_active_publish_jobs = g_active_publish_jobs.wrapping_sub(1);
        pthread_mutex_unlock(&raw mut g_publish_jobs_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create background RTMP publish thread: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(create_result),
        );
        free((*job).stream_id as *mut ::core::ffi::c_void);
        free((*job).destination as *mut ::core::ffi::c_void);
        free(job as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    let mut detach_result: ::core::ffi::c_int = pthread_detach(thread);
    if detach_result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to detach background RTMP publish thread: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(detach_result),
        );
    }
    pthread_mutex_unlock(&raw mut g_publish_jobs_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Scheduled background RTMP restream for go2rtc stream %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_id,
    );
    return r#true != 0;
}
unsafe extern "C" fn stream_listed_in_go2rtc(
    mut stream_id: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if stream_id.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for go2rtc_api_stream_exists\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut res: CURLcode = CURLcode::CURLE_OK;
    let mut url: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut exists: bool = r#false != 0;
    let mut queried: bool = r#false != 0;
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut chunk: MemoryStruct = MemoryStruct {
        memory: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0 as size_t,
    };
    snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"http://%s:%d%s/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            WriteMemoryCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut chunk);
    configure_curl_timeouts(curl, GO2RTC_API_READ_TIMEOUT_SECONDS);
    res = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"CURL request failed for stream_exists: %s\0".as_ptr() as *const ::core::ffi::c_char,
            curl_easy_strerror(res),
        );
    } else {
        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
        if http_code == 200 as ::core::ffi::c_long && !chunk.memory.is_null() {
            let mut json: *mut cJSON = cJSON_Parse(chunk.memory);
            if !json.is_null() {
                exists = cJSON_HasObjectItem(json, stream_id) != 0;
                queried = r#true != 0;
                cJSON_Delete(json);
                if exists {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Stream %s exists in go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
                        stream_id,
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Stream %s not found in go2rtc JSON response (stream not a key)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        stream_id,
                    );
                }
            } else {
                let mut preview: [::core::ffi::c_char; 201] = [0; 201];
                safe_strcpy(
                    &raw mut preview as *mut ::core::ffi::c_char,
                    chunk.memory,
                    201 as size_t,
                    0 as size_t,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to parse go2rtc /api/streams response as JSON. Body preview (first 200 chars): %.200s\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut preview as *mut ::core::ffi::c_char,
                );
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc /api/streams returned status=%ld for stream %s check\0".as_ptr()
                    as *const ::core::ffi::c_char,
                http_code,
                stream_id,
            );
        }
    }
    curl_easy_cleanup(curl);
    free(chunk.memory as *mut ::core::ffi::c_void);
    if !queried {
        return -1 as ::core::ffi::c_int;
    }
    if !exists {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found in go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
            stream_id,
        );
    }
    return if exists as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_stream_exists(
    mut stream_id: *const ::core::ffi::c_char,
) -> bool {
    return stream_listed_in_go2rtc(stream_id) == 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_delete_preload(
    mut stream_id: *const ::core::ffi::c_char,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if stream_id.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for go2rtc_api_delete_preload\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut curl: *mut ::core::ffi::c_void = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut resp: response_buffer_t = response_buffer_t {
        buffer: [0; 65536],
        size: 0 as size_t,
    };
    resp.buffer[0usize] = '\0' as ::core::ffi::c_char;
    let mut encoded_id: [::core::ffi::c_char; 3072] = [0; 3072];
    simple_url_escape(
        stream_id,
        &raw mut encoded_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 3072]>(),
    );
    let mut url: [::core::ffi::c_char; 1024] = [0; 1024];
    snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"http://%s:%d%s/api/preload?src=%s\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
        &raw mut encoded_id as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CUSTOMREQUEST,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            PerRequestWriteCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut resp);
    configure_curl_timeouts(curl, GO2RTC_API_MUTATION_TIMEOUT_SECONDS);
    let mut detached: bool = r#false != 0;
    let mut res: CURLcode = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"CURL request failed for delete_preload %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_id,
            curl_easy_strerror(res),
        );
    } else {
        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
        if http_code == 200 as ::core::ffi::c_long {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Detached go2rtc preload consumer from stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_id,
            );
            detached = r#true != 0;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"No go2rtc preload to detach for stream %s (status %ld): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_id,
                http_code,
                &raw mut resp.buffer as *mut ::core::ffi::c_char,
            );
        }
    }
    curl_easy_cleanup(curl);
    return detached;
}
unsafe extern "C" fn json_counter_is_positive(
    mut object: *const cJSON,
    mut key: *const ::core::ffi::c_char,
) -> bool {
    let mut counter: *const cJSON = cJSON_GetObjectItemCaseSensitive(object, key);
    return cJSON_IsNumber(counter) != 0 && (*counter).valuedouble > 0.0f64;
}
unsafe extern "C" fn producer_has_received_video(mut producer: *const cJSON) -> bool {
    if cJSON_IsObject(producer) == 0 {
        return r#false != 0;
    }
    let mut receivers: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        producer,
        b"receivers\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsArray(receivers) != 0 {
        let mut receiver: *const cJSON = ::core::ptr::null::<cJSON>();
        receiver = if !receivers.is_null() {
            (*receivers).child
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
        while !receiver.is_null() {
            let mut codec: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                receiver,
                b"codec\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut codec_type: *const cJSON = if !codec.is_null() {
                cJSON_GetObjectItemCaseSensitive(
                    codec,
                    b"codec_type\0".as_ptr() as *const ::core::ffi::c_char,
                )
            } else {
                ::core::ptr::null_mut::<cJSON>()
            };
            if cJSON_IsString(codec_type) != 0
                && strcmp(
                    (*codec_type).valuestring,
                    b"video\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                && (json_counter_is_positive(
                    receiver,
                    b"packets\0".as_ptr() as *const ::core::ffi::c_char,
                ) as ::core::ffi::c_int
                    != 0
                    || json_counter_is_positive(
                        receiver,
                        b"bytes\0".as_ptr() as *const ::core::ffi::c_char,
                    ) as ::core::ffi::c_int
                        != 0)
            {
                return r#true != 0;
            }
            receiver = (*receiver).next;
        }
    }
    if !json_counter_is_positive(
        producer,
        b"bytes_recv\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        return r#false != 0;
    }
    let mut medias: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        producer,
        b"medias\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsArray(medias) == 0 {
        return r#false != 0;
    }
    let mut media: *const cJSON = ::core::ptr::null::<cJSON>();
    media = if !medias.is_null() {
        (*medias).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !media.is_null() {
        if cJSON_IsString(media) != 0
            && !strstr(
                (*media).valuestring,
                b"video\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            return r#true != 0;
        }
        media = (*media).next;
    }
    return r#false != 0;
}
unsafe extern "C" fn stream_has_received_video(mut stream: *const cJSON) -> bool {
    if cJSON_IsObject(stream) == 0 {
        return r#false != 0;
    }
    let mut producers: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        stream,
        b"producers\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsArray(producers) == 0 {
        return r#false != 0;
    }
    let mut producer: *const cJSON = ::core::ptr::null::<cJSON>();
    producer = if !producers.is_null() {
        (*producers).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !producer.is_null() {
        if producer_has_received_video(producer) {
            return r#true != 0;
        }
        producer = (*producer).next;
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_parse_stream_activity_json(
    mut json: *const ::core::ffi::c_char,
    mut streams: *mut go2rtc_stream_activity_t,
    mut stream_count: size_t,
) -> bool {
    if json.is_null() || streams.is_null() && stream_count > 0 as size_t {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < stream_count {
        (*streams.offset(i as isize)).video_active = r#false != 0;
        i = i.wrapping_add(1);
    }
    let mut root: *mut cJSON = cJSON_Parse(json);
    if cJSON_IsObject(root) == 0 {
        cJSON_Delete(root);
        return r#false != 0;
    }
    let mut i_0: size_t = 0 as size_t;
    while i_0 < stream_count {
        if !((*streams.offset(i_0 as isize)).stream_name.is_null()
            || *(*streams.offset(i_0 as isize)).stream_name.offset(0isize) as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int)
        {
            let mut stream: *const cJSON =
                cJSON_GetObjectItemCaseSensitive(root, (*streams.offset(i_0 as isize)).stream_name);
            (*streams.offset(i_0 as isize)).video_active = stream_has_received_video(stream);
            if !(*streams.offset(i_0 as isize)).video_active
                && !(*streams.offset(i_0 as isize))
                    .alternate_stream_name
                    .is_null()
                && *(*streams.offset(i_0 as isize))
                    .alternate_stream_name
                    .offset(0isize) as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
            {
                stream = cJSON_GetObjectItemCaseSensitive(
                    root,
                    (*streams.offset(i_0 as isize)).alternate_stream_name,
                );
                (*streams.offset(i_0 as isize)).video_active = stream_has_received_video(stream);
            }
        }
        i_0 = i_0.wrapping_add(1);
    }
    cJSON_Delete(root);
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_get_stream_activity(
    mut streams: *mut go2rtc_stream_activity_t,
    mut stream_count: size_t,
) -> bool {
    if streams.is_null() && stream_count > 0 as size_t {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < stream_count {
        (*streams.offset(i as isize)).video_active = r#false != 0;
        i = i.wrapping_add(1);
    }
    if stream_count == 0 as size_t {
        return r#true != 0;
    }
    if !g_initialized {
        return r#false != 0;
    }
    let mut curl: *mut ::core::ffi::c_void = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL for go2rtc stream activity\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut url: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut response: MemoryStruct = MemoryStruct {
        memory: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0 as size_t,
    };
    snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"http://%s:%d%s/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            WriteMemoryCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut response);
    configure_curl_timeouts(curl, GO2RTC_API_READ_TIMEOUT_SECONDS);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_TIMEOUT_MS,
        500 as ::core::ffi::c_long,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CONNECTTIMEOUT_MS,
        500 as ::core::ffi::c_long,
    );
    let mut result: CURLcode = curl_easy_perform(curl);
    let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    if result.0 == CURLcode::CURLE_OK.0 {
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
    }
    let mut success: bool = result.0 == CURLcode::CURLE_OK.0
        && http_code == 200 as ::core::ffi::c_long
        && !response.memory.is_null()
        && go2rtc_api_parse_stream_activity_json(response.memory, streams, stream_count)
            as ::core::ffi::c_int
            != 0;
    if !success {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not read go2rtc stream activity (curl=%d, status=%ld)\0".as_ptr()
                as *const ::core::ffi::c_char,
            result.0 as ::core::ffi::c_int,
            http_code,
        );
    }
    free(response.memory as *mut ::core::ffi::c_void);
    curl_easy_cleanup(curl);
    return success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_get_webrtc_url(
    mut stream_id: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if stream_id.is_null() || buffer.is_null() || buffer_size == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_api_get_webrtc_url\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut encoded_stream_id: [::core::ffi::c_char; 768] = [0; 768];
    simple_url_escape(
        stream_id,
        &raw mut encoded_stream_id as *mut ::core::ffi::c_char,
        (MAX_STREAM_NAME * 3 as ::core::ffi::c_int) as size_t,
    );
    snprintf(
        buffer,
        buffer_size,
        b"http://%s:%d%s/webrtc/%s\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
        &raw mut encoded_stream_id as *mut ::core::ffi::c_char,
    );
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_update_config() -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut res: CURLcode = CURLcode::CURLE_OK;
    let mut url: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut success: bool = r#false != 0;
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut resp: response_buffer_t = response_buffer_t {
        buffer: [0; 65536],
        size: 0 as size_t,
    };
    resp.buffer[0usize] = '\0' as ::core::ffi::c_char;
    snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"http://%s:%d%s/api/config\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            PerRequestWriteCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut resp);
    configure_curl_timeouts(curl, GO2RTC_API_READ_TIMEOUT_SECONDS);
    res = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"CURL request failed for update_config: %s\0".as_ptr() as *const ::core::ffi::c_char,
            curl_easy_strerror(res),
        );
    } else {
        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
        if http_code == 200 as ::core::ffi::c_long {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Successfully updated go2rtc configuration\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            success = r#true != 0;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to update go2rtc configuration (status %ld): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                http_code,
                &raw mut resp.buffer as *mut ::core::ffi::c_char,
            );
        }
    }
    curl_easy_cleanup(curl);
    return success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_get_streams(
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if buffer.is_null() || buffer_size == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_api_get_streams\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut res: CURLcode = CURLcode::CURLE_OK;
    let mut url: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut success: bool = r#false != 0;
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut resp: response_buffer_t = response_buffer_t {
        buffer: [0; 65536],
        size: 0 as size_t,
    };
    resp.buffer[0usize] = '\0' as ::core::ffi::c_char;
    snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"http://%s:%d%s/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            PerRequestWriteCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut resp);
    configure_curl_timeouts(curl, GO2RTC_API_READ_TIMEOUT_SECONDS);
    res = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"CURL request failed for get_streams: %s\0".as_ptr() as *const ::core::ffi::c_char,
            curl_easy_strerror(res),
        );
    } else {
        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
        if http_code == 200 as ::core::ffi::c_long {
            safe_strcpy(
                buffer,
                &raw mut resp.buffer as *mut ::core::ffi::c_char,
                buffer_size,
                0 as size_t,
            );
            success = r#true != 0;
        }
    }
    curl_easy_cleanup(curl);
    return success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_get_application_info(
    mut rtsp_port: *mut ::core::ffi::c_int,
    mut version: *mut ::core::ffi::c_char,
    mut version_size: size_t,
    mut revision: *mut ::core::ffi::c_char,
    mut revision_size: size_t,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !version.is_null() && version_size > 0 as size_t {
        *version.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !revision.is_null() && revision_size > 0 as size_t {
        *revision.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut res: CURLcode = CURLcode::CURLE_OK;
    let mut url: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut success: bool = r#false != 0;
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut resp: response_buffer_t = response_buffer_t {
        buffer: [0; 65536],
        size: 0 as size_t,
    };
    resp.buffer[0usize] = '\0' as ::core::ffi::c_char;
    snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"http://%s:%d%s/api\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            PerRequestWriteCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut resp);
    configure_curl_timeouts(curl, GO2RTC_API_READ_TIMEOUT_SECONDS);
    res = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"CURL request failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            curl_easy_strerror(res),
        );
    } else {
        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
        if http_code == 200 as ::core::ffi::c_long {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Got go2rtc server info: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut resp.buffer as *mut ::core::ffi::c_char,
            );
            let mut json: *mut cJSON =
                cJSON_Parse(&raw mut resp.buffer as *mut ::core::ffi::c_char);
            if !json.is_null() {
                if !version.is_null() && version_size > 0 as size_t {
                    let mut version_obj: *mut cJSON = cJSON_GetObjectItem(
                        json,
                        b"version\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if !version_obj.is_null() && cJSON_IsString(version_obj) != 0 {
                        safe_strcpy(
                            version,
                            cJSON_GetStringValue(version_obj),
                            version_size,
                            0 as size_t,
                        );
                    }
                }
                if !revision.is_null() && revision_size > 0 as size_t {
                    let mut revision_obj: *mut cJSON = cJSON_GetObjectItem(
                        json,
                        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if !revision_obj.is_null() && cJSON_IsString(revision_obj) != 0 {
                        safe_strcpy(
                            revision,
                            cJSON_GetStringValue(revision_obj),
                            revision_size,
                            0 as size_t,
                        );
                    }
                }
                if !rtsp_port.is_null() {
                    let mut rtsp: *mut cJSON =
                        cJSON_GetObjectItem(json, b"rtsp\0".as_ptr() as *const ::core::ffi::c_char);
                    if !rtsp.is_null() && cJSON_IsObject(rtsp) != 0 {
                        let mut listen: *mut cJSON = cJSON_GetObjectItem(
                            rtsp,
                            b"listen\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !listen.is_null() && cJSON_IsString(listen) != 0 {
                            let mut listen_str: *const ::core::ffi::c_char =
                                cJSON_GetStringValue(listen);
                            let mut separator: *const ::core::ffi::c_char = if !listen_str.is_null()
                            {
                                strrchr(listen_str, ':' as ::core::ffi::c_int)
                            } else {
                                ::core::ptr::null_mut::<::core::ffi::c_char>()
                            };
                            let mut port_text: *const ::core::ffi::c_char = if !separator.is_null()
                            {
                                separator.offset(1 as ::core::ffi::c_int as isize)
                            } else {
                                b"\0".as_ptr() as *const ::core::ffi::c_char
                            };
                            let mut digits: size_t = strspn(
                                port_text,
                                b"0123456789\0".as_ptr() as *const ::core::ffi::c_char,
                            ) as size_t;
                            let mut port: ::core::ffi::c_long = if digits > 0 as size_t
                                && digits <= 5 as size_t
                                && *port_text.offset(digits as isize) as ::core::ffi::c_int
                                    == '\0' as ::core::ffi::c_int
                            {
                                strtol(
                                    port_text,
                                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                    10 as ::core::ffi::c_int,
                                )
                            } else {
                                0 as ::core::ffi::c_long
                            };
                            if port > 0 as ::core::ffi::c_long
                                && port <= 65535 as ::core::ffi::c_long
                            {
                                *rtsp_port = port as ::core::ffi::c_int;
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Extracted RTSP port: %d\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    *rtsp_port,
                                );
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Invalid RTSP listen format: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    if !listen_str.is_null() {
                                        listen_str
                                    } else {
                                        b"NULL\0".as_ptr() as *const ::core::ffi::c_char
                                    },
                                );
                                *rtsp_port = 8554 as ::core::ffi::c_int;
                            }
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"RTSP listen not found in server info\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            *rtsp_port = 8554 as ::core::ffi::c_int;
                        }
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"RTSP section not found in server info\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        *rtsp_port = 8554 as ::core::ffi::c_int;
                    }
                }
                cJSON_Delete(json);
                success = r#true != 0;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to parse server info JSON: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut resp.buffer as *mut ::core::ffi::c_char,
                );
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to get go2rtc server info (status %ld): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                http_code,
                &raw mut resp.buffer as *mut ::core::ffi::c_char,
            );
        }
    }
    curl_easy_cleanup(curl);
    return success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_get_server_info(
    mut rtsp_port: *mut ::core::ffi::c_int,
) -> bool {
    return go2rtc_api_get_application_info(
        rtsp_port,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
    );
}
unsafe extern "C" fn preload_attempt(
    mut stream_id: *const ::core::ffi::c_char,
    mut query: *const ::core::ffi::c_char,
    mut timeout_sec: ::core::ffi::c_long,
) -> bool {
    let mut curl: *mut ::core::ffi::c_void = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL for preload attempt\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut encoded_id: [::core::ffi::c_char; 3072] = [0; 3072];
    simple_url_escape(
        stream_id,
        &raw mut encoded_id as *mut ::core::ffi::c_char,
        (URL_BUFFER_SIZE * 3 as ::core::ffi::c_int) as size_t,
    );
    let mut url: [::core::ffi::c_char; 1024] = [0; 1024];
    snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"http://%s:%d%s/api/preload?src=%s&%s\0".as_ptr() as *const ::core::ffi::c_char,
        g_api_host,
        g_api_port,
        go2rtc_api_get_base_path(),
        &raw mut encoded_id as *mut ::core::ffi::c_char,
        query,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Preloading stream with URL: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    let mut resp: response_buffer_t = response_buffer_t {
        buffer: [0; 65536],
        size: 0 as size_t,
    };
    resp.buffer[0usize] = '\0' as ::core::ffi::c_char;
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CUSTOMREQUEST,
        b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            PerRequestWriteCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut resp);
    configure_curl_timeouts(curl, timeout_sec);
    let mut success: bool = r#false != 0;
    let mut res: CURLcode = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"CURL preload attempt (%s) failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            query,
            curl_easy_strerror(res),
        );
    } else {
        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
        if http_code == 200 as ::core::ffi::c_long {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Successfully preloaded stream in go2rtc: %s (query=%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_id,
                query,
            );
            success = r#true != 0;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to preload stream in go2rtc (status %ld, query=%s): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                http_code,
                query,
                &raw mut resp.buffer as *mut ::core::ffi::c_char,
            );
        }
    }
    curl_easy_cleanup(curl);
    return success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_preload_stream(
    mut stream_id: *const ::core::ffi::c_char,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc API client not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if stream_id.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_api_preload_stream\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return preload_attempt(
        stream_id,
        b"video\0".as_ptr() as *const ::core::ffi::c_char,
        10 as ::core::ffi::c_long,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_api_cleanup() {
    if !g_initialized {
        return;
    }
    pthread_mutex_lock(&raw mut g_publish_jobs_mutex);
    g_accept_publish_jobs = r#false != 0;
    if g_active_publish_jobs > 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Waiting for %zu background RTMP publish job(s) to finish\0".as_ptr()
                as *const ::core::ffi::c_char,
            g_active_publish_jobs,
        );
    }
    while g_active_publish_jobs > 0 as size_t {
        pthread_cond_wait(&raw mut g_publish_jobs_done, &raw mut g_publish_jobs_mutex);
    }
    pthread_mutex_unlock(&raw mut g_publish_jobs_mutex);
    free(g_api_host as *mut ::core::ffi::c_void);
    g_api_host = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_api_port = 0 as ::core::ffi::c_int;
    g_initialized = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc API client cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
