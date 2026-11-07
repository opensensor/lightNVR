unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn uv_strerror(err: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn uv_buf_init(base: *mut ::core::ffi::c_char, len: ::core::ffi::c_uint) -> uv_buf_t;
    unsafe fn uv_is_closing(handle: *const uv_handle_t) -> ::core::ffi::c_int;
    unsafe fn safe_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn safe_free(ptr: *mut ::core::ffi::c_void);
    unsafe fn libuv_connection_close(conn: *mut libuv_connection_t);
    unsafe fn libuv_connection_write(
        conn: *mut libuv_connection,
        req: *mut uv_write_t,
        bufs: *const uv_buf_t,
        nbufs: ::core::ffi::c_uint,
        cb: uv_write_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_connection_reset(conn: *mut libuv_connection_t);
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
pub type ssize_t = isize;
pub type int32_t = i32;
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
pub type uint8_t = u8;
pub type uint16_t = u16;
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
    pub u: C2Rust_Unnamed_7,
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
    pub c2rust_unnamed: C2Rust_Unnamed_5,
    pub handle: *mut uv_stream_t,
    pub cb: uv_shutdown_cb,
}
pub type uv_shutdown_cb =
    Option<unsafe extern "C" fn(*mut uv_shutdown_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_6,
    pub cb: uv_connect_cb,
    pub handle: *mut uv_stream_t,
    pub queue: uv__queue,
}
pub type uv_connect_cb = Option<unsafe extern "C" fn(*mut uv_connect_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
}
pub type uv_read_cb =
    Option<unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> ()>;
pub type uv_alloc_cb = Option<unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_tcp_t = uv_tcp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_write_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_9,
    pub cb: uv_write_cb,
    pub send_handle: *mut uv_stream_t,
    pub handle: *mut uv_stream_t,
    pub queue: uv__queue,
    pub write_index: ::core::ffi::c_uint,
    pub bufs: *mut uv_buf_t,
    pub nbufs: ::core::ffi::c_uint,
    pub error: ::core::ffi::c_int,
    pub bufsml: [uv_buf_t; 4],
}
pub type uv_write_cb = Option<unsafe extern "C" fn(*mut uv_write_t, ::core::ffi::c_int) -> ()>;
pub type uv_write_t = uv_write_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
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
pub type libuv_write_ctx_t = libuv_write_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libuv_write_ctx {
    pub req: uv_write_t,
    pub buf: uv_buf_t,
    pub conn: *mut libuv_connection_t,
    pub free_buffer: bool,
    pub action: write_complete_action_t,
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
pub const LOG_COMPONENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"HTTP\0") };
unsafe extern "C" fn get_status_phrase(
    mut status_code: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    match status_code {
        200 => return b"OK\0".as_ptr() as *const ::core::ffi::c_char,
        201 => return b"Created\0".as_ptr() as *const ::core::ffi::c_char,
        204 => return b"No Content\0".as_ptr() as *const ::core::ffi::c_char,
        206 => return b"Partial Content\0".as_ptr() as *const ::core::ffi::c_char,
        301 => return b"Moved Permanently\0".as_ptr() as *const ::core::ffi::c_char,
        302 => return b"Found\0".as_ptr() as *const ::core::ffi::c_char,
        304 => return b"Not Modified\0".as_ptr() as *const ::core::ffi::c_char,
        400 => return b"Bad Request\0".as_ptr() as *const ::core::ffi::c_char,
        401 => return b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        403 => return b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
        404 => return b"Not Found\0".as_ptr() as *const ::core::ffi::c_char,
        405 => return b"Method Not Allowed\0".as_ptr() as *const ::core::ffi::c_char,
        409 => return b"Conflict\0".as_ptr() as *const ::core::ffi::c_char,
        413 => return b"Payload Too Large\0".as_ptr() as *const ::core::ffi::c_char,
        415 => return b"Unsupported Media Type\0".as_ptr() as *const ::core::ffi::c_char,
        500 => return b"Internal Server Error\0".as_ptr() as *const ::core::ffi::c_char,
        501 => return b"Not Implemented\0".as_ptr() as *const ::core::ffi::c_char,
        503 => return b"Service Unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"Unknown\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_write_cb(mut req: *mut uv_write_t, mut status: ::core::ffi::c_int) {
    let mut ctx: *mut libuv_write_ctx_t = req as *mut libuv_write_ctx_t;
    let mut conn: *mut libuv_connection_t = (*ctx).conn;
    let mut action: write_complete_action_t = (*ctx).action;
    if status < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_write_cb: Write error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(status),
        );
        action = write_complete_action_t::WRITE_ACTION_CLOSE;
    }
    if (*ctx).free_buffer as ::core::ffi::c_int != 0 && !(*ctx).buf.base.is_null() {
        safe_free((*ctx).buf.base as *mut ::core::ffi::c_void);
    }
    safe_free(ctx as *mut ::core::ffi::c_void);
    if !conn.is_null() {
        match action {
            write_complete_action_t::WRITE_ACTION_KEEP_ALIVE => {
                libuv_connection_reset(conn);
            }
            write_complete_action_t::WRITE_ACTION_CLOSE => {
                libuv_connection_close(conn);
            }
            write_complete_action_t::WRITE_ACTION_NONE | _ => {}
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_connection_send(
    mut conn: *mut libuv_connection_t,
    mut data: *mut ::core::ffi::c_char,
    mut len: size_t,
    mut free_after_send: bool,
) -> ::core::ffi::c_int {
    if conn.is_null() || data.is_null() || len == 0 as size_t {
        if free_after_send as ::core::ffi::c_int != 0 && !data.is_null() {
            safe_free(data as *mut ::core::ffi::c_void);
        }
        return -1 as ::core::ffi::c_int;
    }
    if uv_is_closing(&raw mut (*conn).handle as *mut uv_handle_t) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_connection_send: Connection is closing, discarding data\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        if free_after_send {
            safe_free(data as *mut ::core::ffi::c_void);
        }
        return -1 as ::core::ffi::c_int;
    }
    let mut ctx: *mut libuv_write_ctx_t =
        safe_malloc(::core::mem::size_of::<libuv_write_ctx_t>()) as *mut libuv_write_ctx_t;
    if ctx.is_null() {
        if free_after_send {
            safe_free(data as *mut ::core::ffi::c_void);
        }
        return -1 as ::core::ffi::c_int;
    }
    (*ctx).conn = conn;
    (*ctx).buf = uv_buf_init(data, len as ::core::ffi::c_uint);
    (*ctx).free_buffer = free_after_send;
    (*ctx).action = write_complete_action_t::WRITE_ACTION_NONE;
    let mut r: ::core::ffi::c_int = libuv_connection_write(
        conn,
        &raw mut (*ctx).req,
        &raw mut (*ctx).buf,
        1 as ::core::ffi::c_uint,
        Some(libuv_write_cb as unsafe extern "C" fn(*mut uv_write_t, ::core::ffi::c_int) -> ()),
    );
    if r != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_connection_send: Write failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(r),
        );
        if free_after_send {
            safe_free(data as *mut ::core::ffi::c_void);
        }
        safe_free(ctx as *mut ::core::ffi::c_void);
        libuv_connection_close(conn);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_connection_send_ex(
    mut conn: *mut libuv_connection_t,
    mut data: *mut ::core::ffi::c_char,
    mut len: size_t,
    mut free_after_send: bool,
    mut action: write_complete_action_t,
) -> ::core::ffi::c_int {
    if conn.is_null() || data.is_null() || len == 0 as size_t {
        if free_after_send as ::core::ffi::c_int != 0 && !data.is_null() {
            safe_free(data as *mut ::core::ffi::c_void);
        }
        return -1 as ::core::ffi::c_int;
    }
    if uv_is_closing(&raw mut (*conn).handle as *mut uv_handle_t) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_connection_send_ex: Connection is closing, discarding data\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        if free_after_send {
            safe_free(data as *mut ::core::ffi::c_void);
        }
        return -1 as ::core::ffi::c_int;
    }
    let mut ctx: *mut libuv_write_ctx_t =
        safe_malloc(::core::mem::size_of::<libuv_write_ctx_t>()) as *mut libuv_write_ctx_t;
    if ctx.is_null() {
        if free_after_send {
            safe_free(data as *mut ::core::ffi::c_void);
        }
        return -1 as ::core::ffi::c_int;
    }
    (*ctx).conn = conn;
    (*ctx).buf = uv_buf_init(data, len as ::core::ffi::c_uint);
    (*ctx).free_buffer = free_after_send;
    (*ctx).action = action;
    let mut r: ::core::ffi::c_int = libuv_connection_write(
        conn,
        &raw mut (*ctx).req,
        &raw mut (*ctx).buf,
        1 as ::core::ffi::c_uint,
        Some(libuv_write_cb as unsafe extern "C" fn(*mut uv_write_t, ::core::ffi::c_int) -> ()),
    );
    if r != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_connection_send_ex: Write failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(r),
        );
        if free_after_send {
            safe_free(data as *mut ::core::ffi::c_void);
        }
        safe_free(ctx as *mut ::core::ffi::c_void);
        libuv_connection_close(conn);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_send_response(
    mut conn: *mut libuv_connection_t,
    mut response: *const http_response_t,
) -> ::core::ffi::c_int {
    if conn.is_null() || response.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut headers_size: size_t = 256 as size_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*response).num_headers {
        headers_size = headers_size.wrapping_add(
            strlen(
                &raw const (*(&raw const (*response).headers as *const http_header_t)
                    .offset(i as isize))
                .name as *const ::core::ffi::c_char,
            )
            .wrapping_add(strlen(
                &raw const (*(&raw const (*response).headers as *const http_header_t)
                    .offset(i as isize))
                .value as *const ::core::ffi::c_char,
            ))
            .wrapping_add(4 as size_t),
        );
        i += 1;
    }
    if (*response).content_type[0usize] != 0 {
        headers_size = headers_size.wrapping_add(64 as size_t);
    }
    headers_size = headers_size.wrapping_add(64 as size_t);
    headers_size = headers_size.wrapping_add(4 as size_t);
    let mut total_size: size_t = headers_size.wrapping_add((*response).body_length);
    let mut buffer: *mut ::core::ffi::c_char = safe_malloc(total_size) as *mut ::core::ffi::c_char;
    if buffer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_send_response: Failed to allocate response buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut offset: size_t = 0 as size_t;
    let mut remaining: size_t = total_size.wrapping_sub(offset);
    let mut written: ::core::ffi::c_int = snprintf(
        buffer.offset(offset as isize),
        remaining,
        b"HTTP/1.1 %d %s\r\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*response).status_code,
        get_status_phrase((*response).status_code),
    );
    if written < 0 as ::core::ffi::c_int || written as size_t >= remaining {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_send_response: Failed to write status line\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        safe_free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    offset = offset.wrapping_add(written as size_t);
    if (*response).content_type[0usize] != 0 {
        let mut remaining_0: size_t = total_size.wrapping_sub(offset);
        let mut written_0: ::core::ffi::c_int = snprintf(
            buffer.offset(offset as isize),
            remaining_0,
            b"Content-Type: %s\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*response).content_type as *const ::core::ffi::c_char,
        );
        if written_0 < 0 as ::core::ffi::c_int || written_0 as size_t >= remaining_0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_send_response: Failed to write Content-Type header\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            safe_free(buffer as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
        offset = offset.wrapping_add(written_0 as size_t);
    }
    let mut remaining_1: size_t = total_size.wrapping_sub(offset);
    let mut written_1: ::core::ffi::c_int = snprintf(
        buffer.offset(offset as isize),
        remaining_1,
        b"Content-Length: %zu\r\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*response).body_length,
    );
    if written_1 < 0 as ::core::ffi::c_int || written_1 as size_t >= remaining_1 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_send_response: Failed to write Content-Length header\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        safe_free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    offset = offset.wrapping_add(written_1 as size_t);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*response).num_headers {
        let mut remaining_2: size_t = total_size.wrapping_sub(offset);
        let mut written_2: ::core::ffi::c_int = snprintf(
            buffer.offset(offset as isize),
            remaining_2,
            b"%s: %s\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*(&raw const (*response).headers as *const http_header_t)
                .offset(i_0 as isize))
            .name as *const ::core::ffi::c_char,
            &raw const (*(&raw const (*response).headers as *const http_header_t)
                .offset(i_0 as isize))
            .value as *const ::core::ffi::c_char,
        );
        if written_2 < 0 as ::core::ffi::c_int || written_2 as size_t >= remaining_2 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_send_response: Failed to write custom header\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            safe_free(buffer as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
        offset = offset.wrapping_add(written_2 as size_t);
        i_0 += 1;
    }
    let mut remaining_3: size_t = total_size.wrapping_sub(offset);
    let mut written_3: ::core::ffi::c_int = snprintf(
        buffer.offset(offset as isize),
        remaining_3,
        b"\r\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if written_3 < 0 as ::core::ffi::c_int || written_3 as size_t >= remaining_3 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_send_response: Failed to write end-of-headers\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        safe_free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    offset = offset.wrapping_add(written_3 as size_t);
    if !(*response).body.is_null() && (*response).body_length > 0 as size_t {
        if (*response).body_length > total_size.wrapping_sub(offset) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_send_response: Body does not fit into allocated buffer\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            safe_free(buffer as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
        memcpy(
            buffer.offset(offset as isize) as *mut ::core::ffi::c_void,
            (*response).body,
            (*response).body_length,
        );
        offset = offset.wrapping_add((*response).body_length);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_send_response: Sending %d %s (%zu bytes)\0".as_ptr() as *const ::core::ffi::c_char,
        (*response).status_code,
        get_status_phrase((*response).status_code),
        (*response).body_length,
    );
    return libuv_connection_send(conn, buffer, offset, r#true != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_send_response_ex(
    mut conn: *mut libuv_connection_t,
    mut response: *const http_response_t,
    mut action: write_complete_action_t,
) -> ::core::ffi::c_int {
    if conn.is_null() || response.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut headers_size: size_t = 256 as size_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*response).num_headers {
        headers_size = headers_size.wrapping_add(
            strlen(
                &raw const (*(&raw const (*response).headers as *const http_header_t)
                    .offset(i as isize))
                .name as *const ::core::ffi::c_char,
            )
            .wrapping_add(strlen(
                &raw const (*(&raw const (*response).headers as *const http_header_t)
                    .offset(i as isize))
                .value as *const ::core::ffi::c_char,
            ))
            .wrapping_add(4 as size_t),
        );
        i += 1;
    }
    if (*response).content_type[0usize] != 0 {
        headers_size = headers_size.wrapping_add(64 as size_t);
    }
    headers_size = headers_size.wrapping_add(64 as size_t);
    headers_size = headers_size.wrapping_add(4 as size_t);
    let mut total_size: size_t = headers_size.wrapping_add((*response).body_length);
    let mut buffer: *mut ::core::ffi::c_char = safe_malloc(total_size) as *mut ::core::ffi::c_char;
    if buffer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_send_response_ex: Failed to allocate response buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut offset: size_t = 0 as size_t;
    let mut remaining: size_t = total_size.wrapping_sub(offset);
    let mut written: ::core::ffi::c_int = snprintf(
        buffer.offset(offset as isize),
        remaining,
        b"HTTP/1.1 %d %s\r\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*response).status_code,
        get_status_phrase((*response).status_code),
    );
    if written < 0 as ::core::ffi::c_int || written as size_t >= remaining {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_send_response_ex: Failed to write status line\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        safe_free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    offset = offset.wrapping_add(written as size_t);
    if (*response).content_type[0usize] != 0 {
        let mut remaining_0: size_t = total_size.wrapping_sub(offset);
        let mut written_0: ::core::ffi::c_int = snprintf(
            buffer.offset(offset as isize),
            remaining_0,
            b"Content-Type: %s\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*response).content_type as *const ::core::ffi::c_char,
        );
        if written_0 < 0 as ::core::ffi::c_int || written_0 as size_t >= remaining_0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_send_response_ex: Failed to write Content-Type header\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            safe_free(buffer as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
        offset = offset.wrapping_add(written_0 as size_t);
    }
    let mut remaining_1: size_t = total_size.wrapping_sub(offset);
    let mut written_1: ::core::ffi::c_int = snprintf(
        buffer.offset(offset as isize),
        remaining_1,
        b"Content-Length: %zu\r\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*response).body_length,
    );
    if written_1 < 0 as ::core::ffi::c_int || written_1 as size_t >= remaining_1 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_send_response_ex: Failed to write Content-Length header\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        safe_free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    offset = offset.wrapping_add(written_1 as size_t);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*response).num_headers {
        let mut remaining_2: size_t = total_size.wrapping_sub(offset);
        let mut written_2: ::core::ffi::c_int = snprintf(
            buffer.offset(offset as isize),
            remaining_2,
            b"%s: %s\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*(&raw const (*response).headers as *const http_header_t)
                .offset(i_0 as isize))
            .name as *const ::core::ffi::c_char,
            &raw const (*(&raw const (*response).headers as *const http_header_t)
                .offset(i_0 as isize))
            .value as *const ::core::ffi::c_char,
        );
        if written_2 < 0 as ::core::ffi::c_int || written_2 as size_t >= remaining_2 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_send_response_ex: Failed to write custom header\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            safe_free(buffer as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
        offset = offset.wrapping_add(written_2 as size_t);
        i_0 += 1;
    }
    let mut remaining_3: size_t = total_size.wrapping_sub(offset);
    let mut written_3: ::core::ffi::c_int = snprintf(
        buffer.offset(offset as isize),
        remaining_3,
        b"\r\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if written_3 < 0 as ::core::ffi::c_int || written_3 as size_t >= remaining_3 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_send_response_ex: Failed to write end-of-headers\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        safe_free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    offset = offset.wrapping_add(written_3 as size_t);
    if !(*response).body.is_null() && (*response).body_length > 0 as size_t {
        if (*response).body_length > total_size.wrapping_sub(offset) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_send_response_ex: Body does not fit into allocated buffer\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            safe_free(buffer as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
        memcpy(
            buffer.offset(offset as isize) as *mut ::core::ffi::c_void,
            (*response).body,
            (*response).body_length,
        );
        offset = offset.wrapping_add((*response).body_length);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_send_response_ex: Sending %d %s (%zu bytes, action=%d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*response).status_code,
        get_status_phrase((*response).status_code),
        (*response).body_length,
        action.0,
    );
    return libuv_connection_send_ex(conn, buffer, offset, r#true != 0, action);
}
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
