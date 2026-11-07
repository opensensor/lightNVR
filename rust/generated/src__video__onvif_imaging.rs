unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
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
    unsafe fn ezxml_parse_str(s: *mut ::core::ffi::c_char, len: size_t) -> ezxml_t;
    unsafe fn ezxml_free(xml: ezxml_t);
    unsafe fn onvif_create_security_header(
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn onvif_log_soap_fault(
        response: *const ::core::ffi::c_char,
        response_len: size_t,
        context: *const ::core::ffi::c_char,
    );
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn url_redact_for_logging(
        url: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
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
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtof(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_float;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub type ezxml_t = *mut ezxml;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_imaging_float_t {
    pub present: bool,
    pub value: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_imaging_mode_level_t {
    pub present: bool,
    pub mode: [::core::ffi::c_char; 16],
    pub has_level: bool,
    pub level: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_imaging_settings_t {
    pub video_source_token: [::core::ffi::c_char; 64],
    pub brightness: onvif_imaging_float_t,
    pub color_saturation: onvif_imaging_float_t,
    pub contrast: onvif_imaging_float_t,
    pub sharpness: onvif_imaging_float_t,
    pub noise_reduction: onvif_imaging_float_t,
    pub backlight_compensation: onvif_imaging_mode_level_t,
    pub wide_dynamic_range: onvif_imaging_mode_level_t,
    pub tone_compensation: onvif_imaging_mode_level_t,
    pub defogging: onvif_imaging_mode_level_t,
    pub has_ir_cut_filter: bool,
    pub ir_cut_filter: [::core::ffi::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_imaging_float_range_t {
    pub present: bool,
    pub min: ::core::ffi::c_float,
    pub max: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_imaging_mode_level_options_t {
    pub present: bool,
    pub modes: [[::core::ffi::c_char; 16]; 4],
    pub mode_count: ::core::ffi::c_int,
    pub level: onvif_imaging_float_range_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_imaging_options_t {
    pub video_source_token: [::core::ffi::c_char; 64],
    pub brightness: onvif_imaging_float_range_t,
    pub color_saturation: onvif_imaging_float_range_t,
    pub contrast: onvif_imaging_float_range_t,
    pub sharpness: onvif_imaging_float_range_t,
    pub noise_reduction: onvif_imaging_float_range_t,
    pub backlight_compensation: onvif_imaging_mode_level_options_t,
    pub wide_dynamic_range: onvif_imaging_mode_level_options_t,
    pub tone_compensation: onvif_imaging_mode_level_options_t,
    pub defogging: onvif_imaging_mode_level_options_t,
    pub ir_cut_filter_modes: [[::core::ffi::c_char; 16]; 4],
    pub ir_cut_filter_mode_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryStruct {
    pub memory: *mut ::core::ffi::c_char,
    pub size: size_t,
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
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn write_callback(
    mut contents: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut userp: *mut ::core::ffi::c_void,
) -> size_t {
    let mut real_size: size_t = size.wrapping_mul(nmemb);
    let mut mem: *mut MemoryStruct = userp as *mut MemoryStruct;
    let mut ptr: *mut ::core::ffi::c_char = realloc(
        (*mem).memory as *mut ::core::ffi::c_void,
        (*mem)
            .size
            .wrapping_add(real_size)
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    if ptr.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Not enough memory for ONVIF Imaging response\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as size_t;
    }
    (*mem).memory = ptr;
    memcpy(
        (*mem).memory.offset((*mem).size as isize) as *mut ::core::ffi::c_void,
        contents,
        real_size,
    );
    (*mem).size = (*mem).size.wrapping_add(real_size);
    *(*mem).memory.offset((*mem).size as isize) = '\0' as ::core::ffi::c_char;
    return real_size;
}
unsafe extern "C" fn append_escaped(
    mut dst: *mut *mut ::core::ffi::c_char,
    mut remaining: *mut size_t,
    mut text: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if dst.is_null() || (*dst).is_null() || remaining.is_null() || text.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut p: *const ::core::ffi::c_char = text;
    while *p != 0 {
        let mut replacement: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        let mut literal: [::core::ffi::c_char; 2] = [*p, '\0' as ::core::ffi::c_char];
        match *p as ::core::ffi::c_int {
            38 => {
                replacement = b"&amp;\0".as_ptr() as *const ::core::ffi::c_char;
            }
            60 => {
                replacement = b"&lt;\0".as_ptr() as *const ::core::ffi::c_char;
            }
            62 => {
                replacement = b"&gt;\0".as_ptr() as *const ::core::ffi::c_char;
            }
            34 => {
                replacement = b"&quot;\0".as_ptr() as *const ::core::ffi::c_char;
            }
            39 => {
                replacement = b"&apos;\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {
                replacement = &raw mut literal as *mut ::core::ffi::c_char;
            }
        }
        let mut len: size_t = strlen(replacement);
        if *remaining <= len {
            return -1 as ::core::ffi::c_int;
        }
        memcpy(
            *dst as *mut ::core::ffi::c_void,
            replacement as *const ::core::ffi::c_void,
            len,
        );
        *dst = (*dst).offset(len as isize);
        *remaining = (*remaining).wrapping_sub(len);
        p = p.offset(1);
    }
    **dst = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xml_escape(
    mut src: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_char,
    mut dst_size: size_t,
) -> ::core::ffi::c_int {
    if src.is_null() || dst.is_null() || dst_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut out: *mut ::core::ffi::c_char = dst;
    let mut remaining: size_t = dst_size;
    *out = '\0' as ::core::ffi::c_char;
    return append_escaped(&raw mut out, &raw mut remaining, src);
}
unsafe extern "C" fn send_imaging_soap_request(
    mut imaging_url: *const ::core::ffi::c_char,
    mut soap_action: *const ::core::ffi::c_char,
    mut request_body: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if imaging_url.is_null() || request_body.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut curl: *mut ::core::ffi::c_void = curl_easy_init();
    if curl.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize CURL for ONVIF Imaging request\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut chunk: MemoryStruct = MemoryStruct {
        memory: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
    };
    chunk.memory = malloc(1 as size_t) as *mut ::core::ffi::c_char;
    if chunk.memory.is_null() {
        curl_easy_cleanup(curl);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *chunk.memory.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut security_header: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !username.is_null()
        && !password.is_null()
        && *username.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && *password.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        security_header = onvif_create_security_header(username, password);
        if security_header.is_null() {
            free(chunk.memory as *mut ::core::ffi::c_void);
            curl_easy_cleanup(curl);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    } else {
        security_header = strdup(b"\0".as_ptr() as *const ::core::ffi::c_char);
        if security_header.is_null() {
            free(chunk.memory as *mut ::core::ffi::c_void);
            curl_easy_cleanup(curl);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    let mut envelope_size: size_t = strlen(request_body)
        .wrapping_add(strlen(security_header))
        .wrapping_add(2048 as size_t);
    let mut soap_envelope: *mut ::core::ffi::c_char =
        malloc(envelope_size) as *mut ::core::ffi::c_char;
    if soap_envelope.is_null() {
        free(security_header as *mut ::core::ffi::c_void);
        free(chunk.memory as *mut ::core::ffi::c_void);
        curl_easy_cleanup(curl);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    snprintf(
        soap_envelope,
        envelope_size,
        b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" xmlns:timg=\"http://www.onvif.org/ver20/imaging/wsdl\" xmlns:tt=\"http://www.onvif.org/ver10/schema\"><s:Header>%s</s:Header><s:Body>%s</s:Body></s:Envelope>\0"
            .as_ptr() as *const ::core::ffi::c_char,
        security_header,
        request_body,
    );
    let mut headers: *mut curl_slist = ::core::ptr::null_mut::<curl_slist>();
    headers = curl_slist_append(
        headers,
        b"Content-Type: application/soap+xml; charset=utf-8\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if !soap_action.is_null() {
        let mut soap_action_header: [::core::ffi::c_char; 256] = [0; 256];
        snprintf(
            &raw mut soap_action_header as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"SOAPAction: %s\0".as_ptr() as *const ::core::ffi::c_char,
            soap_action,
        );
        headers = curl_slist_append(
            headers,
            &raw mut soap_action_header as *mut ::core::ffi::c_char,
        );
    }
    let mut safe_url: [::core::ffi::c_char; 512] = [0; 512];
    if url_redact_for_logging(
        imaging_url,
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
        b"Sending ONVIF Imaging request to: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut safe_url as *mut ::core::ffi::c_char,
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_URL, imaging_url);
    curl_easy_setopt(curl, CURLoption::CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLoption::CURLOPT_POSTFIELDS, soap_envelope);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            write_callback
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
    let mut response: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: CURLcode = curl_easy_perform(curl);
    if res.0 != CURLcode::CURLE_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF Imaging CURL request failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            curl_easy_strerror(res),
        );
    } else {
        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
        if http_code != 200 as ::core::ffi::c_long {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"ONVIF Imaging request failed with HTTP code %ld\0".as_ptr()
                    as *const ::core::ffi::c_char,
                http_code,
            );
            if chunk.size > 0 as size_t {
                onvif_log_soap_fault(
                    chunk.memory,
                    chunk.size,
                    b"Imaging\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        } else if chunk.size > 0 as size_t {
            response = chunk.memory;
            chunk.memory = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    free(soap_envelope as *mut ::core::ffi::c_void);
    free(security_header as *mut ::core::ffi::c_void);
    free(chunk.memory as *mut ::core::ffi::c_void);
    return response;
}
unsafe extern "C" fn xml_local_name(
    mut name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut colon: *const ::core::ffi::c_char = if !name.is_null() {
        strchr(name, ':' as ::core::ffi::c_int)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    return if !colon.is_null() {
        colon.offset(1 as ::core::ffi::c_int as isize)
    } else {
        name
    };
}
unsafe extern "C" fn find_child_local(
    mut parent: ezxml_t,
    mut name: *const ::core::ffi::c_char,
) -> ezxml_t {
    if parent.is_null() || name.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    let mut child: ezxml_t = (*parent).child;
    while !child.is_null() {
        let mut local: *const ::core::ffi::c_char = xml_local_name((*child).name);
        if !local.is_null() && strcmp(local, name) == 0 as ::core::ffi::c_int {
            return child;
        }
        child = (*child).sibling;
    }
    return ::core::ptr::null_mut::<ezxml>();
}
unsafe extern "C" fn find_descendant_local(
    mut root: ezxml_t,
    mut name: *const ::core::ffi::c_char,
) -> ezxml_t {
    if root.is_null() || name.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    let mut local: *const ::core::ffi::c_char = xml_local_name((*root).name);
    if !local.is_null() && strcmp(local, name) == 0 as ::core::ffi::c_int {
        return root;
    }
    let mut child: ezxml_t = (*root).child;
    while !child.is_null() {
        let mut found: ezxml_t = find_descendant_local(child, name);
        if !found.is_null() {
            return found;
        }
        child = (*child).sibling;
    }
    return ::core::ptr::null_mut::<ezxml>();
}
unsafe extern "C" fn parse_float_text(
    mut text: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_float,
) -> bool {
    if text.is_null()
        || value.is_null()
        || *text.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parsed: ::core::ffi::c_float = strtof(text, &raw mut end);
    if end == text as *mut ::core::ffi::c_char {
        return r#false != 0;
    }
    *value = parsed;
    return r#true != 0;
}
unsafe extern "C" fn parse_float_setting(
    mut parent: ezxml_t,
    mut name: *const ::core::ffi::c_char,
    mut out: *mut onvif_imaging_float_t,
) {
    let mut node: ezxml_t = find_child_local(parent, name);
    if node.is_null() || out.is_null() {
        return;
    }
    if parse_float_text(
        if !node.is_null() {
            (*node).txt
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        &raw mut (*out).value,
    ) {
        (*out).present = r#true != 0;
    }
}
unsafe extern "C" fn parse_mode_level_setting(
    mut parent: ezxml_t,
    mut name: *const ::core::ffi::c_char,
    mut out: *mut onvif_imaging_mode_level_t,
) {
    let mut node: ezxml_t = find_child_local(parent, name);
    if node.is_null() || out.is_null() {
        return;
    }
    (*out).present = r#true != 0;
    let mut mode: ezxml_t =
        find_child_local(node, b"Mode\0".as_ptr() as *const ::core::ffi::c_char);
    if !mode.is_null() {
        safe_strcpy(
            &raw mut (*out).mode as *mut ::core::ffi::c_char,
            if !mode.is_null() {
                (*mode).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    }
    let mut level: ezxml_t =
        find_child_local(node, b"Level\0".as_ptr() as *const ::core::ffi::c_char);
    if !level.is_null()
        && parse_float_text(
            if !level.is_null() {
                (*level).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut (*out).level,
        ) as ::core::ffi::c_int
            != 0
    {
        (*out).has_level = r#true != 0;
    }
}
unsafe extern "C" fn parse_float_range(
    mut parent: ezxml_t,
    mut name: *const ::core::ffi::c_char,
    mut out: *mut onvif_imaging_float_range_t,
) {
    let mut node: ezxml_t = find_child_local(parent, name);
    if node.is_null() || out.is_null() {
        return;
    }
    let mut min_node: ezxml_t =
        find_child_local(node, b"Min\0".as_ptr() as *const ::core::ffi::c_char);
    let mut max_node: ezxml_t =
        find_child_local(node, b"Max\0".as_ptr() as *const ::core::ffi::c_char);
    let mut min_value: ::core::ffi::c_float = 0.0f32;
    let mut max_value: ::core::ffi::c_float = 0.0f32;
    if !min_node.is_null()
        && !max_node.is_null()
        && parse_float_text(
            if !min_node.is_null() {
                (*min_node).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut min_value,
        ) as ::core::ffi::c_int
            != 0
        && parse_float_text(
            if !max_node.is_null() {
                (*max_node).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut max_value,
        ) as ::core::ffi::c_int
            != 0
    {
        (*out).present = r#true != 0;
        (*out).min = min_value;
        (*out).max = max_value;
    }
}
unsafe extern "C" fn parse_mode_level_options(
    mut parent: ezxml_t,
    mut name: *const ::core::ffi::c_char,
    mut out: *mut onvif_imaging_mode_level_options_t,
) {
    let mut node: ezxml_t = find_child_local(parent, name);
    if node.is_null() || out.is_null() {
        return;
    }
    (*out).present = r#true != 0;
    let mut child: ezxml_t = (*node).child;
    while !child.is_null() && (*out).mode_count < 4 as ::core::ffi::c_int {
        let mut local: *const ::core::ffi::c_char = xml_local_name((*child).name);
        if !local.is_null()
            && strcmp(local, b"Mode\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut *(&raw mut (*out).modes as *mut [::core::ffi::c_char; 16])
                    .offset((*out).mode_count as isize) as *mut ::core::ffi::c_char,
                if !child.is_null() {
                    (*child).txt
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            (*out).mode_count += 1;
        }
        child = (*child).ordered;
    }
    parse_float_range(
        node,
        b"Level\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*out).level,
    );
}
unsafe extern "C" fn parse_settings_response(
    mut response: *const ::core::ffi::c_char,
    mut settings: *mut onvif_imaging_settings_t,
) -> ::core::ffi::c_int {
    if response.is_null() || settings.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        settings as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<onvif_imaging_settings_t>(),
    );
    let mut xml: ezxml_t = ezxml_parse_str(response as *mut ::core::ffi::c_char, strlen(response));
    if xml.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to parse ONVIF Imaging settings XML\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut settings_node: ezxml_t = find_descendant_local(
        xml,
        b"ImagingSettings\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if settings_node.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF Imaging settings response did not include ImagingSettings\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        ezxml_free(xml);
        return -1 as ::core::ffi::c_int;
    }
    let mut token: ezxml_t = find_child_local(
        settings_node,
        b"VideoSourceToken\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !token.is_null() {
        safe_strcpy(
            &raw mut (*settings).video_source_token as *mut ::core::ffi::c_char,
            if !token.is_null() {
                (*token).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    parse_float_setting(
        settings_node,
        b"Brightness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).brightness,
    );
    parse_float_setting(
        settings_node,
        b"ColorSaturation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).color_saturation,
    );
    parse_float_setting(
        settings_node,
        b"Contrast\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).contrast,
    );
    parse_float_setting(
        settings_node,
        b"Sharpness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).sharpness,
    );
    parse_float_setting(
        settings_node,
        b"NoiseReduction\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).noise_reduction,
    );
    parse_mode_level_setting(
        settings_node,
        b"BacklightCompensation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).backlight_compensation,
    );
    parse_mode_level_setting(
        settings_node,
        b"WideDynamicRange\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).wide_dynamic_range,
    );
    parse_mode_level_setting(
        settings_node,
        b"ToneCompensation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).tone_compensation,
    );
    parse_mode_level_setting(
        settings_node,
        b"Defogging\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).defogging,
    );
    let mut ir_cut: ezxml_t = find_child_local(
        settings_node,
        b"IrCutFilter\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ir_cut.is_null() {
        (*settings).has_ir_cut_filter = r#true != 0;
        safe_strcpy(
            &raw mut (*settings).ir_cut_filter as *mut ::core::ffi::c_char,
            if !ir_cut.is_null() {
                (*ir_cut).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    }
    ezxml_free(xml);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_options_response(
    mut response: *const ::core::ffi::c_char,
    mut options: *mut onvif_imaging_options_t,
) -> ::core::ffi::c_int {
    if response.is_null() || options.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        options as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<onvif_imaging_options_t>(),
    );
    let mut xml: ezxml_t = ezxml_parse_str(response as *mut ::core::ffi::c_char, strlen(response));
    if xml.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to parse ONVIF Imaging options XML\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut options_node: ezxml_t = find_descendant_local(
        xml,
        b"ImagingOptions\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if options_node.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF Imaging options response did not include ImagingOptions\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        ezxml_free(xml);
        return -1 as ::core::ffi::c_int;
    }
    let mut token: ezxml_t = find_child_local(
        options_node,
        b"VideoSourceToken\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !token.is_null() {
        safe_strcpy(
            &raw mut (*options).video_source_token as *mut ::core::ffi::c_char,
            if !token.is_null() {
                (*token).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    parse_float_range(
        options_node,
        b"Brightness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*options).brightness,
    );
    parse_float_range(
        options_node,
        b"ColorSaturation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*options).color_saturation,
    );
    parse_float_range(
        options_node,
        b"Contrast\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*options).contrast,
    );
    parse_float_range(
        options_node,
        b"Sharpness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*options).sharpness,
    );
    parse_float_range(
        options_node,
        b"NoiseReduction\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*options).noise_reduction,
    );
    parse_mode_level_options(
        options_node,
        b"BacklightCompensationOptions\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*options).backlight_compensation,
    );
    parse_mode_level_options(
        options_node,
        b"WideDynamicRangeOptions\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*options).wide_dynamic_range,
    );
    parse_mode_level_options(
        options_node,
        b"ToneCompensationOptions\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*options).tone_compensation,
    );
    parse_mode_level_options(
        options_node,
        b"DefoggingOptions\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*options).defogging,
    );
    let mut child: ezxml_t = (*options_node).child;
    while !child.is_null() && (*options).ir_cut_filter_mode_count < 4 as ::core::ffi::c_int {
        let mut local: *const ::core::ffi::c_char = xml_local_name((*child).name);
        if !local.is_null()
            && strcmp(
                local,
                b"IrCutFilterModes\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut *(&raw mut (*options).ir_cut_filter_modes
                    as *mut [::core::ffi::c_char; 16])
                    .offset((*options).ir_cut_filter_mode_count as isize)
                    as *mut ::core::ffi::c_char,
                if !child.is_null() {
                    (*child).txt
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            (*options).ir_cut_filter_mode_count += 1;
        }
        child = (*child).ordered;
    }
    ezxml_free(xml);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn onvif_imaging_get_settings(
    mut imaging_url: *const ::core::ffi::c_char,
    mut video_source_token: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut settings: *mut onvif_imaging_settings_t,
) -> ::core::ffi::c_int {
    if imaging_url.is_null() || video_source_token.is_null() || settings.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut token: [::core::ffi::c_char; 256] = [0; 256];
    if xml_escape(
        video_source_token,
        &raw mut token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut request_body: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut request_body as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"<timg:GetImagingSettings><timg:VideoSourceToken>%s</timg:VideoSourceToken></timg:GetImagingSettings>\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut token as *mut ::core::ffi::c_char,
    );
    let mut response: *mut ::core::ffi::c_char = send_imaging_soap_request(
        imaging_url,
        b"http://www.onvif.org/ver20/imaging/wsdl/GetImagingSettings\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut request_body as *mut ::core::ffi::c_char,
        username,
        password,
    );
    if response.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int = parse_settings_response(response, settings);
    free(response as *mut ::core::ffi::c_void);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn onvif_imaging_get_options(
    mut imaging_url: *const ::core::ffi::c_char,
    mut video_source_token: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut options: *mut onvif_imaging_options_t,
) -> ::core::ffi::c_int {
    if imaging_url.is_null() || video_source_token.is_null() || options.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut token: [::core::ffi::c_char; 256] = [0; 256];
    if xml_escape(
        video_source_token,
        &raw mut token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut request_body: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut request_body as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"<timg:GetOptions><timg:VideoSourceToken>%s</timg:VideoSourceToken></timg:GetOptions>\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut token as *mut ::core::ffi::c_char,
    );
    let mut response: *mut ::core::ffi::c_char = send_imaging_soap_request(
        imaging_url,
        b"http://www.onvif.org/ver20/imaging/wsdl/GetOptions\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut request_body as *mut ::core::ffi::c_char,
        username,
        password,
    );
    if response.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int = parse_options_response(response, options);
    free(response as *mut ::core::ffi::c_void);
    return rc;
}
unsafe extern "C" fn append_float_setting_xml(
    mut cursor: *mut *mut ::core::ffi::c_char,
    mut remaining: *mut size_t,
    mut name: *const ::core::ffi::c_char,
    mut setting: *const onvif_imaging_float_t,
) -> ::core::ffi::c_int {
    if setting.is_null() || !(*setting).present {
        return 0 as ::core::ffi::c_int;
    }
    let mut n: ::core::ffi::c_int = snprintf(
        *cursor,
        *remaining,
        b"<tt:%s>%.4f</tt:%s>\0".as_ptr() as *const ::core::ffi::c_char,
        name,
        (*setting).value as ::core::ffi::c_double,
        name,
    );
    if n < 0 as ::core::ffi::c_int || n as size_t >= *remaining {
        return -1 as ::core::ffi::c_int;
    }
    *cursor = (*cursor).offset(n as isize);
    *remaining = (*remaining).wrapping_sub(n as size_t);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn append_mode_level_setting_xml(
    mut cursor: *mut *mut ::core::ffi::c_char,
    mut remaining: *mut size_t,
    mut name: *const ::core::ffi::c_char,
    mut setting: *const onvif_imaging_mode_level_t,
) -> ::core::ffi::c_int {
    if setting.is_null() || !(*setting).present {
        return 0 as ::core::ffi::c_int;
    }
    let mut n: ::core::ffi::c_int = snprintf(
        *cursor,
        *remaining,
        b"<tt:%s>\0".as_ptr() as *const ::core::ffi::c_char,
        name,
    );
    if n < 0 as ::core::ffi::c_int || n as size_t >= *remaining {
        return -1 as ::core::ffi::c_int;
    }
    *cursor = (*cursor).offset(n as isize);
    *remaining = (*remaining).wrapping_sub(n as size_t);
    if (*setting).mode[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut escaped_mode: [::core::ffi::c_char; 64] = [0; 64];
        if xml_escape(
            &raw const (*setting).mode as *const ::core::ffi::c_char,
            &raw mut escaped_mode as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        n = snprintf(
            *cursor,
            *remaining,
            b"<tt:Mode>%s</tt:Mode>\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut escaped_mode as *mut ::core::ffi::c_char,
        );
        if n < 0 as ::core::ffi::c_int || n as size_t >= *remaining {
            return -1 as ::core::ffi::c_int;
        }
        *cursor = (*cursor).offset(n as isize);
        *remaining = (*remaining).wrapping_sub(n as size_t);
    }
    if (*setting).has_level {
        n = snprintf(
            *cursor,
            *remaining,
            b"<tt:Level>%.4f</tt:Level>\0".as_ptr() as *const ::core::ffi::c_char,
            (*setting).level as ::core::ffi::c_double,
        );
        if n < 0 as ::core::ffi::c_int || n as size_t >= *remaining {
            return -1 as ::core::ffi::c_int;
        }
        *cursor = (*cursor).offset(n as isize);
        *remaining = (*remaining).wrapping_sub(n as size_t);
    }
    n = snprintf(
        *cursor,
        *remaining,
        b"</tt:%s>\0".as_ptr() as *const ::core::ffi::c_char,
        name,
    );
    if n < 0 as ::core::ffi::c_int || n as size_t >= *remaining {
        return -1 as ::core::ffi::c_int;
    }
    *cursor = (*cursor).offset(n as isize);
    *remaining = (*remaining).wrapping_sub(n as size_t);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn onvif_imaging_set_settings(
    mut imaging_url: *const ::core::ffi::c_char,
    mut video_source_token: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut settings: *const onvif_imaging_settings_t,
    mut force_persistence: bool,
) -> ::core::ffi::c_int {
    if imaging_url.is_null() || video_source_token.is_null() || settings.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut token: [::core::ffi::c_char; 256] = [0; 256];
    if xml_escape(
        video_source_token,
        &raw mut token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut settings_xml: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut cursor: *mut ::core::ffi::c_char = &raw mut settings_xml as *mut ::core::ffi::c_char;
    let mut remaining: size_t = ::core::mem::size_of::<[::core::ffi::c_char; 2048]>();
    settings_xml[0usize] = '\0' as ::core::ffi::c_char;
    if append_float_setting_xml(
        &raw mut cursor,
        &raw mut remaining,
        b"Brightness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*settings).brightness,
    ) != 0 as ::core::ffi::c_int
        || append_float_setting_xml(
            &raw mut cursor,
            &raw mut remaining,
            b"ColorSaturation\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*settings).color_saturation,
        ) != 0 as ::core::ffi::c_int
        || append_float_setting_xml(
            &raw mut cursor,
            &raw mut remaining,
            b"Contrast\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*settings).contrast,
        ) != 0 as ::core::ffi::c_int
        || append_float_setting_xml(
            &raw mut cursor,
            &raw mut remaining,
            b"Sharpness\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*settings).sharpness,
        ) != 0 as ::core::ffi::c_int
        || append_float_setting_xml(
            &raw mut cursor,
            &raw mut remaining,
            b"NoiseReduction\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*settings).noise_reduction,
        ) != 0 as ::core::ffi::c_int
        || append_mode_level_setting_xml(
            &raw mut cursor,
            &raw mut remaining,
            b"BacklightCompensation\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*settings).backlight_compensation,
        ) != 0 as ::core::ffi::c_int
        || append_mode_level_setting_xml(
            &raw mut cursor,
            &raw mut remaining,
            b"WideDynamicRange\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*settings).wide_dynamic_range,
        ) != 0 as ::core::ffi::c_int
        || append_mode_level_setting_xml(
            &raw mut cursor,
            &raw mut remaining,
            b"ToneCompensation\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*settings).tone_compensation,
        ) != 0 as ::core::ffi::c_int
        || append_mode_level_setting_xml(
            &raw mut cursor,
            &raw mut remaining,
            b"Defogging\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*settings).defogging,
        ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to build ONVIF Imaging settings XML\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*settings).has_ir_cut_filter {
        let mut escaped_ir_cut: [::core::ffi::c_char; 64] = [0; 64];
        if xml_escape(
            &raw const (*settings).ir_cut_filter as *const ::core::ffi::c_char,
            &raw mut escaped_ir_cut as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut n: ::core::ffi::c_int = snprintf(
            cursor,
            remaining,
            b"<tt:IrCutFilter>%s</tt:IrCutFilter>\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut escaped_ir_cut as *mut ::core::ffi::c_char,
        );
        if n < 0 as ::core::ffi::c_int || n as size_t >= remaining {
            return -1 as ::core::ffi::c_int;
        }
        cursor = cursor.offset(n as isize);
        remaining = remaining.wrapping_sub(n as size_t);
    }
    let mut request_body: [::core::ffi::c_char; 3072] = [0; 3072];
    snprintf(
        &raw mut request_body as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 3072]>(),
        b"<timg:SetImagingSettings><timg:VideoSourceToken>%s</timg:VideoSourceToken><timg:ImagingSettings>%s</timg:ImagingSettings><timg:ForcePersistence>%s</timg:ForcePersistence></timg:SetImagingSettings>\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut token as *mut ::core::ffi::c_char,
        &raw mut settings_xml as *mut ::core::ffi::c_char,
        if force_persistence as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut response: *mut ::core::ffi::c_char = send_imaging_soap_request(
        imaging_url,
        b"http://www.onvif.org/ver20/imaging/wsdl/SetImagingSettings\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut request_body as *mut ::core::ffi::c_char,
        username,
        password,
    );
    if response.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int =
        if !strstr(response, b"Fault\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    if rc != 0 as ::core::ffi::c_int {
        onvif_log_soap_fault(
            response,
            strlen(response),
            b"SetImagingSettings\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    free(response as *mut ::core::ffi::c_void);
    return rc;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
