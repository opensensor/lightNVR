unsafe extern "C" {
    unsafe fn http_request_get_header(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    unsafe fn http_request_get_query_param(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
        value: *mut ::core::ffi::c_char,
        value_len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_add_header(
        res: *mut http_response_t,
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
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
    unsafe fn llhttp_should_keep_alive(parser: *const llhttp_t) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn uv_close(handle: *mut uv_handle_t, close_cb: uv_close_cb);
    unsafe fn uv_buf_init(base: *mut ::core::ffi::c_char, len: ::core::ffi::c_uint) -> uv_buf_t;
    unsafe fn uv_is_closing(handle: *const uv_handle_t) -> ::core::ffi::c_int;
    unsafe fn uv_timer_init(_: *mut uv_loop_t, handle: *mut uv_timer_t) -> ::core::ffi::c_int;
    unsafe fn uv_timer_start(
        handle: *mut uv_timer_t,
        cb: uv_timer_cb,
        timeout: uint64_t,
        repeat: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_timer_stop(handle: *mut uv_timer_t) -> ::core::ffi::c_int;
    unsafe fn uv_queue_work(
        r#loop: *mut uv_loop_t,
        req: *mut uv_work_t,
        work_cb: uv_work_cb,
        after_work_cb: uv_after_work_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_send_response_ex(
        conn: *mut libuv_connection_t,
        response: *const http_response_t,
        action: write_complete_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_connection_close(conn: *mut libuv_connection_t);
    unsafe fn libuv_connection_write(
        conn: *mut libuv_connection,
        req: *mut uv_write_t,
        bufs: *const uv_buf_t,
        nbufs: ::core::ffi::c_uint,
        cb: uv_write_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_connection_reset(conn: *mut libuv_connection_t);
    unsafe fn libuv_parse_range_header(
        range_header: *const ::core::ffi::c_char,
        file_size: size_t,
        start: *mut size_t,
        end: *mut size_t,
    ) -> bool;
    unsafe fn storage_source_remote(
        id: uint64_t,
        source: *mut storage_remote_source_t,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_source_renew_lease(id: uint64_t) -> bool;
    unsafe fn storage_s3_read_range(
        target: *const storage_target_t,
        key: *const ::core::ffi::c_char,
        offset: uint64_t,
        length: size_t,
        total_size: uint64_t,
        buffer: *mut ::core::ffi::c_void,
        control: *const storage_transfer_control_t,
        error: *mut ::core::ffi::c_char,
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
    unsafe fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub type uint64_t = u64;
pub type uintptr_t = usize;
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
pub struct uv_timer_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_10,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub timer_cb: uv_timer_cb,
    pub node: C2Rust_Unnamed_9,
    pub timeout: uint64_t,
    pub repeat: uint64_t,
    pub start_id: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub heap: [*mut ::core::ffi::c_void; 3],
    pub queue: uv__queue,
}
pub type uv_timer_cb = Option<unsafe extern "C" fn(*mut uv_timer_t) -> ()>;
pub type uv_timer_t = uv_timer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_write_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_11,
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
pub union C2Rust_Unnamed_11 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_work_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_12,
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
pub union C2Rust_Unnamed_12 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_stream_t {
    pub work: uv_work_t,
    pub write: uv_write_t,
    pub lease_timer: uv_timer_t,
    pub references: ::core::ffi::c_uint,
    pub timer_initialized: bool,
    pub conn: *mut libuv_connection_t,
    pub source: storage_remote_source_t,
    pub cancelled: atomic_bool,
    pub download: bool,
    pub headers_sent: bool,
    pub counted: bool,
    pub offset: uint64_t,
    pub remaining: uint64_t,
    pub length: size_t,
    pub result: ::core::ffi::c_int,
    pub error: [::core::ffi::c_char; 256],
    pub headers: [::core::ffi::c_char; 1024],
    pub header_length: size_t,
    pub buffer: [::core::ffi::c_uchar; 1048576],
}
pub type atomic_bool = bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_remote_source_t {
    pub recording_id: uint64_t,
    pub target: storage_target_t,
    pub key: [::core::ffi::c_char; 512],
    pub size: uint64_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_transfer_control_t {
    pub cancelled: storage_transfer_cancel_fn,
    pub context: *mut ::core::ffi::c_void,
    pub bandwidth_bps: uint64_t,
    pub job_uuid: *const ::core::ffi::c_char,
}
pub type storage_transfer_cancel_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> bool>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_14(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_14 {
    pub const STORAGE_SOURCE_READY: Self = Self(0);
    pub const STORAGE_SOURCE_PREPARING: Self = Self(1);
    pub const STORAGE_SOURCE_ERROR: Self = Self(-1);
    pub const STORAGE_SOURCE_MISSING: Self = Self(-2);
    pub const STORAGE_SOURCE_DELETING: Self = Self(-3);
}
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const ARCHIVE_CHUNK: ::core::ffi::c_uint =
    (1024 as ::core::ffi::c_uint).wrapping_mul(1024 as ::core::ffi::c_uint);
pub const ARCHIVE_STREAMS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut streams: [*mut archive_stream_t; 2] = [::core::ptr::null_mut::<archive_stream_t>(); 2];
static mut active_streams: ::core::ffi::c_uint = 0;
unsafe extern "C" fn archive_release(mut stream: *mut archive_stream_t) {
    (*stream).references = (*stream).references.wrapping_sub(1);
    if (*stream).references == 0 as ::core::ffi::c_uint {
        free(stream as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn archive_timer_closed(mut handle: *mut uv_handle_t) {
    archive_release((*handle).data as *mut archive_stream_t);
}
unsafe extern "C" fn archive_stop_timer(mut stream: *mut archive_stream_t) {
    if (*stream).timer_initialized as ::core::ffi::c_int != 0
        && uv_is_closing(&raw mut (*stream).lease_timer as *mut uv_handle_t) == 0
    {
        uv_timer_stop(&raw mut (*stream).lease_timer);
        uv_close(
            &raw mut (*stream).lease_timer as *mut uv_handle_t,
            Some(archive_timer_closed as unsafe extern "C" fn(*mut uv_handle_t) -> ()),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_archive_close_timer(mut handle: *mut uv_handle_t) -> bool {
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < ARCHIVE_STREAMS as ::core::ffi::c_uint {
        if !streams[i as usize].is_null()
            && handle
                == &raw mut (**(&raw mut streams as *mut *mut archive_stream_t).offset(i as isize))
                    .lease_timer as *mut uv_handle_t
        {
            archive_stop_timer(streams[i as usize]);
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn archive_free(mut stream: *mut archive_stream_t) {
    if (*stream).counted {
        active_streams = active_streams.wrapping_sub(1);
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while i < ARCHIVE_STREAMS as ::core::ffi::c_uint {
            if streams[i as usize] == stream {
                streams[i as usize] = ::core::ptr::null_mut::<archive_stream_t>();
            }
            i = i.wrapping_add(1);
        }
    }
    archive_stop_timer(stream);
    archive_release(stream);
}
unsafe extern "C" fn archive_finish(mut stream: *mut archive_stream_t, mut failed: bool) {
    let mut conn: *mut libuv_connection_t = (*stream).conn;
    if conn.is_null() {
        archive_free(stream);
        return;
    }
    (*conn).archive_stream = NULL;
    (*conn).async_response_pending = r#false != 0;
    let mut headers_sent: bool = (*stream).headers_sent;
    archive_free(stream);
    if failed as ::core::ffi::c_int != 0 && !headers_sent {
        http_response_set_json_error(
            &raw mut (*conn).response,
            503 as ::core::ffi::c_int,
            b"Archive source is unavailable; retry later\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_add_header(
            &raw mut (*conn).response,
            b"Retry-After\0".as_ptr() as *const ::core::ffi::c_char,
            b"5\0".as_ptr() as *const ::core::ffi::c_char,
        );
        libuv_send_response_ex(
            conn,
            &raw mut (*conn).response,
            write_complete_action_t::WRITE_ACTION_CLOSE,
        );
    } else if !failed
        && (*conn).keep_alive as ::core::ffi::c_int != 0
        && llhttp_should_keep_alive(&raw mut (*conn).parser) != 0
        && !::core::ptr::read_volatile::<bool>(&raw const (*(*conn).server).shutting_down)
    {
        libuv_connection_reset(conn);
    } else {
        libuv_connection_close(conn);
    };
}
unsafe extern "C" fn archive_cancelled(mut context: *mut ::core::ffi::c_void) -> bool {
    let mut stream: *mut archive_stream_t = context as *mut archive_stream_t;
    return ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*stream).cancelled).load(::core::sync::atomic::Ordering::SeqCst);
}
unsafe extern "C" fn archive_renew(mut timer: *mut uv_timer_t) {
    let mut stream: *mut archive_stream_t = (*timer).data as *mut archive_stream_t;
    if (*stream).conn.is_null()
        || archive_cancelled(stream as *mut ::core::ffi::c_void) as ::core::ffi::c_int != 0
    {
        archive_stop_timer(stream);
        return;
    }
    if !storage_source_renew_lease((*stream).source.recording_id) {
        ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*stream).cancelled).store(true, ::core::sync::atomic::Ordering::SeqCst);
        archive_stop_timer(stream);
        libuv_connection_close((*stream).conn);
    }
}
unsafe extern "C" fn archive_read(mut work: *mut uv_work_t) {
    let mut stream: *mut archive_stream_t = (*work).data as *mut archive_stream_t;
    (*stream).result = -1 as ::core::ffi::c_int;
    if !archive_cancelled(stream as *mut ::core::ffi::c_void)
        && storage_source_renew_lease((*stream).source.recording_id) as ::core::ffi::c_int != 0
    {
        let mut control: storage_transfer_control_t = storage_transfer_control_t {
            cancelled: Some(
                archive_cancelled as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> bool,
            ),
            context: stream as *mut ::core::ffi::c_void,
            bandwidth_bps: (*stream).source.target.migration_bandwidth_bps,
            job_uuid: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        (*stream).result = storage_s3_read_range(
            &raw mut (*stream).source.target,
            &raw mut (*stream).source.key as *mut ::core::ffi::c_char,
            (*stream).offset,
            (*stream).length,
            (*stream).source.size,
            &raw mut (*stream).buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            &raw mut control,
            &raw mut (*stream).error as *mut ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn archive_written(mut write: *mut uv_write_t, mut status: ::core::ffi::c_int) {
    let mut stream: *mut archive_stream_t = (*write).data as *mut archive_stream_t;
    if (*stream).conn.is_null()
        || status < 0 as ::core::ffi::c_int
        || archive_cancelled(stream as *mut ::core::ffi::c_void) as ::core::ffi::c_int != 0
    {
        archive_finish(stream, r#true != 0);
        return;
    }
    (*stream).offset = ((*stream).offset as ::core::ffi::c_ulong)
        .wrapping_add((*stream).length as ::core::ffi::c_ulong) as uint64_t;
    (*stream).remaining = ((*stream).remaining as ::core::ffi::c_ulong)
        .wrapping_sub((*stream).length as ::core::ffi::c_ulong)
        as uint64_t;
    archive_next(stream);
}
unsafe extern "C" fn archive_read_done(mut work: *mut uv_work_t, mut status: ::core::ffi::c_int) {
    let mut stream: *mut archive_stream_t = (*work).data as *mut archive_stream_t;
    if (*stream).conn.is_null()
        || status < 0 as ::core::ffi::c_int
        || (*stream).result != 0
        || archive_cancelled(stream as *mut ::core::ffi::c_void) as ::core::ffi::c_int != 0
    {
        archive_finish(stream, r#true != 0);
        return;
    }
    let mut buffers: [uv_buf_t; 2] = [uv_buf_t {
        base: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
    }; 2];
    let mut count: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if !(*stream).headers_sent {
        let c2rust_fresh0 = count;
        count = count.wrapping_add(1);
        buffers[c2rust_fresh0 as usize] = uv_buf_init(
            &raw mut (*stream).headers as *mut ::core::ffi::c_char,
            (*stream).header_length as ::core::ffi::c_uint,
        );
    }
    let c2rust_fresh1 = count;
    count = count.wrapping_add(1);
    buffers[c2rust_fresh1 as usize] = uv_buf_init(
        &raw mut (*stream).buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        (*stream).length as ::core::ffi::c_uint,
    );
    (*stream).write.data = stream as *mut ::core::ffi::c_void;
    if libuv_connection_write(
        (*stream).conn,
        &raw mut (*stream).write,
        &raw mut buffers as *mut uv_buf_t,
        count,
        Some(archive_written as unsafe extern "C" fn(*mut uv_write_t, ::core::ffi::c_int) -> ()),
    ) != 0
    {
        archive_finish(stream, r#true != 0);
        return;
    }
    (*stream).headers_sent = r#true != 0;
}
unsafe extern "C" fn archive_next(mut stream: *mut archive_stream_t) {
    if (*stream).conn.is_null()
        || archive_cancelled(stream as *mut ::core::ffi::c_void) as ::core::ffi::c_int != 0
    {
        archive_finish(stream, r#true != 0);
        return;
    }
    if (*stream).remaining == 0 {
        archive_finish(stream, r#false != 0);
        return;
    }
    (*stream).length = if (*stream).remaining < ARCHIVE_CHUNK as uint64_t {
        (*stream).remaining as size_t
    } else {
        ARCHIVE_CHUNK as size_t
    };
    (*stream).work.data = stream as *mut ::core::ffi::c_void;
    if uv_queue_work(
        (*(*(*stream).conn).server).r#loop,
        &raw mut (*stream).work,
        Some(archive_read as unsafe extern "C" fn(*mut uv_work_t) -> ()),
        Some(archive_read_done as unsafe extern "C" fn(*mut uv_work_t, ::core::ffi::c_int) -> ()),
    ) != 0
    {
        archive_finish(stream, r#true != 0);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_archive_serve(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut id: uint64_t,
    mut download: bool,
) -> bool {
    let mut transcode: [::core::ffi::c_char; 8] = [0; 8];
    let mut prepare: [::core::ffi::c_char; 8] = [0; 8];
    if !download
        && http_request_get_query_param(
            req,
            b"transcode\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut transcode as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
        ) > 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut transcode as *mut ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
    {
        return r#false != 0;
    }
    let mut preparing: bool = http_request_get_query_param(
        req,
        b"prepare\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut prepare as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
    ) > 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut prepare as *mut ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0;
    if !download
        && preparing as ::core::ffi::c_int != 0
        && http_request_get_query_param(
            req,
            b"transcode\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut transcode as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
        ) <= 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut source: storage_remote_source_t = storage_remote_source_t {
        recording_id: 0,
        target: storage_target_t {
            uuid: [0; 37],
            name: [0; 128],
            target_type: [0; 24],
            root_path: [0; 512],
            enabled: false,
            is_default: false,
            storage_class: [0; 16],
            reserve_bytes: 0,
            high_watermark_pct: 0.,
            low_watermark_pct: 0.,
            health_status: [0; 16],
            capacity_bytes: 0,
            available_bytes: 0,
            filesystem_device: 0,
            last_probe_at: 0,
            last_success_at: 0,
            last_error: [0; 256],
            revision: 0,
            created_at: 0,
            updated_at: 0,
            recording_count: 0,
            recording_bytes: 0,
            mount_required: false,
            mount_guard_path: [0; 512],
            migration_bandwidth_bps: 0,
            archival_window_start_minute: 0,
            archival_window_end_minute: 0,
            replica_count: 0,
            replica_bytes: 0,
            endpoint: [0; 512],
            region: [0; 64],
            bucket: [0; 128],
            credential_ref: [0; 128],
            archive_budget_bytes: 0,
        },
        key: [0; 512],
        size: 0,
    };
    let mut result: ::core::ffi::c_int = storage_source_remote(id, &raw mut source);
    if result == C2Rust_Unnamed_14::STORAGE_SOURCE_MISSING.0 {
        return r#false != 0;
    }
    if result != C2Rust_Unnamed_14::STORAGE_SOURCE_READY.0 {
        http_response_set_json_error(
            res,
            if result == C2Rust_Unnamed_14::STORAGE_SOURCE_DELETING.0 {
                409 as ::core::ffi::c_int
            } else {
                503 as ::core::ffi::c_int
            },
            b"Recording source is unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    if preparing {
        http_response_set_json(
            res,
            200 as ::core::ffi::c_int,
            b"{\"status\":\"ready\",\"source\":\"archive\"}\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    let mut conn: *mut libuv_connection_t = (*req).user_data as *mut libuv_connection_t;
    if conn.is_null() {
        return r#false != 0;
    }
    let mut stream: *mut archive_stream_t =
        calloc(1 as size_t, ::core::mem::size_of::<archive_stream_t>()) as *mut archive_stream_t;
    if stream.is_null() {
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Archive streaming is busy\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    (*stream).references = 1 as ::core::ffi::c_uint;
    (*stream).source = source;
    (*stream).download = download;
    *&raw mut (*stream).cancelled = r#false != 0;
    (*conn).archive_stream = stream as *mut ::core::ffi::c_void;
    if !(*conn).handler_on_worker {
        recording_archive_start(conn);
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_archive_start(
    mut conn: *mut libuv_connection_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut archive_stream_t = (*conn).archive_stream as *mut archive_stream_t;
    if stream.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    (*stream).conn = conn;
    if active_streams >= ARCHIVE_STREAMS as ::core::ffi::c_uint
        || (*stream).source.size == 0
        || (*stream).source.size > SIZE_MAX as uint64_t
    {
        (*conn).archive_stream = NULL;
        archive_free(stream);
        http_response_set_json_error(
            &raw mut (*conn).response,
            503 as ::core::ffi::c_int,
            b"Archive streaming is busy; retry later\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_add_header(
            &raw mut (*conn).response,
            b"Retry-After\0".as_ptr() as *const ::core::ffi::c_char,
            b"5\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut start: size_t = 0 as size_t;
    let mut end: size_t = ((*stream).source.size as size_t).wrapping_sub(1 as size_t);
    let mut range: *const ::core::ffi::c_char = http_request_get_header(
        &raw mut (*conn).request,
        b"Range\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut valid_range: bool = r#true != 0;
    if !range.is_null() {
        valid_range = strncmp(
            range,
            b"bytes=\0".as_ptr() as *const ::core::ffi::c_char,
            6 as size_t,
        ) == 0
            && *range.offset(6isize) as ::core::ffi::c_int != 0
            && strspn(
                range.offset(6 as ::core::ffi::c_int as isize),
                b"0123456789-\0".as_ptr() as *const ::core::ffi::c_char,
            ) as size_t
                == strlen(range.offset(6 as ::core::ffi::c_int as isize));
        let mut dash: *const ::core::ffi::c_char = strchr(range, '-' as ::core::ffi::c_int);
        valid_range = valid_range as ::core::ffi::c_int != 0
            && !dash.is_null()
            && strchr(
                dash.offset(1 as ::core::ffi::c_int as isize),
                '-' as ::core::ffi::c_int,
            )
            .is_null()
            && strcmp(
                range.offset(6 as ::core::ffi::c_int as isize),
                b"-\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0;
    }
    if !range.is_null()
        && (!valid_range
            || !libuv_parse_range_header(
                range,
                (*stream).source.size as size_t,
                &raw mut start,
                &raw mut end,
            ))
    {
        let mut value: [::core::ffi::c_char; 64] = [0; 64];
        snprintf(
            &raw mut value as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"bytes */%llu\0".as_ptr() as *const ::core::ffi::c_char,
            (*stream).source.size as ::core::ffi::c_ulonglong,
        );
        http_response_set_json_error(
            &raw mut (*conn).response,
            416 as ::core::ffi::c_int,
            b"Requested range is not satisfiable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_add_header(
            &raw mut (*conn).response,
            b"Content-Range\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut value as *mut ::core::ffi::c_char,
        );
        (*conn).archive_stream = NULL;
        archive_free(stream);
        return -1 as ::core::ffi::c_int;
    }
    (*stream).offset = start as uint64_t;
    (*stream).remaining = end.wrapping_sub(start).wrapping_add(1 as size_t) as uint64_t;
    let mut content_range: [::core::ffi::c_char; 128] = [
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
    ];
    let mut disposition: [::core::ffi::c_char; 128] = [
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
    ];
    if !range.is_null() {
        snprintf(
            &raw mut content_range as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            b"Content-Range: bytes %zu-%zu/%llu\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            start,
            end,
            (*stream).source.size as ::core::ffi::c_ulonglong,
        );
    }
    if (*stream).download {
        snprintf(
            &raw mut disposition as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            b"Content-Disposition: attachment; filename=\"recording-%llu.mp4\"\r\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*stream).source.recording_id as ::core::ffi::c_ulonglong,
        );
    }
    let mut n: ::core::ffi::c_int = snprintf(
        &raw mut (*stream).headers as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"HTTP/1.1 %s\r\nContent-Type: video/mp4\r\nContent-Length: %llu\r\nAccept-Ranges: bytes\r\nCache-Control: private, no-store\r\n%s%s\r\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if !range.is_null() {
            b"206 Partial Content\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"200 OK\0".as_ptr() as *const ::core::ffi::c_char
        },
        (*stream).remaining as ::core::ffi::c_ulonglong,
        &raw mut content_range as *mut ::core::ffi::c_char,
        &raw mut disposition as *mut ::core::ffi::c_char,
    );
    (*stream).header_length = n as size_t;
    (*stream).counted = r#true != 0;
    active_streams = active_streams.wrapping_add(1);
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < ARCHIVE_STREAMS as ::core::ffi::c_uint {
        if streams[i as usize].is_null() {
            streams[i as usize] = stream;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    (*conn).async_response_pending = r#true != 0;
    if uv_timer_init((*(*conn).server).r#loop, &raw mut (*stream).lease_timer) != 0 {
        archive_finish(stream, r#true != 0);
        return 0 as ::core::ffi::c_int;
    }
    (*stream).timer_initialized = r#true != 0;
    (*stream).references = (*stream).references.wrapping_add(1);
    (*stream).lease_timer.data = stream as *mut ::core::ffi::c_void;
    if uv_timer_start(
        &raw mut (*stream).lease_timer,
        Some(archive_renew as unsafe extern "C" fn(*mut uv_timer_t) -> ()),
        30000 as uint64_t,
        30000 as uint64_t,
    ) != 0
    {
        archive_finish(stream, r#true != 0);
        return 0 as ::core::ffi::c_int;
    }
    archive_next(stream);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recording_archive_disconnected(mut conn: *mut libuv_connection_t) {
    let mut stream: *mut archive_stream_t = (*conn).archive_stream as *mut archive_stream_t;
    if stream.is_null() {
        return;
    }
    (*conn).archive_stream = NULL;
    if !(*stream).counted {
        archive_free(stream);
        return;
    }
    (*stream).conn = ::core::ptr::null_mut::<libuv_connection_t>();
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*stream).cancelled).store(true, ::core::sync::atomic::Ordering::SeqCst);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
