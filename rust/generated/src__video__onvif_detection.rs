unsafe extern "C" {
    pub type Curl_URL;
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
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn strptime(
        __s: *const ::core::ffi::c_char,
        __fmt: *const ::core::ffi::c_char,
        __tp: *mut tm,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn timegm(__tp: *mut tm) -> time_t;
    unsafe fn curl_free(p: *mut ::core::ffi::c_void);
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
    unsafe fn curl_url() -> *mut CURLU;
    unsafe fn curl_url_cleanup(u: *mut CURLU);
    unsafe fn curl_url_get(
        u: *const CURLU,
        what: CURLUPart,
        part: *mut *mut ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> CURLUcode;
    unsafe fn curl_url_set(
        u: *mut CURLU,
        what: CURLUPart,
        part: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> CURLUcode;
    unsafe fn url_is_http_endpoint(url: *const ::core::ffi::c_char) -> bool;
    unsafe fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn curl_init_global() -> ::core::ffi::c_int;
    unsafe fn is_shutdown_initiated() -> bool;
    unsafe fn event_producer_publish_detection_for_stream(
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_producer_publish_lpr_read(
        camera_uuid: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
        read_uuid: *const ::core::ffi::c_char,
        source: *const ::core::ffi::c_char,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_parse_lpr_events(
        xml_text: *const ::core::ffi::c_char,
        events: *mut onvif_lpr_event_t,
        capacity: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_create_security_header(
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn onvif_log_soap_fault(
        response: *const ::core::ffi::c_char,
        response_len: size_t,
        context: *const ::core::ffi::c_char,
    );
    unsafe fn process_motion_event(
        stream_name: *const ::core::ffi::c_char,
        motion_detected: bool,
        timestamp: time_t,
        is_propagated: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn get_current_recording_id_for_stream(
        stream_name: *const ::core::ffi::c_char,
    ) -> uint64_t;
    unsafe fn filter_detections_by_zones(
        stream_name: *const ::core::ffi::c_char,
        result: *mut detection_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn store_detections_in_db(
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
        timestamp: time_t,
        recording_id: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lpr_crypto_key_available() -> ::core::ffi::c_int;
    unsafe fn ezxml_parse_str(s: *mut ::core::ffi::c_char, len: size_t) -> ezxml_t;
    unsafe fn ezxml_attr(
        xml: ezxml_t,
        attr: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    unsafe fn ezxml_toxml(xml: ezxml_t) -> *mut ::core::ffi::c_char;
    unsafe fn ezxml_free(xml: ezxml_t);
    unsafe fn ezxml_error(xml: ezxml_t) -> *const ::core::ffi::c_char;
    unsafe fn ezxml_new(name: *const ::core::ffi::c_char) -> ezxml_t;
    unsafe fn ezxml_set_txt(xml: ezxml_t, txt: *const ::core::ffi::c_char) -> ezxml_t;
    unsafe fn db_lpr_read_insert(
        input: *const lpr_read_input_t,
        uuid_out: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_lpr_read_from_onvif(
        camera_uuid: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
        event: *const onvif_lpr_event_t,
        input: *mut lpr_read_input_t,
    ) -> ::core::ffi::c_int;
    unsafe fn generate_uuid(uuid: *mut ::core::ffi::c_char, size: size_t);
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strndup(
        __string: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
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
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type time_t = __time_t;
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
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
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
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub type CURL = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_slist {
    pub data: *mut ::core::ffi::c_char,
    pub next: *mut curl_slist,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct curl_infotype(pub ::core::ffi::c_uint);
impl curl_infotype {
    pub const CURLINFO_TEXT: Self = Self(0);
    pub const CURLINFO_HEADER_IN: Self = Self(1);
    pub const CURLINFO_HEADER_OUT: Self = Self(2);
    pub const CURLINFO_DATA_IN: Self = Self(3);
    pub const CURLINFO_DATA_OUT: Self = Self(4);
    pub const CURLINFO_SSL_DATA_IN: Self = Self(5);
    pub const CURLINFO_SSL_DATA_OUT: Self = Self(6);
    pub const CURLINFO_END: Self = Self(7);
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
pub struct CURLUcode(pub ::core::ffi::c_uint);
impl CURLUcode {
    pub const CURLUE_OK: Self = Self(0);
    pub const CURLUE_BAD_HANDLE: Self = Self(1);
    pub const CURLUE_BAD_PARTPOINTER: Self = Self(2);
    pub const CURLUE_MALFORMED_INPUT: Self = Self(3);
    pub const CURLUE_BAD_PORT_NUMBER: Self = Self(4);
    pub const CURLUE_UNSUPPORTED_SCHEME: Self = Self(5);
    pub const CURLUE_URLDECODE: Self = Self(6);
    pub const CURLUE_OUT_OF_MEMORY: Self = Self(7);
    pub const CURLUE_USER_NOT_ALLOWED: Self = Self(8);
    pub const CURLUE_UNKNOWN_PART: Self = Self(9);
    pub const CURLUE_NO_SCHEME: Self = Self(10);
    pub const CURLUE_NO_USER: Self = Self(11);
    pub const CURLUE_NO_PASSWORD: Self = Self(12);
    pub const CURLUE_NO_OPTIONS: Self = Self(13);
    pub const CURLUE_NO_HOST: Self = Self(14);
    pub const CURLUE_NO_PORT: Self = Self(15);
    pub const CURLUE_NO_QUERY: Self = Self(16);
    pub const CURLUE_NO_FRAGMENT: Self = Self(17);
    pub const CURLUE_NO_ZONEID: Self = Self(18);
    pub const CURLUE_BAD_FILE_URL: Self = Self(19);
    pub const CURLUE_BAD_FRAGMENT: Self = Self(20);
    pub const CURLUE_BAD_HOSTNAME: Self = Self(21);
    pub const CURLUE_BAD_IPV6: Self = Self(22);
    pub const CURLUE_BAD_LOGIN: Self = Self(23);
    pub const CURLUE_BAD_PASSWORD: Self = Self(24);
    pub const CURLUE_BAD_PATH: Self = Self(25);
    pub const CURLUE_BAD_QUERY: Self = Self(26);
    pub const CURLUE_BAD_SCHEME: Self = Self(27);
    pub const CURLUE_BAD_SLASHES: Self = Self(28);
    pub const CURLUE_BAD_USER: Self = Self(29);
    pub const CURLUE_LACKS_IDN: Self = Self(30);
    pub const CURLUE_TOO_LARGE: Self = Self(31);
    pub const CURLUE_BACKSLASH: Self = Self(32);
    pub const CURLUE_LAST: Self = Self(33);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CURLUPart(pub ::core::ffi::c_uint);
impl CURLUPart {
    pub const CURLUPART_URL: Self = Self(0);
    pub const CURLUPART_SCHEME: Self = Self(1);
    pub const CURLUPART_USER: Self = Self(2);
    pub const CURLUPART_PASSWORD: Self = Self(3);
    pub const CURLUPART_OPTIONS: Self = Self(4);
    pub const CURLUPART_HOST: Self = Self(5);
    pub const CURLUPART_PORT: Self = Self(6);
    pub const CURLUPART_PATH: Self = Self(7);
    pub const CURLUPART_QUERY: Self = Self(8);
    pub const CURLUPART_FRAGMENT: Self = Self(9);
    pub const CURLUPART_ZONEID: Self = Self(10);
}
pub type CURLU = Curl_URL;
pub type ptrdiff_t = isize;
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
pub type uint8_t = u8;
pub type uint64_t = u64;
pub type atomic_bool = bool;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_t {
    pub label: [::core::ffi::c_char; 32],
    pub confidence: ::core::ffi::c_float,
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub width: ::core::ffi::c_float,
    pub height: ::core::ffi::c_float,
    pub track_id: ::core::ffi::c_int,
    pub zone_id: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_result_t {
    pub count: ::core::ffi::c_int,
    pub detections: [detection_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_subscription_t {
    pub camera_url: [::core::ffi::c_char; 512],
    pub subscription_address: [::core::ffi::c_char; 512],
    pub username: [::core::ffi::c_char; 64],
    pub password: [::core::ffi::c_char; 64],
    pub creation_time: time_t,
    pub renewal_time: time_t,
    pub expiration_time: time_t,
    pub first_pull_failure_time: time_t,
    pub consecutive_pull_failures: ::core::ffi::c_int,
    pub active: bool,
    pub curl: *mut ::core::ffi::c_void,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct onvif_operation_t(pub ::core::ffi::c_uint);
impl onvif_operation_t {
    pub const ONVIF_OPERATION_GET_SERVICES: Self = Self(0);
    pub const ONVIF_OPERATION_CREATE_PULLPOINT: Self = Self(1);
    pub const ONVIF_OPERATION_PULL_MESSAGES: Self = Self(2);
    pub const ONVIF_OPERATION_RENEW: Self = Self(3);
    pub const ONVIF_OPERATION_UNSUBSCRIBE: Self = Self(4);
    pub const ONVIF_OPERATION_COUNT: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_request_status_t {
    pub curl_code: CURLcode,
    pub http_code: ::core::ffi::c_long,
    pub elapsed: ::core::ffi::c_double,
    pub new_connections: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memory_struct_t {
    pub memory: *mut ::core::ffi::c_char,
    pub size: size_t,
}
pub type ezxml_t = *mut ezxml;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ezxml {
    pub name: *mut ::core::ffi::c_char,
    pub attr: *mut *mut ::core::ffi::c_char,
    pub txt: *mut ::core::ffi::c_char,
    pub off: size_t,
    pub next: ezxml_t,
    pub sibling: ezxml_t,
    pub ordered: ezxml_t,
    pub child: ezxml_t,
    pub parent: ezxml_t,
    pub flags: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_lpr_event_t {
    pub source: onvif_lpr_source_t,
    pub asserted: bool,
    pub observed_at_ms: int64_t,
    pub topic: [::core::ffi::c_char; 256],
    pub plate: [::core::ffi::c_char; 64],
    pub has_confidence: bool,
    pub confidence: ::core::ffi::c_float,
    pub country: [::core::ffi::c_char; 64],
    pub region: [::core::ffi::c_char; 64],
    pub plate_type: [::core::ffi::c_char; 64],
    pub direction: [::core::ffi::c_char; 64],
    pub lane: [::core::ffi::c_char; 64],
    pub vehicle_type: [::core::ffi::c_char; 64],
    pub vehicle_color: [::core::ffi::c_char; 64],
    pub object_id: [::core::ffi::c_char; 96],
    pub correlation_id: [::core::ffi::c_char; 96],
    pub has_bounding_box: bool,
    pub bbox_left: ::core::ffi::c_float,
    pub bbox_top: ::core::ffi::c_float,
    pub bbox_right: ::core::ffi::c_float,
    pub bbox_bottom: ::core::ffi::c_float,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct onvif_lpr_source_t(pub ::core::ffi::c_uint);
impl onvif_lpr_source_t {
    pub const ONVIF_LPR_SOURCE_PROFILE_M: Self = Self(0);
    pub const ONVIF_LPR_SOURCE_VENDOR_TOPIC: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lpr_read_input_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub stream_name: [::core::ffi::c_char; 256],
    pub observed_at_ms: int64_t,
    pub source: [::core::ffi::c_char; 32],
    pub vendor_topic: [::core::ffi::c_char; 256],
    pub plate: [::core::ffi::c_char; 64],
    pub has_confidence: bool,
    pub confidence: ::core::ffi::c_float,
    pub country: [::core::ffi::c_char; 64],
    pub region: [::core::ffi::c_char; 64],
    pub plate_type: [::core::ffi::c_char; 64],
    pub direction: [::core::ffi::c_char; 64],
    pub lane: [::core::ffi::c_char; 64],
    pub vehicle_type: [::core::ffi::c_char; 64],
    pub vehicle_color: [::core::ffi::c_char; 64],
    pub object_id: [::core::ffi::c_char; 96],
    pub correlation_id: [::core::ffi::c_char; 96],
    pub has_bounding_box: bool,
    pub bbox_left: ::core::ffi::c_float,
    pub bbox_top: ::core::ffi::c_float,
    pub bbox_right: ::core::ffi::c_float,
    pub bbox_bottom: ::core::ffi::c_float,
    pub recording_id: uint64_t,
}
pub const CURLU_DEFAULT_PORT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PTHREAD_ONCE_INIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_LABEL_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const ONVIF_PULL_DROP_MAX_ATTEMPTS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ONVIF_PULL_DROP_RETRY_DELAY_MS: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
pub const ONVIF_DETECT_RETRY: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn onvif_operation_action(
    mut operation: onvif_operation_t,
) -> *const ::core::ffi::c_char {
    static mut actions: [*const ::core::ffi::c_char; 5] = [
        b"http://www.onvif.org/ver10/device/wsdl/GetServices\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"http://www.onvif.org/ver10/events/wsdl/EventPortType/CreatePullPointSubscriptionRequest\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"http://www.onvif.org/ver10/events/wsdl/PullPointSubscription/PullMessagesRequest\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"http://docs.oasis-open.org/wsn/bw-2/SubscriptionManager/RenewRequest\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"http://docs.oasis-open.org/wsn/bw-2/SubscriptionManager/UnsubscribeRequest\0".as_ptr()
            as *const ::core::ffi::c_char,
    ];
    return if operation.0 < onvif_operation_t::ONVIF_OPERATION_COUNT.0 {
        actions[operation.0 as usize]
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
pub const ONVIF_SUBSCRIPTION_LEASE_SECONDS: ::core::ffi::c_int = 600 as ::core::ffi::c_int;
pub const ONVIF_RENEW_NUMERATOR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ONVIF_RENEW_DENOMINATOR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ONVIF_PULL_FAILURE_WARN_COUNT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ONVIF_PULL_FAILURE_REFRESH_SECONDS: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
static mut initialized: bool = r#false != 0;
static mut curl_handle: *mut ::core::ffi::c_void = NULL;
static mut curl_mutex: pthread_mutex_t = pthread_mutex_t {
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
pub const ONVIF_RESPONSE_MAX_BYTES: ::core::ffi::c_uint = (4 as ::core::ffi::c_uint)
    .wrapping_mul(1024 as ::core::ffi::c_uint)
    .wrapping_mul(1024 as ::core::ffi::c_uint);
unsafe extern "C" fn secure_clear(mut data: *mut ::core::ffi::c_void, mut size: size_t) {
    let mut bytes: *mut ::core::ffi::c_uchar = data as *mut ::core::ffi::c_uchar;
    while !bytes.is_null() && {
        let c2rust_fresh2 = size;
        size = size.wrapping_sub(1);
        c2rust_fresh2 > 0 as size_t
    } {
        let c2rust_fresh3 = bytes;
        bytes = bytes.offset(1);
        ::core::ptr::write_volatile(c2rust_fresh3, 0 as ::core::ffi::c_uchar);
    }
}
pub const MAX_SUBSCRIPTIONS: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
static mut subscriptions: [onvif_subscription_t; 100] = [onvif_subscription_t {
    camera_url: [0; 512],
    subscription_address: [0; 512],
    username: [0; 64],
    password: [0; 64],
    creation_time: 0,
    renewal_time: 0,
    expiration_time: 0,
    first_pull_failure_time: 0,
    consecutive_pull_failures: 0,
    active: false,
    curl: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 100];
static mut subscription_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut subscription_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut subscriptions_shutting_down: bool = r#false != 0;
static mut subscription_locks: [pthread_mutex_t; 100] = [pthread_mutex_t {
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
}; 100];
static mut subscription_locks_once: pthread_once_t = PTHREAD_ONCE_INIT;
unsafe extern "C" fn init_subscription_locks() {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_SUBSCRIPTIONS {
        pthread_mutex_init(
            (&raw mut subscription_locks as *mut pthread_mutex_t).offset(i as isize),
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
        i += 1;
    }
}
unsafe extern "C" fn subscription_lock(
    mut subscription: *const onvif_subscription_t,
) -> *mut pthread_mutex_t {
    let mut index: ptrdiff_t =
        subscription.offset_from(&raw mut subscriptions as *mut onvif_subscription_t);
    if index < 0 as ptrdiff_t || index >= MAX_SUBSCRIPTIONS as ptrdiff_t {
        return ::core::ptr::null_mut::<pthread_mutex_t>();
    }
    pthread_once(
        &raw mut subscription_locks_once,
        Some(init_subscription_locks as unsafe extern "C" fn() -> ()),
    );
    return (&raw mut subscription_locks as *mut pthread_mutex_t).offset(index as isize);
}
unsafe extern "C" fn write_memory_callback(
    mut contents: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> size_t {
    if size != 0 as size_t && nmemb > (SIZE_MAX as size_t).wrapping_div(size) {
        return 0 as size_t;
    }
    let mut realsize: size_t = size.wrapping_mul(nmemb);
    let mut mem: *mut memory_struct_t = userp as *mut memory_struct_t;
    if (*mem).size > ONVIF_RESPONSE_MAX_BYTES as size_t
        || realsize > (ONVIF_RESPONSE_MAX_BYTES as size_t).wrapping_sub((*mem).size)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Rejected oversized ONVIF response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as size_t;
    }
    let mut ptr: *mut ::core::ffi::c_char = realloc(
        (*mem).memory as *mut ::core::ffi::c_void,
        (*mem).size.wrapping_add(realsize).wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    if ptr.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Not enough memory for curl response\0".as_ptr() as *const ::core::ffi::c_char,
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
unsafe extern "C" fn curl_trace_callback(
    mut handle: *mut ::core::ffi::c_void,
    mut r#type: curl_infotype,
    mut data: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut prefix: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    match r#type {
        curl_infotype::CURLINFO_HEADER_OUT => {
            prefix = b">\0".as_ptr() as *const ::core::ffi::c_char;
        }
        curl_infotype::CURLINFO_HEADER_IN => {
            prefix = b"<\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return 0 as ::core::ffi::c_int,
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"ONVIF curl %s headers: %zu bytes (content omitted)\0".as_ptr()
            as *const ::core::ffi::c_char,
        prefix,
        size,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn create_curl_handle() -> *mut ::core::ffi::c_void {
    let mut handle: *mut ::core::ffi::c_void = curl_easy_init();
    if handle.is_null() {
        return NULL;
    }
    let mut allowed: CURLcode = curl_easy_setopt(
        handle,
        CURLoption::CURLOPT_PROTOCOLS_STR,
        b"http,https\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if allowed.0 != CURLcode::CURLE_OK.0
        || curl_easy_setopt(
            handle,
            CURLoption::CURLOPT_FOLLOWLOCATION,
            0 as ::core::ffi::c_long,
        )
        .0 != CURLcode::CURLE_OK.0
        || curl_easy_setopt(
            handle,
            CURLoption::CURLOPT_SSL_VERIFYPEER,
            1 as ::core::ffi::c_long,
        )
        .0 != CURLcode::CURLE_OK.0
        || curl_easy_setopt(
            handle,
            CURLoption::CURLOPT_SSL_VERIFYHOST,
            2 as ::core::ffi::c_long,
        )
        .0 != CURLcode::CURLE_OK.0
    {
        curl_easy_cleanup(handle);
        return NULL;
    }
    if !getenv(b"LIGHTNVR_ONVIF_CURL_TRACE\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        curl_easy_setopt(
            handle,
            CURLoption::CURLOPT_VERBOSE,
            1 as ::core::ffi::c_long,
        );
        curl_easy_setopt(
            handle,
            CURLoption::CURLOPT_DEBUGFUNCTION,
            Some(
                curl_trace_callback
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        curl_infotype,
                        *mut ::core::ffi::c_char,
                        size_t,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    return handle;
}
unsafe extern "C" fn release_subscription_curl(mut subscription: *mut onvif_subscription_t) {
    let mut lock: *mut pthread_mutex_t = subscription_lock(subscription);
    if lock.is_null() {
        return;
    }
    pthread_mutex_lock(lock);
    if !(*subscription).curl.is_null() {
        curl_easy_cleanup((*subscription).curl);
        (*subscription).curl = NULL as *mut ::core::ffi::c_void;
    }
    pthread_mutex_unlock(lock);
}
unsafe extern "C" fn create_onvif_request(
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut request_body: *const ::core::ffi::c_char,
    mut operation: onvif_operation_t,
    mut to: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut action: *const ::core::ffi::c_char = onvif_operation_action(operation);
    if action.is_null() || request_body.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut has_credentials: bool = !username.is_null()
        && strlen(username) > 0 as size_t
        && !password.is_null()
        && strlen(password) > 0 as size_t;
    let mut security_header: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !has_credentials {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Creating ONVIF request without authentication (no credentials provided)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        security_header = strdup(b"\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Creating ONVIF request with WS-Security authentication\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        security_header = onvif_create_security_header(username, password);
        if security_header.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to create WS-Security header\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    if security_header.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut uuid: [::core::ffi::c_char; 48] = [
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
    ];
    generate_uuid(
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 48]>(),
    );
    let mut destination: ezxml_t = ezxml_new(b"wsa:To\0".as_ptr() as *const ::core::ffi::c_char);
    if destination.is_null() {
        free(security_header as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    ezxml_set_txt(
        destination,
        if !to.is_null() {
            to
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut destination_xml: *mut ::core::ffi::c_char = ezxml_toxml(destination);
    ezxml_free(destination);
    if destination_xml.is_null() {
        free(security_header as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut envelope_size: size_t = strlen(request_body)
        .wrapping_add(strlen(security_header))
        .wrapping_add(strlen(action))
        .wrapping_add(strlen(destination_xml))
        .wrapping_add(2048 as size_t);
    let mut soap_request: *mut ::core::ffi::c_char =
        malloc(envelope_size) as *mut ::core::ffi::c_char;
    if soap_request.is_null() {
        free(security_header as *mut ::core::ffi::c_void);
        free(destination_xml as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    snprintf(
        soap_request,
        envelope_size,
        b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" xmlns:wsa=\"http://www.w3.org/2005/08/addressing\"><s:Header><wsa:MessageID>urn:uuid:%s</wsa:MessageID>%s<wsa:Action>%s</wsa:Action>%s</s:Header><s:Body xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">%s</s:Body></s:Envelope>\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
        destination_xml,
        action,
        security_header,
        request_body,
    );
    free(security_header as *mut ::core::ffi::c_void);
    free(destination_xml as *mut ::core::ffi::c_void);
    return soap_request;
}
unsafe extern "C" fn perform_onvif_request_locked(
    mut handle: *mut ::core::ffi::c_void,
    mut full_url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut request_body: *const ::core::ffi::c_char,
    mut operation: onvif_operation_t,
    mut status: *mut onvif_request_status_t,
) -> *mut ::core::ffi::c_char {
    if !status.is_null() {
        memset(
            status as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<onvif_request_status_t>(),
        );
    }
    let mut action: *const ::core::ffi::c_char = onvif_operation_action(operation);
    if action.is_null() || request_body.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !initialized || handle.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF detection system not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !url_is_http_endpoint(full_url) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Rejected invalid ONVIF HTTP(S) endpoint\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"ONVIF Detection: Sending SOAP request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut soap_request: *mut ::core::ffi::c_char =
        create_onvif_request(username, password, request_body, operation, full_url);
    if soap_request.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create ONVIF request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    curl_easy_setopt(handle, CURLoption::CURLOPT_URL, full_url);
    curl_easy_setopt(handle, CURLoption::CURLOPT_POSTFIELDS, soap_request);
    curl_easy_setopt(
        handle,
        CURLoption::CURLOPT_POSTFIELDSIZE,
        strlen(soap_request),
    );
    let mut content_type: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut content_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"Content-Type: application/soap+xml; charset=utf-8; action=\"%s\"\0".as_ptr()
            as *const ::core::ffi::c_char,
        action,
    );
    let mut headers: *mut curl_slist = ::core::ptr::null_mut::<curl_slist>();
    headers = curl_slist_append(headers, &raw mut content_type as *mut ::core::ffi::c_char);
    curl_easy_setopt(handle, CURLoption::CURLOPT_HTTPHEADER, headers);
    let mut chunk: memory_struct_t = memory_struct_t {
        memory: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
    };
    curl_easy_setopt(
        handle,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            write_memory_callback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(
        handle,
        CURLoption::CURLOPT_WRITEDATA,
        &raw mut chunk as *mut ::core::ffi::c_void,
    );
    curl_easy_setopt(
        handle,
        CURLoption::CURLOPT_TIMEOUT,
        10 as ::core::ffi::c_long,
    );
    curl_easy_setopt(
        handle,
        CURLoption::CURLOPT_NOSIGNAL,
        1 as ::core::ffi::c_long,
    );
    let mut res: CURLcode = curl_easy_perform(handle);
    let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut elapsed: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut new_connections: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    curl_easy_getinfo(handle, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
    curl_easy_getinfo(handle, CURLINFO::CURLINFO_TOTAL_TIME, &raw mut elapsed);
    curl_easy_getinfo(
        handle,
        CURLINFO::CURLINFO_NUM_CONNECTS,
        &raw mut new_connections,
    );
    if !status.is_null() {
        (*status).curl_code = res;
        (*status).http_code = http_code;
        (*status).elapsed = elapsed;
        (*status).new_connections = new_connections;
    }
    free(soap_request as *mut ::core::ffi::c_void);
    curl_slist_free_all(headers);
    if res.0 != CURLcode::CURLE_OK.0 {
        if !status.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"ONVIF request %s attempt failed (curl=%d, HTTP=%ld, elapsed=%.3fs, new connections=%ld): %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                action,
                res.0 as ::core::ffi::c_int,
                http_code,
                elapsed,
                new_connections,
                curl_easy_strerror(res),
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"ONVIF request %s failed (curl=%d, HTTP=%ld, elapsed=%.3fs, new connections=%ld): %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                action,
                res.0 as ::core::ffi::c_int,
                http_code,
                elapsed,
                new_connections,
                curl_easy_strerror(res),
            );
        }
        free(chunk.memory as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if http_code != 200 as ::core::ffi::c_long || chunk.size == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF request %s failed (HTTP=%ld, response=%zu bytes, elapsed=%.3fs)\0".as_ptr()
                as *const ::core::ffi::c_char,
            action,
            http_code,
            chunk.size,
            elapsed,
        );
        if chunk.size > 0 as size_t {
            onvif_log_soap_fault(
                chunk.memory,
                chunk.size,
                b"ONVIF Detection\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        free(chunk.memory as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return chunk.memory;
}
unsafe extern "C" fn send_onvif_request_to_url(
    mut full_url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut request_body: *const ::core::ffi::c_char,
    mut operation: onvif_operation_t,
) -> *mut ::core::ffi::c_char {
    pthread_mutex_lock(&raw mut curl_mutex);
    let mut response: *mut ::core::ffi::c_char = perform_onvif_request_locked(
        curl_handle,
        full_url,
        username,
        password,
        request_body,
        operation,
        ::core::ptr::null_mut::<onvif_request_status_t>(),
    );
    pthread_mutex_unlock(&raw mut curl_mutex);
    return response;
}
unsafe extern "C" fn send_onvif_request(
    mut url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut request_body: *const ::core::ffi::c_char,
    mut service: *const ::core::ffi::c_char,
    mut operation: onvif_operation_t,
) -> *mut ::core::ffi::c_char {
    if !initialized || curl_handle.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF detection system not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut full_url: [::core::ffi::c_char; 512] = [0; 512];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut full_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/onvif/%s\0".as_ptr() as *const ::core::ffi::c_char,
        url,
        service,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF request URL too long or formatting failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return send_onvif_request_to_url(
        &raw mut full_url as *mut ::core::ffi::c_char,
        username,
        password,
        request_body,
        operation,
    );
}
unsafe extern "C" fn local_name(mut el: ezxml_t) -> *const ::core::ffi::c_char {
    if el.is_null() || (*el).name.is_null() {
        return b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    let mut colon: *const ::core::ffi::c_char = strchr((*el).name, ':' as ::core::ffi::c_int);
    return if !colon.is_null() {
        colon.offset(1 as ::core::ffi::c_int as isize)
    } else {
        (*el).name
    };
}
unsafe extern "C" fn child_by_local_name(
    mut parent: ezxml_t,
    mut local: *const ::core::ffi::c_char,
) -> ezxml_t {
    if parent.is_null() || local.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    let mut c: ezxml_t = (*parent).child;
    while !c.is_null() {
        if strcmp(local_name(c), local) == 0 as ::core::ffi::c_int {
            return c;
        }
        c = (*c).ordered;
    }
    return ::core::ptr::null_mut::<ezxml>();
}
unsafe extern "C" fn extract_subscription_address(
    mut response: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if response.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut copy: *mut ::core::ffi::c_char = strdup(response);
    if copy.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut root: ezxml_t = ezxml_parse_str(copy, strlen(copy));
    let mut address: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !root.is_null() && *ezxml_error(root).offset(0isize) == 0 {
        let mut body: ezxml_t =
            child_by_local_name(root, b"Body\0".as_ptr() as *const ::core::ffi::c_char);
        let mut reply: ezxml_t = child_by_local_name(
            body,
            b"CreatePullPointSubscriptionResponse\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut reference: ezxml_t = child_by_local_name(
            reply,
            b"SubscriptionReference\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut destination: ezxml_t = child_by_local_name(
            reference,
            b"Address\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !destination.is_null()
            && !(*destination).txt.is_null()
            && *(*destination).txt.offset(0isize) as ::core::ffi::c_int != 0
        {
            address = strdup((*destination).txt);
        }
    }
    if !root.is_null() {
        ezxml_free(root);
    }
    free(copy as *mut ::core::ffi::c_void);
    return address;
}
unsafe extern "C" fn descendant_by_local_name(
    mut node: ezxml_t,
    mut local: *const ::core::ffi::c_char,
) -> ezxml_t {
    if node.is_null() || local.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    if strcmp(local_name(node), local) == 0 as ::core::ffi::c_int {
        return node;
    }
    let mut child: ezxml_t = (*node).child;
    while !child.is_null() {
        let mut r#match: ezxml_t = descendant_by_local_name(child, local);
        if !r#match.is_null() {
            return r#match;
        }
        child = (*child).ordered;
    }
    return ::core::ptr::null_mut::<ezxml>();
}
unsafe extern "C" fn parse_xsd_datetime(
    mut text: *const ::core::ffi::c_char,
    mut timestamp: *mut time_t,
) -> bool {
    if text.is_null() || timestamp.is_null() {
        return r#false != 0;
    }
    let mut parsed: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    memset(
        &raw mut parsed as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tm>(),
    );
    let mut suffix: *mut ::core::ffi::c_char = strptime(
        text,
        b"%Y-%m-%dT%H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed,
    );
    if suffix.is_null() {
        return r#false != 0;
    }
    if *suffix as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
        suffix = suffix.offset(1);
        while *(*__ctype_b_loc())
            .offset(*suffix as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            suffix = suffix.offset(1);
        }
    }
    let mut offset_seconds: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *suffix as ::core::ffi::c_int == 'Z' as ::core::ffi::c_int
        || *suffix as ::core::ffi::c_int == 'z' as ::core::ffi::c_int
    {
        suffix = suffix.offset(1);
    } else if *suffix as ::core::ffi::c_int == '+' as ::core::ffi::c_int
        || *suffix as ::core::ffi::c_int == '-' as ::core::ffi::c_int
    {
        let c2rust_fresh1 = suffix;
        suffix = suffix.offset(1);
        let mut sign: ::core::ffi::c_int =
            if *c2rust_fresh1 as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        if strlen(suffix) < 5 as size_t
            || *(*__ctype_b_loc()).offset(
                *suffix.offset(0isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
            ) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
            || *(*__ctype_b_loc()).offset(
                *suffix.offset(1isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
            ) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
            || *suffix.offset(2isize) as ::core::ffi::c_int != ':' as ::core::ffi::c_int
            || *(*__ctype_b_loc()).offset(
                *suffix.offset(3isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
            ) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
            || *(*__ctype_b_loc()).offset(
                *suffix.offset(4isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
            ) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
        {
            return r#false != 0;
        }
        let mut hours: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut minutes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if sscanf(
            suffix,
            b"%2d:%2d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut hours,
            &raw mut minutes,
        ) != 2 as ::core::ffi::c_int
            || hours > 23 as ::core::ffi::c_int
            || minutes > 59 as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        offset_seconds =
            sign * (hours * 3600 as ::core::ffi::c_int + minutes * 60 as ::core::ffi::c_int);
        suffix = suffix.offset(5 as ::core::ffi::c_int as isize);
    }
    while *(*__ctype_b_loc()).offset(*suffix as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        suffix = suffix.offset(1);
    }
    if *suffix as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut utc: time_t = timegm(&raw mut parsed);
    if utc == -1 as ::core::ffi::c_int as time_t {
        return r#false != 0;
    }
    *timestamp = utc - offset_seconds as time_t;
    return r#true != 0;
}
unsafe extern "C" fn subscription_lease_seconds(
    mut response: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut lease: ::core::ffi::c_int = ONVIF_SUBSCRIPTION_LEASE_SECONDS;
    if response.is_null() {
        return lease;
    }
    let mut copy: *mut ::core::ffi::c_char = strdup(response);
    if copy.is_null() {
        return lease;
    }
    let mut xml: ezxml_t = ezxml_parse_str(copy, strlen(copy));
    if !xml.is_null() {
        let mut current_node: ezxml_t =
            descendant_by_local_name(xml, b"CurrentTime\0".as_ptr() as *const ::core::ffi::c_char);
        let mut termination_node: ezxml_t = descendant_by_local_name(
            xml,
            b"TerminationTime\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut current_time: time_t = 0;
        let mut termination_time: time_t = 0;
        let mut have_current: bool = !current_node.is_null()
            && parse_xsd_datetime(
                if !current_node.is_null() {
                    (*current_node).txt
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                &raw mut current_time,
            ) as ::core::ffi::c_int
                != 0;
        let mut have_termination: bool = !termination_node.is_null()
            && parse_xsd_datetime(
                if !termination_node.is_null() {
                    (*termination_node).txt
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                &raw mut termination_time,
            ) as ::core::ffi::c_int
                != 0;
        let mut duration: time_t = 0 as time_t;
        if have_current as ::core::ffi::c_int != 0 && have_termination as ::core::ffi::c_int != 0 {
            duration = termination_time - current_time;
        } else if have_termination {
            duration = termination_time - time(::core::ptr::null_mut::<time_t>());
        }
        if duration > 0 as time_t
            && duration
                <= (7 as ::core::ffi::c_int
                    * 24 as ::core::ffi::c_int
                    * 60 as ::core::ffi::c_int
                    * 60 as ::core::ffi::c_int) as time_t
        {
            lease = duration as ::core::ffi::c_int;
        }
        ezxml_free(xml);
    }
    free(copy as *mut ::core::ffi::c_void);
    return lease;
}
unsafe extern "C" fn apply_subscription_lease(
    mut subscription: *mut onvif_subscription_t,
    mut lease_seconds: ::core::ffi::c_int,
) {
    if subscription.is_null() {
        return;
    }
    if lease_seconds <= 0 as ::core::ffi::c_int {
        lease_seconds = ONVIF_SUBSCRIPTION_LEASE_SECONDS;
    }
    time(&raw mut (*subscription).creation_time);
    (*subscription).expiration_time = (*subscription).creation_time + lease_seconds as time_t;
    let mut renewal_delay: ::core::ffi::c_int =
        lease_seconds * ONVIF_RENEW_NUMERATOR / ONVIF_RENEW_DENOMINATOR;
    if renewal_delay <= 0 as ::core::ffi::c_int {
        renewal_delay = 1 as ::core::ffi::c_int;
    }
    if renewal_delay >= lease_seconds && lease_seconds > 1 as ::core::ffi::c_int {
        renewal_delay = lease_seconds - 1 as ::core::ffi::c_int;
    }
    (*subscription).renewal_time = (*subscription).creation_time + renewal_delay as time_t;
}
unsafe extern "C" fn is_transport_drop(mut status: *const onvif_request_status_t) -> bool {
    if status.is_null() || (*status).http_code != 0 as ::core::ffi::c_long {
        return r#false != 0;
    }
    return (*status).curl_code.0 == CURLcode::CURLE_GOT_NOTHING.0
        || (*status).curl_code.0 == CURLcode::CURLE_RECV_ERROR.0
        || (*status).curl_code.0 == CURLcode::CURLE_SEND_ERROR.0;
}
unsafe extern "C" fn subscription_address_matches_camera_origin(
    mut camera_url: *const ::core::ffi::c_char,
    mut subscription_address: *const ::core::ffi::c_char,
) -> bool {
    let mut camera_number: ::core::ffi::c_long = 0;
    let mut sub_number: ::core::ffi::c_long = 0;
    if !url_is_http_endpoint(camera_url) || !url_is_http_endpoint(subscription_address) {
        return r#false != 0;
    }
    let mut camera: *mut CURLU = curl_url();
    let mut sub: *mut CURLU = curl_url();
    let mut camera_host: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut camera_scheme: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut camera_port: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sub_host: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sub_scheme: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sub_port: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ok: bool = r#false != 0;
    if !(camera.is_null() || sub.is_null()) {
        if curl_url_set(
            camera,
            CURLUPart::CURLUPART_URL,
            camera_url,
            0 as ::core::ffi::c_uint,
        )
        .0 == CURLUcode::CURLUE_OK.0
        {
            if curl_url_set(
                sub,
                CURLUPart::CURLUPART_URL,
                subscription_address,
                0 as ::core::ffi::c_uint,
            )
            .0 == CURLUcode::CURLUE_OK.0
            {
                if curl_url_get(
                    camera,
                    CURLUPart::CURLUPART_HOST,
                    &raw mut camera_host,
                    0 as ::core::ffi::c_uint,
                )
                .0 == CURLUcode::CURLUE_OK.0
                {
                    if curl_url_get(
                        sub,
                        CURLUPart::CURLUPART_HOST,
                        &raw mut sub_host,
                        0 as ::core::ffi::c_uint,
                    )
                    .0 == CURLUcode::CURLUE_OK.0
                    {
                        if curl_url_get(
                            camera,
                            CURLUPart::CURLUPART_SCHEME,
                            &raw mut camera_scheme,
                            0 as ::core::ffi::c_uint,
                        )
                        .0 == CURLUcode::CURLUE_OK.0
                        {
                            if curl_url_get(
                                sub,
                                CURLUPart::CURLUPART_SCHEME,
                                &raw mut sub_scheme,
                                0 as ::core::ffi::c_uint,
                            )
                            .0 == CURLUcode::CURLUE_OK.0
                            {
                                if curl_url_get(
                                    camera,
                                    CURLUPart::CURLUPART_PORT,
                                    &raw mut camera_port,
                                    CURLU_DEFAULT_PORT as ::core::ffi::c_uint,
                                )
                                .0 == CURLUcode::CURLUE_OK.0
                                {
                                    if curl_url_get(
                                        sub,
                                        CURLUPart::CURLUPART_PORT,
                                        &raw mut sub_port,
                                        CURLU_DEFAULT_PORT as ::core::ffi::c_uint,
                                    )
                                    .0 == CURLUcode::CURLUE_OK.0
                                    {
                                        camera_number = strtol(
                                            camera_port,
                                            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                            10 as ::core::ffi::c_int,
                                        );
                                        sub_number = strtol(
                                            sub_port,
                                            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                            10 as ::core::ffi::c_int,
                                        );
                                        ok = camera_number > 0 as ::core::ffi::c_long
                                            && camera_number == sub_number
                                            && strcasecmp(camera_scheme, sub_scheme)
                                                == 0 as ::core::ffi::c_int
                                            && strcasecmp(camera_host, sub_host)
                                                == 0 as ::core::ffi::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !camera_host.is_null() {
        curl_free(camera_host as *mut ::core::ffi::c_void);
    }
    if !camera_scheme.is_null() {
        curl_free(camera_scheme as *mut ::core::ffi::c_void);
    }
    if !camera_port.is_null() {
        curl_free(camera_port as *mut ::core::ffi::c_void);
    }
    if !sub_host.is_null() {
        curl_free(sub_host as *mut ::core::ffi::c_void);
    }
    if !sub_scheme.is_null() {
        curl_free(sub_scheme as *mut ::core::ffi::c_void);
    }
    if !sub_port.is_null() {
        curl_free(sub_port as *mut ::core::ffi::c_void);
    }
    if !camera.is_null() {
        curl_url_cleanup(camera);
    }
    if !sub.is_null() {
        curl_url_cleanup(sub);
    }
    return ok;
}
unsafe extern "C" fn send_subscription_request(
    mut subscription: *mut onvif_subscription_t,
    mut request_body: *const ::core::ffi::c_char,
    mut operation: onvif_operation_t,
    mut transport_dropped: *mut bool,
) -> *mut ::core::ffi::c_char {
    if !transport_dropped.is_null() {
        *transport_dropped = r#false != 0;
    }
    let mut action: *const ::core::ffi::c_char = onvif_operation_action(operation);
    if subscription.is_null() || request_body.is_null() || action.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if strncasecmp(
        &raw mut (*subscription).subscription_address as *mut ::core::ffi::c_char,
        b"http://\0".as_ptr() as *const ::core::ffi::c_char,
        7 as size_t,
    ) == 0 as ::core::ffi::c_int
        || strncasecmp(
            &raw mut (*subscription).subscription_address as *mut ::core::ffi::c_char,
            b"https://\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        if !subscription_address_matches_camera_origin(
            &raw mut (*subscription).camera_url as *mut ::core::ffi::c_char,
            &raw mut (*subscription).subscription_address as *mut ::core::ffi::c_char,
        ) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Rejected ONVIF subscription endpoint outside the configured camera origin\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        let mut response: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut status: onvif_request_status_t = onvif_request_status_t {
            curl_code: CURLcode(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
            http_code: 0,
            elapsed: 0.,
            new_connections: 0,
        };
        let mut attempts: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut lock: *mut pthread_mutex_t = subscription_lock(subscription);
        if lock.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        pthread_mutex_lock(lock);
        if (*subscription).curl.is_null() {
            (*subscription).curl = create_curl_handle();
        }
        if (*subscription).curl.is_null() {
            pthread_mutex_unlock(lock);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to create a curl handle for the ONVIF subscription of %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*subscription).camera_url as *mut ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        while attempts < ONVIF_PULL_DROP_MAX_ATTEMPTS {
            attempts += 1;
            response = perform_onvif_request_locked(
                (*subscription).curl,
                &raw mut (*subscription).subscription_address as *mut ::core::ffi::c_char,
                &raw mut (*subscription).username as *mut ::core::ffi::c_char,
                &raw mut (*subscription).password as *mut ::core::ffi::c_char,
                request_body,
                operation,
                &raw mut status,
            );
            if !response.is_null()
                || !is_transport_drop(&raw mut status)
                || attempts >= ONVIF_PULL_DROP_MAX_ATTEMPTS
                || is_shutdown_initiated() as ::core::ffi::c_int != 0
            {
                break;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"ONVIF request %s: camera closed the connection without a response (curl=%d); retrying on the same subscription (attempt %d/%d)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                action,
                status.curl_code.0 as ::core::ffi::c_int,
                attempts + 1 as ::core::ffi::c_int,
                6 as ::core::ffi::c_int,
            );
            usleep((ONVIF_PULL_DROP_RETRY_DELAY_MS * 1000 as ::core::ffi::c_int) as __useconds_t);
        }
        pthread_mutex_unlock(lock);
        if !transport_dropped.is_null() {
            *transport_dropped =
                response.is_null() && is_transport_drop(&raw mut status) as ::core::ffi::c_int != 0;
        }
        if response.is_null() && status.curl_code.0 != CURLcode::CURLE_OK.0 {
            _log_message_ctx(
                log_level_t(
                    (if !transport_dropped.is_null()
                        && *transport_dropped as ::core::ffi::c_int != 0
                    {
                        log_level_t::LOG_LEVEL_DEBUG.0 as ::core::ffi::c_int
                    } else {
                        log_level_t::LOG_LEVEL_ERROR.0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ),
                LOG_COMPONENT,
                _log_stream_name,
                b"ONVIF request %s failed after %d attempt%s (last attempt: curl=%d, HTTP=%ld, elapsed=%.3fs, new connections=%ld): %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                action,
                attempts,
                if attempts == 1 as ::core::ffi::c_int {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"s\0".as_ptr() as *const ::core::ffi::c_char
                },
                status.curl_code.0 as ::core::ffi::c_int,
                status.http_code,
                status.elapsed,
                status.new_connections,
                curl_easy_strerror(status.curl_code),
            );
        }
        return response;
    }
    if !strchr(
        &raw mut (*subscription).subscription_address as *mut ::core::ffi::c_char,
        ':' as ::core::ffi::c_int,
    )
    .is_null()
        || strncmp(
            &raw mut (*subscription).subscription_address as *mut ::core::ffi::c_char,
            b"//\0".as_ptr() as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut last_slash: *const ::core::ffi::c_char = strrchr(
        &raw mut (*subscription).subscription_address as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if last_slash.is_null()
        || *last_slash.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return send_onvif_request(
        &raw mut (*subscription).camera_url as *mut ::core::ffi::c_char,
        &raw mut (*subscription).username as *mut ::core::ffi::c_char,
        &raw mut (*subscription).password as *mut ::core::ffi::c_char,
        request_body,
        last_slash.offset(1 as ::core::ffi::c_int as isize),
        operation,
    );
}
unsafe extern "C" fn renew_subscription_locked(
    mut subscription: *mut onvif_subscription_t,
) -> bool {
    let mut request_body: *const ::core::ffi::c_char = b"<Renew xmlns=\"http://docs.oasis-open.org/wsn/b-2\"><TerminationTime>PT600S</TerminationTime></Renew>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut response: *mut ::core::ffi::c_char = send_subscription_request(
        subscription,
        request_body,
        onvif_operation_t::ONVIF_OPERATION_RENEW,
        ::core::ptr::null_mut::<bool>(),
    );
    if response.is_null() {
        return r#false != 0;
    }
    let mut lease_seconds: ::core::ffi::c_int = subscription_lease_seconds(response);
    free(response as *mut ::core::ffi::c_void);
    apply_subscription_lease(subscription, lease_seconds);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Renewed ONVIF subscription for %s for %d seconds\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*subscription).camera_url as *mut ::core::ffi::c_char,
        lease_seconds,
    );
    return r#true != 0;
}
unsafe extern "C" fn unsubscribe_subscription_locked(mut subscription: *mut onvif_subscription_t) {
    if subscription.is_null()
        || !(*subscription).active
        || (*subscription).subscription_address[0usize] as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
    {
        return;
    }
    let mut request_body: *const ::core::ffi::c_char =
        b"<Unsubscribe xmlns=\"http://docs.oasis-open.org/wsn/b-2\"/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut response: *mut ::core::ffi::c_char = send_subscription_request(
        subscription,
        request_body,
        onvif_operation_t::ONVIF_OPERATION_UNSUBSCRIBE,
        ::core::ptr::null_mut::<bool>(),
    );
    if !response.is_null() {
        free(response as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unsubscribed ONVIF PullPoint for %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*subscription).camera_url as *mut ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unable to unsubscribe ONVIF PullPoint for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*subscription).camera_url as *mut ::core::ffi::c_char,
        );
    }
    (*subscription).active = r#false != 0;
    release_subscription_curl(subscription);
}
unsafe extern "C" fn discover_event_service_url(
    mut url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut request_body: *const ::core::ffi::c_char = b"<GetServices xmlns=\"http://www.onvif.org/ver10/device/wsdl\"><IncludeCapability>false</IncludeCapability></GetServices>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut response: *mut ::core::ffi::c_char = send_onvif_request(
        url,
        username,
        password,
        request_body,
        b"device_service\0".as_ptr() as *const ::core::ffi::c_char,
        onvif_operation_t::ONVIF_OPERATION_GET_SERVICES,
    );
    if response.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF: GetServices to device_service endpoint failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut xml: ezxml_t = ezxml_parse_str(response, strlen(response));
    let mut event_url: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !xml.is_null() {
        let mut body: ezxml_t =
            child_by_local_name(xml, b"Body\0".as_ptr() as *const ::core::ffi::c_char);
        let mut resp: ezxml_t = if !body.is_null() {
            child_by_local_name(
                body,
                b"GetServicesResponse\0".as_ptr() as *const ::core::ffi::c_char,
            )
        } else {
            ::core::ptr::null_mut::<ezxml>()
        };
        if !resp.is_null() {
            let mut events_ns: *const ::core::ffi::c_char =
                b"http://www.onvif.org/ver10/events/wsdl\0".as_ptr() as *const ::core::ffi::c_char;
            let mut svc: ezxml_t = (*resp).child;
            while !svc.is_null() && event_url.is_null() {
                if strcmp(
                    local_name(svc),
                    b"Service\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    let mut ns: ezxml_t = child_by_local_name(
                        svc,
                        b"Namespace\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    let mut xaddr: ezxml_t =
                        child_by_local_name(svc, b"XAddr\0".as_ptr() as *const ::core::ffi::c_char);
                    let mut ns_text: *const ::core::ffi::c_char = if !ns.is_null() {
                        if !ns.is_null() {
                            (*ns).txt
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        }
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    };
                    let mut xaddr_text: *const ::core::ffi::c_char = if !xaddr.is_null() {
                        if !xaddr.is_null() {
                            (*xaddr).txt
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        }
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    };
                    if !ns_text.is_null()
                        && !xaddr_text.is_null()
                        && *xaddr_text.offset(0isize) as ::core::ffi::c_int
                            != '\0' as ::core::ffi::c_int
                        && strcmp(ns_text, events_ns) == 0 as ::core::ffi::c_int
                    {
                        event_url = strdup(xaddr_text);
                    }
                }
                svc = (*svc).ordered;
            }
        }
        ezxml_free(xml);
    }
    if event_url.is_null() {
        let mut events_ns_0: *const ::core::ffi::c_char =
            b"http://www.onvif.org/ver10/events/wsdl\0".as_ptr() as *const ::core::ffi::c_char;
        let mut ns_pos: *const ::core::ffi::c_char = strstr(response, events_ns_0);
        if !ns_pos.is_null() {
            let mut xaddr_open: [*const ::core::ffi::c_char; 3] = [
                b"<tds:XAddr>\0".as_ptr() as *const ::core::ffi::c_char,
                b"<XAddr>\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ];
            let mut xaddr_close: [*const ::core::ffi::c_char; 3] = [
                b"</tds:XAddr>\0".as_ptr() as *const ::core::ffi::c_char,
                b"</XAddr>\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ];
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while !xaddr_open[i as usize].is_null() {
                let mut tag: *const ::core::ffi::c_char = strstr(ns_pos, xaddr_open[i as usize]);
                if !tag.is_null() && tag.offset_from(ns_pos) < 2048isize {
                    let mut val_start: *const ::core::ffi::c_char =
                        tag.offset(strlen(xaddr_open[i as usize]) as isize);
                    let mut val_end: *const ::core::ffi::c_char =
                        strstr(val_start, xaddr_close[i as usize]);
                    if !val_end.is_null() {
                        event_url = strndup(val_start, val_end.offset_from(val_start) as size_t);
                        break;
                    }
                }
                i += 1;
            }
        }
    }
    if !event_url.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF: Discovered event service endpoint\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF: Could not find XAddr for events service in GetServices response\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    free(response as *mut ::core::ffi::c_void);
    return event_url;
}
unsafe extern "C" fn get_subscription(
    mut url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
) -> *mut onvif_subscription_t {
    pthread_mutex_lock(&raw mut subscription_mutex);
    if subscriptions_shutting_down {
        pthread_mutex_unlock(&raw mut subscription_mutex);
        return ::core::ptr::null_mut::<onvif_subscription_t>();
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < subscription_count {
        if strcmp(
            &raw mut (*(&raw mut subscriptions as *mut onvif_subscription_t).offset(i as isize))
                .camera_url as *mut ::core::ffi::c_char,
            url,
        ) == 0 as ::core::ffi::c_int
        {
            let mut now: time_t = 0;
            time(&raw mut now);
            if subscriptions[i as usize].active as ::core::ffi::c_int != 0
                && now < subscriptions[i as usize].expiration_time
            {
                if now >= subscriptions[i as usize].renewal_time {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Renewing ONVIF subscription for %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        url,
                    );
                    if !renew_subscription_locked(
                        (&raw mut subscriptions as *mut onvif_subscription_t).offset(i as isize),
                    ) {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"ONVIF subscription renewal failed for %s; recreating it\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            url,
                        );
                        unsubscribe_subscription_locked(
                            (&raw mut subscriptions as *mut onvif_subscription_t)
                                .offset(i as isize),
                        );
                        break;
                    }
                }
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Reusing existing ONVIF subscription for %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    url,
                );
                pthread_mutex_unlock(&raw mut subscription_mutex);
                return (&raw mut subscriptions as *mut onvif_subscription_t).offset(i as isize);
            } else if !subscriptions[i as usize].active {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"ONVIF subscription for %s is inactive, creating new one\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    url,
                );
                break;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"ONVIF subscription for %s expired, replacing it\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    url,
                );
                unsubscribe_subscription_locked(
                    (&raw mut subscriptions as *mut onvif_subscription_t).offset(i as isize),
                );
                break;
            }
        } else {
            i += 1;
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Creating new ONVIF subscription for %s\0".as_ptr() as *const ::core::ffi::c_char,
        url,
    );
    let mut request_body: *const ::core::ffi::c_char = b"<CreatePullPointSubscription xmlns=\"http://www.onvif.org/ver10/events/wsdl\"><InitialTerminationTime>PT600S</InitialTerminationTime></CreatePullPointSubscription>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut discovered_url: *mut ::core::ffi::c_char =
        discover_event_service_url(url, username, password);
    let mut response: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut attempted_event_url: [::core::ffi::c_char; 512] = [
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
    if !discovered_url.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF: Sending CreatePullPointSubscription to discovered endpoint\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        safe_strcpy(
            &raw mut attempted_event_url as *mut ::core::ffi::c_char,
            discovered_url,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        response = send_onvif_request_to_url(
            discovered_url,
            username,
            password,
            request_body,
            onvif_operation_t::ONVIF_OPERATION_CREATE_PULLPOINT,
        );
        free(discovered_url as *mut ::core::ffi::c_void);
    }
    if response.is_null() {
        let mut fallback_services: [*const ::core::ffi::c_char; 4] = [
            b"service\0".as_ptr() as *const ::core::ffi::c_char,
            b"event_service\0".as_ptr() as *const ::core::ffi::c_char,
            b"Events\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !fallback_services[i_0 as usize].is_null() && response.is_null() {
            let mut fallback_url: [::core::ffi::c_char; 512] = [0; 512];
            let mut written: ::core::ffi::c_int = snprintf(
                &raw mut fallback_url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                b"%s/onvif/%s\0".as_ptr() as *const ::core::ffi::c_char,
                url,
                fallback_services[i_0 as usize],
            );
            if !(written < 0 as ::core::ffi::c_int
                || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>())
            {
                if attempted_event_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                    && strcmp(
                        &raw mut attempted_event_url as *mut ::core::ffi::c_char,
                        &raw mut fallback_url as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"ONVIF: Skipping already-tried event endpoint: onvif/%s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        fallback_services[i_0 as usize],
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"ONVIF: Trying fallback event endpoint: onvif/%s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        fallback_services[i_0 as usize],
                    );
                    response = send_onvif_request(
                        url,
                        username,
                        password,
                        request_body,
                        fallback_services[i_0 as usize],
                        onvif_operation_t::ONVIF_OPERATION_CREATE_PULLPOINT,
                    );
                }
            }
            i_0 += 1;
        }
    }
    if response.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create subscription on any endpoint\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut subscription_mutex);
        return ::core::ptr::null_mut::<onvif_subscription_t>();
    }
    let mut subscription_address: *mut ::core::ffi::c_char = extract_subscription_address(response);
    let mut lease_seconds: ::core::ffi::c_int = subscription_lease_seconds(response);
    free(response as *mut ::core::ffi::c_void);
    if subscription_address.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to extract subscription address\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut subscription_mutex);
        return ::core::ptr::null_mut::<onvif_subscription_t>();
    }
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < subscription_count {
        if !subscriptions[i_1 as usize].active {
            slot = i_1;
            break;
        } else {
            i_1 += 1;
        }
    }
    if slot == -1 as ::core::ffi::c_int && subscription_count < MAX_SUBSCRIPTIONS {
        let c2rust_fresh0 = subscription_count;
        subscription_count += 1;
        slot = c2rust_fresh0;
    }
    if slot >= 0 as ::core::ffi::c_int {
        release_subscription_curl(
            (&raw mut subscriptions as *mut onvif_subscription_t).offset(slot as isize),
        );
        memset(
            (&raw mut subscriptions as *mut onvif_subscription_t).offset(slot as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<onvif_subscription_t>(),
        );
        safe_strcpy(
            &raw mut (*(&raw mut subscriptions as *mut onvif_subscription_t).offset(slot as isize))
                .camera_url as *mut ::core::ffi::c_char,
            url,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*(&raw mut subscriptions as *mut onvif_subscription_t).offset(slot as isize))
                .username as *mut ::core::ffi::c_char,
            username,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*(&raw mut subscriptions as *mut onvif_subscription_t).offset(slot as isize))
                .password as *mut ::core::ffi::c_char,
            password,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*(&raw mut subscriptions as *mut onvif_subscription_t).offset(slot as isize))
                .subscription_address as *mut ::core::ffi::c_char,
            subscription_address,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        apply_subscription_lease(
            (&raw mut subscriptions as *mut onvif_subscription_t).offset(slot as isize),
            lease_seconds,
        );
        subscriptions[slot as usize].active = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Successfully created ONVIF subscription (lease: %d seconds)\0".as_ptr()
                as *const ::core::ffi::c_char,
            lease_seconds,
        );
        free(subscription_address as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut subscription_mutex);
        return (&raw mut subscriptions as *mut onvif_subscription_t).offset(slot as isize);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"No space for new ONVIF subscription\0".as_ptr() as *const ::core::ffi::c_char,
    );
    free(subscription_address as *mut ::core::ffi::c_void);
    pthread_mutex_unlock(&raw mut subscription_mutex);
    return ::core::ptr::null_mut::<onvif_subscription_t>();
}
unsafe extern "C" fn note_pull_failure(mut camera_url: *const ::core::ffi::c_char) {
    pthread_mutex_lock(&raw mut subscription_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < subscription_count {
        let mut subscription: *mut onvif_subscription_t =
            (&raw mut subscriptions as *mut onvif_subscription_t).offset(i as isize);
        if strcmp(
            &raw mut (*subscription).camera_url as *mut ::core::ffi::c_char,
            camera_url,
        ) != 0 as ::core::ffi::c_int
            || !(*subscription).active
        {
            i += 1;
        } else {
            let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
            if (*subscription).consecutive_pull_failures == 0 as ::core::ffi::c_int {
                (*subscription).first_pull_failure_time = now;
            }
            (*subscription).consecutive_pull_failures += 1;
            if (*subscription).consecutive_pull_failures == ONVIF_PULL_FAILURE_WARN_COUNT {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"ONVIF PullPoint for %s has failed %d consecutive pulls; retaining the same subscription\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    camera_url,
                    (*subscription).consecutive_pull_failures,
                );
            }
            if (*subscription).consecutive_pull_failures >= ONVIF_PULL_FAILURE_WARN_COUNT
                && now - (*subscription).first_pull_failure_time
                    >= ONVIF_PULL_FAILURE_REFRESH_SECONDS as time_t
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"ONVIF PullPoint for %s has not completed a pull for %d seconds; unsubscribing before refresh\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    camera_url,
                    120 as ::core::ffi::c_int,
                );
                unsubscribe_subscription_locked(subscription);
            }
            break;
        }
    }
    pthread_mutex_unlock(&raw mut subscription_mutex);
}
unsafe extern "C" fn note_pull_success(mut camera_url: *const ::core::ffi::c_char) {
    pthread_mutex_lock(&raw mut subscription_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < subscription_count {
        let mut subscription: *mut onvif_subscription_t =
            (&raw mut subscriptions as *mut onvif_subscription_t).offset(i as isize);
        if strcmp(
            &raw mut (*subscription).camera_url as *mut ::core::ffi::c_char,
            camera_url,
        ) != 0 as ::core::ffi::c_int
        {
            i += 1;
        } else {
            if (*subscription).consecutive_pull_failures >= ONVIF_PULL_FAILURE_WARN_COUNT {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"ONVIF PullPoint for %s recovered after %d failed pulls\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    camera_url,
                    (*subscription).consecutive_pull_failures,
                );
            }
            (*subscription).consecutive_pull_failures = 0 as ::core::ffi::c_int;
            (*subscription).first_pull_failure_time = 0 as time_t;
            break;
        }
    }
    pthread_mutex_unlock(&raw mut subscription_mutex);
}
unsafe extern "C" fn topic_is_motion(mut topic_text: *const ::core::ffi::c_char) -> bool {
    if topic_text.is_null() {
        return r#false != 0;
    }
    return !strstr(
        topic_text,
        b"MotionDetector\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .is_null()
        || !strstr(
            topic_text,
            b"VideoAnalytics/Motion\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"MotionAlarm\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"PeopleDetector\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"TPSmartEventDetector\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"SmartMotion\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"ObjectDetect\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"HumanDetect\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"FaceDetect\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"VehicleDetect\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"PeopleDetect\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"CrossLineDetector\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"CrossRegionDetector\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"IntrusionDetector\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"FieldDetector\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !strstr(
            topic_text,
            b"LineDetector\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null();
}
unsafe extern "C" fn ci_contains(
    mut hay: *const ::core::ffi::c_char,
    mut needle: *const ::core::ffi::c_char,
) -> bool {
    if hay.is_null() || needle.is_null() {
        return r#false != 0;
    }
    let mut nl: size_t = strlen(needle);
    if nl == 0 as size_t {
        return r#false != 0;
    }
    while *hay != 0 {
        if strncasecmp(hay, needle, nl) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        hay = hay.offset(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn normalize_object_class(
    mut raw: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if raw.is_null() || *raw.offset(0isize) == 0 {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if ci_contains(raw, b"human\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
        != 0
        || ci_contains(raw, b"people\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || ci_contains(raw, b"person\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || ci_contains(raw, b"pedestrian\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
    {
        return b"person\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if ci_contains(raw, b"nonmotor\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
        != 0
        || ci_contains(raw, b"bicycle\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || ci_contains(raw, b"bike\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
        || ci_contains(raw, b"cycle\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
    {
        return b"bicycle\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if ci_contains(raw, b"vehicle\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
        != 0
        || ci_contains(raw, b"car\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
    {
        return b"vehicle\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if ci_contains(raw, b"face\0".as_ptr() as *const ::core::ffi::c_char) {
        return b"face\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if ci_contains(raw, b"animal\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
        != 0
        || ci_contains(raw, b"dog\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
        || ci_contains(raw, b"cat\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
        || ci_contains(raw, b"pet\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
    {
        return b"animal\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn find_object_class(
    mut node: ezxml_t,
    mut in_data: bool,
) -> *const ::core::ffi::c_char {
    let mut c: ezxml_t = if !node.is_null() {
        (*node).child
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    while !c.is_null() {
        let mut inside: bool = in_data as ::core::ffi::c_int != 0
            || strcmp(
                local_name(c),
                b"Data\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        if inside as ::core::ffi::c_int != 0
            && strcmp(
                local_name(c),
                b"SimpleItem\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            let mut name: *const ::core::ffi::c_char =
                ezxml_attr(c, b"Name\0".as_ptr() as *const ::core::ffi::c_char);
            if !name.is_null()
                && (strcasecmp(name, b"ObjectType\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        name,
                        b"ObjectClass\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcasecmp(name, b"Type\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int)
            {
                let mut cls: *const ::core::ffi::c_char = normalize_object_class(ezxml_attr(
                    c,
                    b"Value\0".as_ptr() as *const ::core::ffi::c_char,
                ));
                if !cls.is_null() {
                    return cls;
                }
            }
            let mut value: *const ::core::ffi::c_char =
                ezxml_attr(c, b"Value\0".as_ptr() as *const ::core::ffi::c_char);
            if !name.is_null()
                && !value.is_null()
                && (strcasecmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int)
            {
                let mut cls_0: *const ::core::ffi::c_char = normalize_object_class(name);
                if !cls_0.is_null() {
                    return cls_0;
                }
            }
        }
        let mut deep: *const ::core::ffi::c_char = find_object_class(c, inside);
        if !deep.is_null() {
            return deep;
        }
        c = (*c).ordered;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn classify_message(
    mut msg: ezxml_t,
    mut topic_text: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut cls: *const ::core::ffi::c_char = find_object_class(msg, r#false != 0);
    if !cls.is_null() {
        return cls;
    }
    cls = normalize_object_class(topic_text);
    return if !cls.is_null() {
        cls
    } else {
        b"motion\0".as_ptr() as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn value_is_true(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    return strcasecmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn data_has_asserted_item(mut node: ezxml_t, mut in_data: bool) -> bool {
    let mut c: ezxml_t = if !node.is_null() {
        (*node).child
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    while !c.is_null() {
        let mut inside: bool = in_data as ::core::ffi::c_int != 0
            || strcmp(
                local_name(c),
                b"Data\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        if inside as ::core::ffi::c_int != 0
            && strcmp(
                local_name(c),
                b"SimpleItem\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            && value_is_true(ezxml_attr(
                c,
                b"Value\0".as_ptr() as *const ::core::ffi::c_char,
            )) as ::core::ffi::c_int
                != 0
        {
            return r#true != 0;
        }
        if data_has_asserted_item(c, inside) {
            return r#true != 0;
        }
        c = (*c).ordered;
    }
    return r#false != 0;
}
unsafe extern "C" fn xml_find_active_motion(
    mut node: ezxml_t,
    mut label: *mut ::core::ffi::c_char,
    mut label_sz: size_t,
) -> bool {
    let mut any: bool = r#false != 0;
    let mut c: ezxml_t = if !node.is_null() {
        (*node).child
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    while !c.is_null() {
        if strcmp(
            local_name(c),
            b"NotificationMessage\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut topic: ezxml_t =
                child_by_local_name(c, b"Topic\0".as_ptr() as *const ::core::ffi::c_char);
            let mut topic_text: *const ::core::ffi::c_char = if !topic.is_null() {
                if !topic.is_null() {
                    (*topic).txt
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                }
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            };
            if topic_is_motion(topic_text) as ::core::ffi::c_int != 0
                && data_has_asserted_item(c, r#false != 0) as ::core::ffi::c_int != 0
            {
                any = r#true != 0;
                let mut cls: *const ::core::ffi::c_char = classify_message(c, topic_text);
                let mut have_specific: bool = *label.offset(0isize) as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                    && strcmp(label, b"motion\0".as_ptr() as *const ::core::ffi::c_char)
                        != 0 as ::core::ffi::c_int;
                if !cls.is_null()
                    && strcmp(cls, b"motion\0".as_ptr() as *const ::core::ffi::c_char)
                        != 0 as ::core::ffi::c_int
                {
                    if !have_specific {
                        safe_strcpy(label, cls, label_sz, 0 as size_t);
                    }
                } else if *label.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                {
                    safe_strcpy(
                        label,
                        b"motion\0".as_ptr() as *const ::core::ffi::c_char,
                        label_sz,
                        0 as size_t,
                    );
                }
            }
        } else if xml_find_active_motion(c, label, label_sz) {
            any = r#true != 0;
        }
        c = (*c).ordered;
    }
    return any;
}
unsafe extern "C" fn has_motion_event(
    mut response: *const ::core::ffi::c_char,
    mut label: *mut ::core::ffi::c_char,
    mut label_sz: size_t,
) -> bool {
    if !label.is_null() && label_sz > 0 as size_t {
        *label.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if response.is_null() {
        return r#false != 0;
    }
    if strstr(response, b"Motion\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        && strstr(
            response,
            b"PeopleDetector\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        && strstr(
            response,
            b"IsPeople\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        && strstr(response, b"Detect\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        && strstr(
            response,
            b"ObjectType\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        return r#false != 0;
    }
    let mut copy: *mut ::core::ffi::c_char = strdup(response);
    if copy.is_null() {
        return r#false != 0;
    }
    let mut xml: ezxml_t = ezxml_parse_str(copy, strlen(copy));
    if xml.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF Detection: failed to parse PullMessages response, using fallback heuristic\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        let mut fallback: bool = !strstr(
            response,
            b"Value=\"true\"\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
            || !strstr(
                response,
                b"Value='true'\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .is_null()
            || !strstr(
                response,
                b"Value=\"1\"\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .is_null()
            || !strstr(
                response,
                b"Value='1'\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .is_null();
        if fallback as ::core::ffi::c_int != 0 && !label.is_null() && label_sz > 0 as size_t {
            safe_strcpy(
                label,
                b"motion\0".as_ptr() as *const ::core::ffi::c_char,
                label_sz,
                0 as size_t,
            );
        }
        free(copy as *mut ::core::ffi::c_void);
        return fallback;
    }
    let mut motion: bool = xml_find_active_motion(xml, label, label_sz);
    ezxml_free(xml);
    free(copy as *mut ::core::ffi::c_void);
    return motion;
}
unsafe extern "C" fn store_lpr_notifications(
    mut response: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
) {
    if response.is_null() || stream_name.is_null() || *stream_name.offset(0isize) == 0 {
        return;
    }
    let mut events: [onvif_lpr_event_t; 16] = [
        onvif_lpr_event_t {
            source: onvif_lpr_source_t(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
        onvif_lpr_event_t {
            source: onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M,
            asserted: false,
            observed_at_ms: 0,
            topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
        },
    ];
    let mut count: ::core::ffi::c_int = onvif_parse_lpr_events(
        response,
        &raw mut events as *mut onvif_lpr_event_t,
        ::core::mem::size_of::<[onvif_lpr_event_t; 16]>()
            .wrapping_div(::core::mem::size_of::<onvif_lpr_event_t>()),
    );
    if count <= 0 as ::core::ffi::c_int {
        secure_clear(
            &raw mut events as *mut onvif_lpr_event_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[onvif_lpr_event_t; 16]>(),
        );
        return;
    }
    if lpr_crypto_key_available() == 0 {
        static mut warned_missing_key: atomic_bool = r#false != 0;
        if !::core::sync::atomic::AtomicBool::from_ptr(&raw mut warned_missing_key).swap(true, ::core::sync::atomic::Ordering::SeqCst) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"LPR reads are available but protected storage is disabled: LIGHTNVR_LPR_MASTER_KEY_HEX is not configured\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        secure_clear(
            &raw mut events as *mut onvif_lpr_event_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[onvif_lpr_event_t; 16]>(),
        );
        return;
    }
    let mut stream: stream_config_t = stream_config_t {
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
    };
    if get_stream_config_by_name(stream_name, &raw mut stream) != 0 as ::core::ffi::c_int
        || stream.camera_uuid[0usize] == 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot associate ONVIF LPR notifications with stream identity\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        secure_clear(
            &raw mut events as *mut onvif_lpr_event_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[onvif_lpr_event_t; 16]>(),
        );
        return;
    }
    let mut recording_id: uint64_t = get_current_recording_id_for_stream(stream_name);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut input: lpr_read_input_t = lpr_read_input_t {
            camera_uuid: [
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
            ],
            stream_name: [0; 256],
            observed_at_ms: 0,
            source: [0; 32],
            vendor_topic: [0; 256],
            plate: [0; 64],
            has_confidence: false,
            confidence: 0.,
            country: [0; 64],
            region: [0; 64],
            plate_type: [0; 64],
            direction: [0; 64],
            lane: [0; 64],
            vehicle_type: [0; 64],
            vehicle_color: [0; 64],
            object_id: [0; 96],
            correlation_id: [0; 96],
            has_bounding_box: false,
            bbox_left: 0.,
            bbox_top: 0.,
            bbox_right: 0.,
            bbox_bottom: 0.,
            recording_id: 0,
        };
        if db_lpr_read_from_onvif(
            &raw mut stream.camera_uuid as *mut ::core::ffi::c_char,
            stream_name,
            (&raw mut events as *mut onvif_lpr_event_t).offset(i as isize),
            &raw mut input,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Ignored incomplete ONVIF LPR notification for %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
            secure_clear(
                &raw mut input as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<lpr_read_input_t>(),
            );
        } else {
            input.recording_id = recording_id;
            let mut read_uuid: [::core::ffi::c_char; 37] = [
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
            ];
            let mut stored: ::core::ffi::c_int = db_lpr_read_insert(
                &raw mut input,
                &raw mut read_uuid as *mut ::core::ffi::c_char,
            );
            if stored < 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to persist protected ONVIF LPR read for %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                );
            } else if stored == 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stored protected ONVIF LPR read for %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                );
                let mut event_error: [::core::ffi::c_char; 256] = [
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
                if event_producer_publish_lpr_read(
                    &raw mut stream.camera_uuid as *mut ::core::ffi::c_char,
                    stream_name,
                    &raw mut read_uuid as *mut ::core::ffi::c_char,
                    &raw mut input.source as *mut ::core::ffi::c_char,
                    (input.observed_at_ms / 1000 as int64_t) as time_t,
                    &raw mut event_error as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                ) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Protected LPR event enqueue failed for %s: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        stream_name,
                        &raw mut event_error as *mut ::core::ffi::c_char,
                    );
                }
            }
            secure_clear(
                &raw mut input as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<lpr_read_input_t>(),
            );
            secure_clear(
                &raw mut read_uuid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            );
        }
        i += 1;
    }
    secure_clear(
        &raw mut events as *mut onvif_lpr_event_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[onvif_lpr_event_t; 16]>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_onvif_detection_system() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut curl_mutex);
    if initialized as ::core::ffi::c_int != 0 && !curl_handle.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF detection system already initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut curl_mutex);
        return 0 as ::core::ffi::c_int;
    }
    if !curl_handle.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF detection system has a curl handle but is marked as uninitialized, cleaning up\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        curl_easy_cleanup(curl_handle);
        curl_handle = NULL as *mut ::core::ffi::c_void;
    }
    if curl_init_global() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize curl global\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut curl_mutex);
        return -1 as ::core::ffi::c_int;
    }
    curl_handle = create_curl_handle();
    if curl_handle.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize curl handle\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut curl_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if !getenv(b"LIGHTNVR_ONVIF_CURL_TRACE\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF detection: libcurl trace enabled (LIGHTNVR_ONVIF_CURL_TRACE)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    pthread_mutex_lock(&raw mut subscription_mutex);
    subscription_count = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_SUBSCRIPTIONS {
        release_subscription_curl(
            (&raw mut subscriptions as *mut onvif_subscription_t).offset(i as isize),
        );
        i += 1;
    }
    memset(
        &raw mut subscriptions as *mut onvif_subscription_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[onvif_subscription_t; 100]>(),
    );
    subscriptions_shutting_down = r#false != 0;
    pthread_mutex_unlock(&raw mut subscription_mutex);
    initialized = r#true != 0;
    pthread_mutex_unlock(&raw mut curl_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"ONVIF detection system initialized successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_onvif_detection_system() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Shutting down ONVIF detection system (initialized: %s, curl_handle: %p)\0".as_ptr()
            as *const ::core::ffi::c_char,
        if initialized as ::core::ffi::c_int != 0 {
            b"yes\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"no\0".as_ptr() as *const ::core::ffi::c_char
        },
        curl_handle,
    );
    pthread_mutex_lock(&raw mut subscription_mutex);
    subscriptions_shutting_down = r#true != 0;
    if initialized as ::core::ffi::c_int != 0 && !curl_handle.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < subscription_count {
            unsubscribe_subscription_locked(
                (&raw mut subscriptions as *mut onvif_subscription_t).offset(i as isize),
            );
            i += 1;
        }
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < MAX_SUBSCRIPTIONS {
        release_subscription_curl(
            (&raw mut subscriptions as *mut onvif_subscription_t).offset(i_0 as isize),
        );
        i_0 += 1;
    }
    subscription_count = 0 as ::core::ffi::c_int;
    memset(
        &raw mut subscriptions as *mut onvif_subscription_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[onvif_subscription_t; 100]>(),
    );
    pthread_mutex_unlock(&raw mut subscription_mutex);
    pthread_mutex_lock(&raw mut curl_mutex);
    if !curl_handle.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up curl handle\0".as_ptr() as *const ::core::ffi::c_char,
        );
        curl_easy_cleanup(curl_handle);
        curl_handle = NULL as *mut ::core::ffi::c_void;
    }
    initialized = r#false != 0;
    pthread_mutex_unlock(&raw mut curl_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"ONVIF detection system shutdown complete\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_motion_onvif(
    mut onvif_url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut result: *mut detection_result_t,
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if is_shutdown_initiated() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF Detection: System shutdown in progress, skipping detection\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !result.is_null() {
        memset(
            result as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<detection_result_t>(),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF Detection: NULL result pointer provided\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !initialized || curl_handle.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF detection system not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if onvif_url.is_null() || username.is_null() || password.is_null() || result.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for detect_motion_onvif (NULL pointers not allowed)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if strlen(username) == 0 as size_t || strlen(password) == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF Detection: Using camera without authentication (empty credentials)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF Detection: Using camera with authentication\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"ONVIF Detection: Starting detection with URL: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        onvif_url,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"ONVIF Detection: Stream name: %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    let mut subscription: *mut onvif_subscription_t =
        get_subscription(onvif_url, username, password);
    if subscription.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get subscription for %s\0".as_ptr() as *const ::core::ffi::c_char,
            onvif_url,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut request_body: *const ::core::ffi::c_char = b"<PullMessages xmlns=\"http://www.onvif.org/ver10/events/wsdl\">\n  <Timeout>PT5S</Timeout>\n  <MessageLimit>100</MessageLimit>\n</PullMessages>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut event_timestamp: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut transport_dropped: bool = r#false != 0;
    let mut response: *mut ::core::ffi::c_char = send_subscription_request(
        subscription,
        request_body,
        onvif_operation_t::ONVIF_OPERATION_PULL_MESSAGES,
        &raw mut transport_dropped,
    );
    if response.is_null() {
        if !transport_dropped {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to pull messages from subscription\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        note_pull_failure(onvif_url);
        return if transport_dropped as ::core::ffi::c_int != 0 {
            ONVIF_DETECT_RETRY
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    note_pull_success(onvif_url);
    let mut event_label: [::core::ffi::c_char; 32] = [
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
    ];
    store_lpr_notifications(response, stream_name);
    let mut motion_detected: bool = has_motion_event(
        response,
        &raw mut event_label as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    );
    secure_clear(response as *mut ::core::ffi::c_void, strlen(response));
    free(response as *mut ::core::ffi::c_void);
    if motion_detected {
        if event_label[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            safe_strcpy(
                &raw mut event_label as *mut ::core::ffi::c_char,
                b"motion\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                0 as size_t,
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF Detection: %s detected for %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut event_label as *mut ::core::ffi::c_char,
            stream_name,
        );
        (*result).count = 1 as ::core::ffi::c_int;
        safe_strcpy(
            &raw mut (*(&raw mut (*result).detections as *mut detection_t).offset(0isize)).label
                as *mut ::core::ffi::c_char,
            &raw mut event_label as *mut ::core::ffi::c_char,
            MAX_LABEL_LENGTH as size_t,
            0 as size_t,
        );
        (*result).detections[0usize].confidence = 1.0f32;
        (*result).detections[0usize].x = 0.0f32;
        (*result).detections[0usize].y = 0.0f32;
        (*result).detections[0usize].width = 1.0f32;
        (*result).detections[0usize].height = 1.0f32;
        if !stream_name.is_null()
            && *stream_name.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"ONVIF Detection: Filtering detections by zones for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
            let mut filter_ret: ::core::ffi::c_int =
                filter_detections_by_zones(stream_name, result);
            if filter_ret != 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to filter detections by zones, storing all detections\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            let mut recording_id: uint64_t = get_current_recording_id_for_stream(stream_name);
            store_detections_in_db(stream_name, result, event_timestamp, recording_id);
            if (*result).count > 0 as ::core::ffi::c_int {
                let mut event_error: [::core::ffi::c_char; 256] = [
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
                if event_producer_publish_detection_for_stream(
                    stream_name,
                    result,
                    event_timestamp,
                    &raw mut event_error as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                ) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"ONVIF Detection: Event enqueue failed for %s: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        stream_name,
                        &raw mut event_error as *mut ::core::ffi::c_char,
                    );
                }
                process_motion_event(stream_name, r#true != 0, event_timestamp, r#false != 0);
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"No stream name provided, skipping database storage\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF Detection: No motion detected for %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        (*result).count = 0 as ::core::ffi::c_int;
        if !stream_name.is_null()
            && *stream_name.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            process_motion_event(stream_name, r#false != 0, event_timestamp, r#false != 0);
        }
    }
    return 0 as ::core::ffi::c_int;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
