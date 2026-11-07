unsafe extern "C" {
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_free(object: *mut ::core::ffi::c_void);
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
    unsafe fn getrandom(
        __buffer: *mut ::core::ffi::c_void,
        __length: size_t,
        __flags: ::core::ffi::c_uint,
    ) -> ssize_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn getpid() -> __pid_t;
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
    unsafe fn libuv_serve_file(
        conn: *mut libuv_connection_t,
        path: *const ::core::ffi::c_char,
        content_type: *const ::core::ffi::c_char,
        extra_headers: *const ::core::ffi::c_char,
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type clockid_t = __clockid_t;
pub type int32_t = i32;
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint64_t = u64;
pub type uint_fast64_t = ::core::ffi::c_ulong;
pub type uintptr_t = usize;
pub type atomic_uint_fast64_t = uint_fast64_t;
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
pub struct uv_loop_s {
    pub data: *mut ::core::ffi::c_void,
    pub active_handles: ::core::ffi::c_uint,
    pub handle_queue: uv__queue,
    pub active_reqs: C2Rust_Unnamed_5,
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
    pub timer_heap: C2Rust_Unnamed_3,
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
    pub u: C2Rust_Unnamed_2,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: ::core::ffi::c_int,
    pub tree_entry: C2Rust_Unnamed_0,
    pub caught_signals: ::core::ffi::c_uint,
    pub dispatched_signals: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
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
    pub u: C2Rust_Unnamed_1,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
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
pub union C2Rust_Unnamed_2 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
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
    pub u: C2Rust_Unnamed_4,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub async_cb: uv_async_cb,
    pub queue: uv__queue,
    pub pending: ::core::ffi::c_int,
}
pub type uv_async_cb = Option<unsafe extern "C" fn(*mut uv_async_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_mutex_t = pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
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
    pub handlers: *mut C2Rust_Unnamed_10,
    pub handler_count: ::core::ffi::c_int,
    pub handler_capacity: ::core::ffi::c_int,
    pub tls_ctx: *mut ::core::ffi::c_void,
    pub thread: uv_thread_t,
    pub thread_running: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
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
pub const GRND_NONBLOCK: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const LOG_COMPONENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"HTTP\0") };
static mut request_id_counter: atomic_uint_fast64_t = 1 as ::core::ffi::c_ulong;
unsafe extern "C" fn generate_request_id(mut output: *mut ::core::ffi::c_char) {
    let mut random_bytes: [::core::ffi::c_uchar; 16] = [0; 16];
    let mut random_count: ssize_t = getrandom(
        &raw mut random_bytes as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 16]>(),
        GRND_NONBLOCK as ::core::ffi::c_uint,
    );
    if random_count == ::core::mem::size_of::<[::core::ffi::c_uchar; 16]>() as ssize_t {
        random_bytes[6usize] = (random_bytes[6usize] as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int
            | 0x40 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        random_bytes[8usize] = (random_bytes[8usize] as ::core::ffi::c_int
            & 0x3f as ::core::ffi::c_int
            | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        snprintf(
            output,
            REQUEST_ID_MAX as size_t,
            b"%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x\0".as_ptr()
                as *const ::core::ffi::c_char,
            random_bytes[0usize] as ::core::ffi::c_int,
            random_bytes[1usize] as ::core::ffi::c_int,
            random_bytes[2usize] as ::core::ffi::c_int,
            random_bytes[3usize] as ::core::ffi::c_int,
            random_bytes[4usize] as ::core::ffi::c_int,
            random_bytes[5usize] as ::core::ffi::c_int,
            random_bytes[6usize] as ::core::ffi::c_int,
            random_bytes[7usize] as ::core::ffi::c_int,
            random_bytes[8usize] as ::core::ffi::c_int,
            random_bytes[9usize] as ::core::ffi::c_int,
            random_bytes[10usize] as ::core::ffi::c_int,
            random_bytes[11usize] as ::core::ffi::c_int,
            random_bytes[12usize] as ::core::ffi::c_int,
            random_bytes[13usize] as ::core::ffi::c_int,
            random_bytes[14usize] as ::core::ffi::c_int,
            random_bytes[15usize] as ::core::ffi::c_int,
        );
        return;
    }
    let mut timestamp: timespec = timespec {
        tv_sec: 0 as __time_t,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut timestamp);
    let mut counter: uint64_t = ::core::intrinsics::atomic_xadd::<
        _,
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut request_id_counter, 1 as uint_fast64_t);
    snprintf(
        output,
        REQUEST_ID_MAX as size_t,
        b"req-%llx-%lx-%llx\0".as_ptr() as *const ::core::ffi::c_char,
        timestamp.tv_sec as ::core::ffi::c_ulonglong,
        timestamp.tv_nsec as ::core::ffi::c_ulong,
        (counter ^ getpid() as uint64_t) as ::core::ffi::c_ulonglong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn url_decode(
    mut src: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_char,
    mut dst_size: size_t,
) -> ::core::ffi::c_int {
    if src.is_null() || dst.is_null() || dst_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut src_len: size_t = strlen(src);
    let mut i: size_t = 0;
    let mut j: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < src_len && j < dst_size.wrapping_sub(1 as size_t) {
        if *src.offset(i as isize) as ::core::ffi::c_int == '%' as ::core::ffi::c_int
            && i.wrapping_add(2 as size_t) < src_len
        {
            let mut value: ::core::ffi::c_uint = 0;
            if sscanf(
                src.offset(i as isize)
                    .offset(1 as ::core::ffi::c_int as isize),
                b"%2x\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut value,
            ) == 1 as ::core::ffi::c_int
            {
                let c2rust_fresh0 = j;
                j = j.wrapping_add(1);
                *dst.offset(c2rust_fresh0 as isize) = value as ::core::ffi::c_char;
                i = i.wrapping_add(2 as size_t);
            } else {
                let c2rust_fresh1 = j;
                j = j.wrapping_add(1);
                *dst.offset(c2rust_fresh1 as isize) = *src.offset(i as isize);
            }
        } else if *src.offset(i as isize) as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
            let c2rust_fresh2 = j;
            j = j.wrapping_add(1);
            *dst.offset(c2rust_fresh2 as isize) = ' ' as ::core::ffi::c_char;
        } else {
            let c2rust_fresh3 = j;
            j = j.wrapping_add(1);
            *dst.offset(c2rust_fresh3 as isize) = *src.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    *dst.offset(j as isize) = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_request_init(mut req: *mut http_request_t) {
    if req.is_null() {
        return;
    }
    memset(
        req as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<http_request_t>(),
    );
    generate_request_id(&raw mut (*req).request_id as *mut ::core::ffi::c_char);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_request_set_request_id(
    mut req: *mut http_request_t,
    mut request_id: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if req.is_null() || request_id.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut length: size_t = strlen(request_id);
    if length == 0 as size_t || length >= REQUEST_ID_MAX as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: size_t = 0 as size_t;
    while i < length {
        let mut value: ::core::ffi::c_uchar =
            *request_id.offset(i as isize) as ::core::ffi::c_uchar;
        if *(*__ctype_b_loc()).offset(value as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
            && value as ::core::ffi::c_int != '.' as ::core::ffi::c_int
            && value as ::core::ffi::c_int != '_' as ::core::ffi::c_int
            && value as ::core::ffi::c_int != ':' as ::core::ffi::c_int
            && value as ::core::ffi::c_int != '-' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    safe_strcpy(
        &raw mut (*req).request_id as *mut ::core::ffi::c_char,
        request_id,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
        0 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_response_init(mut res: *mut http_response_t) {
    if res.is_null() {
        return;
    }
    memset(
        res as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<http_response_t>(),
    );
    (*res).status_code = 200 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_response_free(mut res: *mut http_response_t) {
    if res.is_null() {
        return;
    }
    if (*res).body_allocated as ::core::ffi::c_int != 0 && !(*res).body.is_null() {
        free((*res).body);
        (*res).body = NULL;
        (*res).body_length = 0 as size_t;
        (*res).body_allocated = r#false != 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_request_get_header(
    mut req: *const http_request_t,
    mut name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if req.is_null() || name.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*req).num_headers {
        if strcasecmp(
            &raw const (*(&raw const (*req).headers as *const http_header_t).offset(i as isize))
                .name as *const ::core::ffi::c_char,
            name,
        ) == 0 as ::core::ffi::c_int
        {
            return &raw const (*(&raw const (*req).headers as *const http_header_t)
                .offset(i as isize))
            .value as *const ::core::ffi::c_char;
        }
        i += 1;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_request_get_query_param(
    mut req: *const http_request_t,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_char,
    mut value_len: size_t,
) -> ::core::ffi::c_int {
    if req.is_null() || name.is_null() || value.is_null() || value_len == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut qs: *const ::core::ffi::c_char =
        &raw const (*req).query_string as *const ::core::ffi::c_char;
    if qs.is_null() || *qs.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut name_len: size_t = strlen(name);
    let mut p: *const ::core::ffi::c_char = qs;
    while !p.is_null() && *p as ::core::ffi::c_int != 0 {
        if strncmp(p, name, name_len) == 0 as ::core::ffi::c_int
            && *p.offset(name_len as isize) as ::core::ffi::c_int == '=' as ::core::ffi::c_int
        {
            let mut val_start: *const ::core::ffi::c_char = p
                .offset(name_len as isize)
                .offset(1 as ::core::ffi::c_int as isize);
            let mut val_end: *const ::core::ffi::c_char =
                strchr(val_start, '&' as ::core::ffi::c_int);
            let mut val_len: size_t = if !val_end.is_null() {
                val_end.offset_from(val_start) as size_t
            } else {
                strlen(val_start)
            };
            if val_len >= value_len {
                val_len = value_len.wrapping_sub(1 as size_t);
            }
            let mut encoded: [::core::ffi::c_char; 1024] = [
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
            if val_len < ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() {
                memcpy(
                    &raw mut encoded as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    val_start as *const ::core::ffi::c_void,
                    val_len,
                );
                encoded[val_len] = '\0' as ::core::ffi::c_char;
                url_decode(
                    &raw mut encoded as *mut ::core::ffi::c_char,
                    value,
                    value_len,
                );
            } else {
                memcpy(
                    value as *mut ::core::ffi::c_void,
                    val_start as *const ::core::ffi::c_void,
                    val_len,
                );
                *value.offset(val_len as isize) = '\0' as ::core::ffi::c_char;
            }
            return strlen(value) as ::core::ffi::c_int;
        }
        p = strchr(p, '&' as ::core::ffi::c_int);
        if !p.is_null() {
            p = p.offset(1);
        }
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_request_get_body_str(
    mut req: *const http_request_t,
    mut buf: *mut ::core::ffi::c_char,
    mut buf_len: size_t,
) -> ::core::ffi::c_int {
    if req.is_null() || buf.is_null() || buf_len == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    if (*req).body.is_null() || (*req).body_len == 0 as size_t {
        *buf.offset(0isize) = '\0' as ::core::ffi::c_char;
        return 0 as ::core::ffi::c_int;
    }
    let mut copy_len: size_t = if (*req).body_len < buf_len.wrapping_sub(1 as size_t) {
        (*req).body_len
    } else {
        buf_len.wrapping_sub(1 as size_t)
    };
    memcpy(buf as *mut ::core::ffi::c_void, (*req).body, copy_len);
    *buf.offset(copy_len as isize) = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_request_extract_path_param(
    mut req: *const http_request_t,
    mut prefix: *const ::core::ffi::c_char,
    mut param_buf: *mut ::core::ffi::c_char,
    mut buf_size: size_t,
) -> ::core::ffi::c_int {
    if req.is_null() || prefix.is_null() || param_buf.is_null() || buf_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut path: *const ::core::ffi::c_char = &raw const (*req).path as *const ::core::ffi::c_char;
    let mut prefix_len: size_t = strlen(prefix);
    if strncmp(path, prefix, prefix_len) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut param: *const ::core::ffi::c_char = path.offset(prefix_len as isize);
    while *param as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        param = param.offset(1);
    }
    let mut end: *const ::core::ffi::c_char = strchr(param, '?' as ::core::ffi::c_int);
    let mut param_len: size_t = if !end.is_null() {
        end.offset_from(param) as size_t
    } else {
        strlen(param)
    };
    while param_len > 0 as size_t
        && *param.offset(param_len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
    {
        param_len = param_len.wrapping_sub(1);
    }
    if param_len == 0 as size_t || param_len >= buf_size {
        return -1 as ::core::ffi::c_int;
    }
    url_decode(param, param_buf, param_len.wrapping_add(1 as size_t));
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_response_add_header(
    mut res: *mut http_response_t,
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if res.is_null() || name.is_null() || value.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if (*res).num_headers >= MAX_HEADERS {
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut (*(&raw mut (*res).headers as *mut http_header_t)
            .offset((*res).num_headers as isize))
        .name as *mut ::core::ffi::c_char,
        name,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*(&raw mut (*res).headers as *mut http_header_t)
            .offset((*res).num_headers as isize))
        .value as *mut ::core::ffi::c_char,
        value,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        0 as size_t,
    );
    (*res).num_headers += 1;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_response_set_body(
    mut res: *mut http_response_t,
    mut body: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if res.is_null() || body.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if (*res).body_allocated as ::core::ffi::c_int != 0 && !(*res).body.is_null() {
        free((*res).body);
    }
    let mut len: size_t = strlen(body);
    (*res).body = malloc(len.wrapping_add(1 as size_t));
    if (*res).body.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memcpy(
        (*res).body,
        body as *const ::core::ffi::c_void,
        len.wrapping_add(1 as size_t),
    );
    (*res).body_length = len;
    (*res).body_allocated = r#true != 0;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_response_add_cors_headers(mut res: *mut http_response_t) {
    if res.is_null() {
        return;
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
        b"Content-Type, Authorization, X-Requested-With, X-Request-ID\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    http_response_add_header(
        res,
        b"Access-Control-Expose-Headers\0".as_ptr() as *const ::core::ffi::c_char,
        b"X-Request-ID, X-Total-Count, Content-Disposition\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    http_response_add_header(
        res,
        b"Access-Control-Allow-Credentials\0".as_ptr() as *const ::core::ffi::c_char,
        b"true\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_add_header(
        res,
        b"Access-Control-Max-Age\0".as_ptr() as *const ::core::ffi::c_char,
        b"86400\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_response_set_json(
    mut res: *mut http_response_t,
    mut status_code: ::core::ffi::c_int,
    mut json_str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if res.is_null() || json_str.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    (*res).status_code = status_code;
    safe_strcpy(
        &raw mut (*res).content_type as *mut ::core::ffi::c_char,
        b"application/json\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    http_response_add_cors_headers(res);
    http_response_add_header(
        res,
        b"Cache-Control\0".as_ptr() as *const ::core::ffi::c_char,
        b"no-cache, no-store, must-revalidate\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_add_header(
        res,
        b"Pragma\0".as_ptr() as *const ::core::ffi::c_char,
        b"no-cache\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_add_header(
        res,
        b"Expires\0".as_ptr() as *const ::core::ffi::c_char,
        b"0\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return http_response_set_body(res, json_str);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_response_set_json_error(
    mut res: *mut http_response_t,
    mut status_code: ::core::ffi::c_int,
    mut error_message: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if res.is_null() || error_message.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if cJSON_AddStringToObject(
        root,
        b"error\0".as_ptr() as *const ::core::ffi::c_char,
        error_message,
    )
    .is_null()
    {
        cJSON_Delete(root);
        return -1 as ::core::ffi::c_int;
    }
    let mut json_buf: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if json_buf.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut ret: ::core::ffi::c_int = http_response_set_json(res, status_code, json_buf);
    cJSON_free(json_buf as *mut ::core::ffi::c_void);
    return ret;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_serve_file(
    mut req: *const http_request_t,
    mut res: *const http_response_t,
    mut file_path: *const ::core::ffi::c_char,
    mut content_type: *const ::core::ffi::c_char,
    mut extra_headers: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if req.is_null() || res.is_null() || file_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"http_serve_file: Invalid parameters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut conn: *mut libuv_connection_t = (*req).user_data as *mut libuv_connection_t;
    if conn.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"http_serve_file: No connection in request user_data\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*conn).handler_on_worker {
        (*conn).deferred_file_serve = r#true != 0;
        safe_strcpy(
            &raw mut (*conn).deferred_file_path as *mut ::core::ffi::c_char,
            file_path,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        if !content_type.is_null() {
            safe_strcpy(
                &raw mut (*conn).deferred_content_type as *mut ::core::ffi::c_char,
                content_type,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                0 as size_t,
            );
        } else {
            (*conn).deferred_content_type[0usize] = '\0' as ::core::ffi::c_char;
        }
        if !extra_headers.is_null() {
            safe_strcpy(
                &raw mut (*conn).deferred_extra_headers as *mut ::core::ffi::c_char,
                extra_headers,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else {
            (*conn).deferred_extra_headers[0usize] = '\0' as ::core::ffi::c_char;
        }
        return 0 as ::core::ffi::c_int;
    }
    return libuv_serve_file(conn, file_path, content_type, extra_headers);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAX_HEADERS: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const REQUEST_ID_MAX: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
