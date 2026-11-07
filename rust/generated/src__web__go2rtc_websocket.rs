unsafe extern "C" {
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    unsafe fn uv_close(handle: *mut uv_handle_t, close_cb: uv_close_cb);
    unsafe fn uv_buf_init(base: *mut ::core::ffi::c_char, len: ::core::ffi::c_uint) -> uv_buf_t;
    unsafe fn uv_read_start(
        _: *mut uv_stream_t,
        alloc_cb: uv_alloc_cb,
        read_cb: uv_read_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_read_stop(_: *mut uv_stream_t) -> ::core::ffi::c_int;
    unsafe fn uv_write(
        req: *mut uv_write_t,
        handle: *mut uv_stream_t,
        bufs: *const uv_buf_t,
        nbufs: ::core::ffi::c_uint,
        cb: uv_write_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_tcp_init(_: *mut uv_loop_t, handle: *mut uv_tcp_t) -> ::core::ffi::c_int;
    unsafe fn uv_tcp_connect(
        req: *mut uv_connect_t,
        handle: *mut uv_tcp_t,
        addr: *const sockaddr,
        cb: uv_connect_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_timer_init(_: *mut uv_loop_t, handle: *mut uv_timer_t) -> ::core::ffi::c_int;
    unsafe fn uv_timer_start(
        handle: *mut uv_timer_t,
        cb: uv_timer_cb,
        timeout: uint64_t,
        repeat: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_timer_stop(handle: *mut uv_timer_t) -> ::core::ffi::c_int;
    unsafe fn uv_ip4_addr(
        ip: *const ::core::ffi::c_char,
        port: ::core::ffi::c_int,
        addr: *mut sockaddr_in,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_set_json_error(
        res: *mut http_response_t,
        status_code: ::core::ffi::c_int,
        error_message: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_send_response_ex(
        conn: *mut libuv_connection_t,
        response: *const http_response_t,
        action: write_complete_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_connection_close(conn: *mut libuv_connection_t);
    unsafe fn httpd_secure_cookies() -> bool;
    unsafe fn libuv_connection_write(
        conn: *mut libuv_connection,
        req: *mut uv_write_t,
        bufs: *const uv_buf_t,
        nbufs: ::core::ffi::c_uint,
        cb: uv_write_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_connection_read_start(
        conn: *mut libuv_connection,
        alloc_cb: uv_alloc_cb,
        read_cb: uv_read_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_connection_read_stop(conn: *mut libuv_connection) -> ::core::ffi::c_int;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
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
    unsafe fn strcasestr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
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
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_timer_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_11,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub timer_cb: uv_timer_cb,
    pub node: C2Rust_Unnamed_10,
    pub timeout: uint64_t,
    pub repeat: uint64_t,
    pub start_id: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub heap: [*mut ::core::ffi::c_void; 3],
    pub queue: uv__queue,
}
pub type uv_timer_cb = Option<unsafe extern "C" fn(*mut uv_timer_t) -> ()>;
pub type uv_timer_t = uv_timer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_write_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_12,
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
pub union C2Rust_Unnamed_12 {
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
    pub handlers: *mut C2Rust_Unnamed_13,
    pub handler_count: ::core::ffi::c_int,
    pub handler_capacity: ::core::ffi::c_int,
    pub tls_ctx: *mut ::core::ffi::c_void,
    pub thread: uv_thread_t,
    pub thread_running: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
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
pub type websocket_proxy_t = websocket_proxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct websocket_proxy {
    pub upstream: uv_tcp_t,
    pub timeout: uv_timer_t,
    pub connect: uv_connect_t,
    pub conn: *mut libuv_connection_t,
    pub next: *mut websocket_proxy,
    pub refs: ::core::ffi::c_uint,
    pub closing: bool,
    pub upgraded: bool,
    pub handshake: [::core::ffi::c_char; 4096],
    pub headers: [::core::ffi::c_char; 8192],
    pub header_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct websocket_write_t {
    pub request: uv_write_t,
    pub proxy: *mut websocket_proxy_t,
    pub resume: *mut uv_stream_t,
    pub data: *mut ::core::ffi::c_char,
}
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const WS_BUFFER_SIZE: ::core::ffi::c_int =
    64 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
pub const WS_CONNECTION_LIMIT: ::core::ffi::c_int = if MAX_STREAMS < 32 as ::core::ffi::c_int {
    64 as ::core::ffi::c_int
} else {
    MAX_STREAMS * 2 as ::core::ffi::c_int
};
static mut proxies: *mut websocket_proxy_t = ::core::ptr::null_mut::<websocket_proxy_t>();
static mut proxy_count: ::core::ffi::c_uint = 0;
unsafe extern "C" fn single_header(
    mut req: *const http_request_t,
    mut name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut value: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*req).num_headers {
        if strcasecmp(
            &raw const (*(&raw const (*req).headers as *const http_header_t).offset(i as isize))
                .name as *const ::core::ffi::c_char,
            name,
        ) == 0 as ::core::ffi::c_int
        {
            if !value.is_null() {
                return ::core::ptr::null::<::core::ffi::c_char>();
            }
            value = &raw const (*(&raw const (*req).headers as *const http_header_t)
                .offset(i as isize))
            .value as *const ::core::ffi::c_char;
        }
        i += 1;
    }
    return value;
}
unsafe extern "C" fn header_token(
    mut value: *const ::core::ffi::c_char,
    mut token: *const ::core::ffi::c_char,
) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    while *value != 0 {
        while *value as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *value as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            || *value as ::core::ffi::c_int == ',' as ::core::ffi::c_int
        {
            value = value.offset(1);
        }
        let mut end: *const ::core::ffi::c_char = strchr(value, ',' as ::core::ffi::c_int);
        if end.is_null() {
            end = value.offset(strlen(value) as isize);
        }
        let mut trimmed: *const ::core::ffi::c_char = end;
        while trimmed > value
            && *(*__ctype_b_loc()).offset(
                *trimmed.offset(-1isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
            ) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            trimmed = trimmed.offset(-1);
        }
        if trimmed.offset_from(value) as size_t == strlen(token)
            && strncasecmp(value, token, strlen(token)) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        value = if *end as ::core::ffi::c_int != 0 {
            end.offset(1 as ::core::ffi::c_int as isize)
        } else {
            end
        };
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_websocket_validate(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut key: *const ::core::ffi::c_char = single_header(
        req,
        b"Sec-WebSocket-Key\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut version: *const ::core::ffi::c_char = single_header(
        req,
        b"Sec-WebSocket-Version\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut valid_key: bool = !key.is_null()
        && strlen(key) == 24 as size_t
        && strcmp(
            key.offset(22 as ::core::ffi::c_int as isize),
            b"==\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while valid_key as ::core::ffi::c_int != 0 && i < 22 as ::core::ffi::c_int {
        valid_key = *(*__ctype_b_loc())
            .offset(*key.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            || *key.offset(i as isize) as ::core::ffi::c_int == '+' as ::core::ffi::c_int
            || *key.offset(i as isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int;
        i += 1;
    }
    if !valid_key
        || version.is_null()
        || strcmp(version, b"13\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        || !header_token(
            single_header(req, b"Upgrade\0".as_ptr() as *const ::core::ffi::c_char),
            b"websocket\0".as_ptr() as *const ::core::ffi::c_char,
        )
        || !header_token(
            single_header(req, b"Connection\0".as_ptr() as *const ::core::ffi::c_char),
            b"upgrade\0".as_ptr() as *const ::core::ffi::c_char,
        )
        || (*req).content_length != 0
        || (*req).body_len != 0
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"WebSocket upgrade required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut origin: *const ::core::ffi::c_char =
        single_header(req, b"Origin\0".as_ptr() as *const ::core::ffi::c_char);
    let mut host: *const ::core::ffi::c_char =
        single_header(req, b"Host\0".as_ptr() as *const ::core::ffi::c_char);
    let mut origin_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*req).num_headers {
        if strcasecmp(
            &raw const (*(&raw const (*req).headers as *const http_header_t).offset(i_0 as isize))
                .name as *const ::core::ffi::c_char,
            b"Origin\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            origin_count += 1;
        }
        i_0 += 1;
    }
    let mut authority: *const ::core::ffi::c_char = if !origin.is_null()
        && strncmp(
            origin,
            b"http://\0".as_ptr() as *const ::core::ffi::c_char,
            7 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        origin.offset(7 as ::core::ffi::c_int as isize)
    } else if !origin.is_null()
        && strncmp(
            origin,
            b"https://\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        origin.offset(8 as ::core::ffi::c_int as isize)
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
    if host.is_null()
        || origin_count > 1 as ::core::ffi::c_int
        || !origin.is_null()
            && (authority.is_null() || strcasecmp(authority, host) != 0 as ::core::ffi::c_int)
    {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"WebSocket origin denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn release_proxy(mut proxy: *mut websocket_proxy_t) {
    (*proxy).refs = (*proxy).refs.wrapping_sub(1);
    if (*proxy).refs != 0 {
        return;
    }
    let mut link: *mut *mut websocket_proxy_t = &raw mut proxies;
    while !(*link).is_null() && *link != proxy {
        link = &raw mut (**link).next as *mut *mut websocket_proxy_t;
    }
    if !(*link).is_null() {
        *link = (*proxy).next as *mut websocket_proxy_t;
    }
    proxy_count = proxy_count.wrapping_sub(1);
    free(proxy as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn handle_closed(mut handle: *mut uv_handle_t) {
    release_proxy((*handle).data as *mut websocket_proxy_t);
}
unsafe extern "C" fn close_proxy(mut proxy: *mut websocket_proxy_t) {
    if (*proxy).closing {
        return;
    }
    (*proxy).closing = r#true != 0;
    if !(*proxy).conn.is_null() {
        let mut conn: *mut libuv_connection_t = (*proxy).conn;
        (*proxy).conn = ::core::ptr::null_mut::<libuv_connection_t>();
        (*conn).go2rtc_websocket = NULL;
        libuv_connection_close(conn);
    }
    uv_read_stop(&raw mut (*proxy).upstream as *mut uv_stream_t);
    uv_timer_stop(&raw mut (*proxy).timeout);
    uv_close(
        &raw mut (*proxy).upstream as *mut uv_handle_t,
        Some(handle_closed as unsafe extern "C" fn(*mut uv_handle_t) -> ()),
    );
    uv_close(
        &raw mut (*proxy).timeout as *mut uv_handle_t,
        Some(handle_closed as unsafe extern "C" fn(*mut uv_handle_t) -> ()),
    );
}
unsafe extern "C" fn fail_proxy(mut proxy: *mut websocket_proxy_t) {
    if !(*proxy).upgraded && !(*proxy).conn.is_null() && !(*proxy).closing {
        let mut conn: *mut libuv_connection_t = (*proxy).conn;
        (*proxy).conn = ::core::ptr::null_mut::<libuv_connection_t>();
        (*conn).go2rtc_websocket = NULL;
        (*conn).async_response_pending = r#false != 0;
        http_response_set_json_error(
            &raw mut (*conn).response,
            502 as ::core::ffi::c_int,
            b"go2rtc WebSocket unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        libuv_send_response_ex(
            conn,
            &raw mut (*conn).response,
            write_complete_action_t::WRITE_ACTION_CLOSE,
        );
    }
    close_proxy(proxy);
}
unsafe extern "C" fn is_client_stream(
    mut proxy: *const websocket_proxy_t,
    mut stream: *const uv_stream_t,
) -> bool {
    return !(*proxy).conn.is_null()
        && stream == &raw mut (*(*proxy).conn).handle as *const uv_stream_t;
}
unsafe extern "C" fn stream_read_start(
    mut proxy: *mut websocket_proxy_t,
    mut stream: *mut uv_stream_t,
    mut alloc_cb: uv_alloc_cb,
    mut read_cb: uv_read_cb,
) -> ::core::ffi::c_int {
    return if is_client_stream(proxy, stream) as ::core::ffi::c_int != 0 {
        libuv_connection_read_start((*proxy).conn, alloc_cb, read_cb)
    } else {
        uv_read_start(stream, alloc_cb, read_cb)
    };
}
unsafe extern "C" fn stream_read_stop(
    mut proxy: *mut websocket_proxy_t,
    mut stream: *mut uv_stream_t,
) {
    if is_client_stream(proxy, stream) {
        libuv_connection_read_stop((*proxy).conn);
    } else {
        uv_read_stop(stream);
    };
}
unsafe extern "C" fn allocate_read(
    mut handle: *mut uv_handle_t,
    mut suggested: size_t,
    mut buf: *mut uv_buf_t,
) {
    (*buf).base = malloc(WS_BUFFER_SIZE as size_t) as *mut ::core::ffi::c_char;
    (*buf).len = (if !(*buf).base.is_null() {
        WS_BUFFER_SIZE
    } else {
        0 as ::core::ffi::c_int
    }) as size_t;
}
unsafe extern "C" fn write_done(mut request: *mut uv_write_t, mut status: ::core::ffi::c_int) {
    let mut write: *mut websocket_write_t = request as *mut websocket_write_t;
    let mut proxy: *mut websocket_proxy_t = (*write).proxy;
    if !(*proxy).closing {
        if status < 0 as ::core::ffi::c_int {
            fail_proxy(proxy);
        } else if !(*write).resume.is_null() {
            let mut callback: uv_read_cb = if (*write).resume
                == &raw mut (*proxy).upstream as *mut uv_stream_t
            {
                Some(
                    upstream_read
                        as unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> (),
                )
            } else {
                Some(
                    client_read
                        as unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> (),
                )
            };
            if stream_read_start(
                proxy,
                (*write).resume,
                Some(
                    allocate_read
                        as unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> (),
                ),
                callback,
            ) < 0 as ::core::ffi::c_int
            {
                fail_proxy(proxy);
            }
        }
    }
    free((*write).data as *mut ::core::ffi::c_void);
    free(write as *mut ::core::ffi::c_void);
    release_proxy(proxy);
}
unsafe extern "C" fn relay(
    mut proxy: *mut websocket_proxy_t,
    mut target: *mut uv_stream_t,
    mut resume: *mut uv_stream_t,
    mut data: *const ::core::ffi::c_char,
    mut length: size_t,
) -> bool {
    let mut write: *mut websocket_write_t =
        calloc(1 as size_t, ::core::mem::size_of::<websocket_write_t>()) as *mut websocket_write_t;
    if write.is_null() {
        return r#false != 0;
    }
    (*write).data = malloc(length) as *mut ::core::ffi::c_char;
    if (*write).data.is_null() {
        free(write as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    memcpy(
        (*write).data as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        length,
    );
    (*write).proxy = proxy;
    (*write).resume = resume;
    let mut buffer: uv_buf_t = uv_buf_init((*write).data, length as ::core::ffi::c_uint);
    if !resume.is_null() {
        stream_read_stop(proxy, resume);
    }
    let mut r: ::core::ffi::c_int = if is_client_stream(proxy, target) as ::core::ffi::c_int != 0 {
        libuv_connection_write(
            (*proxy).conn,
            &raw mut (*write).request,
            &raw mut buffer,
            1 as ::core::ffi::c_uint,
            Some(write_done as unsafe extern "C" fn(*mut uv_write_t, ::core::ffi::c_int) -> ()),
        )
    } else {
        uv_write(
            &raw mut (*write).request,
            target,
            &raw mut buffer,
            1 as ::core::ffi::c_uint,
            Some(write_done as unsafe extern "C" fn(*mut uv_write_t, ::core::ffi::c_int) -> ()),
        )
    };
    if r < 0 as ::core::ffi::c_int {
        free((*write).data as *mut ::core::ffi::c_void);
        free(write as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    (*proxy).refs = (*proxy).refs.wrapping_add(1);
    return r#true != 0;
}
unsafe extern "C" fn client_read(
    mut stream: *mut uv_stream_t,
    mut count: ssize_t,
    mut buf: *const uv_buf_t,
) {
    let mut conn: *mut libuv_connection_t = (*stream).data as *mut libuv_connection_t;
    let mut proxy: *mut websocket_proxy_t = (*conn).go2rtc_websocket as *mut websocket_proxy_t;
    if !proxy.is_null() && !(*proxy).closing {
        if count < 0 as ssize_t
            || count > 0 as ssize_t
                && !relay(
                    proxy,
                    &raw mut (*proxy).upstream as *mut uv_stream_t,
                    stream,
                    (*buf).base,
                    count as size_t,
                )
        {
            close_proxy(proxy);
        }
    }
    free((*buf).base as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn upstream_read(
    mut stream: *mut uv_stream_t,
    mut count: ssize_t,
    mut buf: *const uv_buf_t,
) {
    let mut proxy: *mut websocket_proxy_t = (*stream).data as *mut websocket_proxy_t;
    if (*proxy).closing as ::core::ffi::c_int != 0 || count <= 0 as ssize_t {
        if !(*proxy).closing && count < 0 as ssize_t {
            fail_proxy(proxy);
        }
        free((*buf).base as *mut ::core::ffi::c_void);
        return;
    }
    let mut data: *const ::core::ffi::c_char = (*buf).base;
    let mut length: size_t = count as size_t;
    if !(*proxy).upgraded {
        let mut consumed: size_t = 0 as size_t;
        while consumed < length
            && (*proxy).header_size.wrapping_add(1 as size_t)
                < ::core::mem::size_of::<[::core::ffi::c_char; 8192]>()
        {
            let c2rust_fresh0 = consumed;
            consumed = consumed.wrapping_add(1);
            let c2rust_fresh1 = (*proxy).header_size;
            (*proxy).header_size = (*proxy).header_size.wrapping_add(1);
            (*proxy).headers[c2rust_fresh1] = *data.offset(c2rust_fresh0 as isize);
            (*proxy).headers[(*proxy).header_size] = '\0' as ::core::ffi::c_char;
            if (*proxy).header_size >= 4 as size_t
                && memcmp(
                    (&raw mut (*proxy).headers as *mut ::core::ffi::c_char)
                        .offset((*proxy).header_size as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize))
                        as *const ::core::ffi::c_void,
                    b"\r\n\r\n\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    4 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                break;
            }
        }
        if strstr(
            &raw mut (*proxy).headers as *mut ::core::ffi::c_char,
            b"\r\n\r\n\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            if (*proxy).header_size.wrapping_add(1 as size_t)
                == ::core::mem::size_of::<[::core::ffi::c_char; 8192]>()
            {
                fail_proxy(proxy);
            }
            free((*buf).base as *mut ::core::ffi::c_void);
            return;
        }
        if strncmp(
            &raw mut (*proxy).headers as *mut ::core::ffi::c_char,
            b"HTTP/1.1 101 \0".as_ptr() as *const ::core::ffi::c_char,
            13 as size_t,
        ) != 0 as ::core::ffi::c_int
            || strcasestr(
                &raw mut (*proxy).headers as *mut ::core::ffi::c_char,
                b"\r\nX-LightNVR-Media-Policy: 1\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            fail_proxy(proxy);
            free((*buf).base as *mut ::core::ffi::c_void);
            return;
        }
        (*proxy).upgraded = r#true != 0;
        uv_timer_stop(&raw mut (*proxy).timeout);
        if !relay(
            proxy,
            &raw mut (*(*proxy).conn).handle as *mut uv_stream_t,
            ::core::ptr::null_mut::<uv_stream_t>(),
            &raw mut (*proxy).headers as *mut ::core::ffi::c_char,
            (*proxy).header_size,
        ) || libuv_connection_read_start(
            (*proxy).conn,
            Some(
                allocate_read
                    as unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> (),
            ),
            Some(
                client_read
                    as unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> (),
            ),
        ) < 0 as ::core::ffi::c_int
        {
            close_proxy(proxy);
            free((*buf).base as *mut ::core::ffi::c_void);
            return;
        }
        data = data.offset(consumed as isize);
        length = length.wrapping_sub(consumed);
    }
    if length != 0
        && !relay(
            proxy,
            &raw mut (*(*proxy).conn).handle as *mut uv_stream_t,
            stream,
            data,
            length,
        )
    {
        close_proxy(proxy);
    }
    free((*buf).base as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn connect_done(mut request: *mut uv_connect_t, mut status: ::core::ffi::c_int) {
    let mut proxy: *mut websocket_proxy_t = (*request).data as *mut websocket_proxy_t;
    if (*proxy).closing {
        return;
    }
    if status < 0 as ::core::ffi::c_int
        || !relay(
            proxy,
            &raw mut (*proxy).upstream as *mut uv_stream_t,
            ::core::ptr::null_mut::<uv_stream_t>(),
            &raw mut (*proxy).handshake as *mut ::core::ffi::c_char,
            strlen(&raw mut (*proxy).handshake as *mut ::core::ffi::c_char),
        )
        || uv_read_start(
            &raw mut (*proxy).upstream as *mut uv_stream_t,
            Some(
                allocate_read
                    as unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> (),
            ),
            Some(
                upstream_read
                    as unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> (),
            ),
        ) < 0 as ::core::ffi::c_int
    {
        fail_proxy(proxy);
    }
}
unsafe extern "C" fn handshake_timeout(mut timer: *mut uv_timer_t) {
    fail_proxy((*timer).data as *mut websocket_proxy_t);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_websocket_start(
    mut conn: *mut libuv_connection_t,
    mut url: *const ::core::ffi::c_char,
    mut lease: *const ::core::ffi::c_char,
) {
    let mut proxy: *mut websocket_proxy_t = ::core::ptr::null_mut::<websocket_proxy_t>();
    if proxy_count >= WS_CONNECTION_LIMIT as ::core::ffi::c_uint || {
        proxy = calloc(1 as size_t, ::core::mem::size_of::<websocket_proxy_t>())
            as *mut websocket_proxy_t;
        proxy.is_null()
    } {
        http_response_set_json_error(
            &raw mut (*conn).response,
            503 as ::core::ffi::c_int,
            b"WebSocket capacity reached\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*conn).async_response_pending = r#false != 0;
        libuv_send_response_ex(
            conn,
            &raw mut (*conn).response,
            write_complete_action_t::WRITE_ACTION_CLOSE,
        );
        return;
    }
    let mut target: *const ::core::ffi::c_char = strchr(
        url.offset(strlen(b"http://\0".as_ptr() as *const ::core::ffi::c_char) as isize),
        '/' as ::core::ffi::c_int,
    );
    let mut port: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    sscanf(
        url,
        b"http://127.0.0.1:%d/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut port,
    );
    let mut key: *const ::core::ffi::c_char = single_header(
        &raw mut (*conn).request,
        b"Sec-WebSocket-Key\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut length: ::core::ffi::c_int = snprintf(
        &raw mut (*proxy).handshake as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"GET %s HTTP/1.1\r\nHost: 127.0.0.1:%d\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: %s\r\nSec-WebSocket-Version: 13\r\nX-LightNVR-Media-Lease: %s\r\nX-LightNVR-Media-TLS: %d\r\n\r\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if !target.is_null() {
            target
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        port,
        if !key.is_null() { key } else { b"\0".as_ptr() as *const ::core::ffi::c_char },
        lease,
        if httpd_secure_cookies() as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    if target.is_null()
        || port <= 0 as ::core::ffi::c_int
        || port > 65535 as ::core::ffi::c_int
        || length < 0 as ::core::ffi::c_int
        || length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
        || uv_tcp_init((*(*conn).server).r#loop, &raw mut (*proxy).upstream)
            < 0 as ::core::ffi::c_int
    {
        free(proxy as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            &raw mut (*conn).response,
            502 as ::core::ffi::c_int,
            b"Invalid WebSocket upstream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*conn).async_response_pending = r#false != 0;
        libuv_send_response_ex(
            conn,
            &raw mut (*conn).response,
            write_complete_action_t::WRITE_ACTION_CLOSE,
        );
        return;
    }
    (*proxy).upstream.data = proxy as *mut ::core::ffi::c_void;
    (*proxy).refs = 1 as ::core::ffi::c_uint;
    (*proxy).next = proxies as *mut websocket_proxy;
    proxies = proxy;
    proxy_count = proxy_count.wrapping_add(1);
    if uv_timer_init((*(*conn).server).r#loop, &raw mut (*proxy).timeout) < 0 as ::core::ffi::c_int
    {
        (*proxy).closing = r#true != 0;
        uv_close(
            &raw mut (*proxy).upstream as *mut uv_handle_t,
            Some(handle_closed as unsafe extern "C" fn(*mut uv_handle_t) -> ()),
        );
        libuv_connection_close(conn);
        return;
    }
    (*proxy).refs = (*proxy).refs.wrapping_add(1);
    (*proxy).timeout.data = proxy as *mut ::core::ffi::c_void;
    (*proxy).connect.data = proxy as *mut ::core::ffi::c_void;
    (*proxy).conn = conn;
    (*conn).go2rtc_websocket = proxy as *mut ::core::ffi::c_void;
    let mut address: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    uv_ip4_addr(
        b"127.0.0.1\0".as_ptr() as *const ::core::ffi::c_char,
        port,
        &raw mut address,
    );
    uv_timer_start(
        &raw mut (*proxy).timeout,
        Some(handshake_timeout as unsafe extern "C" fn(*mut uv_timer_t) -> ()),
        5000 as uint64_t,
        0 as uint64_t,
    );
    if uv_tcp_connect(
        &raw mut (*proxy).connect,
        &raw mut (*proxy).upstream,
        &raw mut address as *const sockaddr,
        Some(connect_done as unsafe extern "C" fn(*mut uv_connect_t, ::core::ffi::c_int) -> ()),
    ) < 0 as ::core::ffi::c_int
    {
        fail_proxy(proxy);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_websocket_disconnected(mut conn: *mut libuv_connection_t) {
    let mut proxy: *mut websocket_proxy_t = (*conn).go2rtc_websocket as *mut websocket_proxy_t;
    if proxy.is_null() {
        return;
    }
    (*conn).go2rtc_websocket = NULL;
    (*proxy).conn = ::core::ptr::null_mut::<libuv_connection_t>();
    close_proxy(proxy);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_websocket_shutdown(mut r#loop: *mut uv_loop_t) {
    let mut proxy: *mut websocket_proxy_t = proxies;
    while !proxy.is_null() {
        if (*proxy).upstream.r#loop == r#loop {
            close_proxy(proxy);
        }
        proxy = (*proxy).next as *mut websocket_proxy_t;
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
