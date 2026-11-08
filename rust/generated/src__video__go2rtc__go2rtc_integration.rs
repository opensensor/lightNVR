unsafe extern "C" {
    pub type stream_handle_s;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn curl_easy_strerror(error: CURLcode) -> *const ::core::ffi::c_char;
    unsafe fn curl_easy_init() -> *mut ::core::ffi::c_void;
    unsafe fn curl_easy_setopt(curl: *mut ::core::ffi::c_void, option: CURLoption, ...)
    -> CURLcode;
    unsafe fn curl_easy_perform(curl: *mut ::core::ffi::c_void) -> CURLcode;
    unsafe fn curl_easy_cleanup(curl: *mut ::core::ffi::c_void);
    static mut g_config: config_t;
    unsafe fn go2rtc_consumer_init() -> bool;
    unsafe fn go2rtc_consumer_stop_recording(stream_id: *const ::core::ffi::c_char) -> bool;
    unsafe fn go2rtc_consumer_cleanup();
    unsafe fn go2rtc_stream_init(
        binary_path: *const ::core::ffi::c_char,
        config_dir: *const ::core::ffi::c_char,
        api_port: ::core::ffi::c_int,
    ) -> bool;
    unsafe fn go2rtc_stream_register(
        stream_id: *const ::core::ffi::c_char,
        stream_url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        backchannel_enabled: bool,
        protocol: stream_protocol_t,
        record_audio: bool,
        codec: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn go2rtc_stream_register_substream(
        stream_id: *const ::core::ffi::c_char,
        stream_url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        protocol: stream_protocol_t,
    ) -> bool;
    unsafe fn go2rtc_stream_unregister(stream_id: *const ::core::ffi::c_char) -> bool;
    unsafe fn go2rtc_stream_tcp_port_open(
        host: *const ::core::ffi::c_char,
        port: ::core::ffi::c_int,
        timeout_ms: ::core::ffi::c_int,
    ) -> bool;
    unsafe fn go2rtc_stream_get_rtsp_url(
        stream_id: *const ::core::ffi::c_char,
        buffer: *mut ::core::ffi::c_char,
        buffer_size: size_t,
    ) -> bool;
    unsafe fn go2rtc_stream_is_initialized() -> bool;
    unsafe fn go2rtc_stream_is_ready() -> bool;
    unsafe fn go2rtc_stream_get_api_port() -> ::core::ffi::c_int;
    unsafe fn go2rtc_stream_start_service() -> bool;
    unsafe fn go2rtc_stream_invalidate_ready_cache();
    unsafe fn go2rtc_process_start(api_port: ::core::ffi::c_int) -> bool;
    unsafe fn go2rtc_process_stop() -> bool;
    unsafe fn go2rtc_lifecycle_begin(
        operation: go2rtc_lifecycle_operation_t,
        coalesce: bool,
        intentional: bool,
        guard: *mut go2rtc_lifecycle_guard_t,
    ) -> bool;
    unsafe fn go2rtc_lifecycle_end(guard: *mut go2rtc_lifecycle_guard_t, result: bool);
    unsafe fn go2rtc_lifecycle_intentional_restart_active() -> bool;
    unsafe fn go2rtc_lifecycle_restart_generation() -> uint64_t;
    unsafe fn go2rtc_api_get_base_path() -> *const ::core::ffi::c_char;
    unsafe fn go2rtc_api_stream_exists(stream_id: *const ::core::ffi::c_char) -> bool;
    unsafe fn go2rtc_api_preload_stream(stream_id: *const ::core::ffi::c_char) -> bool;
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
    unsafe fn is_shutdown_initiated() -> bool;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_config(
        handle: stream_handle_t,
        config: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_by_index(index: ::core::ffi::c_int) -> stream_handle_t;
    unsafe fn get_stream_by_name(name: *const ::core::ffi::c_char) -> stream_handle_t;
    unsafe fn get_total_stream_count() -> ::core::ffi::c_int;
    unsafe fn start_mp4_recording_with_trigger(
        stream_name: *const ::core::ffi::c_char,
        trigger_type: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn stop_mp4_recording(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn signal_all_mp4_recordings_reconnect();
    unsafe fn signal_mp4_recording_reconnect(stream_name: *const ::core::ffi::c_char);
    unsafe fn start_hls_stream(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn stop_hls_stream(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn get_all_stream_configs(
        streams: *mut stream_config_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_state_by_name(
        name: *const ::core::ffi::c_char,
    ) -> *mut stream_state_manager_t;
    unsafe fn get_unified_detection_effective_status(
        stream_name: *const ::core::ffi::c_char,
    ) -> stream_status_t;
    unsafe fn get_unified_detection_reconnect_attempts(
        stream_name: *const ::core::ffi::c_char,
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
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
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_protocol_state_t {
    pub protocol: stream_protocol_t,
    pub is_multicast: bool,
    pub reconnect_attempts: ::core::ffi::c_int,
    pub last_reconnect_time: int64_t,
    pub buffer_size: ::core::ffi::c_int,
    pub timeout_ms: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_state_manager_t {
    pub name: [::core::ffi::c_char; 256],
    pub state: stream_state_t,
    pub features: stream_features_t,
    pub protocol_state: stream_protocol_state_t,
    pub timestamp_state: stream_timestamp_state_t,
    pub stats: stream_stats_t,
    pub config: stream_config_t,
    pub mutex: pthread_mutex_t,
    pub state_mutex: pthread_mutex_t,
    pub ref_count: ::core::ffi::c_int,
    pub component_refs: [::core::ffi::c_int; 6],
    pub reader_ctx: *mut ::core::ffi::c_void,
    pub hls_ctx: *mut ::core::ffi::c_void,
    pub mp4_ctx: *mut ::core::ffi::c_void,
    pub detection_ctx: *mut ::core::ffi::c_void,
    pub callbacks_enabled: bool,
    pub last_error_message: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_stats_t {
    pub bytes_received: uint64_t,
    pub frames_received: uint64_t,
    pub frames_dropped: uint64_t,
    pub errors: uint64_t,
    pub reconnects: uint64_t,
    pub bitrate: ::core::ffi::c_double,
    pub fps: ::core::ffi::c_double,
    pub uptime: uint64_t,
    pub last_frame_time: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_timestamp_state_t {
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub expected_next_pts: int64_t,
    pub pts_discontinuity_count: int64_t,
    pub timestamps_initialized: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_features_t {
    pub streaming_enabled: bool,
    pub recording_enabled: bool,
    pub detection_enabled: bool,
    pub motion_detection_enabled: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_state_t(pub ::core::ffi::c_uint);
impl stream_state_t {
    pub const STREAM_STATE_INACTIVE: Self = Self(0);
    pub const STREAM_STATE_STARTING: Self = Self(1);
    pub const STREAM_STATE_ACTIVE: Self = Self(2);
    pub const STREAM_STATE_STOPPING: Self = Self(3);
    pub const STREAM_STATE_ERROR: Self = Self(4);
    pub const STREAM_STATE_RECONNECTING: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stuck_stream_tracker_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub last_bytes_recv: int64_t,
    pub last_bytes_send: int64_t,
    pub stalled_checks: ::core::ffi::c_int,
    pub last_check_time: time_t,
    pub tracking_start_time: time_t,
    pub tracking_active: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct go2rtc_stream_tracking_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub using_go2rtc_for_recording: bool,
    pub using_go2rtc_for_hls: bool,
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
pub type stream_handle_t = *mut stream_handle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_recovery_state_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub active: bool,
    pub attempts_since_progress: ::core::ffi::c_int,
    pub last_attempt_time: time_t,
    pub last_escalation_time: time_t,
    pub healthy_seen: bool,
    pub healthy_restart_generation: uint64_t,
    pub proxy_open_failures: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_status_t(pub ::core::ffi::c_uint);
impl stream_status_t {
    pub const STREAM_STATUS_UNKNOWN: Self = Self(0);
    pub const STREAM_STATUS_STOPPED: Self = Self(1);
    pub const STREAM_STATUS_STARTING: Self = Self(2);
    pub const STREAM_STATUS_RUNNING: Self = Self(3);
    pub const STREAM_STATUS_STOPPING: Self = Self(4);
    pub const STREAM_STATUS_ERROR: Self = Self(5);
    pub const STREAM_STATUS_RECONNECTING: Self = Self(6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_flow_t(pub ::core::ffi::c_uint);
impl stream_flow_t {
    pub const STREAM_FLOW_UNKNOWN: Self = Self(0);
    pub const STREAM_FLOW_MISSING: Self = Self(1);
    pub const STREAM_FLOW_FLOWING: Self = Self(2);
    pub const STREAM_FLOW_STALLED: Self = Self(3);
    pub const STREAM_FLOW_STUCK: Self = Self(4);
}
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
#[inline]
unsafe extern "C" fn configured_stream_slots() -> ::core::ffi::c_int {
    let mut slots: ::core::ffi::c_int = g_config.max_streams;
    if slots <= 0 as ::core::ffi::c_int || slots > MAX_STREAMS {
        return MAX_STREAMS;
    }
    return slots;
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL_0 as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL_0 as *const ::core::ffi::c_char;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAX_TRACKED_STREAMS: ::core::ffi::c_int = MAX_STREAMS;
static mut g_tracked_streams: [go2rtc_stream_tracking_t; 1024] = [
    go2rtc_stream_tracking_t {
        stream_name: [
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
        ],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
    go2rtc_stream_tracking_t {
        stream_name: [0; 256],
        using_go2rtc_for_recording: false,
        using_go2rtc_for_hls: false,
    },
];
static mut g_initialized: bool = r#false != 0;
pub const HEALTH_CHECK_INTERVAL_SEC: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const STREAM_MAX_CONSECUTIVE_FAILURES: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STREAM_REREGISTRATION_COOLDOWN_SEC: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const PROCESS_MAX_API_FAILURES: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PROCESS_MIN_STREAMS_FOR_CONSENSUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PROCESS_RESTART_COOLDOWN_SEC: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
pub const PROCESS_MAX_RESTARTS_PER_WINDOW: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PROCESS_RESTART_WINDOW_SEC: ::core::ffi::c_int = 600 as ::core::ffi::c_int;
pub const STUCK_STREAM_MAX_STALLED_CHECKS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STUCK_STREAM_WARMUP_SEC: ::core::ffi::c_int =
    HEALTH_CHECK_INTERVAL_SEC * (STUCK_STREAM_MAX_STALLED_CHECKS + 1 as ::core::ffi::c_int);
static mut g_stuck_trackers: [stuck_stream_tracker_t; 1024] = [
    stuck_stream_tracker_t {
        stream_name: [
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
        ],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
    stuck_stream_tracker_t {
        stream_name: [0; 256],
        last_bytes_recv: 0,
        last_bytes_send: 0,
        stalled_checks: 0,
        last_check_time: 0,
        tracking_start_time: 0,
        tracking_active: false,
    },
];
static mut g_stuck_tracker_mutex: pthread_mutex_t = pthread_mutex_t {
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
pub const STREAM_RECOVERY_BACKOFF_MAX_SEC: ::core::ffi::c_int =
    15 as ::core::ffi::c_int * 60 as ::core::ffi::c_int;
pub const STREAM_RECOVERY_ESCALATE_AFTER: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STREAM_RECOVERY_ESCALATION_COOLDOWN_SEC: ::core::ffi::c_int =
    30 as ::core::ffi::c_int * 60 as ::core::ffi::c_int;
pub const STREAM_PROXY_OPEN_FAILURE_THRESHOLD: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STREAM_CAMERA_PROBE_TIMEOUT_MS: ::core::ffi::c_int = 3000 as ::core::ffi::c_int;
static mut g_recovery_states: [stream_recovery_state_t; 1024] = [
    stream_recovery_state_t {
        stream_name: [
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
        ],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
    stream_recovery_state_t {
        stream_name: [0; 256],
        active: false,
        attempts_since_progress: 0,
        last_attempt_time: 0,
        last_escalation_time: 0,
        healthy_seen: false,
        healthy_restart_generation: 0,
        proxy_open_failures: 0,
    },
];
static mut g_recovery_mutex: pthread_mutex_t = pthread_mutex_t {
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
unsafe extern "C" fn recovery_state_locked(
    mut stream_name: *const ::core::ffi::c_char,
    mut create: bool,
) -> *mut stream_recovery_state_t {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TRACKED_STREAMS {
        if g_recovery_states[i as usize].active as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut g_recovery_states as *mut stream_recovery_state_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            return (&raw mut g_recovery_states as *mut stream_recovery_state_t).offset(i as isize);
        }
        i += 1;
    }
    if !create {
        return ::core::ptr::null_mut::<stream_recovery_state_t>();
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < MAX_TRACKED_STREAMS {
        if !g_recovery_states[i_0 as usize].active {
            memset(
                (&raw mut g_recovery_states as *mut stream_recovery_state_t).offset(i_0 as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stream_recovery_state_t>(),
            );
            safe_strcpy(
                &raw mut (*(&raw mut g_recovery_states as *mut stream_recovery_state_t)
                    .offset(i_0 as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            g_recovery_states[i_0 as usize].active = r#true != 0;
            return (&raw mut g_recovery_states as *mut stream_recovery_state_t)
                .offset(i_0 as isize);
        }
        i_0 += 1;
    }
    return ::core::ptr::null_mut::<stream_recovery_state_t>();
}
unsafe extern "C" fn recovery_cooldown_sec(
    mut attempts_since_progress: ::core::ffi::c_int,
) -> time_t {
    if attempts_since_progress < 0 as ::core::ffi::c_int {
        attempts_since_progress = 0 as ::core::ffi::c_int;
    }
    if attempts_since_progress > 8 as ::core::ffi::c_int {
        attempts_since_progress = 8 as ::core::ffi::c_int;
    }
    let mut cooldown: time_t =
        (STREAM_REREGISTRATION_COOLDOWN_SEC as time_t) << attempts_since_progress;
    if cooldown > STREAM_RECOVERY_BACKOFF_MAX_SEC as time_t {
        cooldown = STREAM_RECOVERY_BACKOFF_MAX_SEC as time_t;
    }
    return cooldown;
}
unsafe extern "C" fn recovery_cooldown_elapsed(
    mut stream_name: *const ::core::ffi::c_char,
    mut now: time_t,
) -> bool {
    let mut elapsed: bool = r#true != 0;
    pthread_mutex_lock(&raw mut g_recovery_mutex);
    let mut st: *mut stream_recovery_state_t = recovery_state_locked(stream_name, r#false != 0);
    if !st.is_null() && (*st).last_attempt_time > 0 as time_t {
        elapsed =
            now - (*st).last_attempt_time >= recovery_cooldown_sec((*st).attempts_since_progress);
    }
    pthread_mutex_unlock(&raw mut g_recovery_mutex);
    return elapsed;
}
unsafe extern "C" fn note_recovery_attempt(mut stream_name: *const ::core::ffi::c_char) {
    pthread_mutex_lock(&raw mut g_recovery_mutex);
    let mut st: *mut stream_recovery_state_t = recovery_state_locked(stream_name, r#true != 0);
    if !st.is_null() {
        if (*st).attempts_since_progress < INT_MAX {
            (*st).attempts_since_progress += 1;
        }
        (*st).last_attempt_time = time(::core::ptr::null_mut::<time_t>());
        (*st).proxy_open_failures = 0 as ::core::ffi::c_int;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s: scoped recovery #%d since last data flow; next one no sooner than %lds\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_name,
            (*st).attempts_since_progress,
            recovery_cooldown_sec((*st).attempts_since_progress),
        );
    }
    pthread_mutex_unlock(&raw mut g_recovery_mutex);
}
unsafe extern "C" fn note_stream_progress(mut stream_name: *const ::core::ffi::c_char) {
    let mut restart_generation: uint64_t = go2rtc_lifecycle_restart_generation();
    pthread_mutex_lock(&raw mut g_recovery_mutex);
    let mut st: *mut stream_recovery_state_t = recovery_state_locked(stream_name, r#true != 0);
    if !st.is_null() {
        if (*st).attempts_since_progress > 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s: data flow restored after %d scoped recover%s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                (*st).attempts_since_progress,
                if (*st).attempts_since_progress == 1 as ::core::ffi::c_int {
                    b"y\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"ies\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        (*st).attempts_since_progress = 0 as ::core::ffi::c_int;
        (*st).proxy_open_failures = 0 as ::core::ffi::c_int;
        (*st).healthy_seen = r#true != 0;
        (*st).healthy_restart_generation = restart_generation;
    }
    pthread_mutex_unlock(&raw mut g_recovery_mutex);
}
unsafe extern "C" fn note_escalation(mut stream_name: *const ::core::ffi::c_char) {
    pthread_mutex_lock(&raw mut g_recovery_mutex);
    let mut st: *mut stream_recovery_state_t = recovery_state_locked(stream_name, r#true != 0);
    if !st.is_null() {
        (*st).last_escalation_time = time(::core::ptr::null_mut::<time_t>());
        (*st).attempts_since_progress = 0 as ::core::ffi::c_int;
        (*st).proxy_open_failures = 0 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut g_recovery_mutex);
}
unsafe extern "C" fn recovery_proxy_open_failures(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut g_recovery_mutex);
    let mut st: *mut stream_recovery_state_t = recovery_state_locked(stream_name, r#false != 0);
    let mut failures: ::core::ffi::c_int = if !st.is_null() {
        (*st).proxy_open_failures
    } else {
        0 as ::core::ffi::c_int
    };
    pthread_mutex_unlock(&raw mut g_recovery_mutex);
    return failures;
}
unsafe extern "C" fn parse_url_host_port(
    mut url: *const ::core::ffi::c_char,
    mut host: *mut ::core::ffi::c_char,
    mut host_size: size_t,
    mut port: *mut ::core::ffi::c_int,
) -> bool {
    if url.is_null() || host.is_null() || host_size == 0 as size_t || port.is_null() {
        return r#false != 0;
    }
    let mut default_port: ::core::ffi::c_int = 554 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char =
        strstr(url, b"://\0".as_ptr() as *const ::core::ffi::c_char);
    if !p.is_null() {
        let mut scheme_len: size_t = p.offset_from(url) as size_t;
        if scheme_len == 4 as size_t
            && strncasecmp(
                url,
                b"http\0".as_ptr() as *const ::core::ffi::c_char,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            default_port = 80 as ::core::ffi::c_int;
        } else if scheme_len == 5 as size_t
            && strncasecmp(
                url,
                b"https\0".as_ptr() as *const ::core::ffi::c_char,
                5 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            default_port = 443 as ::core::ffi::c_int;
        } else if scheme_len == 5 as size_t
            && strncasecmp(
                url,
                b"rtsps\0".as_ptr() as *const ::core::ffi::c_char,
                5 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            default_port = 322 as ::core::ffi::c_int;
        }
        p = p.offset(3 as ::core::ffi::c_int as isize);
    } else {
        p = url;
    }
    let mut end: *const ::core::ffi::c_char =
        p.offset(strcspn(p, b"/?#\0".as_ptr() as *const ::core::ffi::c_char) as isize);
    let mut at: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut q: *const ::core::ffi::c_char = p;
    while q < end {
        if *q as ::core::ffi::c_int == '@' as ::core::ffi::c_int {
            at = q;
        }
        q = q.offset(1);
    }
    let mut hstart: *const ::core::ffi::c_char = if !at.is_null() {
        at.offset(1 as ::core::ffi::c_int as isize)
    } else {
        p
    };
    let mut hend: *const ::core::ffi::c_char = end;
    let mut pstart: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if hstart < end && *hstart as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
        let mut rb: *const ::core::ffi::c_char = memchr(
            hstart as *const ::core::ffi::c_void,
            ']' as ::core::ffi::c_int,
            end.offset_from(hstart) as size_t,
        ) as *const ::core::ffi::c_char;
        if rb.is_null() {
            return r#false != 0;
        }
        hstart = hstart.offset(1);
        hend = rb;
        if rb.offset(1 as ::core::ffi::c_int as isize) < end
            && *rb.offset(1isize) as ::core::ffi::c_int == ':' as ::core::ffi::c_int
        {
            pstart = rb.offset(2 as ::core::ffi::c_int as isize);
        }
    } else {
        let mut colon: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut q_0: *const ::core::ffi::c_char = hstart;
        while q_0 < end {
            if *q_0 as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                colon = q_0;
            }
            q_0 = q_0.offset(1);
        }
        if !colon.is_null() {
            hend = colon;
            pstart = colon.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    let mut hlen: size_t = hend.offset_from(hstart) as size_t;
    if hlen == 0 as size_t || hlen >= host_size {
        return r#false != 0;
    }
    memcpy(
        host as *mut ::core::ffi::c_void,
        hstart as *const ::core::ffi::c_void,
        hlen,
    );
    *host.offset(hlen as isize) = '\0' as ::core::ffi::c_char;
    *port = default_port;
    if !pstart.is_null() && pstart < end {
        let mut v: ::core::ffi::c_int = atoi(pstart);
        if v > 0 as ::core::ffi::c_int && v <= 65535 as ::core::ffi::c_int {
            *port = v;
        }
    }
    return r#true != 0;
}
unsafe extern "C" fn should_escalate_to_process_restart(
    mut config: *const stream_config_t,
) -> bool {
    if config.is_null()
        || (*config).name[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut current_generation: uint64_t = go2rtc_lifecycle_restart_generation();
    let mut attempts: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut eligible: bool = r#false != 0;
    pthread_mutex_lock(&raw mut g_recovery_mutex);
    let mut st: *mut stream_recovery_state_t = recovery_state_locked(
        &raw const (*config).name as *const ::core::ffi::c_char,
        r#false != 0,
    );
    if !st.is_null() {
        attempts = (*st).attempts_since_progress;
        eligible = (*st).healthy_seen as ::core::ffi::c_int != 0
            && (*st).healthy_restart_generation == current_generation
            && (*st).attempts_since_progress >= STREAM_RECOVERY_ESCALATE_AFTER
            && ((*st).last_escalation_time == 0 as time_t
                || now - (*st).last_escalation_time
                    >= STREAM_RECOVERY_ESCALATION_COOLDOWN_SEC as time_t);
    }
    pthread_mutex_unlock(&raw mut g_recovery_mutex);
    if !eligible {
        return r#false != 0;
    }
    let mut host: [::core::ffi::c_char; 256] = [0; 256];
    let mut port: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !parse_url_host_port(
        &raw const (*config).url as *const ::core::ffi::c_char,
        &raw mut host as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        &raw mut port,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s: cannot parse camera host from URL; not escalating\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*config).name as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !go2rtc_stream_tcp_port_open(
        &raw mut host as *mut ::core::ffi::c_char,
        port,
        STREAM_CAMERA_PROBE_TIMEOUT_MS,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s: %d scoped recoveries without data flow, but camera %s:%d is not reachable from LightNVR; not restarting go2rtc\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).name as *const ::core::ffi::c_char,
            attempts,
            &raw mut host as *mut ::core::ffi::c_char,
            port,
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream %s: %d scoped recoveries without data flow while camera %s:%d is reachable; escalating to a go2rtc process restart\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).name as *const ::core::ffi::c_char,
        attempts,
        &raw mut host as *mut ::core::ffi::c_char,
        port,
    );
    return r#true != 0;
}
static mut g_monitor_thread: pthread_t = 0;
static mut g_monitor_running: bool = r#false != 0;
static mut g_monitor_initialized: bool = r#false != 0;
static mut g_restart_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_last_restart_time: time_t = 0 as time_t;
static mut g_consecutive_api_failures: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_restart_history: [time_t; 5] = [0; 5];
static mut g_restart_history_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn stuck_stream_write_callback(
    mut contents: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> size_t {
    let mut realsize: size_t = size.wrapping_mul(nmemb);
    let mut buffer: *mut *mut ::core::ffi::c_char = userp as *mut *mut ::core::ffi::c_char;
    if (*buffer).is_null() {
        *buffer = malloc(realsize.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
        if (*buffer).is_null() {
            return 0 as size_t;
        }
        memcpy(*buffer as *mut ::core::ffi::c_void, contents, realsize);
        *(*buffer).offset(realsize as isize) = '\0' as ::core::ffi::c_char;
    } else {
        let mut old_len: size_t = strlen(*buffer);
        let mut new_buffer: *mut ::core::ffi::c_char = realloc(
            *buffer as *mut ::core::ffi::c_void,
            old_len.wrapping_add(realsize).wrapping_add(1 as size_t),
        ) as *mut ::core::ffi::c_char;
        if new_buffer.is_null() {
            return 0 as size_t;
        }
        *buffer = new_buffer;
        memcpy(
            (*buffer).offset(old_len as isize) as *mut ::core::ffi::c_void,
            contents,
            realsize,
        );
        *(*buffer).offset(old_len.wrapping_add(realsize) as isize) = '\0' as ::core::ffi::c_char;
    }
    return realsize;
}
unsafe extern "C" fn get_or_create_stuck_tracker(
    mut stream_name: *const ::core::ffi::c_char,
) -> *mut stuck_stream_tracker_t {
    pthread_mutex_lock(&raw mut g_stuck_tracker_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TRACKED_STREAMS {
        if g_stuck_trackers[i as usize].tracking_active as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut g_stuck_trackers as *mut stuck_stream_tracker_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            pthread_mutex_unlock(&raw mut g_stuck_tracker_mutex);
            return (&raw mut g_stuck_trackers as *mut stuck_stream_tracker_t).offset(i as isize);
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < MAX_TRACKED_STREAMS {
        if !g_stuck_trackers[i_0 as usize].tracking_active {
            memset(
                (&raw mut g_stuck_trackers as *mut stuck_stream_tracker_t).offset(i_0 as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stuck_stream_tracker_t>(),
            );
            safe_strcpy(
                &raw mut (*(&raw mut g_stuck_trackers as *mut stuck_stream_tracker_t)
                    .offset(i_0 as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            g_stuck_trackers[i_0 as usize].tracking_active = r#true != 0;
            g_stuck_trackers[i_0 as usize].last_bytes_recv = -1 as int64_t;
            g_stuck_trackers[i_0 as usize].last_bytes_send = -1 as int64_t;
            g_stuck_trackers[i_0 as usize].tracking_start_time =
                time(::core::ptr::null_mut::<time_t>());
            pthread_mutex_unlock(&raw mut g_stuck_tracker_mutex);
            return (&raw mut g_stuck_trackers as *mut stuck_stream_tracker_t).offset(i_0 as isize);
        }
        i_0 += 1;
    }
    pthread_mutex_unlock(&raw mut g_stuck_tracker_mutex);
    return ::core::ptr::null_mut::<stuck_stream_tracker_t>();
}
unsafe extern "C" fn reset_stuck_tracker(mut stream_name: *const ::core::ffi::c_char) {
    pthread_mutex_lock(&raw mut g_stuck_tracker_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TRACKED_STREAMS {
        if g_stuck_trackers[i as usize].tracking_active as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut g_stuck_trackers as *mut stuck_stream_tracker_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            g_stuck_trackers[i as usize].last_bytes_recv = -1 as int64_t;
            g_stuck_trackers[i as usize].last_bytes_send = -1 as int64_t;
            g_stuck_trackers[i as usize].stalled_checks = 0 as ::core::ffi::c_int;
            g_stuck_trackers[i as usize].last_check_time = 0 as time_t;
            g_stuck_trackers[i as usize].tracking_start_time =
                time(::core::ptr::null_mut::<time_t>());
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut g_stuck_tracker_mutex);
}
unsafe extern "C" fn check_stream_data_flow(
    mut stream_name: *const ::core::ffi::c_char,
) -> stream_flow_t {
    let mut t: *mut stuck_stream_tracker_t = get_or_create_stuck_tracker(stream_name);
    if !t.is_null() {
        pthread_mutex_lock(&raw mut g_stuck_tracker_mutex);
        let mut start: time_t = (*t).tracking_start_time;
        pthread_mutex_unlock(&raw mut g_stuck_tracker_mutex);
        let mut now_guard: time_t = time(::core::ptr::null_mut::<time_t>());
        if now_guard - start < STUCK_STREAM_WARMUP_SEC as time_t {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s: skipping stuck check (post-connect warmup, %lds remaining)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                (30 as ::core::ffi::c_int * (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                    as time_t
                    - (now_guard - start),
            );
            return stream_flow_t::STREAM_FLOW_UNKNOWN;
        }
    }
    let mut curl: *mut ::core::ffi::c_void = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to init CURL for stuck stream check\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return stream_flow_t::STREAM_FLOW_UNKNOWN;
    }
    let mut url: [::core::ffi::c_char; 512] = [0; 512];
    let mut api_port_val: ::core::ffi::c_int = go2rtc_stream_get_api_port();
    if api_port_val == 0 as ::core::ffi::c_int {
        api_port_val = 1984 as ::core::ffi::c_int;
    }
    snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"http://localhost:%d%s/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
        api_port_val,
        go2rtc_api_get_base_path(),
    );
    let mut response: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            stuck_stream_write_callback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, &raw mut response);
    curl_easy_setopt(curl, CURLoption::CURLOPT_TIMEOUT, 5 as ::core::ffi::c_long);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CONNECTTIMEOUT,
        3 as ::core::ffi::c_long,
    );
    let mut res: CURLcode = curl_easy_perform(curl);
    curl_easy_cleanup(curl);
    if res.0 != CURLcode::CURLE_OK.0 || response.is_null() {
        if !response.is_null() {
            free(response as *mut ::core::ffi::c_void);
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to fetch stream info for %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
            curl_easy_strerror(res),
        );
        return stream_flow_t::STREAM_FLOW_UNKNOWN;
    }
    let mut root: *mut cJSON = cJSON_Parse(response);
    free(response as *mut ::core::ffi::c_void);
    if root.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to parse stream info JSON for %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return stream_flow_t::STREAM_FLOW_UNKNOWN;
    }
    let mut tracker: *mut stuck_stream_tracker_t = get_or_create_stuck_tracker(stream_name);
    if tracker.is_null() {
        cJSON_Delete(root);
        return stream_flow_t::STREAM_FLOW_UNKNOWN;
    }
    let mut stream_obj: *mut cJSON = cJSON_GetObjectItemCaseSensitive(root, stream_name);
    if stream_obj.is_null()
        && (cJSON_HasObjectItem(root, b"producers\0".as_ptr() as *const ::core::ffi::c_char) != 0
            || cJSON_HasObjectItem(root, b"consumers\0".as_ptr() as *const ::core::ffi::c_char)
                != 0)
    {
        stream_obj = root;
    }
    if stream_obj.is_null() {
        cJSON_Delete(root);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found in go2rtc response\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return stream_flow_t::STREAM_FLOW_MISSING;
    }
    let mut total_bytes_recv: int64_t = 0 as int64_t;
    let mut total_bytes_send: int64_t = 0 as int64_t;
    let mut producers: *mut cJSON = cJSON_GetObjectItem(
        stream_obj,
        b"producers\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsArray(producers) != 0 {
        let mut producer: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
        producer = (if !producers.is_null() {
            (*producers).child
        } else {
            ::core::ptr::null_mut::<cJSON>()
        }) as *mut cJSON;
        while !producer.is_null() {
            let mut bytes: *mut cJSON = cJSON_GetObjectItem(
                producer,
                b"bytes_recv\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if cJSON_IsNumber(bytes) != 0 {
                total_bytes_recv += (*bytes).valuedouble as int64_t;
            }
            producer = (*producer).next as *mut cJSON;
        }
    }
    let mut consumers: *mut cJSON = cJSON_GetObjectItem(
        stream_obj,
        b"consumers\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsArray(consumers) != 0 {
        let mut consumer: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
        consumer = (if !consumers.is_null() {
            (*consumers).child
        } else {
            ::core::ptr::null_mut::<cJSON>()
        }) as *mut cJSON;
        while !consumer.is_null() {
            let mut bytes_0: *mut cJSON = cJSON_GetObjectItem(
                consumer,
                b"bytes_send\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if cJSON_IsNumber(bytes_0) != 0 {
                total_bytes_send += (*bytes_0).valuedouble as int64_t;
            }
            consumer = (*consumer).next as *mut cJSON;
        }
    }
    cJSON_Delete(root);
    let mut flow: stream_flow_t = stream_flow_t::STREAM_FLOW_UNKNOWN;
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_lock(&raw mut g_stuck_tracker_mutex);
    if (*tracker).last_bytes_recv == -1 as int64_t {
        (*tracker).last_bytes_recv = total_bytes_recv;
        (*tracker).last_bytes_send = total_bytes_send;
        (*tracker).last_check_time = now;
        (*tracker).stalled_checks = 0 as ::core::ffi::c_int;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s: initialized byte tracking (recv=%lld, send=%lld)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            total_bytes_recv as ::core::ffi::c_longlong,
            total_bytes_send as ::core::ffi::c_longlong,
        );
    } else {
        let mut data_flowing: bool = total_bytes_recv > (*tracker).last_bytes_recv
            || total_bytes_send > (*tracker).last_bytes_send;
        if data_flowing {
            if (*tracker).stalled_checks > 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stream %s: data flow resumed (recv=%lld, send=%lld)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                    total_bytes_recv as ::core::ffi::c_longlong,
                    total_bytes_send as ::core::ffi::c_longlong,
                );
            }
            (*tracker).stalled_checks = 0 as ::core::ffi::c_int;
            flow = stream_flow_t::STREAM_FLOW_FLOWING;
        } else {
            (*tracker).stalled_checks += 1;
            flow = stream_flow_t::STREAM_FLOW_STALLED;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s: no data flow detected for %d consecutive checks (recv=%lld unchanged, send=%lld unchanged)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                (*tracker).stalled_checks,
                total_bytes_recv as ::core::ffi::c_longlong,
                total_bytes_send as ::core::ffi::c_longlong,
            );
            if (*tracker).stalled_checks >= STUCK_STREAM_MAX_STALLED_CHECKS {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stream %s: appears STUCK - no data flow for %d checks\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                    (*tracker).stalled_checks,
                );
                flow = stream_flow_t::STREAM_FLOW_STUCK;
            }
        }
        (*tracker).last_bytes_recv = total_bytes_recv;
        (*tracker).last_bytes_send = total_bytes_send;
        (*tracker).last_check_time = now;
    }
    pthread_mutex_unlock(&raw mut g_stuck_tracker_mutex);
    return flow;
}
unsafe extern "C" fn find_tracked_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> *mut go2rtc_stream_tracking_t {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TRACKED_STREAMS {
        if g_tracked_streams[i as usize].stream_name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
            && strcmp(
                &raw mut (*(&raw mut g_tracked_streams as *mut go2rtc_stream_tracking_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            return (&raw mut g_tracked_streams as *mut go2rtc_stream_tracking_t)
                .offset(i as isize);
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<go2rtc_stream_tracking_t>();
}
unsafe extern "C" fn add_tracked_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> *mut go2rtc_stream_tracking_t {
    let mut existing: *mut go2rtc_stream_tracking_t = find_tracked_stream(stream_name);
    if !existing.is_null() {
        return existing;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TRACKED_STREAMS {
        if g_tracked_streams[i as usize].stream_name[0usize] as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*(&raw mut g_tracked_streams as *mut go2rtc_stream_tracking_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            g_tracked_streams[i as usize].using_go2rtc_for_recording = r#false != 0;
            g_tracked_streams[i as usize].using_go2rtc_for_hls = r#false != 0;
            return (&raw mut g_tracked_streams as *mut go2rtc_stream_tracking_t)
                .offset(i as isize);
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<go2rtc_stream_tracking_t>();
}
unsafe extern "C" fn is_stream_registered_with_go2rtc(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service is not ready, cannot check if stream is registered\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut exists: bool = go2rtc_api_stream_exists(stream_name);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream %s %s registered with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        if exists as ::core::ffi::c_int != 0 {
            b"is\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"is NOT\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return exists;
}
unsafe extern "C" fn ensure_stream_registered_with_go2rtc(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if !go2rtc_stream_is_ready() {
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
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Waiting for go2rtc service to start... (%d retries left)\0".as_ptr()
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
    let mut stream: stream_handle_t = get_stream_by_name(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return r#false != 0;
    }
    let mut config: stream_config_t = stream_config_t {
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
    if get_stream_config(stream, &raw mut config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get config for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return r#false != 0;
    }
    if !g_config.audio_disabled
        && config.go2rtc_source_override[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s is YAML-defined by go2rtc source override; restarting go2rtc to load config\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return go2rtc_integration_restart_process();
    }
    if is_stream_registered_with_go2rtc(stream_name) {
        return r#true != 0;
    }
    if !go2rtc_stream_register(
        stream_name,
        &raw mut config.url as *mut ::core::ffi::c_char,
        if config.onvif_username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            &raw mut config.onvif_username as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        if config.onvif_password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            &raw mut config.onvif_password as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        config.backchannel_enabled,
        config.protocol,
        config.record_audio,
        &raw mut config.codec as *mut ::core::ffi::c_char,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to register stream %s with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Successfully registered stream %s with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    return r#true != 0;
}
unsafe extern "C" fn stream_needs_reregistration(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if stream_name.is_null() {
        return r#false != 0;
    }
    let mut state: *mut stream_state_manager_t = get_stream_state_by_name(stream_name);
    if state.is_null() {
        return r#false != 0;
    }
    if !(*state).config.enabled {
        return r#false != 0;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut cooldown_elapsed: bool = recovery_cooldown_elapsed(stream_name, now);
    if (*state).state.0 == stream_state_t::STREAM_STATE_ERROR.0
        || (*state).state.0 == stream_state_t::STREAM_STATE_RECONNECTING.0
    {
        let mut failures: ::core::ffi::c_int =
            ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*state).protocol_state.reconnect_attempts,
            );
        if failures >= STREAM_MAX_CONSECUTIVE_FAILURES
            && cooldown_elapsed as ::core::ffi::c_int != 0
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s has %d consecutive failures (state-manager), needs re-registration\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                failures,
            );
            return r#true != 0;
        }
        return r#false != 0;
    }
    if (*state).state.0 == stream_state_t::STREAM_STATE_INACTIVE.0 {
        let mut udt_status: stream_status_t = get_unified_detection_effective_status(stream_name);
        if udt_status.0 == stream_status_t::STREAM_STATUS_RECONNECTING.0 {
            let mut udt_attempts: ::core::ffi::c_int =
                get_unified_detection_reconnect_attempts(stream_name);
            if udt_attempts >= STREAM_MAX_CONSECUTIVE_FAILURES
                && cooldown_elapsed as ::core::ffi::c_int != 0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stream %s has %d UDT reconnect attempts, needs go2rtc re-registration\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    stream_name,
                    udt_attempts,
                );
                return r#true != 0;
            }
        }
    }
    let mut proxy_failures: ::core::ffi::c_int = recovery_proxy_open_failures(stream_name);
    if proxy_failures >= STREAM_PROXY_OPEN_FAILURE_THRESHOLD
        && cooldown_elapsed as ::core::ffi::c_int != 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s has %d consecutive local RTSP open failures, needs go2rtc re-registration\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_name,
            proxy_failures,
        );
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn recover_stream_scoped(mut config: *const stream_config_t) -> bool {
    if config.is_null()
        || (*config).name[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if g_config.audio_disabled as ::core::ffi::c_int != 0
        || (*config).go2rtc_source_override[0usize] as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
    {
        return go2rtc_integration_reload_stream(
            &raw const (*config).name as *const ::core::ffi::c_char,
        );
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot recover YAML-backed stream %s while go2rtc is unavailable\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*config).name as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream %s uses a go2rtc source override; preserving the shared go2rtc process and reconnecting only this stream's consumers\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).name as *const ::core::ffi::c_char,
    );
    return r#true != 0;
}
unsafe extern "C" fn check_stream_consensus() -> bool {
    let mut total_streams: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut failed_streams: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream_count: ::core::ffi::c_int = get_total_stream_count();
    if stream_count < PROCESS_MIN_STREAMS_FOR_CONSENSUS {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < stream_count {
        let mut stream: stream_handle_t = get_stream_by_index(i);
        if !stream.is_null() {
            let mut config: stream_config_t = stream_config_t {
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
            if get_stream_config(stream, &raw mut config) == 0 as ::core::ffi::c_int {
                if config.enabled {
                    total_streams += 1;
                    let mut state: *mut stream_state_manager_t =
                        get_stream_state_by_name(&raw mut config.name as *mut ::core::ffi::c_char);
                    if !state.is_null() {
                        pthread_mutex_lock(&raw mut (*state).mutex);
                        let mut current_state: stream_state_t = (*state).state;
                        pthread_mutex_unlock(&raw mut (*state).mutex);
                        if current_state.0 == stream_state_t::STREAM_STATE_ERROR.0
                            || current_state.0 == stream_state_t::STREAM_STATE_RECONNECTING.0
                        {
                            failed_streams += 1;
                        } else if current_state.0 == stream_state_t::STREAM_STATE_INACTIVE.0 {
                            if get_unified_detection_effective_status(
                                &raw mut config.name as *mut ::core::ffi::c_char,
                            )
                            .0 == stream_status_t::STREAM_STATUS_RECONNECTING.0
                            {
                                failed_streams += 1;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if total_streams >= PROCESS_MIN_STREAMS_FOR_CONSENSUS
        && failed_streams * 2 as ::core::ffi::c_int > total_streams
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream consensus: %d/%d streams failed (majority) - indicates go2rtc issue\0".as_ptr()
                as *const ::core::ffi::c_char,
            failed_streams,
            total_streams,
        );
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn can_restart_go2rtc() -> bool {
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if now - g_last_restart_time < PROCESS_RESTART_COOLDOWN_SEC as time_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc restart blocked: cooldown period (%ld seconds remaining)\0".as_ptr()
                as *const ::core::ffi::c_char,
            120 as time_t - (now - g_last_restart_time),
        );
        return r#false != 0;
    }
    let mut recent_restarts: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < PROCESS_MAX_RESTARTS_PER_WINDOW {
        if g_restart_history[i as usize] > 0 as time_t
            && now - g_restart_history[i as usize] < PROCESS_RESTART_WINDOW_SEC as time_t
        {
            recent_restarts += 1;
        }
        i += 1;
    }
    if recent_restarts >= PROCESS_MAX_RESTARTS_PER_WINDOW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc restart blocked: too many restarts (%d in last %d seconds)\0".as_ptr()
                as *const ::core::ffi::c_char,
            recent_restarts,
            600 as ::core::ffi::c_int,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn restart_go2rtc_process_locked() -> bool {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Attempting to restart managed go2rtc process\0".as_ptr() as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stopping go2rtc process...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_process_stop() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to stop go2rtc process\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    go2rtc_stream_invalidate_ready_cache();
    sleep(2 as ::core::ffi::c_uint);
    let mut api_port: ::core::ffi::c_int = go2rtc_stream_get_api_port();
    if api_port == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get go2rtc API port\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting go2rtc process...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_process_start(api_port) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start go2rtc process\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut retries: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    while retries > 0 as ::core::ffi::c_int && !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Waiting for go2rtc to be ready after restart... (%d retries left)\0".as_ptr()
                as *const ::core::ffi::c_char,
            retries,
        );
        sleep(2 as ::core::ffi::c_uint);
        retries -= 1;
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc failed to become ready after restart\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc process restarted successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Re-registering all streams with go2rtc after restart\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if !go2rtc_integration_register_all_streams() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to re-register all streams after go2rtc restart\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"All streams re-registered successfully after go2rtc restart\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TRACKED_STREAMS {
        if g_tracked_streams[i as usize].stream_name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
        {
            g_tracked_streams[i as usize].using_go2rtc_for_hls = r#false != 0;
        }
        i += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleared HLS preload tracking to force re-preload after go2rtc restart\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    sleep(2 as ::core::ffi::c_uint);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Signaling MP4 recordings to reconnect after go2rtc restart\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    signal_all_mp4_recordings_reconnect();
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    g_last_restart_time = now;
    g_restart_count += 1;
    g_restart_history[g_restart_history_index as usize] = now;
    g_restart_history_index =
        (g_restart_history_index + 1 as ::core::ffi::c_int) % PROCESS_MAX_RESTARTS_PER_WINDOW;
    g_consecutive_api_failures = 0 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc restart completed (total restarts: %d)\0".as_ptr() as *const ::core::ffi::c_char,
        g_restart_count,
    );
    return r#true != 0;
}
unsafe extern "C" fn restart_go2rtc_process(mut intentional: bool) -> bool {
    let mut guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    if !go2rtc_lifecycle_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_RESTART,
        r#true != 0,
        intentional,
        &raw mut guard,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to enter go2rtc lifecycle for restart\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if guard.coalesced {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Coalesced go2rtc restart with lifecycle generation %llu\0".as_ptr()
                as *const ::core::ffi::c_char,
            guard.generation as ::core::ffi::c_ulonglong,
        );
        return guard.result;
    }
    let mut restarted: bool = false;
    if !go2rtc_stream_is_initialized() || !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc modules are not fully initialized, performing full start\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        restarted = go2rtc_integration_full_start_locked();
    } else {
        restarted = restart_go2rtc_process_locked();
    }
    go2rtc_lifecycle_end(&raw mut guard, restarted);
    return restarted;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_restart_process() -> bool {
    return restart_go2rtc_process(r#true != 0);
}
unsafe extern "C" fn unified_health_monitor_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unified go2rtc health monitor thread started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut process_restarted: bool = r#false != 0;
    while g_monitor_running as ::core::ffi::c_int != 0 && !is_shutdown_initiated() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < HEALTH_CHECK_INTERVAL_SEC
            && g_monitor_running as ::core::ffi::c_int != 0
            && !is_shutdown_initiated()
        {
            sleep(1 as ::core::ffi::c_uint);
            i += 1;
        }
        if !g_monitor_running || is_shutdown_initiated() as ::core::ffi::c_int != 0 {
            break;
        }
        if go2rtc_lifecycle_intentional_restart_active() {
            g_consecutive_api_failures = 0 as ::core::ffi::c_int;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Skipping go2rtc health check during intentional lifecycle restart\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if !go2rtc_stream_is_initialized() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc stream module not initialized, skipping health check\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            let mut api_healthy: bool = go2rtc_stream_is_ready();
            if !api_healthy {
                g_consecutive_api_failures += 1;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"go2rtc API health check failed (consecutive failures: %d/%d)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    g_consecutive_api_failures,
                    3 as ::core::ffi::c_int,
                );
                if g_consecutive_api_failures >= PROCESS_MAX_API_FAILURES {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"go2rtc API has failed %d consecutive health checks\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        g_consecutive_api_failures,
                    );
                    let mut consensus_failure: bool = check_stream_consensus();
                    let mut total_stream_count: ::core::ffi::c_int = get_total_stream_count();
                    let mut should_restart: bool = consensus_failure as ::core::ffi::c_int != 0
                        || total_stream_count < PROCESS_MIN_STREAMS_FOR_CONSENSUS;
                    if should_restart {
                        if consensus_failure {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Stream consensus also indicates go2rtc failure - restart required\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Too few streams (%d) for consensus, restarting on API failure alone\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                total_stream_count,
                            );
                        }
                        if can_restart_go2rtc() {
                            if restart_go2rtc_process(r#false != 0) {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"go2rtc process successfully restarted\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                process_restarted = r#true != 0;
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_ERROR,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Failed to restart go2rtc process\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        }
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"go2rtc API unhealthy (%d consecutive failures) but stream consensus OK \xE2\x80\x94 skipping process restart to avoid disrupting working streams\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            g_consecutive_api_failures,
                        );
                    }
                }
            } else {
                if g_consecutive_api_failures > 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"go2rtc API health check succeeded, resetting failure counter\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    g_consecutive_api_failures = 0 as ::core::ffi::c_int;
                }
                if process_restarted {
                    process_restarted = r#false != 0;
                } else {
                    let mut stream_count: ::core::ffi::c_int = get_total_stream_count();
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Health monitor checking %d streams\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        stream_count,
                    );
                    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    's_419: while i_0 < stream_count {
                        if !g_monitor_running || is_shutdown_initiated() as ::core::ffi::c_int != 0
                        {
                            break;
                        }
                        let mut stream: stream_handle_t = get_stream_by_index(i_0);
                        's_203: {
                            if !stream.is_null() {
                                let mut config: stream_config_t = stream_config_t {
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
                                if get_stream_config(stream, &raw mut config)
                                    == 0 as ::core::ffi::c_int
                                {
                                    let mut yaml_backed: bool = !g_config.audio_disabled
                                        && config.go2rtc_source_override[0usize]
                                            as ::core::ffi::c_int
                                            != '\0' as ::core::ffi::c_int;
                                    if get_unified_detection_effective_status(
                                        &raw mut config.name as *mut ::core::ffi::c_char,
                                    )
                                    .0 == stream_status_t::STREAM_STATUS_RUNNING.0
                                    {
                                        note_stream_progress(
                                            &raw mut config.name as *mut ::core::ffi::c_char,
                                        );
                                    }
                                    let mut recovery_needed: bool = stream_needs_reregistration(
                                        &raw mut config.name as *mut ::core::ffi::c_char,
                                    );
                                    let mut reason: *const ::core::ffi::c_char =
                                        b"state-based\0".as_ptr() as *const ::core::ffi::c_char;
                                    if !recovery_needed {
                                        let mut flow: stream_flow_t = check_stream_data_flow(
                                            &raw mut config.name as *mut ::core::ffi::c_char,
                                        );
                                        if flow.0 == stream_flow_t::STREAM_FLOW_FLOWING.0 {
                                            note_stream_progress(
                                                &raw mut config.name as *mut ::core::ffi::c_char,
                                            );
                                            break 's_203;
                                        } else if flow.0 == stream_flow_t::STREAM_FLOW_MISSING.0 {
                                            if !config.enabled
                                                || config.privacy_mode as ::core::ffi::c_int != 0
                                                || yaml_backed as ::core::ffi::c_int != 0
                                                || !recovery_cooldown_elapsed(
                                                    &raw mut config.name
                                                        as *mut ::core::ffi::c_char,
                                                    time(::core::ptr::null_mut::<time_t>()),
                                                )
                                            {
                                                break 's_203;
                                            } else {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_WARN,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Stream %s is missing from go2rtc; re-registering it\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    &raw mut config.name as *mut ::core::ffi::c_char,
                                                );
                                                note_recovery_attempt(
                                                    &raw mut config.name
                                                        as *mut ::core::ffi::c_char,
                                                );
                                                if go2rtc_stream_register(
                                                    &raw mut config.name
                                                        as *mut ::core::ffi::c_char,
                                                    &raw mut config.url as *mut ::core::ffi::c_char,
                                                    if config.onvif_username[0usize]
                                                        as ::core::ffi::c_int
                                                        != '\0' as ::core::ffi::c_int
                                                    {
                                                        &raw mut config.onvif_username
                                                            as *mut ::core::ffi::c_char
                                                    } else {
                                                        ::core::ptr::null_mut::<::core::ffi::c_char>(
                                                        )
                                                    },
                                                    if config.onvif_password[0usize]
                                                        as ::core::ffi::c_int
                                                        != '\0' as ::core::ffi::c_int
                                                    {
                                                        &raw mut config.onvif_password
                                                            as *mut ::core::ffi::c_char
                                                    } else {
                                                        ::core::ptr::null_mut::<::core::ffi::c_char>(
                                                        )
                                                    },
                                                    config.backchannel_enabled,
                                                    config.protocol,
                                                    config.record_audio,
                                                    &raw mut config.codec
                                                        as *mut ::core::ffi::c_char,
                                                ) {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_INFO,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Re-registered missing stream %s with go2rtc\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        &raw mut config.name as *mut ::core::ffi::c_char,
                                                    );
                                                    reset_stuck_tracker(
                                                        &raw mut config.name
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                    signal_mp4_recording_reconnect(
                                                        &raw mut config.name
                                                            as *mut ::core::ffi::c_char,
                                                    );
                                                } else {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_ERROR,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Failed to re-register missing stream %s with go2rtc\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        &raw mut config.name as *mut ::core::ffi::c_char,
                                                    );
                                                }
                                                break 's_203;
                                            }
                                        } else if flow.0 != stream_flow_t::STREAM_FLOW_STUCK.0 {
                                            break 's_203;
                                        } else if !recovery_cooldown_elapsed(
                                            &raw mut config.name as *mut ::core::ffi::c_char,
                                            time(::core::ptr::null_mut::<time_t>()),
                                        ) {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_INFO,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Stream %s is stuck but its recovery cooldown has not elapsed; waiting\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                &raw mut config.name as *mut ::core::ffi::c_char,
                                            );
                                            break 's_203;
                                        } else {
                                            recovery_needed = r#true != 0;
                                            reason =
                                                b"stuck\0".as_ptr() as *const ::core::ffi::c_char;
                                        }
                                    }
                                    if config.enabled {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Stream %s needs recovery (%s)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            &raw mut config.name as *mut ::core::ffi::c_char,
                                            reason,
                                        );
                                        if should_escalate_to_process_restart(&raw mut config) {
                                            if can_restart_go2rtc() as ::core::ffi::c_int != 0
                                                && restart_go2rtc_process(r#false != 0)
                                                    as ::core::ffi::c_int
                                                    != 0
                                            {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"go2rtc process restarted to recover stream %s\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    &raw mut config.name as *mut ::core::ffi::c_char,
                                                );
                                                note_escalation(
                                                    &raw mut config.name
                                                        as *mut ::core::ffi::c_char,
                                                );
                                                process_restarted = r#true != 0;
                                                break 's_419;
                                            } else {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_WARN,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"go2rtc restart for stream %s was rate-limited or failed; falling back to a scoped recovery\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    &raw mut config.name as *mut ::core::ffi::c_char,
                                                );
                                            }
                                        }
                                        note_recovery_attempt(
                                            &raw mut config.name as *mut ::core::ffi::c_char,
                                        );
                                        if recover_stream_scoped(&raw mut config) {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_INFO,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Completed stream-scoped recovery for %s (%s)\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                &raw mut config.name as *mut ::core::ffi::c_char,
                                                reason,
                                            );
                                            reset_stuck_tracker(
                                                &raw mut config.name as *mut ::core::ffi::c_char,
                                            );
                                            let mut state: *mut stream_state_manager_t =
                                                get_stream_state_by_name(
                                                    &raw mut config.name
                                                        as *mut ::core::ffi::c_char,
                                                );
                                            if !state.is_null() {
                                                ::core::intrinsics::atomic_store::<
                                                    _,
                                                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                                                >(
                                                    &raw mut (*state)
                                                        .protocol_state
                                                        .last_reconnect_time,
                                                    time(::core::ptr::null_mut::<time_t>()),
                                                );
                                                ::core::intrinsics::atomic_store::<
                                                    _,
                                                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                                                >(
                                                    &raw mut (*state)
                                                        .protocol_state
                                                        .reconnect_attempts,
                                                    0 as ::core::ffi::c_int,
                                                );
                                            }
                                            signal_mp4_recording_reconnect(
                                                &raw mut config.name as *mut ::core::ffi::c_char,
                                            );
                                        } else {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_ERROR,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Failed stream-scoped recovery for %s (%s)\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                &raw mut config.name as *mut ::core::ffi::c_char,
                                                reason,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        i_0 += 1;
                    }
                }
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unified go2rtc health monitor thread exiting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return NULL_0;
}
unsafe extern "C" fn start_unified_health_monitor() -> bool {
    if g_monitor_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unified health monitor already initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting unified go2rtc health monitor\0".as_ptr() as *const ::core::ffi::c_char,
    );
    memset(
        &raw mut g_restart_history as *mut time_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[time_t; 5]>(),
    );
    g_restart_history_index = 0 as ::core::ffi::c_int;
    g_restart_count = 0 as ::core::ffi::c_int;
    g_last_restart_time = 0 as time_t;
    g_consecutive_api_failures = 0 as ::core::ffi::c_int;
    g_monitor_running = r#true != 0;
    g_monitor_initialized = r#true != 0;
    if pthread_create(
        &raw mut g_monitor_thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            unified_health_monitor_thread
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL_0,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create unified health monitor thread\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        g_monitor_running = r#false != 0;
        g_monitor_initialized = r#false != 0;
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unified go2rtc health monitor started successfully\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return r#true != 0;
}
unsafe extern "C" fn stop_unified_health_monitor() {
    if !g_monitor_initialized {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stopping unified go2rtc health monitor\0".as_ptr() as *const ::core::ffi::c_char,
    );
    g_monitor_running = r#false != 0;
    if pthread_join(
        g_monitor_thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to join unified health monitor thread\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    g_monitor_initialized = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unified go2rtc health monitor stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_init() -> bool {
    if g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc integration module already initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    if !go2rtc_consumer_init() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize go2rtc consumer module\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    memset(
        &raw mut g_tracked_streams as *mut go2rtc_stream_tracking_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (configured_stream_slots() as size_t)
            .wrapping_mul(::core::mem::size_of::<go2rtc_stream_tracking_t>()),
    );
    if !start_unified_health_monitor() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start unified health monitor (non-fatal)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    g_initialized = r#true != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc integration module initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return r#true != 0;
}
unsafe extern "C" fn go2rtc_integration_full_start_locked() -> bool {
    let mut binary_path: *const ::core::ffi::c_char = if g_config.go2rtc_binary_path[0usize]
        as ::core::ffi::c_int
        != '\0' as ::core::ffi::c_int
    {
        &raw mut g_config.go2rtc_binary_path as *mut ::core::ffi::c_char
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    let mut config_dir: *const ::core::ffi::c_char =
        if g_config.go2rtc_config_dir[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            &raw mut g_config.go2rtc_config_dir as *mut ::core::ffi::c_char
        } else {
            b"/tmp/go2rtc\0".as_ptr() as *const ::core::ffi::c_char
        };
    let mut api_port: ::core::ffi::c_int = if g_config.go2rtc_api_port > 0 as ::core::ffi::c_int {
        g_config.go2rtc_api_port
    } else {
        1984 as ::core::ffi::c_int
    };
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc full start (binary=%s, config_dir=%s, api_port=%d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        if !binary_path.is_null() {
            binary_path
        } else {
            b"PATH\0".as_ptr() as *const ::core::ffi::c_char
        },
        config_dir,
        api_port,
    );
    if !go2rtc_stream_is_initialized() {
        if !go2rtc_stream_init(binary_path, config_dir, api_port) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to initialize go2rtc stream module\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc stream module initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc stream module already initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !go2rtc_stream_start_service() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start go2rtc service\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc service started and ready\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_integration_is_initialized() {
        if !go2rtc_integration_init() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to initialize go2rtc integration module\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Registering all existing streams with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_integration_register_all_streams() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to register all streams with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Waiting for streams to be fully registered with go2rtc...\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 12 as ::core::ffi::c_int {
            usleep(250000 as ::core::ffi::c_int as __useconds_t);
            if go2rtc_stream_is_ready() {
                break;
            }
            i += 1;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Streams registered with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc full start complete\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_full_start() -> bool {
    let mut guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    if !go2rtc_lifecycle_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_FULL_START,
        r#true != 0,
        r#true != 0,
        &raw mut guard,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to enter go2rtc lifecycle for full start\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if guard.coalesced {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Coalesced go2rtc full start with lifecycle generation %llu\0".as_ptr()
                as *const ::core::ffi::c_char,
            guard.generation as ::core::ffi::c_ulonglong,
        );
        return guard.result;
    }
    let mut result: bool = go2rtc_integration_full_start_locked();
    go2rtc_lifecycle_end(&raw mut guard, result);
    return result;
}
unsafe extern "C" fn ensure_go2rtc_ready_for_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service is not ready, starting it...\0".as_ptr() as *const ::core::ffi::c_char,
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
        let mut retries: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
        while retries > 0 as ::core::ffi::c_int && !go2rtc_stream_is_ready() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Waiting for go2rtc service to start... (%d retries left)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                retries,
            );
            sleep(2 as ::core::ffi::c_uint);
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
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service started successfully\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !is_stream_registered_with_go2rtc(stream_name) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s is not registered with go2rtc, registering it...\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        if !ensure_stream_registered_with_go2rtc(stream_name) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to register stream %s with go2rtc\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
            return r#false != 0;
        }
        usleep(500000 as ::core::ffi::c_int as __useconds_t);
        if !is_stream_registered_with_go2rtc(stream_name) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s not yet visible in go2rtc /api/streams after registration (may be transient), proceeding anyway\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s confirmed registered with go2rtc\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        }
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_start_recording(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return go2rtc_integration_start_recording_with_trigger(
        stream_name,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_start_recording_with_trigger(
    mut stream_name: *const ::core::ffi::c_char,
    mut trigger_type: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc integration not initialized, using direct MP4 recording for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return start_mp4_recording_with_trigger(stream_name, trigger_type);
    }
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for go2rtc_integration_start_recording\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: stream_handle_t = get_stream_by_name(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut config: stream_config_t = stream_config_t {
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
    if get_stream_config(stream, &raw mut config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get config for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut using_go2rtc: bool = ensure_go2rtc_ready_for_stream(stream_name);
    if using_go2rtc {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using go2rtc's RTSP output as input for MP4 recording of stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        let mut tracking: *mut go2rtc_stream_tracking_t = add_tracked_stream(stream_name);
        if !tracking.is_null() {
            (*tracking).using_go2rtc_for_recording = r#true != 0;
        }
        let mut result: ::core::ffi::c_int =
            start_mp4_recording_with_trigger(stream_name, trigger_type);
        if result == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Started MP4 recording for stream %s using go2rtc's RTSP output\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        }
        return result;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using default recording for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return start_mp4_recording_with_trigger(stream_name, trigger_type);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_stop_recording(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc integration not initialized, using direct stop for recording %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return stop_mp4_recording(stream_name);
    }
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for go2rtc_integration_stop_recording\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut tracking: *mut go2rtc_stream_tracking_t = find_tracked_stream(stream_name);
    if !tracking.is_null() && (*tracking).using_go2rtc_for_recording as ::core::ffi::c_int != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopping recording for stream %s (go2rtc RTSP path)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        let mut ret: ::core::ffi::c_int = stop_mp4_recording(stream_name);
        if ret != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to stop recording for stream %s (error: %d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                ret,
            );
            return ret;
        }
        (*tracking).using_go2rtc_for_recording = r#false != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopped recording for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return 0 as ::core::ffi::c_int;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using default method to stop recording for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return stop_mp4_recording(stream_name);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_start_hls(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc integration not initialized, using direct HLS streaming for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return start_hls_stream(stream_name);
    }
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for go2rtc_integration_start_hls\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if is_shutdown_initiated() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot start HLS stream %s during shutdown\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: stream_handle_t = get_stream_by_name(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut config: stream_config_t = stream_config_t {
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
    if get_stream_config(stream, &raw mut config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get config for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    if g_config.go2rtc_force_native_hls {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"force_native_hls enabled, using ffmpeg-based HLS for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return start_hls_stream(stream_name);
    }
    let mut using_go2rtc: bool = ensure_go2rtc_ready_for_stream(stream_name);
    if using_go2rtc {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using go2rtc native HLS for stream %s (no ffmpeg HLS thread needed)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        let mut existing: *mut go2rtc_stream_tracking_t = find_tracked_stream(stream_name);
        if existing.is_null() || !(*existing).using_go2rtc_for_hls {
            if !go2rtc_api_preload_stream(stream_name) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to preload stream %s in go2rtc - detection snapshots may be intermittent\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    stream_name,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Preloaded stream %s to keep go2rtc producer active for HLS/detection\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    stream_name,
                );
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s already preloaded for go2rtc HLS, skipping redundant preload\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        }
        let mut tracking: *mut go2rtc_stream_tracking_t = add_tracked_stream(stream_name);
        if !tracking.is_null() {
            (*tracking).using_go2rtc_for_hls = r#true != 0;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc native HLS ready for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return 0 as ::core::ffi::c_int;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc not available, using ffmpeg HLS fallback for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return start_hls_stream(stream_name);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_stop_hls(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc integration not initialized, using direct stop for HLS %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return stop_hls_stream(stream_name);
    }
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for go2rtc_integration_stop_hls\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut tracking: *mut go2rtc_stream_tracking_t = find_tracked_stream(stream_name);
    if !tracking.is_null() && (*tracking).using_go2rtc_for_hls as ::core::ffi::c_int != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopping go2rtc native HLS for stream %s (no ffmpeg thread to stop)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        (*tracking).using_go2rtc_for_hls = r#false != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopped go2rtc native HLS for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return 0 as ::core::ffi::c_int;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopping ffmpeg HLS fallback for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return stop_hls_stream(stream_name);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_is_using_go2rtc_for_recording(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if !g_initialized || stream_name.is_null() {
        return r#false != 0;
    }
    let mut tracking: *mut go2rtc_stream_tracking_t = find_tracked_stream(stream_name);
    return if !tracking.is_null() {
        (*tracking).using_go2rtc_for_recording as ::core::ffi::c_int
    } else {
        r#false
    } != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_is_using_go2rtc_for_hls(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if !g_initialized || stream_name.is_null() {
        return r#false != 0;
    }
    let mut tracking: *mut go2rtc_stream_tracking_t = find_tracked_stream(stream_name);
    return if !tracking.is_null() {
        (*tracking).using_go2rtc_for_hls as ::core::ffi::c_int
    } else {
        r#false
    } != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_register_all_streams() -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc integration module not initialized, cannot register streams\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service is not ready\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut ms: ::core::ffi::c_int = configured_stream_slots();
    let mut streams: *mut stream_config_t =
        calloc(ms as size_t, ::core::mem::size_of::<stream_config_t>()) as *mut stream_config_t;
    if streams.is_null() {
        return r#false != 0;
    }
    let mut count: ::core::ffi::c_int = get_all_stream_configs(streams, ms);
    if count <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"No streams found to register with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(streams as *mut ::core::ffi::c_void);
        return r#true != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Registering %d streams with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        count,
    );
    let mut all_success: bool = r#true != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if (*streams.offset(i as isize)).enabled {
            if !g_config.audio_disabled
                && (*streams.offset(i as isize)).go2rtc_source_override[0usize]
                    as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Skipping main stream API registration for %s (has go2rtc source override)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Registering stream %s with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                );
                if !go2rtc_stream_register(
                    &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                    &raw mut (*streams.offset(i as isize)).url as *mut ::core::ffi::c_char,
                    if (*streams.offset(i as isize)).onvif_username[0usize] as ::core::ffi::c_int
                        != '\0' as ::core::ffi::c_int
                    {
                        &raw mut (*streams.offset(i as isize)).onvif_username
                            as *mut ::core::ffi::c_char
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    },
                    if (*streams.offset(i as isize)).onvif_password[0usize] as ::core::ffi::c_int
                        != '\0' as ::core::ffi::c_int
                    {
                        &raw mut (*streams.offset(i as isize)).onvif_password
                            as *mut ::core::ffi::c_char
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    },
                    (*streams.offset(i as isize)).backchannel_enabled,
                    (*streams.offset(i as isize)).protocol,
                    (*streams.offset(i as isize)).record_audio,
                    &raw mut (*streams.offset(i as isize)).codec as *mut ::core::ffi::c_char,
                ) {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to register stream %s with go2rtc\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                    );
                    all_success = r#false != 0;
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Successfully registered stream %s with go2rtc\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                    );
                }
            }
            if (*streams.offset(i as isize)).sub_stream_url[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                let mut sub_name: [::core::ffi::c_char; 264] = [0; 264];
                snprintf(
                    &raw mut sub_name as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 264]>(),
                    b"%s_sub\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Registering sub-stream %s with go2rtc\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut sub_name as *mut ::core::ffi::c_char,
                );
                if !go2rtc_stream_register_substream(
                    &raw mut sub_name as *mut ::core::ffi::c_char,
                    &raw mut (*streams.offset(i as isize)).sub_stream_url
                        as *mut ::core::ffi::c_char,
                    if (*streams.offset(i as isize)).onvif_username[0usize] as ::core::ffi::c_int
                        != '\0' as ::core::ffi::c_int
                    {
                        &raw mut (*streams.offset(i as isize)).onvif_username
                            as *mut ::core::ffi::c_char
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    },
                    if (*streams.offset(i as isize)).onvif_password[0usize] as ::core::ffi::c_int
                        != '\0' as ::core::ffi::c_int
                    {
                        &raw mut (*streams.offset(i as isize)).onvif_password
                            as *mut ::core::ffi::c_char
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    },
                    (*streams.offset(i as isize)).protocol,
                ) {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to register sub-stream %s with go2rtc\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut sub_name as *mut ::core::ffi::c_char,
                    );
                }
            }
        }
        i += 1;
    }
    free(streams as *mut ::core::ffi::c_void);
    return all_success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_sync_streams_from_database() -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc integration module not initialized, cannot sync streams\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service is not ready, cannot sync streams\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut ms2: ::core::ffi::c_int = configured_stream_slots();
    let mut db_streams: *mut stream_config_t =
        calloc(ms2 as size_t, ::core::mem::size_of::<stream_config_t>()) as *mut stream_config_t;
    if db_streams.is_null() {
        return r#false != 0;
    }
    let mut count: ::core::ffi::c_int = get_all_stream_configs(db_streams, ms2);
    if count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get stream configurations from database\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(db_streams as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    if count == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"No streams found in database to sync with go2rtc\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(db_streams as *mut ::core::ffi::c_void);
        return r#true != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Syncing %d streams from database to go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        count,
    );
    let mut all_success: bool = r#true != 0;
    let mut synced: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut skipped: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut failed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if !(*db_streams.offset(i as isize)).enabled {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Skipping disabled stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
            );
            skipped += 1;
        } else if (*db_streams.offset(i as isize)).privacy_mode {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Skipping privacy-mode stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
            );
            skipped += 1;
        } else {
            let mut username: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            let mut password: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            if (*db_streams.offset(i as isize)).onvif_username[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                username = &raw mut (*db_streams.offset(i as isize)).onvif_username
                    as *mut ::core::ffi::c_char;
            }
            if (*db_streams.offset(i as isize)).onvif_password[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                password = &raw mut (*db_streams.offset(i as isize)).onvif_password
                    as *mut ::core::ffi::c_char;
            }
            if !g_config.audio_disabled
                && (*db_streams.offset(i as isize)).go2rtc_source_override[0usize]
                    as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Skipping main stream API sync for %s (has go2rtc source override)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                );
                skipped += 1;
            } else if go2rtc_api_stream_exists(
                &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
            ) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stream %s already exists in go2rtc, skipping\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                );
                skipped += 1;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Registering missing stream %s with go2rtc\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                );
                if !go2rtc_stream_register(
                    &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                    &raw mut (*db_streams.offset(i as isize)).url as *mut ::core::ffi::c_char,
                    username,
                    password,
                    (*db_streams.offset(i as isize)).backchannel_enabled,
                    (*db_streams.offset(i as isize)).protocol,
                    (*db_streams.offset(i as isize)).record_audio,
                    &raw mut (*db_streams.offset(i as isize)).codec as *mut ::core::ffi::c_char,
                ) {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to register stream %s with go2rtc\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                    );
                    all_success = r#false != 0;
                    failed += 1;
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Successfully synced stream %s to go2rtc\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                    );
                    synced += 1;
                }
            }
            if (*db_streams.offset(i as isize)).sub_stream_url[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                let mut sub_name: [::core::ffi::c_char; 264] = [0; 264];
                snprintf(
                    &raw mut sub_name as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 264]>(),
                    b"%s_sub\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                );
                if !go2rtc_api_stream_exists(&raw mut sub_name as *mut ::core::ffi::c_char) {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Registering missing sub-stream %s with go2rtc\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut sub_name as *mut ::core::ffi::c_char,
                    );
                    if !go2rtc_stream_register_substream(
                        &raw mut sub_name as *mut ::core::ffi::c_char,
                        &raw mut (*db_streams.offset(i as isize)).sub_stream_url
                            as *mut ::core::ffi::c_char,
                        username,
                        password,
                        (*db_streams.offset(i as isize)).protocol,
                    ) {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to register sub-stream %s with go2rtc\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut sub_name as *mut ::core::ffi::c_char,
                        );
                        all_success = r#false != 0;
                        failed += 1;
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Successfully synced sub-stream %s to go2rtc\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut sub_name as *mut ::core::ffi::c_char,
                        );
                        synced += 1;
                    }
                }
            }
        }
        i += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc sync complete: %d synced, %d skipped, %d failed\0".as_ptr()
            as *const ::core::ffi::c_char,
        synced,
        skipped,
        failed,
    );
    free(db_streams as *mut ::core::ffi::c_void);
    return all_success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_cleanup() {
    if !g_initialized {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleaning up go2rtc integration module\0".as_ptr() as *const ::core::ffi::c_char,
    );
    stop_unified_health_monitor();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_TRACKED_STREAMS {
        if g_tracked_streams[i as usize].stream_name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
        {
            if g_tracked_streams[i as usize].using_go2rtc_for_recording {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stopping recording for stream %s during cleanup\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*(&raw mut g_tracked_streams as *mut go2rtc_stream_tracking_t)
                        .offset(i as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                );
                go2rtc_consumer_stop_recording(
                    &raw mut (*(&raw mut g_tracked_streams as *mut go2rtc_stream_tracking_t)
                        .offset(i as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                );
            }
            if g_tracked_streams[i as usize].using_go2rtc_for_hls {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stopping HLS streaming for stream %s during cleanup\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*(&raw mut g_tracked_streams as *mut go2rtc_stream_tracking_t)
                        .offset(i as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                );
                go2rtc_integration_stop_hls(
                    &raw mut (*(&raw mut g_tracked_streams as *mut go2rtc_stream_tracking_t)
                        .offset(i as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                );
            }
            g_tracked_streams[i as usize].stream_name[0usize] = '\0' as ::core::ffi::c_char;
            g_tracked_streams[i as usize].using_go2rtc_for_recording = r#false != 0;
            g_tracked_streams[i as usize].using_go2rtc_for_hls = r#false != 0;
        }
        i += 1;
    }
    go2rtc_consumer_cleanup();
    g_initialized = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc integration module cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_is_initialized() -> bool {
    return g_initialized;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_get_rtsp_url(
    mut stream_name: *const ::core::ffi::c_char,
    mut url: *mut ::core::ffi::c_char,
    mut url_size: size_t,
) -> bool {
    if stream_name.is_null() || url.is_null() || url_size == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for go2rtc_get_rtsp_url\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut ready_retries: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    while !go2rtc_stream_is_ready() && ready_retries > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service is not ready, retrying... (%d attempts left)\0".as_ptr()
                as *const ::core::ffi::c_char,
            ready_retries,
        );
        if !go2rtc_stream_start_service() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to start go2rtc service\0".as_ptr() as *const ::core::ffi::c_char,
            );
            ready_retries -= 1;
            sleep(2 as ::core::ffi::c_uint);
        } else {
            let mut wait_retries: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
            while wait_retries > 0 as ::core::ffi::c_int && !go2rtc_stream_is_ready() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Waiting for go2rtc service to start... (%d retries left)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    wait_retries,
                );
                sleep(2 as ::core::ffi::c_uint);
                wait_retries -= 1;
            }
            if go2rtc_stream_is_ready() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"go2rtc service is now ready\0".as_ptr() as *const ::core::ffi::c_char,
                );
                break;
            } else {
                ready_retries -= 1;
            }
        }
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service is not ready after multiple attempts, cannot get RTSP URL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !is_stream_registered_with_go2rtc(stream_name) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s is not registered with go2rtc, attempting to register...\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        if !ensure_stream_registered_with_go2rtc(stream_name) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to register stream %s with go2rtc\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
            return r#false != 0;
        }
        usleep(500000 as ::core::ffi::c_int as __useconds_t);
        if !is_stream_registered_with_go2rtc(stream_name) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s not yet visible in go2rtc /api/streams after registration (may be transient), proceeding to get RTSP URL anyway\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s confirmed registered with go2rtc\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        }
    }
    if !go2rtc_stream_get_rtsp_url(stream_name, url, url_size) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get RTSP URL for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Got RTSP URL for stream %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        url,
    );
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_get_hls_url(
    mut stream_name: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) -> bool {
    if !g_initialized || stream_name.is_null() || buffer.is_null() || buffer_size == 0 as size_t {
        return r#false != 0;
    }
    if !go2rtc_integration_is_using_go2rtc_for_hls(stream_name) {
        return r#false != 0;
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service is not ready, cannot get HLS URL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut encoded_name: [::core::ffi::c_char; 768] = [0; 768];
    simple_url_escape(
        stream_name,
        &raw mut encoded_name as *mut ::core::ffi::c_char,
        (MAX_STREAM_NAME * 3 as ::core::ffi::c_int) as size_t,
    );
    let mut api_port: ::core::ffi::c_int = go2rtc_stream_get_api_port();
    if api_port == 0 as ::core::ffi::c_int {
        api_port = 1984 as ::core::ffi::c_int;
    }
    snprintf(
        buffer,
        buffer_size,
        b"http://localhost:%d%s/api/stream.m3u8?src=%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        api_port,
        go2rtc_api_get_base_path(),
        &raw mut encoded_name as *mut ::core::ffi::c_char,
        if g_config.audio_disabled as ::core::ffi::c_int != 0 {
            b"&video\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Generated go2rtc HLS URL for stream %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        buffer,
    );
    return r#true != 0;
}
unsafe extern "C" fn go2rtc_integration_reload_stream_config_locked(
    mut stream_name: *const ::core::ffi::c_char,
    mut new_url: *const ::core::ffi::c_char,
    mut new_username: *const ::core::ffi::c_char,
    mut new_password: *const ::core::ffi::c_char,
    mut new_backchannel_enabled: ::core::ffi::c_int,
    mut new_protocol: ::core::ffi::c_int,
    mut new_record_audio: ::core::ffi::c_int,
) -> bool {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_integration_reload_stream_config: stream_name is NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Reloading stream configuration for %s in go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    let mut stream: stream_handle_t = get_stream_by_name(stream_name);
    let mut config: stream_config_t = stream_config_t {
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
    let mut have_config: bool = r#false != 0;
    if !stream.is_null() && get_stream_config(stream, &raw mut config) == 0 as ::core::ffi::c_int {
        have_config = r#true != 0;
    }
    if have_config as ::core::ffi::c_int != 0
        && !g_config.audio_disabled
        && config.go2rtc_source_override[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s uses a go2rtc source override; restarting go2rtc to reload YAML config\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return go2rtc_integration_restart_process();
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service is not ready, cannot reload stream config\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut url: *const ::core::ffi::c_char = new_url;
    let mut username: *const ::core::ffi::c_char = new_username;
    let mut password: *const ::core::ffi::c_char = new_password;
    let mut backchannel: bool = if new_backchannel_enabled >= 0 as ::core::ffi::c_int {
        (new_backchannel_enabled != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
    } else {
        r#false
    } != 0;
    let mut protocol: stream_protocol_t = stream_protocol_t::STREAM_PROTOCOL_TCP;
    let mut record_audio: bool = r#false != 0;
    if have_config {
        if url.is_null() {
            url = &raw mut config.url as *mut ::core::ffi::c_char;
        }
        if username.is_null() {
            username = if config.onvif_username[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                &raw mut config.onvif_username as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
        }
        if password.is_null() {
            password = if config.onvif_password[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                &raw mut config.onvif_password as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
        }
        if new_backchannel_enabled < 0 as ::core::ffi::c_int {
            backchannel = config.backchannel_enabled;
        }
        if new_protocol < 0 as ::core::ffi::c_int {
            protocol = config.protocol;
        }
        if new_record_audio < 0 as ::core::ffi::c_int {
            record_audio = config.record_audio;
        }
    }
    if new_record_audio >= 0 as ::core::ffi::c_int {
        record_audio = new_record_audio != 0 as ::core::ffi::c_int;
    }
    if new_protocol >= 0 as ::core::ffi::c_int {
        protocol = stream_protocol_t(new_protocol as ::core::ffi::c_uint);
    }
    if url.is_null() || *url.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_integration_reload_stream_config: No URL available for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return r#false != 0;
    }
    let mut unregistered: bool = go2rtc_stream_unregister(stream_name);
    if unregistered {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unregistered old stream %s from go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s was not registered with go2rtc (or unregister failed)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
    }
    usleep(500000 as ::core::ffi::c_int as __useconds_t);
    let mut codec: *const ::core::ffi::c_char = if have_config as ::core::ffi::c_int != 0 {
        &raw mut config.codec as *mut ::core::ffi::c_char
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    if !go2rtc_stream_register(
        stream_name,
        url,
        username,
        password,
        backchannel,
        protocol,
        record_audio,
        codec,
    ) {
        if unregistered as ::core::ffi::c_int != 0
            && go2rtc_api_stream_exists(stream_name) as ::core::ffi::c_int != 0
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Re-registration of %s reported failure but go2rtc lists the stream; continuing\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to re-register stream %s with go2rtc\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
            return r#false != 0;
        }
    }
    if !go2rtc_api_stream_exists(stream_name) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not visible in go2rtc after re-registration; retrying once\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        usleep(500000 as ::core::ffi::c_int as __useconds_t);
        if !go2rtc_api_stream_exists(stream_name)
            && !go2rtc_stream_register(
                stream_name,
                url,
                username,
                password,
                backchannel,
                protocol,
                record_audio,
                codec,
            )
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to re-register stream %s with go2rtc on retry\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
            return r#false != 0;
        }
    }
    let mut tracking: *mut go2rtc_stream_tracking_t = find_tracked_stream(stream_name);
    if !tracking.is_null() && (*tracking).using_go2rtc_for_hls as ::core::ffi::c_int != 0 {
        if go2rtc_api_preload_stream(stream_name) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Re-attached go2rtc preload for stream %s after reload\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to re-preload stream %s after reload; HLS/detection snapshots may be intermittent until go2rtc can reach the camera\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
        }
    }
    let mut safe_url: [::core::ffi::c_char; 512] = [0; 512];
    if url_redact_for_logging(
        url,
        &raw mut safe_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut safe_url as *mut ::core::ffi::c_char,
            b"[invalid-url]\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Successfully reloaded stream %s in go2rtc with URL: %s (protocol=%s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
        &raw mut safe_url as *mut ::core::ffi::c_char,
        if protocol.0 == stream_protocol_t::STREAM_PROTOCOL_UDP.0 {
            b"UDP\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"TCP\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_reload_stream_config(
    mut stream_name: *const ::core::ffi::c_char,
    mut new_url: *const ::core::ffi::c_char,
    mut new_username: *const ::core::ffi::c_char,
    mut new_password: *const ::core::ffi::c_char,
    mut new_backchannel_enabled: ::core::ffi::c_int,
    mut new_protocol: ::core::ffi::c_int,
    mut new_record_audio: ::core::ffi::c_int,
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
            b"Failed to enter go2rtc lifecycle to reload %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            if !stream_name.is_null() {
                stream_name
            } else {
                b"<null>\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return r#false != 0;
    }
    let mut result: bool = go2rtc_integration_reload_stream_config_locked(
        stream_name,
        new_url,
        new_username,
        new_password,
        new_backchannel_enabled,
        new_protocol,
        new_record_audio,
    );
    go2rtc_lifecycle_end(&raw mut guard, result);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_reload_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_integration_reload_stream: stream_name is NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return go2rtc_integration_reload_stream_config(
        stream_name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_unregister_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_integration_unregister_stream: stream_name is NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service is not ready, cannot unregister stream\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if go2rtc_stream_unregister(stream_name) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unregistered stream %s from go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < MAX_TRACKED_STREAMS {
            if strcmp(
                &raw mut (*(&raw mut g_tracked_streams as *mut go2rtc_stream_tracking_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
            {
                memset(
                    (&raw mut g_tracked_streams as *mut go2rtc_stream_tracking_t).offset(i as isize)
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<go2rtc_stream_tracking_t>(),
                );
                break;
            } else {
                i += 1;
            }
        }
        return r#true != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Failed to unregister stream %s from go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_register_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_integration_register_stream: stream_name is NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !go2rtc_stream_is_ready() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc service is not ready, cannot register stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return r#false != 0;
    }
    let mut stream: stream_handle_t = get_stream_by_name(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found in stream manager\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return r#false != 0;
    }
    let mut config: stream_config_t = stream_config_t {
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
    if get_stream_config(stream, &raw mut config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get config for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return r#false != 0;
    }
    if !g_config.audio_disabled
        && config.go2rtc_source_override[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s has go2rtc source override; restarting go2rtc so YAML source is loaded\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return go2rtc_integration_restart_process();
    }
    if is_stream_registered_with_go2rtc(stream_name) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s is already registered with go2rtc, skipping re-registration\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return r#true != 0;
    }
    let mut skip_main: bool = !g_config.audio_disabled
        && config.go2rtc_source_override[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int;
    let mut username: [::core::ffi::c_char; 64] = [
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
    ];
    let mut password: [::core::ffi::c_char; 64] = [
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
    ];
    if config.onvif_username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut username as *mut ::core::ffi::c_char,
            &raw mut config.onvif_username as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    if config.onvif_password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut password as *mut ::core::ffi::c_char,
            &raw mut config.onvif_password as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    if username[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        let mut url: *const ::core::ffi::c_char = &raw mut config.url as *mut ::core::ffi::c_char;
        if strncmp(
            url,
            b"rtsp://\0".as_ptr() as *const ::core::ffi::c_char,
            7 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            let mut at_sign: *const ::core::ffi::c_char = strchr(
                url.offset(7 as ::core::ffi::c_int as isize),
                '@' as ::core::ffi::c_int,
            );
            if !at_sign.is_null() {
                let mut colon: *const ::core::ffi::c_char = strchr(
                    url.offset(7 as ::core::ffi::c_int as isize),
                    ':' as ::core::ffi::c_int,
                );
                if !colon.is_null() && colon < at_sign {
                    let mut username_len: size_t =
                        colon.offset_from(url.offset(7 as ::core::ffi::c_int as isize)) as size_t;
                    if username_len < ::core::mem::size_of::<[::core::ffi::c_char; 64]>() {
                        safe_strcpy(
                            &raw mut username as *mut ::core::ffi::c_char,
                            url.offset(7 as ::core::ffi::c_int as isize),
                            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                            username_len,
                        );
                        if password[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                            let mut password_len: size_t = at_sign
                                .offset_from(colon.offset(1 as ::core::ffi::c_int as isize))
                                as size_t;
                            if password_len < ::core::mem::size_of::<[::core::ffi::c_char; 64]>() {
                                safe_strcpy(
                                    &raw mut password as *mut ::core::ffi::c_char,
                                    colon.offset(1 as ::core::ffi::c_int as isize),
                                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                                    password_len,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    let mut main_ok: bool = r#true != 0;
    if skip_main {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s has go2rtc source override, skipping main API registration\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
    } else if go2rtc_stream_register(
        stream_name,
        &raw mut config.url as *mut ::core::ffi::c_char,
        if username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            &raw mut username as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        if password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            &raw mut password as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        config.backchannel_enabled,
        config.protocol,
        config.record_audio,
        &raw mut config.codec as *mut ::core::ffi::c_char,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Successfully registered stream %s with go2rtc\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to register stream %s with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        main_ok = r#false != 0;
    }
    if config.sub_stream_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut sub_name: [::core::ffi::c_char; 264] = [0; 264];
        snprintf(
            &raw mut sub_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 264]>(),
            b"%s_sub\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Registering sub-stream %s with go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut sub_name as *mut ::core::ffi::c_char,
        );
        go2rtc_stream_register_substream(
            &raw mut sub_name as *mut ::core::ffi::c_char,
            &raw mut config.sub_stream_url as *mut ::core::ffi::c_char,
            if username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                &raw mut username as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            if password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                &raw mut password as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            config.protocol,
        );
    }
    return main_ok as ::core::ffi::c_int != 0 || skip_main as ::core::ffi::c_int != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_report_proxy_open_failure(
    mut stream_name: *const ::core::ffi::c_char,
) {
    if stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return;
    }
    pthread_mutex_lock(&raw mut g_recovery_mutex);
    let mut st: *mut stream_recovery_state_t = recovery_state_locked(stream_name, r#true != 0);
    if !st.is_null() && (*st).proxy_open_failures < INT_MAX {
        (*st).proxy_open_failures += 1;
    }
    pthread_mutex_unlock(&raw mut g_recovery_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_monitor_is_running() -> bool {
    return g_monitor_initialized as ::core::ffi::c_int != 0
        && g_monitor_running as ::core::ffi::c_int != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_get_restart_count() -> ::core::ffi::c_int {
    return g_restart_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_get_last_restart_time() -> time_t {
    return g_last_restart_time;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_integration_check_health() -> bool {
    return go2rtc_stream_is_ready();
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
