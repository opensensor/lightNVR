unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn difftime(__time1: time_t, __time0: time_t) -> ::core::ffi::c_double;
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
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
    unsafe fn curl_easy_strerror(error: CURLcode) -> *const ::core::ffi::c_char;
    unsafe fn curl_easy_init() -> *mut ::core::ffi::c_void;
    unsafe fn curl_easy_setopt(curl: *mut ::core::ffi::c_void, option: CURLoption, ...)
    -> CURLcode;
    unsafe fn curl_easy_perform(curl: *mut ::core::ffi::c_void) -> CURLcode;
    unsafe fn curl_easy_cleanup(curl: *mut ::core::ffi::c_void);
    unsafe fn curl_easy_getinfo(curl: *mut ::core::ffi::c_void, info: CURLINFO, ...) -> CURLcode;
    unsafe fn is_shutdown_initiated() -> bool;
    static mut g_config: config_t;
    unsafe fn http_request_get_query_param(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
        value: *mut ::core::ffi::c_char,
        value_len: size_t,
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
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateBool(boolean: cJSON_bool) -> *mut cJSON;
    unsafe fn cJSON_CreateNumber(num: ::core::ffi::c_double) -> *mut cJSON;
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
    unsafe fn cJSON_ReplaceItemInObjectCaseSensitive(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        newitem: *mut cJSON,
    ) -> cJSON_bool;
    unsafe fn cJSON_AddBoolToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        boolean: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddNumberToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        number: ::core::ffi::c_double,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn http_server_start(server: http_server_handle_t) -> ::core::ffi::c_int;
    unsafe fn http_server_stop(server: http_server_handle_t);
    unsafe fn authorization_filter_visible_cameras(
        user: *const user_t,
        cameras: *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_secure_cookies() -> bool;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn metrics_snapshot_all(
        out_array: *mut stream_metrics_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn metrics_get_ring_data(
        stream_name: *const ::core::ffi::c_char,
        out: *mut metrics_ring_sample_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn metrics_get_max_streams() -> ::core::ffi::c_int;
    unsafe fn get_storage_health(health: *mut storage_health_t) -> ::core::ffi::c_int;
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
    unsafe fn interruptible_sleep_init(s: *mut interruptible_sleep_t) -> ::core::ffi::c_int;
    unsafe fn interruptible_sleep_reset(s: *mut interruptible_sleep_t);
    unsafe fn interruptible_sleep_wait(s: *mut interruptible_sleep_t, seconds: ::core::ffi::c_int);
    unsafe fn interruptible_sleep_wake(s: *mut interruptible_sleep_t);
    unsafe fn get_unified_detection_state(
        stream_name: *const ::core::ffi::c_char,
    ) -> unified_detection_state_t;
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
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type int64_t = i64;
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
pub type uint32_t = u32;
pub type uint8_t = u8;
pub type uint64_t = u64;
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
pub type int_fast64_t = ::core::ffi::c_long;
pub type uint_fast64_t = ::core::ffi::c_ulong;
pub type atomic_bool = bool;
pub type atomic_int = ::core::ffi::c_int;
pub type atomic_int_fast64_t = int_fast64_t;
pub type atomic_uint_fast64_t = uint_fast64_t;
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub type http_server_handle_t = *mut http_server_t;
pub type http_server_t = http_server;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_server {
    pub config: http_server_config_t,
    pub running: bool,
    pub handlers: *mut C2Rust_Unnamed_1,
    pub handler_count: ::core::ffi::c_int,
    pub handler_capacity: ::core::ffi::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryStruct {
    pub memory: *mut ::core::ffi::c_char,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct interruptible_sleep_t {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub woken: bool,
    pub clock_id: clockid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_metrics_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub active: bool,
    pub health_status: atomic_int,
    pub stream_up: atomic_int,
    pub health_observed: bool,
    pub health_changed_at: time_t,
    pub last_packet_ts: atomic_int_fast64_t,
    pub last_frame_ts: atomic_int_fast64_t,
    pub current_fps: ::core::ffi::c_double,
    pub current_bitrate_bps: ::core::ffi::c_double,
    pub connection_latency_ms: ::core::ffi::c_double,
    pub configured_fps: ::core::ffi::c_double,
    pub frames_total: atomic_uint_fast64_t,
    pub frames_dropped: atomic_uint_fast64_t,
    pub bytes_received: atomic_uint_fast64_t,
    pub reconnects_total: atomic_uint_fast64_t,
    pub uptime_seconds: atomic_uint_fast64_t,
    pub error_decode: atomic_uint_fast64_t,
    pub error_timeout: atomic_uint_fast64_t,
    pub error_protocol: atomic_uint_fast64_t,
    pub error_io: atomic_uint_fast64_t,
    pub recording_active: atomic_int,
    pub recording_bytes_written: atomic_uint_fast64_t,
    pub recording_segments_total: atomic_uint_fast64_t,
    pub recording_gaps_total: atomic_uint_fast64_t,
    pub last_completed_segment_ts: atomic_int_fast64_t,
    pub ring: [metrics_ring_sample_t; 720],
    pub ring_head: ::core::ffi::c_int,
    pub ring_count: ::core::ffi::c_int,
    pub last_segment_end_time: time_t,
    pub expected_segment_duration: ::core::ffi::c_int,
    pub observation_persisted_at: time_t,
    pub stream_start_time: time_t,
    pub source_frames_total: [atomic_uint_fast64_t; 4],
    pub source_bytes_received: [atomic_uint_fast64_t; 4],
    pub prev_source_frames: [uint64_t; 4],
    pub prev_source_bytes: [uint64_t; 4],
    pub prev_sample_time: time_t,
    pub lock: pthread_rwlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct metrics_ring_sample_t {
    pub fps: ::core::ffi::c_float,
    pub bitrate_kbps: ::core::ffi::c_float,
    pub timestamp: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_phase_timer_t {
    pub started: timespec,
    pub request: *const http_request_t,
    pub phase: *const ::core::ffi::c_char,
    pub valid: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct unified_detection_state_t(pub ::core::ffi::c_uint);
impl unified_detection_state_t {
    pub const UDT_STATE_INITIALIZING: Self = Self(0);
    pub const UDT_STATE_CONNECTING: Self = Self(1);
    pub const UDT_STATE_BUFFERING: Self = Self(2);
    pub const UDT_STATE_RECORDING: Self = Self(3);
    pub const UDT_STATE_POST_BUFFER: Self = Self(4);
    pub const UDT_STATE_RECONNECTING: Self = Self(5);
    pub const UDT_STATE_STOPPING: Self = Self(6);
    pub const UDT_STATE_STOPPED: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_health_status_t(pub ::core::ffi::c_uint);
impl stream_health_status_t {
    pub const STREAM_HEALTH_UP: Self = Self(0);
    pub const STREAM_HEALTH_DEGRADED: Self = Self(1);
    pub const STREAM_HEALTH_DOWN: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_health_t {
    pub pressure_level: disk_pressure_level_t,
    pub free_space_pct: ::core::ffi::c_double,
    pub free_space_bytes: uint64_t,
    pub total_space_bytes: uint64_t,
    pub used_space_bytes: uint64_t,
    pub last_check_time: time_t,
    pub last_cleanup_time: time_t,
    pub last_deep_time: time_t,
    pub last_cleanup_deleted: ::core::ffi::c_int,
    pub last_cleanup_freed: uint64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct disk_pressure_level_t(pub ::core::ffi::c_uint);
impl disk_pressure_level_t {
    pub const DISK_PRESSURE_NORMAL: Self = Self(0);
    pub const DISK_PRESSURE_WARNING: Self = Self(1);
    pub const DISK_PRESSURE_CRITICAL: Self = Self(2);
    pub const DISK_PRESSURE_EMERGENCY: Self = Self(3);
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
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STREAM_HEALTH_FRAME_TIMEOUT_UP: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const STREAM_HEALTH_FRAME_TIMEOUT_DOWN: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"HealthCheck\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[inline]
unsafe extern "C" fn api_phase_begin(
    mut request: *const http_request_t,
    mut phase: *const ::core::ffi::c_char,
) -> api_phase_timer_t {
    let mut timer: api_phase_timer_t = api_phase_timer_t {
        started: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        request: request,
        phase: phase,
        valid: false,
    };
    timer.valid = clock_gettime(CLOCK_MONOTONIC, &raw mut timer.started) == 0 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"API phase starting: path=%s phase=%s request_id=%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*request).path as *const ::core::ffi::c_char,
        phase,
        if (*request).request_id[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*request).request_id as *const ::core::ffi::c_char
        } else {
            b"-\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return timer;
}
#[inline]
unsafe extern "C" fn api_phase_end(mut timer: *const api_phase_timer_t) {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if !(*timer).valid || clock_gettime(CLOCK_MONOTONIC, &raw mut now) != 0 as ::core::ffi::c_int {
        return;
    }
    let mut elapsed_ms: ::core::ffi::c_double =
        (now.tv_sec - (*timer).started.tv_sec) as ::core::ffi::c_double * 1000.0f64
            + (now.tv_nsec - (*timer).started.tv_nsec) as ::core::ffi::c_double / 1000000.0f64;
    if elapsed_ms >= 250.0f64 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Slow API phase: path=%s phase=%s duration_ms=%.0f request_id=%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*(*timer).request).path as *const ::core::ffi::c_char,
            (*timer).phase,
            elapsed_ms,
            if (*(*timer).request).request_id[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*(*timer).request).request_id as *const ::core::ffi::c_char
            } else {
                b"-\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_last_health_check: time_t = 0 as time_t;
static mut g_is_healthy: atomic_bool = r#true != 0;
static mut g_failed_health_checks: atomic_int = 0 as ::core::ffi::c_int;
static mut g_total_requests: atomic_int = 0 as ::core::ffi::c_int;
static mut g_failed_requests: atomic_int = 0 as ::core::ffi::c_int;
static mut g_start_time: time_t = 0 as time_t;
static mut g_server_needs_restart: atomic_bool = r#false != 0;
static mut g_last_restart_attempt: time_t = 0 as time_t;
static mut g_restart_attempts: atomic_int = 0 as ::core::ffi::c_int;
static mut MAX_RESTART_ATTEMPTS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
static mut RESTART_COOLDOWN_SECONDS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
static mut g_health_check_thread: pthread_t = 0;
static mut g_health_thread_running: atomic_bool = r#false != 0;
static mut g_health_thread_exited: atomic_bool = r#true != 0;
static mut g_health_thread_stopping: atomic_bool = r#false != 0;
static mut g_health_lifecycle_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut g_health_thread_created: bool = r#false != 0;
static mut g_health_check_interval: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut g_health_wake: interruptible_sleep_t = interruptible_sleep_t {
    mutex: pthread_mutex_t {
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
    cond: pthread_cond_t {
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
    woken: false,
    clock_id: 0,
};
static mut g_health_wake_initialized: bool = r#false != 0;
static mut g_health_check_url: [::core::ffi::c_char; 256] = unsafe {
    ::core::mem::transmute::<
        [u8; 256],
        [::core::ffi::c_char; 256],
    >(
        *b"http://127.0.0.1:8080/api/health?liveness=true\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut g_health_check_tls: bool = r#false != 0;
static mut g_web_server_thread_id: pthread_t = 0 as pthread_t;
static mut g_web_server_thread_id_set: bool = r#false != 0;
static mut g_web_server_thread_running: bool = r#false != 0;
static mut g_web_server_thread_mutex: pthread_mutex_t = pthread_mutex_t {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_process_uptime_seconds() -> ::core::ffi::c_double {
    if g_start_time <= 0 as time_t {
        return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    let mut uptime: ::core::ffi::c_double =
        difftime(time(::core::ptr::null_mut::<time_t>()), g_start_time);
    return if uptime > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        uptime
    } else {
        0 as ::core::ffi::c_int as ::core::ffi::c_double
    };
}
unsafe extern "C" fn write_memory_callback(
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
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to allocate memory for curl response\0".as_ptr() as *const ::core::ffi::c_char,
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
unsafe extern "C" fn perform_health_check() -> bool {
    let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut res: CURLcode = CURLcode::CURLE_OK;
    let mut success: bool = r#false != 0;
    let mut chunk: MemoryStruct = MemoryStruct {
        memory: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
    };
    chunk.memory = malloc(1 as size_t) as *mut ::core::ffi::c_char;
    if chunk.memory.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to allocate initial buffer for health check response\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *chunk.memory.offset(0isize) = '\0' as ::core::ffi::c_char;
    chunk.size = 0 as size_t;
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to initialize curl for health check\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(chunk.memory as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut g_health_check_url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(
        curl,
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
        curl,
        CURLoption::CURLOPT_WRITEDATA,
        &raw mut chunk as *mut ::core::ffi::c_void,
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_TIMEOUT, 5 as ::core::ffi::c_long);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CONNECTTIMEOUT,
        3 as ::core::ffi::c_long,
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_NOSIGNAL, 1 as ::core::ffi::c_long);
    if g_health_check_tls {
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_SSL_VERIFYPEER,
            0 as ::core::ffi::c_long,
        );
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_SSL_VERIFYHOST,
            0 as ::core::ffi::c_long,
        );
    }
    res = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Health check curl failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            curl_easy_strerror(res),
        );
        g_failed_health_checks += 1;
        g_is_healthy = r#false != 0;
    } else {
        let mut response_code: ::core::ffi::c_long = 0;
        curl_easy_getinfo(
            curl,
            CURLINFO::CURLINFO_RESPONSE_CODE,
            &raw mut response_code,
        );
        if response_code == 200 as ::core::ffi::c_long {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Health check succeeded with response: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                chunk.memory,
            );
            success = r#true != 0;
            g_last_health_check = time(::core::ptr::null_mut::<time_t>());
            g_is_healthy = r#true != 0;
            g_failed_health_checks = 0 as ::core::ffi::c_int;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Health check failed with response code: %ld\0".as_ptr()
                    as *const ::core::ffi::c_char,
                response_code,
            );
            g_failed_health_checks += 1;
            g_is_healthy = r#false != 0;
        }
    }
    curl_easy_cleanup(curl);
    free(chunk.memory as *mut ::core::ffi::c_void);
    return success;
}
unsafe extern "C" fn filter_authorized_stream_metrics(
    mut req: *const http_request_t,
    mut snapshots: *mut stream_metrics_t,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if req.is_null()
        || count.is_null()
        || *count <= 0 as ::core::ffi::c_int
        || !g_config.web_auth_enabled
    {
        return 0 as ::core::ffi::c_int;
    }
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
    memset(
        &raw mut user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    if httpd_check_action_access(req, &raw mut user) == 0 {
        *count = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db_fleet_camera_load(&raw mut cameras, &raw mut camera_count) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if authorization_filter_visible_cameras(&raw mut user, cameras, &raw mut camera_count)
        != 0 as ::core::ffi::c_int
    {
        free(cameras as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut visible_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut snapshot_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while snapshot_index < *count {
        let mut visible: bool = r#false != 0;
        let mut camera_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while camera_index < camera_count {
            if strcmp(
                &raw mut (*snapshots.offset(snapshot_index as isize)).stream_name
                    as *mut ::core::ffi::c_char,
                &raw mut (*cameras.offset(camera_index as isize)).name as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                visible = r#true != 0;
                break;
            } else {
                camera_index += 1;
            }
        }
        if visible {
            if visible_count != snapshot_index {
                *snapshots.offset(visible_count as isize) =
                    *snapshots.offset(snapshot_index as isize);
            }
            visible_count += 1;
        }
        snapshot_index += 1;
    }
    free(cameras as *mut ::core::ffi::c_void);
    *count = visible_count;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_health(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/health request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    g_last_health_check = time(::core::ptr::null_mut::<time_t>());
    let mut sparklines_param: [::core::ffi::c_char; 8] =
        [0 as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0];
    let mut include_sparklines: bool = r#false != 0;
    if http_request_get_query_param(
        req,
        b"sparklines\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sparklines_param as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
    ) > 0 as ::core::ffi::c_int
    {
        include_sparklines = strcmp(
            &raw mut sparklines_param as *mut ::core::ffi::c_char,
            b"true\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut sparklines_param as *mut ::core::ffi::c_char,
                b"1\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
    }
    let mut history_param: [::core::ffi::c_char; 8] =
        [0 as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0];
    let mut history_only: bool = r#false != 0;
    if http_request_get_query_param(
        req,
        b"history_only\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut history_param as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
    ) > 0 as ::core::ffi::c_int
    {
        history_only = strcmp(
            &raw mut history_param as *mut ::core::ffi::c_char,
            b"true\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut history_param as *mut ::core::ffi::c_char,
                b"1\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
    }
    let mut health: *mut cJSON = cJSON_CreateObject();
    if health.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create health JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create health JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        health,
        b"healthy\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddNumberToObject(
        health,
        b"uptime\0".as_ptr() as *const ::core::ffi::c_char,
        get_process_uptime_seconds(),
    );
    cJSON_AddNumberToObject(
        health,
        b"totalRequests\0".as_ptr() as *const ::core::ffi::c_char,
        g_total_requests as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        health,
        b"failedRequests\0".as_ptr() as *const ::core::ffi::c_char,
        g_failed_requests as ::core::ffi::c_double,
    );
    let mut timestamp: [::core::ffi::c_char; 32] = [0; 32];
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut tm_buf: tm = tm {
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
    let mut tm_info: *const tm = localtime_r(&raw mut now, &raw mut tm_buf);
    strftime(
        &raw mut timestamp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%Y-%m-%d %H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
        tm_info,
    );
    cJSON_AddStringToObject(
        health,
        b"timestamp\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut timestamp as *mut ::core::ffi::c_char,
    );
    let mut liveness_param: [::core::ffi::c_char; 8] =
        [0 as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0];
    let mut liveness_only: bool = http_request_get_query_param(
        req,
        b"liveness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut liveness_param as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
    ) > 0 as ::core::ffi::c_int
        && (strcmp(
            &raw mut liveness_param as *mut ::core::ffi::c_char,
            b"true\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut liveness_param as *mut ::core::ffi::c_char,
                b"1\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int);
    if liveness_only {
        cJSON_AddBoolToObject(
            health,
            b"liveness\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        let mut body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(health);
        cJSON_Delete(health);
        if body.is_null() {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to encode liveness JSON\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        http_response_set_json(res, 200 as ::core::ffi::c_int, body);
        free(body as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Successfully handled process liveness request\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut phase: api_phase_timer_t = api_phase_begin(
        req,
        b"runtime metrics snapshot\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut max_streams: ::core::ffi::c_int = metrics_get_max_streams();
    let mut snaps: *mut stream_metrics_t = ::core::ptr::null_mut::<stream_metrics_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if max_streams > 0 as ::core::ffi::c_int {
        snaps = calloc(
            max_streams as size_t,
            ::core::mem::size_of::<stream_metrics_t>(),
        ) as *mut stream_metrics_t;
        if !snaps.is_null() {
            count = metrics_snapshot_all(snaps, max_streams);
        }
    }
    api_phase_end(&raw mut phase);
    phase = api_phase_begin(
        req,
        b"camera inventory authorization\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut authorization_result: ::core::ffi::c_int =
        filter_authorized_stream_metrics(req, snaps, &raw mut count);
    api_phase_end(&raw mut phase);
    if authorization_result != 0 as ::core::ffi::c_int {
        free(snaps as *mut ::core::ffi::c_void);
        cJSON_Delete(health);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if history_only {
        let mut detail_arr: *mut cJSON = cJSON_CreateArray();
        if detail_arr.is_null() {
            free(snaps as *mut ::core::ffi::c_void);
            cJSON_Delete(health);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create health history\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < count {
            let mut sd: *mut cJSON = cJSON_CreateObject();
            let mut fps_arr: *mut cJSON = cJSON_CreateArray();
            let mut bitrate_arr: *mut cJSON = cJSON_CreateArray();
            if sd.is_null() || fps_arr.is_null() || bitrate_arr.is_null() {
                cJSON_Delete(sd);
                cJSON_Delete(fps_arr);
                cJSON_Delete(bitrate_arr);
            } else {
                cJSON_AddStringToObject(
                    sd,
                    b"name\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*snaps.offset(i as isize)).stream_name as *mut ::core::ffi::c_char,
                );
                let mut ring_data: [metrics_ring_sample_t; 60] = [metrics_ring_sample_t {
                    fps: 0.,
                    bitrate_kbps: 0.,
                    timestamp: 0,
                }; 60];
                let mut ring_count: ::core::ffi::c_int = metrics_get_ring_data(
                    &raw mut (*snaps.offset(i as isize)).stream_name as *mut ::core::ffi::c_char,
                    &raw mut ring_data as *mut metrics_ring_sample_t,
                    60 as ::core::ffi::c_int,
                );
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < ring_count {
                    cJSON_AddItemToArray(
                        fps_arr,
                        cJSON_CreateNumber(ring_data[j as usize].fps as ::core::ffi::c_double),
                    );
                    cJSON_AddItemToArray(
                        bitrate_arr,
                        cJSON_CreateNumber(
                            ring_data[j as usize].bitrate_kbps as ::core::ffi::c_double,
                        ),
                    );
                    j += 1;
                }
                cJSON_AddItemToObject(
                    sd,
                    b"sparkline_fps\0".as_ptr() as *const ::core::ffi::c_char,
                    fps_arr,
                );
                cJSON_AddItemToObject(
                    sd,
                    b"sparkline_bitrate\0".as_ptr() as *const ::core::ffi::c_char,
                    bitrate_arr,
                );
                cJSON_AddItemToArray(detail_arr, sd);
            }
            i += 1;
        }
        cJSON_AddItemToObject(
            health,
            b"streams_detail\0".as_ptr() as *const ::core::ffi::c_char,
            detail_arr,
        );
        free(snaps as *mut ::core::ffi::c_void);
        let mut history_json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(health);
        if history_json.is_null() {
            cJSON_Delete(health);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to encode health history\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        http_response_set_json(res, 200 as ::core::ffi::c_int, history_json);
        free(history_json as *mut ::core::ffi::c_void);
        cJSON_Delete(health);
        return;
    }
    let mut streams_up: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut streams_degraded: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut streams_down: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < count {
        match stream_health_status_t(
            (*snaps.offset(i_0 as isize)).health_status as ::core::ffi::c_uint,
        ) {
            stream_health_status_t::STREAM_HEALTH_UP => {
                streams_up += 1;
            }
            stream_health_status_t::STREAM_HEALTH_DEGRADED => {
                streams_degraded += 1;
            }
            stream_health_status_t::STREAM_HEALTH_DOWN => {
                streams_down += 1;
            }
            _ => {}
        }
        i_0 += 1;
    }
    let mut overall_status: *const ::core::ffi::c_char =
        b"healthy\0".as_ptr() as *const ::core::ffi::c_char;
    if streams_down > 0 as ::core::ffi::c_int {
        overall_status = b"critical\0".as_ptr() as *const ::core::ffi::c_char;
    } else if streams_degraded > 0 as ::core::ffi::c_int {
        overall_status = b"degraded\0".as_ptr() as *const ::core::ffi::c_char;
    }
    cJSON_ReplaceItemInObjectCaseSensitive(
        health,
        b"healthy\0".as_ptr() as *const ::core::ffi::c_char,
        cJSON_CreateBool(
            (strcmp(
                overall_status,
                b"healthy\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        ),
    );
    cJSON_AddStringToObject(
        health,
        b"status\0".as_ptr() as *const ::core::ffi::c_char,
        overall_status,
    );
    let mut streams_obj: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        streams_obj,
        b"total\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        streams_obj,
        b"up\0".as_ptr() as *const ::core::ffi::c_char,
        streams_up as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        streams_obj,
        b"degraded\0".as_ptr() as *const ::core::ffi::c_char,
        streams_degraded as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        streams_obj,
        b"down\0".as_ptr() as *const ::core::ffi::c_char,
        streams_down as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        health,
        b"streams\0".as_ptr() as *const ::core::ffi::c_char,
        streams_obj,
    );
    let mut storage_health: storage_health_t = storage_health_t {
        pressure_level: disk_pressure_level_t::DISK_PRESSURE_NORMAL,
        free_space_pct: 0.,
        free_space_bytes: 0,
        total_space_bytes: 0,
        used_space_bytes: 0,
        last_check_time: 0,
        last_cleanup_time: 0,
        last_deep_time: 0,
        last_cleanup_deleted: 0,
        last_cleanup_freed: 0,
    };
    phase = api_phase_begin(
        req,
        b"storage health snapshot\0".as_ptr() as *const ::core::ffi::c_char,
    );
    get_storage_health(&raw mut storage_health);
    api_phase_end(&raw mut phase);
    let mut storage_obj: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        storage_obj,
        b"used_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        storage_health.used_space_bytes as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        storage_obj,
        b"available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        storage_health.free_space_bytes as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        health,
        b"storage\0".as_ptr() as *const ::core::ffi::c_char,
        storage_obj,
    );
    phase = api_phase_begin(
        req,
        b"detector state and stream details\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut detail_arr_0: *mut cJSON = cJSON_CreateArray();
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < count {
        let mut sd_0: *mut cJSON = cJSON_CreateObject();
        cJSON_AddStringToObject(
            sd_0,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_1 as isize)).stream_name as *mut ::core::ffi::c_char,
        );
        let mut status_str: *const ::core::ffi::c_char =
            b"down\0".as_ptr() as *const ::core::ffi::c_char;
        match stream_health_status_t(
            (*snaps.offset(i_1 as isize)).health_status as ::core::ffi::c_uint,
        ) {
            stream_health_status_t::STREAM_HEALTH_UP => {
                status_str = b"up\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_health_status_t::STREAM_HEALTH_DEGRADED => {
                status_str = b"degraded\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_health_status_t::STREAM_HEALTH_DOWN => {
                status_str = b"down\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        cJSON_AddStringToObject(
            sd_0,
            b"status\0".as_ptr() as *const ::core::ffi::c_char,
            status_str,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"observed_fps\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).current_fps,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"current_fps\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).current_fps,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"expected_fps\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).configured_fps,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"fps\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).current_fps,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"configured_fps\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).configured_fps,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"bitrate_bps\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).current_bitrate_bps,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"uptime_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).uptime_seconds as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"reconnects\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).reconnects_total as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"last_packet_ts\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).last_packet_ts as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"last_frame_ts\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).last_frame_ts as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"last_completed_segment_ts\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).last_completed_segment_ts as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"frames_total\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).frames_total as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"frames_dropped\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).frames_dropped as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"connection_latency_ms\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).connection_latency_ms,
        );
        let mut packet_age: ::core::ffi::c_double =
            if (*snaps.offset(i_1 as isize)).last_packet_ts > 0 as int_fast64_t {
                difftime(now, (*snaps.offset(i_1 as isize)).last_packet_ts)
            } else {
                1e9f64
            };
        let mut frame_age: ::core::ffi::c_double =
            if (*snaps.offset(i_1 as isize)).last_frame_ts > 0 as int_fast64_t {
                difftime(now, (*snaps.offset(i_1 as isize)).last_frame_ts)
            } else {
                1e9f64
            };
        let mut expected_segment: ::core::ffi::c_int =
            if (*snaps.offset(i_1 as isize)).expected_segment_duration > 0 as ::core::ffi::c_int {
                (*snaps.offset(i_1 as isize)).expected_segment_duration
            } else {
                30 as ::core::ffi::c_int
            };
        let mut recorder_window: ::core::ffi::c_int =
            expected_segment * 2 as ::core::ffi::c_int + 30 as ::core::ffi::c_int;
        if recorder_window < 90 as ::core::ffi::c_int {
            recorder_window = 90 as ::core::ffi::c_int;
        }
        let mut segment_age: ::core::ffi::c_double =
            if (*snaps.offset(i_1 as isize)).last_completed_segment_ts > 0 as int_fast64_t {
                difftime(now, (*snaps.offset(i_1 as isize)).last_completed_segment_ts)
            } else {
                1e9f64
            };
        let mut source_connected: bool =
            packet_age <= STREAM_HEALTH_FRAME_TIMEOUT_DOWN as ::core::ffi::c_double;
        let mut video_advancing: bool =
            frame_age <= STREAM_HEALTH_FRAME_TIMEOUT_UP as ::core::ffi::c_double;
        let mut recorder_writing: bool = segment_age <= recorder_window as ::core::ffi::c_double;
        let mut detector_state: unified_detection_state_t = get_unified_detection_state(
            &raw mut (*snaps.offset(i_1 as isize)).stream_name as *mut ::core::ffi::c_char,
        );
        let mut detector_operating: bool = video_advancing as ::core::ffi::c_int != 0
            && (detector_state.0 == unified_detection_state_t::UDT_STATE_BUFFERING.0
                || detector_state.0 == unified_detection_state_t::UDT_STATE_RECORDING.0
                || detector_state.0 == unified_detection_state_t::UDT_STATE_POST_BUFFER.0);
        cJSON_AddBoolToObject(
            sd_0,
            b"source_connected\0".as_ptr() as *const ::core::ffi::c_char,
            source_connected as cJSON_bool,
        );
        cJSON_AddBoolToObject(
            sd_0,
            b"video_advancing\0".as_ptr() as *const ::core::ffi::c_char,
            video_advancing as cJSON_bool,
        );
        cJSON_AddBoolToObject(
            sd_0,
            b"recorder_writing\0".as_ptr() as *const ::core::ffi::c_char,
            recorder_writing as cJSON_bool,
        );
        cJSON_AddBoolToObject(
            sd_0,
            b"detector_operating\0".as_ptr() as *const ::core::ffi::c_char,
            detector_operating as cJSON_bool,
        );
        cJSON_AddBoolToObject(
            sd_0,
            b"recorder_session_active\0".as_ptr() as *const ::core::ffi::c_char,
            ((*snaps.offset(i_1 as isize)).recording_active != 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int,
        );
        cJSON_AddBoolToObject(
            sd_0,
            b"recording_active\0".as_ptr() as *const ::core::ffi::c_char,
            recorder_writing as cJSON_bool,
        );
        cJSON_AddNumberToObject(
            sd_0,
            b"recording_gaps\0".as_ptr() as *const ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).recording_gaps_total as ::core::ffi::c_double,
        );
        if include_sparklines {
            let mut ring_data_0: [metrics_ring_sample_t; 60] = [metrics_ring_sample_t {
                fps: 0.,
                bitrate_kbps: 0.,
                timestamp: 0,
            }; 60];
            let mut ring_count_0: ::core::ffi::c_int = metrics_get_ring_data(
                &raw mut (*snaps.offset(i_1 as isize)).stream_name as *mut ::core::ffi::c_char,
                &raw mut ring_data_0 as *mut metrics_ring_sample_t,
                60 as ::core::ffi::c_int,
            );
            let mut fps_arr_0: *mut cJSON = cJSON_CreateArray();
            let mut bitrate_arr_0: *mut cJSON = cJSON_CreateArray();
            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_0 < ring_count_0 {
                cJSON_AddItemToArray(
                    fps_arr_0,
                    cJSON_CreateNumber(ring_data_0[j_0 as usize].fps as ::core::ffi::c_double),
                );
                cJSON_AddItemToArray(
                    bitrate_arr_0,
                    cJSON_CreateNumber(
                        ring_data_0[j_0 as usize].bitrate_kbps as ::core::ffi::c_double,
                    ),
                );
                j_0 += 1;
            }
            cJSON_AddItemToObject(
                sd_0,
                b"sparkline_fps\0".as_ptr() as *const ::core::ffi::c_char,
                fps_arr_0,
            );
            cJSON_AddItemToObject(
                sd_0,
                b"sparkline_bitrate\0".as_ptr() as *const ::core::ffi::c_char,
                bitrate_arr_0,
            );
        }
        cJSON_AddItemToArray(detail_arr_0, sd_0);
        i_1 += 1;
    }
    cJSON_AddItemToObject(
        health,
        b"streams_detail\0".as_ptr() as *const ::core::ffi::c_char,
        detail_arr_0,
    );
    api_phase_end(&raw mut phase);
    free(snaps as *mut ::core::ffi::c_void);
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(health);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert health JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(health);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert health JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(health);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/health request\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_web_server_thread_id(mut thread_id: pthread_t) {
    pthread_mutex_lock(&raw mut g_web_server_thread_mutex);
    g_web_server_thread_id = thread_id;
    g_web_server_thread_id_set = r#true != 0;
    g_web_server_thread_running = r#true != 0;
    pthread_mutex_unlock(&raw mut g_web_server_thread_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Web server thread ID set: %lu\0".as_ptr() as *const ::core::ffi::c_char,
        thread_id,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_web_server_thread_stopped() {
    pthread_mutex_lock(&raw mut g_web_server_thread_mutex);
    g_web_server_thread_running = r#false != 0;
    pthread_mutex_unlock(&raw mut g_web_server_thread_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Web server event-loop thread marked stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_hls_health(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/health/hls request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    unsafe extern "C" {
        #[link_name = "get_hls_watchdog_restart_count"]
        unsafe fn get_hls_watchdog_restart_count_0() -> ::core::ffi::c_int;
    }
    let mut restart_count: ::core::ffi::c_int = get_hls_watchdog_restart_count_0();
    let mut health: *mut cJSON = cJSON_CreateObject();
    if health.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create HLS health JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create HLS health JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        health,
        b"healthy\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        health,
        b"status\0".as_ptr() as *const ::core::ffi::c_char,
        b"ok\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        health,
        b"watchdogRestartCount\0".as_ptr() as *const ::core::ffi::c_char,
        restart_count as ::core::ffi::c_double,
    );
    let mut timestamp: [::core::ffi::c_char; 32] = [0; 32];
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut tm_buf: tm = tm {
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
    let mut tm_info: *const tm = localtime_r(&raw mut now, &raw mut tm_buf);
    strftime(
        &raw mut timestamp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%Y-%m-%d %H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
        tm_info,
    );
    cJSON_AddStringToObject(
        health,
        b"timestamp\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut timestamp as *mut ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(health);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert HLS health JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(health);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert HLS health JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(health);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/health/hls request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn is_web_server_thread_running() -> bool {
    pthread_mutex_lock(&raw mut g_web_server_thread_mutex);
    if !g_web_server_thread_id_set {
        pthread_mutex_unlock(&raw mut g_web_server_thread_mutex);
        return r#true != 0;
    }
    let mut running: bool = g_web_server_thread_running;
    pthread_mutex_unlock(&raw mut g_web_server_thread_mutex);
    return running;
}
unsafe extern "C" fn restart_web_server() -> bool {
    unsafe extern "C" {
        static mut http_server: http_server_handle_t;
    }
    if http_server.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Cannot restart web server: no server handle available\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if now - g_last_restart_attempt < RESTART_COOLDOWN_SECONDS as time_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Web server restart attempt too soon, waiting for cooldown\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if g_restart_attempts >= MAX_RESTART_ATTEMPTS {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Maximum web server restart attempts (%d) reached\0".as_ptr()
                as *const ::core::ffi::c_char,
            MAX_RESTART_ATTEMPTS,
        );
        return r#false != 0;
    }
    g_last_restart_attempt = now;
    g_restart_attempts += 1;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Attempting to restart web server (attempt %d/%d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        g_restart_attempts,
        MAX_RESTART_ATTEMPTS,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Stopping web server before restart...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_server_stop(http_server);
    usleep(500000 as ::core::ffi::c_int as __useconds_t);
    if http_server_start(http_server) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to restart web server\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Web server restarted successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    g_restart_attempts = 0 as ::core::ffi::c_int;
    g_is_healthy = r#true != 0;
    g_failed_health_checks = 0 as ::core::ffi::c_int;
    return r#true != 0;
}
unsafe extern "C" fn health_check_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"HealthCheck\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Health check thread started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    while g_health_thread_running {
        if is_shutdown_initiated() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Shutdown initiated, stopping health check thread\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            break;
        } else {
            if g_health_wake_initialized {
                interruptible_sleep_wait(&raw mut g_health_wake, g_health_check_interval);
            } else {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < g_health_check_interval
                    && g_health_thread_running as ::core::ffi::c_int != 0
                {
                    sleep(1 as ::core::ffi::c_uint);
                    if is_shutdown_initiated() {
                        break;
                    }
                    i += 1;
                }
            }
            if !g_health_thread_running || is_shutdown_initiated() as ::core::ffi::c_int != 0 {
                break;
            }
            let mut check_result: bool = perform_health_check();
            if !g_health_thread_running || is_shutdown_initiated() as ::core::ffi::c_int != 0 {
                break;
            }
            if !check_result {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Health check failed (consecutive failures: %d)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    g_failed_health_checks,
                );
                let mut event_loop_running: bool = is_web_server_thread_running();
                if !event_loop_running {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Web server thread is not running!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    g_server_needs_restart = r#true != 0;
                }
                if g_server_needs_restart {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Web server event-loop thread exited, attempting restart\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    if restart_web_server() {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Web server restart succeeded\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        g_server_needs_restart = r#false != 0;
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Web server restart failed\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                } else if g_failed_health_checks >= 2 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Health endpoint remains slow but the web event loop is alive; leaving the listener intact and retrying without an in-process restart\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            } else if g_restart_attempts > 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Health check succeeded after restart, resetting attempt counter\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                g_restart_attempts = 0 as ::core::ffi::c_int;
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Health check thread exiting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    g_health_thread_running = r#false != 0;
    g_health_thread_exited = r#true != 0;
    return NULL;
}
unsafe extern "C" fn reap_health_thread_locked() -> bool {
    if !g_health_thread_created {
        return r#true != 0;
    }
    if !g_health_thread_exited {
        return r#false != 0;
    }
    let mut rc: ::core::ffi::c_int = pthread_join(
        g_health_check_thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to join health check thread: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(rc),
        );
        return r#false != 0;
    }
    g_health_thread_created = r#false != 0;
    g_health_thread_stopping = r#false != 0;
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_health_check_system() {
    pthread_mutex_lock(&raw mut g_health_lifecycle_mutex);
    if !reap_health_thread_locked() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Health check worker still active; initialization deferred\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut g_health_lifecycle_mutex);
        return;
    }
    g_start_time = time(::core::ptr::null_mut::<time_t>());
    g_last_health_check = g_start_time;
    g_is_healthy = r#true != 0;
    g_failed_health_checks = 0 as ::core::ffi::c_int;
    g_total_requests = 0 as ::core::ffi::c_int;
    g_failed_requests = 0 as ::core::ffi::c_int;
    g_server_needs_restart = r#false != 0;
    g_restart_attempts = 0 as ::core::ffi::c_int;
    g_last_restart_attempt = 0 as time_t;
    unsafe extern "C" {
        static mut config: config_t;
    }
    g_health_check_tls = httpd_secure_cookies();
    snprintf(
        &raw mut g_health_check_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        if g_health_check_tls as ::core::ffi::c_int != 0 {
            b"https://127.0.0.1:%d/api/health?liveness=true\0".as_ptr()
                as *const ::core::ffi::c_char
        } else {
            b"http://127.0.0.1:%d/api/health?liveness=true\0".as_ptr() as *const ::core::ffi::c_char
        },
        config.web_port,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Health check system initialized with URL: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_health_check_url as *mut ::core::ffi::c_char,
    );
    pthread_mutex_unlock(&raw mut g_health_lifecycle_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_health_check_thread() {
    pthread_mutex_lock(&raw mut g_health_lifecycle_mutex);
    if !reap_health_thread_locked() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Health check worker already owned; start deferred until it exits\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut g_health_lifecycle_mutex);
        return;
    }
    if !g_health_wake_initialized {
        if interruptible_sleep_init(&raw mut g_health_wake) == 0 as ::core::ffi::c_int {
            g_health_wake_initialized = r#true != 0;
        }
    } else {
        interruptible_sleep_reset(&raw mut g_health_wake);
    }
    g_health_thread_running = r#true != 0;
    g_health_thread_exited = r#false != 0;
    if pthread_create(
        &raw mut g_health_check_thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            health_check_thread_func
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create health check thread\0".as_ptr() as *const ::core::ffi::c_char,
        );
        g_health_thread_running = r#false != 0;
        g_health_thread_exited = r#true != 0;
        pthread_mutex_unlock(&raw mut g_health_lifecycle_mutex);
        return;
    }
    g_health_thread_created = r#true != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Health check thread started with %d second interval\0".as_ptr()
            as *const ::core::ffi::c_char,
        g_health_check_interval,
    );
    pthread_mutex_unlock(&raw mut g_health_lifecycle_mutex);
}
unsafe extern "C" fn stop_health_thread_locked() {
    if reap_health_thread_locked() {
        return;
    }
    g_health_thread_stopping = r#true != 0;
    g_health_thread_running = r#false != 0;
    if g_health_wake_initialized {
        interruptible_sleep_wake(&raw mut g_health_wake);
    }
    let timeout_ms: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
    let poll_interval_ms: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut elapsed_ms: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while elapsed_ms < timeout_ms {
        if reap_health_thread_locked() {
            return;
        }
        usleep((poll_interval_ms * 1000 as ::core::ffi::c_int) as __useconds_t);
        elapsed_ms += poll_interval_ms;
    }
    if !reap_health_thread_locked() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Health check thread still stopping after %d ms; retaining ownership\0".as_ptr()
                as *const ::core::ffi::c_char,
            timeout_ms,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_health_check_thread() {
    pthread_mutex_lock(&raw mut g_health_lifecycle_mutex);
    stop_health_thread_locked();
    pthread_mutex_unlock(&raw mut g_health_lifecycle_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_health_check_system() {
    pthread_mutex_lock(&raw mut g_health_lifecycle_mutex);
    stop_health_thread_locked();
    if !g_health_thread_created {
        g_is_healthy = r#false != 0;
        g_failed_health_checks = 0 as ::core::ffi::c_int;
        g_server_needs_restart = r#false != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Health check system cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    pthread_mutex_unlock(&raw mut g_health_lifecycle_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_health_metrics(mut request_succeeded: bool) {
    g_total_requests += 1;
    if !request_succeeded {
        g_failed_requests += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_web_server_healthy() -> bool {
    return !g_health_thread_stopping
        && g_is_healthy as ::core::ffi::c_int != 0
        && is_web_server_thread_running() as ::core::ffi::c_int != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_failed_health_checks() -> ::core::ffi::c_int {
    return g_failed_health_checks;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_health_metrics() {
    pthread_mutex_lock(&raw mut g_health_lifecycle_mutex);
    if g_health_thread_stopping as ::core::ffi::c_int != 0 && !reap_health_thread_locked() {
        pthread_mutex_unlock(&raw mut g_health_lifecycle_mutex);
        return;
    }
    g_total_requests = 0 as ::core::ffi::c_int;
    g_failed_requests = 0 as ::core::ffi::c_int;
    g_failed_health_checks = 0 as ::core::ffi::c_int;
    g_is_healthy = r#true != 0;
    pthread_mutex_unlock(&raw mut g_health_lifecycle_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_server_restart_needed() -> bool {
    return g_server_needs_restart;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_server_for_restart() {
    g_server_needs_restart = r#true != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Web server marked for restart\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_server_restart_flag() {
    g_server_needs_restart = r#false != 0;
    g_restart_attempts = 0 as ::core::ffi::c_int;
}
