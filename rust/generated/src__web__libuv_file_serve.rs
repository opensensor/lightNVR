unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn llhttp_should_keep_alive(parser: *const llhttp_t) -> ::core::ffi::c_int;
    unsafe fn uv_strerror(err: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn uv_buf_init(base: *mut ::core::ffi::c_char, len: ::core::ffi::c_uint) -> uv_buf_t;
    unsafe fn uv_is_closing(handle: *const uv_handle_t) -> ::core::ffi::c_int;
    unsafe fn uv_fs_req_cleanup(req: *mut uv_fs_t);
    unsafe fn uv_fs_close(
        r#loop: *mut uv_loop_t,
        req: *mut uv_fs_t,
        file: uv_file,
        cb: uv_fs_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_fs_open(
        r#loop: *mut uv_loop_t,
        req: *mut uv_fs_t,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        mode: ::core::ffi::c_int,
        cb: uv_fs_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_fs_read(
        r#loop: *mut uv_loop_t,
        req: *mut uv_fs_t,
        file: uv_file,
        bufs: *const uv_buf_t,
        nbufs: ::core::ffi::c_uint,
        offset: int64_t,
        cb: uv_fs_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn uv_fs_fstat(
        r#loop: *mut uv_loop_t,
        req: *mut uv_fs_t,
        file: uv_file,
        cb: uv_fs_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn safe_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn safe_free(ptr: *mut ::core::ffi::c_void);
    unsafe fn http_request_get_header(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
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
    unsafe fn libuv_connection_write(
        conn: *mut libuv_connection,
        req: *mut uv_write_t,
        bufs: *const uv_buf_t,
        nbufs: ::core::ffi::c_uint,
        cb: uv_write_cb,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_connection_reset(conn: *mut libuv_connection_t);
    unsafe fn libuv_connection_send(
        conn: *mut libuv_connection_t,
        data: *mut ::core::ffi::c_char,
        len: size_t,
        free_after_send: bool,
    ) -> ::core::ffi::c_int;
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
}
pub type size_t = usize;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __mode_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
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
pub type uv_file = ::core::ffi::c_int;
pub type uv_thread_t = pthread_t;
pub type uv_gid_t = gid_t;
pub type uv_uid_t = uid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_fs_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_10,
    pub fs_type: uv_fs_type,
    pub r#loop: *mut uv_loop_t,
    pub cb: uv_fs_cb,
    pub result: ssize_t,
    pub ptr: *mut ::core::ffi::c_void,
    pub path: *const ::core::ffi::c_char,
    pub statbuf: uv_stat_t,
    pub new_path: *const ::core::ffi::c_char,
    pub file: uv_file,
    pub flags: ::core::ffi::c_int,
    pub mode: mode_t,
    pub nbufs: ::core::ffi::c_uint,
    pub bufs: *mut uv_buf_t,
    pub off: off_t,
    pub uid: uv_uid_t,
    pub gid: uv_gid_t,
    pub atime: ::core::ffi::c_double,
    pub mtime: ::core::ffi::c_double,
    pub work_req: uv__work,
    pub bufsml: [uv_buf_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_stat_t {
    pub st_dev: uint64_t,
    pub st_mode: uint64_t,
    pub st_nlink: uint64_t,
    pub st_uid: uint64_t,
    pub st_gid: uint64_t,
    pub st_rdev: uint64_t,
    pub st_ino: uint64_t,
    pub st_size: uint64_t,
    pub st_blksize: uint64_t,
    pub st_blocks: uint64_t,
    pub st_flags: uint64_t,
    pub st_gen: uint64_t,
    pub st_atim: uv_timespec_t,
    pub st_mtim: uv_timespec_t,
    pub st_ctim: uv_timespec_t,
    pub st_birthtim: uv_timespec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_timespec_t {
    pub tv_sec: ::core::ffi::c_long,
    pub tv_nsec: ::core::ffi::c_long,
}
pub type uv_fs_cb = Option<unsafe extern "C" fn(*mut uv_fs_t) -> ()>;
pub type uv_fs_t = uv_fs_s;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct uv_fs_type(pub ::core::ffi::c_int);
impl uv_fs_type {
    pub const UV_FS_UNKNOWN: Self = Self(-1);
    pub const UV_FS_CUSTOM: Self = Self(0);
    pub const UV_FS_OPEN: Self = Self(1);
    pub const UV_FS_CLOSE: Self = Self(2);
    pub const UV_FS_READ: Self = Self(3);
    pub const UV_FS_WRITE: Self = Self(4);
    pub const UV_FS_SENDFILE: Self = Self(5);
    pub const UV_FS_STAT: Self = Self(6);
    pub const UV_FS_LSTAT: Self = Self(7);
    pub const UV_FS_FSTAT: Self = Self(8);
    pub const UV_FS_FTRUNCATE: Self = Self(9);
    pub const UV_FS_UTIME: Self = Self(10);
    pub const UV_FS_FUTIME: Self = Self(11);
    pub const UV_FS_ACCESS: Self = Self(12);
    pub const UV_FS_CHMOD: Self = Self(13);
    pub const UV_FS_FCHMOD: Self = Self(14);
    pub const UV_FS_FSYNC: Self = Self(15);
    pub const UV_FS_FDATASYNC: Self = Self(16);
    pub const UV_FS_UNLINK: Self = Self(17);
    pub const UV_FS_RMDIR: Self = Self(18);
    pub const UV_FS_MKDIR: Self = Self(19);
    pub const UV_FS_MKDTEMP: Self = Self(20);
    pub const UV_FS_RENAME: Self = Self(21);
    pub const UV_FS_SCANDIR: Self = Self(22);
    pub const UV_FS_LINK: Self = Self(23);
    pub const UV_FS_SYMLINK: Self = Self(24);
    pub const UV_FS_READLINK: Self = Self(25);
    pub const UV_FS_CHOWN: Self = Self(26);
    pub const UV_FS_FCHOWN: Self = Self(27);
    pub const UV_FS_REALPATH: Self = Self(28);
    pub const UV_FS_COPYFILE: Self = Self(29);
    pub const UV_FS_LCHOWN: Self = Self(30);
    pub const UV_FS_OPENDIR: Self = Self(31);
    pub const UV_FS_READDIR: Self = Self(32);
    pub const UV_FS_CLOSEDIR: Self = Self(33);
    pub const UV_FS_STATFS: Self = Self(34);
    pub const UV_FS_MKSTEMP: Self = Self(35);
    pub const UV_FS_LUTIME: Self = Self(36);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
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
pub type file_serve_ctx_t = file_serve_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_serve_ctx {
    pub open_req: uv_fs_t,
    pub read_req: uv_fs_t,
    pub stat_req: uv_fs_t,
    pub close_req: uv_fs_t,
    pub write_req: uv_write_t,
    pub conn: *mut libuv_connection_t,
    pub fd: uv_file,
    pub buffer: *mut ::core::ffi::c_char,
    pub buffer_size: size_t,
    pub file_size: size_t,
    pub offset: size_t,
    pub remaining: size_t,
    pub has_range: bool,
    pub range_start: size_t,
    pub range_end: size_t,
    pub headers_sent: bool,
    pub write_error: bool,
    pub content_type: [::core::ffi::c_char; 128],
    pub extra_headers: [::core::ffi::c_char; 512],
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
pub struct file_chunk_write_ctx_t {
    pub req: uv_write_t,
    pub buf: uv_buf_t,
    pub ctx: *mut file_serve_ctx_t,
    pub free_buffer: bool,
}
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UV_FS_O_RDONLY: ::core::ffi::c_int = O_RDONLY;
pub const LOG_COMPONENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"HTTP\0") };
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_get_mime_type(
    mut path: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut ext: *const ::core::ffi::c_char = strrchr(path, '.' as ::core::ffi::c_int);
    if ext.is_null() {
        return b"application/octet-stream\0".as_ptr() as *const ::core::ffi::c_char;
    }
    ext = ext.offset(1);
    if strcasecmp(ext, b"mp4\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"video/mp4\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"m4s\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"video/iso.segment\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"ts\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"video/mp2t\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"m3u8\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return b"application/vnd.apple.mpegurl\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"webm\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return b"video/webm\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"mkv\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"video/x-matroska\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"html\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return b"text/html; charset=utf-8\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"css\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"text/css; charset=utf-8\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"js\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"application/javascript; charset=utf-8\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"json\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return b"application/json; charset=utf-8\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"xml\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"application/xml; charset=utf-8\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"png\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"image/png\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"jpg\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strcasecmp(ext, b"jpeg\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"gif\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"image/gif\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"svg\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"image/svg+xml\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"ico\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"image/x-icon\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"webp\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return b"image/webp\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"woff\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return b"font/woff\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"woff2\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return b"font/woff2\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"ttf\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"font/ttf\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"txt\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"text/plain; charset=utf-8\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(ext, b"pdf\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return b"application/pdf\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return b"application/octet-stream\0".as_ptr() as *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_parse_range_header(
    mut range_header: *const ::core::ffi::c_char,
    mut file_size: size_t,
    mut start: *mut size_t,
    mut end: *mut size_t,
) -> bool {
    if range_header.is_null()
        || strncmp(
            range_header,
            b"bytes=\0".as_ptr() as *const ::core::ffi::c_char,
            6 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut range: *const ::core::ffi::c_char =
        range_header.offset(6 as ::core::ffi::c_int as isize);
    if *range as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        let mut suffix_len: size_t = strtoull(
            range.offset(1 as ::core::ffi::c_int as isize),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) as size_t;
        if suffix_len > file_size {
            suffix_len = file_size;
        }
        *start = file_size.wrapping_sub(suffix_len);
        *end = file_size.wrapping_sub(1 as size_t);
    } else {
        *start = strtoull(
            range,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) as size_t;
        let mut dash: *const ::core::ffi::c_char = strchr(range, '-' as ::core::ffi::c_int);
        if !dash.is_null()
            && *dash.offset(1isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            *end = strtoull(
                dash.offset(1 as ::core::ffi::c_int as isize),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                10 as ::core::ffi::c_int,
            ) as size_t;
        } else {
            *end = file_size.wrapping_sub(1 as size_t);
        }
    }
    if *start >= file_size {
        return r#false != 0;
    }
    if *end >= file_size {
        *end = file_size.wrapping_sub(1 as size_t);
    }
    if *start > *end {
        return r#false != 0;
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_serve_file(
    mut conn: *mut libuv_connection_t,
    mut path: *const ::core::ffi::c_char,
    mut content_type: *const ::core::ffi::c_char,
    mut extra_headers: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if conn.is_null() || path.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    (*conn).async_response_pending = r#true != 0;
    let mut ctx: *mut file_serve_ctx_t =
        safe_calloc(1 as size_t, ::core::mem::size_of::<file_serve_ctx_t>())
            as *mut file_serve_ctx_t;
    if ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_serve_file: Failed to allocate context\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        (*conn).async_response_pending = r#false != 0;
        return -1 as ::core::ffi::c_int;
    }
    (*ctx).conn = conn;
    (*ctx).fd = -1 as ::core::ffi::c_int as uv_file;
    if !content_type.is_null() {
        safe_strcpy(
            &raw mut (*ctx).content_type as *mut ::core::ffi::c_char,
            content_type,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    } else {
        safe_strcpy(
            &raw mut (*ctx).content_type as *mut ::core::ffi::c_char,
            libuv_get_mime_type(path),
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    }
    if !extra_headers.is_null() {
        safe_strcpy(
            &raw mut (*ctx).extra_headers as *mut ::core::ffi::c_char,
            extra_headers,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    let mut range_header: *const ::core::ffi::c_char = http_request_get_header(
        &raw mut (*conn).request,
        b"Range\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !range_header.is_null() {
        (*ctx).has_range = r#true != 0;
    }
    (*ctx).buffer = safe_malloc(LIBUV_FILE_BUFFER_SIZE) as *mut ::core::ffi::c_char;
    if (*ctx).buffer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_serve_file: Failed to allocate buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        safe_free(ctx as *mut ::core::ffi::c_void);
        (*conn).async_response_pending = r#false != 0;
        return -1 as ::core::ffi::c_int;
    }
    (*ctx).buffer_size = LIBUV_FILE_BUFFER_SIZE;
    (*ctx).open_req.data = ctx as *mut ::core::ffi::c_void;
    (*ctx).stat_req.data = ctx as *mut ::core::ffi::c_void;
    (*ctx).read_req.data = ctx as *mut ::core::ffi::c_void;
    (*ctx).close_req.data = ctx as *mut ::core::ffi::c_void;
    let mut r: ::core::ffi::c_int = uv_fs_open(
        (*(*conn).server).r#loop,
        &raw mut (*ctx).open_req,
        path,
        UV_FS_O_RDONLY,
        0 as ::core::ffi::c_int,
        Some(on_file_open as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
    );
    if r != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"libuv_serve_file: Failed to start open: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(r),
        );
        file_serve_cleanup(ctx);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn file_serve_cleanup(mut ctx: *mut file_serve_ctx_t) {
    if ctx.is_null() {
        return;
    }
    if !(*ctx).buffer.is_null() {
        safe_free((*ctx).buffer as *mut ::core::ffi::c_void);
    }
    uv_fs_req_cleanup(&raw mut (*ctx).open_req);
    uv_fs_req_cleanup(&raw mut (*ctx).stat_req);
    uv_fs_req_cleanup(&raw mut (*ctx).read_req);
    uv_fs_req_cleanup(&raw mut (*ctx).close_req);
    safe_free(ctx as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn on_file_open(mut req: *mut uv_fs_t) {
    let mut ctx: *mut file_serve_ctx_t = (*req).data as *mut file_serve_ctx_t;
    let mut conn: *mut libuv_connection_t = (*ctx).conn;
    if (*req).result < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_file_open: Failed to open file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror((*req).result as ::core::ffi::c_int),
        );
        http_response_set_json_error(
            &raw mut (*conn).response,
            404 as ::core::ffi::c_int,
            b"File Not Found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut action: write_complete_action_t = write_complete_action_t(
            (if (*conn).keep_alive as ::core::ffi::c_int != 0
                && llhttp_should_keep_alive(&raw mut (*conn).parser) != 0
            {
                write_complete_action_t::WRITE_ACTION_KEEP_ALIVE.0 as ::core::ffi::c_int
            } else {
                write_complete_action_t::WRITE_ACTION_CLOSE.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        libuv_send_response_ex(conn, &raw mut (*conn).response, action);
        file_serve_cleanup(ctx);
        (*conn).async_response_pending = r#false != 0;
        return;
    }
    (*ctx).fd = (*req).result as uv_file;
    uv_fs_req_cleanup(req);
    let mut r: ::core::ffi::c_int = uv_fs_fstat(
        (*(*(*ctx).conn).server).r#loop,
        &raw mut (*ctx).stat_req,
        (*ctx).fd,
        Some(on_file_stat as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
    );
    if r != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_file_open: Failed to start stat: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(r),
        );
        uv_fs_close(
            (*(*(*ctx).conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
        return;
    }
}
unsafe extern "C" fn on_file_stat(mut req: *mut uv_fs_t) {
    let mut ctx: *mut file_serve_ctx_t = (*req).data as *mut file_serve_ctx_t;
    let mut conn: *mut libuv_connection_t = (*ctx).conn;
    if (*req).result < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_file_stat: Failed to stat file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror((*req).result as ::core::ffi::c_int),
        );
        http_response_set_json_error(
            &raw mut (*conn).response,
            500 as ::core::ffi::c_int,
            b"Failed to stat file\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut action: write_complete_action_t = write_complete_action_t(
            (if (*conn).keep_alive as ::core::ffi::c_int != 0
                && llhttp_should_keep_alive(&raw mut (*conn).parser) != 0
            {
                write_complete_action_t::WRITE_ACTION_KEEP_ALIVE.0 as ::core::ffi::c_int
            } else {
                write_complete_action_t::WRITE_ACTION_CLOSE.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        libuv_send_response_ex(conn, &raw mut (*conn).response, action);
        (*conn).async_response_pending = r#false != 0;
        uv_fs_close(
            (*(*conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
        return;
    }
    (*ctx).file_size = (*req).statbuf.st_size as size_t;
    uv_fs_req_cleanup(req);
    if (*ctx).has_range {
        let mut range_header: *const ::core::ffi::c_char = http_request_get_header(
            &raw mut (*conn).request,
            b"Range\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !libuv_parse_range_header(
            range_header,
            (*ctx).file_size,
            &raw mut (*ctx).range_start,
            &raw mut (*ctx).range_end,
        ) {
            http_response_set_json_error(
                &raw mut (*conn).response,
                416 as ::core::ffi::c_int,
                b"Requested Range Not Satisfiable\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut action_0: write_complete_action_t = write_complete_action_t(
                (if (*conn).keep_alive as ::core::ffi::c_int != 0
                    && llhttp_should_keep_alive(&raw mut (*conn).parser) != 0
                {
                    write_complete_action_t::WRITE_ACTION_KEEP_ALIVE.0 as ::core::ffi::c_int
                } else {
                    write_complete_action_t::WRITE_ACTION_CLOSE.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            );
            libuv_send_response_ex(conn, &raw mut (*conn).response, action_0);
            (*conn).async_response_pending = r#false != 0;
            uv_fs_close(
                (*(*conn).server).r#loop,
                &raw mut (*ctx).close_req,
                (*ctx).fd,
                Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
            );
            return;
        }
        (*ctx).offset = (*ctx).range_start;
        (*ctx).remaining = (*ctx)
            .range_end
            .wrapping_sub((*ctx).range_start)
            .wrapping_add(1 as size_t);
    } else {
        (*ctx).offset = 0 as size_t;
        (*ctx).remaining = (*ctx).file_size;
        (*ctx).range_start = 0 as size_t;
        (*ctx).range_end = (*ctx).file_size.wrapping_sub(1 as size_t);
    }
    let mut headers: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut len: ::core::ffi::c_int = 0;
    if (*ctx).has_range {
        len = snprintf(
            &raw mut headers as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"HTTP/1.1 206 Partial Content\r\nContent-Type: %s\r\nContent-Length: %zu\r\nContent-Range: bytes %zu-%zu/%zu\r\nAccept-Ranges: bytes\r\n%s\r\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).content_type as *mut ::core::ffi::c_char,
            (*ctx).remaining,
            (*ctx).range_start,
            (*ctx).range_end,
            (*ctx).file_size,
            if (*ctx).extra_headers[0usize] as ::core::ffi::c_int != 0 {
                &raw mut (*ctx).extra_headers as *mut ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    } else {
        len = snprintf(
            &raw mut headers as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
            b"HTTP/1.1 200 OK\r\nContent-Type: %s\r\nContent-Length: %zu\r\nAccept-Ranges: bytes\r\n%s\r\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).content_type as *mut ::core::ffi::c_char,
            (*ctx).file_size,
            if (*ctx).extra_headers[0usize] as ::core::ffi::c_int != 0 {
                &raw mut (*ctx).extra_headers as *mut ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut header_buf: *mut ::core::ffi::c_char =
        safe_malloc(len as size_t) as *mut ::core::ffi::c_char;
    if header_buf.is_null() {
        uv_fs_close(
            (*(*(*ctx).conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
        return;
    }
    memcpy(
        header_buf as *mut ::core::ffi::c_void,
        &raw mut headers as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
    );
    if libuv_connection_send((*ctx).conn, header_buf, len as size_t, r#true != 0)
        != 0 as ::core::ffi::c_int
    {
        (*conn).async_response_pending = r#false != 0;
        uv_fs_close(
            (*(*conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
        return;
    }
    (*ctx).headers_sent = r#true != 0;
    send_file_chunk(ctx);
}
unsafe extern "C" fn send_file_chunk(mut ctx: *mut file_serve_ctx_t) {
    if (*ctx).remaining == 0 as size_t {
        uv_fs_close(
            (*(*(*ctx).conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
        return;
    }
    let mut to_read: size_t = if (*ctx).remaining < (*ctx).buffer_size {
        (*ctx).remaining
    } else {
        (*ctx).buffer_size
    };
    let mut buf: uv_buf_t = uv_buf_init((*ctx).buffer, to_read as ::core::ffi::c_uint);
    let mut r: ::core::ffi::c_int = uv_fs_read(
        (*(*(*ctx).conn).server).r#loop,
        &raw mut (*ctx).read_req,
        (*ctx).fd,
        &raw mut buf,
        1 as ::core::ffi::c_uint,
        (*ctx).offset as int64_t,
        Some(on_file_read as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
    );
    if r != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"send_file_chunk: Failed to start read: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(r),
        );
        uv_fs_close(
            (*(*(*ctx).conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
    }
}
unsafe extern "C" fn on_chunk_write_complete(
    mut req: *mut uv_write_t,
    mut status: ::core::ffi::c_int,
) {
    let mut write_ctx: *mut file_chunk_write_ctx_t = req as *mut file_chunk_write_ctx_t;
    let mut ctx: *mut file_serve_ctx_t = (*write_ctx).ctx;
    if (*write_ctx).free_buffer as ::core::ffi::c_int != 0 && !(*write_ctx).buf.base.is_null() {
        safe_free((*write_ctx).buf.base as *mut ::core::ffi::c_void);
    }
    safe_free(write_ctx as *mut ::core::ffi::c_void);
    if status < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_chunk_write_complete: Write error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(status),
        );
        (*ctx).write_error = r#true != 0;
        uv_fs_close(
            (*(*(*ctx).conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
        return;
    }
    send_file_chunk(ctx);
}
unsafe extern "C" fn on_file_read(mut req: *mut uv_fs_t) {
    let mut ctx: *mut file_serve_ctx_t = (*req).data as *mut file_serve_ctx_t;
    if (*req).result < 0 as ssize_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_file_read: Read error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror((*req).result as ::core::ffi::c_int),
        );
        uv_fs_close(
            (*(*(*ctx).conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
        return;
    }
    if (*req).result == 0 as ssize_t {
        uv_fs_close(
            (*(*(*ctx).conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
        return;
    }
    let mut bytes_read: size_t = (*req).result as size_t;
    uv_fs_req_cleanup(req);
    let mut send_buf: *mut ::core::ffi::c_char =
        safe_malloc(bytes_read) as *mut ::core::ffi::c_char;
    if send_buf.is_null() {
        uv_fs_close(
            (*(*(*ctx).conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
        return;
    }
    memcpy(
        send_buf as *mut ::core::ffi::c_void,
        (*ctx).buffer as *const ::core::ffi::c_void,
        bytes_read,
    );
    (*ctx).offset = (*ctx).offset.wrapping_add(bytes_read);
    (*ctx).remaining = (*ctx).remaining.wrapping_sub(bytes_read);
    let mut write_ctx: *mut file_chunk_write_ctx_t = safe_malloc(::core::mem::size_of::<
        file_chunk_write_ctx_t,
    >()) as *mut file_chunk_write_ctx_t;
    if write_ctx.is_null() {
        safe_free(send_buf as *mut ::core::ffi::c_void);
        uv_fs_close(
            (*(*(*ctx).conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
        return;
    }
    (*write_ctx).ctx = ctx;
    (*write_ctx).buf = uv_buf_init(send_buf, bytes_read as ::core::ffi::c_uint);
    (*write_ctx).free_buffer = r#true != 0;
    if uv_is_closing(&raw mut (*(*ctx).conn).handle as *mut uv_handle_t) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_file_read: Connection is closing, aborting file send\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        safe_free(send_buf as *mut ::core::ffi::c_void);
        safe_free(write_ctx as *mut ::core::ffi::c_void);
        uv_fs_close(
            (*(*(*ctx).conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
        return;
    }
    let mut r: ::core::ffi::c_int = libuv_connection_write(
        (*ctx).conn,
        &raw mut (*write_ctx).req,
        &raw mut (*write_ctx).buf,
        1 as ::core::ffi::c_uint,
        Some(
            on_chunk_write_complete
                as unsafe extern "C" fn(*mut uv_write_t, ::core::ffi::c_int) -> (),
        ),
    );
    if r != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_file_read: Write failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            uv_strerror(r),
        );
        safe_free(send_buf as *mut ::core::ffi::c_void);
        safe_free(write_ctx as *mut ::core::ffi::c_void);
        uv_fs_close(
            (*(*(*ctx).conn).server).r#loop,
            &raw mut (*ctx).close_req,
            (*ctx).fd,
            Some(on_file_close as unsafe extern "C" fn(*mut uv_fs_t) -> ()),
        );
    }
}
unsafe extern "C" fn on_file_close(mut req: *mut uv_fs_t) {
    let mut ctx: *mut file_serve_ctx_t = (*req).data as *mut file_serve_ctx_t;
    let mut conn: *mut libuv_connection_t = (*ctx).conn;
    let mut should_manage_connection: bool = (*conn).async_response_pending;
    let mut write_error: bool = (*ctx).write_error;
    file_serve_cleanup(ctx);
    (*conn).async_response_pending = r#false != 0;
    if !should_manage_connection {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_file_close: Connection already managed by error handler\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ::core::ptr::read_volatile::<bool>(&raw const (*(*conn).server).shutting_down) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"on_file_close: Server shutting down, skipping connection management\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if uv_is_closing(&raw mut (*conn).handle as *mut uv_handle_t) == 0 {
        if !write_error
            && (*conn).keep_alive as ::core::ffi::c_int != 0
            && llhttp_should_keep_alive(&raw mut (*conn).parser) != 0
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"on_file_close: Keeping connection alive for reuse\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            libuv_connection_reset(conn);
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"on_file_close: Closing connection (keep_alive=%d, write_error=%d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*conn).keep_alive as ::core::ffi::c_int,
                write_error as ::core::ffi::c_int,
            );
            libuv_connection_close(conn);
        }
    }
}
pub const LIBUV_FILE_BUFFER_SIZE: size_t =
    (64 as ::core::ffi::c_int as size_t).wrapping_mul(1024 as size_t);
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
