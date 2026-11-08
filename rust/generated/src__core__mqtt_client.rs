use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type mosquitto;
    unsafe fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_detach(__th: pthread_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn rewinddir(__dirp: *mut DIR);
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_lib_init() -> ::core::ffi::c_int;
    unsafe fn mosquitto_lib_cleanup() -> ::core::ffi::c_int;
    unsafe fn mosquitto_username_pw_set(
        mosq_0: *mut mosquitto,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_connect_callback_set(
        mosq_0: *mut mosquitto,
        on_connect_0: LIBMOSQ_CB_connect,
    );
    unsafe fn mosquitto_disconnect_callback_set(
        mosq_0: *mut mosquitto,
        on_disconnect_0: LIBMOSQ_CB_disconnect,
    );
    unsafe fn mosquitto_publish_callback_set(
        mosq_0: *mut mosquitto,
        on_publish_0: LIBMOSQ_CB_publish,
    );
    unsafe fn mosquitto_message_callback_set(
        mosq_0: *mut mosquitto,
        on_message_0: LIBMOSQ_CB_message,
    );
    unsafe fn mosquitto_log_callback_set(mosq_0: *mut mosquitto, on_log_0: LIBMOSQ_CB_log);
    unsafe fn mosquitto_connect(
        mosq_0: *mut mosquitto,
        host: *const ::core::ffi::c_char,
        port: ::core::ffi::c_int,
        keepalive: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_disconnect(mosq_0: *mut mosquitto) -> ::core::ffi::c_int;
    unsafe fn mosquitto_new(
        id: *const ::core::ffi::c_char,
        clean_session: bool,
        obj: *mut ::core::ffi::c_void,
    ) -> *mut mosquitto;
    unsafe fn mosquitto_destroy(mosq_0: *mut mosquitto);
    unsafe fn mosquitto_loop_start(mosq_0: *mut mosquitto) -> ::core::ffi::c_int;
    unsafe fn mosquitto_loop_stop(mosq_0: *mut mosquitto, force: bool) -> ::core::ffi::c_int;
    unsafe fn mosquitto_int_option(
        mosq_0: *mut mosquitto,
        option: mosq_opt_t,
        value: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_publish(
        mosq_0: *mut mosquitto,
        mid: *mut ::core::ffi::c_int,
        topic: *const ::core::ffi::c_char,
        payloadlen: ::core::ffi::c_int,
        payload: *const ::core::ffi::c_void,
        qos: ::core::ffi::c_int,
        retain: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_subscribe(
        mosq_0: *mut mosquitto,
        mid: *mut ::core::ffi::c_int,
        sub: *const ::core::ffi::c_char,
        qos: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_will_set(
        mosq_0: *mut mosquitto,
        topic: *const ::core::ffi::c_char,
        payloadlen: ::core::ffi::c_int,
        payload: *const ::core::ffi::c_void,
        qos: ::core::ffi::c_int,
        retain: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
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
    unsafe fn mosquitto_strerror(mosq_errno: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn mosquitto_connack_string(
        connack_code: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    unsafe fn event_envelope_serialize(
        event: *const event_envelope_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn mqtt_presence_configure(
        topic_prefix: *const ::core::ffi::c_char,
        installation_uuid: *const ::core::ffi::c_char,
        run_id: *const ::core::ffi::c_char,
        boot_id: *const ::core::ffi::c_char,
        visibility_scope: *const ::core::ffi::c_char,
        version: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn mqtt_presence_build_will(
        configured_at_ms: int64_t,
        topic: *mut ::core::ffi::c_char,
        topic_size: size_t,
        payload: *mut ::core::ffi::c_char,
        payload_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mqtt_presence_build(
        state: mqtt_presence_state_t,
        operational_state: mqtt_operational_state_t,
        active_incidents: size_t,
        observed_at_ms: int64_t,
        topic: *mut ::core::ffi::c_char,
        topic_size: size_t,
        payload: *mut ::core::ffi::c_char,
        payload_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mqtt_presence_record_connected();
    unsafe fn mqtt_presence_record_disconnected();
    unsafe fn mqtt_presence_record_publish(succeeded: bool, observed_at_ms: int64_t);
    unsafe fn mqtt_presence_get_stats(stats: *mut mqtt_presence_stats_t);
    unsafe fn mqtt_presence_copy_topic(
        topic: *mut ::core::ffi::c_char,
        topic_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mqtt_presence_copy_installation_uuid(
        uuid: *mut ::core::ffi::c_char,
        uuid_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_identity_get_source(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn log_set_thread_context(
        component: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    );
    unsafe fn sanitize_stream_name(
        input: *const ::core::ffi::c_char,
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    );
    unsafe fn ensure_dir(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn linux_restart_read_evidence(
        proc_root: *const ::core::ffi::c_char,
        run_id: *const ::core::ffi::c_char,
        process_start_monotonic_ms: uint64_t,
        evidence: *mut linux_restart_evidence_t,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_snapshot_copy(snapshot: *mut system_health_snapshot_t) -> bool;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn system_health_evaluator_service_active_copy(
        incidents: *mut system_health_incident_view_t,
        capacity: size_t,
    ) -> size_t;
    unsafe fn system_health_evaluator_service_copy_run(
        run: *mut system_health_process_run_t,
    ) -> bool;
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_all_stream_configs(
        streams: *mut stream_config_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn interruptible_sleep_init(s: *mut interruptible_sleep_t) -> ::core::ffi::c_int;
    unsafe fn interruptible_sleep_destroy(s: *mut interruptible_sleep_t);
    unsafe fn interruptible_sleep_wait(s: *mut interruptible_sleep_t, seconds: ::core::ffi::c_int);
    unsafe fn interruptible_sleep_wake(s: *mut interruptible_sleep_t);
    unsafe fn go2rtc_get_snapshot(
        stream_name: *const ::core::ffi::c_char,
        jpeg_data: *mut *mut ::core::ffi::c_uchar,
        jpeg_size: *mut size_t,
    ) -> bool;
    unsafe fn go2rtc_snapshot_cleanup_thread();
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type clockid_t = __clockid_t;
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
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
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
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const PTHREAD_MUTEX_TIMED_NP: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE_NP: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK_NP: Self = Self(2);
    pub const PTHREAD_MUTEX_ADAPTIVE_NP: Self = Self(3);
    pub const PTHREAD_MUTEX_NORMAL: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK: Self = Self(2);
    pub const PTHREAD_MUTEX_DEFAULT: Self = Self(0);
    pub const PTHREAD_MUTEX_FAST_NP: Self = Self(0);
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
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mosq_err_t(pub ::core::ffi::c_int);
impl mosq_err_t {
    pub const MOSQ_ERR_QUOTA_EXCEEDED: Self = Self(-6);
    pub const MOSQ_ERR_AUTH_DELAYED: Self = Self(-5);
    pub const MOSQ_ERR_AUTH_CONTINUE: Self = Self(-4);
    pub const MOSQ_ERR_NO_SUBSCRIBERS: Self = Self(-3);
    pub const MOSQ_ERR_SUB_EXISTS: Self = Self(-2);
    pub const MOSQ_ERR_CONN_PENDING: Self = Self(-1);
    pub const MOSQ_ERR_SUCCESS: Self = Self(0);
    pub const MOSQ_ERR_NOMEM: Self = Self(1);
    pub const MOSQ_ERR_PROTOCOL: Self = Self(2);
    pub const MOSQ_ERR_INVAL: Self = Self(3);
    pub const MOSQ_ERR_NO_CONN: Self = Self(4);
    pub const MOSQ_ERR_CONN_REFUSED: Self = Self(5);
    pub const MOSQ_ERR_NOT_FOUND: Self = Self(6);
    pub const MOSQ_ERR_CONN_LOST: Self = Self(7);
    pub const MOSQ_ERR_TLS: Self = Self(8);
    pub const MOSQ_ERR_PAYLOAD_SIZE: Self = Self(9);
    pub const MOSQ_ERR_NOT_SUPPORTED: Self = Self(10);
    pub const MOSQ_ERR_AUTH: Self = Self(11);
    pub const MOSQ_ERR_ACL_DENIED: Self = Self(12);
    pub const MOSQ_ERR_UNKNOWN: Self = Self(13);
    pub const MOSQ_ERR_ERRNO: Self = Self(14);
    pub const MOSQ_ERR_EAI: Self = Self(15);
    pub const MOSQ_ERR_PROXY: Self = Self(16);
    pub const MOSQ_ERR_PLUGIN_DEFER: Self = Self(17);
    pub const MOSQ_ERR_MALFORMED_UTF8: Self = Self(18);
    pub const MOSQ_ERR_KEEPALIVE: Self = Self(19);
    pub const MOSQ_ERR_LOOKUP: Self = Self(20);
    pub const MOSQ_ERR_MALFORMED_PACKET: Self = Self(21);
    pub const MOSQ_ERR_DUPLICATE_PROPERTY: Self = Self(22);
    pub const MOSQ_ERR_TLS_HANDSHAKE: Self = Self(23);
    pub const MOSQ_ERR_QOS_NOT_SUPPORTED: Self = Self(24);
    pub const MOSQ_ERR_OVERSIZE_PACKET: Self = Self(25);
    pub const MOSQ_ERR_OCSP: Self = Self(26);
    pub const MOSQ_ERR_TIMEOUT: Self = Self(27);
    pub const MOSQ_ERR_ALREADY_EXISTS: Self = Self(31);
    pub const MOSQ_ERR_PLUGIN_IGNORE: Self = Self(32);
    pub const MOSQ_ERR_HTTP_BAD_ORIGIN: Self = Self(33);
    pub const MOSQ_ERR_UNSPECIFIED: Self = Self(128);
    pub const MOSQ_ERR_IMPLEMENTATION_SPECIFIC: Self = Self(131);
    pub const MOSQ_ERR_UNSUPPORTED_PROTOCOL_VERSION: Self = Self(132);
    pub const MOSQ_ERR_CLIENT_IDENTIFIER_NOT_VALID: Self = Self(133);
    pub const MOSQ_ERR_BAD_USERNAME_OR_PASSWORD: Self = Self(134);
    pub const MOSQ_ERR_SERVER_UNAVAILABLE: Self = Self(136);
    pub const MOSQ_ERR_SERVER_BUSY: Self = Self(137);
    pub const MOSQ_ERR_BANNED: Self = Self(138);
    pub const MOSQ_ERR_SERVER_SHUTTING_DOWN: Self = Self(139);
    pub const MOSQ_ERR_BAD_AUTHENTICATION_METHOD: Self = Self(140);
    pub const MOSQ_ERR_SESSION_TAKEN_OVER: Self = Self(142);
    pub const MOSQ_ERR_TOPIC_FILTER_INVALID: Self = Self(143);
    pub const MOSQ_ERR_TOPIC_NAME_INVALID: Self = Self(144);
    pub const MOSQ_ERR_PACKET_ID_IN_USE: Self = Self(145);
    pub const MOSQ_ERR_PACKET_ID_NOT_FOUND: Self = Self(146);
    pub const MOSQ_ERR_RECEIVE_MAXIMUM_EXCEEDED: Self = Self(147);
    pub const MOSQ_ERR_TOPIC_ALIAS_INVALID: Self = Self(148);
    pub const MOSQ_ERR_MESSAGE_RATE_TOO_HIGH: Self = Self(150);
    pub const MOSQ_ERR_ADMINISTRATIVE_ACTION: Self = Self(152);
    pub const MOSQ_ERR_PAYLOAD_FORMAT_INVALID: Self = Self(153);
    pub const MOSQ_ERR_RETAIN_NOT_SUPPORTED: Self = Self(154);
    pub const MOSQ_ERR_USE_ANOTHER_SERVER: Self = Self(156);
    pub const MOSQ_ERR_SERVER_MOVED: Self = Self(157);
    pub const MOSQ_ERR_SHARED_SUBS_NOT_SUPPORTED: Self = Self(158);
    pub const MOSQ_ERR_CONNECTION_RATE_EXCEEDED: Self = Self(159);
    pub const MOSQ_ERR_MAXIMUM_CONNECT_TIME: Self = Self(160);
    pub const MOSQ_ERR_SUBSCRIPTION_IDS_NOT_SUPPORTED: Self = Self(161);
    pub const MOSQ_ERR_WILDCARD_SUBS_NOT_SUPPORTED: Self = Self(162);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mosq_opt_t(pub ::core::ffi::c_uint);
impl mosq_opt_t {
    pub const MOSQ_OPT_PROTOCOL_VERSION: Self = Self(1);
    pub const MOSQ_OPT_SSL_CTX: Self = Self(2);
    pub const MOSQ_OPT_SSL_CTX_WITH_DEFAULTS: Self = Self(3);
    pub const MOSQ_OPT_RECEIVE_MAXIMUM: Self = Self(4);
    pub const MOSQ_OPT_SEND_MAXIMUM: Self = Self(5);
    pub const MOSQ_OPT_TLS_KEYFORM: Self = Self(6);
    pub const MOSQ_OPT_TLS_ENGINE: Self = Self(7);
    pub const MOSQ_OPT_TLS_ENGINE_KPASS_SHA1: Self = Self(8);
    pub const MOSQ_OPT_TLS_OCSP_REQUIRED: Self = Self(9);
    pub const MOSQ_OPT_TLS_ALPN: Self = Self(10);
    pub const MOSQ_OPT_TCP_NODELAY: Self = Self(11);
    pub const MOSQ_OPT_BIND_ADDRESS: Self = Self(12);
    pub const MOSQ_OPT_TLS_USE_OS_CERTS: Self = Self(13);
    pub const MOSQ_OPT_DISABLE_SOCKETPAIR: Self = Self(14);
    pub const MOSQ_OPT_TRANSPORT: Self = Self(15);
    pub const MOSQ_OPT_HTTP_PATH: Self = Self(16);
    pub const MOSQ_OPT_HTTP_HEADER_SIZE: Self = Self(17);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mosquitto_message {
    pub mid: ::core::ffi::c_int,
    pub topic: *mut ::core::ffi::c_char,
    pub payload: *mut ::core::ffi::c_void,
    pub payloadlen: ::core::ffi::c_int,
    pub qos: ::core::ffi::c_int,
    pub retain: bool,
}
pub type LIBMOSQ_CB_connect = Option<
    unsafe extern "C" fn(*mut mosquitto, *mut ::core::ffi::c_void, ::core::ffi::c_int) -> (),
>;
pub type LIBMOSQ_CB_disconnect = Option<
    unsafe extern "C" fn(*mut mosquitto, *mut ::core::ffi::c_void, ::core::ffi::c_int) -> (),
>;
pub type LIBMOSQ_CB_publish = Option<
    unsafe extern "C" fn(*mut mosquitto, *mut ::core::ffi::c_void, ::core::ffi::c_int) -> (),
>;
pub type LIBMOSQ_CB_message = Option<
    unsafe extern "C" fn(*mut mosquitto, *mut ::core::ffi::c_void, *const mosquitto_message) -> (),
>;
pub type LIBMOSQ_CB_log = Option<
    unsafe extern "C" fn(
        *mut mosquitto,
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
    ) -> (),
>;
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
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_scope_t(pub ::core::ffi::c_uint);
impl system_health_scope_t {
    pub const SYSTEM_HEALTH_SCOPE_PROCESS: Self = Self(0);
    pub const SYSTEM_HEALTH_SCOPE_CONTAINER: Self = Self(1);
    pub const SYSTEM_HEALTH_SCOPE_HOST: Self = Self(2);
    pub const SYSTEM_HEALTH_SCOPE_FILESYSTEM: Self = Self(3);
    pub const SYSTEM_HEALTH_SCOPE_DEVICE: Self = Self(4);
    pub const SYSTEM_HEALTH_SCOPE_COUNT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_capability_t(pub ::core::ffi::c_uint);
impl system_health_capability_t {
    pub const SYSTEM_HEALTH_CAPABILITY_AVAILABLE: Self = Self(0);
    pub const SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED: Self = Self(1);
    pub const SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED: Self = Self(2);
    pub const SYSTEM_HEALTH_CAPABILITY_STALE: Self = Self(3);
    pub const SYSTEM_HEALTH_CAPABILITY_ERROR: Self = Self(4);
    pub const SYSTEM_HEALTH_CAPABILITY_COUNT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_freshness_t(pub ::core::ffi::c_uint);
impl system_health_freshness_t {
    pub const SYSTEM_HEALTH_FRESHNESS_UNKNOWN: Self = Self(0);
    pub const SYSTEM_HEALTH_FRESHNESS_FRESH: Self = Self(1);
    pub const SYSTEM_HEALTH_FRESHNESS_STALE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_unit_t(pub ::core::ffi::c_uint);
impl system_health_unit_t {
    pub const SYSTEM_HEALTH_UNIT_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_UNIT_RATIO: Self = Self(1);
    pub const SYSTEM_HEALTH_UNIT_BYTES: Self = Self(2);
    pub const SYSTEM_HEALTH_UNIT_COUNT: Self = Self(3);
    pub const SYSTEM_HEALTH_UNIT_SECONDS: Self = Self(4);
    pub const SYSTEM_HEALTH_UNIT_CELSIUS: Self = Self(5);
    pub const SYSTEM_HEALTH_UNIT_HERTZ: Self = Self(6);
    pub const SYSTEM_HEALTH_UNIT_BOOLEAN: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_severity_t(pub ::core::ffi::c_uint);
impl system_health_severity_t {
    pub const SYSTEM_HEALTH_SEVERITY_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_SEVERITY_WARNING: Self = Self(1);
    pub const SYSTEM_HEALTH_SEVERITY_ERROR: Self = Self(2);
    pub const SYSTEM_HEALTH_SEVERITY_CRITICAL: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_state_t(pub ::core::ffi::c_uint);
impl system_health_state_t {
    pub const SYSTEM_HEALTH_STATE_UNKNOWN: Self = Self(0);
    pub const SYSTEM_HEALTH_STATE_HEALTHY: Self = Self(1);
    pub const SYSTEM_HEALTH_STATE_PENDING: Self = Self(2);
    pub const SYSTEM_HEALTH_STATE_OPEN: Self = Self(3);
    pub const SYSTEM_HEALTH_STATE_RECOVERING: Self = Self(4);
    pub const SYSTEM_HEALTH_STATE_CLOSED: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_condition_t(pub ::core::ffi::c_uint);
impl system_health_condition_t {
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW: Self = Self(0);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL: Self = Self(1);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_SWAP_THRASH: Self = Self(2);
    pub const SYSTEM_HEALTH_CONDITION_CPU_SATURATION: Self = Self(3);
    pub const SYSTEM_HEALTH_CONDITION_CPU_THROTTLED: Self = Self(4);
    pub const SYSTEM_HEALTH_CONDITION_IO_PRESSURE: Self = Self(5);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW: Self = Self(6);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW: Self = Self(7);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY: Self = Self(8);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED: Self = Self(9);
    pub const SYSTEM_HEALTH_CONDITION_THERMAL_HIGH: Self = Self(10);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN: Self = Self(11);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE: Self = Self(12);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED: Self = Self(13);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_JUMP: Self = Self(14);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION: Self = Self(15);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION: Self = Self(16);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED: Self = Self(17);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL: Self = Self(18);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL: Self = Self(19);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_CORRECTED: Self = Self(20);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE: Self = Self(21);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED: Self = Self(22);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_POWER_UNSTABLE: Self = Self(23);
    pub const SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE: Self = Self(24);
    pub const SYSTEM_HEALTH_CONDITION_UNEXPECTED_RESTART: Self = Self(25);
    pub const SYSTEM_HEALTH_CONDITION_EVENT_DELIVERY_DEGRADED: Self = Self(26);
    pub const SYSTEM_HEALTH_CONDITION_COUNT: Self = Self(27);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_observation_t {
    pub metric: [::core::ffi::c_char; 64],
    pub resource_id: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub capability: system_health_capability_t,
    pub freshness: system_health_freshness_t,
    pub unit: system_health_unit_t,
    pub value_valid: bool,
    pub value: ::core::ffi::c_double,
    pub sampled_monotonic_ms: uint64_t,
    pub observed_wall_time_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_snapshot_t {
    pub observations: [system_health_observation_t; 256],
    pub observation_count: size_t,
    pub observations_dropped: size_t,
    pub sequence: uint64_t,
    pub completed_monotonic_ms: uint64_t,
    pub completed_wall_time_ms: int64_t,
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
pub struct event_severity_t(pub ::core::ffi::c_uint);
impl event_severity_t {
    pub const EVENT_SEVERITY_INFO: Self = Self(0);
    pub const EVENT_SEVERITY_WARNING: Self = Self(1);
    pub const EVENT_SEVERITY_ERROR: Self = Self(2);
    pub const EVENT_SEVERITY_CRITICAL: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_envelope_t {
    pub specversion: [::core::ffi::c_char; 4],
    pub id: [::core::ffi::c_char; 37],
    pub r#type: [::core::ffi::c_char; 96],
    pub source: [::core::ffi::c_char; 128],
    pub subject: [::core::ffi::c_char; 96],
    pub time: [::core::ffi::c_char; 32],
    pub datacontenttype: [::core::ffi::c_char; 32],
    pub occurred_at: time_t,
    pub expires_at: time_t,
    pub severity: event_severity_t,
    pub data: *mut cJSON,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mqtt_presence_state_t(pub ::core::ffi::c_uint);
impl mqtt_presence_state_t {
    pub const MQTT_PRESENCE_OFFLINE: Self = Self(0);
    pub const MQTT_PRESENCE_ONLINE: Self = Self(1);
    pub const MQTT_PRESENCE_STOPPING: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mqtt_operational_state_t(pub ::core::ffi::c_uint);
impl mqtt_operational_state_t {
    pub const MQTT_OPERATIONAL_UNKNOWN: Self = Self(0);
    pub const MQTT_OPERATIONAL_HEALTHY: Self = Self(1);
    pub const MQTT_OPERATIONAL_WARNING: Self = Self(2);
    pub const MQTT_OPERATIONAL_ERROR: Self = Self(3);
    pub const MQTT_OPERATIONAL_CRITICAL: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mqtt_presence_stats_t {
    pub configured: bool,
    pub connected: bool,
    pub sequence: uint64_t,
    pub connections: uint64_t,
    pub reconnects: uint64_t,
    pub disconnects: uint64_t,
    pub publish_attempts: uint64_t,
    pub publish_successes: uint64_t,
    pub publish_failures: uint64_t,
    pub last_publish_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_t {
    pub label: [::core::ffi::c_char; 32],
    pub confidence: ::core::ffi::c_float,
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub width: ::core::ffi::c_float,
    pub height: ::core::ffi::c_float,
    pub track_id: ::core::ffi::c_int,
    pub zone_id: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_result_t {
    pub count: ::core::ffi::c_int,
    pub detections: [detection_t; 20],
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
pub struct system_health_process_run_t {
    pub run_id: [::core::ffi::c_char; 37],
    pub boot_id: [::core::ffi::c_char; 64],
    pub started_at_ms: int64_t,
    pub closed_at_ms: int64_t,
    pub clean_close: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_restart_evidence_t {
    pub capability: system_health_capability_t,
    pub boot_id: [::core::ffi::c_char; 37],
    pub run_id: [::core::ffi::c_char; 37],
    pub host_uptime_seconds: ::core::ffi::c_double,
    pub process_start_monotonic_ms: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct health_entity {
    pub suffix: *const ::core::ffi::c_char,
    pub name: *const ::core::ffi::c_char,
    pub value_template: *const ::core::ffi::c_char,
    pub icon: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct interruptible_sleep_t {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub woken: bool,
    pub clock_id: clockid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct motion_state_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub last_detection_time: time_t,
    pub motion_active: bool,
    pub object_counts: [::core::ffi::c_int; 32],
    pub object_labels: [[::core::ffi::c_char; 32]; 32],
    pub num_labels: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_view_t {
    pub incident_id: [::core::ffi::c_char; 37],
    pub condition: system_health_condition_t,
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub state: system_health_state_t,
    pub severity: system_health_severity_t,
    pub first_observed_at_ms: int64_t,
    pub last_observed_at_ms: int64_t,
    pub observation: system_health_observation_t,
    pub persistence_pending: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mqtt_cleanup_op_t(pub ::core::ffi::c_uint);
impl mqtt_cleanup_op_t {
    pub const MQTT_OP_LOOP_STOP: Self = Self(0);
    pub const MQTT_OP_DESTROY: Self = Self(1);
    pub const MQTT_OP_LIB_CLEANUP: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mqtt_cleanup_arg_t {
    pub mosq: *mut mosquitto,
    pub op: mqtt_cleanup_op_t,
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub done: bool,
    pub timed_out: bool,
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g1_start: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g_size: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __g1_orig_size: 0 as ::core::ffi::c_uint,
        __wrefs: 0 as ::core::ffi::c_uint,
        __g_signals: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __unused_initialized_1: 0 as ::core::ffi::c_uint,
        __unused_initialized_2: 0 as ::core::ffi::c_uint,
    },
};
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const LIGHTNVR_VERSION_STRING: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"0.47.3\0") };
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const MOSQ_LOG_INFO: ::core::ffi::c_int = 1;
pub const MOSQ_LOG_NOTICE: ::core::ffi::c_int = 2;
pub const MOSQ_LOG_WARNING: ::core::ffi::c_int = 4;
pub const MOSQ_LOG_ERR: ::core::ffi::c_int = 8;
pub const SYSTEM_HEALTH_MAX_INCIDENTS: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut mosq: *mut mosquitto = ::core::ptr::null_mut::<mosquitto>();
static mut mqtt_config: *const config_t = ::core::ptr::null::<config_t>();
static mut mqtt_library_initialized: bool = r#false != 0;
static mut connected: bool = r#false != 0;
static mut shutting_down: bool = r#false != 0;
static mut mqtt_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_0::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut publish_ack_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_0::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut publish_ack_wake: pthread_cond_t = PTHREAD_COND_INITIALIZER;
static mut publish_ack_waiting: bool = r#false != 0;
static mut publish_ack_received: bool = r#false != 0;
static mut publish_ack_failed: bool = r#false != 0;
static mut publish_ack_mid: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
static mut ha_services_running: bool = r#false != 0;
static mut ha_snapshot_thread_started: bool = r#false != 0;
static mut ha_snapshot_thread: pthread_t = 0;
static mut ha_motion_thread: pthread_t = 0;
static mut ha_snapshot_wake: interruptible_sleep_t = interruptible_sleep_t {
    mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __glibc_reserved: 0,
            __list: __pthread_list_t {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    cond: pthread_cond_t {
        __data: __pthread_cond_s {
            __wseq: __atomic_wide_counter { __value64: 0 },
            __g1_start: __atomic_wide_counter { __value64: 0 },
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
            __unused_initialized_1: 0,
            __unused_initialized_2: 0,
        },
    },
    woken: false,
    clock_id: 0,
};
static mut ha_motion_wake: interruptible_sleep_t = interruptible_sleep_t {
    mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __glibc_reserved: 0,
            __list: __pthread_list_t {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    cond: pthread_cond_t {
        __data: __pthread_cond_s {
            __wseq: __atomic_wide_counter { __value64: 0 },
            __g1_start: __atomic_wide_counter { __value64: 0 },
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
            __unused_initialized_1: 0,
            __unused_initialized_2: 0,
        },
    },
    woken: false,
    clock_id: 0,
};
static mut presence_running: bool = r#false != 0;
static mut presence_thread_started: bool = r#false != 0;
static mut presence_thread: pthread_t = 0;
static mut presence_wake: interruptible_sleep_t = interruptible_sleep_t {
    mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __glibc_reserved: 0,
            __list: __pthread_list_t {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    cond: pthread_cond_t {
        __data: __pthread_cond_s {
            __wseq: __atomic_wide_counter { __value64: 0 },
            __g1_start: __atomic_wide_counter { __value64: 0 },
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
            __unused_initialized_1: 0,
            __unused_initialized_2: 0,
        },
    },
    woken: false,
    clock_id: 0,
};
pub const MAX_MOTION_STREAMS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MOTION_OFF_DELAY_SEC: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
static mut motion_states: [motion_state_t; 16] = [motion_state_t {
    stream_name: [0; 256],
    last_detection_time: 0,
    motion_active: false,
    object_counts: [0; 32],
    object_labels: [[0; 32]; 32],
    num_labels: 0,
}; 16];
static mut num_motion_states: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut motion_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_0::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
unsafe extern "C" fn mqtt_wall_time_ms() -> int64_t {
    let mut value: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_REALTIME, &raw mut value) != 0 as ::core::ffi::c_int {
        return 0 as int64_t;
    }
    return value.tv_sec as int64_t * 1000 as int64_t
        + value.tv_nsec as int64_t / 1000000 as int64_t;
}
unsafe extern "C" fn current_operational_state(
    mut active_count: *mut size_t,
) -> mqtt_operational_state_t {
    if !active_count.is_null() {
        *active_count = 0 as size_t;
    }
    let mut snapshot: system_health_snapshot_t = system_health_snapshot_t {
        observations: [system_health_observation_t {
            metric: [0; 64],
            resource_id: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            freshness: system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_UNKNOWN,
            unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
            value_valid: false,
            value: 0.,
            sampled_monotonic_ms: 0,
            observed_wall_time_ms: 0,
        }; 256],
        observation_count: 0,
        observations_dropped: 0,
        sequence: 0,
        completed_monotonic_ms: 0,
        completed_wall_time_ms: 0,
    };
    let mut baseline_known: bool =
        system_health_snapshot_copy(&raw mut snapshot) as ::core::ffi::c_int != 0
            && snapshot.sequence > 0 as uint64_t
            && snapshot.observation_count > 0 as size_t;
    let mut incidents: [system_health_incident_view_t; 64] = [system_health_incident_view_t {
        incident_id: [0; 37],
        condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
        subject: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
        severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        first_observed_at_ms: 0,
        last_observed_at_ms: 0,
        observation: system_health_observation_t {
            metric: [0; 64],
            resource_id: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            freshness: system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_UNKNOWN,
            unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
            value_valid: false,
            value: 0.,
            sampled_monotonic_ms: 0,
            observed_wall_time_ms: 0,
        },
        persistence_pending: false,
    }; 64];
    let mut count: size_t = system_health_evaluator_service_active_copy(
        &raw mut incidents as *mut system_health_incident_view_t,
        SYSTEM_HEALTH_MAX_INCIDENTS as size_t,
    );
    if !active_count.is_null() {
        *active_count = count;
    }
    let mut maximum: system_health_severity_t =
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE;
    let mut index: size_t = 0 as size_t;
    while index < count {
        if incidents[index].severity.0 > maximum.0 {
            maximum = incidents[index].severity;
        }
        index = index.wrapping_add(1);
    }
    if maximum.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0 {
        return mqtt_operational_state_t::MQTT_OPERATIONAL_CRITICAL;
    }
    if maximum.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_ERROR.0 {
        return mqtt_operational_state_t::MQTT_OPERATIONAL_ERROR;
    }
    if maximum.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0 {
        return mqtt_operational_state_t::MQTT_OPERATIONAL_WARNING;
    }
    return mqtt_operational_state_t(
        (if baseline_known as ::core::ffi::c_int != 0 {
            mqtt_operational_state_t::MQTT_OPERATIONAL_HEALTHY.0 as ::core::ffi::c_int
        } else {
            mqtt_operational_state_t::MQTT_OPERATIONAL_UNKNOWN.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn publish_presence(mut state: mqtt_presence_state_t) -> ::core::ffi::c_int {
    let mut active_count: size_t = 0 as size_t;
    let mut operational: mqtt_operational_state_t =
        current_operational_state(&raw mut active_count);
    let mut topic: [::core::ffi::c_char; 512] = [0; 512];
    let mut payload: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut now: int64_t = mqtt_wall_time_ms();
    if mqtt_presence_build(
        state,
        operational,
        active_count,
        now,
        &raw mut topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut payload as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = mqtt_publish_raw(
        &raw mut topic as *mut ::core::ffi::c_char,
        &raw mut payload as *mut ::core::ffi::c_char,
        r#true != 0,
    );
    mqtt_presence_record_publish(result == 0 as ::core::ffi::c_int, now);
    return result;
}
unsafe extern "C" fn presence_thread_main(
    mut argument: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    while ::core::ptr::read_volatile::<bool>(&raw const presence_running) {
        let mut interval: uint32_t = if !mqtt_config.is_null() {
            (*mqtt_config).health.presence_interval_seconds
        } else {
            60 as uint32_t
        };
        if interval == 0 as uint32_t {
            interval = 60 as ::core::ffi::c_uint as uint32_t;
        }
        interruptible_sleep_wait(&raw mut presence_wake, interval as ::core::ffi::c_int);
        if !::core::ptr::read_volatile::<bool>(&raw const presence_running) {
            break;
        }
        if mqtt_is_connected() {
            publish_presence(mqtt_presence_state_t::MQTT_PRESENCE_ONLINE);
        }
    }
    return NULL;
}
unsafe extern "C" fn mqtt_start_presence_service() -> ::core::ffi::c_int {
    if presence_thread_started {
        return 0 as ::core::ffi::c_int;
    }
    if interruptible_sleep_init(&raw mut presence_wake) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    ::core::ptr::write_volatile(&raw mut presence_running, r#true != 0);
    if pthread_create(
        &raw mut presence_thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            presence_thread_main
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    ) != 0 as ::core::ffi::c_int
    {
        ::core::ptr::write_volatile(&raw mut presence_running, r#false != 0);
        interruptible_sleep_destroy(&raw mut presence_wake);
        return -1 as ::core::ffi::c_int;
    }
    presence_thread_started = r#true != 0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn mqtt_stop_presence_service(mut publish_stopping: bool) {
    if !presence_thread_started {
        return;
    }
    ::core::ptr::write_volatile(&raw mut presence_running, r#false != 0);
    interruptible_sleep_wake(&raw mut presence_wake);
    pthread_join(
        presence_thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
    presence_thread_started = r#false != 0;
    interruptible_sleep_destroy(&raw mut presence_wake);
    if publish_stopping as ::core::ffi::c_int != 0 && mqtt_is_connected() as ::core::ffi::c_int != 0
    {
        publish_presence(mqtt_presence_state_t::MQTT_PRESENCE_STOPPING);
    }
}
unsafe extern "C" fn configure_presence_will(mut config: *const config_t) -> ::core::ffi::c_int {
    let mut source: [::core::ffi::c_char; 128] = [0; 128];
    static mut urn_prefix: [::core::ffi::c_char; 14] = unsafe {
        ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"urn:lightnvr:\0")
    };
    if event_identity_get_source(
        &raw mut source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    ) != 0 as ::core::ffi::c_int
        || strncmp(
            &raw mut source as *mut ::core::ffi::c_char,
            &raw const urn_prefix as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 14]>().wrapping_sub(1 as size_t),
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut installation_uuid: *const ::core::ffi::c_char = (&raw mut source
        as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<[::core::ffi::c_char; 14]>() as isize)
        .offset(-(1 as ::core::ffi::c_uint as isize));
    let mut run: system_health_process_run_t = system_health_process_run_t {
        run_id: [0; 37],
        boot_id: [0; 64],
        started_at_ms: 0,
        closed_at_ms: 0,
        clean_close: false,
    };
    memset(
        &raw mut run as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_process_run_t>(),
    );
    if !system_health_evaluator_service_copy_run(&raw mut run) {
        if lightnvr_uuid_generate_v4(&raw mut run.run_id as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut evidence: linux_restart_evidence_t = linux_restart_evidence_t {
            capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            boot_id: [0; 37],
            run_id: [0; 37],
            host_uptime_seconds: 0.,
            process_start_monotonic_ms: 0,
        };
        if linux_restart_read_evidence(
            b"/proc\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut run.run_id as *mut ::core::ffi::c_char,
            1 as uint64_t,
            &raw mut evidence,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        snprintf(
            &raw mut run.boot_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut evidence.boot_id as *mut ::core::ffi::c_char,
        );
    }
    if mqtt_presence_configure(
        &raw const (*config).mqtt_topic_prefix as *const ::core::ffi::c_char,
        installation_uuid,
        &raw mut run.run_id as *mut ::core::ffi::c_char,
        &raw mut run.boot_id as *mut ::core::ffi::c_char,
        b"process,container,host,filesystem\0".as_ptr() as *const ::core::ffi::c_char,
        LIGHTNVR_VERSION_STRING.as_ptr(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut topic: [::core::ffi::c_char; 512] = [0; 512];
    let mut payload: [::core::ffi::c_char; 1024] = [0; 1024];
    if mqtt_presence_build_will(
        mqtt_wall_time_ms(),
        &raw mut topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut payload as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = mosquitto_will_set(
        mosq,
        &raw mut topic as *mut ::core::ffi::c_char,
        strlen(&raw mut payload as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
        &raw mut payload as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        (*config).mqtt_qos,
        r#true != 0,
    );
    if result != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to configure presence LWT: %s\0".as_ptr() as *const ::core::ffi::c_char,
            mosquitto_strerror(result),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Presence LWT configured on %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut topic as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_init(mut config: *const config_t) -> ::core::ffi::c_int {
    if config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Invalid config pointer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut mqtt_mutex);
    let mut rc: ::core::ffi::c_int = mosq_err_t::MOSQ_ERR_SUCCESS.0;
    if !mqtt_library_initialized {
        rc = mosquitto_lib_init();
        if rc != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT: Failed to initialize mosquitto library: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mosquitto_strerror(rc),
            );
            pthread_mutex_unlock(&raw mut mqtt_mutex);
            return -1 as ::core::ffi::c_int;
        }
        mqtt_library_initialized = r#true != 0;
    }
    mqtt_config = config;
    if !(*config).mqtt_enabled {
        pthread_mutex_unlock(&raw mut mqtt_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Default broker disabled; managed destinations remain available\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (*config).mqtt_broker_host[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Broker host not configured\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut mqtt_mutex);
        return -1 as ::core::ffi::c_int;
    }
    mosq = mosquitto_new(
        &raw const (*config).mqtt_client_id as *const ::core::ffi::c_char,
        r#true != 0,
        NULL,
    );
    if mosq.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to create mosquitto client\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut mqtt_mutex);
        return -1 as ::core::ffi::c_int;
    }
    mosquitto_connect_callback_set(
        mosq,
        Some(
            on_connect
                as unsafe extern "C" fn(
                    *mut mosquitto,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    mosquitto_disconnect_callback_set(
        mosq,
        Some(
            on_disconnect
                as unsafe extern "C" fn(
                    *mut mosquitto,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    mosquitto_message_callback_set(
        mosq,
        Some(
            on_message
                as unsafe extern "C" fn(
                    *mut mosquitto,
                    *mut ::core::ffi::c_void,
                    *const mosquitto_message,
                ) -> (),
        ),
    );
    mosquitto_publish_callback_set(
        mosq,
        Some(
            on_publish
                as unsafe extern "C" fn(
                    *mut mosquitto,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    mosquitto_log_callback_set(
        mosq,
        Some(
            on_log
                as unsafe extern "C" fn(
                    *mut mosquitto,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                ) -> (),
        ),
    );
    if (*config).mqtt_username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        rc = mosquitto_username_pw_set(
            mosq,
            &raw const (*config).mqtt_username as *const ::core::ffi::c_char,
            if (*config).mqtt_password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                &raw const (*config).mqtt_password as *const ::core::ffi::c_char
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            },
        );
        if rc != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT: Failed to set credentials: %s\0".as_ptr() as *const ::core::ffi::c_char,
                mosquitto_strerror(rc),
            );
            mosquitto_destroy(mosq);
            mosq = ::core::ptr::null_mut::<mosquitto>();
            pthread_mutex_unlock(&raw mut mqtt_mutex);
            return -1 as ::core::ffi::c_int;
        }
    }
    if (*config).mqtt_tls_enabled {
        rc = mosquitto_int_option(
            mosq,
            mosq_opt_t::MOSQ_OPT_TLS_USE_OS_CERTS,
            1 as ::core::ffi::c_int,
        );
        if rc != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT: Failed to enable TLS: %s\0".as_ptr() as *const ::core::ffi::c_char,
                mosquitto_strerror(rc),
            );
            mosquitto_destroy(mosq);
            mosq = ::core::ptr::null_mut::<mosquitto>();
            pthread_mutex_unlock(&raw mut mqtt_mutex);
            return -1 as ::core::ffi::c_int;
        }
    }
    if configure_presence_will(config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: General installation presence is unavailable\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    pthread_mutex_unlock(&raw mut mqtt_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Client initialized (broker: %s:%d, client_id: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*config).mqtt_broker_host as *const ::core::ffi::c_char,
        (*config).mqtt_broker_port,
        &raw const (*config).mqtt_client_id as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_connect() -> ::core::ffi::c_int {
    if mosq.is_null() || mqtt_config.is_null() || !(*mqtt_config).mqtt_enabled {
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut mqtt_mutex);
    let mut rc: ::core::ffi::c_int = mosquitto_connect(
        mosq,
        &raw const (*mqtt_config).mqtt_broker_host as *const ::core::ffi::c_char,
        (*mqtt_config).mqtt_broker_port,
        (*mqtt_config).mqtt_keepalive,
    );
    if rc != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to connect to broker: %s\0".as_ptr() as *const ::core::ffi::c_char,
            mosquitto_strerror(rc),
        );
        pthread_mutex_unlock(&raw mut mqtt_mutex);
        return -1 as ::core::ffi::c_int;
    }
    rc = mosquitto_loop_start(mosq);
    if rc != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to start loop: %s\0".as_ptr() as *const ::core::ffi::c_char,
            mosquitto_strerror(rc),
        );
        mosquitto_disconnect(mosq);
        pthread_mutex_unlock(&raw mut mqtt_mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut mqtt_mutex);
    if mqtt_start_presence_service() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to start presence heartbeat worker\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Connecting to broker %s:%d...\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*mqtt_config).mqtt_broker_host as *const ::core::ffi::c_char,
        (*mqtt_config).mqtt_broker_port,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_is_connected() -> bool {
    pthread_mutex_lock(&raw mut mqtt_mutex);
    let mut result: bool = connected;
    pthread_mutex_unlock(&raw mut mqtt_mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_get_presence_stats(mut stats: *mut mqtt_presence_stats_t) {
    mqtt_presence_get_stats(stats);
}
unsafe extern "C" fn on_connect(
    mut m: *mut mosquitto,
    mut userdata: *mut ::core::ffi::c_void,
    mut rc: ::core::ffi::c_int,
) {
    if ::core::ptr::read_volatile::<bool>(&raw const shutting_down) {
        return;
    }
    pthread_mutex_lock(&raw mut mqtt_mutex);
    if ::core::ptr::read_volatile::<bool>(&raw const shutting_down) {
        pthread_mutex_unlock(&raw mut mqtt_mutex);
        return;
    }
    if rc == 0 as ::core::ffi::c_int {
        connected = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Connected to broker successfully\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut mqtt_mutex);
        mqtt_presence_record_connected();
        publish_presence(mqtt_presence_state_t::MQTT_PRESENCE_ONLINE);
        if !mqtt_config.is_null() && (*mqtt_config).mqtt_ha_discovery as ::core::ffi::c_int != 0 {
            let mut avail_topic: [::core::ffi::c_char; 512] = [0; 512];
            snprintf(
                &raw mut avail_topic as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                b"%s/availability\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*mqtt_config).mqtt_topic_prefix as *const ::core::ffi::c_char,
            );
            mqtt_publish_raw(
                &raw mut avail_topic as *mut ::core::ffi::c_char,
                b"online\0".as_ptr() as *const ::core::ffi::c_char,
                r#true != 0,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT: Published availability 'online' to %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut avail_topic as *mut ::core::ffi::c_char,
            );
            let mut status_topic: [::core::ffi::c_char; 512] = [0; 512];
            snprintf(
                &raw mut status_topic as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                b"%s/status\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*mqtt_config).mqtt_ha_discovery_prefix as *const ::core::ffi::c_char,
            );
            let mut sub_rc: ::core::ffi::c_int = mosquitto_subscribe(
                m,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                &raw mut status_topic as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            if sub_rc == mosq_err_t::MOSQ_ERR_SUCCESS.0 {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"MQTT: Subscribed to HA birth topic %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut status_topic as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"MQTT: Failed to subscribe to HA birth topic: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    mosquitto_strerror(sub_rc),
                );
            }
            mqtt_publish_ha_discovery();
            mqtt_start_ha_services();
        }
    } else {
        connected = r#false != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Connection failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            mosquitto_connack_string(rc),
        );
        pthread_mutex_unlock(&raw mut mqtt_mutex);
    };
}
unsafe extern "C" fn on_disconnect(
    mut m: *mut mosquitto,
    mut userdata: *mut ::core::ffi::c_void,
    mut rc: ::core::ffi::c_int,
) {
    if ::core::ptr::read_volatile::<bool>(&raw const shutting_down) {
        connected = r#false != 0;
        mqtt_presence_record_disconnected();
        pthread_mutex_lock(&raw mut publish_ack_mutex);
        if publish_ack_waiting {
            publish_ack_failed = r#true != 0;
            pthread_cond_broadcast(&raw mut publish_ack_wake);
        }
        pthread_mutex_unlock(&raw mut publish_ack_mutex);
        if rc == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT: Disconnected from broker (shutdown)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    pthread_mutex_lock(&raw mut mqtt_mutex);
    if ::core::ptr::read_volatile::<bool>(&raw const shutting_down) {
        pthread_mutex_unlock(&raw mut mqtt_mutex);
        connected = r#false != 0;
        mqtt_presence_record_disconnected();
        return;
    }
    connected = r#false != 0;
    pthread_mutex_unlock(&raw mut mqtt_mutex);
    mqtt_presence_record_disconnected();
    pthread_mutex_lock(&raw mut publish_ack_mutex);
    if publish_ack_waiting {
        publish_ack_failed = r#true != 0;
        pthread_cond_broadcast(&raw mut publish_ack_wake);
    }
    pthread_mutex_unlock(&raw mut publish_ack_mutex);
    if rc == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Disconnected from broker\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Unexpected disconnection (rc=%d), will attempt reconnect\0".as_ptr()
                as *const ::core::ffi::c_char,
            rc,
        );
    };
}
unsafe extern "C" fn on_publish(
    mut m: *mut mosquitto,
    mut userdata: *mut ::core::ffi::c_void,
    mut mid: ::core::ffi::c_int,
) {
    pthread_mutex_lock(&raw mut publish_ack_mutex);
    if publish_ack_waiting as ::core::ffi::c_int != 0 && publish_ack_mid == mid {
        publish_ack_received = r#true != 0;
        pthread_cond_broadcast(&raw mut publish_ack_wake);
    }
    pthread_mutex_unlock(&raw mut publish_ack_mutex);
}
unsafe extern "C" fn on_message(
    mut m: *mut mosquitto,
    mut userdata: *mut ::core::ffi::c_void,
    mut msg: *const mosquitto_message,
) {
    if msg.is_null()
        || (*msg).topic.is_null()
        || (*msg).payload.is_null()
        || ::core::ptr::read_volatile::<bool>(&raw const shutting_down) as ::core::ffi::c_int != 0
    {
        return;
    }
    if !mqtt_config.is_null() && (*mqtt_config).mqtt_ha_discovery as ::core::ffi::c_int != 0 {
        let mut status_topic: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut status_topic as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/status\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*mqtt_config).mqtt_ha_discovery_prefix as *const ::core::ffi::c_char,
        );
        if strcmp(
            (*msg).topic,
            &raw mut status_topic as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut payload_str: [::core::ffi::c_char; 64] = [0; 64];
            let mut len: ::core::ffi::c_int = if (*msg).payloadlen
                < ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int
            {
                (*msg).payloadlen
            } else {
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int
            };
            memcpy(
                &raw mut payload_str as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (*msg).payload,
                len as size_t,
            );
            payload_str[len as usize] = '\0' as ::core::ffi::c_char;
            if strcmp(
                &raw mut payload_str as *mut ::core::ffi::c_char,
                b"online\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"MQTT HA: Home Assistant birth message received, re-publishing discovery\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                mqtt_publish_ha_discovery();
            }
        }
    }
}
unsafe extern "C" fn on_log(
    mut m: *mut mosquitto,
    mut userdata: *mut ::core::ffi::c_void,
    mut level: ::core::ffi::c_int,
    mut str: *const ::core::ffi::c_char,
) {
    match level {
        MOSQ_LOG_ERR => {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT: %s\0".as_ptr() as *const ::core::ffi::c_char,
                str,
            );
        }
        MOSQ_LOG_WARNING => {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT: %s\0".as_ptr() as *const ::core::ffi::c_char,
                str,
            );
        }
        MOSQ_LOG_INFO | MOSQ_LOG_NOTICE => {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT: %s\0".as_ptr() as *const ::core::ffi::c_char,
                str,
            );
        }
        _ => {}
    };
}
pub const MAX_DETECTION_SNAPSHOTS_PER_STREAM: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
unsafe extern "C" fn prune_detection_snapshots(mut dir_path: *const ::core::ffi::c_char) {
    let mut dir: *mut DIR = opendir(dir_path);
    if dir.is_null() {
        return;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ent: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        ent = readdir(dir);
        if ent.is_null() {
            break;
        }
        if (*ent).d_name[0usize] as ::core::ffi::c_int != '.' as ::core::ffi::c_int
            && !strstr(
                &raw mut (*ent).d_name as *mut ::core::ffi::c_char,
                b".jpg\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            count += 1;
        }
    }
    while count >= MAX_DETECTION_SNAPSHOTS_PER_STREAM {
        rewinddir(dir);
        let mut oldest: [::core::ffi::c_char; 256] = ::core::mem::transmute::<
            [u8; 256],
            [::core::ffi::c_char; 256],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        loop {
            ent = readdir(dir);
            if ent.is_null() {
                break;
            }
            if (*ent).d_name[0usize] as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                || strstr(
                    &raw mut (*ent).d_name as *mut ::core::ffi::c_char,
                    b".jpg\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
            {
                continue;
            }
            if oldest[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || strcmp(
                    &raw mut (*ent).d_name as *mut ::core::ffi::c_char,
                    &raw mut oldest as *mut ::core::ffi::c_char,
                ) < 0 as ::core::ffi::c_int
            {
                safe_strcpy(
                    &raw mut oldest as *mut ::core::ffi::c_char,
                    &raw mut (*ent).d_name as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    0 as size_t,
                );
            }
        }
        if oldest[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        let mut victim: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut victim as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            dir_path,
            &raw mut oldest as *mut ::core::ffi::c_char,
        );
        if unlink(&raw mut victim as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
            break;
        }
        count -= 1;
    }
    closedir(dir);
}
unsafe extern "C" fn save_detection_snapshot(
    mut safe_name: *const ::core::ffi::c_char,
    mut timestamp: time_t,
    mut jpeg_data: *const ::core::ffi::c_uchar,
    mut jpeg_size: size_t,
    mut out_path: *mut ::core::ffi::c_char,
    mut out_path_size: size_t,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_url_size: size_t,
) -> ::core::ffi::c_int {
    if mqtt_config.is_null()
        || (*mqtt_config).storage_path[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut dir_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut dir_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/snapshots/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*mqtt_config).storage_path as *const ::core::ffi::c_char,
        safe_name,
    );
    if ensure_dir(&raw mut dir_path as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to create snapshot directory %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut dir_path as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    prune_detection_snapshots(&raw mut dir_path as *mut ::core::ffi::c_char);
    let mut ts_str: [::core::ffi::c_char; 32] = [0; 32];
    let mut tm_buf: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    localtime_r(&raw mut timestamp, &raw mut tm_buf);
    strftime(
        &raw mut ts_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%Y%m%d_%H%M%S\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut tm_buf,
    );
    let mut file_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut file_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/%s.jpg\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut dir_path as *mut ::core::ffi::c_char,
        &raw mut ts_str as *mut ::core::ffi::c_char,
    );
    let mut tmp_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut tmp_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/.%s.jpg.tmp\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut dir_path as *mut ::core::ffi::c_char,
        &raw mut ts_str as *mut ::core::ffi::c_char,
    );
    let mut fd: ::core::ffi::c_int = open(
        &raw mut tmp_path as *mut ::core::ffi::c_char,
        O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC,
        0o644 as ::core::ffi::c_int,
    );
    if fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to open snapshot file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut tmp_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut f: *mut FILE = fdopen(fd, b"wb\0".as_ptr() as *const ::core::ffi::c_char);
    if f.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to open snapshot file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut tmp_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(fd);
        unlink(&raw mut tmp_path as *mut ::core::ffi::c_char);
        return -1 as ::core::ffi::c_int;
    }
    let mut written: size_t = fwrite(
        jpeg_data as *const ::core::ffi::c_void,
        1 as size_t,
        jpeg_size,
        f,
    ) as size_t;
    fclose(f);
    if written != jpeg_size
        || rename(
            &raw mut tmp_path as *mut ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to write snapshot %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut file_path as *mut ::core::ffi::c_char,
        );
        unlink(&raw mut tmp_path as *mut ::core::ffi::c_char);
        return -1 as ::core::ffi::c_int;
    }
    snprintf(
        out_path,
        out_path_size,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut file_path as *mut ::core::ffi::c_char,
    );
    snprintf(
        out_url,
        out_url_size,
        b"/api/snapshots/%s/%s.jpg\0".as_ptr() as *const ::core::ffi::c_char,
        safe_name,
        &raw mut ts_str as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_publish_detection(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
    mut timestamp: time_t,
) -> ::core::ffi::c_int {
    if mosq.is_null() || mqtt_config.is_null() || !(*mqtt_config).mqtt_enabled {
        return 0 as ::core::ffi::c_int;
    }
    if stream_name.is_null() || result.is_null() || (*result).count == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if !mqtt_is_connected() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Not connected, skipping detection publish\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut safe_name: [::core::ffi::c_char; 256] = [0; 256];
    sanitize_stream_name(
        stream_name,
        &raw mut safe_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    let mut topic: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/detections/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*mqtt_config).mqtt_topic_prefix as *const ::core::ffi::c_char,
        &raw mut safe_name as *mut ::core::ffi::c_char,
    );
    let mut snapshot_path: [::core::ffi::c_char; 512] = ::core::mem::transmute::<
        [u8; 512],
        [::core::ffi::c_char; 512],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut snapshot_url: [::core::ffi::c_char; 512] = ::core::mem::transmute::<
        [u8; 512],
        [::core::ffi::c_char; 512],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut snapshot_topic: [::core::ffi::c_char; 512] = ::core::mem::transmute::<
        [u8; 512],
        [::core::ffi::c_char; 512],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut snapshot_camera: stream_config_t = stream_config_t {
        camera_uuid: [0; 37],
        location_uuid: [0; 37],
        name: [0; 256],
        url: [0; 512],
        enabled: false,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        priority: 0,
        record: false,
        segment_duration: 0,
        detection_based_recording: false,
        detection_model: [0; 512],
        detection_interval: 0,
        detection_threshold: 0.,
        pre_detection_buffer: 0,
        post_detection_buffer: 0,
        detection_api_url: [0; 512],
        detection_object_filter: [0; 16],
        detection_object_filter_list: [0; 256],
        streaming_enabled: false,
        protocol: stream_protocol_t::STREAM_PROTOCOL_TCP,
        record_audio: false,
        audio_voice_enhancement: false,
        onvif_username: [0; 64],
        onvif_password: [0; 64],
        onvif_profile: [0; 64],
        onvif_port: 0,
        onvif_discovery_enabled: false,
        is_onvif: false,
        backchannel_enabled: false,
        retention_days: 0,
        detection_retention_days: 0,
        max_storage_mb: 0,
        tier_critical_multiplier: 0.,
        tier_important_multiplier: 0.,
        tier_ephemeral_multiplier: 0.,
        storage_priority: 0,
        ptz_enabled: false,
        ptz_max_x: 0,
        ptz_max_y: 0,
        ptz_max_z: 0,
        ptz_has_home: false,
        record_on_schedule: false,
        recording_schedule: [0; 168],
        detection_record_on_schedule: false,
        detection_recording_schedule: [0; 168],
        tags: [0; 256],
        admin_url: [0; 512],
        privacy_mode: false,
        motion_trigger_source: [0; 256],
        go2rtc_source_override: [0; 8192],
        sub_stream_url: [0; 512],
        detection_url: [0; 512],
        publish_url: [0; 512],
        playback_transport: [0; 24],
        eptz_config: [0; 1024],
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    let snapshot_identity_valid: bool =
        get_stream_config_by_name(stream_name, &raw mut snapshot_camera) == 0 as ::core::ffi::c_int
            && lightnvr_uuid_is_valid(
                &raw mut snapshot_camera.camera_uuid as *mut ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0;
    let mut jpeg_data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut jpeg_size: size_t = 0 as size_t;
    if snapshot_identity_valid as ::core::ffi::c_int != 0
        && go2rtc_get_snapshot(stream_name, &raw mut jpeg_data, &raw mut jpeg_size)
            as ::core::ffi::c_int
            != 0
        && !jpeg_data.is_null()
        && jpeg_size > 0 as size_t
    {
        let mut current_camera: stream_config_t = stream_config_t {
            camera_uuid: [0; 37],
            location_uuid: [0; 37],
            name: [0; 256],
            url: [0; 512],
            enabled: false,
            width: 0,
            height: 0,
            fps: 0,
            codec: [0; 16],
            priority: 0,
            record: false,
            segment_duration: 0,
            detection_based_recording: false,
            detection_model: [0; 512],
            detection_interval: 0,
            detection_threshold: 0.,
            pre_detection_buffer: 0,
            post_detection_buffer: 0,
            detection_api_url: [0; 512],
            detection_object_filter: [0; 16],
            detection_object_filter_list: [0; 256],
            streaming_enabled: false,
            protocol: stream_protocol_t::STREAM_PROTOCOL_TCP,
            record_audio: false,
            audio_voice_enhancement: false,
            onvif_username: [0; 64],
            onvif_password: [0; 64],
            onvif_profile: [0; 64],
            onvif_port: 0,
            onvif_discovery_enabled: false,
            is_onvif: false,
            backchannel_enabled: false,
            retention_days: 0,
            detection_retention_days: 0,
            max_storage_mb: 0,
            tier_critical_multiplier: 0.,
            tier_important_multiplier: 0.,
            tier_ephemeral_multiplier: 0.,
            storage_priority: 0,
            ptz_enabled: false,
            ptz_max_x: 0,
            ptz_max_y: 0,
            ptz_max_z: 0,
            ptz_has_home: false,
            record_on_schedule: false,
            recording_schedule: [0; 168],
            detection_record_on_schedule: false,
            detection_recording_schedule: [0; 168],
            tags: [0; 256],
            admin_url: [0; 512],
            privacy_mode: false,
            motion_trigger_source: [0; 256],
            go2rtc_source_override: [0; 8192],
            sub_stream_url: [0; 512],
            detection_url: [0; 512],
            publish_url: [0; 512],
            playback_transport: [0; 24],
            eptz_config: [0; 1024],
            observation_evidence_enabled: false,
            observation_labels: [0; 512],
            observation_max_age_days: 0,
            observation_max_bytes: 0,
        };
        if get_stream_config_by_name(stream_name, &raw mut current_camera)
            == 0 as ::core::ffi::c_int
            && strcmp(
                &raw mut snapshot_camera.camera_uuid as *mut ::core::ffi::c_char,
                &raw mut current_camera.camera_uuid as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            save_detection_snapshot(
                &raw mut snapshot_camera.camera_uuid as *mut ::core::ffi::c_char,
                timestamp,
                jpeg_data,
                jpeg_size,
                &raw mut snapshot_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                &raw mut snapshot_url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            );
            snprintf(
                &raw mut snapshot_topic as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                b"%s/detections/%s/snapshot\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*mqtt_config).mqtt_topic_prefix as *const ::core::ffi::c_char,
                &raw mut safe_name as *mut ::core::ffi::c_char,
            );
            if mqtt_publish_binary(
                &raw mut snapshot_topic as *mut ::core::ffi::c_char,
                jpeg_data as *const ::core::ffi::c_void,
                jpeg_size,
                (*mqtt_config).mqtt_retain,
            ) != 0 as ::core::ffi::c_int
            {
                snapshot_topic[0usize] = '\0' as ::core::ffi::c_char;
            }
        }
    }
    free(jpeg_data as *mut ::core::ffi::c_void);
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to create JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    cJSON_AddStringToObject(
        root,
        b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    cJSON_AddNumberToObject(
        root,
        b"timestamp\0".as_ptr() as *const ::core::ffi::c_char,
        timestamp as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        (*result).count as ::core::ffi::c_double,
    );
    if snapshot_path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        cJSON_AddStringToObject(
            root,
            b"snapshot_path\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut snapshot_path as *mut ::core::ffi::c_char,
        );
    }
    if snapshot_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        cJSON_AddStringToObject(
            root,
            b"snapshot_url\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut snapshot_url as *mut ::core::ffi::c_char,
        );
    }
    if snapshot_topic[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        cJSON_AddStringToObject(
            root,
            b"snapshot_topic\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut snapshot_topic as *mut ::core::ffi::c_char,
        );
    }
    let mut detections: *mut cJSON = cJSON_CreateArray();
    if detections.is_null() {
        cJSON_Delete(root);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to create JSON array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*result).count && i < MAX_DETECTIONS {
        let mut det: *mut cJSON = cJSON_CreateObject();
        if !det.is_null() {
            cJSON_AddStringToObject(
                det,
                b"label\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*(&raw const (*result).detections as *const detection_t)
                    .offset(i as isize))
                .label as *const ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                det,
                b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
                (*result).detections[i as usize].confidence as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                det,
                b"x\0".as_ptr() as *const ::core::ffi::c_char,
                (*result).detections[i as usize].x as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                det,
                b"y\0".as_ptr() as *const ::core::ffi::c_char,
                (*result).detections[i as usize].y as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                det,
                b"width\0".as_ptr() as *const ::core::ffi::c_char,
                (*result).detections[i as usize].width as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                det,
                b"height\0".as_ptr() as *const ::core::ffi::c_char,
                (*result).detections[i as usize].height as ::core::ffi::c_double,
            );
            if (*result).detections[i as usize].track_id >= 0 as ::core::ffi::c_int {
                cJSON_AddNumberToObject(
                    det,
                    b"track_id\0".as_ptr() as *const ::core::ffi::c_char,
                    (*result).detections[i as usize].track_id as ::core::ffi::c_double,
                );
            }
            if (*result).detections[i as usize].zone_id[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                cJSON_AddStringToObject(
                    det,
                    b"zone_id\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*(&raw const (*result).detections as *const detection_t)
                        .offset(i as isize))
                    .zone_id as *const ::core::ffi::c_char,
                );
            }
            cJSON_AddItemToArray(detections, det);
        }
        i += 1;
    }
    cJSON_AddItemToObject(
        root,
        b"detections\0".as_ptr() as *const ::core::ffi::c_char,
        detections,
    );
    let mut payload: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if payload.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to serialize JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut mqtt_mutex);
    let mut rc: ::core::ffi::c_int = mosquitto_publish(
        mosq,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut topic as *mut ::core::ffi::c_char,
        strlen(payload) as ::core::ffi::c_int,
        payload as *const ::core::ffi::c_void,
        (*mqtt_config).mqtt_qos,
        (*mqtt_config).mqtt_retain,
    );
    pthread_mutex_unlock(&raw mut mqtt_mutex);
    free(payload as *mut ::core::ffi::c_void);
    if rc != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to publish detection: %s\0".as_ptr() as *const ::core::ffi::c_char,
            mosquitto_strerror(rc),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Published %d detection(s) to %s\0".as_ptr() as *const ::core::ffi::c_char,
        (*result).count,
        &raw mut topic as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_publish_raw(
    mut topic: *const ::core::ffi::c_char,
    mut payload: *const ::core::ffi::c_char,
    mut retain: bool,
) -> ::core::ffi::c_int {
    if mosq.is_null() || mqtt_config.is_null() || !(*mqtt_config).mqtt_enabled {
        return 0 as ::core::ffi::c_int;
    }
    if topic.is_null() || payload.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if !mqtt_is_connected() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut mqtt_mutex);
    let mut rc: ::core::ffi::c_int = mosquitto_publish(
        mosq,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        topic,
        strlen(payload) as ::core::ffi::c_int,
        payload as *const ::core::ffi::c_void,
        (*mqtt_config).mqtt_qos,
        retain,
    );
    pthread_mutex_unlock(&raw mut mqtt_mutex);
    if rc != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to publish to %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            topic,
            mosquitto_strerror(rc),
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_publish_raw_confirmed(
    mut topic: *const ::core::ffi::c_char,
    mut payload: *const ::core::ffi::c_char,
    mut retain: bool,
    mut timeout_ms: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if topic.is_null() || payload.is_null() || timeout_ms <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut payload_length: size_t = strlen(payload);
    if payload_length > INT_MAX as size_t {
        return -1 as ::core::ffi::c_int;
    }
    if !mqtt_is_connected() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut publish_ack_mutex);
    if publish_ack_waiting {
        pthread_mutex_unlock(&raw mut publish_ack_mutex);
        return -1 as ::core::ffi::c_int;
    }
    publish_ack_waiting = r#true != 0;
    publish_ack_received = r#false != 0;
    publish_ack_failed = r#false != 0;
    publish_ack_mid = -1 as ::core::ffi::c_int;
    pthread_mutex_lock(&raw mut mqtt_mutex);
    if mosq.is_null()
        || mqtt_config.is_null()
        || !(*mqtt_config).mqtt_enabled
        || !connected
        || ::core::ptr::read_volatile::<bool>(&raw const shutting_down) as ::core::ffi::c_int != 0
    {
        pthread_mutex_unlock(&raw mut mqtt_mutex);
        publish_ack_waiting = r#false != 0;
        pthread_mutex_unlock(&raw mut publish_ack_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut mid: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = mosquitto_publish(
        mosq,
        &raw mut mid,
        topic,
        payload_length as ::core::ffi::c_int,
        payload as *const ::core::ffi::c_void,
        (*mqtt_config).mqtt_qos,
        retain,
    );
    publish_ack_mid = mid;
    pthread_mutex_unlock(&raw mut mqtt_mutex);
    if rc != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to queue confirmed publish to %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            topic,
            mosquitto_strerror(rc),
        );
        publish_ack_waiting = r#false != 0;
        publish_ack_mid = -1 as ::core::ffi::c_int;
        pthread_mutex_unlock(&raw mut publish_ack_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut deadline: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut deadline);
    deadline.tv_sec += (timeout_ms / 1000 as ::core::ffi::c_int) as __time_t;
    deadline.tv_nsec += (timeout_ms % 1000 as ::core::ffi::c_int) as ::core::ffi::c_long
        * 1000000 as ::core::ffi::c_long;
    if deadline.tv_nsec >= 1000000000 as ::core::ffi::c_long {
        deadline.tv_sec += 1;
        deadline.tv_nsec -= 1000000000 as ::core::ffi::c_long;
    }
    while !publish_ack_received && !publish_ack_failed {
        rc = pthread_cond_timedwait(
            &raw mut publish_ack_wake,
            &raw mut publish_ack_mutex,
            &raw mut deadline,
        );
        if rc == ETIMEDOUT {
            break;
        }
        if rc == 0 as ::core::ffi::c_int {
            continue;
        }
        publish_ack_failed = r#true != 0;
        break;
    }
    let mut acknowledged: bool =
        publish_ack_received as ::core::ffi::c_int != 0 && !publish_ack_failed;
    publish_ack_waiting = r#false != 0;
    publish_ack_received = r#false != 0;
    publish_ack_failed = r#false != 0;
    publish_ack_mid = -1 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut publish_ack_mutex);
    if !acknowledged {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Confirmed publish to %s was not acknowledged\0".as_ptr()
                as *const ::core::ffi::c_char,
            topic,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_publish_event(
    mut event: *const event_envelope_t,
) -> ::core::ffi::c_int {
    if mosq.is_null() || mqtt_config.is_null() || !(*mqtt_config).mqtt_enabled {
        return 0 as ::core::ffi::c_int;
    }
    if event.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut subject_id: *const ::core::ffi::c_char = strrchr(
        &raw const (*event).subject as *const ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    subject_id = if !subject_id.is_null() {
        subject_id.offset(1 as ::core::ffi::c_int as isize)
    } else {
        &raw const (*event).subject as *const ::core::ffi::c_char
    };
    if *subject_id.offset(0isize) == 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut topic: [::core::ffi::c_char; 512] = [0; 512];
    let mut topic_length: ::core::ffi::c_int = snprintf(
        &raw mut topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/v1/events/%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*mqtt_config).mqtt_topic_prefix as *const ::core::ffi::c_char,
        &raw const (*event).r#type as *const ::core::ffi::c_char,
        subject_id,
    );
    if topic_length < 0 as ::core::ffi::c_int
        || topic_length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Normalized event topic exceeds maximum length\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
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
    let mut payload: *mut ::core::ffi::c_char = event_envelope_serialize(
        event,
        &raw mut error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if payload.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to serialize event envelope: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut error as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = mqtt_publish_raw(
        &raw mut topic as *mut ::core::ffi::c_char,
        payload,
        r#false != 0,
    );
    free(payload as *mut ::core::ffi::c_void);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_publish_binary(
    mut topic: *const ::core::ffi::c_char,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
    mut retain: bool,
) -> ::core::ffi::c_int {
    if mosq.is_null() || mqtt_config.is_null() || !(*mqtt_config).mqtt_enabled {
        return 0 as ::core::ffi::c_int;
    }
    if topic.is_null() || data.is_null() || len == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    if !mqtt_is_connected() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut mqtt_mutex);
    let mut rc: ::core::ffi::c_int = mosquitto_publish(
        mosq,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        topic,
        len as ::core::ffi::c_int,
        data,
        (*mqtt_config).mqtt_qos,
        retain,
    );
    pthread_mutex_unlock(&raw mut mqtt_mutex);
    if rc != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to publish binary to %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            topic,
            mosquitto_strerror(rc),
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn build_ha_device_block() -> *mut cJSON {
    let mut device: *mut cJSON = cJSON_CreateObject();
    if device.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut ids: *mut cJSON = cJSON_CreateArray();
    cJSON_AddItemToArray(
        ids,
        cJSON_CreateString(b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char),
    );
    cJSON_AddItemToObject(
        device,
        b"identifiers\0".as_ptr() as *const ::core::ffi::c_char,
        ids,
    );
    cJSON_AddStringToObject(
        device,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        b"LightNVR\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        device,
        b"manufacturer\0".as_ptr() as *const ::core::ffi::c_char,
        b"OpenSensor\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        device,
        b"model\0".as_ptr() as *const ::core::ffi::c_char,
        b"LightNVR\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        device,
        b"sw_version\0".as_ptr() as *const ::core::ffi::c_char,
        LIGHTNVR_VERSION_STRING.as_ptr(),
    );
    return device;
}
unsafe extern "C" fn build_ha_origin_block() -> *mut cJSON {
    let mut origin: *mut cJSON = cJSON_CreateObject();
    if origin.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        origin,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        b"LightNVR\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        origin,
        b"sw\0".as_ptr() as *const ::core::ffi::c_char,
        LIGHTNVR_VERSION_STRING.as_ptr(),
    );
    cJSON_AddStringToObject(
        origin,
        b"url\0".as_ptr() as *const ::core::ffi::c_char,
        b"https://github.com/opensensor/lightNVR\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return origin;
}
unsafe extern "C" fn publish_ha_health_discovery() -> ::core::ffi::c_int {
    let mut status_topic: [::core::ffi::c_char; 512] = [0; 512];
    let mut installation_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if mqtt_presence_copy_topic(
        &raw mut status_topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
        || mqtt_presence_copy_installation_uuid(
            &raw mut installation_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        ) != 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut entities: [health_entity; 2] = [
        health_entity {
            suffix: b"system_health\0".as_ptr() as *const ::core::ffi::c_char,
            name: b"System Health\0".as_ptr() as *const ::core::ffi::c_char,
            value_template: b"{{ value_json.overall_state }}\0".as_ptr()
                as *const ::core::ffi::c_char,
            icon: b"mdi:heart-pulse\0".as_ptr() as *const ::core::ffi::c_char,
        },
        health_entity {
            suffix: b"active_health_incidents\0".as_ptr() as *const ::core::ffi::c_char,
            name: b"Active Health Incidents\0".as_ptr() as *const ::core::ffi::c_char,
            value_template: b"{{ value_json.active_incidents }}\0".as_ptr()
                as *const ::core::ffi::c_char,
            icon: b"mdi:alert-circle-outline\0".as_ptr() as *const ::core::ffi::c_char,
        },
    ];
    let mut published: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut index: size_t = 0 as size_t;
    while index
        < ::core::mem::size_of::<[health_entity; 2]>()
            .wrapping_div(::core::mem::size_of::<health_entity>())
    {
        let mut topic: [::core::ffi::c_char; 512] = [0; 512];
        let mut written: ::core::ffi::c_int = snprintf(
            &raw mut topic as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/sensor/lightnvr/%s_%s/config\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*mqtt_config).mqtt_ha_discovery_prefix as *const ::core::ffi::c_char,
            &raw mut installation_uuid as *mut ::core::ffi::c_char,
            entities[index].suffix,
        );
        if !(written < 0 as ::core::ffi::c_int
            || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>())
        {
            let mut payload: *mut cJSON = cJSON_CreateObject();
            if !payload.is_null() {
                let mut unique_id: [::core::ffi::c_char; 160] = [0; 160];
                written = snprintf(
                    &raw mut unique_id as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
                    b"lightnvr_%s_%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut installation_uuid as *mut ::core::ffi::c_char,
                    entities[index].suffix,
                );
                if written < 0 as ::core::ffi::c_int
                    || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 160]>()
                {
                    cJSON_Delete(payload);
                } else {
                    cJSON_AddStringToObject(
                        payload,
                        b"unique_id\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut unique_id as *mut ::core::ffi::c_char,
                    );
                    cJSON_AddStringToObject(
                        payload,
                        b"name\0".as_ptr() as *const ::core::ffi::c_char,
                        entities[index].name,
                    );
                    cJSON_AddStringToObject(
                        payload,
                        b"state_topic\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut status_topic as *mut ::core::ffi::c_char,
                    );
                    cJSON_AddStringToObject(
                        payload,
                        b"value_template\0".as_ptr() as *const ::core::ffi::c_char,
                        entities[index].value_template,
                    );
                    cJSON_AddStringToObject(
                        payload,
                        b"icon\0".as_ptr() as *const ::core::ffi::c_char,
                        entities[index].icon,
                    );
                    let mut availability: *mut cJSON = cJSON_CreateObject();
                    let mut availability_list: *mut cJSON = cJSON_CreateArray();
                    let mut device: *mut cJSON = build_ha_device_block();
                    let mut origin: *mut cJSON = build_ha_origin_block();
                    if availability.is_null()
                        || availability_list.is_null()
                        || device.is_null()
                        || origin.is_null()
                    {
                        cJSON_Delete(availability);
                        cJSON_Delete(availability_list);
                        cJSON_Delete(device);
                        cJSON_Delete(origin);
                        cJSON_Delete(payload);
                    } else {
                        cJSON_AddStringToObject(
                            availability,
                            b"topic\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut status_topic as *mut ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            availability,
                            b"value_template\0".as_ptr() as *const ::core::ffi::c_char,
                            b"{{ value_json.state }}\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            availability,
                            b"payload_available\0".as_ptr() as *const ::core::ffi::c_char,
                            b"online\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            availability,
                            b"payload_not_available\0".as_ptr() as *const ::core::ffi::c_char,
                            b"offline\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        cJSON_AddItemToArray(availability_list, availability);
                        cJSON_AddItemToObject(
                            payload,
                            b"availability\0".as_ptr() as *const ::core::ffi::c_char,
                            availability_list,
                        );
                        cJSON_AddItemToObject(
                            payload,
                            b"device\0".as_ptr() as *const ::core::ffi::c_char,
                            device,
                        );
                        cJSON_AddItemToObject(
                            payload,
                            b"origin\0".as_ptr() as *const ::core::ffi::c_char,
                            origin,
                        );
                        let mut json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(payload);
                        cJSON_Delete(payload);
                        if !json.is_null() {
                            if mqtt_publish_raw(
                                &raw mut topic as *mut ::core::ffi::c_char,
                                json,
                                r#true != 0,
                            ) == 0 as ::core::ffi::c_int
                            {
                                published += 1;
                            }
                            free(json as *mut ::core::ffi::c_void);
                        }
                    }
                }
            }
        }
        index = index.wrapping_add(1);
    }
    return published;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_publish_ha_discovery() -> ::core::ffi::c_int {
    if mosq.is_null()
        || mqtt_config.is_null()
        || !(*mqtt_config).mqtt_enabled
        || !(*mqtt_config).mqtt_ha_discovery
    {
        return 0 as ::core::ffi::c_int;
    }
    if !mqtt_is_connected() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT HA: Not connected, skipping discovery publish\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT HA: Publishing Home Assistant discovery messages...\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut published: ::core::ffi::c_int = publish_ha_health_discovery();
    let mut streams: *mut stream_config_t = calloc(
        MAX_MOTION_STREAMS as size_t,
        ::core::mem::size_of::<stream_config_t>(),
    ) as *mut stream_config_t;
    if streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT HA: Failed to allocate streams buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut num_streams: ::core::ffi::c_int = get_all_stream_configs(streams, MAX_MOTION_STREAMS);
    if num_streams <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT HA: No streams configured; published system discovery only\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(streams as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    let mut prefix: *const ::core::ffi::c_char =
        &raw const (*mqtt_config).mqtt_ha_discovery_prefix as *const ::core::ffi::c_char;
    let mut topic_prefix: *const ::core::ffi::c_char =
        &raw const (*mqtt_config).mqtt_topic_prefix as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < num_streams {
        if !(!(*streams.offset(i as isize)).enabled
            || (*streams.offset(i as isize)).name[0usize] as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int)
        {
            let mut safe_name: [::core::ffi::c_char; 256] = [0; 256];
            sanitize_stream_name(
                &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                &raw mut safe_name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            );
            let mut topic: [::core::ffi::c_char; 512] = [0; 512];
            snprintf(
                &raw mut topic as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                b"%s/camera/lightnvr/%s/config\0".as_ptr() as *const ::core::ffi::c_char,
                prefix,
                &raw mut safe_name as *mut ::core::ffi::c_char,
            );
            let mut payload: *mut cJSON = cJSON_CreateObject();
            if !payload.is_null() {
                let mut unique_id: [::core::ffi::c_char; 256] = [0; 256];
                snprintf(
                    &raw mut unique_id as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    b"lightnvr_%s_camera\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut safe_name as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    payload,
                    b"unique_id\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut unique_id as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    payload,
                    b"name\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                );
                let mut image_topic: [::core::ffi::c_char; 512] = [0; 512];
                snprintf(
                    &raw mut image_topic as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    b"%s/cameras/%s/snapshot\0".as_ptr() as *const ::core::ffi::c_char,
                    topic_prefix,
                    &raw mut safe_name as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    payload,
                    b"topic\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut image_topic as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    payload,
                    b"encoding\0".as_ptr() as *const ::core::ffi::c_char,
                    b"\0".as_ptr() as *const ::core::ffi::c_char,
                );
                let mut avail: *mut cJSON = cJSON_CreateObject();
                let mut avail_topic: [::core::ffi::c_char; 512] = [0; 512];
                snprintf(
                    &raw mut avail_topic as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    b"%s/availability\0".as_ptr() as *const ::core::ffi::c_char,
                    topic_prefix,
                );
                cJSON_AddStringToObject(
                    avail,
                    b"topic\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut avail_topic as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    avail,
                    b"payload_available\0".as_ptr() as *const ::core::ffi::c_char,
                    b"online\0".as_ptr() as *const ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    avail,
                    b"payload_not_available\0".as_ptr() as *const ::core::ffi::c_char,
                    b"offline\0".as_ptr() as *const ::core::ffi::c_char,
                );
                let mut avail_list: *mut cJSON = cJSON_CreateArray();
                cJSON_AddItemToArray(avail_list, avail);
                cJSON_AddItemToObject(
                    payload,
                    b"availability\0".as_ptr() as *const ::core::ffi::c_char,
                    avail_list,
                );
                let mut device: *mut cJSON = build_ha_device_block();
                let mut origin: *mut cJSON = build_ha_origin_block();
                if device.is_null() || origin.is_null() {
                    cJSON_Delete(device);
                    cJSON_Delete(origin);
                    cJSON_Delete(payload);
                } else {
                    cJSON_AddItemToObject(
                        payload,
                        b"device\0".as_ptr() as *const ::core::ffi::c_char,
                        device,
                    );
                    cJSON_AddItemToObject(
                        payload,
                        b"origin\0".as_ptr() as *const ::core::ffi::c_char,
                        origin,
                    );
                    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(payload);
                    cJSON_Delete(payload);
                    if !json_str.is_null() {
                        mqtt_publish_raw(
                            &raw mut topic as *mut ::core::ffi::c_char,
                            json_str,
                            r#true != 0,
                        );
                        free(json_str as *mut ::core::ffi::c_void);
                        published += 1;
                    }
                    let mut topic_0: [::core::ffi::c_char; 512] = [0; 512];
                    snprintf(
                        &raw mut topic_0 as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                        b"%s/binary_sensor/lightnvr/%s_motion/config\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        prefix,
                        &raw mut safe_name as *mut ::core::ffi::c_char,
                    );
                    let mut payload_0: *mut cJSON = cJSON_CreateObject();
                    if !payload_0.is_null() {
                        let mut unique_id_0: [::core::ffi::c_char; 256] = [0; 256];
                        snprintf(
                            &raw mut unique_id_0 as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                            b"lightnvr_%s_motion\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut safe_name as *mut ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            payload_0,
                            b"unique_id\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut unique_id_0 as *mut ::core::ffi::c_char,
                        );
                        let mut name: [::core::ffi::c_char; 256] = [0; 256];
                        snprintf(
                            &raw mut name as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                            b"%s Motion\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            payload_0,
                            b"name\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut name as *mut ::core::ffi::c_char,
                        );
                        let mut state_topic: [::core::ffi::c_char; 512] = [0; 512];
                        snprintf(
                            &raw mut state_topic as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            b"%s/cameras/%s/motion\0".as_ptr() as *const ::core::ffi::c_char,
                            topic_prefix,
                            &raw mut safe_name as *mut ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            payload_0,
                            b"state_topic\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut state_topic as *mut ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            payload_0,
                            b"payload_on\0".as_ptr() as *const ::core::ffi::c_char,
                            b"ON\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            payload_0,
                            b"payload_off\0".as_ptr() as *const ::core::ffi::c_char,
                            b"OFF\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            payload_0,
                            b"device_class\0".as_ptr() as *const ::core::ffi::c_char,
                            b"motion\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        let mut avail_0: *mut cJSON = cJSON_CreateObject();
                        let mut avail_topic_0: [::core::ffi::c_char; 512] = [0; 512];
                        snprintf(
                            &raw mut avail_topic_0 as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            b"%s/availability\0".as_ptr() as *const ::core::ffi::c_char,
                            topic_prefix,
                        );
                        cJSON_AddStringToObject(
                            avail_0,
                            b"topic\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut avail_topic_0 as *mut ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            avail_0,
                            b"payload_available\0".as_ptr() as *const ::core::ffi::c_char,
                            b"online\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            avail_0,
                            b"payload_not_available\0".as_ptr() as *const ::core::ffi::c_char,
                            b"offline\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        let mut avail_list_0: *mut cJSON = cJSON_CreateArray();
                        cJSON_AddItemToArray(avail_list_0, avail_0);
                        cJSON_AddItemToObject(
                            payload_0,
                            b"availability\0".as_ptr() as *const ::core::ffi::c_char,
                            avail_list_0,
                        );
                        let mut device_0: *mut cJSON = build_ha_device_block();
                        let mut origin_0: *mut cJSON = build_ha_origin_block();
                        if device_0.is_null() || origin_0.is_null() {
                            cJSON_Delete(device_0);
                            cJSON_Delete(origin_0);
                            cJSON_Delete(payload_0);
                        } else {
                            cJSON_AddItemToObject(
                                payload_0,
                                b"device\0".as_ptr() as *const ::core::ffi::c_char,
                                device_0,
                            );
                            cJSON_AddItemToObject(
                                payload_0,
                                b"origin\0".as_ptr() as *const ::core::ffi::c_char,
                                origin_0,
                            );
                            let mut json_str_0: *mut ::core::ffi::c_char =
                                cJSON_PrintUnformatted(payload_0);
                            cJSON_Delete(payload_0);
                            if !json_str_0.is_null() {
                                mqtt_publish_raw(
                                    &raw mut topic_0 as *mut ::core::ffi::c_char,
                                    json_str_0,
                                    r#true != 0,
                                );
                                free(json_str_0 as *mut ::core::ffi::c_void);
                                published += 1;
                            }
                            let mut topic_1: [::core::ffi::c_char; 512] = [0; 512];
                            snprintf(
                                &raw mut topic_1 as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                                b"%s/sensor/lightnvr/%s_detection_count/config\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                prefix,
                                &raw mut safe_name as *mut ::core::ffi::c_char,
                            );
                            let mut payload_1: *mut cJSON = cJSON_CreateObject();
                            if !payload_1.is_null() {
                                let mut unique_id_1: [::core::ffi::c_char; 256] = [0; 256];
                                snprintf(
                                    &raw mut unique_id_1 as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                                    b"lightnvr_%s_detection_count\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut safe_name as *mut ::core::ffi::c_char,
                                );
                                cJSON_AddStringToObject(
                                    payload_1,
                                    b"unique_id\0".as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut unique_id_1 as *mut ::core::ffi::c_char,
                                );
                                let mut name_0: [::core::ffi::c_char; 256] = [0; 256];
                                snprintf(
                                    &raw mut name_0 as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                                    b"%s Detections\0".as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut (*streams.offset(i as isize)).name
                                        as *mut ::core::ffi::c_char,
                                );
                                cJSON_AddStringToObject(
                                    payload_1,
                                    b"name\0".as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut name_0 as *mut ::core::ffi::c_char,
                                );
                                let mut state_topic_0: [::core::ffi::c_char; 512] = [0; 512];
                                snprintf(
                                    &raw mut state_topic_0 as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                                    b"%s/cameras/%s/detection_count\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    topic_prefix,
                                    &raw mut safe_name as *mut ::core::ffi::c_char,
                                );
                                cJSON_AddStringToObject(
                                    payload_1,
                                    b"state_topic\0".as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut state_topic_0 as *mut ::core::ffi::c_char,
                                );
                                cJSON_AddStringToObject(
                                    payload_1,
                                    b"icon\0".as_ptr() as *const ::core::ffi::c_char,
                                    b"mdi:motion-sensor\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                let mut avail_1: *mut cJSON = cJSON_CreateObject();
                                let mut avail_topic_1: [::core::ffi::c_char; 512] = [0; 512];
                                snprintf(
                                    &raw mut avail_topic_1 as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                                    b"%s/availability\0".as_ptr() as *const ::core::ffi::c_char,
                                    topic_prefix,
                                );
                                cJSON_AddStringToObject(
                                    avail_1,
                                    b"topic\0".as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut avail_topic_1 as *mut ::core::ffi::c_char,
                                );
                                cJSON_AddStringToObject(
                                    avail_1,
                                    b"payload_available\0".as_ptr() as *const ::core::ffi::c_char,
                                    b"online\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                cJSON_AddStringToObject(
                                    avail_1,
                                    b"payload_not_available\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"offline\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                let mut avail_list_1: *mut cJSON = cJSON_CreateArray();
                                cJSON_AddItemToArray(avail_list_1, avail_1);
                                cJSON_AddItemToObject(
                                    payload_1,
                                    b"availability\0".as_ptr() as *const ::core::ffi::c_char,
                                    avail_list_1,
                                );
                                let mut device_1: *mut cJSON = build_ha_device_block();
                                let mut origin_1: *mut cJSON = build_ha_origin_block();
                                if device_1.is_null() || origin_1.is_null() {
                                    cJSON_Delete(device_1);
                                    cJSON_Delete(origin_1);
                                    cJSON_Delete(payload_1);
                                } else {
                                    cJSON_AddItemToObject(
                                        payload_1,
                                        b"device\0".as_ptr() as *const ::core::ffi::c_char,
                                        device_1,
                                    );
                                    cJSON_AddItemToObject(
                                        payload_1,
                                        b"origin\0".as_ptr() as *const ::core::ffi::c_char,
                                        origin_1,
                                    );
                                    let mut json_str_1: *mut ::core::ffi::c_char =
                                        cJSON_PrintUnformatted(payload_1);
                                    cJSON_Delete(payload_1);
                                    if !json_str_1.is_null() {
                                        mqtt_publish_raw(
                                            &raw mut topic_1 as *mut ::core::ffi::c_char,
                                            json_str_1,
                                            r#true != 0,
                                        );
                                        free(json_str_1 as *mut ::core::ffi::c_void);
                                        published += 1;
                                    }
                                    let mut topic_2: [::core::ffi::c_char; 512] = [0; 512];
                                    snprintf(
                                        &raw mut topic_2 as *mut ::core::ffi::c_char,
                                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                                        b"%s/cameras/%s/motion\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        topic_prefix,
                                        &raw mut safe_name as *mut ::core::ffi::c_char,
                                    );
                                    mqtt_publish_raw(
                                        &raw mut topic_2 as *mut ::core::ffi::c_char,
                                        b"OFF\0".as_ptr() as *const ::core::ffi::c_char,
                                        r#false != 0,
                                    );
                                    snprintf(
                                        &raw mut topic_2 as *mut ::core::ffi::c_char,
                                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                                        b"%s/cameras/%s/detection_count\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        topic_prefix,
                                        &raw mut safe_name as *mut ::core::ffi::c_char,
                                    );
                                    mqtt_publish_raw(
                                        &raw mut topic_2 as *mut ::core::ffi::c_char,
                                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                                        r#false != 0,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT HA: Published %d discovery messages for %d streams\0".as_ptr()
            as *const ::core::ffi::c_char,
        published,
        num_streams,
    );
    free(streams as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_set_motion_state(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
) {
    if mosq.is_null()
        || mqtt_config.is_null()
        || !(*mqtt_config).mqtt_enabled
        || !(*mqtt_config).mqtt_ha_discovery
    {
        return;
    }
    if stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return;
    }
    pthread_mutex_lock(&raw mut motion_mutex);
    let mut state: *mut motion_state_t = ::core::ptr::null_mut::<motion_state_t>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < num_motion_states {
        if strcmp(
            &raw mut (*(&raw mut motion_states as *mut motion_state_t).offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
            stream_name,
        ) == 0 as ::core::ffi::c_int
        {
            state = (&raw mut motion_states as *mut motion_state_t).offset(i as isize);
            break;
        } else {
            i += 1;
        }
    }
    if state.is_null() && num_motion_states < MAX_MOTION_STREAMS {
        let c2rust_fresh0 = num_motion_states;
        num_motion_states += 1;
        state = (&raw mut motion_states as *mut motion_state_t).offset(c2rust_fresh0 as isize);
        memset(
            state as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<motion_state_t>(),
        );
        safe_strcpy(
            &raw mut (*state).stream_name as *mut ::core::ffi::c_char,
            stream_name,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    }
    if state.is_null() {
        pthread_mutex_unlock(&raw mut motion_mutex);
        return;
    }
    (*state).last_detection_time = time(::core::ptr::null_mut::<time_t>());
    let mut should_publish_on: bool = !(*state).motion_active;
    (*state).motion_active = r#true != 0;
    if !result.is_null() && (*result).count > 0 as ::core::ffi::c_int {
        memset(
            &raw mut (*state).object_counts as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_int; 32]>(),
        );
        (*state).num_labels = 0 as ::core::ffi::c_int;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*result).count && i_0 < MAX_DETECTIONS {
            let mut label: *const ::core::ffi::c_char =
                &raw const (*(&raw const (*result).detections as *const detection_t)
                    .offset(i_0 as isize))
                .label as *const ::core::ffi::c_char;
            if *label.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                let mut label_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < (*state).num_labels {
                    if strcmp(
                        &raw mut *(&raw mut (*state).object_labels
                            as *mut [::core::ffi::c_char; 32])
                            .offset(j as isize) as *mut ::core::ffi::c_char,
                        label,
                    ) == 0 as ::core::ffi::c_int
                    {
                        label_idx = j;
                        break;
                    } else {
                        j += 1;
                    }
                }
                if label_idx < 0 as ::core::ffi::c_int
                    && (*state).num_labels < 32 as ::core::ffi::c_int
                {
                    let c2rust_fresh1 = (*state).num_labels;
                    (*state).num_labels += 1;
                    label_idx = c2rust_fresh1;
                    safe_strcpy(
                        &raw mut *(&raw mut (*state).object_labels
                            as *mut [::core::ffi::c_char; 32])
                            .offset(label_idx as isize)
                            as *mut ::core::ffi::c_char,
                        label,
                        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                        0 as size_t,
                    );
                }
                if label_idx >= 0 as ::core::ffi::c_int {
                    (*state).object_counts[label_idx as usize] += 1;
                }
            }
            i_0 += 1;
        }
    }
    let mut total_count: ::core::ffi::c_int = if !result.is_null() {
        (*result).count
    } else {
        0 as ::core::ffi::c_int
    };
    let mut num_labels: ::core::ffi::c_int = (*state).num_labels;
    let mut labels_copy: [[::core::ffi::c_char; 32]; 32] = [[0; 32]; 32];
    let mut counts_copy: [::core::ffi::c_int; 32] = [0; 32];
    memcpy(
        &raw mut labels_copy as *mut [::core::ffi::c_char; 32] as *mut ::core::ffi::c_void,
        &raw mut (*state).object_labels as *mut [::core::ffi::c_char; 32]
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[[::core::ffi::c_char; 32]; 32]>(),
    );
    memcpy(
        &raw mut counts_copy as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
        &raw mut (*state).object_counts as *mut ::core::ffi::c_int as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_int; 32]>(),
    );
    pthread_mutex_unlock(&raw mut motion_mutex);
    let mut safe_name: [::core::ffi::c_char; 256] = [0; 256];
    sanitize_stream_name(
        stream_name,
        &raw mut safe_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if should_publish_on {
        let mut topic: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut topic as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/cameras/%s/motion\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*mqtt_config).mqtt_topic_prefix as *const ::core::ffi::c_char,
            &raw mut safe_name as *mut ::core::ffi::c_char,
        );
        mqtt_publish_raw(
            &raw mut topic as *mut ::core::ffi::c_char,
            b"ON\0".as_ptr() as *const ::core::ffi::c_char,
            r#false != 0,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT HA: Motion ON for %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
    }
    let mut topic_0: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut topic_0 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/cameras/%s/detection_count\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*mqtt_config).mqtt_topic_prefix as *const ::core::ffi::c_char,
        &raw mut safe_name as *mut ::core::ffi::c_char,
    );
    let mut count_str: [::core::ffi::c_char; 16] = [0; 16];
    snprintf(
        &raw mut count_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        total_count,
    );
    mqtt_publish_raw(
        &raw mut topic_0 as *mut ::core::ffi::c_char,
        &raw mut count_str as *mut ::core::ffi::c_char,
        r#false != 0,
    );
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < num_labels {
        let mut topic_1: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut topic_1 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/cameras/%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*mqtt_config).mqtt_topic_prefix as *const ::core::ffi::c_char,
            &raw mut safe_name as *mut ::core::ffi::c_char,
            &raw mut *(&raw mut labels_copy as *mut [::core::ffi::c_char; 32]).offset(i_1 as isize)
                as *mut ::core::ffi::c_char,
        );
        let mut count_str_0: [::core::ffi::c_char; 16] = [0; 16];
        snprintf(
            &raw mut count_str_0 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            counts_copy[i_1 as usize],
        );
        mqtt_publish_raw(
            &raw mut topic_1 as *mut ::core::ffi::c_char,
            &raw mut count_str_0 as *mut ::core::ffi::c_char,
            r#false != 0,
        );
        i_1 += 1;
    }
}
unsafe extern "C" fn ha_snapshot_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"MQTT\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT HA: Snapshot publishing thread started (interval=%ds)\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*mqtt_config).mqtt_ha_snapshot_interval,
    );
    let mut streams: *mut stream_config_t = calloc(
        MAX_MOTION_STREAMS as size_t,
        ::core::mem::size_of::<stream_config_t>(),
    ) as *mut stream_config_t;
    if streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT HA: Failed to allocate streams buffer, snapshot thread exiting\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    while ::core::ptr::read_volatile::<bool>(&raw const ha_services_running) {
        if !mqtt_is_connected() || mqtt_config.is_null() {
            interruptible_sleep_wait(&raw mut ha_snapshot_wake, 1 as ::core::ffi::c_int);
        } else {
            let mut num_streams: ::core::ffi::c_int =
                get_all_stream_configs(streams, MAX_MOTION_STREAMS);
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < num_streams
                && ::core::ptr::read_volatile::<bool>(&raw const ha_services_running)
                    as ::core::ffi::c_int
                    != 0
            {
                if !(!(*streams.offset(i as isize)).enabled
                    || (*streams.offset(i as isize)).name[0usize] as ::core::ffi::c_int
                        == '\0' as ::core::ffi::c_int)
                {
                    let mut jpeg_data: *mut ::core::ffi::c_uchar =
                        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    let mut jpeg_size: size_t = 0 as size_t;
                    if go2rtc_get_snapshot(
                        &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                        &raw mut jpeg_data,
                        &raw mut jpeg_size,
                    ) {
                        let mut safe_name: [::core::ffi::c_char; 256] = [0; 256];
                        sanitize_stream_name(
                            &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                            &raw mut safe_name as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                        );
                        let mut topic: [::core::ffi::c_char; 512] = [0; 512];
                        snprintf(
                            &raw mut topic as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                            b"%s/cameras/%s/snapshot\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw const (*mqtt_config).mqtt_topic_prefix
                                as *const ::core::ffi::c_char,
                            &raw mut safe_name as *mut ::core::ffi::c_char,
                        );
                        mqtt_publish_binary(
                            &raw mut topic as *mut ::core::ffi::c_char,
                            jpeg_data as *const ::core::ffi::c_void,
                            jpeg_size,
                            r#false != 0,
                        );
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_DEBUG,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"MQTT HA: Published snapshot for %s (%zu bytes)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                            jpeg_size,
                        );
                        free(jpeg_data as *mut ::core::ffi::c_void);
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_DEBUG,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"MQTT HA: Failed to get snapshot for %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                        );
                    }
                }
                i += 1;
            }
            interruptible_sleep_wait(
                &raw mut ha_snapshot_wake,
                (*mqtt_config).mqtt_ha_snapshot_interval,
            );
        }
    }
    free(streams as *mut ::core::ffi::c_void);
    go2rtc_snapshot_cleanup_thread();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT HA: Snapshot publishing thread stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return NULL;
}
unsafe extern "C" fn ha_motion_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"MQTT\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT HA: Motion timeout thread started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    while ::core::ptr::read_volatile::<bool>(&raw const ha_services_running) {
        if !mqtt_is_connected() || mqtt_config.is_null() {
            interruptible_sleep_wait(&raw mut ha_motion_wake, 1 as ::core::ffi::c_int);
        } else {
            let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
            pthread_mutex_lock(&raw mut motion_mutex);
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < num_motion_states {
                if motion_states[i as usize].motion_active as ::core::ffi::c_int != 0
                    && now - motion_states[i as usize].last_detection_time
                        >= MOTION_OFF_DELAY_SEC as time_t
                {
                    motion_states[i as usize].motion_active = r#false != 0;
                    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
                    safe_strcpy(
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        &raw mut (*(&raw mut motion_states as *mut motion_state_t)
                            .offset(i as isize))
                        .stream_name as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                        0 as size_t,
                    );
                    let mut safe_name: [::core::ffi::c_char; 256] = [0; 256];
                    sanitize_stream_name(
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        &raw mut safe_name as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    );
                    pthread_mutex_unlock(&raw mut motion_mutex);
                    let mut topic: [::core::ffi::c_char; 512] = [0; 512];
                    snprintf(
                        &raw mut topic as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                        b"%s/cameras/%s/motion\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw const (*mqtt_config).mqtt_topic_prefix as *const ::core::ffi::c_char,
                        &raw mut safe_name as *mut ::core::ffi::c_char,
                    );
                    mqtt_publish_raw(
                        &raw mut topic as *mut ::core::ffi::c_char,
                        b"OFF\0".as_ptr() as *const ::core::ffi::c_char,
                        r#false != 0,
                    );
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"MQTT HA: Motion OFF for %s (timeout)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                    );
                    snprintf(
                        &raw mut topic as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                        b"%s/cameras/%s/detection_count\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw const (*mqtt_config).mqtt_topic_prefix as *const ::core::ffi::c_char,
                        &raw mut safe_name as *mut ::core::ffi::c_char,
                    );
                    mqtt_publish_raw(
                        &raw mut topic as *mut ::core::ffi::c_char,
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                        r#false != 0,
                    );
                    pthread_mutex_lock(&raw mut motion_mutex);
                }
                i += 1;
            }
            pthread_mutex_unlock(&raw mut motion_mutex);
            interruptible_sleep_wait(&raw mut ha_motion_wake, 1 as ::core::ffi::c_int);
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT HA: Motion timeout thread stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_start_ha_services() -> ::core::ffi::c_int {
    if mqtt_config.is_null() || !(*mqtt_config).mqtt_ha_discovery {
        return 0 as ::core::ffi::c_int;
    }
    if ::core::ptr::read_volatile::<bool>(&raw const ha_services_running) {
        return 0 as ::core::ffi::c_int;
    }
    ::core::ptr::write_volatile(&raw mut ha_services_running, r#true != 0);
    ha_snapshot_thread_started = r#false != 0;
    if interruptible_sleep_init(&raw mut ha_snapshot_wake) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT HA: Failed to initialize snapshot wake handle\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        ::core::ptr::write_volatile(&raw mut ha_services_running, r#false != 0);
        return -1 as ::core::ffi::c_int;
    }
    if interruptible_sleep_init(&raw mut ha_motion_wake) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT HA: Failed to initialize motion wake handle\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        interruptible_sleep_destroy(&raw mut ha_snapshot_wake);
        ::core::ptr::write_volatile(&raw mut ha_services_running, r#false != 0);
        return -1 as ::core::ffi::c_int;
    }
    if (*mqtt_config).mqtt_ha_snapshot_interval > 0 as ::core::ffi::c_int {
        if pthread_create(
            &raw mut ha_snapshot_thread,
            ::core::ptr::null::<pthread_attr_t>(),
            Some(
                ha_snapshot_thread_func
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            NULL,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT HA: Failed to create snapshot thread\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            ::core::ptr::write_volatile(&raw mut ha_services_running, r#false != 0);
            interruptible_sleep_destroy(&raw mut ha_snapshot_wake);
            interruptible_sleep_destroy(&raw mut ha_motion_wake);
            return -1 as ::core::ffi::c_int;
        }
        ha_snapshot_thread_started = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT HA: Snapshot publishing started (interval=%ds)\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*mqtt_config).mqtt_ha_snapshot_interval,
        );
    }
    if pthread_create(
        &raw mut ha_motion_thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            ha_motion_thread_func
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT HA: Failed to create motion timeout thread\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        ::core::ptr::write_volatile(&raw mut ha_services_running, r#false != 0);
        if ha_snapshot_thread_started {
            interruptible_sleep_wake(&raw mut ha_snapshot_wake);
            pthread_join(
                ha_snapshot_thread,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
            );
            ha_snapshot_thread_started = r#false != 0;
        }
        interruptible_sleep_destroy(&raw mut ha_snapshot_wake);
        interruptible_sleep_destroy(&raw mut ha_motion_wake);
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT HA: Background services started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_stop_ha_services() {
    if !::core::ptr::read_volatile::<bool>(&raw const ha_services_running) {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT HA: Stopping background services...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    ::core::ptr::write_volatile(&raw mut ha_services_running, r#false != 0);
    interruptible_sleep_wake(&raw mut ha_snapshot_wake);
    interruptible_sleep_wake(&raw mut ha_motion_wake);
    if ha_snapshot_thread_started {
        pthread_join(
            ha_snapshot_thread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        ha_snapshot_thread_started = r#false != 0;
    }
    pthread_join(
        ha_motion_thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
    interruptible_sleep_destroy(&raw mut ha_snapshot_wake);
    interruptible_sleep_destroy(&raw mut ha_motion_wake);
    pthread_mutex_lock(&raw mut motion_mutex);
    num_motion_states = 0 as ::core::ffi::c_int;
    memset(
        &raw mut motion_states as *mut motion_state_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[motion_state_t; 16]>(),
    );
    pthread_mutex_unlock(&raw mut motion_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT HA: Background services stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn mqtt_cleanup_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"MQTT\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut cleanup_arg: *mut mqtt_cleanup_arg_t = arg as *mut mqtt_cleanup_arg_t;
    match (*cleanup_arg).op {
        mqtt_cleanup_op_t::MQTT_OP_LOOP_STOP => {
            mosquitto_loop_stop((*cleanup_arg).mosq, r#true != 0);
        }
        mqtt_cleanup_op_t::MQTT_OP_DESTROY => {
            mosquitto_destroy((*cleanup_arg).mosq);
        }
        mqtt_cleanup_op_t::MQTT_OP_LIB_CLEANUP => {
            mosquitto_lib_cleanup();
        }
        _ => {}
    }
    pthread_mutex_lock(&raw mut (*cleanup_arg).mutex);
    (*cleanup_arg).done = r#true != 0;
    pthread_cond_signal(&raw mut (*cleanup_arg).cond);
    let mut timed_out: bool = (*cleanup_arg).timed_out;
    pthread_mutex_unlock(&raw mut (*cleanup_arg).mutex);
    if timed_out {
        pthread_cond_destroy(&raw mut (*cleanup_arg).cond);
        pthread_mutex_destroy(&raw mut (*cleanup_arg).mutex);
        free(cleanup_arg as *mut ::core::ffi::c_void);
    }
    return NULL;
}
unsafe extern "C" fn mqtt_run_with_timeout(
    mut m: *mut mosquitto,
    mut op: mqtt_cleanup_op_t,
    mut timeout_sec: ::core::ffi::c_int,
    mut op_name: *const ::core::ffi::c_char,
) -> bool {
    let mut thread: pthread_t = 0;
    let mut arg: *mut mqtt_cleanup_arg_t =
        calloc(1 as size_t, ::core::mem::size_of::<mqtt_cleanup_arg_t>())
            as *mut mqtt_cleanup_arg_t;
    if arg.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to allocate resources for %s\0".as_ptr() as *const ::core::ffi::c_char,
            op_name,
        );
        return r#false != 0;
    }
    if pthread_mutex_init(
        &raw mut (*arg).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        free(arg as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to initialize synchronization objects for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            op_name,
        );
        return r#false != 0;
    }
    if pthread_cond_init(
        &raw mut (*arg).cond,
        ::core::ptr::null::<pthread_condattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_destroy(&raw mut (*arg).mutex);
        free(arg as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to initialize synchronization objects for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            op_name,
        );
        return r#false != 0;
    }
    (*arg).mosq = m;
    (*arg).op = op;
    if pthread_create(
        &raw mut thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            mqtt_cleanup_thread
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        arg as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to create thread for %s\0".as_ptr() as *const ::core::ffi::c_char,
            op_name,
        );
        pthread_cond_destroy(&raw mut (*arg).cond);
        pthread_mutex_destroy(&raw mut (*arg).mutex);
        free(arg as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_REALTIME, &raw mut ts) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to get clock time for %s timeout\0".as_ptr()
                as *const ::core::ffi::c_char,
            op_name,
        );
        pthread_mutex_lock(&raw mut (*arg).mutex);
        (*arg).timed_out = r#true != 0;
        pthread_mutex_unlock(&raw mut (*arg).mutex);
        pthread_detach(thread);
        return r#false != 0;
    }
    ts.tv_sec += timeout_sec as __time_t;
    pthread_mutex_lock(&raw mut (*arg).mutex);
    let mut wait_rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*arg).done && wait_rc != ETIMEDOUT {
        wait_rc = pthread_cond_timedwait(&raw mut (*arg).cond, &raw mut (*arg).mutex, &raw mut ts);
    }
    let mut completed: bool = (*arg).done;
    if !completed {
        (*arg).timed_out = r#true != 0;
    }
    pthread_mutex_unlock(&raw mut (*arg).mutex);
    if completed {
        if pthread_join(thread, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>())
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT: Failed to join thread for %s\0".as_ptr() as *const ::core::ffi::c_char,
                op_name,
            );
        }
        pthread_cond_destroy(&raw mut (*arg).cond);
        pthread_mutex_destroy(&raw mut (*arg).mutex);
        free(arg as *mut ::core::ffi::c_void);
        return r#true != 0;
    }
    pthread_detach(thread);
    let mut timeout_msg: [::core::ffi::c_char; 128] = [0; 128];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut timeout_msg as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        b"[MQTT DEBUG] %s timed out after %d seconds\n\0".as_ptr() as *const ::core::ffi::c_char,
        op_name,
        timeout_sec,
    );
    if written >= 0 as ::core::ffi::c_int {
        let mut msg_len: size_t =
            if (written as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 128]>() {
                written as size_t
            } else {
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>().wrapping_sub(1 as size_t)
            };
        if msg_len > 0 as size_t {
            write(
                STDERR_FILENO,
                &raw mut timeout_msg as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                msg_len,
            );
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: %s timed out after %d seconds\0".as_ptr() as *const ::core::ffi::c_char,
        op_name,
        timeout_sec,
    );
    return r#false != 0;
}
unsafe extern "C" fn mqtt_try_lock_with_timeout(mut timeout_ms: ::core::ffi::c_int) -> bool {
    let mut elapsed_ms: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let poll_interval_ms: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    while elapsed_ms < timeout_ms {
        if pthread_mutex_trylock(&raw mut mqtt_mutex) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        usleep((poll_interval_ms * 1000 as ::core::ffi::c_int) as __useconds_t);
        elapsed_ms += poll_interval_ms;
    }
    return r#false != 0;
}
unsafe extern "C" fn mqtt_disconnect_internal() -> bool {
    if mosq.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: No client to disconnect\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    let mut had_timeout: bool = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Attempting to acquire mutex for disconnect...\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if !mqtt_try_lock_with_timeout(2000 as ::core::ffi::c_int) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to acquire mutex for disconnect (timeout), proceeding anyway\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        mosquitto_disconnect(mosq);
        if !mqtt_run_with_timeout(
            mosq,
            mqtt_cleanup_op_t::MQTT_OP_LOOP_STOP,
            2 as ::core::ffi::c_int,
            b"mosquitto_loop_stop\0".as_ptr() as *const ::core::ffi::c_char,
        ) {
            had_timeout = r#true != 0;
        }
        connected = r#false != 0;
        mqtt_presence_record_disconnected();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Forced disconnect without mutex\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return !had_timeout;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Calling mosquitto_disconnect...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    mosquitto_disconnect(mosq);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Calling mosquitto_loop_stop with 2 second timeout...\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if !mqtt_run_with_timeout(
        mosq,
        mqtt_cleanup_op_t::MQTT_OP_LOOP_STOP,
        2 as ::core::ffi::c_int,
        b"mosquitto_loop_stop\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        had_timeout = r#true != 0;
    }
    connected = r#false != 0;
    mqtt_presence_record_disconnected();
    pthread_mutex_unlock(&raw mut mqtt_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Disconnected\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return !had_timeout;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_disconnect() {
    mqtt_stop_presence_service(r#true != 0);
    mqtt_disconnect_internal();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_cleanup() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Starting cleanup...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    mqtt_stop_presence_service(r#true != 0);
    mqtt_stop_ha_services();
    ::core::ptr::write_volatile(&raw mut shutting_down, r#true != 0);
    ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
    if mosq.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: No client to clean up\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mqtt_config = ::core::ptr::null::<config_t>();
        if mqtt_library_initialized {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT: Calling mosquitto_lib_cleanup with 2 second timeout...\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if mqtt_run_with_timeout(
                ::core::ptr::null_mut::<mosquitto>(),
                mqtt_cleanup_op_t::MQTT_OP_LIB_CLEANUP,
                2 as ::core::ffi::c_int,
                b"mosquitto_lib_cleanup\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                mqtt_library_initialized = r#false != 0;
            }
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut had_timeout: bool = r#false != 0;
    if !mqtt_disconnect_internal() {
        had_timeout = r#true != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Attempting to acquire mutex for destroy...\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if !mqtt_try_lock_with_timeout(2000 as ::core::ffi::c_int) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Failed to acquire mutex for destroy (timeout), skipping remaining cleanup\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        mosq = ::core::ptr::null_mut::<mosquitto>();
        mqtt_config = ::core::ptr::null::<config_t>();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Skipped destroy and lib_cleanup due to mutex timeout\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Calling mosquitto_destroy with 2 second timeout...\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut destroyed: bool = mqtt_run_with_timeout(
        mosq,
        mqtt_cleanup_op_t::MQTT_OP_DESTROY,
        2 as ::core::ffi::c_int,
        b"mosquitto_destroy\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if destroyed {
        mosq = ::core::ptr::null_mut::<mosquitto>();
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: mosquitto_destroy timed out, setting mosq to NULL anyway\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        mosq = ::core::ptr::null_mut::<mosquitto>();
        had_timeout = r#true != 0;
    }
    mqtt_config = ::core::ptr::null::<config_t>();
    pthread_mutex_unlock(&raw mut mqtt_mutex);
    if had_timeout {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Skipping mosquitto_lib_cleanup due to previous timeouts\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Cleaned up (with warnings)\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if mqtt_library_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT: Calling mosquitto_lib_cleanup with 2 second timeout...\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        if mqtt_run_with_timeout(
            ::core::ptr::null_mut::<mosquitto>(),
            mqtt_cleanup_op_t::MQTT_OP_LIB_CLEANUP,
            2 as ::core::ffi::c_int,
            b"mosquitto_lib_cleanup\0".as_ptr() as *const ::core::ffi::c_char,
        ) {
            mqtt_library_initialized = r#false != 0;
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT: Cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_reinit(mut config: *const config_t) -> ::core::ffi::c_int {
    if config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT reinit: Invalid config pointer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT reinit: Starting hot-reload...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    mqtt_cleanup();
    ::core::ptr::write_volatile(&raw mut shutting_down, r#false != 0);
    ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
    if mqtt_init(config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT reinit: Failed to initialize MQTT client\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !(*config).mqtt_enabled {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT reinit: Default broker disabled; shared runtime ready\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if mqtt_connect() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT reinit: Failed to connect to MQTT broker, will retry automatically\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MQTT reinit: Connected to MQTT broker\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MQTT reinit: Hot-reload complete\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
