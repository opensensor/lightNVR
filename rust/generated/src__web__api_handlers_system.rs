use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    unsafe fn getpid() -> __pid_t;
    unsafe fn uname(__name: *mut utsname) -> ::core::ffi::c_int;
    unsafe fn getifaddrs(__ifap: *mut *mut ifaddrs) -> ::core::ffi::c_int;
    unsafe fn freeifaddrs(__ifa: *mut ifaddrs);
    unsafe fn inet_ntop(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_void,
        __buf: *mut ::core::ffi::c_char,
        __len: socklen_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn sqlite3_libversion() -> *const ::core::ffi::c_char;
    unsafe fn curl_version_info(stamp: CURLversion) -> *mut curl_version_info_data;
    unsafe fn uv_version_string() -> *const ::core::ffi::c_char;
    unsafe fn mbedtls_version_get_string_full(string: *mut ::core::ffi::c_char);
    unsafe fn avutil_version() -> ::core::ffi::c_uint;
    unsafe fn avcodec_version() -> ::core::ffi::c_uint;
    unsafe fn avformat_version() -> ::core::ffi::c_uint;
    unsafe fn swscale_version() -> ::core::ffi::c_uint;
    unsafe fn swresample_version() -> ::core::ffi::c_uint;
    unsafe fn system_health_capability_name(
        capability: system_health_capability_t,
    ) -> *const ::core::ffi::c_char;
    static mut g_config: config_t;
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
    unsafe fn cJSON_Print(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
    unsafe fn cJSON_AddNullToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddBoolToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        boolean: cJSON_bool,
    ) -> *mut cJSON;
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
    unsafe fn get_go2rtc_memory_usage(memory_usage: *mut ::core::ffi::c_ulonglong) -> bool;
    unsafe fn request_restart();
    unsafe fn get_process_uptime_seconds() -> ::core::ffi::c_double;
    unsafe fn httpd_authorize_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        user: *mut user_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_global_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn audit_log_operation(
        req: *const http_request_t,
        user: *const user_t,
        action: *const ::core::ffi::c_char,
        target_type: *const ::core::ffi::c_char,
        target_uuid: *const ::core::ffi::c_char,
        operation: *const ::core::ffi::c_char,
        outcome: *const ::core::ffi::c_char,
        context: *const cJSON,
    );
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn ensure_dir(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn initiate_shutdown();
    unsafe fn request_background_abort();
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_enabled_stream_count() -> ::core::ffi::c_int;
    unsafe fn get_recording_count(
        start_time: time_t,
        end_time: time_t,
        stream_name: *const ::core::ffi::c_char,
        has_detection: ::core::ffi::c_int,
        detection_label: *const ::core::ffi::c_char,
        protected_filter: ::core::ffi::c_int,
        allowed_streams: *const *const ::core::ffi::c_char,
        allowed_streams_count: ::core::ffi::c_int,
        tag_filter: *const ::core::ffi::c_char,
        capture_method_filter: *const ::core::ffi::c_char,
        include_in_progress: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_storage_bytes(stream_name: *const ::core::ffi::c_char) -> int64_t;
    unsafe fn add_cached_stream_storage_usage_to_json(
        json_obj: *mut cJSON,
        force_refresh: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_snapshot_copy(snapshot: *mut system_health_snapshot_t) -> bool;
    unsafe fn go2rtc_api_get_application_info(
        rtsp_port: *mut ::core::ffi::c_int,
        version: *mut ::core::ffi::c_char,
        version_size: size_t,
        revision: *mut ::core::ffi::c_char,
        revision_size: size_t,
    ) -> bool;
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
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
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
pub type socklen_t = __socklen_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const _SC_ARG_MAX: Self = Self(0);
    pub const _SC_CHILD_MAX: Self = Self(1);
    pub const _SC_CLK_TCK: Self = Self(2);
    pub const _SC_NGROUPS_MAX: Self = Self(3);
    pub const _SC_OPEN_MAX: Self = Self(4);
    pub const _SC_STREAM_MAX: Self = Self(5);
    pub const _SC_TZNAME_MAX: Self = Self(6);
    pub const _SC_JOB_CONTROL: Self = Self(7);
    pub const _SC_SAVED_IDS: Self = Self(8);
    pub const _SC_REALTIME_SIGNALS: Self = Self(9);
    pub const _SC_PRIORITY_SCHEDULING: Self = Self(10);
    pub const _SC_TIMERS: Self = Self(11);
    pub const _SC_ASYNCHRONOUS_IO: Self = Self(12);
    pub const _SC_PRIORITIZED_IO: Self = Self(13);
    pub const _SC_SYNCHRONIZED_IO: Self = Self(14);
    pub const _SC_FSYNC: Self = Self(15);
    pub const _SC_MAPPED_FILES: Self = Self(16);
    pub const _SC_MEMLOCK: Self = Self(17);
    pub const _SC_MEMLOCK_RANGE: Self = Self(18);
    pub const _SC_MEMORY_PROTECTION: Self = Self(19);
    pub const _SC_MESSAGE_PASSING: Self = Self(20);
    pub const _SC_SEMAPHORES: Self = Self(21);
    pub const _SC_SHARED_MEMORY_OBJECTS: Self = Self(22);
    pub const _SC_AIO_LISTIO_MAX: Self = Self(23);
    pub const _SC_AIO_MAX: Self = Self(24);
    pub const _SC_AIO_PRIO_DELTA_MAX: Self = Self(25);
    pub const _SC_DELAYTIMER_MAX: Self = Self(26);
    pub const _SC_MQ_OPEN_MAX: Self = Self(27);
    pub const _SC_MQ_PRIO_MAX: Self = Self(28);
    pub const _SC_VERSION: Self = Self(29);
    pub const _SC_PAGESIZE: Self = Self(30);
    pub const _SC_RTSIG_MAX: Self = Self(31);
    pub const _SC_SEM_NSEMS_MAX: Self = Self(32);
    pub const _SC_SEM_VALUE_MAX: Self = Self(33);
    pub const _SC_SIGQUEUE_MAX: Self = Self(34);
    pub const _SC_TIMER_MAX: Self = Self(35);
    pub const _SC_BC_BASE_MAX: Self = Self(36);
    pub const _SC_BC_DIM_MAX: Self = Self(37);
    pub const _SC_BC_SCALE_MAX: Self = Self(38);
    pub const _SC_BC_STRING_MAX: Self = Self(39);
    pub const _SC_COLL_WEIGHTS_MAX: Self = Self(40);
    pub const _SC_EQUIV_CLASS_MAX: Self = Self(41);
    pub const _SC_EXPR_NEST_MAX: Self = Self(42);
    pub const _SC_LINE_MAX: Self = Self(43);
    pub const _SC_RE_DUP_MAX: Self = Self(44);
    pub const _SC_CHARCLASS_NAME_MAX: Self = Self(45);
    pub const _SC_2_VERSION: Self = Self(46);
    pub const _SC_2_C_BIND: Self = Self(47);
    pub const _SC_2_C_DEV: Self = Self(48);
    pub const _SC_2_FORT_DEV: Self = Self(49);
    pub const _SC_2_FORT_RUN: Self = Self(50);
    pub const _SC_2_SW_DEV: Self = Self(51);
    pub const _SC_2_LOCALEDEF: Self = Self(52);
    pub const _SC_PII: Self = Self(53);
    pub const _SC_PII_XTI: Self = Self(54);
    pub const _SC_PII_SOCKET: Self = Self(55);
    pub const _SC_PII_INTERNET: Self = Self(56);
    pub const _SC_PII_OSI: Self = Self(57);
    pub const _SC_POLL: Self = Self(58);
    pub const _SC_SELECT: Self = Self(59);
    pub const _SC_UIO_MAXIOV: Self = Self(60);
    pub const _SC_IOV_MAX: Self = Self(60);
    pub const _SC_PII_INTERNET_STREAM: Self = Self(61);
    pub const _SC_PII_INTERNET_DGRAM: Self = Self(62);
    pub const _SC_PII_OSI_COTS: Self = Self(63);
    pub const _SC_PII_OSI_CLTS: Self = Self(64);
    pub const _SC_PII_OSI_M: Self = Self(65);
    pub const _SC_T_IOV_MAX: Self = Self(66);
    pub const _SC_THREADS: Self = Self(67);
    pub const _SC_THREAD_SAFE_FUNCTIONS: Self = Self(68);
    pub const _SC_GETGR_R_SIZE_MAX: Self = Self(69);
    pub const _SC_GETPW_R_SIZE_MAX: Self = Self(70);
    pub const _SC_LOGIN_NAME_MAX: Self = Self(71);
    pub const _SC_TTY_NAME_MAX: Self = Self(72);
    pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: Self = Self(73);
    pub const _SC_THREAD_KEYS_MAX: Self = Self(74);
    pub const _SC_THREAD_STACK_MIN: Self = Self(75);
    pub const _SC_THREAD_THREADS_MAX: Self = Self(76);
    pub const _SC_THREAD_ATTR_STACKADDR: Self = Self(77);
    pub const _SC_THREAD_ATTR_STACKSIZE: Self = Self(78);
    pub const _SC_THREAD_PRIORITY_SCHEDULING: Self = Self(79);
    pub const _SC_THREAD_PRIO_INHERIT: Self = Self(80);
    pub const _SC_THREAD_PRIO_PROTECT: Self = Self(81);
    pub const _SC_THREAD_PROCESS_SHARED: Self = Self(82);
    pub const _SC_NPROCESSORS_CONF: Self = Self(83);
    pub const _SC_NPROCESSORS_ONLN: Self = Self(84);
    pub const _SC_PHYS_PAGES: Self = Self(85);
    pub const _SC_AVPHYS_PAGES: Self = Self(86);
    pub const _SC_ATEXIT_MAX: Self = Self(87);
    pub const _SC_PASS_MAX: Self = Self(88);
    pub const _SC_XOPEN_VERSION: Self = Self(89);
    pub const _SC_XOPEN_XCU_VERSION: Self = Self(90);
    pub const _SC_XOPEN_UNIX: Self = Self(91);
    pub const _SC_XOPEN_CRYPT: Self = Self(92);
    pub const _SC_XOPEN_ENH_I18N: Self = Self(93);
    pub const _SC_XOPEN_SHM: Self = Self(94);
    pub const _SC_2_CHAR_TERM: Self = Self(95);
    pub const _SC_2_C_VERSION: Self = Self(96);
    pub const _SC_2_UPE: Self = Self(97);
    pub const _SC_XOPEN_XPG2: Self = Self(98);
    pub const _SC_XOPEN_XPG3: Self = Self(99);
    pub const _SC_XOPEN_XPG4: Self = Self(100);
    pub const _SC_CHAR_BIT: Self = Self(101);
    pub const _SC_CHAR_MAX: Self = Self(102);
    pub const _SC_CHAR_MIN: Self = Self(103);
    pub const _SC_INT_MAX: Self = Self(104);
    pub const _SC_INT_MIN: Self = Self(105);
    pub const _SC_LONG_BIT: Self = Self(106);
    pub const _SC_WORD_BIT: Self = Self(107);
    pub const _SC_MB_LEN_MAX: Self = Self(108);
    pub const _SC_NZERO: Self = Self(109);
    pub const _SC_SSIZE_MAX: Self = Self(110);
    pub const _SC_SCHAR_MAX: Self = Self(111);
    pub const _SC_SCHAR_MIN: Self = Self(112);
    pub const _SC_SHRT_MAX: Self = Self(113);
    pub const _SC_SHRT_MIN: Self = Self(114);
    pub const _SC_UCHAR_MAX: Self = Self(115);
    pub const _SC_UINT_MAX: Self = Self(116);
    pub const _SC_ULONG_MAX: Self = Self(117);
    pub const _SC_USHRT_MAX: Self = Self(118);
    pub const _SC_NL_ARGMAX: Self = Self(119);
    pub const _SC_NL_LANGMAX: Self = Self(120);
    pub const _SC_NL_MSGMAX: Self = Self(121);
    pub const _SC_NL_NMAX: Self = Self(122);
    pub const _SC_NL_SETMAX: Self = Self(123);
    pub const _SC_NL_TEXTMAX: Self = Self(124);
    pub const _SC_XBS5_ILP32_OFF32: Self = Self(125);
    pub const _SC_XBS5_ILP32_OFFBIG: Self = Self(126);
    pub const _SC_XBS5_LP64_OFF64: Self = Self(127);
    pub const _SC_XBS5_LPBIG_OFFBIG: Self = Self(128);
    pub const _SC_XOPEN_LEGACY: Self = Self(129);
    pub const _SC_XOPEN_REALTIME: Self = Self(130);
    pub const _SC_XOPEN_REALTIME_THREADS: Self = Self(131);
    pub const _SC_ADVISORY_INFO: Self = Self(132);
    pub const _SC_BARRIERS: Self = Self(133);
    pub const _SC_BASE: Self = Self(134);
    pub const _SC_C_LANG_SUPPORT: Self = Self(135);
    pub const _SC_C_LANG_SUPPORT_R: Self = Self(136);
    pub const _SC_CLOCK_SELECTION: Self = Self(137);
    pub const _SC_CPUTIME: Self = Self(138);
    pub const _SC_THREAD_CPUTIME: Self = Self(139);
    pub const _SC_DEVICE_IO: Self = Self(140);
    pub const _SC_DEVICE_SPECIFIC: Self = Self(141);
    pub const _SC_DEVICE_SPECIFIC_R: Self = Self(142);
    pub const _SC_FD_MGMT: Self = Self(143);
    pub const _SC_FIFO: Self = Self(144);
    pub const _SC_PIPE: Self = Self(145);
    pub const _SC_FILE_ATTRIBUTES: Self = Self(146);
    pub const _SC_FILE_LOCKING: Self = Self(147);
    pub const _SC_FILE_SYSTEM: Self = Self(148);
    pub const _SC_MONOTONIC_CLOCK: Self = Self(149);
    pub const _SC_MULTI_PROCESS: Self = Self(150);
    pub const _SC_SINGLE_PROCESS: Self = Self(151);
    pub const _SC_NETWORKING: Self = Self(152);
    pub const _SC_READER_WRITER_LOCKS: Self = Self(153);
    pub const _SC_SPIN_LOCKS: Self = Self(154);
    pub const _SC_REGEXP: Self = Self(155);
    pub const _SC_REGEX_VERSION: Self = Self(156);
    pub const _SC_SHELL: Self = Self(157);
    pub const _SC_SIGNALS: Self = Self(158);
    pub const _SC_SPAWN: Self = Self(159);
    pub const _SC_SPORADIC_SERVER: Self = Self(160);
    pub const _SC_THREAD_SPORADIC_SERVER: Self = Self(161);
    pub const _SC_SYSTEM_DATABASE: Self = Self(162);
    pub const _SC_SYSTEM_DATABASE_R: Self = Self(163);
    pub const _SC_TIMEOUTS: Self = Self(164);
    pub const _SC_TYPED_MEMORY_OBJECTS: Self = Self(165);
    pub const _SC_USER_GROUPS: Self = Self(166);
    pub const _SC_USER_GROUPS_R: Self = Self(167);
    pub const _SC_2_PBS: Self = Self(168);
    pub const _SC_2_PBS_ACCOUNTING: Self = Self(169);
    pub const _SC_2_PBS_LOCATE: Self = Self(170);
    pub const _SC_2_PBS_MESSAGE: Self = Self(171);
    pub const _SC_2_PBS_TRACK: Self = Self(172);
    pub const _SC_SYMLOOP_MAX: Self = Self(173);
    pub const _SC_STREAMS: Self = Self(174);
    pub const _SC_2_PBS_CHECKPOINT: Self = Self(175);
    pub const _SC_V6_ILP32_OFF32: Self = Self(176);
    pub const _SC_V6_ILP32_OFFBIG: Self = Self(177);
    pub const _SC_V6_LP64_OFF64: Self = Self(178);
    pub const _SC_V6_LPBIG_OFFBIG: Self = Self(179);
    pub const _SC_HOST_NAME_MAX: Self = Self(180);
    pub const _SC_TRACE: Self = Self(181);
    pub const _SC_TRACE_EVENT_FILTER: Self = Self(182);
    pub const _SC_TRACE_INHERIT: Self = Self(183);
    pub const _SC_TRACE_LOG: Self = Self(184);
    pub const _SC_LEVEL1_ICACHE_SIZE: Self = Self(185);
    pub const _SC_LEVEL1_ICACHE_ASSOC: Self = Self(186);
    pub const _SC_LEVEL1_ICACHE_LINESIZE: Self = Self(187);
    pub const _SC_LEVEL1_DCACHE_SIZE: Self = Self(188);
    pub const _SC_LEVEL1_DCACHE_ASSOC: Self = Self(189);
    pub const _SC_LEVEL1_DCACHE_LINESIZE: Self = Self(190);
    pub const _SC_LEVEL2_CACHE_SIZE: Self = Self(191);
    pub const _SC_LEVEL2_CACHE_ASSOC: Self = Self(192);
    pub const _SC_LEVEL2_CACHE_LINESIZE: Self = Self(193);
    pub const _SC_LEVEL3_CACHE_SIZE: Self = Self(194);
    pub const _SC_LEVEL3_CACHE_ASSOC: Self = Self(195);
    pub const _SC_LEVEL3_CACHE_LINESIZE: Self = Self(196);
    pub const _SC_LEVEL4_CACHE_SIZE: Self = Self(197);
    pub const _SC_LEVEL4_CACHE_ASSOC: Self = Self(198);
    pub const _SC_LEVEL4_CACHE_LINESIZE: Self = Self(199);
    pub const _SC_IPV6: Self = Self(235);
    pub const _SC_RAW_SOCKETS: Self = Self(236);
    pub const _SC_V7_ILP32_OFF32: Self = Self(237);
    pub const _SC_V7_ILP32_OFFBIG: Self = Self(238);
    pub const _SC_V7_LP64_OFF64: Self = Self(239);
    pub const _SC_V7_LPBIG_OFFBIG: Self = Self(240);
    pub const _SC_SS_REPL_MAX: Self = Self(241);
    pub const _SC_TRACE_EVENT_NAME_MAX: Self = Self(242);
    pub const _SC_TRACE_NAME_MAX: Self = Self(243);
    pub const _SC_TRACE_SYS_MAX: Self = Self(244);
    pub const _SC_TRACE_USER_EVENT_MAX: Self = Self(245);
    pub const _SC_XOPEN_STREAMS: Self = Self(246);
    pub const _SC_THREAD_ROBUST_PRIO_INHERIT: Self = Self(247);
    pub const _SC_THREAD_ROBUST_PRIO_PROTECT: Self = Self(248);
    pub const _SC_MINSIGSTKSZ: Self = Self(249);
    pub const _SC_SIGSTKSZ: Self = Self(250);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [::core::ffi::c_char; 65],
    pub nodename: [::core::ffi::c_char; 65],
    pub release: [::core::ffi::c_char; 65],
    pub version: [::core::ffi::c_char; 65],
    pub machine: [::core::ffi::c_char; 65],
    pub domainname: [::core::ffi::c_char; 65],
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
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = u16;
pub type uint8_t = u8;
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
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut ::core::ffi::c_char,
    pub ifa_flags: ::core::ffi::c_uint,
    pub ifa_addr: *mut sockaddr,
    pub ifa_netmask: *mut sockaddr,
    pub ifa_ifu: C2Rust_Unnamed_2,
    pub ifa_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub ifu_broadaddr: *mut sockaddr,
    pub ifu_dstaddr: *mut sockaddr,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const IFF_UP: Self = Self(1);
    pub const IFF_BROADCAST: Self = Self(2);
    pub const IFF_DEBUG: Self = Self(4);
    pub const IFF_LOOPBACK: Self = Self(8);
    pub const IFF_POINTOPOINT: Self = Self(16);
    pub const IFF_NOTRAILERS: Self = Self(32);
    pub const IFF_RUNNING: Self = Self(64);
    pub const IFF_NOARP: Self = Self(128);
    pub const IFF_PROMISC: Self = Self(256);
    pub const IFF_ALLMULTI: Self = Self(512);
    pub const IFF_MASTER: Self = Self(1024);
    pub const IFF_SLAVE: Self = Self(2048);
    pub const IFF_MULTICAST: Self = Self(4096);
    pub const IFF_PORTSEL: Self = Self(8192);
    pub const IFF_AUTOMEDIA: Self = Self(16384);
    pub const IFF_DYNAMIC: Self = Self(32768);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_ll {
    pub sll_family: ::core::ffi::c_ushort,
    pub sll_protocol: ::core::ffi::c_ushort,
    pub sll_ifindex: ::core::ffi::c_int,
    pub sll_hatype: ::core::ffi::c_ushort,
    pub sll_pkttype: ::core::ffi::c_uchar,
    pub sll_halen: ::core::ffi::c_uchar,
    pub sll_addr: [::core::ffi::c_uchar; 8],
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CURLversion(pub ::core::ffi::c_uint);
impl CURLversion {
    pub const CURLVERSION_FIRST: Self = Self(0);
    pub const CURLVERSION_SECOND: Self = Self(1);
    pub const CURLVERSION_THIRD: Self = Self(2);
    pub const CURLVERSION_FOURTH: Self = Self(3);
    pub const CURLVERSION_FIFTH: Self = Self(4);
    pub const CURLVERSION_SIXTH: Self = Self(5);
    pub const CURLVERSION_SEVENTH: Self = Self(6);
    pub const CURLVERSION_EIGHTH: Self = Self(7);
    pub const CURLVERSION_NINTH: Self = Self(8);
    pub const CURLVERSION_TENTH: Self = Self(9);
    pub const CURLVERSION_ELEVENTH: Self = Self(10);
    pub const CURLVERSION_TWELFTH: Self = Self(11);
    pub const CURLVERSION_LAST: Self = Self(12);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_version_info_data {
    pub age: CURLversion,
    pub version: *const ::core::ffi::c_char,
    pub version_num: ::core::ffi::c_uint,
    pub host: *const ::core::ffi::c_char,
    pub features: ::core::ffi::c_int,
    pub ssl_version: *const ::core::ffi::c_char,
    pub ssl_version_num: ::core::ffi::c_long,
    pub libz_version: *const ::core::ffi::c_char,
    pub protocols: *const *const ::core::ffi::c_char,
    pub ares: *const ::core::ffi::c_char,
    pub ares_num: ::core::ffi::c_int,
    pub libidn: *const ::core::ffi::c_char,
    pub iconv_ver_num: ::core::ffi::c_int,
    pub libssh_version: *const ::core::ffi::c_char,
    pub brotli_ver_num: ::core::ffi::c_uint,
    pub brotli_version: *const ::core::ffi::c_char,
    pub nghttp2_ver_num: ::core::ffi::c_uint,
    pub nghttp2_version: *const ::core::ffi::c_char,
    pub quic_version: *const ::core::ffi::c_char,
    pub cainfo: *const ::core::ffi::c_char,
    pub capath: *const ::core::ffi::c_char,
    pub zstd_ver_num: ::core::ffi::c_uint,
    pub zstd_version: *const ::core::ffi::c_char,
    pub hyper_version: *const ::core::ffi::c_char,
    pub gsasl_version: *const ::core::ffi::c_char,
    pub feature_names: *const *const ::core::ffi::c_char,
    pub rtmp_version: *const ::core::ffi::c_char,
}
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_info_interface_t {
    pub name: [::core::ffi::c_char; 16],
    pub ipv4: [::core::ffi::c_char; 16],
    pub ipv6: [::core::ffi::c_char; 46],
    pub mac: [::core::ffi::c_char; 18],
    pub ipv6_link_local: bool,
    pub flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_recording_aggregate_cache_t {
    pub mutex: pthread_mutex_t,
    pub refreshed_at: time_t,
    pub count: ::core::ffi::c_int,
    pub bytes: int64_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_evaluation_t {
    pub decision: authorization_decision_t,
    pub source: authorization_decision_source_t,
    pub policy_version: int64_t,
    pub grant_uuid: [::core::ffi::c_char; 37],
    pub token_uuid: [::core::ffi::c_char; 37],
    pub role_uuid: [::core::ffi::c_char; 37],
    pub role_name: [::core::ffi::c_char; 128],
    pub explanation: [::core::ffi::c_char; 256],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_source_t(pub ::core::ffi::c_uint);
impl authorization_decision_source_t {
    pub const AUTHZ_SOURCE_NONE: Self = Self(0);
    pub const AUTHZ_SOURCE_LEGACY_ROLE: Self = Self(1);
    pub const AUTHZ_SOURCE_POLICY_GRANT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_t(pub ::core::ffi::c_uint);
impl authorization_decision_t {
    pub const AUTHZ_DECISION_DENY: Self = Self(0);
    pub const AUTHZ_DECISION_ALLOW: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 256],
    pub address: [::core::ffi::c_char; 512],
    pub legacy_tags: [::core::ffi::c_char; 256],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub location_name: [::core::ffi::c_char; 128],
    pub location_path: [::core::ffi::c_char; 1024],
    pub location_ancestor_uuids: [[::core::ffi::c_char; 37]; 32],
    pub location_depth: ::core::ffi::c_int,
    pub tags: [fleet_camera_tag_t; 64],
    pub tag_count: ::core::ffi::c_int,
    pub manufacturer: [::core::ffi::c_char; 128],
    pub model: [::core::ffi::c_char; 128],
    pub enabled: bool,
    pub streaming_enabled: bool,
    pub record: bool,
    pub detection_based_recording: bool,
    pub is_onvif: bool,
    pub ptz_enabled: bool,
    pub backchannel_enabled: bool,
    pub health: fleet_health_state_t,
    pub availability: fleet_availability_state_t,
    pub health_changed_at: int64_t,
    pub first_video_at: int64_t,
    pub last_frame_ts: int64_t,
    pub last_recording_at: int64_t,
    pub current_fps: ::core::ffi::c_double,
    pub recording_active: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct fleet_availability_state_t(pub ::core::ffi::c_uint);
impl fleet_availability_state_t {
    pub const FLEET_AVAILABILITY_LIVE: Self = Self(0);
    pub const FLEET_AVAILABILITY_OFFLINE: Self = Self(1);
    pub const FLEET_AVAILABILITY_NEVER_CONNECTED: Self = Self(2);
    pub const FLEET_AVAILABILITY_DISABLED: Self = Self(3);
    pub const FLEET_AVAILABILITY_COUNT: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct fleet_health_state_t(pub ::core::ffi::c_uint);
impl fleet_health_state_t {
    pub const FLEET_HEALTH_UNKNOWN: Self = Self(0);
    pub const FLEET_HEALTH_UP: Self = Self(1);
    pub const FLEET_HEALTH_DEGRADED: Self = Self(2);
    pub const FLEET_HEALTH_DOWN: Self = Self(3);
    pub const FLEET_HEALTH_DISABLED: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_tag_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 256],
}
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PF_PACKET: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const AF_PACKET: ::core::ffi::c_int = PF_PACKET;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const LLHTTP_VERSION_MAJOR: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const LLHTTP_VERSION_MINOR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LLHTTP_VERSION_PATCH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SystemAPI\0") };
pub const SYSTEM_RECORDING_AGGREGATE_TTL_SECONDS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const SYSTEM_INFO_RESPONSE_TTL_SECONDS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
static mut recording_aggregate_cache: system_recording_aggregate_cache_t =
    system_recording_aggregate_cache_t {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0 as ::core::ffi::c_int,
                __count: 0 as ::core::ffi::c_uint,
                __owner: 0 as ::core::ffi::c_int,
                __nusers: 0 as ::core::ffi::c_uint,
                __kind: C2Rust_Unnamed_4::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
                __spins: 0 as ::core::ffi::c_short,
                __glibc_reserved: 0 as ::core::ffi::c_short,
                __list: __pthread_internal_list {
                    __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                    __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
                },
            },
        },
        refreshed_at: 0 as time_t,
        count: 0 as ::core::ffi::c_int,
        bytes: 0 as int64_t,
    };
static mut system_info_response_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_4::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut system_info_response_json: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut system_info_response_refreshed_at: time_t = 0 as time_t;
static mut system_info_response_snapshot_sequence: uint64_t = 0 as uint64_t;
unsafe extern "C" fn find_health_observation(
    mut snapshot: *const system_health_snapshot_t,
    mut metric: *const ::core::ffi::c_char,
    mut resource: *const ::core::ffi::c_char,
) -> *const system_health_observation_t {
    if snapshot.is_null() || metric.is_null() {
        return ::core::ptr::null::<system_health_observation_t>();
    }
    let mut index: size_t = 0 as size_t;
    while index < (*snapshot).observation_count {
        let mut item: *const system_health_observation_t = (&raw const (*snapshot).observations
            as *const system_health_observation_t)
            .offset(index as isize);
        if strcmp(
            &raw const (*item).metric as *const ::core::ffi::c_char,
            metric,
        ) == 0 as ::core::ffi::c_int
            && (resource.is_null()
                || strcmp(
                    &raw const (*item).resource_id as *const ::core::ffi::c_char,
                    resource,
                ) == 0 as ::core::ffi::c_int)
        {
            return item;
        }
        index = index.wrapping_add(1);
    }
    return ::core::ptr::null::<system_health_observation_t>();
}
unsafe extern "C" fn find_effective_health_observation(
    mut snapshot: *const system_health_snapshot_t,
    mut container_metric: *const ::core::ffi::c_char,
    mut host_metric: *const ::core::ffi::c_char,
) -> *const system_health_observation_t {
    let mut container_item: *const system_health_observation_t = find_health_observation(
        snapshot,
        container_metric,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !container_item.is_null()
        && (*container_item).capability.0
            == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    {
        return container_item;
    }
    let mut host_item: *const system_health_observation_t = find_health_observation(
        snapshot,
        host_metric,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    return if !host_item.is_null() {
        host_item
    } else {
        container_item
    };
}
unsafe extern "C" fn health_observation_value(
    mut observation: *const system_health_observation_t,
    mut value: *mut ::core::ffi::c_double,
) -> bool {
    if observation.is_null()
        || !(*observation).value_valid
        || (*observation).capability.0
            != system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
    {
        return r#false != 0;
    }
    *value = (*observation).value;
    return r#true != 0;
}
unsafe extern "C" fn add_health_number(
    mut object: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut observation: *const system_health_observation_t,
    mut scale: ::core::ffi::c_double,
) {
    let mut value: ::core::ffi::c_double = 0.;
    if health_observation_value(observation, &raw mut value) {
        cJSON_AddNumberToObject(object, name, value * scale);
    } else {
        cJSON_AddNullToObject(object, name);
    };
}
unsafe extern "C" fn health_capability(
    mut observation: *const system_health_observation_t,
) -> *const ::core::ffi::c_char {
    return if !observation.is_null() {
        system_health_capability_name((*observation).capability)
    } else {
        b"error\0".as_ptr() as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn get_cached_recording_aggregates(
    mut count: *mut ::core::ffi::c_int,
    mut bytes: *mut int64_t,
) {
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_lock(&raw mut recording_aggregate_cache.mutex);
    if recording_aggregate_cache.refreshed_at == 0 as time_t
        || now - recording_aggregate_cache.refreshed_at
            >= SYSTEM_RECORDING_AGGREGATE_TTL_SECONDS as time_t
    {
        let mut refreshed_count: ::core::ffi::c_int = get_recording_count(
            0 as time_t,
            0 as time_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
            -1 as ::core::ffi::c_int,
            ::core::ptr::null::<*const ::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            r#false != 0,
        );
        let mut refreshed_bytes: int64_t =
            get_stream_storage_bytes(::core::ptr::null::<::core::ffi::c_char>());
        if refreshed_count >= 0 as ::core::ffi::c_int {
            recording_aggregate_cache.count = refreshed_count;
        }
        if refreshed_bytes >= 0 as int64_t {
            recording_aggregate_cache.bytes = refreshed_bytes;
        }
        recording_aggregate_cache.refreshed_at = now;
    }
    *count = recording_aggregate_cache.count;
    *bytes = recording_aggregate_cache.bytes;
    pthread_mutex_unlock(&raw mut recording_aggregate_cache.mutex);
}
unsafe extern "C" fn trim_copy_value(
    mut dest: *mut ::core::ffi::c_char,
    mut dest_size: size_t,
    mut src: *const ::core::ffi::c_char,
) {
    if dest.is_null() || dest_size == 0 as size_t {
        return;
    }
    *dest.offset(0isize) = '\0' as ::core::ffi::c_char;
    if src.is_null() {
        return;
    }
    let mut start: *const ::core::ffi::c_char = ltrim_pos(src);
    let mut end: *const ::core::ffi::c_char =
        rtrim_pos(src, 0 as size_t).offset(-(1 as ::core::ffi::c_int as isize));
    if end.offset_from(start) >= 2isize
        && (*start as ::core::ffi::c_int == '"' as ::core::ffi::c_int
            && *end as ::core::ffi::c_int == '"' as ::core::ffi::c_int
            || *start as ::core::ffi::c_int == '\'' as ::core::ffi::c_int
                && *end as ::core::ffi::c_int == '\'' as ::core::ffi::c_int)
    {
        start = start.offset(1);
        end = end.offset(-1);
    }
    let mut len: size_t = end.offset_from(start) as size_t;
    if len >= dest_size {
        len = dest_size.wrapping_sub(1 as size_t);
    }
    memcpy(
        dest as *mut ::core::ffi::c_void,
        start as *const ::core::ffi::c_void,
        len,
    );
    *dest.offset(len as isize) = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn read_os_release_value(
    mut key: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_char,
    mut value_size: size_t,
) -> bool {
    if key.is_null() || value.is_null() || value_size == 0 as size_t {
        return r#false != 0;
    }
    let mut fp: *mut FILE = fopen(
        b"/etc/os-release\0".as_ptr() as *const ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if fp.is_null() {
        return r#false != 0;
    }
    let mut found: bool = r#false != 0;
    let mut line: [::core::ffi::c_char; 512] = [0; 512];
    let mut key_len: size_t = strlen(key);
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int,
        fp,
    )
    .is_null()
    {
        if !(strncmp(&raw mut line as *mut ::core::ffi::c_char, key, key_len)
            == 0 as ::core::ffi::c_int
            && line[key_len] as ::core::ffi::c_int == '=' as ::core::ffi::c_int)
        {
            continue;
        }
        trim_copy_value(
            value,
            value_size,
            (&raw mut line as *mut ::core::ffi::c_char)
                .offset(key_len as isize)
                .offset(1 as ::core::ffi::c_int as isize),
        );
        found = *value.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
        break;
    }
    fclose(fp);
    return found;
}
unsafe extern "C" fn add_version_entry(
    mut items: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut category: *const ::core::ffi::c_char,
    mut version: *const ::core::ffi::c_char,
    mut details: *const ::core::ffi::c_char,
) {
    if items.is_null() || name.is_null() || category.is_null() || version.is_null() {
        return;
    }
    let mut item: *mut cJSON = cJSON_CreateObject();
    if item.is_null() {
        return;
    }
    cJSON_AddStringToObject(item, b"name\0".as_ptr() as *const ::core::ffi::c_char, name);
    cJSON_AddStringToObject(
        item,
        b"category\0".as_ptr() as *const ::core::ffi::c_char,
        category,
    );
    cJSON_AddStringToObject(
        item,
        b"version\0".as_ptr() as *const ::core::ffi::c_char,
        if *version.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            version
        } else {
            b"Unknown\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if !details.is_null()
        && *details.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        cJSON_AddStringToObject(
            item,
            b"details\0".as_ptr() as *const ::core::ffi::c_char,
            details,
        );
    }
    cJSON_AddItemToArray(items, item);
}
unsafe extern "C" fn format_triplet_version(
    mut version_int: ::core::ffi::c_uint,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_size: size_t,
) {
    if buffer.is_null() || buffer_size == 0 as size_t {
        return;
    }
    snprintf(
        buffer,
        buffer_size,
        b"%u.%u.%u\0".as_ptr() as *const ::core::ffi::c_char,
        version_int >> 16 as ::core::ffi::c_int,
        (version_int & 0xff00 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int,
        version_int & 0xff as ::core::ffi::c_uint,
    );
}
pub const SYSTEM_INFO_MAX_INTERFACES: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
unsafe extern "C" fn find_or_add_interface(
    mut list: *mut system_info_interface_t,
    mut count: *mut size_t,
    mut name: *const ::core::ffi::c_char,
) -> *mut system_info_interface_t {
    let mut i: size_t = 0 as size_t;
    while i < *count {
        if strcmp(
            &raw mut (*list.offset(i as isize)).name as *mut ::core::ffi::c_char,
            name,
        ) == 0 as ::core::ffi::c_int
        {
            return list.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    if *count >= SYSTEM_INFO_MAX_INTERFACES as size_t {
        return ::core::ptr::null_mut::<system_info_interface_t>();
    }
    let c2rust_fresh0 = *count;
    *count = (*count).wrapping_add(1);
    let mut entry: *mut system_info_interface_t = list.offset(c2rust_fresh0 as isize);
    memset(
        entry as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_info_interface_t>(),
    );
    safe_strcpy(
        &raw mut (*entry).name as *mut ::core::ffi::c_char,
        name,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    return entry;
}
unsafe extern "C" fn add_network_interfaces_json(mut interfaces: *mut cJSON) {
    let mut ifaddr: *mut ifaddrs = ::core::ptr::null_mut::<ifaddrs>();
    if getifaddrs(&raw mut ifaddr) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"getifaddrs failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return;
    }
    let mut list: [system_info_interface_t; 64] = [system_info_interface_t {
        name: [0; 16],
        ipv4: [0; 16],
        ipv6: [0; 46],
        mac: [0; 18],
        ipv6_link_local: false,
        flags: 0,
    }; 64];
    let mut count: size_t = 0 as size_t;
    let mut ifa: *const ifaddrs = ifaddr;
    while !ifa.is_null() {
        if !((*ifa).ifa_name.is_null() || (*ifa).ifa_flags & C2Rust_Unnamed_3::IFF_LOOPBACK.0 != 0)
        {
            let mut entry: *mut system_info_interface_t = find_or_add_interface(
                &raw mut list as *mut system_info_interface_t,
                &raw mut count,
                (*ifa).ifa_name,
            );
            if !entry.is_null() {
                (*entry).flags |= (*ifa).ifa_flags;
                if !(*ifa).ifa_addr.is_null() {
                    let mut family: ::core::ffi::c_int =
                        (*(*ifa).ifa_addr).sa_family as ::core::ffi::c_int;
                    if family == AF_INET && (*entry).ipv4[0usize] == 0 {
                        let mut r#in: *const sockaddr_in = (*ifa).ifa_addr as *const sockaddr_in;
                        inet_ntop(
                            AF_INET,
                            &raw const (*r#in).sin_addr as *const ::core::ffi::c_void,
                            &raw mut (*entry).ipv4 as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as socklen_t,
                        );
                    } else if family == AF_INET6 {
                        let mut in6: *const sockaddr_in6 = (*ifa).ifa_addr as *const sockaddr_in6;
                        let mut link_local: bool = ({
                            let mut __a: *const in6_addr = &raw const (*in6).sin6_addr;
                            ((*__a).__in6_u.__u6_addr32[0usize]
                                & __bswap_32(0xffc00000 as __uint32_t)
                                == __bswap_32(0xfe800000 as __uint32_t))
                                as ::core::ffi::c_int
                        }) != 0;
                        if (*entry).ipv6[0usize] == 0
                            || (*entry).ipv6_link_local as ::core::ffi::c_int != 0 && !link_local
                        {
                            inet_ntop(
                                AF_INET6,
                                &raw const (*in6).sin6_addr as *const ::core::ffi::c_void,
                                &raw mut (*entry).ipv6 as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 46]>() as socklen_t,
                            );
                            (*entry).ipv6_link_local = link_local;
                        }
                    } else if family == AF_PACKET {
                        let mut ll: *const sockaddr_ll = (*ifa).ifa_addr as *const sockaddr_ll;
                        let mut nonzero: bool = r#false != 0;
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i < (*ll).sll_halen as ::core::ffi::c_int
                            && i < 6 as ::core::ffi::c_int
                        {
                            if (*ll).sll_addr[i as usize] != 0 {
                                nonzero = r#true != 0;
                            }
                            i += 1;
                        }
                        if (*ll).sll_halen as ::core::ffi::c_int == 6 as ::core::ffi::c_int
                            && nonzero as ::core::ffi::c_int != 0
                        {
                            snprintf(
                                &raw mut (*entry).mac as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 18]>(),
                                b"%02x:%02x:%02x:%02x:%02x:%02x\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*ll).sll_addr[0usize] as ::core::ffi::c_int,
                                (*ll).sll_addr[1usize] as ::core::ffi::c_int,
                                (*ll).sll_addr[2usize] as ::core::ffi::c_int,
                                (*ll).sll_addr[3usize] as ::core::ffi::c_int,
                                (*ll).sll_addr[4usize] as ::core::ffi::c_int,
                                (*ll).sll_addr[5usize] as ::core::ffi::c_int,
                            );
                        }
                    }
                }
            }
        }
        ifa = (*ifa).ifa_next;
    }
    freeifaddrs(ifaddr);
    let mut i_0: size_t = 0 as size_t;
    while i_0 < count {
        let mut entry_0: *const system_info_interface_t =
            (&raw mut list as *mut system_info_interface_t).offset(i_0 as isize);
        let mut has_ipv6: bool =
            (*entry_0).ipv6[0usize] as ::core::ffi::c_int != 0 && !(*entry_0).ipv6_link_local;
        let mut address: *const ::core::ffi::c_char =
            if (*entry_0).ipv4[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*entry_0).ipv4 as *const ::core::ffi::c_char
            } else if has_ipv6 as ::core::ffi::c_int != 0 {
                &raw const (*entry_0).ipv6 as *const ::core::ffi::c_char
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            };
        if !((*entry_0).flags & C2Rust_Unnamed_3::IFF_UP.0 == 0 && address.is_null()) {
            let mut iface: *mut cJSON = cJSON_CreateObject();
            if !iface.is_null() {
                cJSON_AddStringToObject(
                    iface,
                    b"name\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*entry_0).name as *const ::core::ffi::c_char,
                );
                if !address.is_null() {
                    cJSON_AddStringToObject(
                        iface,
                        b"address\0".as_ptr() as *const ::core::ffi::c_char,
                        address,
                    );
                } else {
                    cJSON_AddNullToObject(
                        iface,
                        b"address\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                if (*entry_0).ipv4[0usize] as ::core::ffi::c_int != 0
                    && has_ipv6 as ::core::ffi::c_int != 0
                {
                    cJSON_AddStringToObject(
                        iface,
                        b"ipv6\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw const (*entry_0).ipv6 as *const ::core::ffi::c_char,
                    );
                }
                cJSON_AddStringToObject(
                    iface,
                    b"mac\0".as_ptr() as *const ::core::ffi::c_char,
                    if (*entry_0).mac[0usize] as ::core::ffi::c_int != 0 {
                        &raw const (*entry_0).mac as *const ::core::ffi::c_char
                    } else {
                        b"Unavailable\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                cJSON_AddBoolToObject(
                    iface,
                    b"up\0".as_ptr() as *const ::core::ffi::c_char,
                    ((*entry_0).flags & C2Rust_Unnamed_3::IFF_RUNNING.0 != 0 as ::core::ffi::c_uint)
                        as ::core::ffi::c_int,
                );
                cJSON_AddStringToObject(
                    iface,
                    b"capability\0".as_ptr() as *const ::core::ffi::c_char,
                    b"available\0".as_ptr() as *const ::core::ffi::c_char,
                );
                cJSON_AddItemToArray(interfaces, iface);
            }
        }
        i_0 = i_0.wrapping_add(1);
    }
}
unsafe extern "C" fn add_versions_to_json(mut info: *mut cJSON) {
    if info.is_null() {
        return;
    }
    let mut versions: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if versions.is_null() || items.is_null() {
        if !versions.is_null() {
            cJSON_Delete(versions);
        }
        if !items.is_null() {
            cJSON_Delete(items);
        }
        return;
    }
    let mut details: [::core::ffi::c_char; 256] = [0; 256];
    if LIGHTNVR_GIT_COMMIT[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        snprintf(
            &raw mut details as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"Build date %s \xE2\x80\xA2 commit %s\0".as_ptr() as *const ::core::ffi::c_char,
            LIGHTNVR_BUILD_DATE.as_ptr(),
            LIGHTNVR_GIT_COMMIT.as_ptr(),
        );
    } else {
        snprintf(
            &raw mut details as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"Build date %s\0".as_ptr() as *const ::core::ffi::c_char,
            LIGHTNVR_BUILD_DATE.as_ptr(),
        );
    }
    add_version_entry(
        items,
        b"LightNVR\0".as_ptr() as *const ::core::ffi::c_char,
        b"Application\0".as_ptr() as *const ::core::ffi::c_char,
        LIGHTNVR_VERSION_STRING.as_ptr(),
        &raw mut details as *mut ::core::ffi::c_char,
    );
    let mut system_info: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    if uname(&raw mut system_info) == 0 as ::core::ffi::c_int {
        let mut pretty_name: [::core::ffi::c_char; 256] = [
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
        let mut name: [::core::ffi::c_char; 128] = [
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
        let mut version_id: [::core::ffi::c_char; 128] = [
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
        let mut os_version: [::core::ffi::c_char; 256] = [
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
        if read_os_release_value(
            b"PRETTY_NAME\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut pretty_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        ) {
            safe_strcpy(
                &raw mut os_version as *mut ::core::ffi::c_char,
                &raw mut pretty_name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else if read_os_release_value(
            b"NAME\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        ) {
            if read_os_release_value(
                b"VERSION_ID\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut version_id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            ) {
                snprintf(
                    &raw mut os_version as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut name as *mut ::core::ffi::c_char,
                    &raw mut version_id as *mut ::core::ffi::c_char,
                );
            } else {
                safe_strcpy(
                    &raw mut os_version as *mut ::core::ffi::c_char,
                    &raw mut name as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    0 as size_t,
                );
            }
        } else {
            safe_strcpy(
                &raw mut os_version as *mut ::core::ffi::c_char,
                &raw mut system_info.sysname as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        }
        snprintf(
            &raw mut details as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"%s %s \xE2\x80\xA2 %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut system_info.sysname as *mut ::core::ffi::c_char,
            &raw mut system_info.release as *mut ::core::ffi::c_char,
            &raw mut system_info.machine as *mut ::core::ffi::c_char,
        );
        add_version_entry(
            items,
            b"Base OS\0".as_ptr() as *const ::core::ffi::c_char,
            b"OS\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut os_version as *mut ::core::ffi::c_char,
            &raw mut details as *mut ::core::ffi::c_char,
        );
    }
    let mut go2rtc_version: [::core::ffi::c_char; 64] = [
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
    let mut go2rtc_revision: [::core::ffi::c_char; 64] = [
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
    let mut rtsp_port: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if go2rtc_api_get_application_info(
        &raw mut rtsp_port,
        &raw mut go2rtc_version as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw mut go2rtc_revision as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) {
        snprintf(
            &raw mut details as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"RTSP port %d%s%s\0".as_ptr() as *const ::core::ffi::c_char,
            if rtsp_port > 0 as ::core::ffi::c_int {
                rtsp_port
            } else {
                8554 as ::core::ffi::c_int
            },
            if go2rtc_revision[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                b" \xE2\x80\xA2 revision \0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            if go2rtc_revision[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                &raw mut go2rtc_revision as *mut ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        add_version_entry(
            items,
            b"go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
            b"Service\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut go2rtc_version as *mut ::core::ffi::c_char,
            &raw mut details as *mut ::core::ffi::c_char,
        );
    } else {
        add_version_entry(
            items,
            b"go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
            b"Service\0".as_ptr() as *const ::core::ffi::c_char,
            b"Unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            b"go2rtc API not reachable\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    add_version_entry(
        items,
        b"SQLite\0".as_ptr() as *const ::core::ffi::c_char,
        b"Library\0".as_ptr() as *const ::core::ffi::c_char,
        sqlite3_libversion(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut curl_info: *const curl_version_info_data =
        curl_version_info(CURLversion::CURLVERSION_TWELFTH);
    if !curl_info.is_null() && !(*curl_info).version.is_null() {
        let mut curl_details: [::core::ffi::c_char; 256] = [
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
        if !(*curl_info).ssl_version.is_null() && !(*curl_info).libz_version.is_null() {
            snprintf(
                &raw mut curl_details as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                b"%s \xE2\x80\xA2 zlib %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*curl_info).ssl_version,
                (*curl_info).libz_version,
            );
        } else if !(*curl_info).ssl_version.is_null() {
            safe_strcpy(
                &raw mut curl_details as *mut ::core::ffi::c_char,
                (*curl_info).ssl_version,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else if !(*curl_info).libz_version.is_null() {
            snprintf(
                &raw mut curl_details as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                b"zlib %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*curl_info).libz_version,
            );
        }
        add_version_entry(
            items,
            b"libcurl\0".as_ptr() as *const ::core::ffi::c_char,
            b"Library\0".as_ptr() as *const ::core::ffi::c_char,
            (*curl_info).version,
            &raw mut curl_details as *mut ::core::ffi::c_char,
        );
    }
    let mut mbedtls_version: [::core::ffi::c_char; 64] = [
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
    mbedtls_version_get_string_full(&raw mut mbedtls_version as *mut ::core::ffi::c_char);
    add_version_entry(
        items,
        b"mbedTLS\0".as_ptr() as *const ::core::ffi::c_char,
        b"Library\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut mbedtls_version as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    add_version_entry(
        items,
        b"libuv\0".as_ptr() as *const ::core::ffi::c_char,
        b"Library\0".as_ptr() as *const ::core::ffi::c_char,
        uv_version_string(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut llhttp_version: [::core::ffi::c_char; 32] = [
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
    ];
    snprintf(
        &raw mut llhttp_version as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%d.%d.%d\0".as_ptr() as *const ::core::ffi::c_char,
        LLHTTP_VERSION_MAJOR,
        LLHTTP_VERSION_MINOR,
        LLHTTP_VERSION_PATCH,
    );
    add_version_entry(
        items,
        b"llhttp\0".as_ptr() as *const ::core::ffi::c_char,
        b"Library\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut llhttp_version as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut version_buf: [::core::ffi::c_char; 32] = [
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
    ];
    format_triplet_version(
        avformat_version(),
        &raw mut version_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    );
    add_version_entry(
        items,
        b"libavformat\0".as_ptr() as *const ::core::ffi::c_char,
        b"Library\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut version_buf as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    format_triplet_version(
        avcodec_version(),
        &raw mut version_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    );
    add_version_entry(
        items,
        b"libavcodec\0".as_ptr() as *const ::core::ffi::c_char,
        b"Library\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut version_buf as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    format_triplet_version(
        avutil_version(),
        &raw mut version_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    );
    add_version_entry(
        items,
        b"libavutil\0".as_ptr() as *const ::core::ffi::c_char,
        b"Library\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut version_buf as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    format_triplet_version(
        swscale_version(),
        &raw mut version_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    );
    add_version_entry(
        items,
        b"libswscale\0".as_ptr() as *const ::core::ffi::c_char,
        b"Library\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut version_buf as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    format_triplet_version(
        swresample_version(),
        &raw mut version_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    );
    add_version_entry(
        items,
        b"libswresample\0".as_ptr() as *const ::core::ffi::c_char,
        b"Library\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut version_buf as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    cJSON_AddItemToObject(
        versions,
        b"items\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    cJSON_AddItemToObject(
        info,
        b"versions\0".as_ptr() as *const ::core::ffi::c_char,
        versions,
    );
}
unsafe extern "C" fn get_detector_memory_usage(
    mut memory_usage: *mut ::core::ffi::c_ulonglong,
) -> bool {
    if memory_usage.is_null() {
        return r#false != 0;
    }
    *memory_usage = 0 as ::core::ffi::c_ulonglong;
    let mut pid: pid_t = -1 as pid_t;
    let mut self_0: pid_t = getpid();
    let mut proc_dir: *mut DIR = opendir(b"/proc\0".as_ptr() as *const ::core::ffi::c_char);
    if proc_dir.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to open /proc for light-object-detect search\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut entry: *const dirent = ::core::ptr::null::<dirent>();
    loop {
        entry = readdir(proc_dir);
        if entry.is_null() {
            break;
        }
        let mut d: *const ::core::ffi::c_char =
            &raw const (*entry).d_name as *const ::core::ffi::c_char;
        if (*d as ::core::ffi::c_int) < '1' as ::core::ffi::c_int
            || *d as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            continue;
        }
        let mut ep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut candidate: pid_t = strtol(d, &raw mut ep, 10 as ::core::ffi::c_int) as pid_t;
        if *ep as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            || candidate <= 0 as ::core::ffi::c_int
            || candidate == self_0
        {
            continue;
        }
        let mut cmdline_path: [::core::ffi::c_char; 64] = [0; 64];
        snprintf(
            &raw mut cmdline_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"/proc/%d/cmdline\0".as_ptr() as *const ::core::ffi::c_char,
            candidate,
        );
        let mut cf: *mut FILE = fopen(
            &raw mut cmdline_path as *mut ::core::ffi::c_char,
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cf.is_null() {
            continue;
        }
        let mut cmdline: [::core::ffi::c_char; 512] = [
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
        ];
        let mut bytes: size_t = fread(
            &raw mut cmdline as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            1 as size_t,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1 as size_t),
            cf,
        ) as size_t;
        fclose(cf);
        let mut i: size_t = 0 as size_t;
        while i < bytes {
            if cmdline[i] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                cmdline[i] = ' ' as ::core::ffi::c_char;
            }
            i = i.wrapping_add(1);
        }
        if strstr(
            &raw mut cmdline as *mut ::core::ffi::c_char,
            b"light-object-detect\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            continue;
        }
        pid = candidate;
        break;
    }
    closedir(proc_dir);
    if pid <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"No light-object-detect process found; memory usage is zero\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#true != 0;
    }
    let mut status_path: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut status_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"/proc/%d/status\0".as_ptr() as *const ::core::ffi::c_char,
        pid,
    );
    let mut status_file: *mut FILE = fopen(
        &raw mut status_path as *mut ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if status_file.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to open %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut status_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    }
    let mut status_line: [::core::ffi::c_char; 256] = [0; 256];
    let mut vm_rss: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    while !fgets(
        &raw mut status_line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
        status_file,
    )
    .is_null()
    {
        if strncmp(
            &raw mut status_line as *mut ::core::ffi::c_char,
            b"VmRSS:\0".as_ptr() as *const ::core::ffi::c_char,
            6 as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            continue;
        }
        let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        vm_rss = strtoul(
            (&raw mut status_line as *mut ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize),
            &raw mut endptr,
            10 as ::core::ffi::c_int,
        );
        break;
    }
    fclose(status_file);
    *memory_usage = vm_rss.wrapping_mul(1024 as ::core::ffi::c_ulong) as ::core::ffi::c_ulonglong;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"light-object-detect memory usage (PID %d): %llu bytes\0".as_ptr()
            as *const ::core::ffi::c_char,
        pid,
        *memory_usage,
    );
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_system_info(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/system/info request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut health_snapshot: system_health_snapshot_t = system_health_snapshot_t {
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
    memset(
        &raw mut health_snapshot as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_snapshot_t>(),
    );
    let mut health_snapshot_available: bool = system_health_snapshot_copy(&raw mut health_snapshot);
    let mut health_sequence: uint64_t = if health_snapshot_available as ::core::ffi::c_int != 0 {
        health_snapshot.sequence
    } else {
        0 as uint64_t
    };
    pthread_mutex_lock(&raw mut system_info_response_mutex);
    let mut response_now: time_t = time(::core::ptr::null_mut::<time_t>());
    if !system_info_response_json.is_null()
        && system_info_response_snapshot_sequence == health_sequence
        && response_now - system_info_response_refreshed_at
            < SYSTEM_INFO_RESPONSE_TTL_SECONDS as time_t
    {
        let mut cached_json: *mut ::core::ffi::c_char = strdup(system_info_response_json);
        pthread_mutex_unlock(&raw mut system_info_response_mutex);
        if !cached_json.is_null() {
            http_response_set_json(res, 200 as ::core::ffi::c_int, cached_json);
            free(cached_json as *mut ::core::ffi::c_void);
            return;
        }
    } else {
        pthread_mutex_unlock(&raw mut system_info_response_mutex);
    }
    let mut cached_recording_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cached_recording_bytes: int64_t = 0 as int64_t;
    get_cached_recording_aggregates(
        &raw mut cached_recording_count,
        &raw mut cached_recording_bytes,
    );
    let mut info: *mut cJSON = cJSON_CreateObject();
    if info.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create system info JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create system info JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddStringToObject(
        info,
        b"version\0".as_ptr() as *const ::core::ffi::c_char,
        LIGHTNVR_VERSION_STRING.as_ptr(),
    );
    if LIGHTNVR_GIT_COMMIT[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        cJSON_AddStringToObject(
            info,
            b"git_commit\0".as_ptr() as *const ::core::ffi::c_char,
            LIGHTNVR_GIT_COMMIT.as_ptr(),
        );
    }
    let mut system_info: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    if uname(&raw mut system_info) == 0 as ::core::ffi::c_int {
        let mut cpu: *mut cJSON = cJSON_CreateObject();
        if !cpu.is_null() {
            cJSON_AddStringToObject(
                cpu,
                b"model\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut system_info.machine as *mut ::core::ffi::c_char,
            );
            let mut cpu_quota: *const system_health_observation_t = find_health_observation(
                &raw mut health_snapshot,
                b"container.cpu.quota_cores\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            let mut quota_value: ::core::ffi::c_double = 0.0f64;
            let mut cores: ::core::ffi::c_int =
                sysconf(C2Rust_Unnamed_0::_SC_NPROCESSORS_ONLN.0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            if health_observation_value(cpu_quota, &raw mut quota_value) as ::core::ffi::c_int != 0
                && quota_value > 0.0f64
            {
                cores = quota_value as ::core::ffi::c_int;
                if (cores as ::core::ffi::c_double) < quota_value {
                    cores += 1;
                }
                if cores < 1 as ::core::ffi::c_int {
                    cores = 1 as ::core::ffi::c_int;
                }
            }
            cJSON_AddNumberToObject(
                cpu,
                b"cores\0".as_ptr() as *const ::core::ffi::c_char,
                cores as ::core::ffi::c_double,
            );
            let mut cpu_usage: *const system_health_observation_t =
                find_effective_health_observation(
                    &raw mut health_snapshot,
                    b"container.cpu.usage_ratio\0".as_ptr() as *const ::core::ffi::c_char,
                    b"host.cpu.busy_ratio\0".as_ptr() as *const ::core::ffi::c_char,
                );
            add_health_number(
                cpu,
                b"usage\0".as_ptr() as *const ::core::ffi::c_char,
                cpu_usage,
                100.0f64,
            );
            cJSON_AddStringToObject(
                cpu,
                b"usageCapability\0".as_ptr() as *const ::core::ffi::c_char,
                health_capability(cpu_usage),
            );
            cJSON_AddItemToObject(info, b"cpu\0".as_ptr() as *const ::core::ffi::c_char, cpu);
        }
    }
    let mut memory_total: *const system_health_observation_t = find_effective_health_observation(
        &raw mut health_snapshot,
        b"container.memory.limit_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"host.memory.total_bytes\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut memory_free: *const system_health_observation_t = find_effective_health_observation(
        &raw mut health_snapshot,
        b"container.memory.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"host.memory.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut memory_current: *const system_health_observation_t = find_health_observation(
        &raw mut health_snapshot,
        b"container.memory.current_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut system_total_value: ::core::ffi::c_double = 0.0f64;
    let mut system_free_value: ::core::ffi::c_double = 0.0f64;
    let mut system_used_value: ::core::ffi::c_double = 0.0f64;
    let mut system_total_valid: bool =
        health_observation_value(memory_total, &raw mut system_total_value);
    let mut system_free_valid: bool =
        health_observation_value(memory_free, &raw mut system_free_value);
    let mut system_used_valid: bool =
        health_observation_value(memory_current, &raw mut system_used_value);
    if !system_used_valid
        && system_total_valid as ::core::ffi::c_int != 0
        && system_free_valid as ::core::ffi::c_int != 0
    {
        system_used_value = if system_total_value > system_free_value {
            system_total_value - system_free_value
        } else {
            0.0f64
        };
        system_used_valid = r#true != 0;
    }
    let mut system_total: ::core::ffi::c_ulonglong =
        if system_total_valid as ::core::ffi::c_int != 0 {
            system_total_value as ::core::ffi::c_ulonglong
        } else {
            0 as ::core::ffi::c_ulonglong
        };
    let mut litert_used: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
    let mut process_rss: *const system_health_observation_t = find_health_observation(
        &raw mut health_snapshot,
        b"process.rss_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut process_thread_observation: *const system_health_observation_t =
        find_health_observation(
            &raw mut health_snapshot,
            b"process.threads\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    let mut process_rss_value: ::core::ffi::c_double = 0.0f64;
    let mut process_threads_value: ::core::ffi::c_double = 0.0f64;
    let mut process_rss_valid: bool =
        health_observation_value(process_rss, &raw mut process_rss_value);
    let mut process_threads_valid: bool =
        health_observation_value(process_thread_observation, &raw mut process_threads_value);
    let mut memory: *mut cJSON = cJSON_CreateObject();
    if !memory.is_null() {
        let mut used: ::core::ffi::c_double =
            if process_rss_value > litert_used as ::core::ffi::c_double {
                process_rss_value - litert_used as ::core::ffi::c_double
            } else {
                0.0f64
            };
        add_health_number(
            memory,
            b"total\0".as_ptr() as *const ::core::ffi::c_char,
            memory_total,
            1.0f64,
        );
        if process_rss_valid {
            cJSON_AddNumberToObject(
                memory,
                b"used\0".as_ptr() as *const ::core::ffi::c_char,
                used,
            );
        } else {
            cJSON_AddNullToObject(memory, b"used\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if system_total_valid as ::core::ffi::c_int != 0
            && process_rss_valid as ::core::ffi::c_int != 0
        {
            cJSON_AddNumberToObject(
                memory,
                b"free\0".as_ptr() as *const ::core::ffi::c_char,
                if system_total_value > used {
                    system_total_value - used
                } else {
                    0.0f64
                },
            );
        } else {
            cJSON_AddNullToObject(memory, b"free\0".as_ptr() as *const ::core::ffi::c_char);
        }
        cJSON_AddStringToObject(
            memory,
            b"capability\0".as_ptr() as *const ::core::ffi::c_char,
            health_capability(process_rss),
        );
        cJSON_AddItemToObject(
            info,
            b"memory\0".as_ptr() as *const ::core::ffi::c_char,
            memory,
        );
    }
    if process_threads_valid {
        cJSON_AddNumberToObject(
            info,
            b"threads\0".as_ptr() as *const ::core::ffi::c_char,
            process_threads_value,
        );
    } else {
        cJSON_AddNullToObject(info, b"threads\0".as_ptr() as *const ::core::ffi::c_char);
    }
    cJSON_AddNumberToObject(
        info,
        b"webThreadPoolSize\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.web_thread_pool_size as ::core::ffi::c_double,
    );
    let mut go2rtc_memory: *mut cJSON = cJSON_CreateObject();
    if !go2rtc_memory.is_null() {
        let mut go2rtc_used: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
        let mut go2rtc_memory_valid: bool = get_go2rtc_memory_usage(&raw mut go2rtc_used);
        if go2rtc_memory_valid {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"go2rtc memory usage: %llu bytes\0".as_ptr() as *const ::core::ffi::c_char,
                go2rtc_used,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to get go2rtc memory usage\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        add_health_number(
            go2rtc_memory,
            b"total\0".as_ptr() as *const ::core::ffi::c_char,
            memory_total,
            1.0f64,
        );
        if go2rtc_memory_valid {
            cJSON_AddNumberToObject(
                go2rtc_memory,
                b"used\0".as_ptr() as *const ::core::ffi::c_char,
                go2rtc_used as ::core::ffi::c_double,
            );
        } else {
            cJSON_AddNullToObject(
                go2rtc_memory,
                b"used\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if go2rtc_memory_valid as ::core::ffi::c_int != 0
            && system_total_valid as ::core::ffi::c_int != 0
        {
            cJSON_AddNumberToObject(
                go2rtc_memory,
                b"free\0".as_ptr() as *const ::core::ffi::c_char,
                if system_total > go2rtc_used {
                    system_total.wrapping_sub(go2rtc_used) as ::core::ffi::c_double
                } else {
                    0.0f64
                },
            );
        } else {
            cJSON_AddNullToObject(
                go2rtc_memory,
                b"free\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        cJSON_AddItemToObject(
            info,
            b"go2rtcMemory\0".as_ptr() as *const ::core::ffi::c_char,
            go2rtc_memory,
        );
    }
    let mut detector_memory: *mut cJSON = cJSON_CreateObject();
    if !detector_memory.is_null() {
        let mut detector_used: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
        let mut detector_memory_valid: bool = get_detector_memory_usage(&raw mut detector_used);
        if detector_memory_valid {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"light-object-detect memory usage: %llu bytes\0".as_ptr()
                    as *const ::core::ffi::c_char,
                detector_used,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"light-object-detect not running or memory unavailable\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if litert_used > 0 as ::core::ffi::c_ulonglong {
            detector_used = detector_used.wrapping_add(litert_used);
            detector_memory_valid = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"in-process LiteRT engine memory: %llu bytes\0".as_ptr()
                    as *const ::core::ffi::c_char,
                litert_used,
            );
        }
        add_health_number(
            detector_memory,
            b"total\0".as_ptr() as *const ::core::ffi::c_char,
            memory_total,
            1.0f64,
        );
        if detector_memory_valid {
            cJSON_AddNumberToObject(
                detector_memory,
                b"used\0".as_ptr() as *const ::core::ffi::c_char,
                detector_used as ::core::ffi::c_double,
            );
        } else {
            cJSON_AddNullToObject(
                detector_memory,
                b"used\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if detector_memory_valid as ::core::ffi::c_int != 0
            && system_total_valid as ::core::ffi::c_int != 0
        {
            cJSON_AddNumberToObject(
                detector_memory,
                b"free\0".as_ptr() as *const ::core::ffi::c_char,
                if system_total > detector_used {
                    system_total.wrapping_sub(detector_used) as ::core::ffi::c_double
                } else {
                    0.0f64
                },
            );
        } else {
            cJSON_AddNullToObject(
                detector_memory,
                b"free\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        cJSON_AddItemToObject(
            info,
            b"detectorMemory\0".as_ptr() as *const ::core::ffi::c_char,
            detector_memory,
        );
    }
    let mut system_memory: *mut cJSON = cJSON_CreateObject();
    if !system_memory.is_null() {
        add_health_number(
            system_memory,
            b"total\0".as_ptr() as *const ::core::ffi::c_char,
            memory_total,
            1.0f64,
        );
        if system_used_valid {
            cJSON_AddNumberToObject(
                system_memory,
                b"used\0".as_ptr() as *const ::core::ffi::c_char,
                system_used_value,
            );
        } else {
            cJSON_AddNullToObject(
                system_memory,
                b"used\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        add_health_number(
            system_memory,
            b"free\0".as_ptr() as *const ::core::ffi::c_char,
            memory_free,
            1.0f64,
        );
        cJSON_AddStringToObject(
            system_memory,
            b"capability\0".as_ptr() as *const ::core::ffi::c_char,
            if system_total_valid as ::core::ffi::c_int != 0
                && system_free_valid as ::core::ffi::c_int != 0
            {
                b"available\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                health_capability(if !system_total_valid {
                    memory_total
                } else {
                    memory_free
                })
            },
        );
        cJSON_AddItemToObject(
            info,
            b"systemMemory\0".as_ptr() as *const ::core::ffi::c_char,
            system_memory,
        );
    }
    cJSON_AddNumberToObject(
        info,
        b"uptime\0".as_ptr() as *const ::core::ffi::c_char,
        get_process_uptime_seconds(),
    );
    cJSON_AddStringToObject(
        info,
        b"uptimeCapability\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_capability_name(
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        ),
    );
    let mut recording_total: *const system_health_observation_t = find_health_observation(
        &raw mut health_snapshot,
        b"filesystem.capacity_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut recording_free: *const system_health_observation_t = find_health_observation(
        &raw mut health_snapshot,
        b"filesystem.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if recording_total.is_null() {
        recording_total = find_health_observation(
            &raw mut health_snapshot,
            b"filesystem.capacity_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            b"root\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if recording_free.is_null() {
        recording_free = find_health_observation(
            &raw mut health_snapshot,
            b"filesystem.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            b"root\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut disk: *mut cJSON = cJSON_CreateObject();
    if !disk.is_null() {
        add_health_number(
            disk,
            b"total\0".as_ptr() as *const ::core::ffi::c_char,
            recording_total,
            1.0f64,
        );
        cJSON_AddNumberToObject(
            disk,
            b"used\0".as_ptr() as *const ::core::ffi::c_char,
            cached_recording_bytes as ::core::ffi::c_double,
        );
        add_health_number(
            disk,
            b"free\0".as_ptr() as *const ::core::ffi::c_char,
            recording_free,
            1.0f64,
        );
        cJSON_AddStringToObject(
            disk,
            b"capability\0".as_ptr() as *const ::core::ffi::c_char,
            health_capability(if !recording_total.is_null() {
                recording_total
            } else {
                recording_free
            }),
        );
        cJSON_AddItemToObject(info, b"disk\0".as_ptr() as *const ::core::ffi::c_char, disk);
    }
    let mut root_total: *const system_health_observation_t = find_health_observation(
        &raw mut health_snapshot,
        b"filesystem.capacity_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"root\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut root_free: *const system_health_observation_t = find_health_observation(
        &raw mut health_snapshot,
        b"filesystem.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"root\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut root_total_value: ::core::ffi::c_double = 0.0f64;
    let mut root_free_value: ::core::ffi::c_double = 0.0f64;
    let mut root_total_valid: bool =
        health_observation_value(root_total, &raw mut root_total_value);
    let mut root_free_valid: bool = health_observation_value(root_free, &raw mut root_free_value);
    let mut system_disk: *mut cJSON = cJSON_CreateObject();
    if !system_disk.is_null() {
        add_health_number(
            system_disk,
            b"total\0".as_ptr() as *const ::core::ffi::c_char,
            root_total,
            1.0f64,
        );
        if root_total_valid as ::core::ffi::c_int != 0 && root_free_valid as ::core::ffi::c_int != 0
        {
            cJSON_AddNumberToObject(
                system_disk,
                b"used\0".as_ptr() as *const ::core::ffi::c_char,
                if root_total_value > root_free_value {
                    root_total_value - root_free_value
                } else {
                    0.0f64
                },
            );
        } else {
            cJSON_AddNullToObject(
                system_disk,
                b"used\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        add_health_number(
            system_disk,
            b"free\0".as_ptr() as *const ::core::ffi::c_char,
            root_free,
            1.0f64,
        );
        cJSON_AddStringToObject(
            system_disk,
            b"capability\0".as_ptr() as *const ::core::ffi::c_char,
            if root_total_valid as ::core::ffi::c_int != 0
                && root_free_valid as ::core::ffi::c_int != 0
            {
                b"available\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                health_capability(if !root_total_valid {
                    root_total
                } else {
                    root_free
                })
            },
        );
        cJSON_AddItemToObject(
            info,
            b"systemDisk\0".as_ptr() as *const ::core::ffi::c_char,
            system_disk,
        );
    }
    cJSON_AddNumberToObject(
        info,
        b"global_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.retention_days as ::core::ffi::c_double,
    );
    add_versions_to_json(info);
    let mut network: *mut cJSON = cJSON_CreateObject();
    let mut interfaces: *mut cJSON = cJSON_CreateArray();
    if !network.is_null() && !interfaces.is_null() {
        add_network_interfaces_json(interfaces);
        cJSON_AddItemToObject(
            network,
            b"interfaces\0".as_ptr() as *const ::core::ffi::c_char,
            interfaces,
        );
        cJSON_AddItemToObject(
            info,
            b"network\0".as_ptr() as *const ::core::ffi::c_char,
            network,
        );
    } else {
        cJSON_Delete(network);
        cJSON_Delete(interfaces);
    }
    let mut streams_obj: *mut cJSON = cJSON_CreateObject();
    if !streams_obj.is_null() {
        let mut enabled_streams: ::core::ffi::c_int = get_enabled_stream_count();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Enabled streams count from database: %d\0".as_ptr() as *const ::core::ffi::c_char,
            enabled_streams,
        );
        cJSON_AddNumberToObject(
            streams_obj,
            b"active\0".as_ptr() as *const ::core::ffi::c_char,
            enabled_streams as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            streams_obj,
            b"total\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.max_streams as ::core::ffi::c_double,
        );
        cJSON_AddItemToObject(
            info,
            b"streams\0".as_ptr() as *const ::core::ffi::c_char,
            streams_obj,
        );
    }
    let mut recordings: *mut cJSON = cJSON_CreateObject();
    if !recordings.is_null() {
        let mut recording_count: ::core::ffi::c_int = cached_recording_count;
        let mut recording_size_db: int64_t = cached_recording_bytes;
        let mut recording_size: ::core::ffi::c_ulonglong = if recording_size_db > 0 as int64_t {
            recording_size_db as ::core::ffi::c_ulonglong
        } else {
            0 as ::core::ffi::c_ulonglong
        };
        cJSON_AddNumberToObject(
            recordings,
            b"count\0".as_ptr() as *const ::core::ffi::c_char,
            recording_count as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            recordings,
            b"size\0".as_ptr() as *const ::core::ffi::c_char,
            recording_size as ::core::ffi::c_double,
        );
        cJSON_AddItemToObject(
            info,
            b"recordings\0".as_ptr() as *const ::core::ffi::c_char,
            recordings,
        );
    }
    add_cached_stream_storage_usage_to_json(info, 0 as ::core::ffi::c_int);
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(info);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert system info JSON to string\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        cJSON_Delete(info);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert system info JSON to string\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    pthread_mutex_lock(&raw mut system_info_response_mutex);
    let mut cached_copy: *mut ::core::ffi::c_char = strdup(json_str);
    if !cached_copy.is_null() {
        free(system_info_response_json as *mut ::core::ffi::c_void);
        system_info_response_json = cached_copy;
        system_info_response_refreshed_at = time(::core::ptr::null_mut::<time_t>());
        system_info_response_snapshot_sequence = health_sequence;
    }
    pthread_mutex_unlock(&raw mut system_info_response_mutex);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(info);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/system/info request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_system_restart(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/system/restart request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut audit_user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    let mut evaluation: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
        source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
        policy_version: 0,
        grant_uuid: [0; 37],
        token_uuid: [0; 37],
        role_uuid: [0; 37],
        role_name: [0; 128],
        explanation: [0; 256],
    };
    if httpd_authorize_action(
        req,
        res,
        authorization_action_t::AUTHZ_SYSTEM_ADMIN,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut audit_user,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    let mut success: *mut cJSON = cJSON_CreateObject();
    if success.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create success JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create success JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"system\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"system.restart\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    cJSON_AddBoolToObject(
        success,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        success,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"System is restarting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(success);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert success JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(success);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert success JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"system\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"system.restart\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(success);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"System restart requested via API\0".as_ptr() as *const ::core::ffi::c_char,
    );
    audit_log_operation(
        req,
        &raw mut audit_user,
        b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
        b"system\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"system.restart\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<cJSON>(),
    );
    request_restart();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled POST /api/system/restart request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_system_shutdown(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/system/shutdown request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut audit_user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    let mut evaluation: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
        source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
        policy_version: 0,
        grant_uuid: [0; 37],
        token_uuid: [0; 37],
        role_uuid: [0; 37],
        role_name: [0; 128],
        explanation: [0; 256],
    };
    if httpd_authorize_action(
        req,
        res,
        authorization_action_t::AUTHZ_SYSTEM_ADMIN,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut audit_user,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    let mut success: *mut cJSON = cJSON_CreateObject();
    if success.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create success JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create success JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"system\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"system.shutdown\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    cJSON_AddBoolToObject(
        success,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        success,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"System is shutting down\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(success);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert success JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(success);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert success JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"system\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"system.shutdown\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(success);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"System shutdown requested via API\0".as_ptr() as *const ::core::ffi::c_char,
    );
    audit_log_operation(
        req,
        &raw mut audit_user,
        b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
        b"system\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"system.shutdown\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<cJSON>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Initiating shutdown through coordinator\0".as_ptr() as *const ::core::ffi::c_char,
    );
    initiate_shutdown();
    unsafe extern "C" {
        static mut running: bool;
    }
    running = r#false != 0;
    request_background_abort();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled POST /api/system/shutdown request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_system_backup(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/system/backup request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut audit_user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    let mut evaluation: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
        source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
        policy_version: 0,
        grant_uuid: [0; 37],
        token_uuid: [0; 37],
        role_uuid: [0; 37],
        role_name: [0; 128],
        explanation: [0; 256],
    };
    if httpd_authorize_action(
        req,
        res,
        authorization_action_t::AUTHZ_SYSTEM_ADMIN,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut audit_user,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
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
    let mut tm_info: *const tm = localtime_r(&raw mut now, &raw mut tm_buf);
    let mut timestamp: [::core::ffi::c_char; 20] = [0; 20];
    strftime(
        &raw mut timestamp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 20]>(),
        b"%Y%m%d_%H%M%S\0".as_ptr() as *const ::core::ffi::c_char,
        tm_info,
    );
    let mut backup_filename: [::core::ffi::c_char; 256] = [0; 256];
    snprintf(
        &raw mut backup_filename as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"lightnvr_backup_%s.json\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut timestamp as *mut ::core::ffi::c_char,
    );
    let mut backup_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut backup_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/backups\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.web_root as *mut ::core::ffi::c_char,
    );
    if ensure_dir(&raw mut backup_path as *mut ::core::ffi::c_char) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create backup directory %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut backup_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create backup directory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"backup\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"backup.create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    snprintf(
        &raw mut backup_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/backups/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.web_root as *mut ::core::ffi::c_char,
        &raw mut backup_filename as *mut ::core::ffi::c_char,
    );
    let mut backup_fd: ::core::ffi::c_int = open(
        &raw mut backup_path as *mut ::core::ffi::c_char,
        O_WRONLY | O_CREAT | O_TRUNC,
        0o600 as ::core::ffi::c_int,
    );
    if backup_fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create backup file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"backup\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"backup.create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        let mut error: *mut cJSON = cJSON_CreateObject();
        if error.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to create error JSON object\0".as_ptr() as *const ::core::ffi::c_char,
            );
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create error JSON\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddBoolToObject(
            error,
            b"success\0".as_ptr() as *const ::core::ffi::c_char,
            r#false,
        );
        let mut error_msg: [::core::ffi::c_char; 256] = [0; 256];
        snprintf(
            &raw mut error_msg as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"Failed to create backup: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        cJSON_AddStringToObject(
            error,
            b"message\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut error_msg as *mut ::core::ffi::c_char,
        );
        let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(error);
        if json_str.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to convert error JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_Delete(error);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to convert error JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        http_response_set_json_error(res, 500 as ::core::ffi::c_int, json_str);
        free(json_str as *mut ::core::ffi::c_void);
        cJSON_Delete(error);
        return;
    }
    let mut backup_file: *mut FILE =
        fdopen(backup_fd, b"w\0".as_ptr() as *const ::core::ffi::c_char);
    if backup_file.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to open backup file stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(backup_fd);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to open backup file stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"backup\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"backup.create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    let mut backup: *mut cJSON = cJSON_CreateObject();
    if backup.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create backup JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fclose(backup_file);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create backup JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"backup\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"backup.create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    cJSON_AddStringToObject(
        backup,
        b"version\0".as_ptr() as *const ::core::ffi::c_char,
        LIGHTNVR_VERSION_STRING.as_ptr(),
    );
    cJSON_AddStringToObject(
        backup,
        b"timestamp\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut timestamp as *mut ::core::ffi::c_char,
    );
    let mut config: *mut cJSON = cJSON_CreateObject();
    if config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create config JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(backup);
        fclose(backup_file);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create config JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"backup\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"backup.create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    cJSON_AddNumberToObject(
        config,
        b"web_port\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.web_port as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        config,
        b"web_bind_ip\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.web_bind_ip as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        config,
        b"web_root\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.web_root as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        config,
        b"log_file\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.log_file as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        config,
        b"pid_file\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.pid_file as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        config,
        b"db_path\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.db_path as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        config,
        b"storage_path\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        config,
        b"max_storage_size\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.max_storage_size as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        config,
        b"max_streams\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.max_streams as ::core::ffi::c_double,
    );
    let mut streams: *mut cJSON = cJSON_CreateArray();
    if streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create streams JSON array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(config);
        cJSON_Delete(backup);
        fclose(backup_file);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create streams JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"backup\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"backup.create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if (*g_config.streams.offset(i as isize)).name[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
        {
            let mut stream: *mut cJSON = cJSON_CreateObject();
            if stream.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to create stream JSON object\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                cJSON_AddStringToObject(
                    stream,
                    b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*g_config.streams.offset(i as isize)).camera_uuid
                        as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    stream,
                    b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*g_config.streams.offset(i as isize)).location_uuid
                        as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    stream,
                    b"name\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*g_config.streams.offset(i as isize)).name
                        as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    stream,
                    b"url\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*g_config.streams.offset(i as isize)).url as *mut ::core::ffi::c_char,
                );
                cJSON_AddBoolToObject(
                    stream,
                    b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
                    (*g_config.streams.offset(i as isize)).enabled as cJSON_bool,
                );
                cJSON_AddNumberToObject(
                    stream,
                    b"width\0".as_ptr() as *const ::core::ffi::c_char,
                    (*g_config.streams.offset(i as isize)).width as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    stream,
                    b"height\0".as_ptr() as *const ::core::ffi::c_char,
                    (*g_config.streams.offset(i as isize)).height as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    stream,
                    b"fps\0".as_ptr() as *const ::core::ffi::c_char,
                    (*g_config.streams.offset(i as isize)).fps as ::core::ffi::c_double,
                );
                cJSON_AddStringToObject(
                    stream,
                    b"codec\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*g_config.streams.offset(i as isize)).codec
                        as *mut ::core::ffi::c_char,
                );
                cJSON_AddBoolToObject(
                    stream,
                    b"record\0".as_ptr() as *const ::core::ffi::c_char,
                    (*g_config.streams.offset(i as isize)).record as cJSON_bool,
                );
                cJSON_AddNumberToObject(
                    stream,
                    b"priority\0".as_ptr() as *const ::core::ffi::c_char,
                    (*g_config.streams.offset(i as isize)).priority as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    stream,
                    b"segment_duration\0".as_ptr() as *const ::core::ffi::c_char,
                    (*g_config.streams.offset(i as isize)).segment_duration
                        as ::core::ffi::c_double,
                );
                cJSON_AddBoolToObject(
                    stream,
                    b"is_onvif\0".as_ptr() as *const ::core::ffi::c_char,
                    (*g_config.streams.offset(i as isize)).is_onvif as cJSON_bool,
                );
                cJSON_AddStringToObject(
                    stream,
                    b"onvif_username\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*g_config.streams.offset(i as isize)).onvif_username
                        as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    stream,
                    b"onvif_password\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*g_config.streams.offset(i as isize)).onvif_password
                        as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    stream,
                    b"onvif_profile\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*g_config.streams.offset(i as isize)).onvif_profile
                        as *mut ::core::ffi::c_char,
                );
                cJSON_AddNumberToObject(
                    stream,
                    b"onvif_port\0".as_ptr() as *const ::core::ffi::c_char,
                    (*g_config.streams.offset(i as isize)).onvif_port as ::core::ffi::c_double,
                );
                cJSON_AddItemToArray(streams, stream);
            }
        }
        i += 1;
    }
    cJSON_AddItemToObject(
        config,
        b"streams\0".as_ptr() as *const ::core::ffi::c_char,
        streams,
    );
    cJSON_AddItemToObject(
        backup,
        b"config\0".as_ptr() as *const ::core::ffi::c_char,
        config,
    );
    let mut json_str_0: *mut ::core::ffi::c_char = cJSON_Print(backup);
    if json_str_0.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert backup JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(backup);
        fclose(backup_file);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert backup JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"backup\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"backup.create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    let mut write_result: ::core::ffi::c_int = fputs(json_str_0, backup_file);
    free(json_str_0 as *mut ::core::ffi::c_void);
    cJSON_Delete(backup);
    let mut close_result: ::core::ffi::c_int = fclose(backup_file);
    if write_result == EOF || close_result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to finish backup file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        audit_log_operation(
            req,
            &raw mut audit_user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"backup\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"backup.create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to write backup file\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_log_operation(
        req,
        &raw mut audit_user,
        b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
        b"backup\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"backup.create\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<cJSON>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Configuration backup created: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut backup_path as *mut ::core::ffi::c_char,
    );
    let mut success: *mut cJSON = cJSON_CreateObject();
    if success.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create success JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create success JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        success,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        success,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"Backup created successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut backup_url: [::core::ffi::c_char; 256] = [0; 256];
    snprintf(
        &raw mut backup_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"/backups/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut backup_filename as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        success,
        b"backupUrl\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut backup_url as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        success,
        b"filename\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut backup_filename as *mut ::core::ffi::c_char,
    );
    json_str_0 = cJSON_PrintUnformatted(success);
    if json_str_0.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert success JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(success);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert success JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str_0);
    free(json_str_0 as *mut ::core::ffi::c_void);
    cJSON_Delete(success);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled POST /api/system/backup request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_system_status(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/system/status request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut status: *mut cJSON = cJSON_CreateObject();
    if status.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create status JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create status JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddStringToObject(
        status,
        b"status\0".as_ptr() as *const ::core::ffi::c_char,
        b"ok\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        status,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"System running normally\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(status);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert status JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(status);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert status JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(status);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/system/status request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[inline]
unsafe extern "C" fn ltrim_pos(
    mut input: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if input.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut start: *mut ::core::ffi::c_uchar = input as *mut ::core::ffi::c_uchar;
    while *start as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*start as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISgraph.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        start = start.offset(1);
    }
    return start as *const ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn rtrim_pos(
    mut input: *const ::core::ffi::c_char,
    mut input_size: size_t,
) -> *const ::core::ffi::c_char {
    if input.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if input_size > 0 as size_t {
        end = input
            .offset(strnlen(input, input_size) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
    } else {
        end = input
            .offset(strlen(input) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
    }
    while end > input
        && *(*__ctype_b_loc()).offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISgraph.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        end = end.offset(-1);
    }
    return end.offset(1 as ::core::ffi::c_int as isize);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_VERSION_STRING: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"0.47.2\0") };
pub const LIGHTNVR_BUILD_DATE: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*crate::build_info::LIGHTNVR_BUILD_DATE)
};
pub const LIGHTNVR_GIT_COMMIT: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*crate::build_info::LIGHTNVR_GIT_COMMIT)
};
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
}
