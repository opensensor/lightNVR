unsafe extern "C" {
    pub type libuv_tls_context;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn uv_loop_init(r#loop: *mut uv_loop_t) -> ::core::ffi::c_int;
    unsafe fn uv_loop_close(r#loop: *mut uv_loop_t) -> ::core::ffi::c_int;
    unsafe fn uv_loop_alive(r#loop: *const uv_loop_t) -> ::core::ffi::c_int;
    unsafe fn uv_run(_: *mut uv_loop_t, mode: uv_run_mode) -> ::core::ffi::c_int;
    unsafe fn uv_stop(_: *mut uv_loop_t);
    unsafe fn uv_strerror(err: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn uv_walk(r#loop: *mut uv_loop_t, walk_cb: uv_walk_cb, arg: *mut ::core::ffi::c_void);
    unsafe fn uv_close(handle: *mut uv_handle_t, close_cb: uv_close_cb);
    unsafe fn uv_listen(
        stream: *mut uv_stream_t,
        backlog: ::core::ffi::c_int,
        cb: uv_connection_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_accept(server: *mut uv_stream_t, client: *mut uv_stream_t) -> ::core::ffi::c_int;
    unsafe fn uv_is_closing(handle: *const uv_handle_t) -> ::core::ffi::c_int;
    unsafe fn uv_tcp_init(_: *mut uv_loop_t, handle: *mut uv_tcp_t) -> ::core::ffi::c_int;
    unsafe fn uv_tcp_bind(
        handle: *mut uv_tcp_t,
        addr: *const sockaddr,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_tcp_getpeername(
        handle: *const uv_tcp_t,
        name: *mut sockaddr,
        namelen: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_async_init(
        _: *mut uv_loop_t,
        r#async: *mut uv_async_t,
        async_cb: uv_async_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_async_send(r#async: *mut uv_async_t) -> ::core::ffi::c_int;
    unsafe fn uv_ip4_addr(
        ip: *const ::core::ffi::c_char,
        port: ::core::ffi::c_int,
        addr: *mut sockaddr_in,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_ip4_name(
        src: *const sockaddr_in,
        dst: *mut ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_ip6_name(
        src: *const sockaddr_in6,
        dst: *mut ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_thread_create(
        tid: *mut uv_thread_t,
        entry: uv_thread_cb,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_thread_join(tid: *mut uv_thread_t) -> ::core::ffi::c_int;
    unsafe fn pthread_self() -> pthread_t;
    unsafe fn safe_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn safe_realloc(ptr: *mut ::core::ffi::c_void, size: size_t)
    -> *mut ::core::ffi::c_void;
    unsafe fn safe_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn safe_free(ptr: *mut ::core::ffi::c_void);
    static mut g_config: config_t;
    unsafe fn libuv_connection_close(conn: *mut libuv_connection_t);
    unsafe fn libuv_tls_compiled_in() -> bool;
    unsafe fn libuv_tls_context_create(
        cert_path: *const ::core::ffi::c_char,
        key_path: *const ::core::ffi::c_char,
    ) -> *mut libuv_tls_context_t;
    unsafe fn libuv_tls_context_free(ctx: *mut libuv_tls_context_t);
    unsafe fn libuv_tls_session_start(
        conn: *mut libuv_connection,
        ctx: *mut libuv_tls_context_t,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_connection_read_start(
        conn: *mut libuv_connection,
        alloc_cb: uv_alloc_cb,
        read_cb: uv_read_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_connection_create(server: *mut libuv_server_t) -> *mut libuv_connection_t;
    unsafe fn libuv_alloc_cb(handle: *mut uv_handle_t, suggested_size: size_t, buf: *mut uv_buf_t);
    unsafe fn libuv_read_cb(stream: *mut uv_stream_t, nread: ssize_t, buf: *const uv_buf_t);
    unsafe fn libuv_close_cb(handle: *mut uv_handle_t);
    unsafe fn httpd_set_secure_cookies(secure: bool);
    unsafe fn recording_archive_close_timer(handle: *mut uv_handle_t) -> bool;
    unsafe fn thumbnail_thread_init(r#loop: *mut uv_loop_t) -> ::core::ffi::c_int;
    unsafe fn thumbnail_thread_shutdown();
    unsafe fn go2rtc_proxy_thread_init(r#loop: *mut uv_loop_t) -> ::core::ffi::c_int;
    unsafe fn go2rtc_proxy_thread_shutdown();
    unsafe fn go2rtc_websocket_shutdown(r#loop: *mut uv_loop_t);
    unsafe fn set_web_server_thread_id(thread_id: pthread_t);
    unsafe fn mark_web_server_thread_stopped();
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
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __useconds_t = ::core::ffi::c_uint;
pub type ssize_t = isize;
pub type int32_t = i32;
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
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
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
pub struct uv_run_mode(pub ::core::ffi::c_uint);
impl uv_run_mode {
    pub const UV_RUN_DEFAULT: Self = Self(0);
    pub const UV_RUN_ONCE: Self = Self(1);
    pub const UV_RUN_NOWAIT: Self = Self(2);
}
pub type uv_walk_cb =
    Option<unsafe extern "C" fn(*mut uv_handle_t, *mut ::core::ffi::c_void) -> ()>;
pub type uv_thread_cb = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
pub struct http_server {
    pub config: http_server_config_t,
    pub running: bool,
    pub handlers: *mut C2Rust_Unnamed_10,
    pub handler_count: ::core::ffi::c_int,
    pub handler_capacity: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub path: [::core::ffi::c_char; 256],
    pub method: [::core::ffi::c_char; 16],
    pub handler: request_handler_t,
}
pub type http_server_t = http_server;
pub type http_server_handle_t = *mut http_server_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
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
pub type libuv_tls_context_t = libuv_tls_context;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const LOG_COMPONENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"HTTP\0") };
pub const INITIAL_HANDLER_CAPACITY: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
unsafe extern "C" fn stop_async_cb(mut handle: *mut uv_async_t) {
    let mut server: *mut libuv_server_t = (*handle).data as *mut libuv_server_t;
    if !server.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"stop_async_cb: Received stop signal, closing listener and stopping event loop\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        if uv_is_closing(&raw mut (*server).listener as *mut uv_handle_t) == 0 {
            uv_close(&raw mut (*server).listener as *mut uv_handle_t, None);
        }
        uv_stop((*server).r#loop);
    }
}
unsafe extern "C" fn libuv_server_init_internal(
    mut config: *const http_server_config_t,
    mut existing_loop: *mut uv_loop_t,
) -> http_server_handle_t {
    if config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_init: NULL config\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<http_server_t>();
    }
    if getenv(b"UV_THREADPOOL_SIZE\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        let mut pool_size_str: [::core::ffi::c_char; 16] = [0; 16];
        let mut pool_size: ::core::ffi::c_int = g_config.web_thread_pool_size;
        if pool_size < 2 as ::core::ffi::c_int {
            pool_size = 2 as ::core::ffi::c_int;
        }
        if pool_size > 128 as ::core::ffi::c_int {
            pool_size = 128 as ::core::ffi::c_int;
        }
        snprintf(
            &raw mut pool_size_str as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            pool_size,
        );
        setenv(
            b"UV_THREADPOOL_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut pool_size_str as *mut ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_init: Set UV_THREADPOOL_SIZE=%d for handler offloading\0".as_ptr()
                as *const ::core::ffi::c_char,
            pool_size,
        );
    }
    let mut server: *mut libuv_server_t =
        safe_calloc(1 as size_t, ::core::mem::size_of::<libuv_server_t>()) as *mut libuv_server_t;
    if server.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_init: Failed to allocate server\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<http_server_t>();
    }
    memcpy(
        &raw mut (*server).config as *mut ::core::ffi::c_void,
        config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<http_server_config_t>(),
    );
    if (*config).ssl_enabled {
        if !libuv_tls_compiled_in() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_init: ssl_enabled is set but this build has no TLS support (rebuild with -DENABLE_SSL=ON); serving plain HTTP\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            (*server).tls_ctx = libuv_tls_context_create(
                &raw const (*config).cert_path as *const ::core::ffi::c_char,
                &raw const (*config).key_path as *const ::core::ffi::c_char,
            ) as *mut ::core::ffi::c_void;
            if (*server).tls_ctx.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"libuv_server_init: HTTPS is enabled but the certificate or key could not be loaded; refusing to start the web server\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                safe_free(server as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<http_server_t>();
            }
        }
    }
    if !existing_loop.is_null() {
        (*server).r#loop = existing_loop;
        (*server).owns_loop = r#false != 0;
    } else {
        (*server).r#loop = safe_malloc(::core::mem::size_of::<uv_loop_t>()) as *mut uv_loop_t;
        if (*server).r#loop.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_init: Failed to allocate event loop\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            safe_free(server as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<http_server_t>();
        }
        if uv_loop_init((*server).r#loop) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_init: Failed to initialize event loop\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            safe_free((*server).r#loop as *mut ::core::ffi::c_void);
            safe_free(server as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<http_server_t>();
        }
        (*server).owns_loop = r#true != 0;
    }
    if uv_tcp_init((*server).r#loop, &raw mut (*server).listener) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_init: Failed to initialize TCP handle\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        if (*server).owns_loop {
            uv_loop_close((*server).r#loop);
            safe_free((*server).r#loop as *mut ::core::ffi::c_void);
        }
        libuv_tls_context_free((*server).tls_ctx as *mut libuv_tls_context_t);
        safe_free(server as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<http_server_t>();
    }
    (*server).listener.data = server as *mut ::core::ffi::c_void;
    if uv_async_init(
        (*server).r#loop,
        &raw mut (*server).stop_async,
        Some(stop_async_cb as unsafe extern "C" fn(*mut uv_async_t) -> ()),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_init: Failed to initialize async handle\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        uv_close(&raw mut (*server).listener as *mut uv_handle_t, None);
        if (*server).owns_loop {
            uv_loop_close((*server).r#loop);
            safe_free((*server).r#loop as *mut ::core::ffi::c_void);
        }
        libuv_tls_context_free((*server).tls_ctx as *mut libuv_tls_context_t);
        safe_free(server as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<http_server_t>();
    }
    (*server).stop_async.data = server as *mut ::core::ffi::c_void;
    (*server).handlers = safe_calloc(
        INITIAL_HANDLER_CAPACITY as size_t,
        ::core::mem::size_of::<C2Rust_Unnamed_11>(),
    ) as *mut C2Rust_Unnamed_11;
    if (*server).handlers.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_init: Failed to allocate handler registry\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        uv_close(&raw mut (*server).listener as *mut uv_handle_t, None);
        if (*server).owns_loop {
            uv_loop_close((*server).r#loop);
            safe_free((*server).r#loop as *mut ::core::ffi::c_void);
        }
        libuv_tls_context_free((*server).tls_ctx as *mut libuv_tls_context_t);
        safe_free(server as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<http_server_t>();
    }
    (*server).handler_capacity = INITIAL_HANDLER_CAPACITY;
    (*server).handler_count = 0 as ::core::ffi::c_int;
    httpd_set_secure_cookies(!(*server).tls_ctx.is_null());
    if thumbnail_thread_init((*server).r#loop) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_init: Failed to initialize thumbnail thread subsystem\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if go2rtc_proxy_thread_init((*server).r#loop) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_init: Failed to initialize go2rtc proxy thread subsystem\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_server_init: Server initialized on %s:%d (%s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*config).bind_ip,
        (*config).port,
        if !(*server).tls_ctx.is_null() {
            b"HTTPS\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"HTTP\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return server as http_server_handle_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_server_init(
    mut config: *const http_server_config_t,
) -> http_server_handle_t {
    return libuv_server_init_internal(config, ::core::ptr::null_mut::<uv_loop_t>());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_server_init_with_loop(
    mut config: *const http_server_config_t,
    mut r#loop: *mut uv_loop_t,
) -> http_server_handle_t {
    return libuv_server_init_internal(config, r#loop);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_server_get_loop(mut handle: http_server_handle_t) -> *mut uv_loop_t {
    let mut server: *mut libuv_server_t = handle as *mut libuv_server_t;
    return if !server.is_null() {
        (*server).r#loop
    } else {
        ::core::ptr::null_mut::<uv_loop_t>()
    };
}
unsafe extern "C" fn libuv_server_reset_loop(
    mut server: *mut libuv_server_t,
) -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_server_reset_loop: Attempting to recycle the event loop\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut close_result: ::core::ffi::c_int = uv_loop_close((*server).r#loop);
    if close_result != 0 as ::core::ffi::c_int {
        let mut active_handles: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        uv_walk(
            (*server).r#loop,
            Some(
                count_handles_cb
                    as unsafe extern "C" fn(*mut uv_handle_t, *mut ::core::ffi::c_void) -> (),
            ),
            &raw mut active_handles as *mut ::core::ffi::c_void,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_reset_loop: Refusing to abandon a busy event loop (close returned %d (%s); %d active handles). In-flight thread-pool work still references it -- recycling now would crash the process when that work completes. Leaving the loop intact.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            close_result,
            uv_strerror(close_result),
            active_handles,
        );
        return -1 as ::core::ffi::c_int;
    }
    if uv_loop_init((*server).r#loop) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_reset_loop: Failed to re-initialize event loop\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_server_reset_loop: Event loop recycled successfully\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_server_start(
    mut handle: http_server_handle_t,
) -> ::core::ffi::c_int {
    let mut server: *mut libuv_server_t = handle as *mut libuv_server_t;
    if server.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    ::core::ptr::write_volatile(&raw mut (*server).shutting_down, r#false != 0);
    let mut need_reinit: bool = uv_is_closing(&raw mut (*server).listener as *mut uv_handle_t) != 0
        || uv_is_closing(&raw mut (*server).stop_async as *mut uv_handle_t) != 0;
    if need_reinit {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_start: Handles are closed, reinitializing\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut wait_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while (uv_is_closing(&raw mut (*server).listener as *mut uv_handle_t) != 0
            || uv_is_closing(&raw mut (*server).stop_async as *mut uv_handle_t) != 0)
            && wait_count < 100 as ::core::ffi::c_int
        {
            uv_run((*server).r#loop, uv_run_mode::UV_RUN_NOWAIT);
            usleep(10000 as __useconds_t);
            wait_count += 1;
        }
        if uv_is_closing(&raw mut (*server).listener as *mut uv_handle_t) != 0
            || uv_is_closing(&raw mut (*server).stop_async as *mut uv_handle_t) != 0
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_start: Handles still closing after timeout, creating fresh event loop\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            if (*server).owns_loop {
                if libuv_server_reset_loop(server) != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"libuv_server_start: Failed to create fresh event loop\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"libuv_server_start: Cannot reset shared event loop\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if uv_tcp_init((*server).r#loop, &raw mut (*server).listener) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_start: Failed to reinitialize TCP handle\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*server).listener.data = server as *mut ::core::ffi::c_void;
        if uv_async_init(
            (*server).r#loop,
            &raw mut (*server).stop_async,
            Some(stop_async_cb as unsafe extern "C" fn(*mut uv_async_t) -> ()),
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_start: Failed to reinitialize async handle\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*server).stop_async.data = server as *mut ::core::ffi::c_void;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_start: Handles reinitialized successfully\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut r: ::core::ffi::c_int = uv_ip4_addr(
        (*server).config.bind_ip,
        (*server).config.port,
        &raw mut addr,
    );
    if r != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_start: IPv4 addr/port failed for %s:%d: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*server).config.bind_ip,
            (*server).config.port,
            uv_strerror(r),
        );
        return -1 as ::core::ffi::c_int;
    }
    r = uv_tcp_bind(
        &raw mut (*server).listener,
        &raw mut addr as *const sockaddr,
        0 as ::core::ffi::c_uint,
    );
    if r != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_start: Bind failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(r),
        );
        return -1 as ::core::ffi::c_int;
    }
    r = uv_listen(
        &raw mut (*server).listener as *mut uv_stream_t,
        128 as ::core::ffi::c_int,
        Some(on_connection as unsafe extern "C" fn(*mut uv_stream_t, ::core::ffi::c_int) -> ()),
    );
    if r != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_start: Listen failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(r),
        );
        return -1 as ::core::ffi::c_int;
    }
    ::core::ptr::write_volatile(&raw mut (*server).running, r#true != 0);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_server_start: Listening on %s:%d\0".as_ptr() as *const ::core::ffi::c_char,
        (*server).config.bind_ip,
        (*server).config.port,
    );
    if (*server).owns_loop {
        r = uv_thread_create(
            &raw mut (*server).thread,
            Some(server_thread_func as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            server as *mut ::core::ffi::c_void,
        );
        if r != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_start: Failed to create server thread\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*server).thread_running = r#true != 0;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn server_thread_func(mut arg: *mut ::core::ffi::c_void) {
    let mut server: *mut libuv_server_t = arg as *mut libuv_server_t;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_server: Event loop thread started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    set_web_server_thread_id(pthread_self());
    let MAX_UNEXPECTED_RESTARTS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let RESTART_DELAY_SEC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    let mut unexpected_restarts: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while ::core::ptr::read_volatile::<bool>(&raw const (*server).running) {
        let mut run_result: ::core::ffi::c_int =
            uv_run((*server).r#loop, uv_run_mode::UV_RUN_DEFAULT);
        if !::core::ptr::read_volatile::<bool>(&raw const (*server).running) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server: Event loop thread exiting normally (shutdown requested)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            break;
        } else {
            unexpected_restarts += 1;
            let mut active_handles: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            uv_walk(
                (*server).r#loop,
                Some(
                    count_handles_cb
                        as unsafe extern "C" fn(*mut uv_handle_t, *mut ::core::ffi::c_void) -> (),
                ),
                &raw mut active_handles as *mut ::core::ffi::c_void,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server: Event loop exited UNEXPECTEDLY (running=true, uv_run returned %d, active_handles=%d, restart_attempt=%d/%d)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                run_result,
                active_handles,
                unexpected_restarts,
                MAX_UNEXPECTED_RESTARTS,
            );
            if unexpected_restarts > MAX_UNEXPECTED_RESTARTS {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"libuv_server: Too many unexpected restarts (%d), giving up \xE2\x80\x94 health check thread will handle recovery\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    unexpected_restarts,
                );
                break;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"libuv_server: Retrying uv_run() in %d seconds...\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    RESTART_DELAY_SEC,
                );
                sleep(RESTART_DELAY_SEC as ::core::ffi::c_uint);
            }
        }
    }
    mark_web_server_thread_stopped();
    (*server).thread_running = r#false != 0;
}
unsafe extern "C" fn close_walk_cb(
    mut handle: *mut uv_handle_t,
    mut arg: *mut ::core::ffi::c_void,
) {
    if uv_is_closing(handle) == 0 {
        if recording_archive_close_timer(handle) {
            return;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"close_walk_cb: Closing handle type %d\0".as_ptr() as *const ::core::ffi::c_char,
            (*handle).r#type.0,
        );
        if (*handle).r#type.0 == uv_handle_type::UV_TCP.0 && !(*handle).data.is_null() {
            uv_close(
                handle,
                Some(libuv_close_cb as unsafe extern "C" fn(*mut uv_handle_t) -> ()),
            );
        } else {
            uv_close(handle, None);
        }
    }
}
unsafe extern "C" fn count_handles_cb(
    mut handle: *mut uv_handle_t,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut count: *mut ::core::ffi::c_int = arg as *mut ::core::ffi::c_int;
    if uv_is_closing(handle) == 0 {
        *count += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_server_stop(mut handle: http_server_handle_t) {
    let mut server: *mut libuv_server_t = handle as *mut libuv_server_t;
    if server.is_null()
        || ::core::ptr::read_volatile::<bool>(&raw const (*server).shutting_down)
            as ::core::ffi::c_int
            != 0
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_server_stop: Stopping server\0".as_ptr() as *const ::core::ffi::c_char,
    );
    ::core::ptr::write_volatile(&raw mut (*server).running, r#false != 0);
    ::core::ptr::write_volatile(&raw mut (*server).shutting_down, r#true != 0);
    if (*server).thread_running {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_stop: Signaling event loop thread to stop\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        uv_async_send(&raw mut (*server).stop_async);
        uv_stop((*server).r#loop);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_stop: Waiting for server thread to exit\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let max_wait_ms: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
        let check_interval_ms: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
        let mut elapsed_ms: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while elapsed_ms < max_wait_ms && (*server).thread_running as ::core::ffi::c_int != 0 {
            uv_async_send(&raw mut (*server).stop_async);
            uv_stop((*server).r#loop);
            usleep((check_interval_ms * 1000 as ::core::ffi::c_int) as __useconds_t);
            elapsed_ms += check_interval_ms;
        }
        uv_thread_join(&raw mut (*server).thread);
        (*server).thread_running = r#false != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_server_stop: Server thread exited after %d ms\0".as_ptr()
                as *const ::core::ffi::c_char,
            elapsed_ms,
        );
    }
    if uv_is_closing(&raw mut (*server).listener as *mut uv_handle_t) == 0 {
        uv_close(&raw mut (*server).listener as *mut uv_handle_t, None);
    }
    if uv_is_closing(&raw mut (*server).stop_async as *mut uv_handle_t) == 0 {
        uv_close(&raw mut (*server).stop_async as *mut uv_handle_t, None);
    }
    go2rtc_proxy_thread_shutdown();
    go2rtc_websocket_shutdown((*server).r#loop);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_server_stop: Closing all active handles\0".as_ptr() as *const ::core::ffi::c_char,
    );
    uv_walk(
        (*server).r#loop,
        Some(
            close_walk_cb as unsafe extern "C" fn(*mut uv_handle_t, *mut ::core::ffi::c_void) -> (),
        ),
        NULL,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_server_stop: Processing close callbacks\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let max_wait_ms_0: ::core::ffi::c_int = 3000 as ::core::ffi::c_int;
    let check_interval_ms_0: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut elapsed_ms_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while elapsed_ms_0 < max_wait_ms_0 {
        let mut active: ::core::ffi::c_int = uv_run((*server).r#loop, uv_run_mode::UV_RUN_NOWAIT);
        if active == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_stop: All handles closed after %d ms\0".as_ptr()
                    as *const ::core::ffi::c_char,
                elapsed_ms_0,
            );
            break;
        } else {
            if elapsed_ms_0 > 0 as ::core::ffi::c_int
                && elapsed_ms_0 % 1000 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                let mut handle_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                uv_walk(
                    (*server).r#loop,
                    Some(
                        count_handles_cb
                            as unsafe extern "C" fn(
                                *mut uv_handle_t,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                    &raw mut handle_count as *mut ::core::ffi::c_void,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"libuv_server_stop: Still waiting for %d handles after %d ms\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    handle_count,
                    elapsed_ms_0,
                );
            }
            usleep((check_interval_ms_0 * 1000 as ::core::ffi::c_int) as __useconds_t);
            elapsed_ms_0 += check_interval_ms_0;
        }
    }
    if elapsed_ms_0 >= max_wait_ms_0 {
        let mut handle_count_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        uv_walk(
            (*server).r#loop,
            Some(
                count_handles_cb
                    as unsafe extern "C" fn(*mut uv_handle_t, *mut ::core::ffi::c_void) -> (),
            ),
            &raw mut handle_count_0 as *mut ::core::ffi::c_void,
        );
        if handle_count_0 == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_stop: Timeout with no open handles - handler work is still in flight on the thread pool (web_thread_pool_size=%d). The event loop stays alive until it drains.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                g_config.web_thread_pool_size,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_stop: Timeout waiting for handles to close, %d handles still active\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                handle_count_0,
            );
        }
        while uv_loop_alive((*server).r#loop) != 0 {
            uv_run((*server).r#loop, uv_run_mode::UV_RUN_DEFAULT);
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_server_stop: Server stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_server_destroy(mut handle: http_server_handle_t) {
    let mut server: *mut libuv_server_t = handle as *mut libuv_server_t;
    if server.is_null() {
        return;
    }
    if !::core::ptr::read_volatile::<bool>(&raw const (*server).shutting_down) {
        libuv_server_stop(handle);
    }
    thumbnail_thread_shutdown();
    go2rtc_proxy_thread_shutdown();
    if (*server).owns_loop as ::core::ffi::c_int != 0 && !(*server).r#loop.is_null() {
        let mut closed: ::core::ffi::c_int = uv_loop_close((*server).r#loop);
        if closed != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_destroy: Refusing to free busy event loop: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                uv_strerror(closed),
            );
            return;
        }
    }
    if !(*server).handlers.is_null() {
        safe_free((*server).handlers as *mut ::core::ffi::c_void);
    }
    if !(*server).tls_ctx.is_null() {
        libuv_tls_context_free((*server).tls_ctx as *mut libuv_tls_context_t);
        (*server).tls_ctx = NULL;
        httpd_set_secure_cookies(r#false != 0);
    }
    if (*server).owns_loop as ::core::ffi::c_int != 0 && !(*server).r#loop.is_null() {
        safe_free((*server).r#loop as *mut ::core::ffi::c_void);
    }
    safe_free(server as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_server_destroy: Server destroyed\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_server_register_handler(
    mut handle: http_server_handle_t,
    mut path: *const ::core::ffi::c_char,
    mut method: *const ::core::ffi::c_char,
    mut handler: request_handler_t,
) -> ::core::ffi::c_int {
    let mut server: *mut libuv_server_t = handle as *mut libuv_server_t;
    if server.is_null() || path.is_null() || handler.is_none() {
        return -1 as ::core::ffi::c_int;
    }
    if (*server).handler_count >= (*server).handler_capacity {
        let mut new_capacity: ::core::ffi::c_int =
            (*server).handler_capacity * 2 as ::core::ffi::c_int;
        let mut new_handlers: *mut ::core::ffi::c_void = safe_realloc(
            (*server).handlers as *mut ::core::ffi::c_void,
            (new_capacity as size_t).wrapping_mul(::core::mem::size_of::<C2Rust_Unnamed_11>()),
        );
        if new_handlers.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"libuv_server_register_handler: Failed to expand handler registry\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*server).handlers = new_handlers as *mut C2Rust_Unnamed_11;
        (*server).handler_capacity = new_capacity;
    }
    let mut idx: ::core::ffi::c_int = (*server).handler_count;
    safe_strcpy(
        &raw mut (*(*server).handlers.offset(idx as isize)).path as *mut ::core::ffi::c_char,
        path,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    if !method.is_null() {
        safe_strcpy(
            &raw mut (*(*server).handlers.offset(idx as isize)).method as *mut ::core::ffi::c_char,
            method,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    } else {
        (*(*server).handlers.offset(idx as isize)).method[0usize] = '\0' as ::core::ffi::c_char;
    }
    (*(*server).handlers.offset(idx as isize)).handler = handler;
    (*server).handler_count += 1;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"libuv_server_register_handler: Registered %s %s\0".as_ptr() as *const ::core::ffi::c_char,
        if !method.is_null() {
            method
        } else {
            b"*\0".as_ptr() as *const ::core::ffi::c_char
        },
        path,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn on_connection(mut listener: *mut uv_stream_t, mut status: ::core::ffi::c_int) {
    if status < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_connection: Error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(status),
        );
        return;
    }
    let mut server: *mut libuv_server_t = (*listener).data as *mut libuv_server_t;
    let mut conn: *mut libuv_connection_t = libuv_connection_create(server);
    if conn.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_connection: Failed to create connection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut r: ::core::ffi::c_int =
        uv_accept(listener, &raw mut (*conn).handle as *mut uv_stream_t);
    if r != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_connection: Accept failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(r),
        );
        libuv_connection_close(conn);
        return;
    }
    let mut addr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut addr_len: ::core::ffi::c_int =
        ::core::mem::size_of::<sockaddr_storage>() as ::core::ffi::c_int;
    uv_tcp_getpeername(
        &raw mut (*conn).handle,
        &raw mut addr as *mut sockaddr,
        &raw mut addr_len,
    );
    let mut ip: [::core::ffi::c_char; 64] = [
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
    if addr.ss_family as ::core::ffi::c_int == AF_INET {
        uv_ip4_name(
            &raw mut addr as *mut sockaddr_in,
            &raw mut ip as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        );
    } else if addr.ss_family as ::core::ffi::c_int == AF_INET6 {
        uv_ip6_name(
            &raw mut addr as *mut sockaddr_in6,
            &raw mut ip as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        );
    }
    safe_strcpy(
        &raw mut (*conn).request.client_ip as *mut ::core::ffi::c_char,
        &raw mut ip as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"on_connection: New connection from %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut ip as *mut ::core::ffi::c_char,
    );
    if !(*server).tls_ctx.is_null()
        && libuv_tls_session_start(conn, (*server).tls_ctx as *mut libuv_tls_context_t)
            != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_connection: Failed to start TLS session for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut ip as *mut ::core::ffi::c_char,
        );
        libuv_connection_close(conn);
        return;
    }
    r = libuv_connection_read_start(
        conn,
        Some(libuv_alloc_cb as unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> ()),
        Some(
            libuv_read_cb as unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> (),
        ),
    );
    if r != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_connection: Failed to start reading: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(r),
        );
        libuv_connection_close(conn);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_server_start(mut server: http_server_handle_t) -> ::core::ffi::c_int {
    return libuv_server_start(server);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_server_stop(mut server: http_server_handle_t) {
    libuv_server_stop(server);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_server_destroy(mut server: http_server_handle_t) {
    libuv_server_destroy(server);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn http_server_register_handler(
    mut server: http_server_handle_t,
    mut path: *const ::core::ffi::c_char,
    mut method: *const ::core::ffi::c_char,
    mut handler: request_handler_t,
) -> ::core::ffi::c_int {
    return libuv_server_register_handler(server, path, method, handler);
}
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
