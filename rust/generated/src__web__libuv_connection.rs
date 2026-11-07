unsafe extern "C" {
    pub type internal_state;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn llhttp_init(
        parser: *mut llhttp_t,
        r#type: llhttp_type_t,
        settings: *const llhttp_settings_t,
    );
    unsafe fn llhttp_get_method(parser: *mut llhttp_t) -> uint8_t;
    unsafe fn llhttp_reset(parser: *mut llhttp_t);
    unsafe fn llhttp_settings_init(settings: *mut llhttp_settings_t);
    unsafe fn llhttp_execute(
        parser: *mut llhttp_t,
        data: *const ::core::ffi::c_char,
        len: size_t,
    ) -> llhttp_errno_t;
    unsafe fn llhttp_should_keep_alive(parser: *const llhttp_t) -> ::core::ffi::c_int;
    unsafe fn llhttp_get_error_reason(parser: *const llhttp_t) -> *const ::core::ffi::c_char;
    unsafe fn llhttp_errno_name(err: llhttp_errno_t) -> *const ::core::ffi::c_char;
    unsafe fn llhttp_method_name(method: llhttp_method_t) -> *const ::core::ffi::c_char;
    unsafe fn uv_strerror(err: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn uv_close(handle: *mut uv_handle_t, close_cb: uv_close_cb);
    unsafe fn uv_is_closing(handle: *const uv_handle_t) -> ::core::ffi::c_int;
    unsafe fn uv_tcp_init(_: *mut uv_loop_t, handle: *mut uv_tcp_t) -> ::core::ffi::c_int;
    unsafe fn uv_queue_work(
        r#loop: *mut uv_loop_t,
        req: *mut uv_work_t,
        work_cb: uv_work_cb,
        after_work_cb: uv_after_work_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn deflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn deflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    unsafe fn deflateBound(strm: z_streamp, sourceLen: uLong) -> uLong;
    unsafe fn deflateInit2_(
        strm: z_streamp,
        level: ::core::ffi::c_int,
        method: ::core::ffi::c_int,
        windowBits: ::core::ffi::c_int,
        memLevel: ::core::ffi::c_int,
        strategy: ::core::ffi::c_int,
        version: *const ::core::ffi::c_char,
        stream_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn safe_realloc(ptr: *mut ::core::ffi::c_void, size: size_t)
    -> *mut ::core::ffi::c_void;
    unsafe fn safe_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn safe_free(ptr: *mut ::core::ffi::c_void);
    static mut g_config: config_t;
    unsafe fn http_request_get_header(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
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
    unsafe fn http_request_init(req: *mut http_request_t);
    unsafe fn http_request_set_request_id(
        req: *mut http_request_t,
        request_id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_init(res: *mut http_response_t);
    unsafe fn http_response_free(res: *mut http_response_t);
    unsafe fn url_decode(
        src: *const ::core::ffi::c_char,
        dst: *mut ::core::ffi::c_char,
        dst_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_serve_file(
        conn: *mut libuv_connection_t,
        path: *const ::core::ffi::c_char,
        content_type: *const ::core::ffi::c_char,
        extra_headers: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_send_response_ex(
        conn: *mut libuv_connection_t,
        response: *const http_response_t,
        action: write_complete_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_tls_session_free(conn: *mut libuv_connection);
    unsafe fn libuv_connection_read_start(
        conn: *mut libuv_connection,
        alloc_cb: uv_alloc_cb,
        read_cb: uv_read_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_connection_read_stop(conn: *mut libuv_connection) -> ::core::ffi::c_int;
    unsafe fn libuv_get_mime_type(path: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    unsafe fn recording_archive_start(conn: *mut libuv_connection_t) -> ::core::ffi::c_int;
    unsafe fn recording_archive_disconnected(conn: *mut libuv_connection_t);
    unsafe fn handle_go2rtc_proxy(req: *const http_request_t, res: *mut http_response_t);
    unsafe fn go2rtc_proxy_thread_submit(
        conn: *mut libuv_connection_t,
        action: write_complete_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_websocket_disconnected(conn: *mut libuv_connection_t);
    unsafe fn update_health_metrics(request_succeeded: bool);
    unsafe fn httpd_peer_is_trusted_proxy(req: *const http_request_t) -> bool;
    unsafe fn audit_log_sensitive_operation_begin(
        req: *const http_request_t,
        context: *mut audit_sensitive_operation_context_t,
    );
    unsafe fn audit_log_sensitive_operation_end(
        req: *const http_request_t,
        res: *const http_response_t,
        context: *mut audit_sensitive_operation_context_t,
    );
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
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
    unsafe fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type clockid_t = __clockid_t;
pub type int32_t = i32;
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type uintptr_t = usize;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct llhttp_errno(pub ::core::ffi::c_uint);
impl llhttp_errno {
    pub const HPE_OK: Self = Self(0);
    pub const HPE_INTERNAL: Self = Self(1);
    pub const HPE_STRICT: Self = Self(2);
    pub const HPE_LF_EXPECTED: Self = Self(3);
    pub const HPE_UNEXPECTED_CONTENT_LENGTH: Self = Self(4);
    pub const HPE_CLOSED_CONNECTION: Self = Self(5);
    pub const HPE_INVALID_METHOD: Self = Self(6);
    pub const HPE_INVALID_URL: Self = Self(7);
    pub const HPE_INVALID_CONSTANT: Self = Self(8);
    pub const HPE_INVALID_VERSION: Self = Self(9);
    pub const HPE_INVALID_HEADER_TOKEN: Self = Self(10);
    pub const HPE_INVALID_CONTENT_LENGTH: Self = Self(11);
    pub const HPE_INVALID_CHUNK_SIZE: Self = Self(12);
    pub const HPE_INVALID_STATUS: Self = Self(13);
    pub const HPE_INVALID_EOF_STATE: Self = Self(14);
    pub const HPE_INVALID_TRANSFER_ENCODING: Self = Self(15);
    pub const HPE_CB_MESSAGE_BEGIN: Self = Self(16);
    pub const HPE_CB_HEADERS_COMPLETE: Self = Self(17);
    pub const HPE_CB_MESSAGE_COMPLETE: Self = Self(18);
    pub const HPE_CB_CHUNK_HEADER: Self = Self(19);
    pub const HPE_CB_CHUNK_COMPLETE: Self = Self(20);
    pub const HPE_PAUSED: Self = Self(21);
    pub const HPE_PAUSED_UPGRADE: Self = Self(22);
    pub const HPE_PAUSED_H2_UPGRADE: Self = Self(23);
    pub const HPE_USER: Self = Self(24);
    pub const HPE_CR_EXPECTED: Self = Self(25);
    pub const HPE_CB_URL_COMPLETE: Self = Self(26);
    pub const HPE_CB_STATUS_COMPLETE: Self = Self(27);
    pub const HPE_CB_HEADER_FIELD_COMPLETE: Self = Self(28);
    pub const HPE_CB_HEADER_VALUE_COMPLETE: Self = Self(29);
    pub const HPE_UNEXPECTED_SPACE: Self = Self(30);
    pub const HPE_CB_RESET: Self = Self(31);
    pub const HPE_CB_METHOD_COMPLETE: Self = Self(32);
    pub const HPE_CB_VERSION_COMPLETE: Self = Self(33);
    pub const HPE_CB_CHUNK_EXTENSION_NAME_COMPLETE: Self = Self(34);
    pub const HPE_CB_CHUNK_EXTENSION_VALUE_COMPLETE: Self = Self(35);
    pub const HPE_CB_PROTOCOL_COMPLETE: Self = Self(38);
}
pub type llhttp_errno_t = llhttp_errno;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct llhttp_type(pub ::core::ffi::c_uint);
impl llhttp_type {
    pub const HTTP_BOTH: Self = Self(0);
    pub const HTTP_REQUEST: Self = Self(1);
    pub const HTTP_RESPONSE: Self = Self(2);
}
pub type llhttp_type_t = llhttp_type;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct llhttp_method(pub ::core::ffi::c_uint);
impl llhttp_method {
    pub const HTTP_DELETE: Self = Self(0);
    pub const HTTP_GET: Self = Self(1);
    pub const HTTP_HEAD: Self = Self(2);
    pub const HTTP_POST: Self = Self(3);
    pub const HTTP_PUT: Self = Self(4);
    pub const HTTP_CONNECT: Self = Self(5);
    pub const HTTP_OPTIONS: Self = Self(6);
    pub const HTTP_TRACE: Self = Self(7);
    pub const HTTP_COPY: Self = Self(8);
    pub const HTTP_LOCK: Self = Self(9);
    pub const HTTP_MKCOL: Self = Self(10);
    pub const HTTP_MOVE: Self = Self(11);
    pub const HTTP_PROPFIND: Self = Self(12);
    pub const HTTP_PROPPATCH: Self = Self(13);
    pub const HTTP_SEARCH: Self = Self(14);
    pub const HTTP_UNLOCK: Self = Self(15);
    pub const HTTP_BIND: Self = Self(16);
    pub const HTTP_REBIND: Self = Self(17);
    pub const HTTP_UNBIND: Self = Self(18);
    pub const HTTP_ACL: Self = Self(19);
    pub const HTTP_REPORT: Self = Self(20);
    pub const HTTP_MKACTIVITY: Self = Self(21);
    pub const HTTP_CHECKOUT: Self = Self(22);
    pub const HTTP_MERGE: Self = Self(23);
    pub const HTTP_MSEARCH: Self = Self(24);
    pub const HTTP_NOTIFY: Self = Self(25);
    pub const HTTP_SUBSCRIBE: Self = Self(26);
    pub const HTTP_UNSUBSCRIBE: Self = Self(27);
    pub const HTTP_PATCH: Self = Self(28);
    pub const HTTP_PURGE: Self = Self(29);
    pub const HTTP_MKCALENDAR: Self = Self(30);
    pub const HTTP_LINK: Self = Self(31);
    pub const HTTP_UNLINK: Self = Self(32);
    pub const HTTP_SOURCE: Self = Self(33);
    pub const HTTP_PRI: Self = Self(34);
    pub const HTTP_DESCRIBE: Self = Self(35);
    pub const HTTP_ANNOUNCE: Self = Self(36);
    pub const HTTP_SETUP: Self = Self(37);
    pub const HTTP_PLAY: Self = Self(38);
    pub const HTTP_PAUSE: Self = Self(39);
    pub const HTTP_TEARDOWN: Self = Self(40);
    pub const HTTP_GET_PARAMETER: Self = Self(41);
    pub const HTTP_SET_PARAMETER: Self = Self(42);
    pub const HTTP_REDIRECT: Self = Self(43);
    pub const HTTP_RECORD: Self = Self(44);
    pub const HTTP_FLUSH: Self = Self(45);
    pub const HTTP_QUERY: Self = Self(46);
}
pub type llhttp_method_t = llhttp_method;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__queue {
    pub next: *mut uv__queue,
    pub prev: *mut uv__queue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__work {
    pub work: Option<unsafe extern "C" fn(*mut uv__work) -> ()>,
    pub done: Option<unsafe extern "C" fn(*mut uv__work, ::core::ffi::c_int) -> ()>,
    pub r#loop: *mut uv_loop_s,
    pub wq: uv__queue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_loop_s {
    pub data: *mut ::core::ffi::c_void,
    pub active_handles: ::core::ffi::c_uint,
    pub handle_queue: uv__queue,
    pub active_reqs: C2Rust_Unnamed_4,
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
    pub timer_heap: C2Rust_Unnamed_2,
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
    pub u: C2Rust_Unnamed_1,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: ::core::ffi::c_int,
    pub tree_entry: C2Rust_Unnamed,
    pub caught_signals: ::core::ffi::c_uint,
    pub dispatched_signals: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
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
    pub u: C2Rust_Unnamed_0,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
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
pub union C2Rust_Unnamed_1 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
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
    pub u: C2Rust_Unnamed_3,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub async_cb: uv_async_cb,
    pub queue: uv__queue,
    pub pending: ::core::ffi::c_int,
}
pub type uv_async_cb = Option<unsafe extern "C" fn(*mut uv_async_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_mutex_t = pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
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
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_5 {
    pub const UV_E2BIG: Self = Self(-7);
    pub const UV_EACCES: Self = Self(-13);
    pub const UV_EADDRINUSE: Self = Self(-98);
    pub const UV_EADDRNOTAVAIL: Self = Self(-99);
    pub const UV_EAFNOSUPPORT: Self = Self(-97);
    pub const UV_EAGAIN: Self = Self(-11);
    pub const UV_EAI_ADDRFAMILY: Self = Self(-3000);
    pub const UV_EAI_AGAIN: Self = Self(-3001);
    pub const UV_EAI_BADFLAGS: Self = Self(-3002);
    pub const UV_EAI_BADHINTS: Self = Self(-3013);
    pub const UV_EAI_CANCELED: Self = Self(-3003);
    pub const UV_EAI_FAIL: Self = Self(-3004);
    pub const UV_EAI_FAMILY: Self = Self(-3005);
    pub const UV_EAI_MEMORY: Self = Self(-3006);
    pub const UV_EAI_NODATA: Self = Self(-3007);
    pub const UV_EAI_NONAME: Self = Self(-3008);
    pub const UV_EAI_OVERFLOW: Self = Self(-3009);
    pub const UV_EAI_PROTOCOL: Self = Self(-3014);
    pub const UV_EAI_SERVICE: Self = Self(-3010);
    pub const UV_EAI_SOCKTYPE: Self = Self(-3011);
    pub const UV_EALREADY: Self = Self(-114);
    pub const UV_EBADF: Self = Self(-9);
    pub const UV_EBUSY: Self = Self(-16);
    pub const UV_ECANCELED: Self = Self(-125);
    pub const UV_ECHARSET: Self = Self(-4080);
    pub const UV_ECONNABORTED: Self = Self(-103);
    pub const UV_ECONNREFUSED: Self = Self(-111);
    pub const UV_ECONNRESET: Self = Self(-104);
    pub const UV_EDESTADDRREQ: Self = Self(-89);
    pub const UV_EEXIST: Self = Self(-17);
    pub const UV_EFAULT: Self = Self(-14);
    pub const UV_EFBIG: Self = Self(-27);
    pub const UV_EHOSTUNREACH: Self = Self(-113);
    pub const UV_EINTR: Self = Self(-4);
    pub const UV_EINVAL: Self = Self(-22);
    pub const UV_EIO: Self = Self(-5);
    pub const UV_EISCONN: Self = Self(-106);
    pub const UV_EISDIR: Self = Self(-21);
    pub const UV_ELOOP: Self = Self(-40);
    pub const UV_EMFILE: Self = Self(-24);
    pub const UV_EMSGSIZE: Self = Self(-90);
    pub const UV_ENAMETOOLONG: Self = Self(-36);
    pub const UV_ENETDOWN: Self = Self(-100);
    pub const UV_ENETUNREACH: Self = Self(-101);
    pub const UV_ENFILE: Self = Self(-23);
    pub const UV_ENOBUFS: Self = Self(-105);
    pub const UV_ENODEV: Self = Self(-19);
    pub const UV_ENOENT: Self = Self(-2);
    pub const UV_ENOMEM: Self = Self(-12);
    pub const UV_ENONET: Self = Self(-64);
    pub const UV_ENOPROTOOPT: Self = Self(-92);
    pub const UV_ENOSPC: Self = Self(-28);
    pub const UV_ENOSYS: Self = Self(-38);
    pub const UV_ENOTCONN: Self = Self(-107);
    pub const UV_ENOTDIR: Self = Self(-20);
    pub const UV_ENOTEMPTY: Self = Self(-39);
    pub const UV_ENOTSOCK: Self = Self(-88);
    pub const UV_ENOTSUP: Self = Self(-95);
    pub const UV_EOVERFLOW: Self = Self(-75);
    pub const UV_EPERM: Self = Self(-1);
    pub const UV_EPIPE: Self = Self(-32);
    pub const UV_EPROTO: Self = Self(-71);
    pub const UV_EPROTONOSUPPORT: Self = Self(-93);
    pub const UV_EPROTOTYPE: Self = Self(-91);
    pub const UV_ERANGE: Self = Self(-34);
    pub const UV_EROFS: Self = Self(-30);
    pub const UV_ESHUTDOWN: Self = Self(-108);
    pub const UV_ESPIPE: Self = Self(-29);
    pub const UV_ESRCH: Self = Self(-3);
    pub const UV_ETIMEDOUT: Self = Self(-110);
    pub const UV_ETXTBSY: Self = Self(-26);
    pub const UV_EXDEV: Self = Self(-18);
    pub const UV_UNKNOWN: Self = Self(-4094);
    pub const UV_EOF: Self = Self(-4095);
    pub const UV_ENXIO: Self = Self(-6);
    pub const UV_EMLINK: Self = Self(-31);
    pub const UV_EHOSTDOWN: Self = Self(-112);
    pub const UV_EREMOTEIO: Self = Self(-121);
    pub const UV_ENOTTY: Self = Self(-25);
    pub const UV_EFTYPE: Self = Self(-134);
    pub const UV_EILSEQ: Self = Self(-84);
    pub const UV_ESOCKTNOSUPPORT: Self = Self(-94);
    pub const UV_ENODATA: Self = Self(-61);
    pub const UV_EUNATCH: Self = Self(-49);
    pub const UV_ENOEXEC: Self = Self(-8);
    pub const UV_ERRNO_MAX: Self = Self(-4096);
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
    pub u: C2Rust_Unnamed_8,
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
    pub c2rust_unnamed: C2Rust_Unnamed_6,
    pub handle: *mut uv_stream_t,
    pub cb: uv_shutdown_cb,
}
pub type uv_shutdown_cb =
    Option<unsafe extern "C" fn(*mut uv_shutdown_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_7,
    pub cb: uv_connect_cb,
    pub handle: *mut uv_stream_t,
    pub queue: uv__queue,
}
pub type uv_connect_cb = Option<unsafe extern "C" fn(*mut uv_connect_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
}
pub type uv_read_cb =
    Option<unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> ()>;
pub type uv_alloc_cb = Option<unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_tcp_t = uv_tcp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_work_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_10,
    pub r#loop: *mut uv_loop_t,
    pub work_cb: uv_work_cb,
    pub after_work_cb: uv_after_work_cb,
    pub work_req: uv__work,
}
pub type uv_after_work_cb = Option<unsafe extern "C" fn(*mut uv_work_t, ::core::ffi::c_int) -> ()>;
pub type uv_work_t = uv_work_s;
pub type uv_work_cb = Option<unsafe extern "C" fn(*mut uv_work_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
}
pub type Byte = ::core::ffi::c_uchar;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut ::core::ffi::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut ::core::ffi::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: ::core::ffi::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
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
pub struct handler_work_t {
    pub work: uv_work_t,
    pub conn: *mut libuv_connection_t,
    pub handler: request_handler_t,
    pub action: write_complete_action_t,
    pub audit_context: audit_sensitive_operation_context_t,
    pub queued_timer: api_phase_timer_t,
    pub completion_timer: api_phase_timer_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_phase_timer_t {
    pub started: timespec,
    pub request: *const http_request_t,
    pub phase: *const ::core::ffi::c_char,
    pub valid: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_sensitive_operation_context_t {
    pub applicable: bool,
    pub has_user: bool,
    pub identity_is_camera_uuid: bool,
    pub action: authorization_action_t,
    pub user: user_t,
    pub operation: [::core::ffi::c_char; 48],
    pub target_type: [::core::ffi::c_char; 32],
    pub identity: [::core::ffi::c_char; 256],
    pub target_uuid: [::core::ffi::c_char; 37],
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
    pub const AUTHZ_ACTION_COUNT: Self = Self(19);
}
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ZLIB_VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"1.3.2\0") };
pub const Z_FINISH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_STREAM_END: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAX_HEADERS: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"HTTP\0") };
pub const HTTP_COMPRESSION_MIN_BYTES: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
unsafe extern "C" fn request_accepts_gzip(mut request: *const http_request_t) -> bool {
    let mut header: *const ::core::ffi::c_char = http_request_get_header(
        request,
        b"Accept-Encoding\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if header.is_null() {
        return r#false != 0;
    }
    let mut copy: [::core::ffi::c_char; 1024] = [0; 1024];
    safe_strcpy(
        &raw mut copy as *mut ::core::ffi::c_char,
        header,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        0 as size_t,
    );
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wildcard_allowed: bool = r#false != 0;
    let mut token: *mut ::core::ffi::c_char = strtok_r(
        &raw mut copy as *mut ::core::ffi::c_char,
        b",\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !token.is_null() {
        while *token as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *token as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            token = token.offset(1);
        }
        let mut parameters: *mut ::core::ffi::c_char = strchr(token, ';' as ::core::ffi::c_int);
        if !parameters.is_null() {
            let c2rust_fresh0 = parameters;
            parameters = parameters.offset(1);
            *c2rust_fresh0 = '\0' as ::core::ffi::c_char;
        }
        let mut end: *mut ::core::ffi::c_char = token.offset(strlen(token) as isize);
        while end > token
            && (*end.offset(-1isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *end.offset(-1isize) as ::core::ffi::c_int == '\t' as ::core::ffi::c_int)
        {
            end = end.offset(-1);
            *end = '\0' as ::core::ffi::c_char;
        }
        if !(strcasecmp(token, b"gzip\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
            && strcmp(token, b"*\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int)
        {
            let mut quality: ::core::ffi::c_double = 1.0f64;
            let mut parameter: *mut ::core::ffi::c_char = parameters;
            while !parameter.is_null() {
                let mut next: *mut ::core::ffi::c_char =
                    strchr(parameter, ';' as ::core::ffi::c_int);
                if !next.is_null() {
                    let c2rust_fresh1 = next;
                    next = next.offset(1);
                    *c2rust_fresh1 = '\0' as ::core::ffi::c_char;
                }
                while *parameter as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    || *parameter as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                {
                    parameter = parameter.offset(1);
                }
                if strncasecmp(
                    parameter,
                    b"q=\0".as_ptr() as *const ::core::ffi::c_char,
                    2 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    let mut quality_end: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    quality = strtod(
                        parameter.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut quality_end,
                    );
                    if quality_end == parameter.offset(2 as ::core::ffi::c_int as isize)
                        || quality < 0.0f64
                        || quality > 1.0f64
                    {
                        quality = 0.0f64;
                    }
                }
                parameter = next;
            }
            if strcasecmp(token, b"gzip\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                return quality > 0.0f64;
            }
            wildcard_allowed = quality > 0.0f64;
        }
        token = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    return wildcard_allowed;
}
unsafe extern "C" fn response_content_is_compressible(
    mut content_type: *const ::core::ffi::c_char,
) -> bool {
    return !content_type.is_null()
        && (strncasecmp(
            content_type,
            b"text/\0".as_ptr() as *const ::core::ffi::c_char,
            5 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncasecmp(
                content_type,
                b"application/json\0".as_ptr() as *const ::core::ffi::c_char,
                16 as size_t,
            ) == 0 as ::core::ffi::c_int
            || strncasecmp(
                content_type,
                b"application/javascript\0".as_ptr() as *const ::core::ffi::c_char,
                22 as size_t,
            ) == 0 as ::core::ffi::c_int
            || strncasecmp(
                content_type,
                b"application/xml\0".as_ptr() as *const ::core::ffi::c_char,
                15 as size_t,
            ) == 0 as ::core::ffi::c_int
            || strncasecmp(
                content_type,
                b"image/svg+xml\0".as_ptr() as *const ::core::ffi::c_char,
                13 as size_t,
            ) == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn response_has_header(
    mut response: *const http_response_t,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*response).num_headers {
        if strcasecmp(
            &raw const (*(&raw const (*response).headers as *const http_header_t)
                .offset(i as isize))
            .name as *const ::core::ffi::c_char,
            name,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn maybe_compress_response(
    mut request: *const http_request_t,
    mut response: *mut http_response_t,
) {
    if !g_config.web_compression_enabled
        || !request_accepts_gzip(request)
        || response.is_null()
        || (*response).body.is_null()
        || (*response).body_length < HTTP_COMPRESSION_MIN_BYTES as size_t
        || !response_content_is_compressible(
            &raw mut (*response).content_type as *mut ::core::ffi::c_char,
        )
        || response_has_header(
            response,
            b"Content-Encoding\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        || (*response).status_code == 204 as ::core::ffi::c_int
        || (*response).status_code == 206 as ::core::ffi::c_int
        || (*response).status_code == 304 as ::core::ffi::c_int
        || (*request).method.0 == http_method_t::HTTP_METHOD_HEAD.0
    {
        return;
    }
    let mut stream: z_stream = z_stream_s {
        next_in: ::core::ptr::null_mut::<Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: ::core::ptr::null_mut::<Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        state: ::core::ptr::null_mut::<internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    if deflateInit2_(
        &raw mut stream,
        -1 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
    ) != Z_OK
    {
        return;
    }
    let mut bound: uLong = deflateBound(&raw mut stream, (*response).body_length as uLong);
    let mut compressed: *mut ::core::ffi::c_uchar =
        safe_malloc(bound as size_t) as *mut ::core::ffi::c_uchar;
    if compressed.is_null() {
        deflateEnd(&raw mut stream);
        return;
    }
    stream.next_in = (*response).body as *mut Bytef;
    stream.avail_in = (*response).body_length as uInt;
    stream.next_out = compressed as *mut Bytef;
    stream.avail_out = bound as uInt;
    let mut rc: ::core::ffi::c_int = deflate(&raw mut stream, Z_FINISH);
    let mut compressed_length: size_t = stream.total_out as size_t;
    deflateEnd(&raw mut stream);
    if rc != Z_STREAM_END || compressed_length >= (*response).body_length {
        safe_free(compressed as *mut ::core::ffi::c_void);
        return;
    }
    if (*response).body_allocated {
        safe_free((*response).body);
    }
    (*response).body = compressed as *mut ::core::ffi::c_void;
    (*response).body_length = compressed_length;
    (*response).body_allocated = r#true != 0;
    http_response_add_header(
        response,
        b"Content-Encoding\0".as_ptr() as *const ::core::ffi::c_char,
        b"gzip\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_add_header(
        response,
        b"Vary\0".as_ptr() as *const ::core::ffi::c_char,
        b"Accept-Encoding\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_connection_create(
    mut server: *mut libuv_server_t,
) -> *mut libuv_connection_t {
    let mut conn: *mut libuv_connection_t =
        safe_calloc(1 as size_t, ::core::mem::size_of::<libuv_connection_t>())
            as *mut libuv_connection_t;
    if conn.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_connection_create: Failed to allocate connection\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<libuv_connection_t>();
    }
    if uv_tcp_init((*server).r#loop, &raw mut (*conn).handle) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_connection_create: Failed to init TCP handle\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        safe_free(conn as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<libuv_connection_t>();
    }
    (*conn).handle.data = conn as *mut ::core::ffi::c_void;
    (*conn).server = server;
    (*conn).recv_buffer = safe_malloc(LIBUV_RECV_BUFFER_INITIAL) as *mut ::core::ffi::c_char;
    if (*conn).recv_buffer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_connection_create: Failed to allocate receive buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        uv_close(
            &raw mut (*conn).handle as *mut uv_handle_t,
            Some(libuv_close_cb as unsafe extern "C" fn(*mut uv_handle_t) -> ()),
        );
        return ::core::ptr::null_mut::<libuv_connection_t>();
    }
    (*conn).recv_buffer_size = LIBUV_RECV_BUFFER_INITIAL;
    (*conn).recv_buffer_used = 0 as size_t;
    llhttp_settings_init(&raw mut (*conn).settings);
    (*conn).settings.on_url = Some(
        on_url
            as unsafe extern "C" fn(
                *mut llhttp_t,
                *const ::core::ffi::c_char,
                size_t,
            ) -> ::core::ffi::c_int,
    ) as llhttp_data_cb;
    (*conn).settings.on_url_complete =
        Some(on_url_complete as unsafe extern "C" fn(*mut llhttp_t) -> ::core::ffi::c_int)
            as llhttp_cb;
    (*conn).settings.on_header_field = Some(
        on_header_field
            as unsafe extern "C" fn(
                *mut llhttp_t,
                *const ::core::ffi::c_char,
                size_t,
            ) -> ::core::ffi::c_int,
    ) as llhttp_data_cb;
    (*conn).settings.on_header_value = Some(
        on_header_value
            as unsafe extern "C" fn(
                *mut llhttp_t,
                *const ::core::ffi::c_char,
                size_t,
            ) -> ::core::ffi::c_int,
    ) as llhttp_data_cb;
    (*conn).settings.on_header_value_complete =
        Some(on_header_value_complete as unsafe extern "C" fn(*mut llhttp_t) -> ::core::ffi::c_int)
            as llhttp_cb;
    (*conn).settings.on_headers_complete =
        Some(on_headers_complete as unsafe extern "C" fn(*mut llhttp_t) -> ::core::ffi::c_int)
            as llhttp_cb;
    (*conn).settings.on_body = Some(
        on_body
            as unsafe extern "C" fn(
                *mut llhttp_t,
                *const ::core::ffi::c_char,
                size_t,
            ) -> ::core::ffi::c_int,
    ) as llhttp_data_cb;
    (*conn).settings.on_message_complete =
        Some(on_message_complete as unsafe extern "C" fn(*mut llhttp_t) -> ::core::ffi::c_int)
            as llhttp_cb;
    llhttp_init(
        &raw mut (*conn).parser,
        llhttp_type::HTTP_REQUEST,
        &raw mut (*conn).settings,
    );
    (*conn).parser.data = conn as *mut ::core::ffi::c_void;
    http_request_init(&raw mut (*conn).request);
    http_response_init(&raw mut (*conn).response);
    (*conn).keep_alive = r#true != 0;
    return conn;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_connection_reset(mut conn: *mut libuv_connection_t) {
    if conn.is_null() {
        return;
    }
    let mut client_ip: [::core::ffi::c_char; 64] = [0; 64];
    safe_strcpy(
        &raw mut client_ip as *mut ::core::ffi::c_char,
        &raw mut (*conn).request.client_ip as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    go2rtc_websocket_disconnected(conn);
    recording_archive_disconnected(conn);
    http_response_free(&raw mut (*conn).response);
    http_request_init(&raw mut (*conn).request);
    http_response_init(&raw mut (*conn).response);
    safe_strcpy(
        &raw mut (*conn).request.client_ip as *mut ::core::ffi::c_char,
        &raw mut client_ip as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    llhttp_reset(&raw mut (*conn).parser);
    (*conn).headers_complete = r#false != 0;
    (*conn).message_complete = r#false != 0;
    (*conn).current_header_field[0usize] = '\0' as ::core::ffi::c_char;
    (*conn).current_header_field_len = 0 as size_t;
    (*conn).recv_buffer_used = 0 as size_t;
    (*conn).body_offset = 0 as size_t;
    (*conn).body_present = r#false != 0;
    (*conn).handler_on_worker = r#false != 0;
    (*conn).deferred_file_serve = r#false != 0;
    (*conn).async_response_pending = r#false != 0;
    (*conn).deferred_action = write_complete_action_t::WRITE_ACTION_NONE;
    (*conn).requests_handled += 1;
    if uv_is_closing(&raw mut (*conn).handle as *mut uv_handle_t) == 0 {
        libuv_connection_read_start(
            conn,
            Some(
                libuv_alloc_cb
                    as unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> (),
            ),
            Some(
                libuv_read_cb
                    as unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> (),
            ),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_connection_close(mut conn: *mut libuv_connection_t) {
    if conn.is_null() {
        return;
    }
    go2rtc_websocket_disconnected(conn);
    if uv_is_closing(&raw mut (*conn).handle as *mut uv_handle_t) == 0 {
        libuv_connection_read_stop(conn);
        uv_close(
            &raw mut (*conn).handle as *mut uv_handle_t,
            Some(libuv_close_cb as unsafe extern "C" fn(*mut uv_handle_t) -> ()),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_connection_destroy(mut conn: *mut libuv_connection_t) {
    if conn.is_null() {
        return;
    }
    go2rtc_websocket_disconnected(conn);
    recording_archive_disconnected(conn);
    http_response_free(&raw mut (*conn).response);
    libuv_tls_session_free(conn);
    if !(*conn).recv_buffer.is_null() {
        safe_free((*conn).recv_buffer as *mut ::core::ffi::c_void);
    }
    safe_free(conn as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_close_cb(mut handle: *mut uv_handle_t) {
    let mut conn: *mut libuv_connection_t = (*handle).data as *mut libuv_connection_t;
    if !conn.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_close_cb: Connection closed after %d requests\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*conn).requests_handled,
        );
        (*conn).close_completed = r#true != 0;
        if !(*conn).handler_on_worker {
            libuv_connection_destroy(conn);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_alloc_cb(
    mut handle: *mut uv_handle_t,
    mut suggested_size: size_t,
    mut buf: *mut uv_buf_t,
) {
    let mut conn: *mut libuv_connection_t = (*handle).data as *mut libuv_connection_t;
    if conn.is_null() || (*conn).recv_buffer.is_null() {
        (*buf).base = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*buf).len = 0 as size_t;
        return;
    }
    let mut available: size_t = (*conn)
        .recv_buffer_size
        .wrapping_sub((*conn).recv_buffer_used);
    if available < 1024 as size_t {
        let mut new_size: size_t = (*conn).recv_buffer_size.wrapping_mul(2 as size_t);
        if new_size > LIBUV_RECV_BUFFER_MAX {
            new_size = LIBUV_RECV_BUFFER_MAX;
        }
        if new_size > (*conn).recv_buffer_size {
            let mut new_buf: *mut ::core::ffi::c_char =
                safe_realloc((*conn).recv_buffer as *mut ::core::ffi::c_void, new_size)
                    as *mut ::core::ffi::c_char;
            if !new_buf.is_null() {
                (*conn).recv_buffer = new_buf;
                (*conn).recv_buffer_size = new_size;
                available = new_size.wrapping_sub((*conn).recv_buffer_used);
            }
        }
    }
    (*buf).base = (*conn)
        .recv_buffer
        .offset((*conn).recv_buffer_used as isize);
    (*buf).len = available;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_read_cb(
    mut stream: *mut uv_stream_t,
    mut nread: ssize_t,
    mut buf: *const uv_buf_t,
) {
    let mut conn: *mut libuv_connection_t = (*stream).data as *mut libuv_connection_t;
    if conn.is_null() {
        return;
    }
    if nread < 0 as ssize_t {
        if nread != C2Rust_Unnamed_5::UV_EOF.0 as ssize_t {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_read_cb: Read error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                uv_strerror(nread as ::core::ffi::c_int),
            );
        }
        libuv_connection_close(conn);
        return;
    }
    if nread == 0 as ssize_t {
        return;
    }
    (*conn).recv_buffer_used = (*conn).recv_buffer_used.wrapping_add(nread as size_t);
    let mut err: llhttp_errno =
        llhttp_execute(&raw mut (*conn).parser, (*buf).base, nread as size_t);
    if err.0 != llhttp_errno::HPE_OK.0 {
        if err.0 == llhttp_errno::HPE_PAUSED.0 {
            return;
        }
        if err.0 == llhttp_errno::HPE_CLOSED_CONNECTION.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_read_cb: Connection closed by parser (expected after Connection: close)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            libuv_connection_close(conn);
            return;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_read_cb: Parse error: %s %s\0".as_ptr() as *const ::core::ffi::c_char,
            llhttp_errno_name(err),
            llhttp_get_error_reason(&raw mut (*conn).parser),
        );
        (*conn).response.status_code = 400 as ::core::ffi::c_int;
        http_response_set_json_error(
            &raw mut (*conn).response,
            400 as ::core::ffi::c_int,
            b"Bad Request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        libuv_send_response_ex(
            conn,
            &raw mut (*conn).response,
            write_complete_action_t::WRITE_ACTION_CLOSE,
        );
        return;
    }
}
unsafe extern "C" fn request_path_has_dot_segment(mut path: *const ::core::ffi::c_char) -> bool {
    if path.is_null() {
        return r#true != 0;
    }
    let mut decoded: [::core::ffi::c_char; 512] = [0; 512];
    if url_decode(
        path,
        &raw mut decoded as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    let mut segment: *const ::core::ffi::c_char = &raw mut decoded as *mut ::core::ffi::c_char;
    while *segment != 0 {
        let mut slash: *const ::core::ffi::c_char = strchr(segment, '/' as ::core::ffi::c_int);
        let mut length: size_t = if !slash.is_null() {
            slash.offset_from(segment) as size_t
        } else {
            strlen(segment)
        };
        if length == 1 as size_t
            && *segment.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || length == 2 as size_t
                && *segment.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && *segment.offset(1isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        if slash.is_null() {
            break;
        }
        segment = slash.offset(1 as ::core::ffi::c_int as isize);
    }
    return r#false != 0;
}
unsafe extern "C" fn on_url(
    mut parser: *mut llhttp_t,
    mut at: *const ::core::ffi::c_char,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut conn: *mut libuv_connection_t = (*parser).data as *mut libuv_connection_t;
    let mut used: size_t = strlen(&raw mut (*conn).request.uri as *mut ::core::ffi::c_char);
    if length >= ::core::mem::size_of::<[::core::ffi::c_char; 1536]>().wrapping_sub(used) {
        return llhttp_errno::HPE_USER.0 as ::core::ffi::c_int;
    }
    memcpy(
        (&raw mut (*conn).request.uri as *mut ::core::ffi::c_char).offset(used as isize)
            as *mut ::core::ffi::c_void,
        at as *const ::core::ffi::c_void,
        length,
    );
    (*conn).request.uri[used.wrapping_add(length)] = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn on_url_complete(mut parser: *mut llhttp_t) -> ::core::ffi::c_int {
    let mut conn: *mut libuv_connection_t = (*parser).data as *mut libuv_connection_t;
    let mut query: *const ::core::ffi::c_char = strchr(
        &raw mut (*conn).request.uri as *mut ::core::ffi::c_char,
        '?' as ::core::ffi::c_int,
    );
    if !query.is_null() {
        let mut path_len: size_t =
            query.offset_from(&raw mut (*conn).request.uri as *mut ::core::ffi::c_char) as size_t;
        if path_len >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
            || strlen(query.offset(1 as ::core::ffi::c_int as isize))
                >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
        {
            return llhttp_errno::HPE_USER.0 as ::core::ffi::c_int;
        }
        memcpy(
            &raw mut (*conn).request.path as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw mut (*conn).request.uri as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            path_len,
        );
        (*conn).request.path[path_len] = '\0' as ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*conn).request.query_string as *mut ::core::ffi::c_char,
            query.offset(1 as ::core::ffi::c_int as isize),
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            0 as size_t,
        );
    } else {
        if strlen(&raw mut (*conn).request.uri as *mut ::core::ffi::c_char)
            >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        {
            return llhttp_errno::HPE_USER.0 as ::core::ffi::c_int;
        }
        safe_strcpy(
            &raw mut (*conn).request.path as *mut ::core::ffi::c_char,
            &raw mut (*conn).request.uri as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        (*conn).request.query_string[0usize] = '\0' as ::core::ffi::c_char;
    }
    if request_path_has_dot_segment(&raw mut (*conn).request.path as *mut ::core::ffi::c_char) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_url: rejecting request path with dot segment: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*conn).request.path as *mut ::core::ffi::c_char,
        );
        return llhttp_errno::HPE_USER.0 as ::core::ffi::c_int;
    }
    let mut method: *const ::core::ffi::c_char = llhttp_method_name(llhttp_method(
        llhttp_get_method(parser) as ::core::ffi::c_uint,
    ));
    safe_strcpy(
        &raw mut (*conn).request.method_str as *mut ::core::ffi::c_char,
        method,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    if strcmp(method, b"GET\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        (*conn).request.method = http_method_t::HTTP_METHOD_GET;
    } else if strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        (*conn).request.method = http_method_t::HTTP_METHOD_POST;
    } else if strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        (*conn).request.method = http_method_t::HTTP_METHOD_PUT;
    } else if strcmp(method, b"DELETE\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        (*conn).request.method = http_method_t::HTTP_METHOD_DELETE;
    } else if strcmp(method, b"OPTIONS\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        (*conn).request.method = http_method_t::HTTP_METHOD_OPTIONS;
    } else if strcmp(method, b"HEAD\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        (*conn).request.method = http_method_t::HTTP_METHOD_HEAD;
    } else if strcmp(method, b"PATCH\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        (*conn).request.method = http_method_t::HTTP_METHOD_PATCH;
    } else {
        (*conn).request.method = http_method_t::HTTP_METHOD_UNKNOWN;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn on_header_field(
    mut parser: *mut llhttp_t,
    mut at: *const ::core::ffi::c_char,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut conn: *mut libuv_connection_t = (*parser).data as *mut libuv_connection_t;
    let mut used: size_t = (*conn).current_header_field_len;
    if length >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>().wrapping_sub(used) {
        return llhttp_errno::HPE_USER.0 as ::core::ffi::c_int;
    }
    memcpy(
        (&raw mut (*conn).current_header_field as *mut ::core::ffi::c_char).offset(used as isize)
            as *mut ::core::ffi::c_void,
        at as *const ::core::ffi::c_void,
        length,
    );
    (*conn).current_header_field_len = (*conn).current_header_field_len.wrapping_add(length);
    (*conn).current_header_field[(*conn).current_header_field_len] = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn on_header_value(
    mut parser: *mut llhttp_t,
    mut at: *const ::core::ffi::c_char,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut conn: *mut libuv_connection_t = (*parser).data as *mut libuv_connection_t;
    if (*conn).request.num_headers >= MAX_HEADERS {
        return llhttp_errno::HPE_USER.0 as ::core::ffi::c_int;
    }
    let mut value: *mut ::core::ffi::c_char = &raw mut (*(&raw mut (*conn).request.headers
        as *mut http_header_t)
        .offset((*conn).request.num_headers as isize))
    .value as *mut ::core::ffi::c_char;
    let mut used: size_t = strlen(value);
    if length >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>().wrapping_sub(used) {
        return llhttp_errno::HPE_USER.0 as ::core::ffi::c_int;
    }
    memcpy(
        value.offset(used as isize) as *mut ::core::ffi::c_void,
        at as *const ::core::ffi::c_void,
        length,
    );
    *value.offset(used.wrapping_add(length) as isize) = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn on_header_value_complete(mut parser: *mut llhttp_t) -> ::core::ffi::c_int {
    let mut conn: *mut libuv_connection_t = (*parser).data as *mut libuv_connection_t;
    if (*conn).request.num_headers >= MAX_HEADERS {
        return llhttp_errno::HPE_USER.0 as ::core::ffi::c_int;
    }
    let c2rust_fresh2 = (*conn).request.num_headers;
    (*conn).request.num_headers += 1;
    let mut idx: ::core::ffi::c_int = c2rust_fresh2;
    safe_strcpy(
        &raw mut (*(&raw mut (*conn).request.headers as *mut http_header_t).offset(idx as isize))
            .name as *mut ::core::ffi::c_char,
        &raw mut (*conn).current_header_field as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    if strcasecmp(
        &raw mut (*conn).current_header_field as *mut ::core::ffi::c_char,
        b"Content-Type\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut (*conn).request.content_type as *mut ::core::ffi::c_char,
            &raw mut (*(&raw mut (*conn).request.headers as *mut http_header_t)
                .offset(idx as isize))
            .value as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    } else if strcasecmp(
        &raw mut (*conn).current_header_field as *mut ::core::ffi::c_char,
        b"Content-Length\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*conn).request.content_length = strtoull(
            &raw mut (*(&raw mut (*conn).request.headers as *mut http_header_t)
                .offset(idx as isize))
            .value as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) as uint64_t;
    } else if strcasecmp(
        &raw mut (*conn).current_header_field as *mut ::core::ffi::c_char,
        b"User-Agent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut (*conn).request.user_agent as *mut ::core::ffi::c_char,
            &raw mut (*(&raw mut (*conn).request.headers as *mut http_header_t)
                .offset(idx as isize))
            .value as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    } else if strcasecmp(
        &raw mut (*conn).current_header_field as *mut ::core::ffi::c_char,
        b"Connection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*conn).keep_alive = strcasecmp(
            &raw mut (*(&raw mut (*conn).request.headers as *mut http_header_t)
                .offset(idx as isize))
            .value as *mut ::core::ffi::c_char,
            b"close\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int;
    } else if strcasecmp(
        &raw mut (*conn).current_header_field as *mut ::core::ffi::c_char,
        b"X-Request-ID\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if httpd_peer_is_trusted_proxy(&raw mut (*conn).request) {
            http_request_set_request_id(
                &raw mut (*conn).request,
                &raw mut (*(&raw mut (*conn).request.headers as *mut http_header_t)
                    .offset(idx as isize))
                .value as *mut ::core::ffi::c_char,
            );
        }
    }
    (*conn).current_header_field_len = 0 as size_t;
    (*conn).current_header_field[0usize] = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn on_headers_complete(mut parser: *mut llhttp_t) -> ::core::ffi::c_int {
    let mut conn: *mut libuv_connection_t = (*parser).data as *mut libuv_connection_t;
    (*conn).headers_complete = r#true != 0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn on_body(
    mut parser: *mut llhttp_t,
    mut at: *const ::core::ffi::c_char,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut conn: *mut libuv_connection_t = (*parser).data as *mut libuv_connection_t;
    if !(*conn).body_present {
        if at >= (*conn).recv_buffer
            && at
                <= (*conn)
                    .recv_buffer
                    .offset((*conn).recv_buffer_size as isize)
        {
            (*conn).body_offset = at.offset_from((*conn).recv_buffer) as size_t;
            (*conn).body_present = r#true != 0;
            (*conn).request.body_len = length;
        }
    } else {
        (*conn).request.body_len = (*conn).request.body_len.wrapping_add(length);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn path_matches_pattern(
    mut path: *const ::core::ffi::c_char,
    mut pattern: *const ::core::ffi::c_char,
) -> bool {
    let mut p: *const ::core::ffi::c_char = path;
    let mut pat: *const ::core::ffi::c_char = pattern;
    while *pat != 0 {
        if *pat as ::core::ffi::c_int == '*' as ::core::ffi::c_int {
            return r#true != 0;
        } else if *pat as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
            pat = pat.offset(1);
            while *p as ::core::ffi::c_int != 0
                && *p as ::core::ffi::c_int != '/' as ::core::ffi::c_int
            {
                p = p.offset(1);
            }
            if *pat == 0 {
                return *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int;
            }
            if *p == 0 {
                return r#false != 0;
            }
        } else if *pat as ::core::ffi::c_int == *p as ::core::ffi::c_int {
            pat = pat.offset(1);
            p = p.offset(1);
        } else {
            return r#false != 0;
        }
    }
    return *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        && *pat as ::core::ffi::c_int == '\0' as ::core::ffi::c_int;
}
unsafe extern "C" fn handler_work_cb(mut req: *mut uv_work_t) {
    let mut hw: *mut handler_work_t = (*req).data as *mut handler_work_t;
    api_phase_end(&raw mut (*hw).queued_timer);
    let mut execution: api_phase_timer_t = api_phase_begin(
        &raw mut (*(*hw).conn).request,
        b"handler execution\0".as_ptr() as *const ::core::ffi::c_char,
    );
    audit_log_sensitive_operation_begin(
        &raw mut (*(*hw).conn).request,
        &raw mut (*hw).audit_context,
    );
    (*hw).handler.expect("non-null function pointer")(
        &raw mut (*(*hw).conn).request,
        &raw mut (*(*hw).conn).response,
    );
    audit_log_sensitive_operation_end(
        &raw mut (*(*hw).conn).request,
        &raw mut (*(*hw).conn).response,
        &raw mut (*hw).audit_context,
    );
    maybe_compress_response(
        &raw mut (*(*hw).conn).request,
        &raw mut (*(*hw).conn).response,
    );
    api_phase_end(&raw mut execution);
    (*hw).completion_timer = api_phase_begin(
        &raw mut (*(*hw).conn).request,
        b"event loop completion wait\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn handler_after_work_cb(
    mut req: *mut uv_work_t,
    mut status: ::core::ffi::c_int,
) {
    let mut hw: *mut handler_work_t = (*req).data as *mut handler_work_t;
    if status != C2Rust_Unnamed_5::UV_ECANCELED.0 {
        api_phase_end(&raw mut (*hw).completion_timer);
    }
    let mut conn: *mut libuv_connection_t = (*hw).conn;
    let mut action: write_complete_action_t = (*hw).action;
    safe_free(hw as *mut ::core::ffi::c_void);
    (*conn).handler_on_worker = r#false != 0;
    if (*conn).close_completed {
        libuv_connection_destroy(conn);
        return;
    }
    if ::core::ptr::read_volatile::<bool>(&raw const (*(*conn).server).shutting_down)
        as ::core::ffi::c_int
        != 0
        || uv_is_closing(&raw mut (*conn).handle as *mut uv_handle_t) != 0
    {
        libuv_connection_close(conn);
        return;
    }
    if (*conn).request.request_id[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        http_response_add_header(
            &raw mut (*conn).response,
            b"X-Request-ID\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*conn).request.request_id as *mut ::core::ffi::c_char,
        );
    }
    if status == C2Rust_Unnamed_5::UV_ECANCELED.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"handler_after_work_cb: Work cancelled, closing connection\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        libuv_connection_close(conn);
        return;
    }
    if !(*conn).archive_stream.is_null() {
        if recording_archive_start(conn) == 0 as ::core::ffi::c_int {
            return;
        }
        libuv_send_response_ex(conn, &raw mut (*conn).response, action);
        return;
    }
    if (*conn).deferred_file_serve {
        (*conn).deferred_file_serve = r#false != 0;
        let mut ct: *const ::core::ffi::c_char =
            if (*conn).deferred_content_type[0usize] as ::core::ffi::c_int != 0 {
                &raw mut (*conn).deferred_content_type as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
        let mut eh: *const ::core::ffi::c_char =
            if (*conn).deferred_extra_headers[0usize] as ::core::ffi::c_int != 0 {
                &raw mut (*conn).deferred_extra_headers as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
        if libuv_serve_file(
            conn,
            &raw mut (*conn).deferred_file_path as *mut ::core::ffi::c_char,
            ct,
            eh,
        ) == 0 as ::core::ffi::c_int
        {
            update_health_metrics(r#true != 0);
            return;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"handler_after_work_cb: Deferred file serve failed for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*conn).deferred_file_path as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            &raw mut (*conn).response,
            500 as ::core::ffi::c_int,
            b"Failed to serve file\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*conn).async_response_pending {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"handler_after_work_cb: Async response pending, skipping response send\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    update_health_metrics((*conn).response.status_code < 500 as ::core::ffi::c_int);
    libuv_send_response_ex(conn, &raw mut (*conn).response, action);
}
unsafe extern "C" fn static_file_resolve_handler(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut conn: *mut libuv_connection_t = (*req).user_data as *mut libuv_connection_t;
    let mut server: *const libuv_server_t = (*conn).server;
    let mut file_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut file_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*server).config.web_root,
        &raw const (*req).path as *const ::core::ffi::c_char,
    );
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
    let mut original_exists: bool =
        stat(&raw mut file_path as *mut ::core::ffi::c_char, &raw mut st)
            == 0 as ::core::ffi::c_int;
    if original_exists as ::core::ffi::c_int != 0
        && st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
    {
        snprintf(
            &raw mut file_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s%s/index.html\0".as_ptr() as *const ::core::ffi::c_char,
            (*server).config.web_root,
            &raw const (*req).path as *const ::core::ffi::c_char,
        );
        original_exists = stat(&raw mut file_path as *mut ::core::ffi::c_char, &raw mut st)
            == 0 as ::core::ffi::c_int
            && st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t;
    }
    let mut may_serve_gzip: bool = g_config.web_compression_enabled as ::core::ffi::c_int != 0
        && request_accepts_gzip(req) as ::core::ffi::c_int != 0
        && http_request_get_header(req, b"Range\0".as_ptr() as *const ::core::ffi::c_char)
            .is_null();
    let mut gz_path: [::core::ffi::c_char; 512] = [0; 512];
    let mut gz_written: ::core::ffi::c_int = snprintf(
        &raw mut gz_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s.gz\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut file_path as *mut ::core::ffi::c_char,
    );
    let mut gz_stat: stat = stat {
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
    let mut gzip_exists: bool = may_serve_gzip as ::core::ffi::c_int != 0
        && gz_written > 0 as ::core::ffi::c_int
        && (gz_written as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        && stat(
            &raw mut gz_path as *mut ::core::ffi::c_char,
            &raw mut gz_stat,
        ) == 0 as ::core::ffi::c_int
        && gz_stat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t;
    if gzip_exists {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Serving gzip static file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut gz_path as *mut ::core::ffi::c_char,
        );
        safe_strcpy(
            &raw mut (*conn).deferred_file_path as *mut ::core::ffi::c_char,
            &raw mut gz_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*conn).deferred_content_type as *mut ::core::ffi::c_char,
            libuv_get_mime_type(&raw mut file_path as *mut ::core::ffi::c_char),
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*conn).deferred_extra_headers as *mut ::core::ffi::c_char,
            b"Content-Encoding: gzip\r\nVary: Accept-Encoding\r\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        (*conn).deferred_file_serve = r#true != 0;
    } else if original_exists {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Serving static file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        );
        safe_strcpy(
            &raw mut (*conn).deferred_file_path as *mut ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        (*conn).deferred_content_type[0usize] = '\0' as ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*conn).deferred_extra_headers as *mut ::core::ffi::c_char,
            b"Vary: Accept-Encoding\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        (*conn).deferred_file_serve = r#true != 0;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Static file not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Not Found\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn on_message_complete(mut parser: *mut llhttp_t) -> ::core::ffi::c_int {
    let mut conn: *mut libuv_connection_t = (*parser).data as *mut libuv_connection_t;
    (*conn).message_complete = r#true != 0;
    if (*conn).body_present {
        (*conn).request.body =
            (*conn).recv_buffer.offset((*conn).body_offset as isize) as *mut ::core::ffi::c_void;
    } else {
        (*conn).request.body = NULL;
        (*conn).request.body_len = 0 as size_t;
    }
    let mut server: *mut libuv_server_t = (*conn).server;
    let mut handler: request_handler_t = None;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    's_67: while i < (*server).handler_count {
        's_36: {
            if (*(*server).handlers.offset(i as isize)).method[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                if strcmp(
                    &raw mut (*(*server).handlers.offset(i as isize)).method
                        as *mut ::core::ffi::c_char,
                    &raw mut (*conn).request.method_str as *mut ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    break 's_36;
                }
            }
            if path_matches_pattern(
                &raw mut (*conn).request.path as *mut ::core::ffi::c_char,
                &raw mut (*(*server).handlers.offset(i as isize)).path as *mut ::core::ffi::c_char,
            ) {
                handler = (*(*server).handlers.offset(i as isize)).handler;
                break 's_67;
            }
        }
        i += 1;
    }
    let mut action: write_complete_action_t = write_complete_action_t(
        (if (*conn).keep_alive as ::core::ffi::c_int != 0 && llhttp_should_keep_alive(parser) != 0 {
            write_complete_action_t::WRITE_ACTION_KEEP_ALIVE.0 as ::core::ffi::c_int
        } else {
            write_complete_action_t::WRITE_ACTION_CLOSE.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*conn).request.user_data = conn as *mut ::core::ffi::c_void;
    if handler
        == Some(
            handle_go2rtc_proxy
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        )
        || strcmp(
            &raw mut (*conn).request.path as *mut ::core::ffi::c_char,
            b"/go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strncmp(
            &raw mut (*conn).request.path as *mut ::core::ffi::c_char,
            b"/go2rtc/\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        libuv_connection_read_stop(conn);
        if go2rtc_proxy_thread_submit(conn, action) == 0 as ::core::ffi::c_int {
            return llhttp_errno::HPE_PAUSED.0 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_message_complete: go2rtc proxy thread submit failed for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*conn).request.path as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            &raw mut (*conn).response,
            503 as ::core::ffi::c_int,
            b"Service temporarily unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        libuv_send_response_ex(conn, &raw mut (*conn).response, action);
        return llhttp_errno::HPE_OK.0 as ::core::ffi::c_int;
    }
    if handler.is_some() {
        libuv_connection_read_stop(conn);
        let mut hw: *mut handler_work_t =
            safe_malloc(::core::mem::size_of::<handler_work_t>()) as *mut handler_work_t;
        if hw.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"on_message_complete: Failed to allocate handler work context\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            http_response_set_json_error(
                &raw mut (*conn).response,
                500 as ::core::ffi::c_int,
                b"Internal Server Error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            libuv_send_response_ex(conn, &raw mut (*conn).response, action);
            return llhttp_errno::HPE_OK.0 as ::core::ffi::c_int;
        }
        (*hw).work.data = hw as *mut ::core::ffi::c_void;
        (*hw).conn = conn;
        (*hw).handler = handler;
        (*hw).action = action;
        (*hw).queued_timer = api_phase_begin(
            &raw mut (*conn).request,
            b"HTTP worker queue wait\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*conn).handler_on_worker = r#true != 0;
        (*conn).deferred_action = action;
        let mut r: ::core::ffi::c_int = uv_queue_work(
            (*server).r#loop,
            &raw mut (*hw).work,
            Some(handler_work_cb as unsafe extern "C" fn(*mut uv_work_t) -> ()),
            Some(
                handler_after_work_cb
                    as unsafe extern "C" fn(*mut uv_work_t, ::core::ffi::c_int) -> (),
            ),
        );
        if r != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"on_message_complete: uv_queue_work failed: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                uv_strerror(r),
            );
            (*conn).handler_on_worker = r#false != 0;
            safe_free(hw as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                &raw mut (*conn).response,
                500 as ::core::ffi::c_int,
                b"Internal Server Error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            libuv_send_response_ex(conn, &raw mut (*conn).response, action);
            return llhttp_errno::HPE_OK.0 as ::core::ffi::c_int;
        }
        return llhttp_errno::HPE_PAUSED.0 as ::core::ffi::c_int;
    } else {
        libuv_connection_read_stop(conn);
        let mut hw_0: *mut handler_work_t =
            safe_malloc(::core::mem::size_of::<handler_work_t>()) as *mut handler_work_t;
        if hw_0.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"on_message_complete: Failed to allocate static file work context\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            http_response_set_json_error(
                &raw mut (*conn).response,
                500 as ::core::ffi::c_int,
                b"Internal Server Error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            libuv_send_response_ex(conn, &raw mut (*conn).response, action);
            return llhttp_errno::HPE_OK.0 as ::core::ffi::c_int;
        }
        (*hw_0).work.data = hw_0 as *mut ::core::ffi::c_void;
        (*hw_0).conn = conn;
        (*hw_0).handler = Some(
            static_file_resolve_handler
                as unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> (),
        ) as request_handler_t;
        (*hw_0).action = action;
        (*hw_0).queued_timer = api_phase_begin(
            &raw mut (*conn).request,
            b"HTTP worker queue wait\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*conn).handler_on_worker = r#true != 0;
        (*conn).deferred_action = action;
        let mut r_0: ::core::ffi::c_int = uv_queue_work(
            (*server).r#loop,
            &raw mut (*hw_0).work,
            Some(handler_work_cb as unsafe extern "C" fn(*mut uv_work_t) -> ()),
            Some(
                handler_after_work_cb
                    as unsafe extern "C" fn(*mut uv_work_t, ::core::ffi::c_int) -> (),
            ),
        );
        if r_0 != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"on_message_complete: uv_queue_work failed for static file: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                uv_strerror(r_0),
            );
            (*conn).handler_on_worker = r#false != 0;
            safe_free(hw_0 as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                &raw mut (*conn).response,
                500 as ::core::ffi::c_int,
                b"Internal Server Error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            libuv_send_response_ex(conn, &raw mut (*conn).response, action);
            return llhttp_errno::HPE_OK.0 as ::core::ffi::c_int;
        }
        return llhttp_errno::HPE_PAUSED.0 as ::core::ffi::c_int;
    };
}
pub const LIBUV_RECV_BUFFER_INITIAL: size_t = 4096 as ::core::ffi::c_int as size_t;
pub const LIBUV_RECV_BUFFER_MAX: size_t =
    (1024 as ::core::ffi::c_int as size_t).wrapping_mul(1024 as size_t);
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
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
