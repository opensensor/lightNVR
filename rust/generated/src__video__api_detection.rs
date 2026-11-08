unsafe extern "C" {
    pub type curl_mime;
    pub type curl_mimepart;
    pub type jpeg_encoder_cache;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn curl_mime_init(easy: *mut ::core::ffi::c_void) -> *mut curl_mime;
    unsafe fn curl_mime_free(mime: *mut curl_mime);
    unsafe fn curl_mime_addpart(mime: *mut curl_mime) -> *mut curl_mimepart;
    unsafe fn curl_mime_name(
        part: *mut curl_mimepart,
        name: *const ::core::ffi::c_char,
    ) -> CURLcode;
    unsafe fn curl_mime_filename(
        part: *mut curl_mimepart,
        filename: *const ::core::ffi::c_char,
    ) -> CURLcode;
    unsafe fn curl_mime_type(
        part: *mut curl_mimepart,
        mimetype: *const ::core::ffi::c_char,
    ) -> CURLcode;
    unsafe fn curl_mime_data(
        part: *mut curl_mimepart,
        data: *const ::core::ffi::c_char,
        datasize: size_t,
    ) -> CURLcode;
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
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Print(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_GetErrorPtr() -> *const ::core::ffi::c_char;
    unsafe fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    static mut g_config: config_t;
    unsafe fn api_detection_format_name(
        format: api_detection_format_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn api_detection_format_parse(
        name: *const ::core::ffi::c_char,
        out: *mut api_detection_format_t,
    ) -> bool;
    unsafe fn curl_init_global() -> ::core::ffi::c_int;
    unsafe fn is_shutdown_initiated() -> bool;
    unsafe fn event_producer_publish_detection(
        camera_uuid: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_producer_publish_detection_for_stream(
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn base64_encode_alloc(
        input: *const ::core::ffi::c_uchar,
        input_len: size_t,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn detection_evidence_observed_at_ms(frame_ts: time_t) -> int64_t;
    unsafe fn detection_evidence_capture_jpeg(
        camera: *const detection_evidence_camera_t,
        engine: *const detection_evidence_engine_t,
        detections: *const detection_t,
        count: ::core::ffi::c_int,
        observed_at_ms: int64_t,
        jpeg: *mut uint8_t,
        jpeg_size: size_t,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        transform: *const observation_evidence_transform_t,
        submit_result: *mut observation_evidence_result_t,
        asset_uuid_out: *mut ::core::ffi::c_char,
    ) -> detection_evidence_status_t;
    unsafe fn filter_detections_by_zones(
        stream_name: *const ::core::ffi::c_char,
        result: *mut detection_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn filter_detections_by_stream_objects(
        stream_name: *const ::core::ffi::c_char,
        result: *mut detection_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn jpeg_encoder_cache_encode_to_memory(
        encoder: *mut jpeg_encoder_cache_t,
        frame_data: *const ::core::ffi::c_uchar,
        out_data: *mut *mut ::core::ffi::c_uchar,
        out_size: *mut size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn jpeg_encoder_get_cached(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        channels: ::core::ffi::c_int,
        quality: ::core::ffi::c_int,
    ) -> *mut jpeg_encoder_cache_t;
    unsafe fn jpeg_encoder_cleanup_all();
    unsafe fn store_detections_in_db(
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
        timestamp: time_t,
        recording_id: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn store_detections_in_db_for_camera(
        camera_uuid: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
        timestamp: time_t,
        recording_id: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_get_snapshot(
        stream_name: *const ::core::ffi::c_char,
        jpeg_data: *mut *mut ::core::ffi::c_uchar,
        jpeg_size: *mut size_t,
    ) -> bool;
    unsafe fn go2rtc_integration_is_initialized() -> bool;
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
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub struct api_detection_format_t(pub ::core::ffi::c_uint);
impl api_detection_format_t {
    pub const API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT: Self = Self(0);
    pub const API_DETECTION_FORMAT_DOODS2: Self = Self(1);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_evidence_result_t(pub ::core::ffi::c_int);
impl observation_evidence_result_t {
    pub const OBS_EVIDENCE_ACCEPTED: Self = Self(0);
    pub const OBS_EVIDENCE_DROPPED_QUEUE_FULL: Self = Self(1);
    pub const OBS_EVIDENCE_DROPPED_BUDGET: Self = Self(2);
    pub const OBS_EVIDENCE_DISABLED: Self = Self(3);
    pub const OBS_EVIDENCE_INVALID: Self = Self(-1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_transform_mode_t(pub ::core::ffi::c_uint);
impl observation_transform_mode_t {
    pub const OBS_TRANSFORM_NONE: Self = Self(0);
    pub const OBS_TRANSFORM_RESIZE: Self = Self(1);
    pub const OBS_TRANSFORM_LETTERBOX: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_evidence_transform_t {
    pub mode: observation_transform_mode_t,
    pub source_width: ::core::ffi::c_int,
    pub source_height: ::core::ffi::c_int,
    pub inference_width: ::core::ffi::c_int,
    pub inference_height: ::core::ffi::c_int,
    pub scale_x: ::core::ffi::c_double,
    pub scale_y: ::core::ffi::c_double,
    pub pad_x: ::core::ffi::c_double,
    pub pad_y: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_detection_options_t {
    pub format: api_detection_format_t,
    pub backend: [::core::ffi::c_char; 32],
    pub detector_name: [::core::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_detection_response_meta_t {
    pub letterbox: bool,
    pub inference_width: ::core::ffi::c_int,
    pub inference_height: ::core::ffi::c_int,
    pub scale_x: ::core::ffi::c_double,
    pub scale_y: ::core::ffi::c_double,
    pub pad_x: ::core::ffi::c_double,
    pub pad_y: ::core::ffi::c_double,
    pub pixel_units: bool,
    pub model: [::core::ffi::c_char; 128],
    pub model_version: [::core::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_detection_evidence_t {
    pub camera_uuid: *const ::core::ffi::c_char,
    pub engine_key: *const ::core::ffi::c_char,
    pub evidence_enabled: bool,
    pub labels_csv: *const ::core::ffi::c_char,
    pub observed_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memory_struct_t {
    pub memory: *mut ::core::ffi::c_char,
    pub size: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct submitted_image_t {
    pub jpeg: *mut uint8_t,
    pub size: size_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub transform: observation_evidence_transform_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct detection_evidence_status_t(pub ::core::ffi::c_uint);
impl detection_evidence_status_t {
    pub const DETECTION_EVIDENCE_SUBMITTED: Self = Self(0);
    pub const DETECTION_EVIDENCE_NOT_ENABLED: Self = Self(1);
    pub const DETECTION_EVIDENCE_NOT_ELIGIBLE: Self = Self(2);
    pub const DETECTION_EVIDENCE_NOT_OBJECT_ENGINE: Self = Self(3);
    pub const DETECTION_EVIDENCE_ERROR: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_evidence_engine_t {
    pub engine_key: *const ::core::ffi::c_char,
    pub engine_type: *const ::core::ffi::c_char,
    pub model_path: *const ::core::ffi::c_char,
    pub model_version: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_evidence_camera_t {
    pub camera_uuid: *const ::core::ffi::c_char,
    pub stream_name: *const ::core::ffi::c_char,
    pub evidence_enabled: bool,
    pub labels_csv: *const ::core::ffi::c_char,
    pub recording_id: int64_t,
}
pub type jpeg_encoder_cache_t = jpeg_encoder_cache;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MAX_LABEL_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MAX_ZONE_ID_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut initialized: bool = r#false != 0;
static mut curl_mutex: pthread_mutex_t = pthread_mutex_t {
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
pub const API_DETECTION_JPEG_QUALITY_DEFAULT: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
pub const API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE: ::core::ffi::c_int =
    1024 as ::core::ffi::c_int;
pub const DOODS2_BODY_PREFIX_FORMAT: [::core::ffi::c_char; 57] = unsafe {
    ::core::mem::transmute::<[u8; 57], [::core::ffi::c_char; 57]>(
        *b"{\"id\":%s,\"detector_name\":%s,\"detect\":{\"*\":%.2f},\"data\":\"\0",
    )
};
pub const ASCII_PRINTABLE_MIN: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const ASCII_PRINTABLE_MAX: ::core::ffi::c_int = 126 as ::core::ffi::c_int;
pub const API_DETECTION_SNAPSHOT_UNAVAILABLE: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
unsafe extern "C" fn is_safe_base_url(mut url: *const ::core::ffi::c_char) -> bool {
    if url.is_null() {
        return r#false != 0;
    }
    let mut p: *const ::core::ffi::c_uchar = url as *const ::core::ffi::c_uchar;
    while *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if (*p as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == 127 as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        p = p.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn setup_common_curl_options(mut handle: *mut ::core::ffi::c_void) {
    if handle.is_null() {
        return;
    }
    curl_easy_setopt(
        handle,
        CURLoption::CURLOPT_NOSIGNAL,
        1 as ::core::ffi::c_long,
    );
    curl_easy_setopt(
        handle,
        CURLoption::CURLOPT_FOLLOWLOCATION,
        0 as ::core::ffi::c_long,
    );
}
unsafe extern "C" fn is_api_detection_system_initialized() -> bool {
    let mut is_ready: bool = r#false != 0;
    pthread_mutex_lock(&raw mut curl_mutex);
    is_ready = initialized;
    pthread_mutex_unlock(&raw mut curl_mutex);
    return is_ready;
}
unsafe extern "C" fn normalize_api_detection_threshold(
    mut threshold: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return if threshold > 0.0f32 {
        threshold
    } else {
        0.5f32
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_detection_should_use_go2rtc_snapshot(
    mut frame_data: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut channels: ::core::ffi::c_int,
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    let mut has_decoded_frame: bool = !frame_data.is_null()
        && width > 0 as ::core::ffi::c_int
        && height > 0 as ::core::ffi::c_int
        && channels > 0 as ::core::ffi::c_int;
    return !has_decoded_frame
        && !stream_name.is_null()
        && *stream_name.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
}
unsafe extern "C" fn sanitize_backend(
    mut backend: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    static mut default_backend: *const ::core::ffi::c_char =
        b"onnx\0".as_ptr() as *const ::core::ffi::c_char;
    if backend.is_null()
        || *backend.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return default_backend;
    }
    let mut p: *const ::core::ffi::c_char = backend;
    while *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_char = *p;
        if !(c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
            && c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            || c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
            || c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '_' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '-' as ::core::ffi::c_int)
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"API Detection: Invalid character '%c' in backend '%s', using default '%s' instead.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                c as ::core::ffi::c_int,
                backend,
                default_backend,
            );
            return default_backend;
        }
        p = p.offset(1);
    }
    return backend;
}
unsafe extern "C" fn validate_api_detection_base_url(
    mut base_url: *const ::core::ffi::c_char,
    mut context: *const ::core::ffi::c_char,
) -> bool {
    if base_url.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: API URL is NULL.\0".as_ptr() as *const ::core::ffi::c_char,
            context,
        );
        return r#false != 0;
    }
    if !is_safe_base_url(base_url) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Invalid API URL '%s' (contains spaces or control characters).\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
            base_url,
        );
        return r#false != 0;
    }
    if strncmp(
        base_url,
        b"http://\0".as_ptr() as *const ::core::ffi::c_char,
        7 as size_t,
    ) != 0 as ::core::ffi::c_int
        && strncmp(
            base_url,
            b"https://\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Invalid URL format: %s (must start with http:// or https://)\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
            base_url,
        );
        return r#false != 0;
    }
    let mut authority_start: *const ::core::ffi::c_char =
        strstr(base_url, b"://\0".as_ptr() as *const ::core::ffi::c_char);
    if authority_start.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Invalid API URL '%s' (missing scheme separator).\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
            base_url,
        );
        return r#false != 0;
    }
    authority_start = authority_start.offset(3 as ::core::ffi::c_int as isize);
    let mut authority_end: *const ::core::ffi::c_char = authority_start;
    while *authority_end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && *authority_end as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        && *authority_end as ::core::ffi::c_int != '?' as ::core::ffi::c_int
        && *authority_end as ::core::ffi::c_int != '#' as ::core::ffi::c_int
    {
        authority_end = authority_end.offset(1);
    }
    if authority_end == authority_start {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Invalid API URL '%s' (missing host).\0".as_ptr() as *const ::core::ffi::c_char,
            context,
            base_url,
        );
        return r#false != 0;
    }
    if !memchr(
        authority_start as *const ::core::ffi::c_void,
        '@' as ::core::ffi::c_int,
        authority_end.offset_from(authority_start) as size_t,
    )
    .is_null()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Invalid API URL '%s' (userinfo is not allowed).\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
            base_url,
        );
        return r#false != 0;
    }
    let mut first_qmark: *const ::core::ffi::c_char = strchr(base_url, '?' as ::core::ffi::c_int);
    let mut second_qmark: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !first_qmark.is_null() {
        second_qmark = strchr(
            first_qmark.offset(1 as ::core::ffi::c_int as isize),
            '?' as ::core::ffi::c_int,
        );
    }
    if !strchr(base_url, '#' as ::core::ffi::c_int).is_null() || !second_qmark.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Invalid API URL '%s' (contains fragment or multiple '?').\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
            base_url,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn resolve_api_url(
    mut api_url: *const ::core::ffi::c_char,
    mut context: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if !api_url.is_null()
        && strcmp(
            api_url,
            b"api-detection\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Using API URL from config: %s\0".as_ptr() as *const ::core::ffi::c_char,
            context,
            if g_config.api_detection_url[0usize] as ::core::ffi::c_int != 0 {
                &raw mut g_config.api_detection_url as *mut ::core::ffi::c_char
            } else {
                b"NULL\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return &raw mut g_config.api_detection_url as *mut ::core::ffi::c_char;
    }
    return api_url;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_detection_options_from_config(
    mut options: *mut api_detection_options_t,
) {
    if options.is_null() {
        return;
    }
    memset(
        options as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<api_detection_options_t>(),
    );
    let mut format: api_detection_format_t =
        api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT;
    if g_config.api_detection_format[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && !api_detection_format_parse(
            &raw mut g_config.api_detection_format as *mut ::core::ffi::c_char,
            &raw mut format,
        )
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"API Detection: Unknown api_detection format '%s', using %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut g_config.api_detection_format as *mut ::core::ffi::c_char,
            api_detection_format_name(
                api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT,
            ),
        );
        format = api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT;
    }
    (*options).format = format;
    safe_strcpy(
        &raw mut (*options).backend as *mut ::core::ffi::c_char,
        &raw mut g_config.api_detection_backend as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*options).detector_name as *mut ::core::ffi::c_char,
        &raw mut g_config.api_detection_detector_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_detection_options_apply_json(
    mut options: *mut api_detection_options_t,
    mut config_json: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if options.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if config_json.is_null()
        || *config_json.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut root: *mut cJSON = cJSON_Parse(config_json);
    if root.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"API Detection: Engine config is not valid JSON\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if cJSON_IsObject(root) == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"API Detection: Engine config must be a JSON object\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        rc = -1 as ::core::ffi::c_int;
    } else {
        let mut format: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            root,
            b"format\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !format.is_null() && cJSON_IsNull(format) == 0 {
            let mut parsed: api_detection_format_t =
                api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT;
            if cJSON_IsString(format) != 0
                && api_detection_format_parse((*format).valuestring, &raw mut parsed)
                    as ::core::ffi::c_int
                    != 0
            {
                (*options).format = parsed;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"API Detection: Engine config names an unknown format%s%s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    if cJSON_IsString(format) != 0 {
                        b": \0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    if cJSON_IsString(format) != 0 {
                        (*format).valuestring
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                rc = -1 as ::core::ffi::c_int;
            }
        }
        let mut backend: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            root,
            b"backend\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsString(backend) != 0
            && *(*backend).valuestring.offset(0isize) as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*options).backend as *mut ::core::ffi::c_char,
                (*backend).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                0 as size_t,
            );
        }
        let mut detector: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            root,
            b"detector_name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsString(detector) != 0
            && *(*detector).valuestring.offset(0isize) as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*options).detector_name as *mut ::core::ffi::c_char,
                (*detector).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
    }
    cJSON_Delete(root);
    return rc;
}
unsafe extern "C" fn resolve_options(
    mut options: *const api_detection_options_t,
    mut storage: *mut api_detection_options_t,
) -> *const api_detection_options_t {
    if !options.is_null() {
        return options;
    }
    api_detection_options_from_config(storage);
    return storage;
}
unsafe extern "C" fn effective_detector_name(
    mut options: *const api_detection_options_t,
) -> *const ::core::ffi::c_char {
    return if (*options).detector_name[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        &raw const (*options).detector_name as *const ::core::ffi::c_char
    } else {
        b"default\0".as_ptr() as *const ::core::ffi::c_char
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_detection_build_request_url(
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
    mut base_url: *const ::core::ffi::c_char,
    mut threshold: ::core::ffi::c_float,
    mut options: *const api_detection_options_t,
) -> ::core::ffi::c_int {
    if buffer.is_null()
        || buffer_size == 0 as size_t
        || base_url.is_null()
        || !is_safe_base_url(base_url)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut storage: api_detection_options_t = api_detection_options_t {
        format: api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT,
        backend: [0; 32],
        detector_name: [0; 64],
    };
    options = resolve_options(options, &raw mut storage);
    let mut url_len: ::core::ffi::c_int = 0;
    if (*options).format.0 == api_detection_format_t::API_DETECTION_FORMAT_DOODS2.0 {
        url_len = snprintf(
            buffer,
            buffer_size,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            base_url,
        );
    } else {
        let mut backend_param: *const ::core::ffi::c_char =
            sanitize_backend(&raw const (*options).backend as *const ::core::ffi::c_char);
        let mut actual_threshold: ::core::ffi::c_float =
            normalize_api_detection_threshold(threshold);
        let mut separator: ::core::ffi::c_char =
            (if !strchr(base_url, '?' as ::core::ffi::c_int).is_null() {
                '&' as ::core::ffi::c_int
            } else {
                '?' as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
        url_len = snprintf(
            buffer,
            buffer_size,
            b"%s%cbackend=%s&confidence_threshold=%.2f&return_image=false\0".as_ptr()
                as *const ::core::ffi::c_char,
            base_url,
            separator as ::core::ffi::c_int,
            backend_param,
            actual_threshold as ::core::ffi::c_double,
        );
    }
    if url_len < 0 as ::core::ffi::c_int || url_len as size_t >= buffer_size {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_quote(mut text: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut item: *mut cJSON = cJSON_CreateString(if !text.is_null() {
        text
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    });
    if item.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut quoted: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(item);
    cJSON_Delete(item);
    return quoted;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_detection_build_doods2_body(
    mut jpeg_data: *const ::core::ffi::c_uchar,
    mut jpeg_size: size_t,
    mut threshold: ::core::ffi::c_float,
    mut request_id: *const ::core::ffi::c_char,
    mut options: *const api_detection_options_t,
) -> *mut ::core::ffi::c_char {
    if jpeg_data.is_null() || jpeg_size == 0 as size_t {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut storage: api_detection_options_t = api_detection_options_t {
        format: api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT,
        backend: [0; 32],
        detector_name: [0; 64],
    };
    options = resolve_options(options, &raw mut storage);
    let mut id_json: *mut ::core::ffi::c_char = json_quote(
        if !request_id.is_null() && *request_id.offset(0isize) as ::core::ffi::c_int != 0 {
            request_id
        } else {
            b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut detector_json: *mut ::core::ffi::c_char = json_quote(effective_detector_name(options));
    let mut encoded: *mut ::core::ffi::c_char = base64_encode_alloc(jpeg_data, jpeg_size);
    let mut body: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !id_json.is_null() && !detector_json.is_null() && !encoded.is_null() {
        let mut percent: ::core::ffi::c_double =
            normalize_api_detection_threshold(threshold) as ::core::ffi::c_double * 100.0f64;
        let mut prefix_len: ::core::ffi::c_int = snprintf(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as size_t,
            DOODS2_BODY_PREFIX_FORMAT.as_ptr(),
            id_json,
            detector_json,
            percent,
        );
        let mut encoded_len: size_t = strlen(encoded);
        if prefix_len > 0 as ::core::ffi::c_int {
            let mut total: size_t = (prefix_len as size_t)
                .wrapping_add(encoded_len)
                .wrapping_add(3 as size_t);
            body = malloc(total) as *mut ::core::ffi::c_char;
            if !body.is_null() {
                let mut written: ::core::ffi::c_int = snprintf(
                    body,
                    total,
                    DOODS2_BODY_PREFIX_FORMAT.as_ptr(),
                    id_json,
                    detector_json,
                    percent,
                );
                if written == prefix_len {
                    memcpy(
                        body.offset(written as isize) as *mut ::core::ffi::c_void,
                        encoded as *const ::core::ffi::c_void,
                        encoded_len,
                    );
                    *body.offset((written as size_t).wrapping_add(encoded_len) as isize) =
                        '"' as ::core::ffi::c_char;
                    *body.offset(
                        (written as size_t)
                            .wrapping_add(encoded_len)
                            .wrapping_add(1 as size_t) as isize,
                    ) = '}' as ::core::ffi::c_char;
                    *body.offset(
                        (written as size_t)
                            .wrapping_add(encoded_len)
                            .wrapping_add(2 as size_t) as isize,
                    ) = '\0' as ::core::ffi::c_char;
                } else {
                    free(body as *mut ::core::ffi::c_void);
                    body = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            }
        }
    }
    if body.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"API Detection: Failed to build DOODS2 request body (%zu byte JPEG)\0".as_ptr()
                as *const ::core::ffi::c_char,
            jpeg_size,
        );
    }
    free(id_json as *mut ::core::ffi::c_void);
    free(detector_json as *mut ::core::ffi::c_void);
    free(encoded as *mut ::core::ffi::c_void);
    return body;
}
unsafe extern "C" fn number_item(
    mut object: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_double,
) -> bool {
    let mut item: *const cJSON = if !object.is_null() {
        cJSON_GetObjectItem(object, key)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if item.is_null() || cJSON_IsNumber(item) == 0 {
        return r#false != 0;
    }
    *out = (*item).valuedouble;
    return r#true != 0;
}
unsafe extern "C" fn extract_box(
    mut detection: *const cJSON,
    mut x_min: *mut ::core::ffi::c_double,
    mut y_min: *mut ::core::ffi::c_double,
    mut x_max: *mut ::core::ffi::c_double,
    mut y_max: *mut ::core::ffi::c_double,
) -> bool {
    let mut nested: *const cJSON = cJSON_GetObjectItem(
        detection,
        b"bounding_box\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut source: *const cJSON = if !nested.is_null() && cJSON_IsObject(nested) != 0 {
        nested
    } else {
        detection
    };
    if number_item(
        source,
        b"x_min\0".as_ptr() as *const ::core::ffi::c_char,
        x_min,
    ) as ::core::ffi::c_int
        != 0
        && number_item(
            source,
            b"y_min\0".as_ptr() as *const ::core::ffi::c_char,
            y_min,
        ) as ::core::ffi::c_int
            != 0
        && number_item(
            source,
            b"x_max\0".as_ptr() as *const ::core::ffi::c_char,
            x_max,
        ) as ::core::ffi::c_int
            != 0
        && number_item(
            source,
            b"y_max\0".as_ptr() as *const ::core::ffi::c_char,
            y_max,
        ) as ::core::ffi::c_int
            != 0
    {
        return r#true != 0;
    }
    return number_item(
        detection,
        b"left\0".as_ptr() as *const ::core::ffi::c_char,
        x_min,
    ) as ::core::ffi::c_int
        != 0
        && number_item(
            detection,
            b"top\0".as_ptr() as *const ::core::ffi::c_char,
            y_min,
        ) as ::core::ffi::c_int
            != 0
        && number_item(
            detection,
            b"right\0".as_ptr() as *const ::core::ffi::c_char,
            x_max,
        ) as ::core::ffi::c_int
            != 0
        && number_item(
            detection,
            b"bottom\0".as_ptr() as *const ::core::ffi::c_char,
            y_max,
        ) as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn log_json_item(
    mut message: *const ::core::ffi::c_char,
    mut item: *const cJSON,
    mut as_error: bool,
) {
    let mut json_str: *mut ::core::ffi::c_char = cJSON_Print(item);
    if json_str.is_null() {
        return;
    }
    if as_error {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            message,
            json_str,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            message,
            json_str,
        );
    }
    free(json_str as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn parse_response_meta(
    mut root: *const cJSON,
    mut meta: *mut api_detection_response_meta_t,
) {
    memset(
        meta as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<api_detection_response_meta_t>(),
    );
    (*meta).pad_y = -1.0f64;
    (*meta).pad_x = (*meta).pad_y;
    let mut model: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(root, b"model\0".as_ptr() as *const ::core::ffi::c_char);
    if cJSON_IsString(model) != 0
        && *(*model).valuestring.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && strlen((*model).valuestring) < ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
    {
        safe_strcpy(
            &raw mut (*meta).model as *mut ::core::ffi::c_char,
            (*model).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    }
    let mut version: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"model_version\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsString(version) != 0
        && *(*version).valuestring.offset(0isize) as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
        && strlen((*version).valuestring) < ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
    {
        safe_strcpy(
            &raw mut (*meta).model_version as *mut ::core::ffi::c_char,
            (*version).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    let mut lb: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"letterbox\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if lb.is_null() || cJSON_IsObject(lb) == 0 {
        return;
    }
    let mut w: ::core::ffi::c_double = 0.0f64;
    let mut h: ::core::ffi::c_double = 0.0f64;
    let mut v: ::core::ffi::c_double = 0.0f64;
    if !number_item(
        lb,
        b"width\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut w,
    ) || !number_item(
        lb,
        b"height\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut h,
    ) || !(w >= 1.0f64)
        || !(h >= 1.0f64)
        || w > 65535.0f64
        || h > 65535.0f64
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"API Detection: Ignoring letterbox declaration without a valid width/height\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*meta).inference_width = w as ::core::ffi::c_int;
    (*meta).inference_height = h as ::core::ffi::c_int;
    if number_item(
        lb,
        b"scale\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut v,
    ) as ::core::ffi::c_int
        != 0
        && v > 0.0f64
    {
        (*meta).scale_y = v;
        (*meta).scale_x = (*meta).scale_y;
    }
    if number_item(
        lb,
        b"scale_x\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut v,
    ) as ::core::ffi::c_int
        != 0
        && v > 0.0f64
    {
        (*meta).scale_x = v;
    }
    if number_item(
        lb,
        b"scale_y\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut v,
    ) as ::core::ffi::c_int
        != 0
        && v > 0.0f64
    {
        (*meta).scale_y = v;
    }
    if number_item(
        lb,
        b"pad_x\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut v,
    ) as ::core::ffi::c_int
        != 0
        && v >= 0.0f64
    {
        (*meta).pad_x = v;
    }
    if number_item(
        lb,
        b"pad_y\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut v,
    ) as ::core::ffi::c_int
        != 0
        && v >= 0.0f64
    {
        (*meta).pad_y = v;
    }
    let mut units: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(lb, b"box_units\0".as_ptr() as *const ::core::ffi::c_char);
    (*meta).pixel_units = cJSON_IsString(units) != 0
        && strcasecmp(
            (*units).valuestring,
            b"pixels\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    (*meta).letterbox = r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_detection_parse_response(
    mut json: *const ::core::ffi::c_char,
    mut options: *const api_detection_options_t,
    mut result: *mut detection_result_t,
) -> ::core::ffi::c_int {
    return api_detection_parse_response_meta(
        json,
        options,
        result,
        ::core::ptr::null_mut::<api_detection_response_meta_t>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_detection_parse_response_meta(
    mut json: *const ::core::ffi::c_char,
    mut options: *const api_detection_options_t,
    mut result: *mut detection_result_t,
    mut meta: *mut api_detection_response_meta_t,
) -> ::core::ffi::c_int {
    if !meta.is_null() {
        memset(
            meta as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<api_detection_response_meta_t>(),
        );
        (*meta).pad_y = -1.0f64;
        (*meta).pad_x = (*meta).pad_y;
    }
    if json.is_null() || result.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    let mut storage: api_detection_options_t = api_detection_options_t {
        format: api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT,
        backend: [0; 32],
        detector_name: [0; 64],
    };
    options = resolve_options(options, &raw mut storage);
    let doods2: bool = (*options).format.0 == api_detection_format_t::API_DETECTION_FORMAT_DOODS2.0;
    let mut root: *mut cJSON = cJSON_Parse(json);
    if root.is_null() {
        let mut error_ptr: *const ::core::ffi::c_char = cJSON_GetErrorPtr();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"API Detection: Failed to parse JSON response: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            if !error_ptr.is_null() {
                error_ptr
            } else {
                b"Unknown error\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return -1 as ::core::ffi::c_int;
    }
    if !meta.is_null() && cJSON_IsObject(root) != 0 {
        parse_response_meta(root, meta);
    }
    if doods2 {
        let mut error: *const cJSON =
            cJSON_GetObjectItem(root, b"error\0".as_ptr() as *const ::core::ffi::c_char);
        if !error.is_null()
            && cJSON_IsString(error) != 0
            && *(*error).valuestring.offset(0isize) as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"API Detection: DOODS2 reported an error: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*error).valuestring,
            );
            cJSON_Delete(root);
            return -1 as ::core::ffi::c_int;
        }
    }
    let mut detections: *const cJSON =
        cJSON_GetObjectItem(root, b"detections\0".as_ptr() as *const ::core::ffi::c_char);
    if !detections.is_null() && cJSON_IsNull(detections) != 0 {
        cJSON_Delete(root);
        return 0 as ::core::ffi::c_int;
    }
    if detections.is_null() || cJSON_IsArray(detections) == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"API Detection: Invalid JSON response: missing or invalid 'detections' array\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        log_json_item(
            b"API Detection: Full JSON response\0".as_ptr() as *const ::core::ffi::c_char,
            root,
            r#true != 0,
        );
        cJSON_Delete(root);
        return -1 as ::core::ffi::c_int;
    }
    let mut array_size: ::core::ffi::c_int = cJSON_GetArraySize(detections);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < array_size {
        if (*result).count >= MAX_DETECTIONS {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"API Detection: Maximum number of detections reached (%d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                20 as ::core::ffi::c_int,
            );
            break;
        } else {
            let mut detection: *const cJSON = cJSON_GetArrayItem(detections, i);
            if !(detection.is_null() || cJSON_IsObject(detection) == 0) {
                let mut label: *const cJSON = cJSON_GetObjectItem(
                    detection,
                    b"label\0".as_ptr() as *const ::core::ffi::c_char,
                );
                let mut confidence: ::core::ffi::c_double = 0.0f64;
                let mut x_min: ::core::ffi::c_double = 0.0f64;
                let mut y_min: ::core::ffi::c_double = 0.0f64;
                let mut x_max: ::core::ffi::c_double = 0.0f64;
                let mut y_max: ::core::ffi::c_double = 0.0f64;
                if label.is_null()
                    || cJSON_IsString(label) == 0
                    || !number_item(
                        detection,
                        b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut confidence,
                    )
                    || !extract_box(
                        detection,
                        &raw mut x_min,
                        &raw mut y_min,
                        &raw mut x_max,
                        &raw mut y_max,
                    )
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"API Detection: Invalid detection data in JSON response\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    log_json_item(
                        b"API Detection: Detection JSON\0".as_ptr() as *const ::core::ffi::c_char,
                        detection,
                        r#false != 0,
                    );
                } else {
                    if doods2 {
                        confidence /= 100.0f64;
                    }
                    if confidence < 0.0f64 {
                        confidence = 0.0f64;
                    } else if confidence > 1.0f64 {
                        confidence = 1.0f64;
                    }
                    let mut entry: *mut detection_t = (&raw mut (*result).detections
                        as *mut detection_t)
                        .offset((*result).count as isize);
                    safe_strcpy(
                        &raw mut (*entry).label as *mut ::core::ffi::c_char,
                        (*label).valuestring,
                        MAX_LABEL_LENGTH as size_t,
                        0 as size_t,
                    );
                    (*entry).confidence = confidence as ::core::ffi::c_float;
                    (*entry).x = x_min as ::core::ffi::c_float;
                    (*entry).y = y_min as ::core::ffi::c_float;
                    (*entry).width = (x_max - x_min) as ::core::ffi::c_float;
                    (*entry).height = (y_max - y_min) as ::core::ffi::c_float;
                    let mut track_id: *const cJSON = cJSON_GetObjectItem(
                        detection,
                        b"track_id\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    (*entry).track_id = if !track_id.is_null() && cJSON_IsNumber(track_id) != 0 {
                        (*track_id).valuedouble as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    };
                    let mut zone_id: *const cJSON = cJSON_GetObjectItem(
                        detection,
                        b"zone_id\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if !zone_id.is_null() && cJSON_IsString(zone_id) != 0 {
                        safe_strcpy(
                            &raw mut (*entry).zone_id as *mut ::core::ffi::c_char,
                            (*zone_id).valuestring,
                            MAX_ZONE_ID_LENGTH as size_t,
                            0 as size_t,
                        );
                    } else {
                        (*entry).zone_id[0usize] = '\0' as ::core::ffi::c_char;
                    }
                    (*result).count += 1;
                }
            }
            i += 1;
        }
    }
    cJSON_Delete(root);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_detection_apply_response_geometry(
    mut meta: *const api_detection_response_meta_t,
    mut image_w: ::core::ffi::c_int,
    mut image_h: ::core::ffi::c_int,
    mut source_w: ::core::ffi::c_int,
    mut source_h: ::core::ffi::c_int,
    mut result: *mut detection_result_t,
    mut transform: *mut observation_evidence_transform_t,
) -> ::core::ffi::c_int {
    if image_w <= 0 as ::core::ffi::c_int || image_h <= 0 as ::core::ffi::c_int || result.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    if source_w <= 0 as ::core::ffi::c_int || source_h <= 0 as ::core::ffi::c_int {
        source_w = image_w;
        source_h = image_h;
    }
    let mut t: observation_evidence_transform_t = observation_evidence_transform_t {
        mode: observation_transform_mode_t::OBS_TRANSFORM_NONE,
        source_width: 0,
        source_height: 0,
        inference_width: 0,
        inference_height: 0,
        scale_x: 0.,
        scale_y: 0.,
        pad_x: 0.,
        pad_y: 0.,
    };
    memset(
        &raw mut t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_evidence_transform_t>(),
    );
    t.mode = observation_transform_mode_t::OBS_TRANSFORM_NONE;
    t.source_width = source_w;
    t.source_height = source_h;
    t.inference_width = image_w;
    t.inference_height = image_h;
    t.scale_y = 1.0f64;
    t.scale_x = t.scale_y;
    if !meta.is_null()
        && (*meta).letterbox as ::core::ffi::c_int != 0
        && (*meta).inference_width > 0 as ::core::ffi::c_int
        && (*meta).inference_height > 0 as ::core::ffi::c_int
    {
        let iw: ::core::ffi::c_double = (*meta).inference_width as ::core::ffi::c_double;
        let ih: ::core::ffi::c_double = (*meta).inference_height as ::core::ffi::c_double;
        let mut sx: ::core::ffi::c_double = (*meta).scale_x;
        let mut sy: ::core::ffi::c_double = (*meta).scale_y;
        if !(sx > 0.0f64) || !(sy > 0.0f64) {
            let mut uniform: ::core::ffi::c_double = if (iw / image_w as ::core::ffi::c_double)
                < ih / image_h as ::core::ffi::c_double
            {
                iw / image_w as ::core::ffi::c_double
            } else {
                ih / image_h as ::core::ffi::c_double
            };
            if !(sx > 0.0f64) {
                sx = uniform;
            }
            if !(sy > 0.0f64) {
                sy = uniform;
            }
        }
        let mut px: ::core::ffi::c_double = (*meta).pad_x;
        let mut py: ::core::ffi::c_double = (*meta).pad_y;
        if px < 0.0f64 {
            px = floor((iw - round(image_w as ::core::ffi::c_double * sx)) / 2.0f64);
        }
        if py < 0.0f64 {
            py = floor((ih - round(image_h as ::core::ffi::c_double * sy)) / 2.0f64);
        }
        if px < 0.0f64 {
            px = 0.0f64;
        }
        if py < 0.0f64 {
            py = 0.0f64;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*result).count {
            let mut d: *mut detection_t =
                (&raw mut (*result).detections as *mut detection_t).offset(i as isize);
            let mut u: ::core::ffi::c_double = (*d).x as ::core::ffi::c_double;
            let mut v: ::core::ffi::c_double = (*d).y as ::core::ffi::c_double;
            let mut du: ::core::ffi::c_double = (*d).width as ::core::ffi::c_double;
            let mut dv: ::core::ffi::c_double = (*d).height as ::core::ffi::c_double;
            if !(*meta).pixel_units {
                u *= iw;
                du *= iw;
                v *= ih;
                dv *= ih;
            }
            (*d).x = ((u - px) / sx / image_w as ::core::ffi::c_double) as ::core::ffi::c_float;
            (*d).y = ((v - py) / sy / image_h as ::core::ffi::c_double) as ::core::ffi::c_float;
            (*d).width = (du / sx / image_w as ::core::ffi::c_double) as ::core::ffi::c_float;
            (*d).height = (dv / sy / image_h as ::core::ffi::c_double) as ::core::ffi::c_float;
            i += 1;
        }
        t.mode = observation_transform_mode_t::OBS_TRANSFORM_LETTERBOX;
        t.inference_width = (*meta).inference_width;
        t.inference_height = (*meta).inference_height;
        t.scale_x = sx;
        t.scale_y = sy;
        t.pad_x = px;
        t.pad_y = py;
    }
    if !transform.is_null() {
        *transform = t;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_detection_jpeg_dimensions(
    mut jpeg: *const uint8_t,
    mut size: size_t,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if jpeg.is_null()
        || size < 4 as size_t
        || width.is_null()
        || height.is_null()
        || *jpeg.offset(0isize) as ::core::ffi::c_int != 0xff as ::core::ffi::c_int
        || *jpeg.offset(1isize) as ::core::ffi::c_int != 0xd8 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut pos: size_t = 2 as size_t;
    while pos.wrapping_add(4 as size_t) <= size {
        if *jpeg.offset(pos as isize) as ::core::ffi::c_int != 0xff as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        let mut marker: uint8_t = *jpeg.offset(pos.wrapping_add(1 as size_t) as isize);
        if marker as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
            pos = pos.wrapping_add(1);
        } else if marker as ::core::ffi::c_int == 0xd8 as ::core::ffi::c_int
            || marker as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int
            || marker as ::core::ffi::c_int >= 0xd0 as ::core::ffi::c_int
                && marker as ::core::ffi::c_int <= 0xd7 as ::core::ffi::c_int
        {
            pos = pos.wrapping_add(2 as size_t);
        } else {
            if marker as ::core::ffi::c_int == 0xd9 as ::core::ffi::c_int
                || marker as ::core::ffi::c_int == 0xda as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            let mut seg: size_t = (*jpeg.offset(pos.wrapping_add(2 as size_t) as isize) as size_t)
                << 8 as ::core::ffi::c_int
                | *jpeg.offset(pos.wrapping_add(3 as size_t) as isize) as size_t;
            if seg < 2 as size_t || pos.wrapping_add(2 as size_t).wrapping_add(seg) > size {
                return -1 as ::core::ffi::c_int;
            }
            let mut sof: bool = marker as ::core::ffi::c_int >= 0xc0 as ::core::ffi::c_int
                && marker as ::core::ffi::c_int <= 0xcf as ::core::ffi::c_int
                && marker as ::core::ffi::c_int != 0xc4 as ::core::ffi::c_int
                && marker as ::core::ffi::c_int != 0xc8 as ::core::ffi::c_int
                && marker as ::core::ffi::c_int != 0xcc as ::core::ffi::c_int;
            if sof {
                if seg < 7 as size_t {
                    return -1 as ::core::ffi::c_int;
                }
                let mut h: ::core::ffi::c_int = (*jpeg
                    .offset(pos.wrapping_add(5 as size_t) as isize)
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *jpeg.offset(pos.wrapping_add(6 as size_t) as isize) as ::core::ffi::c_int;
                let mut w: ::core::ffi::c_int = (*jpeg
                    .offset(pos.wrapping_add(7 as size_t) as isize)
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *jpeg.offset(pos.wrapping_add(8 as size_t) as isize) as ::core::ffi::c_int;
                if w <= 0 as ::core::ffi::c_int || h <= 0 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                *width = w;
                *height = h;
                return 0 as ::core::ffi::c_int;
            }
            pos = pos.wrapping_add((2 as size_t).wrapping_add(seg));
        }
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn write_memory_callback(
    mut contents: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> size_t {
    if nmemb != 0 as size_t && size > (SIZE_MAX as size_t).wrapping_div(nmemb) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Not enough memory for curl response (size overflow)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as size_t;
    }
    let mut realsize: size_t = size.wrapping_mul(nmemb);
    let mut mem: *mut memory_struct_t = userp as *mut memory_struct_t;
    if realsize
        > (SIZE_MAX as size_t)
            .wrapping_sub((*mem).size)
            .wrapping_sub(1 as size_t)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Not enough memory for curl response (size overflow)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as size_t;
    }
    let mut required_size: size_t = (*mem).size.wrapping_add(realsize).wrapping_add(1 as size_t);
    if required_size > (*mem).capacity {
        let mut new_capacity: size_t = if (*mem).capacity > 0 as size_t {
            (*mem).capacity
        } else {
            API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE as size_t
        };
        while new_capacity < required_size {
            if new_capacity > (SIZE_MAX as size_t).wrapping_div(2 as size_t) {
                new_capacity = required_size;
                break;
            } else {
                new_capacity = new_capacity.wrapping_mul(2 as size_t);
            }
        }
        let mut new_memory: *mut ::core::ffi::c_char =
            realloc((*mem).memory as *mut ::core::ffi::c_void, new_capacity)
                as *mut ::core::ffi::c_char;
        if new_memory.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Not enough memory for curl response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 0 as size_t;
        }
        (*mem).memory = new_memory;
        (*mem).capacity = new_capacity;
    }
    memcpy(
        (*mem).memory.offset((*mem).size as isize) as *mut ::core::ffi::c_void,
        contents,
        realsize,
    );
    (*mem).size = (*mem).size.wrapping_add(realsize);
    *(*mem).memory.offset((*mem).size as isize) = 0 as ::core::ffi::c_char;
    return realsize;
}
unsafe extern "C" fn is_tls_ca_error(mut res: CURLcode) -> bool {
    if res.0 == CURLcode::CURLE_PEER_FAILED_VERIFICATION.0 {
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn log_tls_error_details(
    mut context: *const ::core::ffi::c_char,
    mut curl: *mut ::core::ffi::c_void,
    mut res: CURLcode,
    mut url: *const ::core::ffi::c_char,
) {
    if !is_tls_ca_error(res) {
        return;
    }
    let mut ssl_verify_result: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    if !curl.is_null() {
        curl_easy_getinfo(
            curl,
            CURLINFO::CURLINFO_SSL_VERIFYRESULT,
            &raw mut ssl_verify_result,
        );
    }
    let mut ssl_cert_file: *const ::core::ffi::c_char =
        getenv(b"SSL_CERT_FILE\0".as_ptr() as *const ::core::ffi::c_char);
    let mut ssl_cert_dir: *const ::core::ffi::c_char =
        getenv(b"SSL_CERT_DIR\0".as_ptr() as *const ::core::ffi::c_char);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"%s: TLS certificate verification failed for %s\0".as_ptr() as *const ::core::ffi::c_char,
        context,
        if !url.is_null() {
            url
        } else {
            b"(unknown URL)\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"%s: libcurl SSL verify result=%ld, SSL_CERT_FILE=%s, SSL_CERT_DIR=%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        context,
        ssl_verify_result,
        if !ssl_cert_file.is_null()
            && *ssl_cert_file.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            ssl_cert_file
        } else {
            b"(unset)\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !ssl_cert_dir.is_null()
            && *ssl_cert_dir.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            ssl_cert_dir
        } else {
            b"(unset)\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"%s: Ensure a readable CA bundle is installed (for containers, install ca-certificates) or configure SSL_CERT_FILE/SSL_CERT_DIR to valid paths.\0"
            .as_ptr() as *const ::core::ffi::c_char,
        context,
    );
}
unsafe extern "C" fn make_response_preview(
    mut chunk: *const memory_struct_t,
    mut preview: *mut ::core::ffi::c_char,
    mut preview_size: size_t,
) {
    let mut copy_len: size_t = 0 as size_t;
    if !(*chunk).memory.is_null() && preview_size > 0 as size_t {
        copy_len = if (*chunk).size < preview_size.wrapping_sub(1 as size_t) {
            (*chunk).size
        } else {
            preview_size.wrapping_sub(1 as size_t)
        };
        memcpy(
            preview as *mut ::core::ffi::c_void,
            (*chunk).memory as *const ::core::ffi::c_void,
            copy_len,
        );
    }
    if preview_size > 0 as size_t {
        *preview.offset(copy_len as isize) = '\0' as ::core::ffi::c_char;
    }
    let mut i: size_t = 0 as size_t;
    while i < copy_len {
        if (*preview.offset(i as isize) as ::core::ffi::c_int) < ASCII_PRINTABLE_MIN
            || *preview.offset(i as isize) as ::core::ffi::c_int > ASCII_PRINTABLE_MAX
        {
            *preview.offset(i as isize) = '.' as ::core::ffi::c_char;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn build_multipart_body(
    mut curl: *mut ::core::ffi::c_void,
    mut jpeg_data: *const ::core::ffi::c_uchar,
    mut jpeg_size: size_t,
    mut context: *const ::core::ffi::c_char,
) -> *mut curl_mime {
    let mut mime: *mut curl_mime = curl_mime_init(curl);
    if mime.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Failed to create mime structure\0".as_ptr() as *const ::core::ffi::c_char,
            context,
        );
        return ::core::ptr::null_mut::<curl_mime>();
    }
    let mut part: *mut curl_mimepart = curl_mime_addpart(mime);
    if part.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Failed to add mime part\0".as_ptr() as *const ::core::ffi::c_char,
            context,
        );
        curl_mime_free(mime);
        return ::core::ptr::null_mut::<curl_mime>();
    }
    let mut rc: CURLcode = CURLcode::CURLE_OK;
    rc = curl_mime_name(part, b"file\0".as_ptr() as *const ::core::ffi::c_char);
    if rc.0 != CURLcode::CURLE_OK.0
        || {
            rc = curl_mime_data(part, jpeg_data as *const ::core::ffi::c_char, jpeg_size);
            rc.0 != CURLcode::CURLE_OK.0
        }
        || {
            rc = curl_mime_filename(
                part,
                b"snapshot.jpg\0".as_ptr() as *const ::core::ffi::c_char,
            );
            rc.0 != CURLcode::CURLE_OK.0
        }
        || {
            rc = curl_mime_type(part, b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char);
            rc.0 != CURLcode::CURLE_OK.0
        }
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Failed to build multipart body: %s\0".as_ptr() as *const ::core::ffi::c_char,
            context,
            curl_easy_strerror(rc),
        );
        curl_mime_free(mime);
        return ::core::ptr::null_mut::<curl_mime>();
    }
    return mime;
}
unsafe extern "C" fn perform_detection_request(
    mut context: *const ::core::ffi::c_char,
    mut base_url: *const ::core::ffi::c_char,
    mut jpeg_data: *const ::core::ffi::c_uchar,
    mut jpeg_size: size_t,
    mut threshold: ::core::ffi::c_float,
    mut request_id: *const ::core::ffi::c_char,
    mut options: *const api_detection_options_t,
    mut result: *mut detection_result_t,
    mut meta: *mut api_detection_response_meta_t,
) -> ::core::ffi::c_int {
    let mut res: CURLcode = CURLcode::CURLE_OK;
    let mut http_code: ::core::ffi::c_long = 0;
    let mut curl: *mut ::core::ffi::c_void = NULL;
    let mut mime: *mut curl_mime = ::core::ptr::null_mut::<curl_mime>();
    let mut headers: *mut curl_slist = ::core::ptr::null_mut::<curl_slist>();
    let mut body: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut chunk: memory_struct_t = memory_struct_t {
        memory: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
        capacity: 0,
    };
    let mut request_url: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut preview: [::core::ffi::c_char; 64] = [0; 64];
    let mut ret: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if api_detection_build_request_url(
        &raw mut request_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        base_url,
        threshold,
        options,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Failed to construct request URL for %s\0".as_ptr() as *const ::core::ffi::c_char,
            context,
            base_url,
        );
        return -1 as ::core::ffi::c_int;
    }
    curl = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Failed to initialize CURL handle\0".as_ptr() as *const ::core::ffi::c_char,
            context,
        );
        return -1 as ::core::ffi::c_int;
    }
    '_cleanup: {
        if (*options).format.0 == api_detection_format_t::API_DETECTION_FORMAT_DOODS2.0 {
            body = api_detection_build_doods2_body(
                jpeg_data, jpeg_size, threshold, request_id, options,
            );
            if body.is_null() {
                break '_cleanup;
            } else {
                headers = curl_slist_append(
                    headers,
                    b"Content-Type: application/json\0".as_ptr() as *const ::core::ffi::c_char,
                );
                curl_easy_setopt(curl, CURLoption::CURLOPT_POSTFIELDS, body);
                curl_easy_setopt(
                    curl,
                    CURLoption::CURLOPT_POSTFIELDSIZE_LARGE,
                    strlen(body) as curl_off_t,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"%s: Sending DOODS2 request to %s (detector: %s, threshold: %.2f, %zu byte JPEG)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    context,
                    &raw mut request_url as *mut ::core::ffi::c_char,
                    effective_detector_name(options),
                    normalize_api_detection_threshold(threshold)
                        as ::core::ffi::c_double,
                    jpeg_size,
                );
            }
        } else {
            mime = build_multipart_body(curl, jpeg_data, jpeg_size, context);
            if mime.is_null() {
                break '_cleanup;
            } else {
                curl_easy_setopt(curl, CURLoption::CURLOPT_MIMEPOST, mime);
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"%s: Sending request to %s (backend: %s, threshold: %.2f, %zu byte JPEG)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    context,
                    &raw mut request_url as *mut ::core::ffi::c_char,
                    sanitize_backend(&raw const (*options).backend as *const ::core::ffi::c_char),
                    normalize_api_detection_threshold(threshold) as ::core::ffi::c_double,
                    jpeg_size,
                );
            }
        }
        headers = curl_slist_append(
            headers,
            b"accept: application/json\0".as_ptr() as *const ::core::ffi::c_char,
        );
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_URL,
            &raw mut request_url as *mut ::core::ffi::c_char,
        );
        curl_easy_setopt(curl, CURLoption::CURLOPT_HTTPHEADER, headers);
        chunk.memory = malloc(API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE as size_t)
            as *mut ::core::ffi::c_char;
        if chunk.memory.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"%s: Failed to allocate memory for curl response buffer\0".as_ptr()
                    as *const ::core::ffi::c_char,
                context,
            );
        } else {
            chunk.size = 0 as size_t;
            chunk.capacity = API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE as size_t;
            *chunk.memory.offset(0isize) = '\0' as ::core::ffi::c_char;
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
            curl_easy_setopt(curl, CURLoption::CURLOPT_TIMEOUT, 10 as ::core::ffi::c_long);
            setup_common_curl_options(curl);
            res = curl_easy_perform(curl);
            if res.0 != CURLcode::CURLE_OK.0 {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"%s: curl_easy_perform() failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    context,
                    curl_easy_strerror(res),
                );
                log_tls_error_details(
                    context,
                    curl,
                    res,
                    &raw mut request_url as *mut ::core::ffi::c_char,
                );
                if res.0 == CURLcode::CURLE_COULDNT_CONNECT.0 {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"%s: Could not connect to server at %s. Is the API server running?\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        context,
                        &raw mut request_url as *mut ::core::ffi::c_char,
                    );
                } else if res.0 == CURLcode::CURLE_OPERATION_TIMEDOUT.0 {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"%s: Connection to %s timed out. Server might be slow or unreachable.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        context,
                        &raw mut request_url as *mut ::core::ffi::c_char,
                    );
                } else if res.0 == CURLcode::CURLE_COULDNT_RESOLVE_HOST.0 {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"%s: Could not resolve host %s. Check your network connection and DNS settings.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        context,
                        &raw mut request_url as *mut ::core::ffi::c_char,
                    );
                }
            } else {
                http_code = 0 as ::core::ffi::c_long;
                curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
                if http_code != 200 as ::core::ffi::c_long {
                    make_response_preview(
                        &raw mut chunk,
                        &raw mut preview as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    );
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"%s: API request failed with HTTP code %ld (format: %s, response: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        context,
                        http_code,
                        api_detection_format_name((*options).format),
                        &raw mut preview as *mut ::core::ffi::c_char,
                    );
                } else if chunk.memory.is_null() || chunk.size == 0 as size_t {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"%s: Empty response from server\0".as_ptr() as *const ::core::ffi::c_char,
                        context,
                    );
                } else {
                    make_response_preview(
                        &raw mut chunk,
                        &raw mut preview as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    );
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"%s: Response preview: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        context,
                        &raw mut preview as *mut ::core::ffi::c_char,
                    );
                    ret = api_detection_parse_response_meta(chunk.memory, options, result, meta);
                    if ret != 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"%s: Response size: %zu bytes\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            context,
                            chunk.size,
                        );
                    }
                }
            }
        }
    }
    free(body as *mut ::core::ffi::c_void);
    free(chunk.memory as *mut ::core::ffi::c_void);
    curl_mime_free(mime);
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    return ret;
}
unsafe extern "C" fn has_camera_uuid(mut evidence: *const api_detection_evidence_t) -> bool {
    return !evidence.is_null()
        && !(*evidence).camera_uuid.is_null()
        && *(*evidence).camera_uuid.offset(0isize) as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int;
}
unsafe extern "C" fn apply_submitted_geometry(
    mut context: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut meta: *const api_detection_response_meta_t,
    mut source_w: ::core::ffi::c_int,
    mut source_h: ::core::ffi::c_int,
    mut result: *mut detection_result_t,
    mut image: *mut submitted_image_t,
) {
    if (*image).width <= 0 as ::core::ffi::c_int || (*image).height <= 0 as ::core::ffi::c_int {
        if api_detection_jpeg_dimensions(
            (*image).jpeg,
            (*image).size,
            &raw mut (*image).width,
            &raw mut (*image).height,
        ) != 0 as ::core::ffi::c_int
        {
            (*image).height = 0 as ::core::ffi::c_int;
            (*image).width = (*image).height;
        }
    }
    if (*image).width <= 0 as ::core::ffi::c_int || (*image).height <= 0 as ::core::ffi::c_int {
        if (*meta).letterbox {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"%s: Cannot map the declared letterbox for %s: submitted JPEG size unknown\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                context,
                if !stream_name.is_null() {
                    stream_name
                } else {
                    b"(none)\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        return;
    }
    api_detection_apply_response_geometry(
        meta,
        (*image).width,
        (*image).height,
        source_w,
        source_h,
        result,
        &raw mut (*image).transform,
    );
}
unsafe extern "C" fn submit_api_evidence(
    mut context: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
    mut threshold: ::core::ffi::c_float,
    mut recording_id: uint64_t,
    mut options: *const api_detection_options_t,
    mut evidence: *const api_detection_evidence_t,
    mut meta: *const api_detection_response_meta_t,
    mut image: *mut submitted_image_t,
) {
    let mut jpeg: *mut uint8_t = (*image).jpeg;
    (*image).jpeg = ::core::ptr::null_mut::<uint8_t>();
    if !has_camera_uuid(evidence)
        || !(*evidence).evidence_enabled
        || (*result).count <= 0 as ::core::ffi::c_int
        || jpeg.is_null()
        || (*image).width <= 0 as ::core::ffi::c_int
        || (*image).height <= 0 as ::core::ffi::c_int
    {
        free(jpeg as *mut ::core::ffi::c_void);
        return;
    }
    let mut survivors: [detection_t; 20] = [detection_t {
        label: [0; 32],
        confidence: 0.,
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
        track_id: 0,
        zone_id: [0; 32],
    }; 20];
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*result).count && n < MAX_DETECTIONS {
        if !(threshold > 0.0f32 && (*result).detections[i as usize].confidence < threshold) {
            let c2rust_fresh0 = n;
            n += 1;
            survivors[c2rust_fresh0 as usize] = (*result).detections[i as usize];
        }
        i += 1;
    }
    if n == 0 as ::core::ffi::c_int {
        free(jpeg as *mut ::core::ffi::c_void);
        return;
    }
    let mut model: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if (*meta).model[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        model = &raw const (*meta).model as *const ::core::ffi::c_char;
    } else if (*options).format.0 == api_detection_format_t::API_DETECTION_FORMAT_DOODS2.0
        && (*options).detector_name[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && strcmp(
            &raw const (*options).detector_name as *const ::core::ffi::c_char,
            b"default\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        model = &raw const (*options).detector_name as *const ::core::ffi::c_char;
    }
    let mut camera: detection_evidence_camera_t = detection_evidence_camera_t {
        camera_uuid: (*evidence).camera_uuid,
        stream_name: stream_name,
        evidence_enabled: r#true != 0,
        labels_csv: (*evidence).labels_csv,
        recording_id: recording_id as int64_t,
    };
    let mut engine: detection_evidence_engine_t = detection_evidence_engine_t {
        engine_key: (*evidence).engine_key,
        engine_type: b"api\0".as_ptr() as *const ::core::ffi::c_char,
        model_path: model,
        model_version: if (*meta).model_version[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*meta).model_version as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
    };
    let mut submitted: observation_evidence_result_t =
        observation_evidence_result_t::OBS_EVIDENCE_INVALID;
    let mut status: detection_evidence_status_t = detection_evidence_capture_jpeg(
        &raw mut camera,
        &raw mut engine,
        &raw mut survivors as *mut detection_t,
        n,
        (*evidence).observed_at_ms,
        jpeg,
        (*image).size,
        (*image).width,
        (*image).height,
        &raw mut (*image).transform,
        &raw mut submitted,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    if status.0 == detection_evidence_status_t::DETECTION_EVIDENCE_SUBMITTED.0
        && submitted.0 != observation_evidence_result_t::OBS_EVIDENCE_ACCEPTED.0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Observation evidence for %s not queued (%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
            stream_name,
            submitted.0,
        );
    }
}
unsafe extern "C" fn finalize_detections(
    mut context: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *mut detection_result_t,
    mut frame_timestamp: time_t,
    mut recording_id: uint64_t,
    mut threshold: ::core::ffi::c_float,
    mut options: *const api_detection_options_t,
    mut evidence: *const api_detection_evidence_t,
    mut meta: *const api_detection_response_meta_t,
    mut image: *mut submitted_image_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: No stream name provided, skipping database storage\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
        );
        free((*image).jpeg as *mut ::core::ffi::c_void);
        (*image).jpeg = ::core::ptr::null_mut::<uint8_t>();
        return 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"%s: Filtering %d detections by zones for stream %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        context,
        (*result).count,
        stream_name,
    );
    if filter_detections_by_zones(stream_name, result) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Failed to filter detections by zones for stream %s, aborting detection pipeline for this frame\0"
                .as_ptr() as *const ::core::ffi::c_char,
            context,
            stream_name,
        );
        free((*image).jpeg as *mut ::core::ffi::c_void);
        (*image).jpeg = ::core::ptr::null_mut::<uint8_t>();
        return -1 as ::core::ffi::c_int;
    }
    filter_detections_by_stream_objects(stream_name, result);
    let mut timestamp: time_t = if frame_timestamp != 0 as time_t {
        frame_timestamp
    } else {
        time(::core::ptr::null_mut::<time_t>())
    };
    let by_camera: bool = has_camera_uuid(evidence);
    if by_camera {
        store_detections_in_db_for_camera(
            (*evidence).camera_uuid,
            stream_name,
            result,
            timestamp,
            recording_id,
        );
    } else {
        store_detections_in_db(stream_name, result, timestamp, recording_id);
    }
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
        let mut rc: ::core::ffi::c_int = if by_camera as ::core::ffi::c_int != 0 {
            event_producer_publish_detection(
                (*evidence).camera_uuid,
                stream_name,
                result,
                timestamp,
                &raw mut event_error as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            )
        } else {
            event_producer_publish_detection_for_stream(
                stream_name,
                result,
                timestamp,
                &raw mut event_error as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            )
        };
        if rc != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"%s: Event enqueue failed for %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                context,
                stream_name,
                &raw mut event_error as *mut ::core::ffi::c_char,
            );
        }
    }
    submit_api_evidence(
        context,
        stream_name,
        result,
        threshold,
        recording_id,
        options,
        evidence,
        meta,
        image,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn stamp_evidence(
    mut r#in: *const api_detection_evidence_t,
    mut storage: *mut api_detection_evidence_t,
    mut frame_timestamp: time_t,
) -> *const api_detection_evidence_t {
    if r#in.is_null() {
        return ::core::ptr::null::<api_detection_evidence_t>();
    }
    *storage = *r#in;
    if (*storage).observed_at_ms <= 0 as int64_t {
        (*storage).observed_at_ms = detection_evidence_observed_at_ms(frame_timestamp);
    }
    return storage;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_api_detection_system() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut curl_mutex);
    if initialized {
        pthread_mutex_unlock(&raw mut curl_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"API detection system already initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if curl_init_global() != 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut curl_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize curl global\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    initialized = r#true != 0;
    pthread_mutex_unlock(&raw mut curl_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"API detection system initialized successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_api_detection_system() {
    let mut was_initialized: bool = r#false != 0;
    pthread_mutex_lock(&raw mut curl_mutex);
    was_initialized = initialized;
    initialized = r#false != 0;
    pthread_mutex_unlock(&raw mut curl_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Shutting down API detection system (initialized: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        if was_initialized as ::core::ffi::c_int != 0 {
            b"yes\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"no\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    jpeg_encoder_cleanup_all();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"API detection system shutdown complete\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_objects_api_with_options(
    mut api_url: *const ::core::ffi::c_char,
    mut frame_data: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut channels: ::core::ffi::c_int,
    mut result: *mut detection_result_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut threshold: ::core::ffi::c_float,
    mut recording_id: uint64_t,
    mut frame_timestamp: time_t,
    mut options: *const api_detection_options_t,
) -> ::core::ffi::c_int {
    return detect_objects_api_with_evidence(
        api_url,
        frame_data,
        width,
        height,
        channels,
        result,
        stream_name,
        threshold,
        recording_id,
        frame_timestamp,
        options,
        ::core::ptr::null::<api_detection_evidence_t>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_objects_api_with_evidence(
    mut api_url: *const ::core::ffi::c_char,
    mut frame_data: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut channels: ::core::ffi::c_int,
    mut result: *mut detection_result_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut threshold: ::core::ffi::c_float,
    mut recording_id: uint64_t,
    mut frame_timestamp: time_t,
    mut options: *const api_detection_options_t,
    mut evidence_in: *const api_detection_evidence_t,
) -> ::core::ffi::c_int {
    static mut context: *const ::core::ffi::c_char =
        b"API Detection\0".as_ptr() as *const ::core::ffi::c_char;
    let mut evidence_storage: api_detection_evidence_t = api_detection_evidence_t {
        camera_uuid: ::core::ptr::null::<::core::ffi::c_char>(),
        engine_key: ::core::ptr::null::<::core::ffi::c_char>(),
        evidence_enabled: false,
        labels_csv: ::core::ptr::null::<::core::ffi::c_char>(),
        observed_at_ms: 0,
    };
    let mut evidence: *const api_detection_evidence_t =
        stamp_evidence(evidence_in, &raw mut evidence_storage, frame_timestamp);
    if is_shutdown_initiated() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: System shutdown in progress, skipping detection\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
        );
        return -1 as ::core::ffi::c_int;
    }
    if result.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: NULL result pointer provided\0".as_ptr() as *const ::core::ffi::c_char,
            context,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    let mut actual_api_url: *const ::core::ffi::c_char = resolve_api_url(api_url, context);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"%s: Starting detection with API URL: %s\0".as_ptr() as *const ::core::ffi::c_char,
        context,
        if !actual_api_url.is_null() {
            actual_api_url
        } else {
            b"NULL\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"%s: Stream name: %s\0".as_ptr() as *const ::core::ffi::c_char,
        context,
        if !stream_name.is_null() {
            stream_name
        } else {
            b"NULL\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if !is_api_detection_system_initialized() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"API detection system not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !validate_api_detection_base_url(actual_api_url, context) {
        return -1 as ::core::ffi::c_int;
    }
    let mut storage: api_detection_options_t = api_detection_options_t {
        format: api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT,
        backend: [0; 32],
        detector_name: [0; 64],
    };
    options = resolve_options(options, &raw mut storage);
    let mut jpeg_data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut jpeg_size: size_t = 0 as size_t;
    let mut go2rtc_initialized: bool = r#false != 0;
    let mut snapshot_ok: bool = r#false != 0;
    if api_detection_should_use_go2rtc_snapshot(frame_data, width, height, channels, stream_name) {
        go2rtc_initialized = go2rtc_integration_is_initialized();
        if go2rtc_initialized {
            snapshot_ok = go2rtc_get_snapshot(stream_name, &raw mut jpeg_data, &raw mut jpeg_size);
        }
    }
    if snapshot_ok {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Successfully fetched snapshot from go2rtc: %zu bytes\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
            jpeg_size,
        );
    } else {
        if stream_name.is_null()
            || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"%s: No stream name provided for go2rtc snapshot, using cached JPEG encoding\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                context,
            );
        } else if !go2rtc_initialized {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"%s: go2rtc not initialized, using cached JPEG encoding\0".as_ptr()
                    as *const ::core::ffi::c_char,
                context,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"%s: Failed to get snapshot from go2rtc, falling back to cached JPEG encoding\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                context,
            );
        }
        let mut encoder: *mut jpeg_encoder_cache_t =
            jpeg_encoder_get_cached(width, height, channels, API_DETECTION_JPEG_QUALITY_DEFAULT);
        if encoder.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"%s: Failed to get cached JPEG encoder\0".as_ptr() as *const ::core::ffi::c_char,
                context,
            );
            return -1 as ::core::ffi::c_int;
        }
        if jpeg_encoder_cache_encode_to_memory(
            encoder,
            frame_data,
            &raw mut jpeg_data,
            &raw mut jpeg_size,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"%s: Failed to encode frame to JPEG using cached encoder\0".as_ptr()
                    as *const ::core::ffi::c_char,
                context,
            );
            return -1 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Encoded frame to JPEG using cached encoder: %zu bytes\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
            jpeg_size,
        );
    }
    if jpeg_data.is_null() || jpeg_size == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: No JPEG data available\0".as_ptr() as *const ::core::ffi::c_char,
            context,
        );
        free(jpeg_data as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut meta: api_detection_response_meta_t = api_detection_response_meta_t {
        letterbox: false,
        inference_width: 0,
        inference_height: 0,
        scale_x: 0.,
        scale_y: 0.,
        pad_x: 0.,
        pad_y: 0.,
        pixel_units: false,
        model: [0; 128],
        model_version: [0; 64],
    };
    let mut ret: ::core::ffi::c_int = perform_detection_request(
        context,
        actual_api_url,
        jpeg_data,
        jpeg_size,
        threshold,
        stream_name,
        options,
        result,
        &raw mut meta,
    );
    if ret == 0 as ::core::ffi::c_int {
        let mut image: submitted_image_t = submitted_image_t {
            jpeg: jpeg_data as *mut uint8_t,
            size: jpeg_size,
            width: if snapshot_ok as ::core::ffi::c_int != 0 {
                0 as ::core::ffi::c_int
            } else {
                width
            },
            height: if snapshot_ok as ::core::ffi::c_int != 0 {
                0 as ::core::ffi::c_int
            } else {
                height
            },
            transform: observation_evidence_transform_t {
                mode: observation_transform_mode_t::OBS_TRANSFORM_NONE,
                source_width: 0,
                source_height: 0,
                inference_width: 0,
                inference_height: 0,
                scale_x: 0.,
                scale_y: 0.,
                pad_x: 0.,
                pad_y: 0.,
            },
        };
        jpeg_data = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        apply_submitted_geometry(
            context,
            stream_name,
            &raw mut meta,
            if snapshot_ok as ::core::ffi::c_int != 0 {
                0 as ::core::ffi::c_int
            } else {
                width
            },
            if snapshot_ok as ::core::ffi::c_int != 0 {
                0 as ::core::ffi::c_int
            } else {
                height
            },
            result,
            &raw mut image,
        );
        ret = finalize_detections(
            context,
            stream_name,
            result,
            frame_timestamp,
            recording_id,
            threshold,
            options,
            evidence,
            &raw mut meta,
            &raw mut image,
        );
    }
    free(jpeg_data as *mut ::core::ffi::c_void);
    if ret != 0 as ::core::ffi::c_int {
        (*result).count = 0 as ::core::ffi::c_int;
        return ret;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"%s: Successfully detected %d objects\0".as_ptr() as *const ::core::ffi::c_char,
        context,
        (*result).count,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_objects_api(
    mut api_url: *const ::core::ffi::c_char,
    mut frame_data: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut channels: ::core::ffi::c_int,
    mut result: *mut detection_result_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut threshold: ::core::ffi::c_float,
    mut recording_id: uint64_t,
    mut frame_timestamp: time_t,
) -> ::core::ffi::c_int {
    return detect_objects_api_with_options(
        api_url,
        frame_data,
        width,
        height,
        channels,
        result,
        stream_name,
        threshold,
        recording_id,
        frame_timestamp,
        ::core::ptr::null::<api_detection_options_t>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_objects_api_snapshot_with_options(
    mut api_url: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *mut detection_result_t,
    mut threshold: ::core::ffi::c_float,
    mut recording_id: uint64_t,
    mut frame_timestamp: time_t,
    mut options: *const api_detection_options_t,
) -> ::core::ffi::c_int {
    return detect_objects_api_snapshot_with_evidence(
        api_url,
        stream_name,
        result,
        threshold,
        recording_id,
        frame_timestamp,
        options,
        ::core::ptr::null::<api_detection_evidence_t>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_objects_api_snapshot_with_evidence(
    mut api_url: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *mut detection_result_t,
    mut threshold: ::core::ffi::c_float,
    mut recording_id: uint64_t,
    mut frame_timestamp: time_t,
    mut options: *const api_detection_options_t,
    mut evidence_in: *const api_detection_evidence_t,
) -> ::core::ffi::c_int {
    static mut context: *const ::core::ffi::c_char =
        b"API Detection (snapshot)\0".as_ptr() as *const ::core::ffi::c_char;
    let mut evidence_storage: api_detection_evidence_t = api_detection_evidence_t {
        camera_uuid: ::core::ptr::null::<::core::ffi::c_char>(),
        engine_key: ::core::ptr::null::<::core::ffi::c_char>(),
        evidence_enabled: false,
        labels_csv: ::core::ptr::null::<::core::ffi::c_char>(),
        observed_at_ms: 0,
    };
    let mut evidence: *const api_detection_evidence_t =
        stamp_evidence(evidence_in, &raw mut evidence_storage, frame_timestamp);
    if is_shutdown_initiated() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: System shutdown in progress, skipping detection\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
        );
        return -1 as ::core::ffi::c_int;
    }
    if stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Stream name is required\0".as_ptr() as *const ::core::ffi::c_char,
            context,
        );
        return -1 as ::core::ffi::c_int;
    }
    if result.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: NULL result pointer provided\0".as_ptr() as *const ::core::ffi::c_char,
            context,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    let mut actual_api_url: *const ::core::ffi::c_char = resolve_api_url(api_url, context);
    if !is_api_detection_system_initialized() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"API detection system not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !validate_api_detection_base_url(actual_api_url, context) {
        return -1 as ::core::ffi::c_int;
    }
    let mut storage: api_detection_options_t = api_detection_options_t {
        format: api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT,
        backend: [0; 32],
        detector_name: [0; 64],
    };
    options = resolve_options(options, &raw mut storage);
    let mut jpeg_data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut jpeg_size: size_t = 0 as size_t;
    if !go2rtc_integration_is_initialized() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: go2rtc not initialized, skipping snapshot for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
            stream_name,
        );
        return API_DETECTION_SNAPSHOT_UNAVAILABLE;
    }
    if !go2rtc_get_snapshot(stream_name, &raw mut jpeg_data, &raw mut jpeg_size) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: Failed to get snapshot from go2rtc for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            context,
            stream_name,
        );
        return API_DETECTION_SNAPSHOT_UNAVAILABLE;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"%s: Successfully fetched snapshot from go2rtc: %zu bytes\0".as_ptr()
            as *const ::core::ffi::c_char,
        context,
        jpeg_size,
    );
    if jpeg_data.is_null() || jpeg_size == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: No JPEG data available\0".as_ptr() as *const ::core::ffi::c_char,
            context,
        );
        free(jpeg_data as *mut ::core::ffi::c_void);
        return API_DETECTION_SNAPSHOT_UNAVAILABLE;
    }
    let mut meta: api_detection_response_meta_t = api_detection_response_meta_t {
        letterbox: false,
        inference_width: 0,
        inference_height: 0,
        scale_x: 0.,
        scale_y: 0.,
        pad_x: 0.,
        pad_y: 0.,
        pixel_units: false,
        model: [0; 128],
        model_version: [0; 64],
    };
    let mut ret: ::core::ffi::c_int = perform_detection_request(
        context,
        actual_api_url,
        jpeg_data,
        jpeg_size,
        threshold,
        stream_name,
        options,
        result,
        &raw mut meta,
    );
    if ret == 0 as ::core::ffi::c_int {
        let mut image: submitted_image_t = submitted_image_t {
            jpeg: jpeg_data as *mut uint8_t,
            size: jpeg_size,
            width: 0,
            height: 0,
            transform: observation_evidence_transform_t {
                mode: observation_transform_mode_t::OBS_TRANSFORM_NONE,
                source_width: 0,
                source_height: 0,
                inference_width: 0,
                inference_height: 0,
                scale_x: 0.,
                scale_y: 0.,
                pad_x: 0.,
                pad_y: 0.,
            },
        };
        jpeg_data = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        apply_submitted_geometry(
            context,
            stream_name,
            &raw mut meta,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            result,
            &raw mut image,
        );
        ret = finalize_detections(
            context,
            stream_name,
            result,
            frame_timestamp,
            recording_id,
            threshold,
            options,
            evidence,
            &raw mut meta,
            &raw mut image,
        );
    }
    free(jpeg_data as *mut ::core::ffi::c_void);
    if ret != 0 as ::core::ffi::c_int {
        (*result).count = 0 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"%s: Successfully detected %d objects\0".as_ptr() as *const ::core::ffi::c_char,
        context,
        (*result).count,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_objects_api_snapshot(
    mut api_url: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *mut detection_result_t,
    mut threshold: ::core::ffi::c_float,
    mut recording_id: uint64_t,
    mut frame_timestamp: time_t,
) -> ::core::ffi::c_int {
    return detect_objects_api_snapshot_with_options(
        api_url,
        stream_name,
        result,
        threshold,
        recording_id,
        frame_timestamp,
        ::core::ptr::null::<api_detection_options_t>(),
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
