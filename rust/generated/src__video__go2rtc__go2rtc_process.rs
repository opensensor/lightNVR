use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type sqlite3;
    unsafe fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    unsafe fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off_t,
        __whence: ::core::ffi::c_int,
    ) -> __off_t;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn execv(
        __path: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn execl(
        __path: *const ::core::ffi::c_char,
        __arg: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn _exit(__status: ::core::ffi::c_int) -> !;
    unsafe fn getpid() -> __pid_t;
    unsafe fn getpgrp() -> __pid_t;
    unsafe fn getpgid(__pid: __pid_t) -> __pid_t;
    unsafe fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::core::ffi::c_int;
    unsafe fn fork() -> __pid_t;
    unsafe fn readlink(
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn ftruncate(__fd: ::core::ffi::c_int, __length: __off_t) -> ::core::ffi::c_int;
    unsafe fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    unsafe fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigprocmask(
        __how: ::core::ffi::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn chmod(__file: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    unsafe fn fchmod(__fd: ::core::ffi::c_int, __mode: __mode_t) -> ::core::ffi::c_int;
    unsafe fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn curl_easy_strerror(error: CURLcode) -> *const ::core::ffi::c_char;
    unsafe fn curl_easy_init() -> *mut ::core::ffi::c_void;
    unsafe fn curl_easy_setopt(curl: *mut ::core::ffi::c_void, option: CURLoption, ...)
    -> CURLcode;
    unsafe fn curl_easy_perform(curl: *mut ::core::ffi::c_void) -> CURLcode;
    unsafe fn curl_easy_cleanup(curl: *mut ::core::ffi::c_void);
    unsafe fn curl_easy_getinfo(curl: *mut ::core::ffi::c_void, info: CURLINFO, ...) -> CURLcode;
    unsafe fn go2rtc_parse_external_candidates(
        list: *const ::core::ffi::c_char,
        port: ::core::ffi::c_int,
        out: *mut [::core::ffi::c_char; 128],
        max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_api_get_base_path() -> *const ::core::ffi::c_char;
    unsafe fn go2rtc_api_apply_base_path_override(
        yaml: *const ::core::ffi::c_char,
        length: size_t,
    ) -> bool;
    unsafe fn go2rtc_api_get_server_info(rtsp_port: *mut ::core::ffi::c_int) -> bool;
    unsafe fn go2rtc_lifecycle_begin(
        operation: go2rtc_lifecycle_operation_t,
        coalesce: bool,
        intentional: bool,
        guard: *mut go2rtc_lifecycle_guard_t,
    ) -> bool;
    unsafe fn go2rtc_lifecycle_try_begin(
        operation: go2rtc_lifecycle_operation_t,
        intentional: bool,
        guard: *mut go2rtc_lifecycle_guard_t,
    ) -> bool;
    unsafe fn go2rtc_lifecycle_end(guard: *mut go2rtc_lifecycle_guard_t, result: bool);
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    static mut g_config: config_t;
    unsafe fn mkdir_recursive(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn publish_url_next(
        cursor: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn publish_url_list_is_valid(list: *const ::core::ffi::c_char) -> bool;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn string_has_non_whitespace(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn yaml_validate_go2rtc_override(
        src: *const ::core::ffi::c_char,
        len: size_t,
        out: *mut yaml_validation_result_t,
    );
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_all_stream_configs(
        streams: *mut stream_config_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_get_system_setting_alloc(
        key: *const ::core::ffi::c_char,
        out: *mut *mut ::core::ffi::c_char,
        out_len: *mut size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_set_system_setting(
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
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
    unsafe fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
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
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn mkostemp(
        __template: *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn realpath(
        __name: *const ::core::ffi::c_char,
        __resolved: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __clockid_t = ::core::ffi::c_int;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
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
pub struct C2Rust_Unnamed {
    pub configured_path: [::core::ffi::c_char; 4096],
    pub configured_path_present: bool,
    pub configured_path_exists: bool,
    pub configured_path_executable: bool,
    pub configured_path_version_ok: bool,
    pub probe_paths_tried: [::core::ffi::c_char; 1024],
    pub service_port_open: bool,
    pub service_http_ok: bool,
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
pub struct detection_engine_config_t {
    pub enabled: bool,
    pub num_threads: ::core::ffi::c_int,
    pub delegate: [::core::ffi::c_char; 16],
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
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub type uint8_t = u8;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_validation_result_t {
    pub valid: ::core::ffi::c_int,
    pub err_message: [::core::ffi::c_char; 256],
    pub err_line: ::core::ffi::c_int,
    pub err_column: ::core::ffi::c_int,
    pub duplicate_keys: ::core::ffi::c_int,
    pub non_mapping_root: ::core::ffi::c_int,
    pub parse_error: ::core::ffi::c_int,
    pub warnings: [[::core::ffi::c_char; 192]; 16],
    pub warning_count: ::core::ffi::c_int,
}
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const NSIG: ::core::ffi::c_int = _NSIG;
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ESRCH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ECHILD: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GO2RTC_FFMPEG_RTSP_INPUT: [::core::ffi::c_char; 126] = unsafe {
    ::core::mem::transmute::<
        [u8; 126],
        [::core::ffi::c_char; 126],
    >(
        *b"-threads:v 2 -fflags nobuffer -flags low_delay -timeout {timeout} -user_agent go2rtc/ffmpeg -rtsp_flags prefer_tcp -i {input}\0",
    )
};
pub const GO2RTC_FFMPEG_H264: [::core::ffi::c_char; 215] = unsafe {
    ::core::mem::transmute::<
        [u8; 215],
        [::core::ffi::c_char; 215],
    >(
        *b"-c:v libx264 -threads:v 2 -filter_threads 2 -g:v 30 -preset:v superfast -tune:v zerolatency -pix_fmt:v yuv420p -vf \"scale=w='min(iw,1920)':h='min(ih,1080)':force_original_aspect_ratio=decrease:force_divisible_by=2\"\0",
    )
};
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LIGHTNVR_VERSION_STRING: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"0.47.3\0") };
pub const GO2RTC_MAX_EXTERNAL_CANDIDATES: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const GO2RTC_BASE_PATH: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"/go2rtc\0") };
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
#[inline]
unsafe extern "C" fn configured_stream_slots() -> ::core::ffi::c_int {
    let mut slots: ::core::ffi::c_int = g_config.max_streams;
    if slots <= 0 as ::core::ffi::c_int || slots > MAX_STREAMS {
        return MAX_STREAMS;
    }
    return slots;
}
unsafe extern "C" fn yaml_escape_string(
    mut src: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_char,
    mut dst_size: size_t,
) -> *mut ::core::ffi::c_char {
    let mut j: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while *src.offset(i as isize) as ::core::ffi::c_int != 0
        && j.wrapping_add(2 as size_t) < dst_size
    {
        if *src.offset(i as isize) as ::core::ffi::c_int == '"' as ::core::ffi::c_int
            || *src.offset(i as isize) as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
        {
            let c2rust_fresh10 = j;
            j = j.wrapping_add(1);
            *dst.offset(c2rust_fresh10 as isize) = '\\' as ::core::ffi::c_char;
        }
        let c2rust_fresh11 = j;
        j = j.wrapping_add(1);
        *dst.offset(c2rust_fresh11 as isize) = *src.offset(i as isize);
        i = i.wrapping_add(1);
    }
    *dst.offset(j as isize) = '\0' as ::core::ffi::c_char;
    return dst;
}
unsafe extern "C" fn basename_equals(
    mut path_or_name: *const ::core::ffi::c_char,
    mut expected_name: *const ::core::ffi::c_char,
) -> bool {
    if path_or_name.is_null()
        || expected_name.is_null()
        || *expected_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut base: *const ::core::ffi::c_char = strrchr(path_or_name, '/' as ::core::ffi::c_int);
    base = if !base.is_null() {
        base.offset(1 as ::core::ffi::c_int as isize)
    } else {
        path_or_name
    };
    return strcmp(base, expected_name) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_proc_cmdline(
    mut pid: pid_t,
    mut cmdline: *mut ::core::ffi::c_char,
    mut cmdline_size: size_t,
    mut bytes_read: *mut size_t,
) -> bool {
    if cmdline.is_null() || cmdline_size == 0 as size_t {
        return r#false != 0;
    }
    let mut cmdline_path: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut cmdline_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"/proc/%d/cmdline\0".as_ptr() as *const ::core::ffi::c_char,
        pid,
    );
    let mut fp: *mut FILE = fopen(
        &raw mut cmdline_path as *mut ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if fp.is_null() {
        return r#false != 0;
    }
    let mut bytes: size_t = fread(
        cmdline as *mut ::core::ffi::c_void,
        1 as size_t,
        cmdline_size.wrapping_sub(1 as size_t),
        fp,
    ) as size_t;
    fclose(fp);
    *cmdline.offset(bytes as isize) = '\0' as ::core::ffi::c_char;
    if !bytes_read.is_null() {
        *bytes_read = bytes;
    }
    return r#true != 0;
}
unsafe extern "C" fn proc_first_cmdline_arg_basename_equals(
    mut pid: pid_t,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    let mut cmdline: [::core::ffi::c_char; 1024] = [
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
    let mut bytes_read: size_t = 0 as size_t;
    if !read_proc_cmdline(
        pid,
        &raw mut cmdline as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        &raw mut bytes_read,
    ) || bytes_read == 0 as size_t
    {
        return r#false != 0;
    }
    let mut arg0_len: size_t = strnlen(&raw mut cmdline as *mut ::core::ffi::c_char, bytes_read);
    if arg0_len == 0 as size_t {
        return r#false != 0;
    }
    let mut arg0: [::core::ffi::c_char; 4096] = [0; 4096];
    if arg0_len >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() {
        arg0_len =
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>().wrapping_sub(1usize) as size_t;
    }
    memcpy(
        &raw mut arg0 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        &raw mut cmdline as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        arg0_len,
    );
    arg0[arg0_len] = '\0' as ::core::ffi::c_char;
    return basename_equals(&raw mut arg0 as *mut ::core::ffi::c_char, name);
}
unsafe extern "C" fn scan_proc_for_argv0_basename(
    mut name: *const ::core::ffi::c_char,
    mut pids: *mut pid_t,
    mut max_pids: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut self_0: pid_t = getpid();
    let mut proc_dir: *mut DIR = opendir(b"/proc\0".as_ptr() as *const ::core::ffi::c_char);
    if proc_dir.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut entry: *const dirent = ::core::ptr::null::<dirent>();
    loop {
        entry = readdir(proc_dir);
        if !(!entry.is_null() && found < max_pids) {
            break;
        }
        let mut d: *const ::core::ffi::c_char =
            &raw const (*entry).d_name as *const ::core::ffi::c_char;
        if (*d as ::core::ffi::c_int) < '1' as ::core::ffi::c_int
            || *d as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            continue;
        }
        let mut ep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut pid: pid_t = strtol(d, &raw mut ep, 10 as ::core::ffi::c_int) as pid_t;
        if *ep as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            || pid <= 0 as ::core::ffi::c_int
            || pid == self_0
        {
            continue;
        }
        if proc_first_cmdline_arg_basename_equals(pid, name) {
            let c2rust_fresh1 = found;
            found += 1;
            *pids.offset(c2rust_fresh1 as isize) = pid;
        }
    }
    closedir(proc_dir);
    return found;
}
unsafe extern "C" fn check_tcp_port_open(mut port: ::core::ffi::c_int) -> bool {
    let mut hex_port: [::core::ffi::c_char; 8] = [0; 8];
    snprintf(
        &raw mut hex_port as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
        b":%04X\0".as_ptr() as *const ::core::ffi::c_char,
        port,
    );
    let mut tcp_files: [*const ::core::ffi::c_char; 3] = [
        b"/proc/net/tcp\0".as_ptr() as *const ::core::ffi::c_char,
        b"/proc/net/tcp6\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !tcp_files[i as usize].is_null() {
        let mut fp: *mut FILE = fopen(
            tcp_files[i as usize],
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !fp.is_null() {
            let mut line: [::core::ffi::c_char; 512] = [0; 512];
            fgets(
                &raw mut line as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int,
                fp,
            );
            while !fgets(
                &raw mut line as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int,
                fp,
            )
            .is_null()
            {
                let mut fields: [*mut ::core::ffi::c_char; 5] = [
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ];
                let mut saveptr: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut linecopy: [::core::ffi::c_char; 512] = [0; 512];
                safe_strcpy(
                    &raw mut linecopy as *mut ::core::ffi::c_char,
                    &raw mut line as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    0 as size_t,
                );
                let mut f: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut tok: *mut ::core::ffi::c_char = strtok_r(
                    &raw mut linecopy as *mut ::core::ffi::c_char,
                    b" \t\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut saveptr,
                );
                while !tok.is_null() && f < 5 as ::core::ffi::c_int {
                    let c2rust_fresh2 = f;
                    f += 1;
                    fields[c2rust_fresh2 as usize] = tok;
                    tok = strtok_r(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b" \t\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut saveptr,
                    );
                }
                if f < 4 as ::core::ffi::c_int
                    || fields[1usize].is_null()
                    || fields[3usize].is_null()
                {
                    continue;
                }
                if strstr(
                    fields[1usize],
                    &raw mut hex_port as *mut ::core::ffi::c_char,
                )
                .is_null()
                {
                    continue;
                }
                if strcmp(
                    fields[3usize],
                    b"0A\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    fclose(fp);
                    return r#true != 0;
                }
            }
            fclose(fp);
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn process_owns_tcp_listen_port(
    mut pid: pid_t,
    mut port: ::core::ffi::c_int,
) -> bool {
    if pid <= 0 as ::core::ffi::c_int
        || port <= 0 as ::core::ffi::c_int
        || port > 65535 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut listen_inodes: [::core::ffi::c_ulong; 64] = [0; 64];
    let mut inode_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tcp_files: [*const ::core::ffi::c_char; 3] = [
        b"/proc/net/tcp\0".as_ptr() as *const ::core::ffi::c_char,
        b"/proc/net/tcp6\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !tcp_files[i as usize].is_null() && inode_count < 64 as ::core::ffi::c_int {
        let mut fp: *mut FILE = fopen(
            tcp_files[i as usize],
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !fp.is_null() {
            let mut line: [::core::ffi::c_char; 512] = [0; 512];
            if fgets(
                &raw mut line as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int,
                fp,
            )
            .is_null()
            {
                fclose(fp);
            } else {
                while !fgets(
                    &raw mut line as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int,
                    fp,
                )
                .is_null()
                    && inode_count < 64 as ::core::ffi::c_int
                {
                    let mut local: [::core::ffi::c_char; 80] = [
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
                    ];
                    let mut remote: [::core::ffi::c_char; 80] = [
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
                    ];
                    let mut state: [::core::ffi::c_char; 8] =
                        [0 as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0];
                    let mut inode: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
                    let mut matched: ::core::ffi::c_int = sscanf(
                        &raw mut line as *mut ::core::ffi::c_char,
                        b" %*d: %79s %79s %7s %*s %*s %*s %*u %*u %lu\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut local as *mut ::core::ffi::c_char,
                        &raw mut remote as *mut ::core::ffi::c_char,
                        &raw mut state as *mut ::core::ffi::c_char,
                        &raw mut inode,
                    );
                    if matched != 4 as ::core::ffi::c_int
                        || strcmp(
                            &raw mut state as *mut ::core::ffi::c_char,
                            b"0A\0".as_ptr() as *const ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int
                        || inode == 0 as ::core::ffi::c_ulong
                    {
                        continue;
                    }
                    let mut colon: *mut ::core::ffi::c_char = strrchr(
                        &raw mut local as *mut ::core::ffi::c_char,
                        ':' as ::core::ffi::c_int,
                    );
                    if colon.is_null() {
                        continue;
                    }
                    let mut end: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    let mut local_port: ::core::ffi::c_long = strtol(
                        colon.offset(1 as ::core::ffi::c_int as isize),
                        &raw mut end,
                        16 as ::core::ffi::c_int,
                    );
                    if end.is_null()
                        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                        || local_port != port as ::core::ffi::c_long
                    {
                        continue;
                    }
                    let c2rust_fresh0 = inode_count;
                    inode_count += 1;
                    listen_inodes[c2rust_fresh0 as usize] = inode;
                }
                fclose(fp);
            }
        }
        i += 1;
    }
    if inode_count == 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut fd_dir_path: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut fd_dir_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"/proc/%d/fd\0".as_ptr() as *const ::core::ffi::c_char,
        pid,
    );
    let mut fd_dir: *mut DIR = opendir(&raw mut fd_dir_path as *mut ::core::ffi::c_char);
    if fd_dir.is_null() {
        return r#false != 0;
    }
    let mut owns_port: bool = r#false != 0;
    let mut entry: *const dirent = ::core::ptr::null::<dirent>();
    while !owns_port && {
        entry = readdir(fd_dir);
        !entry.is_null()
    } {
        if (*entry).d_name[0usize] as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
            continue;
        }
        let mut fd_path: [::core::ffi::c_char; 4096] = [0; 4096];
        let mut target: [::core::ffi::c_char; 128] = [0; 128];
        let mut n: ::core::ffi::c_int = snprintf(
            &raw mut fd_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut fd_dir_path as *mut ::core::ffi::c_char,
            &raw const (*entry).d_name as *const ::core::ffi::c_char,
        );
        if n <= 0 as ::core::ffi::c_int
            || n >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int
        {
            continue;
        }
        let mut target_len: ssize_t = readlink(
            &raw mut fd_path as *mut ::core::ffi::c_char,
            &raw mut target as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>().wrapping_sub(1 as size_t),
        );
        if target_len <= 0 as ssize_t {
            continue;
        }
        target[target_len as usize] = '\0' as ::core::ffi::c_char;
        let mut inode_0: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
        if sscanf(
            &raw mut target as *mut ::core::ffi::c_char,
            b"socket:[%lu]\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut inode_0,
        ) != 1 as ::core::ffi::c_int
        {
            continue;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < inode_count {
            if listen_inodes[i_0 as usize] == inode_0 {
                owns_port = r#true != 0;
                break;
            } else {
                i_0 += 1;
            }
        }
    }
    closedir(fd_dir);
    return owns_port;
}
unsafe extern "C" fn find_go2rtc_serving_pid(
    mut api_port: ::core::ffi::c_int,
    mut serving_pid: *mut pid_t,
) -> bool {
    let mut pids: [pid_t; 64] = [0; 64];
    let mut count: ::core::ffi::c_int = scan_proc_for_argv0_basename(
        b"go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut pids as *mut pid_t,
        64 as ::core::ffi::c_int,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if process_owns_tcp_listen_port(pids[i as usize], api_port) {
            if !serving_pid.is_null() {
                *serving_pid = pids[i as usize];
            }
            return r#true != 0;
        }
        i += 1;
    }
    if !serving_pid.is_null() {
        *serving_pid = -1 as ::core::ffi::c_int as pid_t;
    }
    return r#false != 0;
}
unsafe extern "C" fn find_binary_in_path(
    mut name: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) {
    let mut path_env: *const ::core::ffi::c_char =
        getenv(b"PATH\0".as_ptr() as *const ::core::ffi::c_char);
    if path_env.is_null() {
        path_env = b"/usr/local/bin:/usr/bin:/bin\0".as_ptr() as *const ::core::ffi::c_char;
    }
    let mut path_copy: [::core::ffi::c_char; 4096] = [0; 4096];
    safe_strcpy(
        &raw mut path_copy as *mut ::core::ffi::c_char,
        path_env,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        0 as size_t,
    );
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dir: *const ::core::ffi::c_char = strtok_r(
        &raw mut path_copy as *mut ::core::ffi::c_char,
        b":\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !dir.is_null() {
        let mut candidate: [::core::ffi::c_char; 4096] = [0; 4096];
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut n: ::core::ffi::c_int = snprintf(
            &raw mut candidate as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            dir,
            name,
        );
        if n > 0 as ::core::ffi::c_int
            && n < ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int
            && access(&raw mut candidate as *mut ::core::ffi::c_char, X_OK)
                == 0 as ::core::ffi::c_int
            && stat(&raw mut candidate as *mut ::core::ffi::c_char, &raw mut st)
                == 0 as ::core::ffi::c_int
        {
            if st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t {
                safe_strcpy(
                    out,
                    &raw mut candidate as *mut ::core::ffi::c_char,
                    out_size,
                    0 as size_t,
                );
                return;
            }
        }
        dir = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b":\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    *out.offset(0isize) = '\0' as ::core::ffi::c_char;
}
static mut g_binary_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut g_config_dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut g_config_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut g_override_path: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut g_override_quarantined_path: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut g_process_pid: pid_t = -1 as pid_t;
static mut g_process_start_ticks: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
static mut g_initialized: bool = r#false != 0;
static mut g_using_external_service: bool = r#false != 0;
static mut g_managed_listener_policy: [::core::ffi::c_char; 512] = [0; 512];
static mut g_rtsp_port: ::core::ffi::c_int = 8554 as ::core::ffi::c_int;
static mut g_api_port: ::core::ffi::c_int = 1984 as ::core::ffi::c_int;
pub const FAST_DEATH_THRESHOLD_SEC: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const QUARANTINE_FAST_DEATH_COUNT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const QUARANTINE_WINDOW_SEC: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const FAST_DEATH_RING_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
static mut g_last_start_time: time_t = 0 as time_t;
static mut g_last_stop_was_managed: bool = r#false != 0;
static mut g_fast_death_history: [time_t; 8] = [0; 8];
static mut g_fast_death_history_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn discard_response_data(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut userdata: *mut ::core::ffi::c_void,
) -> size_t {
    return size.wrapping_mul(nmemb);
}
unsafe extern "C" fn is_go2rtc_running_as_service(mut api_port: ::core::ffi::c_int) -> bool {
    let mut port_in_use: bool = check_tcp_port_open(api_port);
    if port_in_use {
        let mut serving_pid: pid_t = -1 as pid_t;
        if find_go2rtc_serving_pid(api_port, &raw mut serving_pid) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc PID %d is already serving API port %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                serving_pid,
                api_port,
            );
            return r#true != 0;
        }
        let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut res: CURLcode = CURLcode::CURLE_OK;
        let mut url: [::core::ffi::c_char; 256] = [0; 256];
        let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
        curl = curl_easy_init();
        if curl.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to initialize curl\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        snprintf(
            &raw mut url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"http://localhost:%d%s/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
            api_port,
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
                discard_response_data
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
        res = curl_easy_perform(curl);
        if res.0 != CURLcode::CURLE_OK.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Curl request failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                curl_easy_strerror(res),
            );
            curl_easy_cleanup(curl);
            return r#false != 0;
        }
        curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
        curl_easy_cleanup(curl);
        if http_code == 200 as ::core::ffi::c_long || http_code == 401 as ::core::ffi::c_long {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Port %d responds like go2rtc (HTTP %ld), but no go2rtc PID owns the socket\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                api_port,
                http_code,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Port %d returned HTTP %ld, not a verified go2rtc service\0".as_ptr()
                    as *const ::core::ffi::c_char,
                api_port,
                http_code,
            );
        }
    }
    return r#false != 0;
}
static mut k_well_known_go2rtc_paths: [*const ::core::ffi::c_char; 7] = [
    b"/bin/go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
    b"/usr/local/bin/go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
    b"/usr/bin/go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
    b"/opt/go2rtc/go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
    b"/rootfs/usr/local/go2rtc/go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
    b"/go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut g_discovery_diag: C2Rust_Unnamed = C2Rust_Unnamed {
    configured_path: [0; 4096],
    configured_path_present: false,
    configured_path_exists: false,
    configured_path_executable: false,
    configured_path_version_ok: false,
    probe_paths_tried: [0; 1024],
    service_port_open: false,
    service_http_ok: false,
};
unsafe extern "C" fn discovery_diag_reset() {
    memset(
        &raw mut g_discovery_diag as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<C2Rust_Unnamed>(),
    );
}
unsafe extern "C" fn discovery_diag_append_probe(mut path: *const ::core::ffi::c_char) {
    if path.is_null() {
        return;
    }
    let mut used: size_t =
        strlen(&raw mut g_discovery_diag.probe_paths_tried as *mut ::core::ffi::c_char);
    let mut cap: size_t = ::core::mem::size_of::<[::core::ffi::c_char; 1024]>();
    if used.wrapping_add(1 as size_t) >= cap {
        return;
    }
    let mut sep: *const ::core::ffi::c_char = if used > 0 as size_t {
        b",\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut need: size_t = strlen(sep).wrapping_add(strlen(path));
    if used.wrapping_add(need).wrapping_add(1 as size_t) >= cap {
        return;
    }
    snprintf(
        (&raw mut g_discovery_diag.probe_paths_tried as *mut ::core::ffi::c_char)
            .offset(used as isize),
        cap.wrapping_sub(used),
        b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        sep,
        path,
    );
}
unsafe extern "C" fn probe_go2rtc_version(
    mut path: *const ::core::ffi::c_char,
    mut version_out: *mut ::core::ffi::c_char,
    mut version_out_sz: size_t,
) -> ::core::ffi::c_int {
    if !version_out.is_null() && version_out_sz > 0 as size_t {
        *version_out.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if path.is_null() || *path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut pipefd: [::core::ffi::c_int; 2] = [0; 2];
    if pipe(&raw mut pipefd as *mut ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"probe_go2rtc_version: pipe() failed for %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut pid: pid_t = fork();
    if pid < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"probe_go2rtc_version: fork() failed for %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
        close(pipefd[0usize]);
        close(pipefd[1usize]);
        return 0 as ::core::ffi::c_int;
    }
    if pid == 0 as ::core::ffi::c_int {
        close(pipefd[0usize]);
        if dup2(pipefd[1usize], STDOUT_FILENO) == -1 as ::core::ffi::c_int
            || dup2(pipefd[1usize], STDERR_FILENO) == -1 as ::core::ffi::c_int
        {
            _exit(127 as ::core::ffi::c_int);
        }
        close(pipefd[1usize]);
        let mut devnull: ::core::ffi::c_int = open(
            b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
            O_RDONLY,
        );
        if devnull >= 0 as ::core::ffi::c_int {
            dup2(devnull, STDIN_FILENO);
            close(devnull);
        }
        execl(
            path,
            path,
            b"--version\0".as_ptr() as *const ::core::ffi::c_char,
            NULL as *mut ::core::ffi::c_char,
        );
        _exit(127 as ::core::ffi::c_int);
    }
    close(pipefd[1usize]);
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut buf_used: size_t = 0 as size_t;
    let timeout_ms_total: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
    let mut t_start: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut t_start);
    let mut timed_out: bool = r#false != 0;
    loop {
        let mut t_now: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_MONOTONIC, &raw mut t_now);
        let mut elapsed_ms: ::core::ffi::c_long = (t_now.tv_sec - t_start.tv_sec)
            * 1000 as ::core::ffi::c_long
            + (t_now.tv_nsec - t_start.tv_nsec) / 1000000 as ::core::ffi::c_long;
        let mut remain_ms: ::core::ffi::c_int = timeout_ms_total - elapsed_ms as ::core::ffi::c_int;
        if remain_ms <= 0 as ::core::ffi::c_int {
            timed_out = r#true != 0;
            break;
        } else {
            let mut pfd: pollfd = pollfd {
                fd: pipefd[0usize],
                events: POLLIN as ::core::ffi::c_short,
                revents: 0,
            };
            let mut pr: ::core::ffi::c_int = poll(&raw mut pfd, 1 as nfds_t, remain_ms);
            if pr < 0 as ::core::ffi::c_int {
                if *__errno_location() != EINTR {
                    break;
                }
            } else if pr == 0 as ::core::ffi::c_int {
                timed_out = r#true != 0;
                break;
            } else {
                if pfd.revents as ::core::ffi::c_int & (POLLIN | POLLHUP | POLLERR) == 0 {
                    continue;
                }
                if buf_used
                    >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>().wrapping_sub(1usize)
                {
                    let mut scratch: [::core::ffi::c_char; 512] = [0; 512];
                    let mut drained: ssize_t = read(
                        pipefd[0usize],
                        &raw mut scratch as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    );
                    if drained <= 0 as ssize_t {
                        break;
                    }
                } else {
                    let mut n: ssize_t = read(
                        pipefd[0usize],
                        (&raw mut buf as *mut ::core::ffi::c_char).offset(buf_used as isize)
                            as *mut ::core::ffi::c_void,
                        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
                            .wrapping_sub(1 as size_t)
                            .wrapping_sub(buf_used),
                    );
                    if n < 0 as ssize_t {
                        if *__errno_location() != EINTR {
                            break;
                        }
                    } else {
                        if n == 0 as ssize_t {
                            break;
                        }
                        buf_used = buf_used.wrapping_add(n as size_t);
                    }
                }
            }
        }
    }
    buf[buf_used] = '\0' as ::core::ffi::c_char;
    close(pipefd[0usize]);
    's_349: {
        if timed_out {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"probe_go2rtc_version: timeout waiting for %s --version\0".as_ptr()
                    as *const ::core::ffi::c_char,
                path,
            );
            kill(pid, SIGKILL);
        } else {
            let mut status_nb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut w_nb: pid_t = 0;
            loop {
                w_nb = waitpid(pid, &raw mut status_nb, WNOHANG) as pid_t;
                if !(w_nb == -1 as ::core::ffi::c_int && *__errno_location() == EINTR) {
                    break;
                }
            }
            if w_nb == 0 as ::core::ffi::c_int {
                kill(pid, SIGKILL);
            } else if w_nb == pid {
                if !(status_nb & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                    || (status_nb & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
                break 's_349;
            }
        }
        let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut waited: pid_t = 0;
        loop {
            waited = waitpid(pid, &raw mut status, 0 as ::core::ffi::c_int) as pid_t;
            if !(waited == -1 as ::core::ffi::c_int && *__errno_location() == EINTR) {
                break;
            }
        }
        if timed_out {
            return 0 as ::core::ffi::c_int;
        }
        if waited != pid {
            return 0 as ::core::ffi::c_int;
        }
        if !(status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            || (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut r#match: *const ::core::ffi::c_char = strstr(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"go2rtc version \0".as_ptr() as *const ::core::ffi::c_char,
    );
    if r#match.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !version_out.is_null() && version_out_sz > 0 as size_t {
        let mut line_end: *const ::core::ffi::c_char = strchr(r#match, '\n' as ::core::ffi::c_int);
        let mut len: size_t = if !line_end.is_null() {
            line_end.offset_from(r#match) as size_t
        } else {
            strlen(r#match)
        };
        if len >= version_out_sz {
            len = version_out_sz.wrapping_sub(1 as size_t);
        }
        memcpy(
            version_out as *mut ::core::ffi::c_void,
            r#match as *const ::core::ffi::c_void,
            len,
        );
        *version_out.offset(len as isize) = '\0' as ::core::ffi::c_char;
    }
    return 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_probe_version(
    mut path: *const ::core::ffi::c_char,
    mut version_out: *mut ::core::ffi::c_char,
    mut version_out_sz: size_t,
) -> ::core::ffi::c_int {
    return probe_go2rtc_version(path, version_out, version_out_sz);
}
unsafe extern "C" fn log_probe_attempt(
    mut path: *const ::core::ffi::c_char,
    mut exists: bool,
    mut executable: bool,
    mut checked_version: bool,
    mut version_ok: bool,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc binary probe: tried %s: exists=%s, executable=%s, version_ok=%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        path,
        if exists as ::core::ffi::c_int != 0 {
            b"yes\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"no\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !exists {
            b"n/a\0".as_ptr() as *const ::core::ffi::c_char
        } else if executable as ::core::ffi::c_int != 0 {
            b"yes\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"no\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !checked_version {
            b"n/a\0".as_ptr() as *const ::core::ffi::c_char
        } else if version_ok as ::core::ffi::c_int != 0 {
            b"yes\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"no\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
unsafe extern "C" fn try_go2rtc_candidate(
    mut candidate: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> bool {
    discovery_diag_append_probe(candidate);
    let mut exists: bool = access(candidate, F_OK) == 0 as ::core::ffi::c_int;
    let mut executable: bool =
        exists as ::core::ffi::c_int != 0 && access(candidate, X_OK) == 0 as ::core::ffi::c_int;
    let mut version_ok: bool = r#false != 0;
    let mut checked_version: bool = r#false != 0;
    if executable {
        checked_version = r#true != 0;
        version_ok = probe_go2rtc_version(
            candidate,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as size_t,
        ) == 1 as ::core::ffi::c_int;
    }
    log_probe_attempt(candidate, exists, executable, checked_version, version_ok);
    if executable as ::core::ffi::c_int != 0 && version_ok as ::core::ffi::c_int != 0 {
        safe_strcpy(out, candidate, out_size, 0 as size_t);
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn check_go2rtc_in_path(
    mut binary_path: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) -> bool {
    if binary_path.is_null() || buffer_size == 0 as size_t {
        return r#false != 0;
    }
    *binary_path.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !k_well_known_go2rtc_paths[i as usize].is_null() {
        if try_go2rtc_candidate(
            k_well_known_go2rtc_paths[i as usize],
            binary_path,
            buffer_size,
        ) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Found go2rtc binary at well-known path: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                binary_path,
            );
            return r#true != 0;
        }
        i += 1;
    }
    let mut path_candidate: [::core::ffi::c_char; 4096] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    find_binary_in_path(
        b"go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut path_candidate as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
    );
    if path_candidate[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if try_go2rtc_candidate(
            &raw mut path_candidate as *mut ::core::ffi::c_char,
            binary_path,
            buffer_size,
        ) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Found go2rtc binary in PATH: %s\0".as_ptr() as *const ::core::ffi::c_char,
                binary_path,
            );
            return r#true != 0;
        }
    } else {
        discovery_diag_append_probe(b"<PATH:not-found>\0".as_ptr() as *const ::core::ffi::c_char);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc binary probe: PATH walk found no 'go2rtc' entry\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_init(
    mut binary_path: *const ::core::ffi::c_char,
    mut config_dir: *const ::core::ffi::c_char,
    mut api_port: ::core::ffi::c_int,
) -> bool {
    if g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc process manager already initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if config_dir.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid config_dir parameter for go2rtc_process_init\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if mkdir_recursive(config_dir) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create go2rtc config directory: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            config_dir,
        );
        return r#false != 0;
    }
    g_config_dir = strdup(config_dir);
    let mut config_path_len: size_t = strlen(config_dir)
        .wrapping_add(strlen(
            b"/go2rtc.yaml\0".as_ptr() as *const ::core::ffi::c_char
        ))
        .wrapping_add(1 as size_t);
    g_config_path = malloc(config_path_len) as *mut ::core::ffi::c_char;
    if g_config_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for config path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(g_config_dir as *mut ::core::ffi::c_void);
        g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return r#false != 0;
    }
    snprintf(
        g_config_path,
        config_path_len,
        b"%s/go2rtc.yaml\0".as_ptr() as *const ::core::ffi::c_char,
        config_dir,
    );
    let mut override_path_len: size_t = strlen(config_dir)
        .wrapping_add(strlen(
            b"/override.yaml\0".as_ptr() as *const ::core::ffi::c_char
        ))
        .wrapping_add(1 as size_t);
    g_override_path = malloc(override_path_len) as *mut ::core::ffi::c_char;
    if g_override_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for override path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(g_config_dir as *mut ::core::ffi::c_void);
        free(g_config_path as *mut ::core::ffi::c_void);
        g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
        g_config_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return r#false != 0;
    }
    snprintf(
        g_override_path,
        override_path_len,
        b"%s/override.yaml\0".as_ptr() as *const ::core::ffi::c_char,
        config_dir,
    );
    let mut quar_path_len: size_t = strlen(config_dir)
        .wrapping_add(strlen(
            b"/override.quarantined.yaml\0".as_ptr() as *const ::core::ffi::c_char
        ))
        .wrapping_add(1 as size_t);
    g_override_quarantined_path = malloc(quar_path_len) as *mut ::core::ffi::c_char;
    if g_override_quarantined_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for override quarantine path\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(g_config_dir as *mut ::core::ffi::c_void);
        free(g_config_path as *mut ::core::ffi::c_void);
        free(g_override_path as *mut ::core::ffi::c_void);
        g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
        g_config_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
        g_override_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return r#false != 0;
    }
    snprintf(
        g_override_quarantined_path,
        quar_path_len,
        b"%s/override.quarantined.yaml\0".as_ptr() as *const ::core::ffi::c_char,
        config_dir,
    );
    g_api_port = if api_port > 0 as ::core::ffi::c_int {
        api_port
    } else {
        1984 as ::core::ffi::c_int
    };
    discovery_diag_reset();
    if !binary_path.is_null()
        && *binary_path.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        g_discovery_diag.configured_path_present = r#true != 0;
        safe_strcpy(
            &raw mut g_discovery_diag.configured_path as *mut ::core::ffi::c_char,
            binary_path,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            0 as size_t,
        );
        g_discovery_diag.configured_path_exists =
            access(binary_path, F_OK) == 0 as ::core::ffi::c_int;
        g_discovery_diag.configured_path_executable =
            g_discovery_diag.configured_path_exists as ::core::ffi::c_int != 0
                && access(binary_path, X_OK) == 0 as ::core::ffi::c_int;
        if g_discovery_diag.configured_path_executable {
            g_discovery_diag.configured_path_version_ok = probe_go2rtc_version(
                binary_path,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                0 as size_t,
            ) == 1 as ::core::ffi::c_int;
        }
    }
    if go2rtc_process_generate_override_file(g_override_path) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unable to resolve go2rtc override before service discovery\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    g_discovery_diag.service_port_open = check_tcp_port_open(g_api_port);
    let mut service_running: bool = is_go2rtc_running_as_service(g_api_port);
    g_discovery_diag.service_http_ok = service_running;
    let mut final_binary_path: [::core::ffi::c_char; 4096] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut found_binary: bool = r#false != 0;
    if g_discovery_diag.configured_path_version_ok {
        safe_strcpy(
            &raw mut final_binary_path as *mut ::core::ffi::c_char,
            binary_path,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            0 as size_t,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using provided go2rtc binary: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut final_binary_path as *mut ::core::ffi::c_char,
        );
        found_binary = r#true != 0;
    } else if g_discovery_diag.configured_path_executable {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Configured go2rtc binary at %s is executable but did not respond to --version as expected; falling back to discovery\0"
                .as_ptr() as *const ::core::ffi::c_char,
            binary_path,
        );
    } else if g_discovery_diag.configured_path_present {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc binary not found or not executable at specified path: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            binary_path,
        );
    }
    if !found_binary {
        if check_go2rtc_in_path(
            &raw mut final_binary_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        ) {
            found_binary = r#true != 0;
        }
    }
    if service_running {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc is already running as a service, will use the existing service\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        g_using_external_service = r#true != 0;
        if found_binary {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Also caching fallback binary for service-death recovery: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut final_binary_path as *mut ::core::ffi::c_char,
            );
            g_binary_path = strdup(&raw mut final_binary_path as *mut ::core::ffi::c_char);
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"No local go2rtc binary found; running service is the only option (no fallback if the service dies)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            g_binary_path = strdup(b"\0".as_ptr() as *const ::core::ffi::c_char);
        }
    } else {
        g_using_external_service = r#false != 0;
        if !found_binary {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc binary not found in well-known paths or PATH, and no running service detected\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            free(g_config_dir as *mut ::core::ffi::c_void);
            g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free(g_config_path as *mut ::core::ffi::c_void);
            g_config_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free(g_override_path as *mut ::core::ffi::c_void);
            g_override_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free(g_override_quarantined_path as *mut ::core::ffi::c_void);
            g_override_quarantined_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return r#false != 0;
        }
        g_binary_path = strdup(&raw mut final_binary_path as *mut ::core::ffi::c_char);
    }
    g_initialized = r#true != 0;
    if g_using_external_service {
        if !g_binary_path.is_null()
            && *g_binary_path.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc process manager initialized to use existing service (fallback binary cached: %s), config dir: %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                g_binary_path,
                g_config_dir,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc process manager initialized to use existing service, config dir: %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                g_config_dir,
            );
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc process manager initialized with binary: %s, config dir: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            g_binary_path,
            g_config_dir,
        );
    }
    return r#true != 0;
}
unsafe extern "C" fn go2rtc_process_generate_config_locked(
    mut config_path: *const ::core::ffi::c_char,
    mut api_port: ::core::ffi::c_int,
) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc process manager not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut config_fd: ::core::ffi::c_int = open(
        config_path,
        O_WRONLY | O_CREAT | O_NOFOLLOW | O_CLOEXEC | O_NONBLOCK,
        0o600 as ::core::ffi::c_int,
    );
    if config_fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open go2rtc config file for writing: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            config_path,
        );
        return r#false != 0;
    }
    let mut private_config: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if fstat(config_fd, &raw mut private_config) != 0 as ::core::ffi::c_int
        || !(private_config.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
        || private_config.st_nlink != 1 as __nlink_t
        || fchmod(config_fd, 0o600 as __mode_t) != 0 as ::core::ffi::c_int
        || ftruncate(config_fd, 0 as __off_t) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot prepare private go2rtc config file\0".as_ptr() as *const ::core::ffi::c_char,
        );
        close(config_fd);
        return r#false != 0;
    }
    let mut config_file: *mut FILE =
        fdopen(config_fd, b"w\0".as_ptr() as *const ::core::ffi::c_char);
    if config_file.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create file stream for go2rtc config file: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            config_path,
        );
        close(config_fd);
        return r#false != 0;
    }
    let mut global_config: *mut config_t = &raw mut g_config;
    fprintf(
        config_file,
        b"# go2rtc configuration generated by NVR software\n\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        config_file,
        b"api:\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        config_file,
        b"  listen: 127.0.0.1:%d\n\0".as_ptr() as *const ::core::ffi::c_char,
        api_port,
    );
    fprintf(
        config_file,
        b"  base_path: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        GO2RTC_BASE_PATH.as_ptr(),
    );
    let mut rtsp_port: ::core::ffi::c_int =
        if (*global_config).go2rtc_rtsp_port > 0 as ::core::ffi::c_int {
            (*global_config).go2rtc_rtsp_port
        } else {
            8554 as ::core::ffi::c_int
        };
    fprintf(
        config_file,
        b"\nrtsp:\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        config_file,
        b"  listen: \"127.0.0.1:%d\"\n\0".as_ptr() as *const ::core::ffi::c_char,
        rtsp_port,
    );
    if (*global_config).go2rtc_webrtc_enabled {
        fprintf(
            config_file,
            b"\nwebrtc:\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if (*global_config).go2rtc_webrtc_listen_port > 0 as ::core::ffi::c_int {
            fprintf(
                config_file,
                b"  listen: \":%d\"\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*global_config).go2rtc_webrtc_listen_port,
            );
        }
        if (*global_config).go2rtc_stun_enabled as ::core::ffi::c_int != 0
            || (*global_config).go2rtc_ice_servers[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            || (*global_config).turn_enabled as ::core::ffi::c_int != 0
                && (*global_config).turn_server_url[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
        {
            fprintf(
                config_file,
                b"  ice_servers:\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if (*global_config).go2rtc_ice_servers[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                let mut ice_servers_copy: [::core::ffi::c_char; 512] = [0; 512];
                safe_strcpy(
                    &raw mut ice_servers_copy as *mut ::core::ffi::c_char,
                    &raw mut (*global_config).go2rtc_ice_servers as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    0 as size_t,
                );
                let mut saveptr: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut token: *mut ::core::ffi::c_char = strtok_r(
                    &raw mut ice_servers_copy as *mut ::core::ffi::c_char,
                    b",\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut saveptr,
                );
                while !token.is_null() {
                    while *token as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
                        token = token.offset(1);
                    }
                    let mut end: *mut ::core::ffi::c_char = token
                        .offset(strlen(token) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize));
                    while end > token && *end as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
                        end = end.offset(-1);
                    }
                    *end.offset(1 as ::core::ffi::c_int as isize) = '\0' as ::core::ffi::c_char;
                    fprintf(
                        config_file,
                        b"    - urls: [\"%s\"]\n\0".as_ptr() as *const ::core::ffi::c_char,
                        token,
                    );
                    token = strtok_r(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b",\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut saveptr,
                    );
                }
            } else if (*global_config).go2rtc_stun_enabled {
                fprintf(
                    config_file,
                    b"    - urls:\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                fprintf(
                    config_file,
                    b"      - \"stun:%s\"\n\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*global_config).go2rtc_stun_server as *mut ::core::ffi::c_char,
                );
                fprintf(
                    config_file,
                    b"      - \"stun:stun1.l.google.com:19302\"\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                fprintf(
                    config_file,
                    b"      - \"stun:stun2.l.google.com:19302\"\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                fprintf(
                    config_file,
                    b"      - \"stun:stun3.l.google.com:19302\"\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                fprintf(
                    config_file,
                    b"      - \"stun:stun4.l.google.com:19302\"\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc config: turn_enabled=%d, turn_server_url='%s', turn_username='%s'\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*global_config).turn_enabled as ::core::ffi::c_int,
                &raw mut (*global_config).turn_server_url as *mut ::core::ffi::c_char,
                &raw mut (*global_config).turn_username as *mut ::core::ffi::c_char,
            );
            if (*global_config).turn_enabled as ::core::ffi::c_int != 0
                && (*global_config).turn_server_url[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"go2rtc config: Adding TURN server to config\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                fprintf(
                    config_file,
                    b"    - urls: [\"%s\"]\n\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*global_config).turn_server_url as *mut ::core::ffi::c_char,
                );
                if (*global_config).turn_username[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                {
                    fprintf(
                        config_file,
                        b"      username: \"%s\"\n\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*global_config).turn_username as *mut ::core::ffi::c_char,
                    );
                }
                if (*global_config).turn_password[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                {
                    fprintf(
                        config_file,
                        b"      credential: \"%s\"\n\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*global_config).turn_password as *mut ::core::ffi::c_char,
                    );
                }
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"go2rtc config: TURN server NOT added (enabled=%d, url_empty=%d)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*global_config).turn_enabled as ::core::ffi::c_int,
                    ((*global_config).turn_server_url[0usize] as ::core::ffi::c_int
                        == '\0' as ::core::ffi::c_int) as ::core::ffi::c_int,
                );
            }
        }
        fprintf(
            config_file,
            b"  candidates:\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut webrtc_port: ::core::ffi::c_int =
            if (*global_config).go2rtc_webrtc_listen_port > 0 as ::core::ffi::c_int {
                (*global_config).go2rtc_webrtc_listen_port
            } else {
                8555 as ::core::ffi::c_int
            };
        let mut candidates: [[::core::ffi::c_char; 128]; 8] = [[0; 128]; 8];
        let mut candidate_count: ::core::ffi::c_int = go2rtc_parse_external_candidates(
            &raw mut (*global_config).go2rtc_external_ip as *mut ::core::ffi::c_char,
            webrtc_port,
            &raw mut candidates as *mut [::core::ffi::c_char; 128],
            GO2RTC_MAX_EXTERNAL_CANDIDATES,
        );
        if candidate_count < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc config: ignoring invalid external_ip list; using auto-detect\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if candidate_count <= 0 as ::core::ffi::c_int {
            fprintf(
                config_file,
                b"    - \"*:%d\"\n\0".as_ptr() as *const ::core::ffi::c_char,
                webrtc_port,
            );
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < candidate_count {
            fprintf(
                config_file,
                b"    - \"%s\"\n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut *(&raw mut candidates as *mut [::core::ffi::c_char; 128])
                    .offset(i as isize) as *mut ::core::ffi::c_char,
            );
            i += 1;
        }
        if (*global_config).go2rtc_stun_enabled {
            fprintf(
                config_file,
                b"    - \"stun:%d\"\n\0".as_ptr() as *const ::core::ffi::c_char,
                webrtc_port,
            );
        }
        fprintf(config_file, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    fprintf(
        config_file,
        b"ffmpeg:\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    static mut guard_paths: [*const ::core::ffi::c_char; 3] = [
        b"/bin/lightnvr-ffmpeg-guard\0".as_ptr() as *const ::core::ffi::c_char,
        b"/usr/bin/lightnvr-ffmpeg-guard\0".as_ptr() as *const ::core::ffi::c_char,
        b"/usr/local/bin/lightnvr-ffmpeg-guard\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i_0: size_t = 0 as size_t;
    while i_0
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if access(guard_paths[i_0], X_OK) == 0 as ::core::ffi::c_int {
            fprintf(
                config_file,
                b"  bin: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                guard_paths[i_0],
            );
            break;
        } else {
            i_0 = i_0.wrapping_add(1);
        }
    }
    let mut escaped_transcode: [::core::ffi::c_char; 1024] = [0; 1024];
    fprintf(
        config_file,
        b"  rtsp: \"%s\"\n\0".as_ptr() as *const ::core::ffi::c_char,
        yaml_escape_string(
            GO2RTC_FFMPEG_RTSP_INPUT.as_ptr(),
            &raw mut escaped_transcode as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ),
    );
    fprintf(
        config_file,
        b"  h264: \"%s\"\n\0".as_ptr() as *const ::core::ffi::c_char,
        yaml_escape_string(
            GO2RTC_FFMPEG_H264.as_ptr(),
            &raw mut escaped_transcode as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ),
    );
    fprintf(
        config_file,
        b"  h265: \"-codec:v libx265 -g:v 30 -preset:v superfast\"\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        config_file,
        b"\nstreams:\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if write_stream_overrides(config_file) == 0 as ::core::ffi::c_int {
        fprintf(
            config_file,
            b"  # Streams will be added dynamically via API\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    write_publish_config(config_file);
    let mut write_failed: ::core::ffi::c_int = ferror(config_file);
    if fclose(config_file) != 0 as ::core::ffi::c_int {
        write_failed = 1 as ::core::ffi::c_int;
    }
    if write_failed != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to write go2rtc configuration file\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Generated go2rtc configuration file: %s\0".as_ptr() as *const ::core::ffi::c_char,
        config_path,
    );
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_generate_config(
    mut config_path: *const ::core::ffi::c_char,
    mut api_port: ::core::ffi::c_int,
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
            b"Failed to enter go2rtc lifecycle for config generation\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut result: bool = go2rtc_process_generate_config_locked(config_path, api_port);
    go2rtc_lifecycle_end(&raw mut guard, result);
    return result;
}
unsafe extern "C" fn write_stream_overrides(mut fp: *mut FILE) -> ::core::ffi::c_int {
    if fp.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if get_db_handle().is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if g_config.audio_disabled {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Ignoring per-stream go2rtc source overrides while the global audio policy is enabled\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut ms: ::core::ffi::c_int = configured_stream_slots();
    let mut streams: *mut stream_config_t =
        calloc(ms as size_t, ::core::mem::size_of::<stream_config_t>()) as *mut stream_config_t;
    if streams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = get_all_stream_configs(streams, ms);
    let mut written: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if (*streams.offset(i as isize)).enabled {
            if !(*streams.offset(i as isize)).privacy_mode {
                if string_has_non_whitespace(
                    &raw mut (*streams.offset(i as isize)).go2rtc_source_override
                        as *mut ::core::ffi::c_char,
                ) {
                    let mut escaped_name: [::core::ffi::c_char; 512] = [0; 512];
                    yaml_escape_string(
                        &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                        &raw mut escaped_name as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    );
                    let mut r#override: *const ::core::ffi::c_char =
                        &raw mut (*streams.offset(i as isize)).go2rtc_source_override
                            as *mut ::core::ffi::c_char;
                    let mut is_single_line: bool =
                        strchr(r#override, '\n' as ::core::ffi::c_int).is_null();
                    if is_single_line {
                        fprintf(
                            fp,
                            b"  \"%s\": %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut escaped_name as *mut ::core::ffi::c_char,
                            r#override,
                        );
                    } else {
                        fprintf(
                            fp,
                            b"  \"%s\":\n\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut escaped_name as *mut ::core::ffi::c_char,
                        );
                        let mut p: *const ::core::ffi::c_char = r#override;
                        while *p != 0 {
                            let mut eol: *const ::core::ffi::c_char =
                                strchr(p, '\n' as ::core::ffi::c_int);
                            if !eol.is_null() {
                                fprintf(
                                    fp,
                                    b"    %.*s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                    eol.offset_from(p) as ::core::ffi::c_int,
                                    p,
                                );
                                p = eol.offset(1 as ::core::ffi::c_int as isize);
                            } else {
                                fprintf(
                                    fp,
                                    b"    %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                    p,
                                );
                                break;
                            }
                        }
                    }
                    written += 1;
                }
            }
        }
        i += 1;
    }
    free(streams as *mut ::core::ffi::c_void);
    return written;
}
unsafe extern "C" fn write_publish_config(mut fp: *mut FILE) -> ::core::ffi::c_int {
    if fp.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if get_db_handle().is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut ms: ::core::ffi::c_int = configured_stream_slots();
    let mut streams: *mut stream_config_t =
        calloc(ms as size_t, ::core::mem::size_of::<stream_config_t>()) as *mut stream_config_t;
    if streams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = get_all_stream_configs(streams, ms);
    let mut written: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if (*streams.offset(i as isize)).enabled {
            if !(*streams.offset(i as isize)).privacy_mode {
                if (*streams.offset(i as isize)).publish_url[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                {
                    if !publish_url_list_is_valid(
                        &raw mut (*streams.offset(i as isize)).publish_url
                            as *mut ::core::ffi::c_char,
                    ) {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Skipping invalid publish_url for stream %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                        );
                    } else {
                        if written == 0 as ::core::ffi::c_int {
                            fprintf(fp, b"\npublish:\n\0".as_ptr() as *const ::core::ffi::c_char);
                        }
                        let mut escaped_name: [::core::ffi::c_char; 512] = [0; 512];
                        yaml_escape_string(
                            &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                            &raw mut escaped_name as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                        );
                        fprintf(
                            fp,
                            b"  \"%s\":\n\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut escaped_name as *mut ::core::ffi::c_char,
                        );
                        let mut destination: [::core::ffi::c_char; 512] = [0; 512];
                        let mut cursor: *const ::core::ffi::c_char =
                            &raw mut (*streams.offset(i as isize)).publish_url
                                as *mut ::core::ffi::c_char;
                        loop {
                            cursor = publish_url_next(
                                cursor,
                                &raw mut destination as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            );
                            if cursor.is_null() {
                                break;
                            }
                            let mut escaped_dest: [::core::ffi::c_char; 1024] = [0; 1024];
                            yaml_escape_string(
                                &raw mut destination as *mut ::core::ffi::c_char,
                                &raw mut escaped_dest as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                            );
                            fprintf(
                                fp,
                                b"    - \"%s\"\n\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut escaped_dest as *mut ::core::ffi::c_char,
                            );
                        }
                        written += 1;
                    }
                }
            }
        }
        i += 1;
    }
    free(streams as *mut ::core::ffi::c_void);
    return written;
}
unsafe extern "C" fn mask_url_userinfo_in_place(mut buf: *mut ::core::ffi::c_char) {
    if buf.is_null() {
        return;
    }
    let mut p: *mut ::core::ffi::c_char = buf;
    loop {
        p = strstr(p, b"://\0".as_ptr() as *const ::core::ffi::c_char);
        if p.is_null() {
            break;
        }
        let mut scheme_end: *mut ::core::ffi::c_char = p.offset(3 as ::core::ffi::c_int as isize);
        let mut q: *mut ::core::ffi::c_char = scheme_end;
        let mut at: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        while *q as ::core::ffi::c_int != 0
            && *q as ::core::ffi::c_int != '/' as ::core::ffi::c_int
            && *q as ::core::ffi::c_int != '?' as ::core::ffi::c_int
            && *q as ::core::ffi::c_int != '#' as ::core::ffi::c_int
            && *q as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
            && *q as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
            && *q as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
            && *q as ::core::ffi::c_int != '"' as ::core::ffi::c_int
            && *q as ::core::ffi::c_int != '\'' as ::core::ffi::c_int
        {
            if *q as ::core::ffi::c_int == '@' as ::core::ffi::c_int {
                at = q;
                break;
            } else {
                q = q.offset(1);
            }
        }
        if !at.is_null() {
            let mut has_colon: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut r: *mut ::core::ffi::c_char = scheme_end;
            while r < at {
                if *r as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                    has_colon = 1 as ::core::ffi::c_int;
                    break;
                } else {
                    r = r.offset(1);
                }
            }
            if has_colon != 0 {
                let mut r_0: *mut ::core::ffi::c_char = scheme_end;
                while r_0 < at {
                    *r_0 = '*' as ::core::ffi::c_char;
                    r_0 = r_0.offset(1);
                }
            }
            p = at.offset(1 as ::core::ffi::c_int as isize);
        } else {
            p = q;
            if *p != 0 {
                p = p.offset(1);
            }
        }
    }
}
unsafe extern "C" fn read_file_tail(
    mut path: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut cap: size_t,
) {
    if out.is_null() || cap == 0 as size_t {
        return;
    }
    *out.offset(0isize) = '\0' as ::core::ffi::c_char;
    if path.is_null() || *path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return;
    }
    let mut fd: ::core::ffi::c_int = open(path, O_RDONLY | O_CLOEXEC);
    if fd < 0 as ::core::ffi::c_int {
        return;
    }
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if fstat(fd, &raw mut st) != 0 as ::core::ffi::c_int || st.st_size <= 0 as __off_t {
        close(fd);
        return;
    }
    let mut want: size_t = cap.wrapping_sub(1 as size_t);
    let mut off: off_t = 0 as off_t;
    if st.st_size as size_t > want {
        off = st.st_size - want as off_t;
    } else {
        want = st.st_size as size_t;
    }
    if lseek(fd, off, SEEK_SET) == -1 as ::core::ffi::c_int as off_t {
        close(fd);
        return;
    }
    let mut read_total: size_t = 0 as size_t;
    while read_total < want {
        let mut n: ssize_t = read(
            fd,
            out.offset(read_total as isize) as *mut ::core::ffi::c_void,
            want.wrapping_sub(read_total),
        );
        if n < 0 as ssize_t {
            if *__errno_location() != EINTR {
                break;
            }
        } else {
            if n == 0 as ssize_t {
                break;
            }
            read_total = read_total.wrapping_add(n as size_t);
        }
    }
    *out.offset(read_total as isize) = '\0' as ::core::ffi::c_char;
    close(fd);
}
unsafe extern "C" fn quarantine_override_file() {
    if g_override_path.is_null() || g_override_quarantined_path.is_null() {
        return;
    }
    if rename(g_override_path, g_override_quarantined_path) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"T4b: failed to quarantine %s -> %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            g_override_path,
            g_override_quarantined_path,
            strerror(*__errno_location()),
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"T4b: quarantined go2rtc override after crash loop: %s -> %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        g_override_path,
        g_override_quarantined_path,
    );
    let mut log_tail: [::core::ffi::c_char; 2048] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut log_path: [::core::ffi::c_char; 4096] = [0; 4096];
    snprintf(
        &raw mut log_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"%s/go2rtc.log\0".as_ptr() as *const ::core::ffi::c_char,
        if !g_config_dir.is_null() {
            g_config_dir
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    read_file_tail(
        &raw mut log_path as *mut ::core::ffi::c_char,
        &raw mut log_tail as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
    );
    mask_url_userinfo_in_place(&raw mut log_tail as *mut ::core::ffi::c_char);
    let mut reason: *mut ::core::ffi::c_char =
        malloc(::core::mem::size_of::<[::core::ffi::c_char; 2048]>().wrapping_add(512 as size_t))
            as *mut ::core::ffi::c_char;
    if !reason.is_null() {
        snprintf(
            reason,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()
                .wrapping_add(512 as size_t),
            b"go2rtc crashed >= %d times (each within %ds of start) within a %ds rolling window with override.yaml in use. Quarantined to %s.\n\nLast %zu bytes of go2rtc.log:\n%s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            QUARANTINE_FAST_DEATH_COUNT,
            FAST_DEATH_THRESHOLD_SEC,
            QUARANTINE_WINDOW_SEC,
            g_override_quarantined_path,
            strlen(&raw mut log_tail as *mut ::core::ffi::c_char),
            &raw mut log_tail as *mut ::core::ffi::c_char,
        );
        if db_set_system_setting(
            b"go2rtc_config_override_disabled_reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"T4b: failed to persist quarantine reason to DB\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        free(reason as *mut ::core::ffi::c_void);
    }
    memset(
        &raw mut g_fast_death_history as *mut time_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[time_t; 8]>(),
    );
    g_fast_death_history_idx = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn check_and_handle_crash_loop() {
    if g_last_stop_was_managed {
        g_last_stop_was_managed = r#false != 0;
        return;
    }
    if g_last_start_time == 0 as time_t {
        return;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut lifetime: time_t = now - g_last_start_time;
    if lifetime < 0 as time_t || lifetime >= FAST_DEATH_THRESHOLD_SEC as time_t {
        return;
    }
    g_fast_death_history[g_fast_death_history_idx as usize] = now;
    g_fast_death_history_idx =
        (g_fast_death_history_idx + 1 as ::core::ffi::c_int) % FAST_DEATH_RING_SIZE;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"T4b: go2rtc previous instance lived only %lds (< %ds threshold)\0".as_ptr()
            as *const ::core::ffi::c_char,
        lifetime,
        10 as ::core::ffi::c_int,
    );
    let mut recent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < FAST_DEATH_RING_SIZE {
        if g_fast_death_history[i as usize] > 0 as time_t
            && now - g_fast_death_history[i as usize] < QUARANTINE_WINDOW_SEC as time_t
        {
            recent += 1;
        }
        i += 1;
    }
    if recent < QUARANTINE_FAST_DEATH_COUNT {
        return;
    }
    if !g_override_path.is_null() && access(g_override_path, F_OK) == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"T4b: %d fast-death events in %ds \xE2\x80\x94 quarantining override.yaml\0".as_ptr()
                as *const ::core::ffi::c_char,
            recent,
            60 as ::core::ffi::c_int,
        );
        quarantine_override_file();
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"T4b: %d fast-death events but override.yaml is not in use; lightNVR cannot mitigate further\0"
                .as_ptr() as *const ::core::ffi::c_char,
            recent,
        );
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_validate_existing_override_on_upgrade() {
    let mut marker: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut marker_len: size_t = 0 as size_t;
    let mut rc: ::core::ffi::c_int = db_get_system_setting_alloc(
        b"go2rtc_override_validated_version\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut marker,
        &raw mut marker_len,
    );
    if rc == 0 as ::core::ffi::c_int
        && !marker.is_null()
        && strcmp(marker, LIGHTNVR_VERSION_STRING.as_ptr()) == 0 as ::core::ffi::c_int
    {
        free(marker as *mut ::core::ffi::c_void);
        return;
    }
    free(marker as *mut ::core::ffi::c_void);
    let mut r#override: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut override_len: size_t = 0 as size_t;
    let mut orc: ::core::ffi::c_int = db_get_system_setting_alloc(
        b"go2rtc_config_override\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut r#override,
        &raw mut override_len,
    );
    if orc != 0 as ::core::ffi::c_int || r#override.is_null() || override_len == 0 as size_t {
        free(r#override as *mut ::core::ffi::c_void);
        if db_set_system_setting(
            b"go2rtc_override_validated_version\0".as_ptr() as *const ::core::ffi::c_char,
            LIGHTNVR_VERSION_STRING.as_ptr(),
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"T14: failed to set validation marker\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    let mut vr: yaml_validation_result_t = yaml_validation_result_t {
        valid: 0,
        err_message: [0; 256],
        err_line: 0,
        err_column: 0,
        duplicate_keys: 0,
        non_mapping_root: 0,
        parse_error: 0,
        warnings: [[0; 192]; 16],
        warning_count: 0,
    };
    yaml_validate_go2rtc_override(r#override, override_len, &raw mut vr);
    if vr.valid == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"T14: existing go2rtc_config_override is INVALID after upgrade (%s); quarantining\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut vr.err_message as *mut ::core::ffi::c_char,
        );
        if db_set_system_setting(
            b"go2rtc_config_override_quarantined\0".as_ptr() as *const ::core::ffi::c_char,
            r#override,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"T14: failed to copy override to quarantine slot \xE2\x80\x94 leaving live setting alone\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            free(r#override as *mut ::core::ffi::c_void);
            return;
        }
        let mut reason: [::core::ffi::c_char; 320] = [0; 320];
        snprintf(
            &raw mut reason as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 320]>(),
            b"Pre-existing override quarantined on upgrade: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut vr.err_message as *mut ::core::ffi::c_char,
        );
        if db_set_system_setting(
            b"go2rtc_config_override_disabled_reason\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut reason as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"T14: failed to persist quarantine reason\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if db_set_system_setting(
            b"go2rtc_config_override\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"T14: failed to clear live override after copy\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"T14: original override preserved at system_settings.go2rtc_config_override_quarantined; review and re-save via the UI to re-enable\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    } else if vr.valid == 1 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"T14: existing go2rtc_config_override validated cleanly (%d warnings)\0".as_ptr()
                as *const ::core::ffi::c_char,
            vr.warning_count,
        );
    }
    free(r#override as *mut ::core::ffi::c_void);
    if vr.valid != -1 as ::core::ffi::c_int {
        if db_set_system_setting(
            b"go2rtc_override_validated_version\0".as_ptr() as *const ::core::ffi::c_char,
            LIGHTNVR_VERSION_STRING.as_ptr(),
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"T14: failed to set validation marker\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_clear_override_quarantine() {
    if g_override_quarantined_path.is_null() {
        return;
    }
    if unlink(g_override_quarantined_path) == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"T4b: cleared quarantined go2rtc override file %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            g_override_quarantined_path,
        );
    } else if *__errno_location() != ENOENT {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"T4b: failed to remove quarantined override %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            g_override_quarantined_path,
            strerror(*__errno_location()),
        );
    }
    if db_set_system_setting(
        b"go2rtc_config_override_disabled_reason\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"T4b: failed to clear quarantine reason in DB\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    g_last_start_time = 0 as time_t;
    memset(
        &raw mut g_fast_death_history as *mut time_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[time_t; 8]>(),
    );
    g_fast_death_history_idx = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn go2rtc_process_generate_override_file_locked(
    mut override_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if override_path.is_null()
        || *override_path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc_process_generate_override_file: invalid path\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if g_config.audio_disabled {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Ignoring global go2rtc override while the global audio policy is enabled\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        if unlink(override_path) != 0 as ::core::ffi::c_int && *__errno_location() != ENOENT {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to remove go2rtc override under audio policy: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
        go2rtc_api_apply_base_path_override(
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as size_t,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut reason: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut reason_len: size_t = 0 as size_t;
    if db_get_system_setting_alloc(
        b"go2rtc_config_override_disabled_reason\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut reason,
        &raw mut reason_len,
    ) == 0 as ::core::ffi::c_int
        && !reason.is_null()
        && reason_len > 0 as size_t
    {
        free(reason as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc override is currently QUARANTINED \xE2\x80\x94 skipping override.yaml. Save a new override to clear the quarantine.\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        if unlink(override_path) != 0 as ::core::ffi::c_int && *__errno_location() != ENOENT {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to remove override during quarantine: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
        go2rtc_api_apply_base_path_override(
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as size_t,
        );
        return 0 as ::core::ffi::c_int;
    }
    free(reason as *mut ::core::ffi::c_void);
    if !g_config_dir.is_null()
        && *g_config_dir.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        let mut dst: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        if stat(g_config_dir, &raw mut dst) == 0 as ::core::ffi::c_int
            && dst.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
        {
            let mut perm: mode_t = dst.st_mode & 0o777 as mode_t;
            if perm != 0o700 as mode_t && perm != 0o750 as mode_t {
                if chmod(g_config_dir, 0o700 as __mode_t) == 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"go2rtc config dir %s was 0%o; tightened to 0700\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        g_config_dir,
                        perm,
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"go2rtc config dir %s is 0%o (want 0700/0750); chmod failed: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        g_config_dir,
                        perm,
                        strerror(*__errno_location()),
                    );
                }
            }
        }
    }
    let mut content: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut content_len: size_t = 0 as size_t;
    let mut db_rc: ::core::ffi::c_int = db_get_system_setting_alloc(
        b"go2rtc_config_override\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut content,
        &raw mut content_len,
    );
    let mut have_content: bool =
        db_rc == 0 as ::core::ffi::c_int && !content.is_null() && content_len > 0 as size_t;
    if have_content as ::core::ffi::c_int != 0 && content_len > 65535 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc override in DB is %zu bytes (cap 65535) \xE2\x80\x94 treating as absent and starting on base config alone\0"
                .as_ptr() as *const ::core::ffi::c_char,
            content_len,
        );
        free(content as *mut ::core::ffi::c_void);
        content = ::core::ptr::null_mut::<::core::ffi::c_char>();
        content_len = 0 as size_t;
        have_content = r#false != 0;
    }
    if !have_content {
        free(content as *mut ::core::ffi::c_void);
        if unlink(override_path) != 0 as ::core::ffi::c_int && *__errno_location() != ENOENT {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to remove stale go2rtc override file %s: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                override_path,
                strerror(*__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        if stat(override_path, &raw mut st) == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stale go2rtc override file %s still present after unlink\0".as_ptr()
                    as *const ::core::ffi::c_char,
                override_path,
            );
            return -1 as ::core::ffi::c_int;
        }
        if *__errno_location() != ENOENT {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to verify go2rtc override file %s removed: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                override_path,
                strerror(*__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc override file absent (no DB setting)\0".as_ptr() as *const ::core::ffi::c_char,
        );
        go2rtc_api_apply_base_path_override(
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as size_t,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut override_tmp_path: [::core::ffi::c_char; 4096] = [0; 4096];
    if snprintf(
        &raw mut override_tmp_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"%s.tmp.XXXXXX\0".as_ptr() as *const ::core::ffi::c_char,
        override_path,
    ) >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Temporary path for go2rtc override file is too long: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            override_path,
        );
        free(content as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut fd: ::core::ffi::c_int = mkostemp(
        &raw mut override_tmp_path as *mut ::core::ffi::c_char,
        O_CLOEXEC,
    );
    if fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open temporary go2rtc override file %s for writing: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut override_tmp_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        free(content as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut private_override: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if fstat(fd, &raw mut private_override) != 0 as ::core::ffi::c_int
        || !(private_override.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
        || private_override.st_nlink != 1 as __nlink_t
        || fchmod(fd, 0o600 as __mode_t) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot prepare private temporary go2rtc override file %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut override_tmp_path as *mut ::core::ffi::c_char,
        );
        close(fd);
        unlink(&raw mut override_tmp_path as *mut ::core::ffi::c_char);
        free(content as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut written: size_t = 0 as size_t;
    while written < content_len {
        let mut n: ssize_t = write(
            fd,
            content.offset(written as isize) as *const ::core::ffi::c_void,
            content_len.wrapping_sub(written),
        );
        if n <= 0 as ssize_t {
            if n < 0 as ssize_t && *__errno_location() == EINTR {
                continue;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Write to temporary go2rtc override file %s failed: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut override_tmp_path as *mut ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            close(fd);
            unlink(&raw mut override_tmp_path as *mut ::core::ffi::c_char);
            free(content as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        } else {
            written = written.wrapping_add(n as size_t);
        }
    }
    if fsync(fd) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"fsync on temporary go2rtc override file %s failed: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut override_tmp_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(fd);
        unlink(&raw mut override_tmp_path as *mut ::core::ffi::c_char);
        free(content as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    if close(fd) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"close on temporary go2rtc override file %s failed: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut override_tmp_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        unlink(&raw mut override_tmp_path as *mut ::core::ffi::c_char);
        free(content as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    if rename(
        &raw mut override_tmp_path as *mut ::core::ffi::c_char,
        override_path,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to atomically publish go2rtc override file %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            override_path,
            strerror(*__errno_location()),
        );
        unlink(&raw mut override_tmp_path as *mut ::core::ffi::c_char);
        free(content as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Wrote %zu bytes to go2rtc override file %s\0".as_ptr() as *const ::core::ffi::c_char,
        content_len,
        override_path,
    );
    let mut prefix_ok: bool = go2rtc_api_apply_base_path_override(content, content_len);
    free(content as *mut ::core::ffi::c_void);
    if !prefix_ok {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid go2rtc api.base_path: use an absolute path without a trailing slash, or an empty string\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_generate_override_file(
    mut override_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
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
            b"Failed to enter go2rtc lifecycle for override generation\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int =
        go2rtc_process_generate_override_file_locked(override_path);
    go2rtc_lifecycle_end(&raw mut guard, result == 0 as ::core::ffi::c_int);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_get_override_path() -> *const ::core::ffi::c_char {
    return g_override_path;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_get_listener_policy(
    mut policy: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> bool {
    if policy.is_null() || capacity == 0 as size_t {
        return r#false != 0;
    }
    *policy.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    if !go2rtc_lifecycle_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_CHECK,
        r#false != 0,
        r#false != 0,
        &raw mut guard,
    ) {
        return r#false != 0;
    }
    let mut available: bool = g_initialized as ::core::ffi::c_int != 0
        && !g_using_external_service
        && g_process_pid > 0 as ::core::ffi::c_int
        && g_managed_listener_policy[0usize] as ::core::ffi::c_int != 0
        && strlen(&raw mut g_managed_listener_policy as *mut ::core::ffi::c_char) < capacity;
    if available {
        strcpy(
            policy,
            &raw mut g_managed_listener_policy as *mut ::core::ffi::c_char,
        );
    }
    go2rtc_lifecycle_end(&raw mut guard, available);
    return available;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_get_config_path() -> *const ::core::ffi::c_char {
    return g_config_path;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_generate_startup_config(
    mut binary_path: *const ::core::ffi::c_char,
    mut config_dir: *const ::core::ffi::c_char,
    mut api_port: ::core::ffi::c_int,
) -> bool {
    if config_dir.is_null()
        || *config_dir.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid config_dir for go2rtc startup config generation\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot generate startup config: process manager already initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut resolved_api_port: ::core::ffi::c_int = if api_port > 0 as ::core::ffi::c_int {
        api_port
    } else {
        1984 as ::core::ffi::c_int
    };
    if mkdir_recursive(config_dir) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create go2rtc config directory: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            config_dir,
        );
        return r#false != 0;
    }
    g_config_dir = strdup(config_dir);
    if g_config_dir.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for config dir\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut config_path_len: size_t = strlen(config_dir)
        .wrapping_add(strlen(
            b"/go2rtc.yaml\0".as_ptr() as *const ::core::ffi::c_char
        ))
        .wrapping_add(1 as size_t);
    g_config_path = malloc(config_path_len) as *mut ::core::ffi::c_char;
    if g_config_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for config path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(g_config_dir as *mut ::core::ffi::c_void);
        g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return r#false != 0;
    }
    snprintf(
        g_config_path,
        config_path_len,
        b"%s/go2rtc.yaml\0".as_ptr() as *const ::core::ffi::c_char,
        config_dir,
    );
    let mut override_path_len: size_t = strlen(config_dir)
        .wrapping_add(strlen(
            b"/override.yaml\0".as_ptr() as *const ::core::ffi::c_char
        ))
        .wrapping_add(1 as size_t);
    g_override_path = malloc(override_path_len) as *mut ::core::ffi::c_char;
    if g_override_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for override path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(g_config_dir as *mut ::core::ffi::c_void);
        free(g_config_path as *mut ::core::ffi::c_void);
        g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
        g_config_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return r#false != 0;
    }
    snprintf(
        g_override_path,
        override_path_len,
        b"%s/override.yaml\0".as_ptr() as *const ::core::ffi::c_char,
        config_dir,
    );
    let mut quar_path_len: size_t = strlen(config_dir)
        .wrapping_add(strlen(
            b"/override.quarantined.yaml\0".as_ptr() as *const ::core::ffi::c_char
        ))
        .wrapping_add(1 as size_t);
    g_override_quarantined_path = malloc(quar_path_len) as *mut ::core::ffi::c_char;
    if g_override_quarantined_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for override quarantine path\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(g_config_dir as *mut ::core::ffi::c_void);
        free(g_config_path as *mut ::core::ffi::c_void);
        free(g_override_path as *mut ::core::ffi::c_void);
        g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
        g_config_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
        g_override_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return r#false != 0;
    }
    snprintf(
        g_override_quarantined_path,
        quar_path_len,
        b"%s/override.quarantined.yaml\0".as_ptr() as *const ::core::ffi::c_char,
        config_dir,
    );
    g_api_port = resolved_api_port;
    g_rtsp_port = if g_config.go2rtc_rtsp_port > 0 as ::core::ffi::c_int {
        g_config.go2rtc_rtsp_port
    } else {
        8554 as ::core::ffi::c_int
    };
    g_initialized = r#true != 0;
    let mut ok: bool = go2rtc_process_generate_config(g_config_path, resolved_api_port);
    if ok as ::core::ffi::c_int != 0
        && !get_db_handle().is_null()
        && go2rtc_process_generate_override_file(g_override_path) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to generate go2rtc startup override file: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            g_override_path,
        );
        ok = r#false != 0;
    }
    free(g_config_dir as *mut ::core::ffi::c_void);
    free(g_config_path as *mut ::core::ffi::c_void);
    free(g_override_path as *mut ::core::ffi::c_void);
    free(g_override_quarantined_path as *mut ::core::ffi::c_void);
    g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_config_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_override_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_override_quarantined_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_initialized = r#false != 0;
    g_using_external_service = r#false != 0;
    return ok;
}
unsafe extern "C" fn is_zombie_process(mut pid: pid_t) -> bool {
    let mut stat_path: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut stat_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"/proc/%d/stat\0".as_ptr() as *const ::core::ffi::c_char,
        pid,
    );
    let mut fp: *mut FILE = fopen(
        &raw mut stat_path as *mut ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if fp.is_null() {
        return r#false != 0;
    }
    let mut line: [::core::ffi::c_char; 512] = [0; 512];
    if !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int,
        fp,
    )
    .is_null()
    {
        fclose(fp);
        let mut close_paren: *const ::core::ffi::c_char = strrchr(
            &raw mut line as *mut ::core::ffi::c_char,
            ')' as ::core::ffi::c_int,
        );
        if !close_paren.is_null()
            && *close_paren.offset(1isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        {
            let mut state: ::core::ffi::c_char = *close_paren.offset(2isize);
            if state as ::core::ffi::c_int == 'Z' as ::core::ffi::c_int {
                return r#true != 0;
            }
        }
    } else {
        fclose(fp);
    }
    return r#false != 0;
}
unsafe extern "C" fn process_start_ticks(mut pid: pid_t) -> ::core::ffi::c_ulonglong {
    let mut stat_path: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut stat_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"/proc/%d/stat\0".as_ptr() as *const ::core::ffi::c_char,
        pid,
    );
    let mut fp: *mut FILE = fopen(
        &raw mut stat_path as *mut ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if fp.is_null() {
        return 0 as ::core::ffi::c_ulonglong;
    }
    let mut line: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut read_ok: bool = !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as ::core::ffi::c_int,
        fp,
    )
    .is_null();
    fclose(fp);
    if !read_ok {
        return 0 as ::core::ffi::c_ulonglong;
    }
    let mut close_paren: *mut ::core::ffi::c_char = strrchr(
        &raw mut line as *mut ::core::ffi::c_char,
        ')' as ::core::ffi::c_int,
    );
    if close_paren.is_null()
        || *close_paren.offset(1isize) as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_ulonglong;
    }
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token: *mut ::core::ffi::c_char = strtok_r(
        close_paren.offset(2 as ::core::ffi::c_int as isize),
        b" \0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    let mut field: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    while !token.is_null() {
        if field == 22 as ::core::ffi::c_int {
            return strtoull(
                token,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                10 as ::core::ffi::c_int,
            );
        }
        token = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b" \0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
        field += 1;
    }
    return 0 as ::core::ffi::c_ulonglong;
}
unsafe extern "C" fn managed_process_identity_matches(mut pid: pid_t) -> bool {
    if pid <= 0 as ::core::ffi::c_int || g_process_start_ticks == 0 as ::core::ffi::c_ulonglong {
        return r#false != 0;
    }
    let mut actual_ticks: ::core::ffi::c_ulonglong = process_start_ticks(pid);
    if actual_ticks != g_process_start_ticks {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Managed go2rtc PID %d identity changed (expected start=%llu, actual=%llu)\0".as_ptr()
                as *const ::core::ffi::c_char,
            pid,
            g_process_start_ticks,
            actual_ticks,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn wait_for_process_termination(
    mut pid: pid_t,
    mut timeout_ms: ::core::ffi::c_int,
) -> bool {
    let mut elapsed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut interval: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    while elapsed < timeout_ms {
        let mut status: ::core::ffi::c_int = 0;
        let mut result: pid_t = waitpid(pid, &raw mut status, WNOHANG);
        if result == pid || pid < 0 as ::core::ffi::c_int && result > 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Process %d successfully reaped\0".as_ptr() as *const ::core::ffi::c_char,
                pid,
            );
            return r#true != 0;
        } else if result == -1 as ::core::ffi::c_int && *__errno_location() == ECHILD {
            if kill(pid, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
                return r#true != 0;
            }
            if is_zombie_process(pid) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Process %d is zombie (not our child)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pid,
                );
                return r#true != 0;
            }
        }
        usleep((interval * 1000 as ::core::ffi::c_int) as __useconds_t);
        elapsed += interval;
    }
    return r#false != 0;
}
unsafe extern "C" fn signal_managed_process(mut pid: pid_t, mut sig: ::core::ffi::c_int) {
    let mut pgid: pid_t = getpgid(pid);
    if pgid == pid && pgid != getpgrp() {
        if kill(-pgid, sig) == 0 as ::core::ffi::c_int || *__errno_location() == ESRCH {
            return;
        }
    }
    if kill(pid, sig) != 0 as ::core::ffi::c_int && *__errno_location() != ESRCH {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to signal managed go2rtc PID %d: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pid,
            strerror(*__errno_location()),
        );
    }
}
unsafe extern "C" fn terminate_and_reap_child(mut pid: pid_t) -> bool {
    if pid <= 0 as ::core::ffi::c_int {
        return r#true != 0;
    }
    signal_managed_process(pid, SIGTERM);
    if wait_for_process_termination(pid, 2000 as ::core::ffi::c_int) {
        return r#true != 0;
    }
    signal_managed_process(pid, SIGKILL);
    return wait_for_process_termination(pid, 1000 as ::core::ffi::c_int);
}
unsafe extern "C" fn child_exited_and_reaped(mut pid: pid_t) -> bool {
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut result: pid_t = 0;
    loop {
        result = waitpid(pid, &raw mut status, WNOHANG) as pid_t;
        if !(result == -1 as ::core::ffi::c_int && *__errno_location() == EINTR) {
            break;
        }
    }
    if result == pid {
        if status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc child %d exited with status %d\0".as_ptr() as *const ::core::ffi::c_char,
                pid,
                (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int,
            );
        } else if ((status & 0x7f as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_schar as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc child %d exited on signal %d\0".as_ptr() as *const ::core::ffi::c_char,
                pid,
                status & 0x7f as ::core::ffi::c_int,
            );
        }
        return r#true != 0;
    }
    return result == -1 as ::core::ffi::c_int && *__errno_location() == ECHILD;
}
unsafe extern "C" fn process_parent_pid(mut pid: pid_t) -> pid_t {
    let mut path: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"/proc/%d/status\0".as_ptr() as *const ::core::ffi::c_char,
        pid,
    );
    let mut fp: *mut FILE = fopen(
        &raw mut path as *mut ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if fp.is_null() {
        return -1 as pid_t;
    }
    let mut line: [::core::ffi::c_char; 256] = [0; 256];
    let mut parent: pid_t = -1 as pid_t;
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
        fp,
    )
    .is_null()
    {
        if sscanf(
            &raw mut line as *mut ::core::ffi::c_char,
            b"PPid:%d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut parent,
        ) == 1 as ::core::ffi::c_int
        {
            break;
        }
    }
    fclose(fp);
    return parent;
}
unsafe extern "C" fn go2rtc_process_is_running_locked() -> bool {
    if !g_initialized {
        return r#false != 0;
    }
    if g_using_external_service {
        return is_go2rtc_running_as_service(g_api_port);
    }
    if g_process_pid > 0 as ::core::ffi::c_int {
        let mut identity_matches: bool = managed_process_identity_matches(g_process_pid);
        if identity_matches as ::core::ffi::c_int != 0
            && process_owns_tcp_listen_port(g_process_pid, g_api_port) as ::core::ffi::c_int != 0
        {
            return r#true != 0;
        }
        if !identity_matches || is_zombie_process(g_process_pid) as ::core::ffi::c_int != 0 {
            let mut dead_pid: pid_t = g_process_pid;
            waitpid(
                dead_pid,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                WNOHANG,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Tracked go2rtc process (PID: %d) exited\0".as_ptr() as *const ::core::ffi::c_char,
                dead_pid,
            );
            g_process_pid = -1 as ::core::ffi::c_int as pid_t;
            g_process_start_ticks = 0 as ::core::ffi::c_ulonglong;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Tracked go2rtc PID %d is alive but is not serving API port %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                g_process_pid,
                g_api_port,
            );
        }
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_is_running() -> bool {
    let mut guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    if !go2rtc_lifecycle_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_CHECK,
        r#false != 0,
        r#false != 0,
        &raw mut guard,
    ) {
        return r#false != 0;
    }
    let mut result: bool = go2rtc_process_is_running_locked();
    go2rtc_lifecycle_end(&raw mut guard, result);
    return result;
}
unsafe extern "C" fn go2rtc_process_start_locked(mut api_port: ::core::ffi::c_int) -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc process manager not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    check_and_handle_crash_loop();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Regenerating go2rtc configuration file fresh at startup\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if !go2rtc_process_generate_config(g_config_path, api_port) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to regenerate go2rtc configuration at startup\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut override_path: *const ::core::ffi::c_char = go2rtc_process_get_override_path();
    if !override_path.is_null()
        && go2rtc_process_generate_override_file(override_path) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Refusing to start go2rtc: failed to refresh override file %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            override_path,
        );
        return r#false != 0;
    }
    if is_go2rtc_running_as_service(api_port) {
        let mut serving_pid: pid_t = -1 as pid_t;
        find_go2rtc_serving_pid(api_port, &raw mut serving_pid);
        if serving_pid > 0 as ::core::ffi::c_int && process_parent_pid(serving_pid) == getpid() {
            g_using_external_service = r#false != 0;
            g_process_pid = serving_pid;
            g_process_start_ticks = process_start_ticks(serving_pid);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Recovered verified managed go2rtc serving PID %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                serving_pid,
            );
        } else if serving_pid > 0 as ::core::ffi::c_int && serving_pid != g_process_pid {
            g_using_external_service = r#true != 0;
            g_process_pid = -1 as ::core::ffi::c_int as pid_t;
            g_process_start_ticks = 0 as ::core::ffi::c_ulonglong;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Using verified external go2rtc serving PID %d on port %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                serving_pid,
                api_port,
            );
        }
        let mut retries: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
        let mut got_rtsp_port: bool = r#false != 0;
        while retries > 0 as ::core::ffi::c_int && !got_rtsp_port {
            if go2rtc_api_get_server_info(&raw mut g_rtsp_port) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Retrieved RTSP port from go2rtc API: %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    g_rtsp_port,
                );
                got_rtsp_port = r#true != 0;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Could not retrieve RTSP port from go2rtc API, retrying... (%d retries left)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    retries,
                );
                sleep(1 as ::core::ffi::c_uint);
                retries -= 1;
            }
        }
        if !got_rtsp_port {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Could not retrieve RTSP port from go2rtc API after multiple attempts, using default: %d\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                g_rtsp_port,
            );
        }
        return r#true != 0;
    }
    if go2rtc_process_is_running() {
        if check_tcp_port_open(api_port) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc is already running and listening on port %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                api_port,
            );
            let mut retries_0: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
            let mut got_rtsp_port_0: bool = r#false != 0;
            while retries_0 > 0 as ::core::ffi::c_int && !got_rtsp_port_0 {
                if go2rtc_api_get_server_info(&raw mut g_rtsp_port) {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Retrieved RTSP port from go2rtc API: %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        g_rtsp_port,
                    );
                    got_rtsp_port_0 = r#true != 0;
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Could not retrieve RTSP port from go2rtc API, retrying... (%d retries left)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        retries_0,
                    );
                    sleep(1 as ::core::ffi::c_uint);
                    retries_0 -= 1;
                }
            }
            if !got_rtsp_port_0 {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Could not retrieve RTSP port from go2rtc API after multiple attempts, using default: %d\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    g_rtsp_port,
                );
            }
            return r#true != 0;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc is running but not listening on port %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                api_port,
            );
            return r#false != 0;
        }
    }
    if g_binary_path.is_null()
        || *g_binary_path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc binary discovery failed:\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if g_discovery_diag.configured_path_present {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"  configured_path='%s' (exists=%s, executable=%s, version=%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut g_discovery_diag.configured_path as *mut ::core::ffi::c_char,
                if g_discovery_diag.configured_path_exists as ::core::ffi::c_int != 0 {
                    b"yes\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"no\0".as_ptr() as *const ::core::ffi::c_char
                },
                if !g_discovery_diag.configured_path_exists {
                    b"n/a\0".as_ptr() as *const ::core::ffi::c_char
                } else if g_discovery_diag.configured_path_executable as ::core::ffi::c_int != 0 {
                    b"yes\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"no\0".as_ptr() as *const ::core::ffi::c_char
                },
                if !g_discovery_diag.configured_path_executable {
                    b"n/a\0".as_ptr() as *const ::core::ffi::c_char
                } else if g_discovery_diag.configured_path_version_ok as ::core::ffi::c_int != 0 {
                    b"ok\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"bad\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"  configured_path='' (none supplied)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"  path_probe_paths_tried=%s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_discovery_diag.probe_paths_tried[0usize] as ::core::ffi::c_int != 0 {
                &raw mut g_discovery_diag.probe_paths_tried as *mut ::core::ffi::c_char
            } else {
                b"<none>\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"  service_check: port_open=%s, http_ok=%s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_discovery_diag.service_port_open as ::core::ffi::c_int != 0 {
                b"yes\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"no\0".as_ptr() as *const ::core::ffi::c_char
            },
            if g_discovery_diag.service_http_ok as ::core::ffi::c_int != 0 {
                b"yes\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"no\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No go2rtc binary available and no running service detected\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if check_tcp_port_open(api_port) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Port %d is already in use\0".as_ptr() as *const ::core::ffi::c_char,
            api_port,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot start go2rtc because port %d is already in use\0".as_ptr()
                as *const ::core::ffi::c_char,
            api_port,
        );
        return r#false != 0;
    }
    g_rtsp_port = if g_config.go2rtc_rtsp_port > 0 as ::core::ffi::c_int {
        g_config.go2rtc_rtsp_port
    } else {
        8554 as ::core::ffi::c_int
    };
    let mut log_path: [::core::ffi::c_char; 4096] = [0; 4096];
    if g_config.log_file[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut log_dir: [::core::ffi::c_char; 4096] = [0; 4096];
        safe_strcpy(
            &raw mut log_dir as *mut ::core::ffi::c_char,
            &raw mut g_config.log_file as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            0 as size_t,
        );
        let mut last_slash: *mut ::core::ffi::c_char = strrchr(
            &raw mut log_dir as *mut ::core::ffi::c_char,
            '/' as ::core::ffi::c_int,
        );
        if !last_slash.is_null() {
            *last_slash.offset(1 as ::core::ffi::c_int as isize) = '\0' as ::core::ffi::c_char;
            snprintf(
                &raw mut log_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                b"%sgo2rtc.log\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut log_dir as *mut ::core::ffi::c_char,
            );
        } else {
            snprintf(
                &raw mut log_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                b"%s/go2rtc.log\0".as_ptr() as *const ::core::ffi::c_char,
                g_config_dir,
            );
        }
    } else {
        snprintf(
            &raw mut log_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            b"%s/go2rtc.log\0".as_ptr() as *const ::core::ffi::c_char,
            g_config_dir,
        );
    }
    let mut resolved_binary: [::core::ffi::c_char; 4096] = [0; 4096];
    if realpath(
        g_binary_path,
        &raw mut resolved_binary as *mut ::core::ffi::c_char,
    )
    .is_null()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to resolve go2rtc binary path '%s': %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            g_binary_path,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    }
    let mut binary_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if stat(
        &raw mut resolved_binary as *mut ::core::ffi::c_char,
        &raw mut binary_stat,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc path no longer exists? %s (%s)\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut resolved_binary as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    }
    if !(binary_stat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc path is not a file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut resolved_binary as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut have_override: bool = !override_path.is_null()
        && *override_path.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && access(override_path, R_OK) == 0 as ::core::ffi::c_int;
    let mut listener_policy: [::core::ffi::c_char; 512] = [0; 512];
    let mut managed_api_port: ::core::ffi::c_int = if api_port > 0 as ::core::ffi::c_int {
        api_port
    } else {
        1984 as ::core::ffi::c_int
    };
    let mut managed_rtsp_port: ::core::ffi::c_int =
        if g_config.go2rtc_rtsp_port > 0 as ::core::ffi::c_int {
            g_config.go2rtc_rtsp_port
        } else {
            8554 as ::core::ffi::c_int
        };
    if managed_api_port > 65535 as ::core::ffi::c_int
        || managed_rtsp_port > 65535 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid managed go2rtc listener port\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    snprintf(
        &raw mut listener_policy as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"{\"api\":{\"listen\":\"127.0.0.1:%d\",\"tls_listen\":\"\",\"unix_listen\":\"\",\"media_authority_port\":%d},\"rtsp\":{\"listen\":\"127.0.0.1:%d\"}}\0"
            .as_ptr() as *const ::core::ffi::c_char,
        managed_api_port,
        g_config.web_port,
        managed_rtsp_port,
    );
    let mut argv: [*mut ::core::ffi::c_char; 8] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 8];
    let mut ai: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let c2rust_fresh3 = ai;
    ai += 1;
    argv[c2rust_fresh3 as usize] =
        b"go2rtc\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let c2rust_fresh4 = ai;
    ai += 1;
    argv[c2rust_fresh4 as usize] =
        b"--config\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let c2rust_fresh5 = ai;
    ai += 1;
    argv[c2rust_fresh5 as usize] = g_config_path;
    if have_override {
        let c2rust_fresh6 = ai;
        ai += 1;
        argv[c2rust_fresh6 as usize] =
            b"--config\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        let c2rust_fresh7 = ai;
        ai += 1;
        argv[c2rust_fresh7 as usize] = override_path as *mut ::core::ffi::c_char;
    }
    let c2rust_fresh8 = ai;
    ai += 1;
    argv[c2rust_fresh8 as usize] =
        b"--config\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let c2rust_fresh9 = ai;
    ai += 1;
    argv[c2rust_fresh9 as usize] = &raw mut listener_policy as *mut ::core::ffi::c_char;
    argv[ai as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if have_override {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Executing go2rtc with command: %s --config %s --config %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut resolved_binary as *mut ::core::ffi::c_char,
            g_config_path,
            override_path,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Executing go2rtc with command: %s --config %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut resolved_binary as *mut ::core::ffi::c_char,
            g_config_path,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Managed go2rtc listener policy (applied last): %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut listener_policy as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Using go2rtc log file: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut log_path as *mut ::core::ffi::c_char,
    );
    let mut pid: pid_t = fork();
    if pid < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to fork process for go2rtc: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    } else if pid == 0 as ::core::ffi::c_int {
        setpgid(0 as __pid_t, 0 as __pid_t);
        let mut sig: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while sig < NSIG {
            signal(sig, SIG_DFL);
            sig += 1;
        }
        let mut empty_mask: sigset_t = sigset_t { __val: [0; 16] };
        sigemptyset(&raw mut empty_mask);
        sigprocmask(
            SIG_SETMASK,
            &raw mut empty_mask,
            ::core::ptr::null_mut::<sigset_t>(),
        );
        let mut log_fd: ::core::ffi::c_int = open(
            &raw mut log_path as *mut ::core::ffi::c_char,
            O_WRONLY | O_CREAT | O_APPEND | O_NOFOLLOW | O_NONBLOCK,
            0o600 as ::core::ffi::c_int,
        );
        if log_fd == -1 as ::core::ffi::c_int {
            _exit(EXIT_FAILURE);
        }
        let mut log_stat: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        if fstat(log_fd, &raw mut log_stat) != 0 as ::core::ffi::c_int
            || !(log_stat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
            || log_stat.st_nlink != 1 as __nlink_t
            || fchmod(log_fd, 0o600 as __mode_t) != 0 as ::core::ffi::c_int
        {
            close(log_fd);
            _exit(EXIT_FAILURE);
        }
        if dup2(log_fd, STDOUT_FILENO) < 0 as ::core::ffi::c_int
            || dup2(log_fd, STDERR_FILENO) < 0 as ::core::ffi::c_int
        {
            close(log_fd);
            _exit(EXIT_FAILURE);
        }
        if log_fd > STDERR_FILENO {
            close(log_fd);
        }
        execv(
            &raw mut resolved_binary as *mut ::core::ffi::c_char,
            &raw mut argv as *mut *mut ::core::ffi::c_char,
        );
        static mut exec_fail_msg: [::core::ffi::c_char; 26] = unsafe {
            ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"Failed to execute go2rtc\n\0",
            )
        };
        let mut exec_fail_written: ssize_t = write(
            STDERR_FILENO,
            &raw const exec_fail_msg as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 26]>().wrapping_sub(1 as size_t),
        );
        _exit(EXIT_FAILURE);
    } else {
        if setpgid(pid, pid) != 0 as ::core::ffi::c_int
            && *__errno_location() != EACCES
            && *__errno_location() != ESRCH
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to set go2rtc child %d process group: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pid,
                strerror(*__errno_location()),
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Launched go2rtc candidate process with PID: %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            pid,
        );
        sleep(1 as ::core::ffi::c_uint);
        if child_exited_and_reaped(pid) as ::core::ffi::c_int != 0
            || kill(pid, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc process %d failed to start\0".as_ptr() as *const ::core::ffi::c_char,
                pid,
            );
            terminate_and_reap_child(pid);
            return r#false != 0;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Waiting for go2rtc API to be ready...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut api_retries: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        let mut api_ready: bool = r#false != 0;
        while api_retries > 0 as ::core::ffi::c_int && !api_ready {
            let mut curl: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut res: CURLcode = CURLcode::CURLE_OK;
            let mut url: [::core::ffi::c_char; 256] = [0; 256];
            let mut http_code: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
            if child_exited_and_reaped(pid) as ::core::ffi::c_int != 0
                || kill(pid, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"go2rtc process %d no longer running\0".as_ptr() as *const ::core::ffi::c_char,
                    pid,
                );
                terminate_and_reap_child(pid);
                return r#false != 0;
            }
            curl = curl_easy_init();
            if curl.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to initialize curl\0".as_ptr() as *const ::core::ffi::c_char,
                );
                sleep(1 as ::core::ffi::c_uint);
                api_retries -= 1;
            } else {
                snprintf(
                    &raw mut url as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    b"http://localhost:%d%s/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
                    api_port,
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
                        discard_response_data
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
                res = curl_easy_perform(curl);
                if res.0 != CURLcode::CURLE_OK.0 {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Curl request failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        curl_easy_strerror(res),
                    );
                    curl_easy_cleanup(curl);
                    sleep(1 as ::core::ffi::c_uint);
                    api_retries -= 1;
                } else {
                    curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut http_code);
                    curl_easy_cleanup(curl);
                    if http_code == 200 as ::core::ffi::c_long
                        || http_code == 401 as ::core::ffi::c_long
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"go2rtc API is ready (HTTP %ld)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            http_code,
                        );
                        api_ready = r#true != 0;
                        break;
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Waiting for go2rtc API to be ready... (%d retries left)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            api_retries,
                        );
                        sleep(1 as ::core::ffi::c_uint);
                        api_retries -= 1;
                    }
                }
            }
        }
        if !api_ready {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc API did not become ready; rejecting candidate PID %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pid,
            );
            terminate_and_reap_child(pid);
            return r#false != 0;
        }
        if !process_owns_tcp_listen_port(pid, api_port) {
            let mut actual_serving_pid: pid_t = -1 as pid_t;
            find_go2rtc_serving_pid(api_port, &raw mut actual_serving_pid);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"go2rtc candidate PID %d does not own API port %d (serving PID: %d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pid,
                api_port,
                actual_serving_pid,
            );
            terminate_and_reap_child(pid);
            return r#false != 0;
        }
        let mut start_ticks: ::core::ffi::c_ulonglong = process_start_ticks(pid);
        if start_ticks == 0 as ::core::ffi::c_ulonglong {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Could not capture stable identity for go2rtc candidate PID %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pid,
            );
            terminate_and_reap_child(pid);
            return r#false != 0;
        }
        g_process_pid = pid;
        g_process_start_ticks = start_ticks;
        g_using_external_service = r#false != 0;
        strcpy(
            &raw mut g_managed_listener_policy as *mut ::core::ffi::c_char,
            &raw mut listener_policy as *mut ::core::ffi::c_char,
        );
        g_last_start_time = time(::core::ptr::null_mut::<time_t>());
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Verified go2rtc serving PID: %d\0".as_ptr() as *const ::core::ffi::c_char,
            pid,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Attempting to retrieve RTSP port from go2rtc API...\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut retries_1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        let mut got_rtsp_port_1: bool = r#false != 0;
        while retries_1 > 0 as ::core::ffi::c_int && !got_rtsp_port_1 {
            if go2rtc_api_get_server_info(&raw mut g_rtsp_port) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Retrieved RTSP port from go2rtc API: %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    g_rtsp_port,
                );
                got_rtsp_port_1 = r#true != 0;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Could not retrieve RTSP port from go2rtc API, retrying... (%d retries left)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    retries_1,
                );
                sleep(1 as ::core::ffi::c_uint);
                retries_1 -= 1;
            }
        }
        if !got_rtsp_port_1 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Could not retrieve RTSP port from go2rtc API after multiple attempts, using default: %d\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                g_rtsp_port,
            );
        }
        return r#true != 0;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_start(mut api_port: ::core::ffi::c_int) -> bool {
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
            b"Failed to enter go2rtc lifecycle for process start\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if guard.coalesced {
        return guard.result;
    }
    let mut result: bool = go2rtc_process_start_locked(api_port);
    go2rtc_lifecycle_end(&raw mut guard, result);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_get_rtsp_port() -> ::core::ffi::c_int {
    return g_rtsp_port;
}
unsafe extern "C" fn go2rtc_process_stop_locked() -> bool {
    if !g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"go2rtc process manager not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !g_using_external_service
        && !g_binary_path.is_null()
        && *g_binary_path.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        let mut pid: pid_t = g_process_pid;
        if pid <= 0 as ::core::ffi::c_int {
            g_process_start_ticks = 0 as ::core::ffi::c_ulonglong;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"No verified managed go2rtc PID to stop\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#true != 0;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopping verified managed go2rtc PID %d\0".as_ptr() as *const ::core::ffi::c_char,
            pid,
        );
        let mut result: bool = r#true != 0;
        if managed_process_identity_matches(pid) {
            g_last_stop_was_managed = !is_zombie_process(pid);
            signal_managed_process(pid, SIGTERM);
            if !wait_for_process_termination(pid, 3000 as ::core::ffi::c_int) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Managed go2rtc PID %d did not stop on SIGTERM; sending SIGKILL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pid,
                );
                signal_managed_process(pid, SIGKILL);
                result = wait_for_process_termination(pid, 1000 as ::core::ffi::c_int);
            }
        } else {
            g_last_stop_was_managed = r#false != 0;
            waitpid(pid, ::core::ptr::null_mut::<::core::ffi::c_int>(), WNOHANG);
        }
        g_process_pid = -1 as ::core::ffi::c_int as pid_t;
        g_process_start_ticks = 0 as ::core::ffi::c_ulonglong;
        if result {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stopped and reaped managed go2rtc PID %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pid,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Managed go2rtc PID %d may still be running\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pid,
            );
        }
        return result;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Not stopping go2rtc as we're using an existing service\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_stop() -> bool {
    let mut guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    if !go2rtc_lifecycle_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_PROCESS_STOP,
        r#false != 0,
        r#true != 0,
        &raw mut guard,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to enter go2rtc lifecycle for process stop\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut result: bool = go2rtc_process_stop_locked();
    go2rtc_lifecycle_end(&raw mut guard, result);
    return result;
}
unsafe extern "C" fn go2rtc_process_cleanup_locked() {
    if !g_initialized {
        return;
    }
    if !g_using_external_service
        && !g_binary_path.is_null()
        && *g_binary_path.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopping go2rtc process that we started during cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        go2rtc_process_stop_locked();
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Not stopping go2rtc during cleanup as we're using an existing service\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    free(g_binary_path as *mut ::core::ffi::c_void);
    free(g_config_dir as *mut ::core::ffi::c_void);
    free(g_config_path as *mut ::core::ffi::c_void);
    free(g_override_path as *mut ::core::ffi::c_void);
    free(g_override_quarantined_path as *mut ::core::ffi::c_void);
    g_binary_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_config_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_config_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_override_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_override_quarantined_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_process_pid = -1 as ::core::ffi::c_int as pid_t;
    g_process_start_ticks = 0 as ::core::ffi::c_ulonglong;
    g_using_external_service = r#false != 0;
    g_managed_listener_policy[0usize] = '\0' as ::core::ffi::c_char;
    g_api_port = 1984 as ::core::ffi::c_int;
    g_initialized = r#false != 0;
    g_last_start_time = 0 as time_t;
    g_last_stop_was_managed = r#false != 0;
    memset(
        &raw mut g_fast_death_history as *mut time_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[time_t; 8]>(),
    );
    g_fast_death_history_idx = 0 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"go2rtc process manager cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_cleanup() {
    let mut guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    if !go2rtc_lifecycle_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_CLEANUP,
        r#false != 0,
        r#true != 0,
        &raw mut guard,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to enter go2rtc lifecycle for process cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    go2rtc_process_cleanup_locked();
    go2rtc_lifecycle_end(&raw mut guard, r#true != 0);
}
unsafe extern "C" fn go2rtc_process_get_pid_locked() -> ::core::ffi::c_int {
    if g_process_pid > 0 as ::core::ffi::c_int {
        let mut identity_matches: bool = managed_process_identity_matches(g_process_pid);
        if identity_matches as ::core::ffi::c_int != 0
            && process_owns_tcp_listen_port(g_process_pid, g_api_port) as ::core::ffi::c_int != 0
        {
            return g_process_pid;
        }
        if !identity_matches || is_zombie_process(g_process_pid) as ::core::ffi::c_int != 0 {
            child_exited_and_reaped(g_process_pid);
            g_process_pid = -1 as ::core::ffi::c_int as pid_t;
            g_process_start_ticks = 0 as ::core::ffi::c_ulonglong;
        }
    }
    if g_using_external_service {
        let mut serving_pid: pid_t = -1 as pid_t;
        if find_go2rtc_serving_pid(g_api_port, &raw mut serving_pid) {
            return serving_pid;
        }
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_get_pid() -> ::core::ffi::c_int {
    let mut guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    if !go2rtc_lifecycle_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_CHECK,
        r#false != 0,
        r#false != 0,
        &raw mut guard,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    let mut pid: ::core::ffi::c_int = go2rtc_process_get_pid_locked();
    go2rtc_lifecycle_end(&raw mut guard, pid > 0 as ::core::ffi::c_int);
    return pid;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_process_try_get_pid(mut pid: *mut ::core::ffi::c_int) -> bool {
    if pid.is_null() {
        return r#false != 0;
    }
    *pid = -1 as ::core::ffi::c_int;
    let mut guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    if !go2rtc_lifecycle_try_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_CHECK,
        r#false != 0,
        &raw mut guard,
    ) {
        return r#false != 0;
    }
    *pid = go2rtc_process_get_pid_locked();
    go2rtc_lifecycle_end(&raw mut guard, *pid > 0 as ::core::ffi::c_int);
    return r#true != 0;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const WNOHANG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGKILL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const _NSIG: ::core::ffi::c_int = __SIGRTMAX + 1 as ::core::ffi::c_int;
pub const __SIGRTMAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
