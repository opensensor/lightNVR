unsafe extern "C" {
    pub type mbedtls_md_info_t;
    pub type mbedtls_pk_info_t;
    pub type mbedtls_ssl_transform;
    pub type mbedtls_ssl_handshake_params;
    pub type mbedtls_ssl_key_cert;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
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
    unsafe fn uv_is_closing(handle: *const uv_handle_t) -> ::core::ffi::c_int;
    unsafe fn uv_idle_init(_: *mut uv_loop_t, idle: *mut uv_idle_t) -> ::core::ffi::c_int;
    unsafe fn uv_idle_start(idle: *mut uv_idle_t, cb: uv_idle_cb) -> ::core::ffi::c_int;
    unsafe fn uv_idle_stop(idle: *mut uv_idle_t) -> ::core::ffi::c_int;
    unsafe fn safe_realloc(ptr: *mut ::core::ffi::c_void, size: size_t)
    -> *mut ::core::ffi::c_void;
    unsafe fn safe_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn safe_free(ptr: *mut ::core::ffi::c_void);
    unsafe fn libuv_connection_close(conn: *mut libuv_connection_t);
    unsafe fn mbedtls_ssl_init(ssl: *mut mbedtls_ssl_context);
    unsafe fn mbedtls_ssl_setup(
        ssl: *mut mbedtls_ssl_context,
        conf: *const mbedtls_ssl_config,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ssl_conf_rng(
        conf: *mut mbedtls_ssl_config,
        f_rng: Option<mbedtls_f_rng_t>,
        p_rng: *mut ::core::ffi::c_void,
    );
    unsafe fn mbedtls_ssl_set_bio(
        ssl: *mut mbedtls_ssl_context,
        p_bio: *mut ::core::ffi::c_void,
        f_send: Option<mbedtls_ssl_send_t>,
        f_recv: Option<mbedtls_ssl_recv_t>,
        f_recv_timeout: Option<mbedtls_ssl_recv_timeout_t>,
    );
    unsafe fn mbedtls_ssl_conf_own_cert(
        conf: *mut mbedtls_ssl_config,
        own_cert: *mut mbedtls_x509_crt,
        pk_key: *mut mbedtls_pk_context,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ssl_conf_alpn_protocols(
        conf: *mut mbedtls_ssl_config,
        protos: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ssl_get_bytes_avail(ssl: *const mbedtls_ssl_context) -> size_t;
    unsafe fn mbedtls_ssl_handshake(ssl: *mut mbedtls_ssl_context) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ssl_read(
        ssl: *mut mbedtls_ssl_context,
        buf: *mut ::core::ffi::c_uchar,
        len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ssl_write(
        ssl: *mut mbedtls_ssl_context,
        buf: *const ::core::ffi::c_uchar,
        len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ssl_free(ssl: *mut mbedtls_ssl_context);
    unsafe fn mbedtls_ssl_config_init(conf: *mut mbedtls_ssl_config);
    unsafe fn mbedtls_ssl_config_defaults(
        conf: *mut mbedtls_ssl_config,
        endpoint: ::core::ffi::c_int,
        transport: ::core::ffi::c_int,
        preset: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ssl_config_free(conf: *mut mbedtls_ssl_config);
    unsafe fn mbedtls_x509_crt_parse_file(
        chain: *mut mbedtls_x509_crt,
        path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_x509_crt_init(crt: *mut mbedtls_x509_crt);
    unsafe fn mbedtls_x509_crt_free(crt: *mut mbedtls_x509_crt);
    unsafe fn mbedtls_pk_init(ctx: *mut mbedtls_pk_context);
    unsafe fn mbedtls_pk_free(ctx: *mut mbedtls_pk_context);
    unsafe fn mbedtls_pk_check_pair(
        r#pub: *const mbedtls_pk_context,
        prv: *const mbedtls_pk_context,
        f_rng: Option<mbedtls_f_rng_t>,
        p_rng: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_pk_parse_keyfile(
        ctx: *mut mbedtls_pk_context,
        path: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        f_rng: Option<mbedtls_f_rng_t>,
        p_rng: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_entropy_init(ctx: *mut mbedtls_entropy_context);
    unsafe fn mbedtls_entropy_free(ctx: *mut mbedtls_entropy_context);
    unsafe fn mbedtls_entropy_func(
        data: *mut ::core::ffi::c_void,
        output: *mut ::core::ffi::c_uchar,
        len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ctr_drbg_init(ctx: *mut mbedtls_ctr_drbg_context);
    unsafe fn mbedtls_ctr_drbg_seed(
        ctx: *mut mbedtls_ctr_drbg_context,
        f_entropy: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_uchar,
                size_t,
            ) -> ::core::ffi::c_int,
        >,
        p_entropy: *mut ::core::ffi::c_void,
        custom: *const ::core::ffi::c_uchar,
        len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ctr_drbg_free(ctx: *mut mbedtls_ctr_drbg_context);
    unsafe fn mbedtls_ctr_drbg_random(
        p_rng: *mut ::core::ffi::c_void,
        output: *mut ::core::ffi::c_uchar,
        output_len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn mbedtls_strerror(
        errnum: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_char,
        buflen: size_t,
    );
    unsafe fn psa_crypto_init() -> psa_status_t;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type time_t = __time_t;
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
pub struct uv_idle_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_10,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub idle_cb: uv_idle_cb,
    pub queue: uv__queue,
}
pub type uv_idle_cb = Option<unsafe extern "C" fn(*mut uv_idle_t) -> ()>;
pub type uv_idle_t = uv_idle_s;
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
    pub handlers: *mut C2Rust_Unnamed_12,
    pub handler_count: ::core::ffi::c_int,
    pub handler_capacity: ::core::ffi::c_int,
    pub tls_ctx: *mut ::core::ffi::c_void,
    pub thread: uv_thread_t,
    pub thread_running: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
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
pub struct libuv_tls_context {
    pub conf: mbedtls_ssl_config,
    pub cert: mbedtls_x509_crt,
    pub key: mbedtls_pk_context,
    pub entropy: mbedtls_entropy_context,
    pub drbg: mbedtls_ctr_drbg_context,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_ctr_drbg_context {
    pub private_counter: [::core::ffi::c_uchar; 16],
    pub private_reseed_counter: ::core::ffi::c_int,
    pub private_prediction_resistance: ::core::ffi::c_int,
    pub private_entropy_len: size_t,
    pub private_reseed_interval: ::core::ffi::c_int,
    pub private_aes_ctx: mbedtls_aes_context,
    pub private_f_entropy: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_p_entropy: *mut ::core::ffi::c_void,
    pub private_mutex: mbedtls_threading_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_threading_mutex_t {
    pub private_mutex: pthread_mutex_t,
    pub private_state: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_aes_context {
    pub private_nr: ::core::ffi::c_int,
    pub private_rk_offset: size_t,
    pub private_buf: [uint32_t; 68],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_entropy_context {
    pub private_accumulator: mbedtls_md_context_t,
    pub private_accumulator_started: ::core::ffi::c_int,
    pub private_source_count: ::core::ffi::c_int,
    pub private_source: [mbedtls_entropy_source_state; 20],
    pub private_mutex: mbedtls_threading_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_entropy_source_state {
    pub private_f_source: mbedtls_entropy_f_source_ptr,
    pub private_p_source: *mut ::core::ffi::c_void,
    pub private_size: size_t,
    pub private_threshold: size_t,
    pub private_strong: ::core::ffi::c_int,
}
pub type mbedtls_entropy_f_source_ptr = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_uchar,
        size_t,
        *mut size_t,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_md_context_t {
    pub private_md_info: *const mbedtls_md_info_t,
    pub private_md_ctx: *mut ::core::ffi::c_void,
    pub private_hmac_ctx: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_pk_context {
    pub private_pk_info: *const mbedtls_pk_info_t,
    pub private_pk_ctx: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_x509_crt {
    pub private_own_buffer: ::core::ffi::c_int,
    pub raw: mbedtls_x509_buf,
    pub tbs: mbedtls_x509_buf,
    pub version: ::core::ffi::c_int,
    pub serial: mbedtls_x509_buf,
    pub sig_oid: mbedtls_x509_buf,
    pub issuer_raw: mbedtls_x509_buf,
    pub subject_raw: mbedtls_x509_buf,
    pub issuer: mbedtls_x509_name,
    pub subject: mbedtls_x509_name,
    pub valid_from: mbedtls_x509_time,
    pub valid_to: mbedtls_x509_time,
    pub pk_raw: mbedtls_x509_buf,
    pub pk: mbedtls_pk_context,
    pub issuer_id: mbedtls_x509_buf,
    pub subject_id: mbedtls_x509_buf,
    pub v3_ext: mbedtls_x509_buf,
    pub subject_alt_names: mbedtls_x509_sequence,
    pub subject_key_id: mbedtls_x509_buf,
    pub authority_key_id: mbedtls_x509_authority,
    pub certificate_policies: mbedtls_x509_sequence,
    pub private_ext_types: ::core::ffi::c_int,
    pub private_ca_istrue: ::core::ffi::c_int,
    pub private_max_pathlen: ::core::ffi::c_int,
    pub private_key_usage: ::core::ffi::c_uint,
    pub ext_key_usage: mbedtls_x509_sequence,
    pub private_ns_cert_type: ::core::ffi::c_uchar,
    pub private_sig: mbedtls_x509_buf,
    pub private_sig_md: mbedtls_md_type_t,
    pub private_sig_pk: mbedtls_pk_type_t,
    pub private_sig_opts: *mut ::core::ffi::c_void,
    pub next: *mut mbedtls_x509_crt,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mbedtls_pk_type_t(pub ::core::ffi::c_uint);
impl mbedtls_pk_type_t {
    pub const MBEDTLS_PK_NONE: Self = Self(0);
    pub const MBEDTLS_PK_RSA: Self = Self(1);
    pub const MBEDTLS_PK_ECKEY: Self = Self(2);
    pub const MBEDTLS_PK_ECKEY_DH: Self = Self(3);
    pub const MBEDTLS_PK_ECDSA: Self = Self(4);
    pub const MBEDTLS_PK_RSA_ALT: Self = Self(5);
    pub const MBEDTLS_PK_RSASSA_PSS: Self = Self(6);
    pub const MBEDTLS_PK_OPAQUE: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mbedtls_md_type_t(pub ::core::ffi::c_uint);
impl mbedtls_md_type_t {
    pub const MBEDTLS_MD_NONE: Self = Self(0);
    pub const MBEDTLS_MD_MD5: Self = Self(3);
    pub const MBEDTLS_MD_RIPEMD160: Self = Self(4);
    pub const MBEDTLS_MD_SHA1: Self = Self(5);
    pub const MBEDTLS_MD_SHA224: Self = Self(8);
    pub const MBEDTLS_MD_SHA256: Self = Self(9);
    pub const MBEDTLS_MD_SHA384: Self = Self(10);
    pub const MBEDTLS_MD_SHA512: Self = Self(11);
    pub const MBEDTLS_MD_SHA3_224: Self = Self(16);
    pub const MBEDTLS_MD_SHA3_256: Self = Self(17);
    pub const MBEDTLS_MD_SHA3_384: Self = Self(18);
    pub const MBEDTLS_MD_SHA3_512: Self = Self(19);
}
pub type mbedtls_x509_buf = mbedtls_asn1_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_asn1_buf {
    pub tag: ::core::ffi::c_int,
    pub len: size_t,
    pub p: *mut ::core::ffi::c_uchar,
}
pub type mbedtls_x509_sequence = mbedtls_asn1_sequence;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_asn1_sequence {
    pub buf: mbedtls_asn1_buf,
    pub next: *mut mbedtls_asn1_sequence,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_x509_authority {
    pub keyIdentifier: mbedtls_x509_buf,
    pub authorityCertIssuer: mbedtls_x509_sequence,
    pub authorityCertSerialNumber: mbedtls_x509_buf,
    pub raw: mbedtls_x509_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_x509_time {
    pub year: ::core::ffi::c_int,
    pub mon: ::core::ffi::c_int,
    pub day: ::core::ffi::c_int,
    pub hour: ::core::ffi::c_int,
    pub min: ::core::ffi::c_int,
    pub sec: ::core::ffi::c_int,
}
pub type mbedtls_x509_name = mbedtls_asn1_named_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_asn1_named_data {
    pub oid: mbedtls_asn1_buf,
    pub val: mbedtls_asn1_buf,
    pub next: *mut mbedtls_asn1_named_data,
    pub private_next_merged: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_ssl_config {
    pub private_max_tls_version: mbedtls_ssl_protocol_version,
    pub private_min_tls_version: mbedtls_ssl_protocol_version,
    pub private_endpoint: uint8_t,
    pub private_transport: uint8_t,
    pub private_authmode: uint8_t,
    pub private_allow_legacy_renegotiation: uint8_t,
    pub private_mfl_code: uint8_t,
    pub private_encrypt_then_mac: uint8_t,
    pub private_extended_ms: uint8_t,
    pub private_anti_replay: uint8_t,
    pub private_disable_renegotiation: uint8_t,
    pub private_session_tickets: uint8_t,
    pub private_new_session_tickets_count: uint16_t,
    pub private_cert_req_ca_list: uint8_t,
    pub private_respect_cli_pref: uint8_t,
    pub private_ignore_unexpected_cid: uint8_t,
    pub private_dtls_srtp_mki_support: uint8_t,
    pub private_ciphersuite_list: *const ::core::ffi::c_int,
    pub private_tls13_kex_modes: ::core::ffi::c_int,
    pub private_f_dbg: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub private_p_dbg: *mut ::core::ffi::c_void,
    pub private_f_rng: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_p_rng: *mut ::core::ffi::c_void,
    pub private_f_get_cache: Option<mbedtls_ssl_cache_get_t>,
    pub private_f_set_cache: Option<mbedtls_ssl_cache_set_t>,
    pub private_p_cache: *mut ::core::ffi::c_void,
    pub private_f_sni: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut mbedtls_ssl_context,
            *const ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_p_sni: *mut ::core::ffi::c_void,
    pub private_f_vrfy: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut mbedtls_x509_crt,
            ::core::ffi::c_int,
            *mut uint32_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_p_vrfy: *mut ::core::ffi::c_void,
    pub private_f_psk: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut mbedtls_ssl_context,
            *const ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_p_psk: *mut ::core::ffi::c_void,
    pub private_f_cookie_write: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_uchar,
            *mut ::core::ffi::c_uchar,
            *const ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_f_cookie_check: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_uchar,
            size_t,
            *const ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_p_cookie: *mut ::core::ffi::c_void,
    pub private_f_ticket_write: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const mbedtls_ssl_session,
            *mut ::core::ffi::c_uchar,
            *const ::core::ffi::c_uchar,
            *mut size_t,
            *mut uint32_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_f_ticket_parse: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut mbedtls_ssl_session,
            *mut ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_p_ticket: *mut ::core::ffi::c_void,
    pub private_cid_len: size_t,
    pub private_cert_profile: *const mbedtls_x509_crt_profile,
    pub private_key_cert: *mut mbedtls_ssl_key_cert,
    pub private_ca_chain: *mut mbedtls_x509_crt,
    pub private_ca_crl: *mut mbedtls_x509_crl,
    pub private_sig_hashes: *const ::core::ffi::c_int,
    pub private_sig_algs: *const uint16_t,
    pub private_curve_list: *const mbedtls_ecp_group_id,
    pub private_group_list: *const uint16_t,
    pub private_dhm_P: mbedtls_mpi,
    pub private_dhm_G: mbedtls_mpi,
    pub private_psk: *mut ::core::ffi::c_uchar,
    pub private_psk_len: size_t,
    pub private_psk_identity: *mut ::core::ffi::c_uchar,
    pub private_psk_identity_len: size_t,
    pub private_alpn_list: *mut *const ::core::ffi::c_char,
    pub private_dtls_srtp_profile_list: *const mbedtls_ssl_srtp_profile,
    pub private_dtls_srtp_profile_list_len: size_t,
    pub private_read_timeout: uint32_t,
    pub private_hs_timeout_min: uint32_t,
    pub private_hs_timeout_max: uint32_t,
    pub private_renego_max_records: ::core::ffi::c_int,
    pub private_renego_period: [::core::ffi::c_uchar; 8],
    pub private_badmac_limit: ::core::ffi::c_uint,
    pub private_dhm_min_bitlen: ::core::ffi::c_uint,
    pub private_user_data: mbedtls_ssl_user_data_t,
    pub private_f_cert_cb: mbedtls_ssl_hs_cb_t,
    pub private_dn_hints: *const mbedtls_x509_crt,
}
pub type mbedtls_ssl_hs_cb_t =
    Option<unsafe extern "C" fn(*mut mbedtls_ssl_context) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_ssl_context {
    pub private_conf: *const mbedtls_ssl_config,
    pub private_state: ::core::ffi::c_int,
    pub private_renego_status: ::core::ffi::c_int,
    pub private_renego_records_seen: ::core::ffi::c_int,
    pub private_tls_version: mbedtls_ssl_protocol_version,
    pub private_badmac_seen_or_in_hsfraglen: ::core::ffi::c_uint,
    pub private_f_vrfy: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut mbedtls_x509_crt,
            ::core::ffi::c_int,
            *mut uint32_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_p_vrfy: *mut ::core::ffi::c_void,
    pub private_f_send: Option<mbedtls_ssl_send_t>,
    pub private_f_recv: Option<mbedtls_ssl_recv_t>,
    pub private_f_recv_timeout: Option<mbedtls_ssl_recv_timeout_t>,
    pub private_p_bio: *mut ::core::ffi::c_void,
    pub private_session_in: *mut mbedtls_ssl_session,
    pub private_session_out: *mut mbedtls_ssl_session,
    pub private_session: *mut mbedtls_ssl_session,
    pub private_session_negotiate: *mut mbedtls_ssl_session,
    pub private_handshake: *mut mbedtls_ssl_handshake_params,
    pub private_transform_in: *mut mbedtls_ssl_transform,
    pub private_transform_out: *mut mbedtls_ssl_transform,
    pub private_transform: *mut mbedtls_ssl_transform,
    pub private_transform_negotiate: *mut mbedtls_ssl_transform,
    pub private_transform_application: *mut mbedtls_ssl_transform,
    pub private_p_timer: *mut ::core::ffi::c_void,
    pub private_f_set_timer: Option<mbedtls_ssl_set_timer_t>,
    pub private_f_get_timer: Option<mbedtls_ssl_get_timer_t>,
    pub private_in_buf: *mut ::core::ffi::c_uchar,
    pub private_in_ctr: *mut ::core::ffi::c_uchar,
    pub private_in_hdr: *mut ::core::ffi::c_uchar,
    pub private_in_cid: *mut ::core::ffi::c_uchar,
    pub private_in_len: *mut ::core::ffi::c_uchar,
    pub private_in_iv: *mut ::core::ffi::c_uchar,
    pub private_in_msg: *mut ::core::ffi::c_uchar,
    pub private_in_offt: *mut ::core::ffi::c_uchar,
    pub private_in_msgtype: ::core::ffi::c_int,
    pub private_in_msglen: size_t,
    pub private_in_left: size_t,
    pub private_in_epoch: uint16_t,
    pub private_next_record_offset: size_t,
    pub private_in_window_top: uint64_t,
    pub private_in_window: uint64_t,
    pub private_in_hslen: size_t,
    pub private_nb_zero: ::core::ffi::c_int,
    pub private_keep_current_message: ::core::ffi::c_int,
    pub private_send_alert: ::core::ffi::c_uchar,
    pub private_alert_type: ::core::ffi::c_uchar,
    pub private_alert_reason: ::core::ffi::c_int,
    pub private_disable_datagram_packing: uint8_t,
    pub private_out_buf: *mut ::core::ffi::c_uchar,
    pub private_out_ctr: *mut ::core::ffi::c_uchar,
    pub private_out_hdr: *mut ::core::ffi::c_uchar,
    pub private_out_cid: *mut ::core::ffi::c_uchar,
    pub private_out_len: *mut ::core::ffi::c_uchar,
    pub private_out_iv: *mut ::core::ffi::c_uchar,
    pub private_out_msg: *mut ::core::ffi::c_uchar,
    pub private_out_msgtype: ::core::ffi::c_int,
    pub private_out_msglen: size_t,
    pub private_out_left: size_t,
    pub private_cur_out_ctr: [::core::ffi::c_uchar; 8],
    pub private_mtu: uint16_t,
    pub private_hostname: *mut ::core::ffi::c_char,
    pub private_alpn_chosen: *const ::core::ffi::c_char,
    pub private_dtls_srtp_info: mbedtls_dtls_srtp_info,
    pub private_cli_id: *mut ::core::ffi::c_uchar,
    pub private_cli_id_len: size_t,
    pub private_secure_renegotiation: ::core::ffi::c_int,
    pub private_verify_data_len: size_t,
    pub private_own_verify_data: [::core::ffi::c_char; 12],
    pub private_peer_verify_data: [::core::ffi::c_char; 12],
    pub private_own_cid: [::core::ffi::c_uchar; 32],
    pub private_own_cid_len: uint8_t,
    pub private_negotiate_cid: uint8_t,
    pub private_f_export_keys: Option<mbedtls_ssl_export_keys_t>,
    pub private_p_export_keys: *mut ::core::ffi::c_void,
    pub private_user_data: mbedtls_ssl_user_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mbedtls_ssl_user_data_t {
    pub n: uintptr_t,
    pub p: *mut ::core::ffi::c_void,
}
pub type mbedtls_ssl_export_keys_t = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    mbedtls_ssl_key_export_type,
    *const ::core::ffi::c_uchar,
    size_t,
    *const ::core::ffi::c_uchar,
    *const ::core::ffi::c_uchar,
    mbedtls_tls_prf_types,
) -> ();
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mbedtls_tls_prf_types(pub ::core::ffi::c_uint);
impl mbedtls_tls_prf_types {
    pub const MBEDTLS_SSL_TLS_PRF_NONE: Self = Self(0);
    pub const MBEDTLS_SSL_TLS_PRF_SHA384: Self = Self(1);
    pub const MBEDTLS_SSL_TLS_PRF_SHA256: Self = Self(2);
    pub const MBEDTLS_SSL_HKDF_EXPAND_SHA384: Self = Self(3);
    pub const MBEDTLS_SSL_HKDF_EXPAND_SHA256: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mbedtls_ssl_key_export_type(pub ::core::ffi::c_uint);
impl mbedtls_ssl_key_export_type {
    pub const MBEDTLS_SSL_KEY_EXPORT_TLS12_MASTER_SECRET: Self = Self(0);
    pub const MBEDTLS_SSL_KEY_EXPORT_TLS1_3_CLIENT_EARLY_SECRET: Self = Self(1);
    pub const MBEDTLS_SSL_KEY_EXPORT_TLS1_3_EARLY_EXPORTER_SECRET: Self = Self(2);
    pub const MBEDTLS_SSL_KEY_EXPORT_TLS1_3_CLIENT_HANDSHAKE_TRAFFIC_SECRET: Self = Self(3);
    pub const MBEDTLS_SSL_KEY_EXPORT_TLS1_3_SERVER_HANDSHAKE_TRAFFIC_SECRET: Self = Self(4);
    pub const MBEDTLS_SSL_KEY_EXPORT_TLS1_3_CLIENT_APPLICATION_TRAFFIC_SECRET: Self = Self(5);
    pub const MBEDTLS_SSL_KEY_EXPORT_TLS1_3_SERVER_APPLICATION_TRAFFIC_SECRET: Self = Self(6);
}
pub type mbedtls_dtls_srtp_info = mbedtls_dtls_srtp_info_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_dtls_srtp_info_t {
    pub private_chosen_dtls_srtp_profile: mbedtls_ssl_srtp_profile,
    pub private_mki_len: uint16_t,
    pub private_mki_value: [::core::ffi::c_uchar; 255],
}
pub type mbedtls_ssl_srtp_profile = uint16_t;
pub type mbedtls_ssl_get_timer_t =
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int;
pub type mbedtls_ssl_set_timer_t =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t, uint32_t) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_ssl_session {
    pub private_mfl_code: ::core::ffi::c_uchar,
    pub private_exported: ::core::ffi::c_uchar,
    pub private_endpoint: uint8_t,
    pub private_tls_version: mbedtls_ssl_protocol_version,
    pub private_start: mbedtls_time_t,
    pub private_ciphersuite: ::core::ffi::c_int,
    pub private_id_len: size_t,
    pub private_id: [::core::ffi::c_uchar; 32],
    pub private_master: [::core::ffi::c_uchar; 48],
    pub private_peer_cert: *mut mbedtls_x509_crt,
    pub private_verify_result: uint32_t,
    pub private_ticket: *mut ::core::ffi::c_uchar,
    pub private_ticket_len: size_t,
    pub private_ticket_lifetime: uint32_t,
    pub private_ticket_creation_time: mbedtls_ms_time_t,
    pub private_ticket_age_add: uint32_t,
    pub private_ticket_flags: uint8_t,
    pub private_resumption_key_len: uint8_t,
    pub private_resumption_key: [::core::ffi::c_uchar; 48],
    pub private_hostname: *mut ::core::ffi::c_char,
    pub private_ticket_reception_time: mbedtls_ms_time_t,
    pub private_encrypt_then_mac: ::core::ffi::c_int,
    pub private_app_secrets: mbedtls_ssl_tls13_application_secrets,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_ssl_tls13_application_secrets {
    pub client_application_traffic_secret_N: [::core::ffi::c_uchar; 64],
    pub server_application_traffic_secret_N: [::core::ffi::c_uchar; 64],
    pub exporter_master_secret: [::core::ffi::c_uchar; 64],
    pub resumption_master_secret: [::core::ffi::c_uchar; 64],
}
pub type mbedtls_ms_time_t = int64_t;
pub type mbedtls_time_t = time_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mbedtls_ssl_protocol_version(pub ::core::ffi::c_uint);
impl mbedtls_ssl_protocol_version {
    pub const MBEDTLS_SSL_VERSION_UNKNOWN: Self = Self(0);
    pub const MBEDTLS_SSL_VERSION_TLS1_2: Self = Self(771);
    pub const MBEDTLS_SSL_VERSION_TLS1_3: Self = Self(772);
}
pub type mbedtls_ssl_recv_timeout_t = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_uchar,
    size_t,
    uint32_t,
) -> ::core::ffi::c_int;
pub type mbedtls_ssl_recv_t = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_uchar,
    size_t,
) -> ::core::ffi::c_int;
pub type mbedtls_ssl_send_t = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *const ::core::ffi::c_uchar,
    size_t,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_mpi {
    pub private_p: *mut mbedtls_mpi_uint,
    pub private_s: ::core::ffi::c_short,
    pub private_n: ::core::ffi::c_ushort,
}
pub type mbedtls_mpi_uint = uint64_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mbedtls_ecp_group_id(pub ::core::ffi::c_uint);
impl mbedtls_ecp_group_id {
    pub const MBEDTLS_ECP_DP_NONE: Self = Self(0);
    pub const MBEDTLS_ECP_DP_SECP192R1: Self = Self(1);
    pub const MBEDTLS_ECP_DP_SECP224R1: Self = Self(2);
    pub const MBEDTLS_ECP_DP_SECP256R1: Self = Self(3);
    pub const MBEDTLS_ECP_DP_SECP384R1: Self = Self(4);
    pub const MBEDTLS_ECP_DP_SECP521R1: Self = Self(5);
    pub const MBEDTLS_ECP_DP_BP256R1: Self = Self(6);
    pub const MBEDTLS_ECP_DP_BP384R1: Self = Self(7);
    pub const MBEDTLS_ECP_DP_BP512R1: Self = Self(8);
    pub const MBEDTLS_ECP_DP_CURVE25519: Self = Self(9);
    pub const MBEDTLS_ECP_DP_SECP192K1: Self = Self(10);
    pub const MBEDTLS_ECP_DP_SECP224K1: Self = Self(11);
    pub const MBEDTLS_ECP_DP_SECP256K1: Self = Self(12);
    pub const MBEDTLS_ECP_DP_CURVE448: Self = Self(13);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_x509_crl {
    pub raw: mbedtls_x509_buf,
    pub tbs: mbedtls_x509_buf,
    pub version: ::core::ffi::c_int,
    pub sig_oid: mbedtls_x509_buf,
    pub issuer_raw: mbedtls_x509_buf,
    pub issuer: mbedtls_x509_name,
    pub this_update: mbedtls_x509_time,
    pub next_update: mbedtls_x509_time,
    pub entry: mbedtls_x509_crl_entry,
    pub crl_ext: mbedtls_x509_buf,
    pub private_sig_oid2: mbedtls_x509_buf,
    pub private_sig: mbedtls_x509_buf,
    pub private_sig_md: mbedtls_md_type_t,
    pub private_sig_pk: mbedtls_pk_type_t,
    pub private_sig_opts: *mut ::core::ffi::c_void,
    pub next: *mut mbedtls_x509_crl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_x509_crl_entry {
    pub raw: mbedtls_x509_buf,
    pub serial: mbedtls_x509_buf,
    pub revocation_date: mbedtls_x509_time,
    pub entry_ext: mbedtls_x509_buf,
    pub next: *mut mbedtls_x509_crl_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_x509_crt_profile {
    pub allowed_mds: uint32_t,
    pub allowed_pks: uint32_t,
    pub allowed_curves: uint32_t,
    pub rsa_min_bitlen: uint32_t,
}
pub type mbedtls_ssl_cache_set_t = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *const ::core::ffi::c_uchar,
    size_t,
    *const mbedtls_ssl_session,
) -> ::core::ffi::c_int;
pub type mbedtls_ssl_cache_get_t = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *const ::core::ffi::c_uchar,
    size_t,
    *mut mbedtls_ssl_session,
) -> ::core::ffi::c_int;
pub type libuv_tls_context_t = libuv_tls_context;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub type mbedtls_f_rng_t = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_uchar,
    size_t,
) -> ::core::ffi::c_int;
pub type psa_status_t = int32_t;
pub type libuv_tls_session_t = libuv_tls_session;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libuv_tls_session {
    pub ssl: mbedtls_ssl_context,
    pub r#in: *mut ::core::ffi::c_uchar,
    pub in_len: size_t,
    pub in_off: size_t,
    pub in_cap: size_t,
    pub out: *mut ::core::ffi::c_uchar,
    pub out_len: size_t,
    pub out_cap: size_t,
    pub alloc_cb: uv_alloc_cb,
    pub read_cb: uv_read_cb,
    pub reading: bool,
    pub handshake_done: bool,
    pub failed: bool,
    pub drain: uv_idle_t,
    pub drain_init: bool,
    pub conn: *mut libuv_connection_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tls_write_t {
    pub req: uv_write_t,
    pub buf: uv_buf_t,
    pub user_req: *mut uv_write_t,
    pub user_cb: uv_write_cb,
}
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const LOG_COMPONENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"HTTP\0") };
pub const TLS_READ_CHUNK: size_t =
    (32 as ::core::ffi::c_int as size_t).wrapping_mul(1024 as size_t);
pub const TLS_INPUT_MAX: size_t =
    (256 as ::core::ffi::c_int as size_t).wrapping_mul(1024 as size_t);
pub const TLS_PLAINTEXT_HINT: size_t =
    (64 as ::core::ffi::c_int as size_t).wrapping_mul(1024 as size_t);
unsafe extern "C" fn tls_error_string(
    mut ret: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
    mut len: size_t,
) {
    mbedtls_strerror(ret, buf, len);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_tls_compiled_in() -> bool {
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_tls_context_create(
    mut cert_path: *const ::core::ffi::c_char,
    mut key_path: *const ::core::ffi::c_char,
) -> *mut libuv_tls_context_t {
    static mut alpn: [*const ::core::ffi::c_char; 2] = [
        b"http/1.1\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut err: [::core::ffi::c_char; 160] = [0; 160];
    let mut ret: ::core::ffi::c_int = 0;
    if cert_path.is_null()
        || *cert_path.offset(0isize) == 0
        || key_path.is_null()
        || *key_path.offset(0isize) == 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"HTTPS: ssl_cert_path and ssl_key_path must both be set when ssl_enabled = true\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<libuv_tls_context_t>();
    }
    if psa_crypto_init() != PSA_SUCCESS {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"HTTPS: failed to initialize PSA crypto\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<libuv_tls_context_t>();
    }
    let mut ctx: *mut libuv_tls_context_t =
        safe_calloc(1 as size_t, ::core::mem::size_of::<libuv_tls_context_t>())
            as *mut libuv_tls_context_t;
    if ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"HTTPS: out of memory creating TLS context\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<libuv_tls_context_t>();
    }
    mbedtls_ssl_config_init(&raw mut (*ctx).conf);
    mbedtls_x509_crt_init(&raw mut (*ctx).cert);
    mbedtls_pk_init(&raw mut (*ctx).key);
    mbedtls_entropy_init(&raw mut (*ctx).entropy);
    mbedtls_ctr_drbg_init(&raw mut (*ctx).drbg);
    static mut personalization: [::core::ffi::c_char; 15] = unsafe {
        ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"lightnvr-https\0")
    };
    ret = mbedtls_ctr_drbg_seed(
        &raw mut (*ctx).drbg,
        Some(
            mbedtls_entropy_func
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut (*ctx).entropy as *mut ::core::ffi::c_void,
        &raw const personalization as *const ::core::ffi::c_char as *const ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_char; 15]>().wrapping_sub(1 as size_t),
    );
    if ret != 0 as ::core::ffi::c_int {
        tls_error_string(
            ret,
            &raw mut err as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"HTTPS: failed to seed the random generator: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut err as *mut ::core::ffi::c_char,
        );
    } else {
        ret = mbedtls_x509_crt_parse_file(&raw mut (*ctx).cert, cert_path);
        if ret != 0 as ::core::ffi::c_int {
            if ret > 0 as ::core::ffi::c_int {
                snprintf(
                    &raw mut err as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
                    b"%d certificate(s) in the file could not be parsed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ret,
                );
            } else {
                tls_error_string(
                    ret,
                    &raw mut err as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
                );
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"HTTPS: cannot load certificate %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                cert_path,
                &raw mut err as *mut ::core::ffi::c_char,
            );
        } else {
            ret = mbedtls_pk_parse_keyfile(
                &raw mut (*ctx).key,
                key_path,
                ::core::ptr::null::<::core::ffi::c_char>(),
                Some(
                    mbedtls_ctr_drbg_random
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_uchar,
                            size_t,
                        ) -> ::core::ffi::c_int,
                ),
                &raw mut (*ctx).drbg as *mut ::core::ffi::c_void,
            );
            if ret != 0 as ::core::ffi::c_int {
                tls_error_string(
                    ret,
                    &raw mut err as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"HTTPS: cannot load private key %s: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    key_path,
                    &raw mut err as *mut ::core::ffi::c_char,
                );
            } else {
                ret = mbedtls_pk_check_pair(
                    &raw mut (*ctx).cert.pk,
                    &raw mut (*ctx).key,
                    Some(
                        mbedtls_ctr_drbg_random
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut ::core::ffi::c_uchar,
                                size_t,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    &raw mut (*ctx).drbg as *mut ::core::ffi::c_void,
                );
                if ret != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"HTTPS: private key %s does not match certificate %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        key_path,
                        cert_path,
                    );
                } else {
                    ret = mbedtls_ssl_config_defaults(
                        &raw mut (*ctx).conf,
                        MBEDTLS_SSL_IS_SERVER,
                        MBEDTLS_SSL_TRANSPORT_STREAM,
                        MBEDTLS_SSL_PRESET_DEFAULT,
                    );
                    if ret != 0 as ::core::ffi::c_int {
                        tls_error_string(
                            ret,
                            &raw mut err as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
                        );
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"HTTPS: failed to apply TLS defaults: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut err as *mut ::core::ffi::c_char,
                        );
                    } else {
                        mbedtls_ssl_conf_rng(
                            &raw mut (*ctx).conf,
                            Some(
                                mbedtls_ctr_drbg_random
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *mut ::core::ffi::c_uchar,
                                        size_t,
                                    )
                                        -> ::core::ffi::c_int,
                            ),
                            &raw mut (*ctx).drbg as *mut ::core::ffi::c_void,
                        );
                        mbedtls_ssl_conf_min_tls_version(
                            &raw mut (*ctx).conf,
                            mbedtls_ssl_protocol_version::MBEDTLS_SSL_VERSION_TLS1_2,
                        );
                        mbedtls_ssl_conf_alpn_protocols(
                            &raw mut (*ctx).conf,
                            &raw mut alpn as *mut *const ::core::ffi::c_char,
                        );
                        ret = mbedtls_ssl_conf_own_cert(
                            &raw mut (*ctx).conf,
                            &raw mut (*ctx).cert,
                            &raw mut (*ctx).key,
                        );
                        if ret != 0 as ::core::ffi::c_int {
                            tls_error_string(
                                ret,
                                &raw mut err as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
                            );
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT.as_ptr(),
                                _log_stream_name,
                                b"HTTPS: cannot use certificate %s: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                cert_path,
                                &raw mut err as *mut ::core::ffi::c_char,
                            );
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT.as_ptr(),
                                _log_stream_name,
                                b"HTTPS: loaded certificate %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                cert_path,
                            );
                            return ctx;
                        }
                    }
                }
            }
        }
    }
    libuv_tls_context_free(ctx);
    return ::core::ptr::null_mut::<libuv_tls_context_t>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_tls_context_free(mut ctx: *mut libuv_tls_context_t) {
    if ctx.is_null() {
        return;
    }
    mbedtls_ssl_config_free(&raw mut (*ctx).conf);
    mbedtls_x509_crt_free(&raw mut (*ctx).cert);
    mbedtls_pk_free(&raw mut (*ctx).key);
    mbedtls_ctr_drbg_free(&raw mut (*ctx).drbg);
    mbedtls_entropy_free(&raw mut (*ctx).entropy);
    safe_free(ctx as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn bio_recv(
    mut opaque: *mut ::core::ffi::c_void,
    mut buf: *mut ::core::ffi::c_uchar,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut s: *mut libuv_tls_session_t = opaque as *mut libuv_tls_session_t;
    let mut available: size_t = (*s).in_len.wrapping_sub((*s).in_off);
    if available == 0 as size_t {
        return MBEDTLS_ERR_SSL_WANT_READ;
    }
    if len > available {
        len = available;
    }
    memcpy(
        buf as *mut ::core::ffi::c_void,
        (*s).r#in.offset((*s).in_off as isize) as *const ::core::ffi::c_void,
        len,
    );
    (*s).in_off = (*s).in_off.wrapping_add(len);
    return len as ::core::ffi::c_int;
}
unsafe extern "C" fn bio_send(
    mut opaque: *mut ::core::ffi::c_void,
    mut buf: *const ::core::ffi::c_uchar,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut s: *mut libuv_tls_session_t = opaque as *mut libuv_tls_session_t;
    if len > INT32_MAX as size_t {
        len = INT32_MAX as size_t;
    }
    if len > (*s).out_cap.wrapping_sub((*s).out_len) {
        let mut capacity: size_t = if (*s).out_cap != 0 {
            (*s).out_cap
        } else {
            4096 as size_t
        };
        while capacity.wrapping_sub((*s).out_len) < len {
            if capacity > (SIZE_MAX as size_t).wrapping_div(2 as size_t) {
                return MBEDTLS_ERR_SSL_ALLOC_FAILED;
            }
            capacity = capacity.wrapping_mul(2 as size_t);
        }
        let mut grown: *mut ::core::ffi::c_uchar =
            safe_realloc((*s).out as *mut ::core::ffi::c_void, capacity)
                as *mut ::core::ffi::c_uchar;
        if grown.is_null() {
            return MBEDTLS_ERR_SSL_ALLOC_FAILED;
        }
        (*s).out = grown;
        (*s).out_cap = capacity;
    }
    memcpy(
        (*s).out.offset((*s).out_len as isize) as *mut ::core::ffi::c_void,
        buf as *const ::core::ffi::c_void,
        len,
    );
    (*s).out_len = (*s).out_len.wrapping_add(len);
    return len as ::core::ffi::c_int;
}
unsafe extern "C" fn tls_write_done(mut req: *mut uv_write_t, mut status: ::core::ffi::c_int) {
    let mut write: *mut tls_write_t = req as *mut tls_write_t;
    let mut user_req: *mut uv_write_t = (*write).user_req;
    let mut user_cb: uv_write_cb = (*write).user_cb;
    safe_free((*write).buf.base as *mut ::core::ffi::c_void);
    safe_free(write as *mut ::core::ffi::c_void);
    if user_cb.is_some() {
        user_cb.expect("non-null function pointer")(user_req, status);
    }
}
unsafe extern "C" fn flush_output(
    mut conn: *mut libuv_connection_t,
    mut user_req: *mut uv_write_t,
    mut user_cb: uv_write_cb,
) -> ::core::ffi::c_int {
    let mut s: *mut libuv_tls_session_t = (*conn).tls_session as *mut libuv_tls_session_t;
    if (*s).out_len == 0 as size_t && user_req.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut write: *mut tls_write_t =
        safe_calloc(1 as size_t, ::core::mem::size_of::<tls_write_t>()) as *mut tls_write_t;
    if write.is_null() {
        return C2Rust_Unnamed_5::UV_ENOMEM.0;
    }
    (*write).buf = uv_buf_init(
        (*s).out as *mut ::core::ffi::c_char,
        (*s).out_len as ::core::ffi::c_uint,
    );
    (*write).user_req = user_req;
    (*write).user_cb = user_cb;
    (*s).out = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*s).out_len = 0 as size_t;
    (*s).out_cap = 0 as size_t;
    if !user_req.is_null() {
        (*user_req).handle = &raw mut (*conn).handle as *mut uv_stream_t;
    }
    let mut r: ::core::ffi::c_int = uv_write(
        &raw mut (*write).req,
        &raw mut (*conn).handle as *mut uv_stream_t,
        &raw mut (*write).buf,
        1 as ::core::ffi::c_uint,
        Some(tls_write_done as unsafe extern "C" fn(*mut uv_write_t, ::core::ffi::c_int) -> ()),
    );
    if r != 0 as ::core::ffi::c_int {
        safe_free((*write).buf.base as *mut ::core::ffi::c_void);
        safe_free(write as *mut ::core::ffi::c_void);
    }
    return r;
}
unsafe extern "C" fn close_after_alert(mut req: *mut uv_write_t, mut status: ::core::ffi::c_int) {
    let mut conn: *mut libuv_connection_t = (if !(*req).handle.is_null() {
        (*(*req).handle).data
    } else {
        NULL
    }) as *mut libuv_connection_t;
    safe_free(req as *mut ::core::ffi::c_void);
    if !conn.is_null() {
        libuv_connection_close(conn);
    }
}
unsafe extern "C" fn fail_session(mut conn: *mut libuv_connection_t) {
    let mut s: *mut libuv_tls_session_t = (*conn).tls_session as *mut libuv_tls_session_t;
    (*s).failed = r#true != 0;
    (*s).reading = r#false != 0;
    uv_read_stop(&raw mut (*conn).handle as *mut uv_stream_t);
    if (*s).out_len > 0 as size_t && uv_is_closing(&raw mut (*conn).handle as *mut uv_handle_t) == 0
    {
        let mut req: *mut uv_write_t =
            safe_calloc(1 as size_t, ::core::mem::size_of::<uv_write_t>()) as *mut uv_write_t;
        if !req.is_null()
            && flush_output(
                conn,
                req,
                Some(
                    close_after_alert
                        as unsafe extern "C" fn(*mut uv_write_t, ::core::ffi::c_int) -> (),
                ),
            ) == 0 as ::core::ffi::c_int
        {
            return;
        }
        safe_free(req as *mut ::core::ffi::c_void);
    }
    libuv_connection_close(conn);
}
unsafe extern "C" fn deliver_status(mut conn: *mut libuv_connection_t, mut status: ssize_t) {
    let mut s: *mut libuv_tls_session_t = (*conn).tls_session as *mut libuv_tls_session_t;
    if !(*s).reading || (*s).read_cb.is_none() {
        libuv_connection_close(conn);
        return;
    }
    let mut buf: uv_buf_t = uv_buf_init(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_uint,
    );
    (*s).alloc_cb.expect("non-null function pointer")(
        &raw mut (*conn).handle as *mut uv_handle_t,
        TLS_PLAINTEXT_HINT,
        &raw mut buf,
    );
    (*s).read_cb.expect("non-null function pointer")(
        &raw mut (*conn).handle as *mut uv_stream_t,
        status,
        &raw mut buf,
    );
}
unsafe extern "C" fn session_active(
    mut conn: *mut libuv_connection_t,
    mut s: *mut libuv_tls_session_t,
) -> bool {
    return (*conn).tls_session == s as *mut ::core::ffi::c_void
        && !(*s).failed
        && (*s).reading as ::core::ffi::c_int != 0
        && uv_is_closing(&raw mut (*conn).handle as *mut uv_handle_t) == 0;
}
unsafe extern "C" fn process_input(mut conn: *mut libuv_connection_t) {
    let mut s: *mut libuv_tls_session_t = (*conn).tls_session as *mut libuv_tls_session_t;
    let mut err: [::core::ffi::c_char; 128] = [0; 128];
    if !(*s).handshake_done {
        let mut ret: ::core::ffi::c_int = mbedtls_ssl_handshake(&raw mut (*s).ssl);
        if ret == MBEDTLS_ERR_SSL_WANT_READ || ret == MBEDTLS_ERR_SSL_WANT_WRITE {
            if flush_output(conn, ::core::ptr::null_mut::<uv_write_t>(), None)
                != 0 as ::core::ffi::c_int
            {
                fail_session(conn);
            }
            return;
        }
        if ret != 0 as ::core::ffi::c_int {
            tls_error_string(
                ret,
                &raw mut err as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"HTTPS: handshake with %s failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*conn).request.client_ip as *mut ::core::ffi::c_char,
                &raw mut err as *mut ::core::ffi::c_char,
            );
            fail_session(conn);
            return;
        }
        (*s).handshake_done = r#true != 0;
        if flush_output(conn, ::core::ptr::null_mut::<uv_write_t>(), None)
            != 0 as ::core::ffi::c_int
        {
            fail_session(conn);
            return;
        }
    }
    while session_active(conn, s) {
        if mbedtls_ssl_get_bytes_avail(&raw mut (*s).ssl) == 0 as size_t
            && (*s).in_off == (*s).in_len
        {
            return;
        }
        let mut buf: uv_buf_t = uv_buf_init(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_uint,
        );
        (*s).alloc_cb.expect("non-null function pointer")(
            &raw mut (*conn).handle as *mut uv_handle_t,
            TLS_PLAINTEXT_HINT,
            &raw mut buf,
        );
        if buf.base.is_null() || buf.len == 0 as size_t {
            (*s).read_cb.expect("non-null function pointer")(
                &raw mut (*conn).handle as *mut uv_stream_t,
                C2Rust_Unnamed_5::UV_ENOBUFS.0 as ssize_t,
                &raw mut buf,
            );
            return;
        }
        let mut total: size_t = 0 as size_t;
        let mut ret_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while total < buf.len {
            ret_0 = mbedtls_ssl_read(
                &raw mut (*s).ssl,
                (buf.base as *mut ::core::ffi::c_uchar).offset(total as isize),
                buf.len.wrapping_sub(total),
            );
            if ret_0 <= 0 as ::core::ffi::c_int {
                break;
            }
            total = total.wrapping_add(ret_0 as size_t);
        }
        let mut status: ssize_t = 0 as ssize_t;
        if total < buf.len {
            if ret_0 == 0 as ::core::ffi::c_int || ret_0 == MBEDTLS_ERR_SSL_PEER_CLOSE_NOTIFY {
                status = C2Rust_Unnamed_5::UV_EOF.0 as ssize_t;
            } else if ret_0 != MBEDTLS_ERR_SSL_WANT_READ && ret_0 != MBEDTLS_ERR_SSL_WANT_WRITE {
                tls_error_string(
                    ret_0,
                    &raw mut err as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"HTTPS: read from %s failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*conn).request.client_ip as *mut ::core::ffi::c_char,
                    &raw mut err as *mut ::core::ffi::c_char,
                );
                status = C2Rust_Unnamed_5::UV_EPROTO.0 as ssize_t;
            }
        }
        if (*s).out_len > 0 as size_t
            && flush_output(conn, ::core::ptr::null_mut::<uv_write_t>(), None)
                != 0 as ::core::ffi::c_int
        {
            status = C2Rust_Unnamed_5::UV_EPROTO.0 as ssize_t;
        }
        if total > 0 as size_t {
            (*s).read_cb.expect("non-null function pointer")(
                &raw mut (*conn).handle as *mut uv_stream_t,
                total as ssize_t,
                &raw mut buf,
            );
            if status != 0 as ssize_t && session_active(conn, s) as ::core::ffi::c_int != 0 {
                deliver_status(conn, status);
            }
        } else {
            (*s).read_cb.expect("non-null function pointer")(
                &raw mut (*conn).handle as *mut uv_stream_t,
                status,
                &raw mut buf,
            );
        }
        if status != 0 as ssize_t {
            if status == C2Rust_Unnamed_5::UV_EPROTO.0 as ssize_t
                && (*conn).tls_session == s as *mut ::core::ffi::c_void
            {
                (*s).failed = r#true != 0;
            }
            return;
        }
        if total < buf.len {
            return;
        }
    }
}
unsafe extern "C" fn has_buffered_input(mut s: *mut libuv_tls_session_t) -> bool {
    return (*s).in_off < (*s).in_len
        || mbedtls_ssl_get_bytes_avail(&raw mut (*s).ssl) > 0 as size_t;
}
unsafe extern "C" fn drain_cb(mut handle: *mut uv_idle_t) {
    uv_idle_stop(handle);
    let mut s: *mut libuv_tls_session_t = (*handle).data as *mut libuv_tls_session_t;
    let mut conn: *mut libuv_connection_t = (*s).conn;
    if conn.is_null()
        || (*conn).tls_session != s as *mut ::core::ffi::c_void
        || !session_active(conn, s)
    {
        return;
    }
    process_input(conn);
}
unsafe extern "C" fn schedule_drain(
    mut conn: *mut libuv_connection_t,
    mut s: *mut libuv_tls_session_t,
) {
    if !(*s).drain_init {
        if uv_idle_init((*conn).handle.r#loop, &raw mut (*s).drain) != 0 as ::core::ffi::c_int {
            return;
        }
        (*s).drain.data = s as *mut ::core::ffi::c_void;
        (*s).drain_init = r#true != 0;
    }
    uv_idle_start(
        &raw mut (*s).drain,
        Some(drain_cb as unsafe extern "C" fn(*mut uv_idle_t) -> ()),
    );
}
unsafe extern "C" fn tls_alloc(
    mut handle: *mut uv_handle_t,
    mut suggested_size: size_t,
    mut buf: *mut uv_buf_t,
) {
    let mut conn: *mut libuv_connection_t = (*handle).data as *mut libuv_connection_t;
    let mut s: *mut libuv_tls_session_t = (if !conn.is_null() {
        (*conn).tls_session
    } else {
        NULL
    }) as *mut libuv_tls_session_t;
    (*buf).base = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*buf).len = 0 as size_t;
    if s.is_null() || (*s).failed as ::core::ffi::c_int != 0 {
        return;
    }
    if (*s).in_off > 0 as size_t {
        memmove(
            (*s).r#in as *mut ::core::ffi::c_void,
            (*s).r#in.offset((*s).in_off as isize) as *const ::core::ffi::c_void,
            (*s).in_len.wrapping_sub((*s).in_off),
        );
        (*s).in_len = (*s).in_len.wrapping_sub((*s).in_off);
        (*s).in_off = 0 as size_t;
    }
    let mut needed: size_t = (*s).in_len.wrapping_add(TLS_READ_CHUNK);
    if needed > TLS_INPUT_MAX {
        return;
    }
    if needed > (*s).in_cap {
        let mut grown: *mut ::core::ffi::c_uchar =
            safe_realloc((*s).r#in as *mut ::core::ffi::c_void, needed)
                as *mut ::core::ffi::c_uchar;
        if grown.is_null() {
            return;
        }
        (*s).r#in = grown;
        (*s).in_cap = needed;
    }
    (*buf).base = ((*s).r#in as *mut ::core::ffi::c_char).offset((*s).in_len as isize);
    (*buf).len = TLS_READ_CHUNK;
}
unsafe extern "C" fn tls_read(
    mut stream: *mut uv_stream_t,
    mut nread: ssize_t,
    mut buf: *const uv_buf_t,
) {
    let mut conn: *mut libuv_connection_t = (*stream).data as *mut libuv_connection_t;
    if conn.is_null() || (*conn).tls_session.is_null() {
        return;
    }
    let mut s: *mut libuv_tls_session_t = (*conn).tls_session as *mut libuv_tls_session_t;
    if (*s).failed as ::core::ffi::c_int != 0 || nread == 0 as ssize_t {
        return;
    }
    if nread < 0 as ssize_t {
        if (*s).handshake_done {
            deliver_status(conn, nread);
        } else {
            libuv_connection_close(conn);
        }
        return;
    }
    (*s).in_len = (*s).in_len.wrapping_add(nread as size_t);
    process_input(conn);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_tls_session_start(
    mut conn: *mut libuv_connection_t,
    mut ctx: *mut libuv_tls_context_t,
) -> ::core::ffi::c_int {
    if conn.is_null() || ctx.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut s: *mut libuv_tls_session_t =
        safe_calloc(1 as size_t, ::core::mem::size_of::<libuv_tls_session_t>())
            as *mut libuv_tls_session_t;
    if s.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    mbedtls_ssl_init(&raw mut (*s).ssl);
    let mut ret: ::core::ffi::c_int = mbedtls_ssl_setup(&raw mut (*s).ssl, &raw mut (*ctx).conf);
    if ret != 0 as ::core::ffi::c_int {
        let mut err: [::core::ffi::c_char; 128] = [0; 128];
        tls_error_string(
            ret,
            &raw mut err as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"HTTPS: cannot set up TLS session: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut err as *mut ::core::ffi::c_char,
        );
        mbedtls_ssl_free(&raw mut (*s).ssl);
        safe_free(s as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    mbedtls_ssl_set_bio(
        &raw mut (*s).ssl,
        s as *mut ::core::ffi::c_void,
        Some(
            bio_send
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            bio_recv
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        None,
    );
    (*s).conn = conn;
    (*conn).tls_session = s as *mut ::core::ffi::c_void;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn session_release(mut s: *mut libuv_tls_session_t) {
    mbedtls_ssl_free(&raw mut (*s).ssl);
    safe_free((*s).r#in as *mut ::core::ffi::c_void);
    safe_free((*s).out as *mut ::core::ffi::c_void);
    safe_free(s as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn drain_closed(mut handle: *mut uv_handle_t) {
    session_release((*handle).data as *mut libuv_tls_session_t);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_tls_session_free(mut conn: *mut libuv_connection_t) {
    if conn.is_null() || (*conn).tls_session.is_null() {
        return;
    }
    let mut s: *mut libuv_tls_session_t = (*conn).tls_session as *mut libuv_tls_session_t;
    (*conn).tls_session = NULL;
    (*s).conn = ::core::ptr::null_mut::<libuv_connection_t>();
    if (*s).drain_init {
        uv_close(
            &raw mut (*s).drain as *mut uv_handle_t,
            Some(drain_closed as unsafe extern "C" fn(*mut uv_handle_t) -> ()),
        );
        return;
    }
    session_release(s);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_connection_is_tls(mut conn: *const libuv_connection_t) -> bool {
    return !conn.is_null() && !(*conn).tls_session.is_null();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_connection_write(
    mut conn: *mut libuv_connection_t,
    mut req: *mut uv_write_t,
    mut bufs: *const uv_buf_t,
    mut nbufs: ::core::ffi::c_uint,
    mut cb: uv_write_cb,
) -> ::core::ffi::c_int {
    let mut s: *mut libuv_tls_session_t = (*conn).tls_session as *mut libuv_tls_session_t;
    if s.is_null() {
        return uv_write(
            req,
            &raw mut (*conn).handle as *mut uv_stream_t,
            bufs,
            nbufs,
            cb,
        );
    }
    if (*s).failed as ::core::ffi::c_int != 0 || !(*s).handshake_done {
        return C2Rust_Unnamed_5::UV_ENOTCONN.0;
    }
    if uv_is_closing(&raw mut (*conn).handle as *mut uv_handle_t) != 0 {
        return C2Rust_Unnamed_5::UV_EBADF.0;
    }
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < nbufs {
        let mut data: *const ::core::ffi::c_uchar =
            (*bufs.offset(i as isize)).base as *const ::core::ffi::c_uchar;
        let mut remaining: size_t = (*bufs.offset(i as isize)).len;
        while remaining > 0 as size_t {
            let mut ret: ::core::ffi::c_int = mbedtls_ssl_write(&raw mut (*s).ssl, data, remaining);
            if ret < 0 as ::core::ffi::c_int {
                let mut err: [::core::ffi::c_char; 128] = [0; 128];
                tls_error_string(
                    ret,
                    &raw mut err as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"HTTPS: encrypting response for %s failed: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*conn).request.client_ip as *mut ::core::ffi::c_char,
                    &raw mut err as *mut ::core::ffi::c_char,
                );
                (*s).failed = r#true != 0;
                (*s).out_len = 0 as size_t;
                return C2Rust_Unnamed_5::UV_EPROTO.0;
            }
            data = data.offset(ret as isize);
            remaining = remaining.wrapping_sub(ret as size_t);
        }
        i = i.wrapping_add(1);
    }
    return flush_output(conn, req, cb);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_connection_read_start(
    mut conn: *mut libuv_connection_t,
    mut alloc_cb: uv_alloc_cb,
    mut read_cb: uv_read_cb,
) -> ::core::ffi::c_int {
    let mut s: *mut libuv_tls_session_t = (*conn).tls_session as *mut libuv_tls_session_t;
    if s.is_null() {
        return uv_read_start(
            &raw mut (*conn).handle as *mut uv_stream_t,
            alloc_cb,
            read_cb,
        );
    }
    if (*s).failed {
        return C2Rust_Unnamed_5::UV_EPROTO.0;
    }
    (*s).alloc_cb = alloc_cb;
    (*s).read_cb = read_cb;
    (*s).reading = r#true != 0;
    let mut r: ::core::ffi::c_int = uv_read_start(
        &raw mut (*conn).handle as *mut uv_stream_t,
        Some(tls_alloc as unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> ()),
        Some(tls_read as unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> ()),
    );
    if r == C2Rust_Unnamed_5::UV_EALREADY.0 {
        r = 0 as ::core::ffi::c_int;
    }
    if r == 0 as ::core::ffi::c_int
        && (*s).handshake_done as ::core::ffi::c_int != 0
        && has_buffered_input(s) as ::core::ffi::c_int != 0
    {
        schedule_drain(conn, s);
    }
    return r;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn libuv_connection_read_stop(
    mut conn: *mut libuv_connection_t,
) -> ::core::ffi::c_int {
    let mut s: *mut libuv_tls_session_t = (*conn).tls_session as *mut libuv_tls_session_t;
    if !s.is_null() {
        (*s).reading = r#false != 0;
        if (*s).drain_init {
            uv_idle_stop(&raw mut (*s).drain);
        }
    }
    return uv_read_stop(&raw mut (*conn).handle as *mut uv_stream_t);
}
pub const MBEDTLS_ERR_SSL_PEER_CLOSE_NOTIFY: ::core::ffi::c_int = -0x7880 as ::core::ffi::c_int;
pub const MBEDTLS_ERR_SSL_ALLOC_FAILED: ::core::ffi::c_int = -0x7f00 as ::core::ffi::c_int;
pub const MBEDTLS_ERR_SSL_WANT_READ: ::core::ffi::c_int = -0x6900 as ::core::ffi::c_int;
pub const MBEDTLS_ERR_SSL_WANT_WRITE: ::core::ffi::c_int = -0x6880 as ::core::ffi::c_int;
pub const MBEDTLS_SSL_TRANSPORT_STREAM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MBEDTLS_SSL_IS_SERVER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MBEDTLS_SSL_PRESET_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mbedtls_ssl_conf_min_tls_version(
    mut conf: *mut mbedtls_ssl_config,
    mut tls_version: mbedtls_ssl_protocol_version,
) {
    (*conf).private_min_tls_version = tls_version;
}
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PSA_SUCCESS: psa_status_t = 0 as ::core::ffi::c_int as psa_status_t;
