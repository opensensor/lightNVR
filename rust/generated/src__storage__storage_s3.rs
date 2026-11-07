use ::c2rust_bitfields;
use ::libc;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Curl_URL;
    pub type sqlite3_stmt;
    pub type sqlite3;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_DeleteItemFromObjectCaseSensitive(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
    );
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
    unsafe fn cJSON_AddArrayToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn tmpfile() -> *mut FILE;
    unsafe fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    unsafe fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn rewind(__stream: *mut FILE);
    unsafe fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn ftello(__stream: *mut FILE) -> __off_t;
    unsafe fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn curl_easy_escape(
        curl: *mut ::core::ffi::c_void,
        string: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn curl_free(p: *mut ::core::ffi::c_void);
    unsafe fn curl_slist_append(
        list: *mut curl_slist,
        data: *const ::core::ffi::c_char,
    ) -> *mut curl_slist;
    unsafe fn curl_slist_free_all(list: *mut curl_slist);
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
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...)
    -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn openat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_sha256_init(ctx: *mut mbedtls_sha256_context);
    unsafe fn mbedtls_sha256_free(ctx: *mut mbedtls_sha256_context);
    unsafe fn mbedtls_sha256_starts(
        ctx: *mut mbedtls_sha256_context,
        is224: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_sha256_update(
        ctx: *mut mbedtls_sha256_context,
        input: *const ::core::ffi::c_uchar,
        ilen: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_sha256_finish(
        ctx: *mut mbedtls_sha256_context,
        output: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn geteuid() -> __uid_t;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn ezxml_parse_str(s: *mut ::core::ffi::c_char, len: size_t) -> ezxml_t;
    unsafe fn ezxml_child(xml: ezxml_t, name: *const ::core::ffi::c_char) -> ezxml_t;
    unsafe fn ezxml_free(xml: ezxml_t);
    unsafe fn ezxml_error(xml: ezxml_t) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
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
    unsafe fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
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
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = isize;
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
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_target_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub target_type: [::core::ffi::c_char; 24],
    pub root_path: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub is_default: bool,
    pub storage_class: [::core::ffi::c_char; 16],
    pub reserve_bytes: uint64_t,
    pub high_watermark_pct: ::core::ffi::c_double,
    pub low_watermark_pct: ::core::ffi::c_double,
    pub health_status: [::core::ffi::c_char; 16],
    pub capacity_bytes: uint64_t,
    pub available_bytes: uint64_t,
    pub filesystem_device: uint64_t,
    pub last_probe_at: int64_t,
    pub last_success_at: int64_t,
    pub last_error: [::core::ffi::c_char; 256],
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub recording_count: uint64_t,
    pub recording_bytes: uint64_t,
    pub mount_required: bool,
    pub mount_guard_path: [::core::ffi::c_char; 512],
    pub migration_bandwidth_bps: uint64_t,
    pub archival_window_start_minute: ::core::ffi::c_int,
    pub archival_window_end_minute: ::core::ffi::c_int,
    pub replica_count: uint64_t,
    pub replica_bytes: uint64_t,
    pub endpoint: [::core::ffi::c_char; 512],
    pub region: [::core::ffi::c_char; 64],
    pub bucket: [::core::ffi::c_char; 128],
    pub credential_ref: [::core::ffi::c_char; 128],
    pub archive_budget_bytes: uint64_t,
}
pub type storage_transfer_cancel_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_transfer_control_t {
    pub cancelled: storage_transfer_cancel_fn,
    pub context: *mut ::core::ffi::c_void,
    pub bandwidth_bps: uint64_t,
    pub job_uuid: *const ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const STORAGE_S3_OK: Self = Self(0);
    pub const STORAGE_S3_ERROR: Self = Self(-1);
    pub const STORAGE_S3_MISSING: Self = Self(-2);
    pub const STORAGE_S3_CANCELLED: Self = Self(-3);
    pub const STORAGE_S3_CONFLICT: Self = Self(-4);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s3_io_t {
    pub body: *mut ::core::ffi::c_char,
    pub length: size_t,
    pub file: *mut FILE,
    pub digest: *mut mbedtls_sha256_context,
    pub bytes: uint64_t,
    pub limit: uint64_t,
    pub control: *const storage_transfer_control_t,
    pub status: ::core::ffi::c_long,
    pub etag: [::core::ffi::c_char; 192],
    pub range: [::core::ffi::c_char; 64],
    pub content_range: [::core::ffi::c_char; 128],
    pub data: *mut ::core::ffi::c_uchar,
    pub anonymous: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_sha256_context {
    pub private_buffer: [::core::ffi::c_uchar; 64],
    pub private_total: [uint32_t; 2],
    pub private_state: [uint32_t; 8],
    pub private_is224: ::core::ffi::c_int,
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
pub type CURL = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s3_credentials_t {
    pub access: [::core::ffi::c_char; 256],
    pub secret: [::core::ffi::c_char; 256],
    pub token: [::core::ffi::c_char; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_slist {
    pub data: *mut ::core::ffi::c_char,
    pub next: *mut curl_slist,
}
pub type curl_off_t = ::core::ffi::c_long;
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
pub type cJSON_bool = ::core::ffi::c_int;
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
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const STORAGE_TARGET_ERROR_MAX: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const F_DUPFD_CLOEXEC: ::core::ffi::c_int = 1030 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_path_open_directory_at(
    mut parent: ::core::ffi::c_int,
    mut relative: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if relative.is_null()
        || *relative.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
    {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
    let mut directory: ::core::ffi::c_int = fcntl(parent, F_DUPFD_CLOEXEC, 0 as ::core::ffi::c_int);
    if directory < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    while *relative != 0 {
        let mut component: [::core::ffi::c_char; 256] = [0; 256];
        let mut length: size_t =
            strcspn(relative, b"/\0".as_ptr() as *const ::core::ffi::c_char) as size_t;
        if length == 0 as size_t || length >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>() {
            close(directory);
            *__errno_location() = EINVAL;
            return -1 as ::core::ffi::c_int;
        }
        memcpy(
            &raw mut component as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            relative as *const ::core::ffi::c_void,
            length,
        );
        component[length] = '\0' as ::core::ffi::c_char;
        if !strchr(
            &raw mut component as *mut ::core::ffi::c_char,
            '/' as ::core::ffi::c_int,
        )
        .is_null()
            || strcmp(
                &raw mut component as *mut ::core::ffi::c_char,
                b".\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut component as *mut ::core::ffi::c_char,
                b"..\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            close(directory);
            *__errno_location() = EINVAL;
            return -1 as ::core::ffi::c_int;
        }
        let mut next: ::core::ffi::c_int = openat(
            directory,
            &raw mut component as *mut ::core::ffi::c_char,
            O_RDONLY | O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW,
        );
        let mut saved_errno: ::core::ffi::c_int = *__errno_location();
        close(directory);
        if next < 0 as ::core::ffi::c_int {
            *__errno_location() = saved_errno;
            return -1 as ::core::ffi::c_int;
        }
        directory = next;
        relative = relative.offset(length as isize);
        if *relative as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            relative = relative.offset(1);
        }
    }
    return directory;
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const S3_REPLY_LIMIT: ::core::ffi::c_uint =
    (128 as ::core::ffi::c_uint).wrapping_mul(1024 as ::core::ffi::c_uint);
pub const S3_SECRET_LIMIT: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
unsafe extern "C" fn error_set(
    mut error: *mut ::core::ffi::c_char,
    mut message: *const ::core::ffi::c_char,
) {
    safe_strcpy(
        error,
        message,
        STORAGE_TARGET_ERROR_MAX as size_t,
        0 as size_t,
    );
}
unsafe extern "C" fn wipe(mut pointer: *mut ::core::ffi::c_void, mut length: size_t) {
    let mut bytes: *mut ::core::ffi::c_uchar = pointer as *mut ::core::ffi::c_uchar;
    loop {
        let c2rust_fresh3 = length;
        length = length.wrapping_sub(1);
        if c2rust_fresh3 == 0 {
            break;
        }
        let c2rust_fresh4 = bytes;
        bytes = bytes.offset(1);
        ::core::ptr::write_volatile(c2rust_fresh4, 0 as ::core::ffi::c_uchar);
    }
}
unsafe extern "C" fn safe_identifier(
    mut value: *const ::core::ffi::c_char,
    mut maximum: size_t,
    mut dots: bool,
) -> bool {
    let mut length: size_t = if !value.is_null() {
        strnlen(value, maximum)
    } else {
        0 as size_t
    };
    if length == 0 || length >= maximum {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < length {
        let mut c: ::core::ffi::c_uchar = *value.offset(i as isize) as ::core::ffi::c_uchar;
        if !(*(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed_0::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            || c as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '_' as ::core::ffi::c_int
            || dots as ::core::ffi::c_int != 0
                && c as ::core::ffi::c_int == '.' as ::core::ffi::c_int)
        {
            return r#false != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn safe_key(mut key: *const ::core::ffi::c_char, mut empty: bool) -> bool {
    if key.is_null()
        || !empty && *key == 0
        || *key as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        || strlen(key) >= MAX_PATH_LENGTH as size_t
    {
        return r#false != 0;
    }
    let mut segment: *const ::core::ffi::c_char = key;
    let mut p: *const ::core::ffi::c_char = key;
    loop {
        if *p as ::core::ffi::c_int != 0
            && ((*p as ::core::ffi::c_uchar as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
                || *p as ::core::ffi::c_uchar as ::core::ffi::c_int == 127 as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\\' as ::core::ffi::c_int)
        {
            return r#false != 0;
        }
        if *p as ::core::ffi::c_int == '/' as ::core::ffi::c_int || *p == 0 {
            let mut n: size_t = p.offset_from(segment) as size_t;
            if n == 1 as size_t
                && *segment.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                || n == 2 as size_t
                    && strncmp(
                        segment,
                        b"..\0".as_ptr() as *const ::core::ffi::c_char,
                        2 as size_t,
                    ) == 0
            {
                return r#false != 0;
            }
            segment = p.offset(1 as ::core::ffi::c_int as isize);
        }
        if *p == 0 {
            break;
        }
        p = p.offset(1);
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_s3_validate(
    mut target: *const storage_target_t,
    mut error: *mut ::core::ffi::c_char,
    mut length: size_t,
) -> bool {
    let mut message: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if target.is_null()
        || !safe_identifier(
            &raw const (*target).bucket as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            r#true != 0,
        )
        || !safe_identifier(
            &raw const (*target).region as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            r#false != 0,
        )
        || !safe_identifier(
            &raw const (*target).credential_ref as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            r#false != 0,
        )
    {
        message = b"S3 bucket, region and credential reference are required identifiers\0".as_ptr()
            as *const ::core::ffi::c_char;
    } else {
        let mut url: *mut CURLU = curl_url();
        let mut scheme: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut host: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut user: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut query: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut fragment: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut valid: bool = !url.is_null()
            && curl_url_set(
                url,
                CURLUPart::CURLUPART_URL,
                &raw const (*target).endpoint as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_uint,
            )
            .0 == CURLUcode::CURLUE_OK.0;
        if valid {
            curl_url_get(
                url,
                CURLUPart::CURLUPART_SCHEME,
                &raw mut scheme,
                0 as ::core::ffi::c_uint,
            );
            curl_url_get(
                url,
                CURLUPart::CURLUPART_HOST,
                &raw mut host,
                0 as ::core::ffi::c_uint,
            );
            curl_url_get(
                url,
                CURLUPart::CURLUPART_USER,
                &raw mut user,
                0 as ::core::ffi::c_uint,
            );
            curl_url_get(
                url,
                CURLUPart::CURLUPART_QUERY,
                &raw mut query,
                0 as ::core::ffi::c_uint,
            );
            curl_url_get(
                url,
                CURLUPart::CURLUPART_FRAGMENT,
                &raw mut fragment,
                0 as ::core::ffi::c_uint,
            );
            curl_url_get(
                url,
                CURLUPart::CURLUPART_PATH,
                &raw mut path,
                0 as ::core::ffi::c_uint,
            );
            let mut allow_http: *const ::core::ffi::c_char =
                getenv(b"LIGHTNVR_ARCHIVE_ALLOW_HTTP\0".as_ptr() as *const ::core::ffi::c_char);
            valid = !scheme.is_null()
                && !host.is_null()
                && *host as ::core::ffi::c_int != 0
                && user.is_null()
                && query.is_null()
                && fragment.is_null()
                && (path.is_null()
                    || strcmp(path, b"/\0".as_ptr() as *const ::core::ffi::c_char) == 0)
                && (strcmp(scheme, b"https\0".as_ptr() as *const ::core::ffi::c_char) == 0
                    || strcmp(scheme, b"http\0".as_ptr() as *const ::core::ffi::c_char) == 0
                        && !allow_http.is_null()
                        && strcmp(allow_http, b"1\0".as_ptr() as *const ::core::ffi::c_char) == 0);
        }
        curl_free(scheme as *mut ::core::ffi::c_void);
        curl_free(host as *mut ::core::ffi::c_void);
        curl_free(user as *mut ::core::ffi::c_void);
        curl_free(query as *mut ::core::ffi::c_void);
        curl_free(fragment as *mut ::core::ffi::c_void);
        curl_free(path as *mut ::core::ffi::c_void);
        if !url.is_null() {
            curl_url_cleanup(url);
        }
        if !valid {
            message = b"S3 endpoint must be an HTTPS origin without credentials, query or path\0"
                .as_ptr() as *const ::core::ffi::c_char;
        }
        let mut prefix: [::core::ffi::c_char; 160] = [0; 160];
        snprintf(
            &raw mut prefix as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
            b"s3://%s/\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*target).bucket as *const ::core::ffi::c_char,
        );
        if message.is_null()
            && (strncmp(
                &raw const (*target).root_path as *const ::core::ffi::c_char,
                &raw mut prefix as *mut ::core::ffi::c_char,
                strlen(&raw mut prefix as *mut ::core::ffi::c_char),
            ) != 0
                || !safe_key(
                    (&raw const (*target).root_path as *const ::core::ffi::c_char)
                        .offset(strlen(&raw mut prefix as *mut ::core::ffi::c_char) as isize),
                    r#true != 0,
                ))
        {
            message = b"S3 root_path must be s3://bucket/managed-prefix without traversal\0"
                .as_ptr() as *const ::core::ffi::c_char;
        }
        if message.is_null()
            && ((*target).is_default as ::core::ffi::c_int != 0
                || (*target).mount_required as ::core::ffi::c_int != 0)
        {
            message = b"Object storage is an archive target and cannot be the capture default or a mount\0"
                .as_ptr() as *const ::core::ffi::c_char;
        }
    }
    if !message.is_null() && !error.is_null() && length != 0 {
        safe_strcpy(error, message, length, 0 as size_t);
    }
    return message.is_null();
}
unsafe extern "C" fn safe_credentials_directory(mut directory: *const ::core::ffi::c_char) -> bool {
    let mut total: size_t = if !directory.is_null() {
        strlen(directory)
    } else {
        0 as size_t
    };
    if total < 2 as size_t
        || total >= PATH_MAX as size_t
        || *directory.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = directory as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if (*cursor as ::core::ffi::c_uint) < 0x20 as ::core::ffi::c_uint
            || *cursor as ::core::ffi::c_uint == 0x7f as ::core::ffi::c_uint
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    let mut component: *const ::core::ffi::c_char =
        directory.offset(1 as ::core::ffi::c_int as isize);
    loop {
        let mut length: size_t =
            strcspn(component, b"/\0".as_ptr() as *const ::core::ffi::c_char) as size_t;
        if length == 0 as size_t
            && *component.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        if length == 1 as size_t
            && *component.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || length == 2 as size_t
                && *component.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && *component.offset(1isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        if *component.offset(length as isize) == 0 {
            break;
        }
        component = component.offset(length.wrapping_add(1 as size_t) as isize);
    }
    return r#true != 0;
}
unsafe extern "C" fn private_json_read(
    mut target: *const storage_target_t,
    mut suffix: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut directory: *const ::core::ffi::c_char =
        getenv(b"LIGHTNVR_ARCHIVE_CREDENTIALS_DIR\0".as_ptr() as *const ::core::ffi::c_char);
    if directory.is_null() {
        directory = b"/etc/lightnvr/archive-credentials\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if !safe_credentials_directory(directory) {
        return ::core::ptr::null_mut::<cJSON>();
    }
    if !safe_identifier(
        &raw const (*target).credential_ref as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        r#false != 0,
    ) {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut filename: [::core::ffi::c_char; 160] = [0; 160];
    snprintf(
        &raw mut filename as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
        b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).credential_ref as *const ::core::ffi::c_char,
        suffix,
    );
    let mut filesystem_root: ::core::ffi::c_int = open(
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
        O_RDONLY | O_DIRECTORY | O_CLOEXEC,
    );
    if filesystem_root < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut dir: ::core::ffi::c_int = safe_path_open_directory_at(
        filesystem_root,
        directory.offset(1 as ::core::ffi::c_int as isize),
    );
    close(filesystem_root);
    if dir < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut descriptor: ::core::ffi::c_int = openat(
        dir,
        &raw mut filename as *mut ::core::ffi::c_char,
        O_RDONLY | O_CLOEXEC | O_NOFOLLOW | O_NONBLOCK,
    );
    close(dir);
    if descriptor < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut status: stat = stat {
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
    let mut buffer: [::core::ffi::c_char; 8193] = [0; 8193];
    let mut valid: bool = fstat(descriptor, &raw mut status) == 0 as ::core::ffi::c_int
        && status.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
        && status.st_mode & 0o77 as __mode_t == 0
        && (status.st_uid == 0 as __uid_t || status.st_uid == geteuid())
        && status.st_size > 0 as __off_t
        && status.st_size <= S3_SECRET_LIMIT as __off_t;
    let mut count: ssize_t = if valid as ::core::ffi::c_int != 0 {
        read(
            descriptor,
            &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 8193]>().wrapping_sub(1 as size_t),
        )
    } else {
        -1 as ssize_t
    };
    close(descriptor);
    if count <= 0 as ssize_t || count != status.st_size as ssize_t {
        return ::core::ptr::null_mut::<cJSON>();
    }
    buffer[count as usize] = 0 as ::core::ffi::c_char;
    let mut json: *mut cJSON = cJSON_Parse(&raw mut buffer as *mut ::core::ffi::c_char);
    wipe(
        &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 8193]>(),
    );
    return json;
}
unsafe extern "C" fn credentials_read(
    mut target: *const storage_target_t,
    mut credentials: *mut s3_credentials_t,
) -> ::core::ffi::c_int {
    let mut json: *mut cJSON =
        private_json_read(target, b"\0".as_ptr() as *const ::core::ffi::c_char);
    let mut access: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        json,
        b"access_key_id\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut secret: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        json,
        b"secret_access_key\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut token: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        json,
        b"session_token\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut valid: bool = cJSON_IsString(access) != 0
        && cJSON_IsString(secret) != 0
        && *(*access).valuestring.offset(0isize) as ::core::ffi::c_int != 0
        && *(*secret).valuestring.offset(0isize) as ::core::ffi::c_int != 0
        && strlen((*access).valuestring) < ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
        && strlen((*secret).valuestring) < ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
        && (token.is_null()
            || cJSON_IsString(token) != 0
                && strlen((*token).valuestring)
                    < ::core::mem::size_of::<[::core::ffi::c_char; 4096]>());
    if valid {
        safe_strcpy(
            &raw mut (*credentials).access as *mut ::core::ffi::c_char,
            (*access).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*credentials).secret as *mut ::core::ffi::c_char,
            (*secret).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        if !token.is_null() {
            safe_strcpy(
                &raw mut (*credentials).token as *mut ::core::ffi::c_char,
                (*token).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                0 as size_t,
            );
        }
        let mut p: *const ::core::ffi::c_char =
            &raw mut (*credentials).token as *mut ::core::ffi::c_char;
        while *p != 0 {
            if (*p as ::core::ffi::c_uchar as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == 127 as ::core::ffi::c_int
            {
                valid = r#false != 0;
            }
            p = p.offset(1);
        }
    }
    if cJSON_IsString(secret) != 0 {
        wipe(
            (*secret).valuestring as *mut ::core::ffi::c_void,
            strlen((*secret).valuestring),
        );
    }
    if cJSON_IsString(token) != 0 {
        wipe(
            (*token).valuestring as *mut ::core::ffi::c_void,
            strlen((*token).valuestring),
        );
    }
    cJSON_Delete(json);
    return if valid as ::core::ffi::c_int != 0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn response_header(
    mut buffer: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut count: size_t,
    mut context: *mut ::core::ffi::c_void,
) -> size_t {
    let mut io: *mut s3_io_t = context as *mut s3_io_t;
    let mut length: size_t = size.wrapping_mul(count);
    if length > 12 as size_t
        && memcmp(
            buffer as *const ::core::ffi::c_void,
            b"HTTP/\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            5 as size_t,
        ) == 0
    {
        let mut space: *const ::core::ffi::c_char = memchr(
            buffer as *const ::core::ffi::c_void,
            ' ' as ::core::ffi::c_int,
            length,
        ) as *const ::core::ffi::c_char;
        if !space.is_null()
            && space.offset(4 as ::core::ffi::c_int as isize) <= buffer.offset(length as isize)
        {
            (*io).status = ((*space.offset(1isize) as ::core::ffi::c_int
                - '0' as ::core::ffi::c_int)
                * 100 as ::core::ffi::c_int
                + (*space.offset(2isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                    * 10 as ::core::ffi::c_int
                + *space.offset(3isize) as ::core::ffi::c_int
                - '0' as ::core::ffi::c_int) as ::core::ffi::c_long;
        }
    }
    if length > 5 as size_t
        && strncasecmp(
            buffer,
            b"ETag:\0".as_ptr() as *const ::core::ffi::c_char,
            5 as size_t,
        ) == 0
    {
        let mut begin: size_t = 5 as size_t;
        let mut end: size_t = length;
        while begin < end
            && *(*__ctype_b_loc()).offset(*buffer.offset(begin as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            begin = begin.wrapping_add(1);
        }
        while end > begin
            && *(*__ctype_b_loc())
                .offset(*buffer.offset(end.wrapping_sub(1 as size_t) as isize)
                    as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            end = end.wrapping_sub(1);
        }
        if end.wrapping_sub(begin) < ::core::mem::size_of::<[::core::ffi::c_char; 192]>() {
            memcpy(
                &raw mut (*io).etag as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                buffer.offset(begin as isize) as *const ::core::ffi::c_void,
                end.wrapping_sub(begin),
            );
            (*io).etag[end.wrapping_sub(begin)] = 0 as ::core::ffi::c_char;
        }
    }
    if length > 14 as size_t
        && strncasecmp(
            buffer,
            b"Content-Range:\0".as_ptr() as *const ::core::ffi::c_char,
            14 as size_t,
        ) == 0
    {
        let mut begin_0: size_t = 14 as size_t;
        let mut end_0: size_t = length;
        while begin_0 < end_0
            && *(*__ctype_b_loc()).offset(*buffer.offset(begin_0 as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            begin_0 = begin_0.wrapping_add(1);
        }
        while end_0 > begin_0
            && *(*__ctype_b_loc())
                .offset(*buffer.offset(end_0.wrapping_sub(1 as size_t) as isize)
                    as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            end_0 = end_0.wrapping_sub(1);
        }
        if end_0.wrapping_sub(begin_0) < ::core::mem::size_of::<[::core::ffi::c_char; 128]>() {
            memcpy(
                &raw mut (*io).content_range as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                buffer.offset(begin_0 as isize) as *const ::core::ffi::c_void,
                end_0.wrapping_sub(begin_0),
            );
            (*io).content_range[end_0.wrapping_sub(begin_0)] = 0 as ::core::ffi::c_char;
        }
    }
    return length;
}
unsafe extern "C" fn receive_bytes(
    mut buffer: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut count: size_t,
    mut context: *mut ::core::ffi::c_void,
) -> size_t {
    let mut io: *mut s3_io_t = context as *mut s3_io_t;
    if size != 0 && count > (SIZE_MAX as size_t).wrapping_div(size) {
        return 0 as size_t;
    }
    let mut length: size_t = size.wrapping_mul(count);
    if (*io).status < 200 as ::core::ffi::c_long || (*io).status >= 300 as ::core::ffi::c_long {
        return length;
    }
    if (*io).bytes > (*io).limit
        || length > ((*io).limit as size_t).wrapping_sub((*io).bytes as size_t)
    {
        return 0 as size_t;
    }
    if !(*io).data.is_null() {
        memcpy(
            (*io).data.offset((*io).bytes as isize) as *mut ::core::ffi::c_void,
            buffer as *const ::core::ffi::c_void,
            length,
        );
    }
    if !(*io).file.is_null()
        && fwrite(
            buffer as *const ::core::ffi::c_void,
            1 as size_t,
            length,
            (*io).file,
        ) as size_t
            != length
    {
        return 0 as size_t;
    }
    if !(*io).digest.is_null()
        && mbedtls_sha256_update((*io).digest, buffer as *mut ::core::ffi::c_uchar, length)
            != 0 as ::core::ffi::c_int
    {
        return 0 as size_t;
    }
    if !(*io).body.is_null() {
        if (*io).length.wrapping_add(length) >= S3_REPLY_LIMIT as size_t {
            return 0 as size_t;
        }
        memcpy(
            (*io).body.offset((*io).length as isize) as *mut ::core::ffi::c_void,
            buffer as *const ::core::ffi::c_void,
            length,
        );
        (*io).length = (*io).length.wrapping_add(length);
        *(*io).body.offset((*io).length as isize) = 0 as ::core::ffi::c_char;
    }
    (*io).bytes = ((*io).bytes as ::core::ffi::c_ulong).wrapping_add(length as ::core::ffi::c_ulong)
        as uint64_t;
    return length;
}
unsafe extern "C" fn progress(
    mut context: *mut ::core::ffi::c_void,
    mut dt: curl_off_t,
    mut dn: curl_off_t,
    mut ut: curl_off_t,
    mut un: curl_off_t,
) -> ::core::ffi::c_int {
    let mut control: *const storage_transfer_control_t =
        context as *const storage_transfer_control_t;
    return if !control.is_null()
        && (*control).cancelled.is_some()
        && (*control).cancelled.expect("non-null function pointer")((*control).context)
            as ::core::ffi::c_int
            != 0
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn request(
    mut target: *const storage_target_t,
    mut key: *const ::core::ffi::c_char,
    mut query: *const ::core::ffi::c_char,
    mut method: *const ::core::ffi::c_char,
    mut upload: *mut FILE,
    mut upload_size: uint64_t,
    mut checksum: *const ::core::ffi::c_char,
    mut io: *mut s3_io_t,
    mut object_size: *mut uint64_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !storage_s3_validate(target, error, STORAGE_TARGET_ERROR_MAX as size_t)
        || !key.is_null() && !safe_key(key, r#false != 0)
    {
        return C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    let mut credentials: s3_credentials_t = s3_credentials_t {
        access: [
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
        secret: [0; 256],
        token: [0; 4096],
    };
    if !(*io).anonymous && credentials_read(target, &raw mut credentials) != 0 {
        error_set(
            error,
            b"Archive credential file is missing, invalid or has unsafe permissions\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        wipe(
            &raw mut credentials as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<s3_credentials_t>(),
        );
        return C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    let mut curl: *mut ::core::ffi::c_void = curl_easy_init();
    let mut raw_key: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut root_prefix: [::core::ffi::c_char; 160] = [0; 160];
    snprintf(
        &raw mut root_prefix as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
        b"s3://%s/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).bucket as *const ::core::ffi::c_char,
    );
    let mut prefix: *const ::core::ffi::c_char = (&raw const (*target).root_path
        as *const ::core::ffi::c_char)
        .offset(strlen(&raw mut root_prefix as *mut ::core::ffi::c_char) as isize);
    snprintf(
        &raw mut raw_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        if !key.is_null() {
            prefix
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !key.is_null()
            && *prefix as ::core::ffi::c_int != 0
            && *prefix.offset(strlen(prefix).wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int
                != '/' as ::core::ffi::c_int
        {
            b"/\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !key.is_null() {
            key
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut encoded: *mut ::core::ffi::c_char = if !curl.is_null() {
        curl_easy_escape(
            curl,
            &raw mut raw_key as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        )
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    if !encoded.is_null() {
        let mut read_0: *mut ::core::ffi::c_char = encoded;
        let mut write: *mut ::core::ffi::c_char = encoded;
        while *read_0 != 0 {
            if strncmp(
                read_0,
                b"%2F\0".as_ptr() as *const ::core::ffi::c_char,
                3 as size_t,
            ) == 0
            {
                let c2rust_fresh0 = write;
                write = write.offset(1);
                *c2rust_fresh0 = '/' as ::core::ffi::c_char;
                read_0 = read_0.offset(3 as ::core::ffi::c_int as isize);
            } else {
                let c2rust_fresh1 = read_0;
                read_0 = read_0.offset(1);
                let c2rust_fresh2 = write;
                write = write.offset(1);
                *c2rust_fresh2 = *c2rust_fresh1;
            }
        }
        *write = 0 as ::core::ffi::c_char;
    }
    let mut url: [::core::ffi::c_char; 2560] = [0; 2560];
    let mut endpoint: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut endpoint as *mut ::core::ffi::c_char,
        &raw const (*target).endpoint as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut endpoint_length: size_t = strlen(&raw mut endpoint as *mut ::core::ffi::c_char);
    while endpoint_length != 0
        && endpoint[endpoint_length.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
    {
        endpoint_length = endpoint_length.wrapping_sub(1);
        endpoint[endpoint_length] = 0 as ::core::ffi::c_char;
    }
    let mut length: ::core::ffi::c_int = snprintf(
        &raw mut url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2560]>(),
        b"%s/%s%s%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut endpoint as *mut ::core::ffi::c_char,
        &raw const (*target).bucket as *const ::core::ffi::c_char,
        if !key.is_null() {
            b"/\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !encoded.is_null() {
            encoded
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !query.is_null() {
            b"?\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !query.is_null() {
            query
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if curl.is_null()
        || encoded.is_null()
        || length < 0 as ::core::ffi::c_int
        || length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 2560]>()
    {
        if !curl.is_null() {
            curl_easy_cleanup(curl);
        }
        curl_free(encoded as *mut ::core::ffi::c_void);
        wipe(
            &raw mut credentials as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<s3_credentials_t>(),
        );
        error_set(
            error,
            b"Cannot construct archive request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    let mut signature: [::core::ffi::c_char; 128] = [0; 128];
    let mut hash_header: [::core::ffi::c_char; 120] = [0; 120];
    let mut token_header: [::core::ffi::c_char; 4200] = [0; 4200];
    snprintf(
        &raw mut signature as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        b"aws:amz:%s:s3\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).region as *const ::core::ffi::c_char,
    );
    let mut headers: *mut curl_slist = ::core::ptr::null_mut::<curl_slist>();
    headers = curl_slist_append(headers, b"Expect:\0".as_ptr() as *const ::core::ffi::c_char);
    if !checksum.is_null() && *checksum as ::core::ffi::c_int != 0 {
        snprintf(
            &raw mut hash_header as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 120]>(),
            b"x-amz-content-sha256: %s\0".as_ptr() as *const ::core::ffi::c_char,
            checksum,
        );
        headers = curl_slist_append(headers, &raw mut hash_header as *mut ::core::ffi::c_char);
    }
    if *(&raw mut credentials.token as *mut ::core::ffi::c_char) != 0 {
        snprintf(
            &raw mut token_header as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4200]>(),
            b"x-amz-security-token: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut credentials.token as *mut ::core::ffi::c_char,
        );
        headers = curl_slist_append(headers, &raw mut token_header as *mut ::core::ffi::c_char);
    }
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_URL,
        &raw mut url as *mut ::core::ffi::c_char,
    );
    if !(*io).anonymous {
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_AWS_SIGV4,
            &raw mut signature as *mut ::core::ffi::c_char,
        );
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_USERNAME,
            &raw mut credentials.access as *mut ::core::ffi::c_char,
        );
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_PASSWORD,
            &raw mut credentials.secret as *mut ::core::ffi::c_char,
        );
    }
    curl_easy_setopt(curl, CURLoption::CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_FOLLOWLOCATION,
        0 as ::core::ffi::c_long,
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_NOSIGNAL, 1 as ::core::ffi::c_long);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_CONNECTTIMEOUT,
        10 as ::core::ffi::c_long,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_TIMEOUT,
        if !upload.is_null() || !(*io).digest.is_null() || !(*io).file.is_null() {
            3600 as ::core::ffi::c_long
        } else {
            20 as ::core::ffi::c_long
        },
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_LOW_SPEED_LIMIT,
        1024 as ::core::ffi::c_long,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_LOW_SPEED_TIME,
        30 as ::core::ffi::c_long,
    );
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_WRITEFUNCTION,
        Some(
            receive_bytes
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_WRITEDATA, io);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_HEADERFUNCTION,
        Some(
            response_header
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    size_t,
                    size_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_HEADERDATA, io);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_XFERINFOFUNCTION,
        Some(
            progress
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    curl_off_t,
                    curl_off_t,
                    curl_off_t,
                    curl_off_t,
                ) -> ::core::ffi::c_int,
        ),
    );
    curl_easy_setopt(curl, CURLoption::CURLOPT_XFERINFODATA, (*io).control);
    curl_easy_setopt(
        curl,
        CURLoption::CURLOPT_NOPROGRESS,
        0 as ::core::ffi::c_long,
    );
    if !(*io).control.is_null() && (*(*io).control).bandwidth_bps != 0 {
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_MAX_SEND_SPEED_LARGE,
            (*(*io).control).bandwidth_bps as curl_off_t,
        );
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_MAX_RECV_SPEED_LARGE,
            (*(*io).control).bandwidth_bps as curl_off_t,
        );
    }
    if !upload.is_null() {
        curl_easy_setopt(curl, CURLoption::CURLOPT_UPLOAD, 1 as ::core::ffi::c_long);
        curl_easy_setopt(curl, CURLoption::CURLOPT_READDATA, upload);
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_INFILESIZE_LARGE,
            upload_size as curl_off_t,
        );
    }
    if strcmp(method, b"HEAD\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        curl_easy_setopt(curl, CURLoption::CURLOPT_NOBODY, 1 as ::core::ffi::c_long);
    }
    if *(&raw mut (*io).range as *mut ::core::ffi::c_char) != 0 {
        curl_easy_setopt(
            curl,
            CURLoption::CURLOPT_RANGE,
            &raw mut (*io).range as *mut ::core::ffi::c_char,
        );
    }
    curl_easy_setopt(curl, CURLoption::CURLOPT_CUSTOMREQUEST, method);
    let mut code: CURLcode = curl_easy_perform(curl);
    let mut status: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    curl_easy_getinfo(curl, CURLINFO::CURLINFO_RESPONSE_CODE, &raw mut status);
    if !object_size.is_null()
        && code.0 == CURLcode::CURLE_OK.0
        && status == 200 as ::core::ffi::c_long
    {
        let mut bytes: curl_off_t = -1 as curl_off_t;
        curl_easy_getinfo(
            curl,
            CURLINFO::CURLINFO_CONTENT_LENGTH_DOWNLOAD_T,
            &raw mut bytes,
        );
        if bytes < 0 as curl_off_t {
            code = CURLcode::CURLE_BAD_CONTENT_ENCODING;
        } else {
            *object_size = bytes as uint64_t;
        }
    }
    curl_easy_cleanup(curl);
    curl_slist_free_all(headers);
    curl_free(encoded as *mut ::core::ffi::c_void);
    wipe(
        &raw mut credentials as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<s3_credentials_t>(),
    );
    wipe(
        &raw mut token_header as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 4200]>(),
    );
    if code.0 == CURLcode::CURLE_ABORTED_BY_CALLBACK.0 {
        error_set(
            error,
            b"Archive operation cancelled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return C2Rust_Unnamed::STORAGE_S3_CANCELLED.0;
    }
    if code.0 != CURLcode::CURLE_OK.0 {
        snprintf(
            error,
            STORAGE_TARGET_ERROR_MAX as size_t,
            b"Archive transport failed (%d)\0".as_ptr() as *const ::core::ffi::c_char,
            code.0 as ::core::ffi::c_int,
        );
        return C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    if status == 404 as ::core::ffi::c_long {
        return C2Rust_Unnamed::STORAGE_S3_MISSING.0;
    }
    if status < 200 as ::core::ffi::c_long || status >= 300 as ::core::ffi::c_long {
        snprintf(
            error,
            STORAGE_TARGET_ERROR_MAX as size_t,
            b"Archive request failed (HTTP %ld)\0".as_ptr() as *const ::core::ffi::c_char,
            status,
        );
        return if status == 409 as ::core::ffi::c_long || status == 412 as ::core::ffi::c_long {
            C2Rust_Unnamed::STORAGE_S3_CONFLICT.0
        } else {
            C2Rust_Unnamed::STORAGE_S3_ERROR.0
        };
    }
    return C2Rust_Unnamed::STORAGE_S3_OK.0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_s3_read_range(
    mut target: *const storage_target_t,
    mut key: *const ::core::ffi::c_char,
    mut offset: uint64_t,
    mut length: size_t,
    mut total_size: uint64_t,
    mut buffer: *mut ::core::ffi::c_void,
    mut control: *const storage_transfer_control_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if length == 0
        || offset >= total_size
        || length > (total_size as size_t).wrapping_sub(offset as size_t)
        || length
            > (1024 as ::core::ffi::c_uint).wrapping_mul(1024 as ::core::ffi::c_uint) as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut io: s3_io_t = s3_io_t {
        body: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        length: 0,
        file: ::core::ptr::null_mut::<FILE>(),
        digest: ::core::ptr::null_mut::<mbedtls_sha256_context>(),
        bytes: 0,
        limit: length as uint64_t,
        control: control,
        status: 0,
        etag: [0; 192],
        range: [0; 64],
        content_range: [0; 128],
        data: buffer as *mut ::core::ffi::c_uchar,
        anonymous: false,
    };
    snprintf(
        &raw mut io.range as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%llu-%llu\0".as_ptr() as *const ::core::ffi::c_char,
        offset as ::core::ffi::c_ulonglong,
        offset
            .wrapping_add(length as uint64_t)
            .wrapping_sub(1 as uint64_t) as ::core::ffi::c_ulonglong,
    );
    let mut result: ::core::ffi::c_int = request(
        target,
        key,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<FILE>(),
        0 as uint64_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut io,
        ::core::ptr::null_mut::<uint64_t>(),
        error,
    );
    let mut expected: [::core::ffi::c_char; 128] = [0; 128];
    snprintf(
        &raw mut expected as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        b"bytes %llu-%llu/%llu\0".as_ptr() as *const ::core::ffi::c_char,
        offset as ::core::ffi::c_ulonglong,
        offset
            .wrapping_add(length as uint64_t)
            .wrapping_sub(1 as uint64_t) as ::core::ffi::c_ulonglong,
        total_size as ::core::ffi::c_ulonglong,
    );
    if result == 0
        && (io.status != 206 as ::core::ffi::c_long
            || io.bytes != length as uint64_t
            || strcmp(
                &raw mut io.content_range as *mut ::core::ffi::c_char,
                &raw mut expected as *mut ::core::ffi::c_char,
            ) != 0)
    {
        error_set(
            error,
            b"Archive server returned an invalid byte range\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        result = -1 as ::core::ffi::c_int;
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_s3_stat(
    mut target: *const storage_target_t,
    mut key: *const ::core::ffi::c_char,
    mut size: *mut uint64_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut io: s3_io_t = s3_io_t {
        body: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        length: 0,
        file: ::core::ptr::null_mut::<FILE>(),
        digest: ::core::ptr::null_mut::<mbedtls_sha256_context>(),
        bytes: 0,
        limit: S3_REPLY_LIMIT as uint64_t,
        control: ::core::ptr::null::<storage_transfer_control_t>(),
        status: 0,
        etag: [0; 192],
        range: [0; 64],
        content_range: [0; 128],
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        anonymous: false,
    };
    return request(
        target,
        key,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"HEAD\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<FILE>(),
        0 as uint64_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut io,
        size,
        error,
    );
}
pub const S3_PART_BYTES: ::core::ffi::c_ulonglong = (8 as ::core::ffi::c_ulonglong)
    .wrapping_mul(1024 as ::core::ffi::c_ulonglong)
    .wrapping_mul(1024 as ::core::ffi::c_ulonglong);
unsafe extern "C" fn upload_checkpoint(
    mut uuid: *const ::core::ffi::c_char,
    mut id: *mut ::core::ffi::c_char,
    mut parts: *mut *mut cJSON,
    mut save: bool,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() || !lightnvr_uuid_is_valid(uuid) {
        return -1 as ::core::ffi::c_int;
    }
    let mut json: *mut ::core::ffi::c_char = if save as ::core::ffi::c_int != 0 {
        cJSON_PrintUnformatted(*parts)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    if save as ::core::ffi::c_int != 0 && json.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        if save as ::core::ffi::c_int != 0 {
            b"UPDATE storage_migration_jobs SET upload_id=?2,upload_parts=?3,bytes_copied=MIN(bytes_total,COALESCE(json_array_length(?3,'$.parts')*json_extract(?3,'$.part_size'),0)),updated_at=strftime('%s','now') WHERE uuid=?1;\0"
                .as_ptr() as *const ::core::ffi::c_char
        } else {
            b"SELECT upload_id,upload_parts FROM storage_migration_jobs WHERE uuid=?1;\0".as_ptr()
                as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if save {
            sqlite3_bind_text(
                stmt,
                2 as ::core::ffi::c_int,
                id,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                stmt,
                3 as ::core::ffi::c_int,
                json,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        rc = sqlite3_step(stmt);
        if !save && rc == SQLITE_ROW {
            safe_strcpy(
                id,
                sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
                1024 as size_t,
                0 as size_t,
            );
            *parts =
                cJSON_Parse(sqlite3_column_text(stmt, 1 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char);
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut result: ::core::ffi::c_int = if save as ::core::ffi::c_int != 0 {
        if rc == SQLITE_DONE && sqlite3_changes(db) == 1 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        }
    } else if rc == SQLITE_ROW {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
    pthread_mutex_unlock(mutex);
    free(json as *mut ::core::ffi::c_void);
    return result;
}
unsafe extern "C" fn upload_query(
    mut id: *const ::core::ffi::c_char,
    mut part: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut escaped: *mut ::core::ffi::c_char = curl_easy_escape(NULL, id, 0 as ::core::ffi::c_int);
    if escaped.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut length: size_t = strlen(escaped).wrapping_add(64 as size_t);
    let mut query: *mut ::core::ffi::c_char = malloc(length) as *mut ::core::ffi::c_char;
    if !query.is_null() {
        if part != 0 {
            snprintf(
                query,
                length,
                b"partNumber=%d&uploadId=%s\0".as_ptr() as *const ::core::ffi::c_char,
                part,
                escaped,
            );
        } else {
            snprintf(
                query,
                length,
                b"uploadId=%s\0".as_ptr() as *const ::core::ffi::c_char,
                escaped,
            );
        }
    }
    curl_free(escaped as *mut ::core::ffi::c_void);
    return query;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_s3_abort_upload(
    mut target: *const storage_target_t,
    mut key: *const ::core::ffi::c_char,
    mut id: *const ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if id.is_null() || *id == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut query: *mut ::core::ffi::c_char = upload_query(id, 0 as ::core::ffi::c_int);
    if query.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut io: s3_io_t = s3_io_t {
        body: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        length: 0,
        file: ::core::ptr::null_mut::<FILE>(),
        digest: ::core::ptr::null_mut::<mbedtls_sha256_context>(),
        bytes: 0,
        limit: S3_REPLY_LIMIT as uint64_t,
        control: ::core::ptr::null::<storage_transfer_control_t>(),
        status: 0,
        etag: [0; 192],
        range: [0; 64],
        content_range: [0; 128],
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        anonymous: false,
    };
    let mut result: ::core::ffi::c_int = request(
        target,
        key,
        query,
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<FILE>(),
        0 as uint64_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut io,
        ::core::ptr::null_mut::<uint64_t>(),
        error,
    );
    free(query as *mut ::core::ffi::c_void);
    return if result == C2Rust_Unnamed::STORAGE_S3_MISSING.0 {
        0 as ::core::ffi::c_int
    } else {
        result
    };
}
unsafe extern "C" fn response_xml(
    mut body: *mut ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) -> ezxml_t {
    if body.is_null() || !strstr(body, b"<!\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    let mut root: ezxml_t = ezxml_parse_str(body, strlen(body));
    if root.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    let mut local: *const ::core::ffi::c_char = if !(*root).name.is_null() {
        strrchr((*root).name, ':' as ::core::ffi::c_int)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    local = if !local.is_null() {
        local.offset(1 as ::core::ffi::c_int as isize)
    } else {
        (*root).name
    };
    if *ezxml_error(root) as ::core::ffi::c_int != 0 || local.is_null() || strcmp(local, name) != 0
    {
        ezxml_free(root);
        return ::core::ptr::null_mut::<ezxml>();
    }
    return root;
}
unsafe extern "C" fn file_section_hash(
    mut file: *mut FILE,
    mut offset: uint64_t,
    mut bytes: uint64_t,
    mut hash: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if fseeko(file, offset as __off_t, SEEK_SET) != 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut context: mbedtls_sha256_context = mbedtls_sha256_context {
        private_buffer: [0; 64],
        private_total: [0; 2],
        private_state: [0; 8],
        private_is224: 0,
    };
    mbedtls_sha256_init(&raw mut context);
    let mut rc: ::core::ffi::c_int =
        mbedtls_sha256_starts(&raw mut context, 0 as ::core::ffi::c_int);
    let mut buffer: [::core::ffi::c_uchar; 32768] = [0; 32768];
    let mut digest: [::core::ffi::c_uchar; 32] = [0; 32];
    while rc == 0 && bytes != 0 {
        let mut count: size_t =
            if bytes > ::core::mem::size_of::<[::core::ffi::c_uchar; 32768]>() as uint64_t {
                ::core::mem::size_of::<[::core::ffi::c_uchar; 32768]>()
            } else {
                bytes as size_t
            };
        if fread(
            &raw mut buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            1 as size_t,
            count,
            file,
        ) as size_t
            != count
        {
            rc = -1 as ::core::ffi::c_int;
            break;
        } else {
            rc = mbedtls_sha256_update(
                &raw mut context,
                &raw mut buffer as *mut ::core::ffi::c_uchar,
                count,
            );
            bytes = (bytes as ::core::ffi::c_ulong).wrapping_sub(count as ::core::ffi::c_ulong)
                as uint64_t;
        }
    }
    if rc == 0 {
        rc = mbedtls_sha256_finish(
            &raw mut context,
            &raw mut digest as *mut ::core::ffi::c_uchar,
        );
    }
    mbedtls_sha256_free(&raw mut context);
    if rc == 0 {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 32 as ::core::ffi::c_int {
            snprintf(
                hash.offset((i * 2 as ::core::ffi::c_int) as isize),
                3 as size_t,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                digest[i as usize] as ::core::ffi::c_int,
            );
            i += 1;
        }
    }
    return if rc != 0 || fseeko(file, offset as __off_t, SEEK_SET) != 0 {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn multipart_upload(
    mut target: *const storage_target_t,
    mut key: *const ::core::ffi::c_char,
    mut file: *mut FILE,
    mut bytes: uint64_t,
    mut checksum: *const ::core::ffi::c_char,
    mut control: *const storage_transfer_control_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut id: [::core::ffi::c_char; 1024] = [
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
    let mut checkpoint: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if upload_checkpoint(
        (*control).job_uuid,
        &raw mut id as *mut ::core::ffi::c_char,
        &raw mut checkpoint,
        r#false != 0,
    ) != 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut parts: *mut cJSON = cJSON_GetObjectItemCaseSensitive(
        checkpoint,
        b"parts\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut hash: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        checkpoint,
        b"checksum\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut part_size: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        checkpoint,
        b"part_size\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut chunk: uint64_t = (bytes
        .wrapping_add(9999 as uint64_t)
        .wrapping_div(10000 as uint64_t) as ::core::ffi::c_ulonglong)
        .wrapping_add(S3_PART_BYTES)
        .wrapping_sub(1 as ::core::ffi::c_ulonglong)
        .wrapping_div(S3_PART_BYTES)
        .wrapping_mul(S3_PART_BYTES) as uint64_t;
    if (chunk as ::core::ffi::c_ulonglong) < S3_PART_BYTES {
        chunk = S3_PART_BYTES as uint64_t;
    }
    let mut valid: bool = cJSON_IsArray(parts) != 0
        && cJSON_IsString(hash) != 0
        && strcmp((*hash).valuestring, checksum) == 0
        && cJSON_IsNumber(part_size) != 0
        && (*part_size).valuedouble == chunk as ::core::ffi::c_double
        && cJSON_GetArraySize(parts)
            <= bytes
                .wrapping_add(chunk)
                .wrapping_sub(1 as uint64_t)
                .wrapping_div(chunk) as ::core::ffi::c_int;
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !valid {
        if *(&raw mut id as *mut ::core::ffi::c_char) as ::core::ffi::c_int != 0
            && storage_s3_abort_upload(target, key, &raw mut id as *mut ::core::ffi::c_char, error)
                != 0
        {
            cJSON_Delete(checkpoint);
            return -1 as ::core::ffi::c_int;
        }
        id[0usize] = 0 as ::core::ffi::c_char;
        cJSON_Delete(checkpoint);
        checkpoint = cJSON_CreateObject();
        if checkpoint.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        cJSON_AddStringToObject(
            checkpoint,
            b"checksum\0".as_ptr() as *const ::core::ffi::c_char,
            checksum,
        );
        cJSON_AddNumberToObject(
            checkpoint,
            b"part_size\0".as_ptr() as *const ::core::ffi::c_char,
            chunk as ::core::ffi::c_double,
        );
        parts = cJSON_AddArrayToObject(
            checkpoint,
            b"parts\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if parts.is_null()
            || upload_checkpoint(
                (*control).job_uuid,
                &raw mut id as *mut ::core::ffi::c_char,
                &raw mut checkpoint,
                r#true != 0,
            ) != 0
        {
            cJSON_Delete(checkpoint);
            return -1 as ::core::ffi::c_int;
        }
    }
    let mut body: *mut ::core::ffi::c_char =
        calloc(1 as size_t, S3_REPLY_LIMIT as size_t) as *mut ::core::ffi::c_char;
    if body.is_null() {
        cJSON_Delete(checkpoint);
        return -1 as ::core::ffi::c_int;
    }
    if *(&raw mut id as *mut ::core::ffi::c_char) == 0 {
        cJSON_DeleteItemFromObjectCaseSensitive(
            checkpoint,
            b"parts\0".as_ptr() as *const ::core::ffi::c_char,
        );
        parts = cJSON_AddArrayToObject(
            checkpoint,
            b"parts\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if parts.is_null() {
            free(body as *mut ::core::ffi::c_void);
            cJSON_Delete(checkpoint);
            return -1 as ::core::ffi::c_int;
        }
        let mut io: s3_io_t = s3_io_t {
            body: body,
            length: 0,
            file: ::core::ptr::null_mut::<FILE>(),
            digest: ::core::ptr::null_mut::<mbedtls_sha256_context>(),
            bytes: 0,
            limit: S3_REPLY_LIMIT.wrapping_sub(1 as ::core::ffi::c_uint) as uint64_t,
            control: control,
            status: 0,
            etag: [0; 192],
            range: [0; 64],
            content_range: [0; 128],
            data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            anonymous: false,
        };
        result = request(
            target,
            key,
            b"uploads\0".as_ptr() as *const ::core::ffi::c_char,
            b"POST\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<FILE>(),
            0 as uint64_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut io,
            ::core::ptr::null_mut::<uint64_t>(),
            error,
        );
        let mut root: ezxml_t = if result == 0 as ::core::ffi::c_int {
            response_xml(
                body,
                b"InitiateMultipartUploadResult\0".as_ptr() as *const ::core::ffi::c_char,
            )
        } else {
            ::core::ptr::null_mut::<ezxml>()
        };
        let mut value: *const ::core::ffi::c_char = if !root.is_null() {
            if !ezxml_child(root, b"UploadId\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
                (*ezxml_child(root, b"UploadId\0".as_ptr() as *const ::core::ffi::c_char)).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            }
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
        if value.is_null()
            || *value == 0
            || strlen(value) >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
        {
            result = -1 as ::core::ffi::c_int;
        } else {
            safe_strcpy(
                &raw mut id as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                0 as size_t,
            );
        }
        if !root.is_null() {
            ezxml_free(root);
        }
        if result == 0
            && upload_checkpoint(
                (*control).job_uuid,
                &raw mut id as *mut ::core::ffi::c_char,
                &raw mut checkpoint,
                r#true != 0,
            ) != 0
        {
            storage_s3_abort_upload(target, key, &raw mut id as *mut ::core::ffi::c_char, error);
            result = -1 as ::core::ffi::c_int;
        }
    }
    let mut completed: ::core::ffi::c_int = cJSON_GetArraySize(parts);
    let mut offset: uint64_t = (completed as uint64_t).wrapping_mul(chunk);
    while result == 0 && offset < bytes {
        let mut length: uint64_t = if bytes.wrapping_sub(offset) < chunk {
            bytes.wrapping_sub(offset)
        } else {
            chunk
        };
        let mut hash_hex: [::core::ffi::c_char; 65] = [0; 65];
        let mut query: *mut ::core::ffi::c_char = upload_query(
            &raw mut id as *mut ::core::ffi::c_char,
            completed + 1 as ::core::ffi::c_int,
        );
        if query.is_null()
            || file_section_hash(
                file,
                offset,
                length,
                &raw mut hash_hex as *mut ::core::ffi::c_char,
            ) != 0
        {
            free(query as *mut ::core::ffi::c_void);
            result = -1 as ::core::ffi::c_int;
            break;
        } else {
            let mut io_0: s3_io_t = s3_io_t {
                body: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                length: 0,
                file: ::core::ptr::null_mut::<FILE>(),
                digest: ::core::ptr::null_mut::<mbedtls_sha256_context>(),
                bytes: 0,
                limit: S3_REPLY_LIMIT as uint64_t,
                control: control,
                status: 0,
                etag: [0; 192],
                range: [0; 64],
                content_range: [0; 128],
                data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                anonymous: false,
            };
            result = request(
                target,
                key,
                query,
                b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
                file,
                length,
                &raw mut hash_hex as *mut ::core::ffi::c_char,
                &raw mut io_0,
                ::core::ptr::null_mut::<uint64_t>(),
                error,
            );
            free(query as *mut ::core::ffi::c_void);
            if result == C2Rust_Unnamed::STORAGE_S3_MISSING.0 {
                id[0usize] = 0 as ::core::ffi::c_char;
                cJSON_DeleteItemFromObjectCaseSensitive(
                    checkpoint,
                    b"parts\0".as_ptr() as *const ::core::ffi::c_char,
                );
                cJSON_AddArrayToObject(
                    checkpoint,
                    b"parts\0".as_ptr() as *const ::core::ffi::c_char,
                );
                upload_checkpoint(
                    (*control).job_uuid,
                    &raw mut id as *mut ::core::ffi::c_char,
                    &raw mut checkpoint,
                    r#true != 0,
                );
                result = C2Rust_Unnamed::STORAGE_S3_ERROR.0;
                break;
            } else {
                if result != 0 {
                    break;
                }
                if io_0.etag[0usize] == 0 {
                    result = -1 as ::core::ffi::c_int;
                    break;
                } else {
                    let mut p: *const ::core::ffi::c_char =
                        &raw mut io_0.etag as *mut ::core::ffi::c_char;
                    while *p != 0 {
                        if (*p as ::core::ffi::c_uchar as ::core::ffi::c_int)
                            < 32 as ::core::ffi::c_int
                            || !strchr(
                                b"<>&\0".as_ptr() as *const ::core::ffi::c_char,
                                *p as ::core::ffi::c_int,
                            )
                            .is_null()
                        {
                            result = -1 as ::core::ffi::c_int;
                        }
                        p = p.offset(1);
                    }
                    if result != 0 {
                        break;
                    }
                    if cJSON_AddItemToArray(
                        parts,
                        cJSON_CreateString(&raw mut io_0.etag as *mut ::core::ffi::c_char),
                    ) == 0
                    {
                        result = -1 as ::core::ffi::c_int;
                        break;
                    } else {
                        completed += 1;
                        result = upload_checkpoint(
                            (*control).job_uuid,
                            &raw mut id as *mut ::core::ffi::c_char,
                            &raw mut checkpoint,
                            r#true != 0,
                        );
                        offset = offset.wrapping_add(chunk);
                    }
                }
            }
        }
    }
    if result == 0 {
        let mut manifest: *mut FILE = tmpfile();
        if manifest.is_null() {
            result = -1 as ::core::ffi::c_int;
        } else {
            fputs(
                b"<CompleteMultipartUpload>\0".as_ptr() as *const ::core::ffi::c_char,
                manifest,
            );
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < completed {
                let mut etag: *const cJSON = cJSON_GetArrayItem(parts, i);
                if cJSON_IsString(etag) == 0 {
                    result = -1 as ::core::ffi::c_int;
                    break;
                } else {
                    fprintf(
                        manifest,
                        b"<Part><PartNumber>%d</PartNumber><ETag>%s</ETag></Part>\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        i + 1 as ::core::ffi::c_int,
                        (*etag).valuestring,
                    );
                    i += 1;
                }
            }
            fputs(
                b"</CompleteMultipartUpload>\0".as_ptr() as *const ::core::ffi::c_char,
                manifest,
            );
            let mut length_0: off_t = ftello(manifest);
            let mut hash_hex_0: [::core::ffi::c_char; 65] = [0; 65];
            let mut query_0: *mut ::core::ffi::c_char = upload_query(
                &raw mut id as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            memset(
                body as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                S3_REPLY_LIMIT as size_t,
            );
            let mut io_1: s3_io_t = s3_io_t {
                body: body,
                length: 0,
                file: ::core::ptr::null_mut::<FILE>(),
                digest: ::core::ptr::null_mut::<mbedtls_sha256_context>(),
                bytes: 0,
                limit: S3_REPLY_LIMIT.wrapping_sub(1 as ::core::ffi::c_uint) as uint64_t,
                control: control,
                status: 0,
                etag: [0; 192],
                range: [0; 64],
                content_range: [0; 128],
                data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                anonymous: false,
            };
            if result == 0
                && !query_0.is_null()
                && length_0 > 0 as off_t
                && file_section_hash(
                    manifest,
                    0 as uint64_t,
                    length_0 as uint64_t,
                    &raw mut hash_hex_0 as *mut ::core::ffi::c_char,
                ) == 0
            {
                result = request(
                    target,
                    key,
                    query_0,
                    b"POST\0".as_ptr() as *const ::core::ffi::c_char,
                    manifest,
                    length_0 as uint64_t,
                    &raw mut hash_hex_0 as *mut ::core::ffi::c_char,
                    &raw mut io_1,
                    ::core::ptr::null_mut::<uint64_t>(),
                    error,
                );
            } else {
                result = -1 as ::core::ffi::c_int;
            }
            let mut missing_upload: bool = result == C2Rust_Unnamed::STORAGE_S3_MISSING.0;
            let mut failure_body: *mut ::core::ffi::c_char = strdup(body);
            let mut failure: ezxml_t = if !failure_body.is_null() {
                response_xml(
                    failure_body,
                    b"Error\0".as_ptr() as *const ::core::ffi::c_char,
                )
            } else {
                ::core::ptr::null_mut::<ezxml>()
            };
            if !failure.is_null() {
                if strcmp(
                    if !ezxml_child(failure, b"Code\0".as_ptr() as *const ::core::ffi::c_char)
                        .is_null()
                    {
                        (*ezxml_child(failure, b"Code\0".as_ptr() as *const ::core::ffi::c_char))
                            .txt
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    b"NoSuchUpload\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0
                {
                    missing_upload = r#true != 0;
                }
                ezxml_free(failure);
            }
            free(failure_body as *mut ::core::ffi::c_void);
            let mut root_0: ezxml_t = if result == 0 as ::core::ffi::c_int {
                response_xml(
                    body,
                    b"CompleteMultipartUploadResult\0".as_ptr() as *const ::core::ffi::c_char,
                )
            } else {
                ::core::ptr::null_mut::<ezxml>()
            };
            if root_0.is_null() {
                result = -1 as ::core::ffi::c_int;
            }
            if missing_upload {
                id[0usize] = 0 as ::core::ffi::c_char;
                cJSON_DeleteItemFromObjectCaseSensitive(
                    checkpoint,
                    b"parts\0".as_ptr() as *const ::core::ffi::c_char,
                );
                cJSON_AddArrayToObject(
                    checkpoint,
                    b"parts\0".as_ptr() as *const ::core::ffi::c_char,
                );
                upload_checkpoint(
                    (*control).job_uuid,
                    &raw mut id as *mut ::core::ffi::c_char,
                    &raw mut checkpoint,
                    r#true != 0,
                );
                result = -1 as ::core::ffi::c_int;
            }
            if !root_0.is_null() {
                ezxml_free(root_0);
            }
            free(query_0 as *mut ::core::ffi::c_void);
            fclose(manifest);
        }
    }
    if result == 0 {
        id[0usize] = 0 as ::core::ffi::c_char;
        result = upload_checkpoint(
            (*control).job_uuid,
            &raw mut id as *mut ::core::ffi::c_char,
            &raw mut checkpoint,
            r#true != 0,
        );
    }
    free(body as *mut ::core::ffi::c_void);
    cJSON_Delete(checkpoint);
    if result != 0 && *error == 0 {
        error_set(
            error,
            b"Multipart archive upload could not complete; checkpoint retained for retry\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_s3_upload(
    mut target: *const storage_target_t,
    mut key: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
    mut checksum: *const ::core::ffi::c_char,
    mut control: *const storage_transfer_control_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut descriptor: ::core::ffi::c_int = open(path, O_RDONLY | O_CLOEXEC | O_NOFOLLOW);
    let mut status: stat = stat {
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
    if descriptor < 0 as ::core::ffi::c_int
        || fstat(descriptor, &raw mut status) != 0
        || !(status.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
        || status.st_size < 0 as __off_t
    {
        if descriptor >= 0 as ::core::ffi::c_int {
            close(descriptor);
        }
        error_set(
            error,
            b"Archive source is not a readable regular file\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    let mut file: *mut FILE = fdopen(descriptor, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
    if file.is_null() {
        close(descriptor);
        return C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    let mut io: s3_io_t = s3_io_t {
        body: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        length: 0,
        file: ::core::ptr::null_mut::<FILE>(),
        digest: ::core::ptr::null_mut::<mbedtls_sha256_context>(),
        bytes: 0,
        limit: S3_REPLY_LIMIT as uint64_t,
        control: control,
        status: 0,
        etag: [0; 192],
        range: [0; 64],
        content_range: [0; 128],
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        anonymous: false,
    };
    let mut result: ::core::ffi::c_int = if !control.is_null()
        && !(*control).job_uuid.is_null()
        && status.st_size as uint64_t as ::core::ffi::c_ulonglong >= S3_PART_BYTES
    {
        multipart_upload(
            target,
            key,
            file,
            status.st_size as uint64_t,
            checksum,
            control,
            error,
        )
    } else {
        request(
            target,
            key,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
            file,
            status.st_size as uint64_t,
            checksum,
            &raw mut io,
            ::core::ptr::null_mut::<uint64_t>(),
            error,
        )
    };
    fclose(file);
    return result;
}
unsafe extern "C" fn receive_object(
    mut target: *const storage_target_t,
    mut key: *const ::core::ffi::c_char,
    mut file: *mut FILE,
    mut expected_size: uint64_t,
    mut checksum: *const ::core::ffi::c_char,
    mut control: *const storage_transfer_control_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut digest: mbedtls_sha256_context = mbedtls_sha256_context {
        private_buffer: [0; 64],
        private_total: [0; 2],
        private_state: [0; 8],
        private_is224: 0,
    };
    mbedtls_sha256_init(&raw mut digest);
    if mbedtls_sha256_starts(&raw mut digest, 0 as ::core::ffi::c_int) != 0 {
        mbedtls_sha256_free(&raw mut digest);
        return C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    let mut io: s3_io_t = s3_io_t {
        body: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        length: 0,
        file: file,
        digest: &raw mut digest,
        bytes: 0,
        limit: expected_size,
        control: control,
        status: 0,
        etag: [0; 192],
        range: [0; 64],
        content_range: [0; 128],
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        anonymous: false,
    };
    let mut result: ::core::ffi::c_int = request(
        target,
        key,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<FILE>(),
        0 as uint64_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut io,
        ::core::ptr::null_mut::<uint64_t>(),
        error,
    );
    let mut hash: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut hex: [::core::ffi::c_char; 65] = [0; 65];
    if result == 0 as ::core::ffi::c_int
        && mbedtls_sha256_finish(&raw mut digest, &raw mut hash as *mut ::core::ffi::c_uchar) != 0
    {
        result = C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    mbedtls_sha256_free(&raw mut digest);
    if result == 0 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 32 as ::core::ffi::c_int {
            snprintf(
                (&raw mut hex as *mut ::core::ffi::c_char)
                    .offset((i * 2 as ::core::ffi::c_int) as isize),
                3 as size_t,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                hash[i as usize] as ::core::ffi::c_int,
            );
            i += 1;
        }
        if io.bytes != expected_size
            || !checksum.is_null()
                && *checksum as ::core::ffi::c_int != 0
                && strcmp(&raw mut hex as *mut ::core::ffi::c_char, checksum) != 0
        {
            error_set(
                error,
                b"Archive content length or SHA-256 verification failed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            result = C2Rust_Unnamed::STORAGE_S3_CONFLICT.0;
        }
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_s3_verify(
    mut target: *const storage_target_t,
    mut key: *const ::core::ffi::c_char,
    mut size: uint64_t,
    mut checksum: *const ::core::ffi::c_char,
    mut control: *const storage_transfer_control_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return receive_object(
        target,
        key,
        ::core::ptr::null_mut::<FILE>(),
        size,
        checksum,
        control,
        error,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_s3_download(
    mut target: *const storage_target_t,
    mut key: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
    mut expected_size: uint64_t,
    mut expected_checksum: *const ::core::ffi::c_char,
    mut control: *const storage_transfer_control_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut descriptor: ::core::ffi::c_int = open(
        path,
        O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC | O_NOFOLLOW,
        0o600 as ::core::ffi::c_int,
    );
    if descriptor < 0 as ::core::ffi::c_int {
        error_set(
            error,
            b"Cannot create archive staging file\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    let mut file: *mut FILE = fdopen(descriptor, b"wb\0".as_ptr() as *const ::core::ffi::c_char);
    if file.is_null() {
        close(descriptor);
        unlink(path);
        return C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    let mut result: ::core::ffi::c_int = receive_object(
        target,
        key,
        file,
        expected_size,
        expected_checksum,
        control,
        error,
    );
    if result == 0 as ::core::ffi::c_int && (fflush(file) != 0 || fsync(fileno(file)) != 0) {
        result = C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    if fclose(file) != 0 && result == 0 as ::core::ffi::c_int {
        result = C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    if result != 0 as ::core::ffi::c_int {
        unlink(path);
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_s3_delete(
    mut target: *const storage_target_t,
    mut key: *const ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut io: s3_io_t = s3_io_t {
        body: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        length: 0,
        file: ::core::ptr::null_mut::<FILE>(),
        digest: ::core::ptr::null_mut::<mbedtls_sha256_context>(),
        bytes: 0,
        limit: S3_REPLY_LIMIT as uint64_t,
        control: ::core::ptr::null::<storage_transfer_control_t>(),
        status: 0,
        etag: [0; 192],
        range: [0; 64],
        content_range: [0; 128],
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        anonymous: false,
    };
    let mut result: ::core::ffi::c_int = request(
        target,
        key,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<FILE>(),
        0 as uint64_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut io,
        ::core::ptr::null_mut::<uint64_t>(),
        error,
    );
    if result != 0 as ::core::ffi::c_int && result != C2Rust_Unnamed::STORAGE_S3_MISSING.0 {
        return result;
    }
    let mut size: uint64_t = 0;
    result = storage_s3_stat(target, key, &raw mut size, error);
    if result == C2Rust_Unnamed::STORAGE_S3_MISSING.0 {
        return C2Rust_Unnamed::STORAGE_S3_OK.0;
    }
    if result == C2Rust_Unnamed::STORAGE_S3_OK.0 {
        error_set(
            error,
            b"Archive object still exists after deletion\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return C2Rust_Unnamed::STORAGE_S3_ERROR.0;
}
unsafe extern "C" fn empty_configuration(
    mut body: *mut ::core::ffi::c_char,
    mut length: size_t,
    mut expected: *const ::core::ffi::c_char,
) -> bool {
    if !strstr(body, b"<!\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        return r#false != 0;
    }
    let mut root: ezxml_t = ezxml_parse_str(body, length);
    let mut valid: bool = !root.is_null() && *ezxml_error(root) == 0;
    if valid {
        let mut name: *const ::core::ffi::c_char = strrchr((*root).name, ':' as ::core::ffi::c_int);
        name = if !name.is_null() {
            name.offset(1 as ::core::ffi::c_int as isize)
        } else {
            (*root).name
        };
        valid = strcmp(name, expected) == 0 && (*root).child.is_null();
        let mut p: *const ::core::ffi::c_char = (*root).txt;
        while valid as ::core::ffi::c_int != 0 && !p.is_null() && *p as ::core::ffi::c_int != 0 {
            if *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
            {
                valid = r#false != 0;
            }
            p = p.offset(1);
        }
    }
    if !root.is_null() {
        ezxml_free(root);
    }
    return valid;
}
unsafe extern "C" fn xml_local_name(mut node: ezxml_t) -> *const ::core::ffi::c_char {
    let mut colon: *const ::core::ffi::c_char = if !node.is_null() && !(*node).name.is_null() {
        strrchr((*node).name, ':' as ::core::ffi::c_int)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    return if !colon.is_null() {
        colon.offset(1 as ::core::ffi::c_int as isize)
    } else if !node.is_null() && !(*node).name.is_null() {
        (*node).name
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn safe_lifecycle(mut body: *mut ::core::ffi::c_char) -> bool {
    let mut root: ezxml_t = response_xml(
        body,
        b"LifecycleConfiguration\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if root.is_null() {
        return r#false != 0;
    }
    let mut safe: bool = r#true != 0;
    let mut rule: ezxml_t = (*root).child;
    while !rule.is_null() && safe as ::core::ffi::c_int != 0 {
        if strcmp(
            xml_local_name(rule),
            b"Rule\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0
        {
            safe = r#false != 0;
            break;
        } else {
            let mut abort_rule: bool = r#false != 0;
            let mut field: ezxml_t = (*rule).child;
            while !field.is_null() && safe as ::core::ffi::c_int != 0 {
                let mut name: *const ::core::ffi::c_char = xml_local_name(field);
                if strcmp(
                    name,
                    b"AbortIncompleteMultipartUpload\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0
                {
                    let mut days: ezxml_t = (*field).child;
                    let mut end: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    let mut value: ::core::ffi::c_long = if !days.is_null() {
                        strtol((*days).txt, &raw mut end, 10 as ::core::ffi::c_int)
                    } else {
                        0 as ::core::ffi::c_long
                    };
                    abort_rule = !days.is_null()
                        && strcmp(
                            xml_local_name(days),
                            b"DaysAfterInitiation\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0
                        && (*days).ordered.is_null()
                        && value >= 1 as ::core::ffi::c_long
                        && value <= 7 as ::core::ffi::c_long
                        && !end.is_null()
                        && *end == 0;
                    if !abort_rule {
                        safe = r#false != 0;
                    }
                } else if strcmp(name, b"ID\0".as_ptr() as *const ::core::ffi::c_char) != 0
                    && strcmp(name, b"Filter\0".as_ptr() as *const ::core::ffi::c_char) != 0
                    && strcmp(name, b"Prefix\0".as_ptr() as *const ::core::ffi::c_char) != 0
                    && strcmp(name, b"Status\0".as_ptr() as *const ::core::ffi::c_char) != 0
                {
                    safe = r#false != 0;
                }
                field = (*field).ordered;
            }
            if !abort_rule {
                safe = r#false != 0;
            }
            rule = (*rule).ordered;
        }
    }
    ezxml_free(root);
    return safe;
}
unsafe extern "C" fn lifecycle_attested(mut target: *const storage_target_t) -> bool {
    let mut json: *mut cJSON = private_json_read(
        target,
        b".lifecycle.json\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut endpoint: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        json,
        b"endpoint\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut region: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(json, b"region\0".as_ptr() as *const ::core::ffi::c_char);
    let mut bucket: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(json, b"bucket\0".as_ptr() as *const ::core::ffi::c_char);
    let mut checked: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        json,
        b"checked_at\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut xml: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        json,
        b"lifecycle_configuration_xml\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut now: ::core::ffi::c_double =
        time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_double;
    let mut valid: bool = cJSON_IsString(endpoint) != 0
        && strcmp(
            (*endpoint).valuestring,
            &raw const (*target).endpoint as *const ::core::ffi::c_char,
        ) == 0
        && cJSON_IsString(region) != 0
        && strcmp(
            (*region).valuestring,
            &raw const (*target).region as *const ::core::ffi::c_char,
        ) == 0
        && cJSON_IsString(bucket) != 0
        && strcmp(
            (*bucket).valuestring,
            &raw const (*target).bucket as *const ::core::ffi::c_char,
        ) == 0
        && cJSON_IsNumber(checked) != 0
        && (*checked).valuedouble > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*checked).valuedouble <= now
        && now - (*checked).valuedouble < 3600 as ::core::ffi::c_int as ::core::ffi::c_double
        && cJSON_IsString(xml) != 0
        && safe_lifecycle((*xml).valuestring) as ::core::ffi::c_int != 0;
    cJSON_Delete(json);
    return valid;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_s3_probe(
    mut target: *mut storage_target_t,
    mut write_test: bool,
) -> ::core::ffi::c_int {
    let mut error: [::core::ffi::c_char; 256] = [
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
    let mut body: *mut ::core::ffi::c_char =
        calloc(1 as size_t, S3_REPLY_LIMIT as size_t) as *mut ::core::ffi::c_char;
    if body.is_null() {
        return C2Rust_Unnamed::STORAGE_S3_ERROR.0;
    }
    let mut io: s3_io_t = s3_io_t {
        body: body,
        length: 0,
        file: ::core::ptr::null_mut::<FILE>(),
        digest: ::core::ptr::null_mut::<mbedtls_sha256_context>(),
        bytes: 0,
        limit: S3_REPLY_LIMIT.wrapping_sub(1 as ::core::ffi::c_uint) as uint64_t,
        control: ::core::ptr::null::<storage_transfer_control_t>(),
        status: 0,
        etag: [0; 192],
        range: [0; 64],
        content_range: [0; 128],
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        anonymous: false,
    };
    let mut result: ::core::ffi::c_int = request(
        target,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"versioning\0".as_ptr() as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<FILE>(),
        0 as uint64_t,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut io,
        ::core::ptr::null_mut::<uint64_t>(),
        &raw mut error as *mut ::core::ffi::c_char,
    );
    if result == 0 as ::core::ffi::c_int
        && !empty_configuration(
            body,
            io.length,
            b"VersioningConfiguration\0".as_ptr() as *const ::core::ffi::c_char,
        )
    {
        error_set(
            &raw mut error as *mut ::core::ffi::c_char,
            b"Versioned buckets require a version-aware adapter; use an unversioned dedicated bucket\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        result = C2Rust_Unnamed::STORAGE_S3_CONFLICT.0;
    }
    if result == 0 as ::core::ffi::c_int {
        memset(
            body as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            S3_REPLY_LIMIT as size_t,
        );
        io.length = 0 as size_t;
        io.bytes = 0 as uint64_t;
        result = request(
            target,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"lifecycle\0".as_ptr() as *const ::core::ffi::c_char,
            b"GET\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<FILE>(),
            0 as uint64_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut io,
            ::core::ptr::null_mut::<uint64_t>(),
            &raw mut error as *mut ::core::ffi::c_char,
        );
        if result == C2Rust_Unnamed::STORAGE_S3_MISSING.0 {
            result = 0 as ::core::ffi::c_int;
        } else if result != 0 as ::core::ffi::c_int && io.status == 403 as ::core::ffi::c_long {
            if lifecycle_attested(target) {
                result = 0 as ::core::ffi::c_int;
            } else {
                error_set(
                    &raw mut error as *mut ::core::ffi::c_char,
                    b"Lifecycle inspection denied; a fresh operator-verified .lifecycle.json file is required with bucket-scoped credentials\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        } else if result == 0 as ::core::ffi::c_int && !safe_lifecycle(body) {
            error_set(
                &raw mut error as *mut ::core::ffi::c_char,
                b"Bucket lifecycle may only abort incomplete uploads after 1-7 days; expiry and transitions are unsupported\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            result = C2Rust_Unnamed::STORAGE_S3_CONFLICT.0;
        }
    }
    free(body as *mut ::core::ffi::c_void);
    if result == 0 as ::core::ffi::c_int && write_test as ::core::ffi::c_int != 0 {
        let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
        let mut key: [::core::ffi::c_char; 96] = [0; 96];
        let mut payload: *mut FILE = tmpfile();
        if payload.is_null()
            || lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char) != 0
        {
            result = C2Rust_Unnamed::STORAGE_S3_ERROR.0;
        } else {
            snprintf(
                &raw mut key as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
                b".lightnvr-probes/%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut uuid as *mut ::core::ffi::c_char,
            );
            fputs(b"1\0".as_ptr() as *const ::core::ffi::c_char, payload);
            rewind(payload);
            let mut response: s3_io_t = s3_io_t {
                body: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                length: 0,
                file: ::core::ptr::null_mut::<FILE>(),
                digest: ::core::ptr::null_mut::<mbedtls_sha256_context>(),
                bytes: 0,
                limit: S3_REPLY_LIMIT as uint64_t,
                control: ::core::ptr::null::<storage_transfer_control_t>(),
                status: 0,
                etag: [0; 192],
                range: [0; 64],
                content_range: [0; 128],
                data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                anonymous: false,
            };
            let mut checksum: *const ::core::ffi::c_char =
                b"6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b\0".as_ptr()
                    as *const ::core::ffi::c_char;
            result = request(
                target,
                &raw mut key as *mut ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"PUT\0".as_ptr() as *const ::core::ffi::c_char,
                payload,
                1 as uint64_t,
                checksum,
                &raw mut response,
                ::core::ptr::null_mut::<uint64_t>(),
                &raw mut error as *mut ::core::ffi::c_char,
            );
            if result == 0 as ::core::ffi::c_int {
                result = storage_s3_verify(
                    target,
                    &raw mut key as *mut ::core::ffi::c_char,
                    1 as uint64_t,
                    checksum,
                    ::core::ptr::null::<storage_transfer_control_t>(),
                    &raw mut error as *mut ::core::ffi::c_char,
                );
            }
            if result == 0 as ::core::ffi::c_int {
                let mut anonymous: s3_io_t = s3_io_t {
                    body: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    length: 0,
                    file: ::core::ptr::null_mut::<FILE>(),
                    digest: ::core::ptr::null_mut::<mbedtls_sha256_context>(),
                    bytes: 0,
                    limit: S3_REPLY_LIMIT as uint64_t,
                    control: ::core::ptr::null::<storage_transfer_control_t>(),
                    status: 0,
                    etag: [0; 192],
                    range: [0; 64],
                    content_range: [0; 128],
                    data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                    anonymous: r#true != 0,
                };
                request(
                    target,
                    &raw mut key as *mut ::core::ffi::c_char,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"GET\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<FILE>(),
                    0 as uint64_t,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    &raw mut anonymous,
                    ::core::ptr::null_mut::<uint64_t>(),
                    &raw mut error as *mut ::core::ffi::c_char,
                );
                if anonymous.status != 401 as ::core::ffi::c_long
                    && anonymous.status != 403 as ::core::ffi::c_long
                    && anonymous.status != 404 as ::core::ffi::c_long
                {
                    error_set(
                        &raw mut error as *mut ::core::ffi::c_char,
                        b"Archive probe is publicly readable or private access could not be verified\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    result = C2Rust_Unnamed::STORAGE_S3_CONFLICT.0;
                }
            }
            let mut cleanup: ::core::ffi::c_int = storage_s3_delete(
                target,
                &raw mut key as *mut ::core::ffi::c_char,
                &raw mut error as *mut ::core::ffi::c_char,
            );
            if cleanup != 0 as ::core::ffi::c_int {
                result = cleanup;
            }
        }
        if !payload.is_null() {
            fclose(payload);
        }
    }
    (*target).capacity_bytes = 0 as uint64_t;
    (*target).available_bytes = 0 as uint64_t;
    (*target).filesystem_device = 0 as uint64_t;
    (*target).last_probe_at = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    if result == 0 as ::core::ffi::c_int {
        (*target).last_success_at = (*target).last_probe_at;
    }
    safe_strcpy(
        &raw mut (*target).health_status as *mut ::core::ffi::c_char,
        if result == 0 as ::core::ffi::c_int {
            b"healthy\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"unavailable\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*target).last_error as *mut ::core::ffi::c_char,
        if result == 0 as ::core::ffi::c_int {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        } else if *(&raw mut error as *mut ::core::ffi::c_char) as ::core::ffi::c_int != 0 {
            &raw mut error as *mut ::core::ffi::c_char
        } else {
            b"Archive probe failed\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    return result;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PATH_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
