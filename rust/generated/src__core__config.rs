use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    unsafe fn renameat(
        __oldfd: ::core::ffi::c_int,
        __old: *const ::core::ffi::c_char,
        __newfd: ::core::ffi::c_int,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn getcwd(__buf: *mut ::core::ffi::c_char, __size: size_t) -> *mut ::core::ffi::c_char;
    unsafe fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    unsafe fn getpid() -> __pid_t;
    unsafe fn unlinkat(
        __fd: ::core::ffi::c_int,
        __name: *const ::core::ffi::c_char,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn ftruncate(__fd: ::core::ffi::c_int, __length: __off_t) -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn openat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fstatat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __buf: *mut stat,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fchmod(__fd: ::core::ffi::c_int, __mode: __mode_t) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn ini_parse(
        filename: *const ::core::ffi::c_char,
        handler: ini_handler,
        user: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_policy_settings_defaults(
        settings: *mut system_health_policy_settings_t,
    );
    unsafe fn system_health_policy_validate_settings(
        settings: *const system_health_policy_settings_t,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn mkdir_recursive(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn ensure_path(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn chmod_parent(path: *const ::core::ffi::c_char, mode: mode_t) -> ::core::ffi::c_int;
    unsafe fn begin_transaction() -> ::core::ffi::c_int;
    unsafe fn commit_transaction() -> ::core::ffi::c_int;
    unsafe fn rollback_transaction() -> ::core::ffi::c_int;
    unsafe fn add_stream_config(stream: *const stream_config_t) -> uint64_t;
    unsafe fn delete_stream_config(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_all_stream_configs(
        streams: *mut stream_config_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn count_stream_configs() -> ::core::ffi::c_int;
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
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
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn realpath(
        __name: *const ::core::ffi::c_char,
        __resolved: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type atomic_uint = ::core::ffi::c_uint;
pub type ini_handler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;
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
pub struct api_detection_format_t(pub ::core::ffi::c_uint);
impl api_detection_format_t {
    pub const API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT: Self = Self(0);
    pub const API_DETECTION_FORMAT_DOODS2: Self = Self(1);
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env_config_mapping_t {
    pub env_name: *const ::core::ffi::c_char,
    pub r#type: config_field_type_t,
    pub offset: size_t,
    pub size: size_t,
    pub default_str_value: *const ::core::ffi::c_char,
    pub default_int_value: ::core::ffi::c_int,
    pub default_bool_value: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct config_field_type_t(pub ::core::ffi::c_uint);
impl config_field_type_t {
    pub const CONFIG_TYPE_BOOL: Self = Self(0);
    pub const CONFIG_TYPE_INT: Self = Self(1);
    pub const CONFIG_TYPE_STRING: Self = Self(2);
}
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const LOG_USER: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_DAEMON: ::core::ffi::c_int = (3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL0: ::core::ffi::c_int = (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL1: ::core::ffi::c_int = (17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL2: ::core::ffi::c_int = (18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL3: ::core::ffi::c_int = (19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL4: ::core::ffi::c_int = (20 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL5: ::core::ffi::c_int = (21 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL6: ::core::ffi::c_int = (22 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL7: ::core::ffi::c_int = (23 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const API_DETECTION_FORMAT_MAX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const API_DETECTION_FORMAT_NAME_LIGHT_OBJECT_DETECT: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"light-object-detect\0")
};
pub const API_DETECTION_FORMAT_NAME_DOODS2: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"doods2\0") };
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MAX_URL_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const DEFAULT_MAX_STREAMS: ::core::ffi::c_int = if MAX_STREAMS < 32 as ::core::ffi::c_int {
    MAX_STREAMS
} else {
    32 as ::core::ffi::c_int
};
pub const WEB_TRUSTED_PROXY_CIDRS_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const DB_STARTUP_CHECK_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DB_STARTUP_CHECK_QUICK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DB_STARTUP_CHECK_FULL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DB_BACKUP_VERIFY_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DB_BACKUP_VERIFY_QUICK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DB_BACKUP_VERIFY_FULL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OBSERVATION_DEFAULT_MAX_AGE_DAYS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const OBSERVATION_DEFAULT_MAX_BYTES: ::core::ffi::c_longlong =
    2147483648 as ::core::ffi::c_longlong;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut g_config: config_t = config_t {
    pid_file: [0; 512],
    log_file: [0; 512],
    log_level: 0,
    syslog_enabled: false,
    syslog_ident: [0; 64],
    syslog_facility: 0,
    storage_path: [0; 512],
    storage_path_hls: [0; 512],
    max_storage_size: 0,
    retention_days: 0,
    auto_delete_oldest: false,
    mp4_faststart: false,
    storage_min_free_pct: 0,
    storage_pressure_warning_pct: 0.,
    storage_pressure_critical_pct: 0.,
    storage_pressure_emergency_pct: 0.,
    generate_thumbnails: false,
    thumbnails_per_recording: 0,
    record_mp4_directly: false,
    mp4_storage_path: [0; 512],
    mp4_directory_format: [0; 32],
    mp4_segment_duration: 0,
    mp4_retention_days: 0,
    models_path: [0; 512],
    api_detection_url: [0; 512],
    api_detection_backend: [0; 32],
    api_detection_format: [0; 32],
    api_detection_detector_name: [0; 64],
    default_detection_threshold: 0,
    default_pre_detection_buffer: 0,
    default_post_detection_buffer: 0,
    default_playback_transport: [0; 24],
    detection_grace_period: 0,
    db_path: [0; 512],
    db_backup_interval_minutes: 0,
    db_backup_retention_count: 0,
    db_post_backup_script: [0; 512],
    db_startup_check: 0,
    db_backup_verify: 0,
    web_thread_pool_size: 0,
    web_port: 0,
    web_bind_ip: [0; 32],
    web_root: [0; 512],
    web_ssl_enabled: false,
    web_ssl_cert_path: [0; 512],
    web_ssl_key_path: [0; 512],
    web_auth_enabled: false,
    web_username: [0; 32],
    web_password: [0; 32],
    audio_disabled: false,
    auto_disabled: false,
    webrtc_disabled: false,
    hls_disabled: false,
    mse_disabled: false,
    auth_timeout_hours: 0,
    auth_absolute_timeout_hours: 0,
    trusted_device_days: 0,
    trusted_proxy_cidrs: [0; 1024],
    demo_mode: false,
    force_mfa_on_login: false,
    login_rate_limit_enabled: false,
    login_rate_limit_max_attempts: 0,
    login_rate_limit_window_seconds: 0,
    web_compression_enabled: false,
    web_use_minified_assets: false,
    web_cache_max_age_html: 0,
    web_cache_max_age_css: 0,
    web_cache_max_age_js: 0,
    web_cache_max_age_images: 0,
    web_cache_max_age_fonts: 0,
    web_cache_max_age_default: 0,
    onvif_discovery_enabled: false,
    onvif_discovery_interval: 0,
    onvif_discovery_network: [0; 64],
    max_streams: 0,
    streams: ::core::ptr::null_mut::<stream_config_t>(),
    buffer_size: 0,
    use_swap: false,
    swap_file: [0; 512],
    swap_size: 0,
    memory_constrained: false,
    hw_accel_enabled: false,
    hw_accel_device: [0; 32],
    go2rtc_enabled: false,
    go2rtc_binary_path: [0; 512],
    go2rtc_config_dir: [0; 512],
    go2rtc_api_port: 0,
    go2rtc_rtsp_port: 0,
    go2rtc_force_native_hls: false,
    go2rtc_proxy_max_inflight: 0,
    go2rtc_webrtc_enabled: false,
    go2rtc_webrtc_listen_port: 0,
    go2rtc_stun_enabled: false,
    go2rtc_stun_server: [0; 256],
    go2rtc_external_ip: [0; 256],
    go2rtc_ice_servers: [0; 512],
    webrtc_connection_timeout_ms: 0,
    webrtc_ice_recovery_timeout_ms: 0,
    turn_enabled: false,
    turn_server_url: [0; 256],
    turn_username: [0; 64],
    turn_password: [0; 64],
    mqtt_enabled: false,
    mqtt_broker_host: [0; 256],
    mqtt_broker_port: 0,
    mqtt_username: [0; 64],
    mqtt_password: [0; 128],
    mqtt_client_id: [0; 64],
    mqtt_topic_prefix: [0; 128],
    mqtt_tls_enabled: false,
    mqtt_keepalive: 0,
    mqtt_qos: 0,
    mqtt_retain: false,
    mqtt_ha_discovery: false,
    mqtt_ha_discovery_prefix: [0; 128],
    mqtt_ha_snapshot_interval: 0,
    health: system_health_policy_settings_t {
        enabled: false,
        profile: [0; 16],
        fast_interval_seconds: 0,
        normal_interval_seconds: 0,
        slow_interval_seconds: 0,
        device_interval_seconds: 0,
        write_probe_enabled: false,
        hardware_provider: [0; 16],
        presence_interval_seconds: 0,
        incident_retention_days: 0,
    },
    detection_engine: detection_engine_config_t {
        enabled: false,
        num_threads: 0,
        delegate: [0; 16],
    },
};
static mut mp4_faststart_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_1::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn config_get_mp4_faststart(mut config: *const config_t) -> bool {
    pthread_mutex_lock(&raw mut mp4_faststart_mutex);
    let mut enabled: bool = (*config).mp4_faststart;
    pthread_mutex_unlock(&raw mut mp4_faststart_mutex);
    return enabled;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn config_set_mp4_faststart(mut config: *mut config_t, mut enabled: bool) {
    pthread_mutex_lock(&raw mut mp4_faststart_mutex);
    (*config).mp4_faststart = enabled;
    pthread_mutex_unlock(&raw mut mp4_faststart_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn config_copy(mut destination: *mut config_t, mut source: *const config_t) {
    pthread_mutex_lock(&raw mut mp4_faststart_mutex);
    if destination != source as *mut config_t {
        memcpy(
            destination as *mut ::core::ffi::c_void,
            source as *const ::core::ffi::c_void,
            ::core::mem::size_of::<config_t>(),
        );
    }
    pthread_mutex_unlock(&raw mut mp4_faststart_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn playback_transport_is_valid(
    mut value: *const ::core::ffi::c_char,
) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    static mut valid_values: [*const ::core::ffi::c_char; 6] = [
        b"auto\0".as_ptr() as *const ::core::ffi::c_char,
        b"webrtc_only\0".as_ptr() as *const ::core::ffi::c_char,
        b"mse_only\0".as_ptr() as *const ::core::ffi::c_char,
        b"hls_only\0".as_ptr() as *const ::core::ffi::c_char,
        b"webrtc_then_mse\0".as_ptr() as *const ::core::ffi::c_char,
        b"mse_then_hls\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if strcmp(value, valid_values[i]) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn safe_atoi(
    mut str: *const ::core::ffi::c_char,
    mut fallback: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if str.is_null() || *str == 0 {
        return fallback;
    }
    let mut parsed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut val: ::core::ffi::c_long = strtol(str, &raw mut end, 10 as ::core::ffi::c_int);
    if end == str as *mut ::core::ffi::c_char {
        return fallback;
    }
    while *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && *(*__ctype_b_loc()).offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        end = end.offset(1);
    }
    if *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || *__errno_location() == ERANGE
        || val < INT_MIN as ::core::ffi::c_long
        || val > INT_MAX as ::core::ffi::c_long
    {
        return fallback;
    }
    parsed = val as ::core::ffi::c_int;
    return parsed;
}
unsafe extern "C" fn parse_bool_strict(
    mut value: *const ::core::ffi::c_char,
    mut result: *mut bool,
) -> bool {
    if value.is_null() || result.is_null() {
        return r#false != 0;
    }
    if strcasecmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strcasecmp(value, b"yes\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(value, b"on\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        *result = r#true != 0;
        return r#true != 0;
    }
    if strcasecmp(value, b"false\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcmp(value, b"0\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strcasecmp(value, b"no\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(value, b"off\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        *result = r#false != 0;
        return r#true != 0;
    }
    return r#false != 0;
}
static mut env_config_mappings: [env_config_mapping_t; 33] = [
    env_config_mapping_t {
        env_name: b"TURN_ENABLED\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_BOOL,
        offset: 10432 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"TURN_SERVER_URL\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 10433 as size_t,
        size: 256 as size_t,
        default_str_value: b"\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"TURN_USERNAME\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 10689 as size_t,
        size: 64 as size_t,
        default_str_value: b"\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"TURN_PASSWORD\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 10753 as size_t,
        size: 64 as size_t,
        default_str_value: b"\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"GO2RTC_ENABLED\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_BOOL,
        offset: 8346 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#true != 0,
    },
    env_config_mapping_t {
        env_name: b"GO2RTC_WEBRTC_ENABLED\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_BOOL,
        offset: 9388 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#true != 0,
    },
    env_config_mapping_t {
        env_name: b"GO2RTC_STUN_ENABLED\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_BOOL,
        offset: 9396 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#true != 0,
    },
    env_config_mapping_t {
        env_name: b"GO2RTC_STUN_SERVER\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 9397 as size_t,
        size: 256 as size_t,
        default_str_value: b"stun.l.google.com:19302\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"GO2RTC_EXTERNAL_IP\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 9653 as size_t,
        size: 256 as size_t,
        default_str_value: b"\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"GO2RTC_ICE_SERVERS\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 9909 as size_t,
        size: 512 as size_t,
        default_str_value: b"\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"GO2RTC_WEBRTC_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_INT,
        offset: 9392 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 8555 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"WEBRTC_CONNECTION_TIMEOUT_MS\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_INT,
        offset: 10424 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 30000 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"WEBRTC_ICE_RECOVERY_TIMEOUT_MS\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_INT,
        offset: 10428 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 5000 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"WEB_PORT\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_INT,
        offset: 4976 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 8080 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"WEB_BIND_IP\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 4980 as size_t,
        size: 32 as size_t,
        default_str_value: b"0.0.0.0\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"WEB_SSL_ENABLED\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_BOOL,
        offset: 5524 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"WEB_SSL_CERT_PATH\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 5525 as size_t,
        size: MAX_PATH_LENGTH as size_t,
        default_str_value: b"\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"WEB_SSL_KEY_PATH\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 6037 as size_t,
        size: MAX_PATH_LENGTH as size_t,
        default_str_value: b"\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"WEB_AUTH_ENABLED\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_BOOL,
        offset: 6549 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#true != 0,
    },
    env_config_mapping_t {
        env_name: b"WEB_USERNAME\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 6550 as size_t,
        size: 32 as size_t,
        default_str_value: b"admin\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"WEB_TRUSTED_PROXY_CIDRS\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 6632 as size_t,
        size: WEB_TRUSTED_PROXY_CIDRS_MAX as size_t,
        default_str_value: b"\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"DEMO_MODE\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_BOOL,
        offset: 7656 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"AUDIO_DISABLED\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_BOOL,
        offset: 6614 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"LOG_LEVEL\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_INT,
        offset: 1024 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 2 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"STORAGE_PATH\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 1100 as size_t,
        size: MAX_PATH_LENGTH as size_t,
        default_str_value: b"/var/lib/lightnvr/recordings\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"RETENTION_DAYS\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_INT,
        offset: 2136 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 30 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"DB_PATH\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 3932 as size_t,
        size: MAX_PATH_LENGTH as size_t,
        default_str_value: b"/var/lib/lightnvr/lightnvr.db\0".as_ptr()
            as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"DB_BACKUP_INTERVAL_MINUTES\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_INT,
        offset: 4444 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"DB_BACKUP_RETENTION_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_INT,
        offset: 4448 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 6 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"DB_POST_BACKUP_SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_STRING,
        offset: 4452 as size_t,
        size: MAX_PATH_LENGTH as size_t,
        default_str_value: b"\0".as_ptr() as *const ::core::ffi::c_char,
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"DB_STARTUP_CHECK\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_INT,
        offset: 4964 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: DB_STARTUP_CHECK_OFF,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: b"DB_BACKUP_VERIFY\0".as_ptr() as *const ::core::ffi::c_char,
        r#type: config_field_type_t::CONFIG_TYPE_INT,
        offset: 4968 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: DB_BACKUP_VERIFY_FULL,
        default_bool_value: r#false != 0,
    },
    env_config_mapping_t {
        env_name: ::core::ptr::null::<::core::ffi::c_char>(),
        r#type: config_field_type_t::CONFIG_TYPE_BOOL,
        offset: 0 as size_t,
        size: 0 as size_t,
        default_str_value: ::core::ptr::null::<::core::ffi::c_char>(),
        default_int_value: 0 as ::core::ffi::c_int,
        default_bool_value: r#false != 0,
    },
];
unsafe extern "C" fn parse_env_bool(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    return strcasecmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcasecmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(value, b"yes\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(value, b"on\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_string_default(
    mut current: *const ::core::ffi::c_char,
    mut default_val: *const ::core::ffi::c_char,
) -> bool {
    if default_val.is_null()
        || *default_val.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return current.is_null()
            || *current.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int;
    }
    return !current.is_null() && strcmp(current, default_val) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn apply_env_overrides(mut config: *mut config_t) {
    if config.is_null() {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Checking for LIGHTNVR_ environment variable overrides\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !env_config_mappings[i as usize].env_name.is_null() {
        let mut mapping: *const env_config_mapping_t =
            (&raw const env_config_mappings as *const env_config_mapping_t).offset(i as isize);
        let mut env_name: [::core::ffi::c_char; 128] = [0; 128];
        snprintf(
            &raw mut env_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            b"LIGHTNVR_%s\0".as_ptr() as *const ::core::ffi::c_char,
            (*mapping).env_name,
        );
        let mut env_value: *const ::core::ffi::c_char =
            getenv(&raw mut env_name as *mut ::core::ffi::c_char);
        if !env_value.is_null() {
            let mut field_ptr: *mut ::core::ffi::c_void = (config as *mut ::core::ffi::c_char)
                .offset((*mapping).offset as isize)
                as *mut ::core::ffi::c_void;
            let mut is_default: bool = r#false != 0;
            match (*mapping).r#type {
                config_field_type_t::CONFIG_TYPE_BOOL => {
                    let mut bool_ptr: *const bool = field_ptr as *const bool;
                    is_default = *bool_ptr as ::core::ffi::c_int
                        == (*mapping).default_bool_value as ::core::ffi::c_int;
                }
                config_field_type_t::CONFIG_TYPE_INT => {
                    let mut int_ptr: *const ::core::ffi::c_int =
                        field_ptr as *const ::core::ffi::c_int;
                    is_default = *int_ptr == (*mapping).default_int_value;
                }
                config_field_type_t::CONFIG_TYPE_STRING => {
                    let mut str_ptr: *const ::core::ffi::c_char =
                        field_ptr as *const ::core::ffi::c_char;
                    is_default = is_string_default(str_ptr, (*mapping).default_str_value);
                }
                _ => {}
            }
            if !is_default {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Config field for %s already set, skipping env override\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut env_name as *mut ::core::ffi::c_char,
                );
            } else {
                match (*mapping).r#type {
                    config_field_type_t::CONFIG_TYPE_BOOL => {
                        let mut bool_ptr_0: *mut bool = field_ptr as *mut bool;
                        let mut new_value: bool = parse_env_bool(env_value);
                        *bool_ptr_0 = new_value;
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Applied env override: %s=%s (bool: %s)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut env_name as *mut ::core::ffi::c_char,
                            env_value,
                            if new_value as ::core::ffi::c_int != 0 {
                                b"true\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"false\0".as_ptr() as *const ::core::ffi::c_char
                            },
                        );
                    }
                    config_field_type_t::CONFIG_TYPE_INT => {
                        let mut int_ptr_0: *mut ::core::ffi::c_int =
                            field_ptr as *mut ::core::ffi::c_int;
                        let mut endptr: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut new_value_0: ::core::ffi::c_int = 0;
                        *__errno_location() = 0 as ::core::ffi::c_int;
                        let mut parsed: ::core::ffi::c_long =
                            strtol(env_value, &raw mut endptr, 10 as ::core::ffi::c_int);
                        while *endptr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                            && *(*__ctype_b_loc()).offset(
                                *endptr as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                            ) as ::core::ffi::c_int
                                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int
                                    as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int
                                != 0
                        {
                            endptr = endptr.offset(1);
                        }
                        if endptr == env_value as *mut ::core::ffi::c_char
                            || *endptr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                            || *__errno_location() == ERANGE
                            || parsed < INT_MIN as ::core::ffi::c_long
                            || parsed > INT_MAX as ::core::ffi::c_long
                        {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Invalid integer for %s: '%s'; keeping existing value %d\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut env_name as *mut ::core::ffi::c_char,
                                env_value,
                                *int_ptr_0,
                            );
                        } else {
                            new_value_0 = parsed as ::core::ffi::c_int;
                            *int_ptr_0 = new_value_0;
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Applied env override: %s=%s (int: %d)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut env_name as *mut ::core::ffi::c_char,
                                env_value,
                                new_value_0,
                            );
                        }
                    }
                    config_field_type_t::CONFIG_TYPE_STRING => {
                        let mut str_ptr_0: *mut ::core::ffi::c_char =
                            field_ptr as *mut ::core::ffi::c_char;
                        safe_strcpy(str_ptr_0, env_value, (*mapping).size, 0 as size_t);
                        if !strstr(
                            (*mapping).env_name,
                            b"PASSWORD\0".as_ptr() as *const ::core::ffi::c_char,
                        )
                        .is_null()
                            || !strstr(
                                (*mapping).env_name,
                                b"SECRET\0".as_ptr() as *const ::core::ffi::c_char,
                            )
                            .is_null()
                        {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Applied env override: %s=*** (string, masked)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut env_name as *mut ::core::ffi::c_char,
                            );
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Applied env override: %s=%s (string)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut env_name as *mut ::core::ffi::c_char,
                                env_value,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_default_config(mut config: *mut config_t) {
    if config.is_null() {
        return;
    }
    if !(*config).streams.is_null() {
        free((*config).streams as *mut ::core::ffi::c_void);
        (*config).streams = ::core::ptr::null_mut::<stream_config_t>();
    }
    memset(
        config as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<config_t>(),
    );
    (*config).max_streams = DEFAULT_MAX_STREAMS;
    (*config).streams = calloc(
        (*config).max_streams as size_t,
        ::core::mem::size_of::<stream_config_t>(),
    ) as *mut stream_config_t;
    if (*config).streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"load_default_config: failed to allocate streams array\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut sc_cores: ::core::ffi::c_long =
        sysconf(C2Rust_Unnamed::_SC_NPROCESSORS_ONLN.0 as ::core::ffi::c_int);
    let mut num_cores: ::core::ffi::c_int = 0;
    if sc_cores == -1 as ::core::ffi::c_long {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"load_default_config: sysconf(_SC_NPROCESSORS_ONLN) failed, using default web thread pool size\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        num_cores = 0 as ::core::ffi::c_int;
    } else if sc_cores > INT_MAX as ::core::ffi::c_long {
        num_cores = INT_MAX;
    } else {
        num_cores = sc_cores as ::core::ffi::c_int;
    }
    (*config).web_thread_pool_size = if num_cores > 0 as ::core::ffi::c_int {
        num_cores * 2 as ::core::ffi::c_int
    } else {
        8 as ::core::ffi::c_int
    };
    if (*config).web_thread_pool_size < 2 as ::core::ffi::c_int {
        (*config).web_thread_pool_size = 2 as ::core::ffi::c_int;
    }
    if (*config).web_thread_pool_size > 128 as ::core::ffi::c_int {
        (*config).web_thread_pool_size = 128 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut (*config).pid_file as *mut ::core::ffi::c_char,
        b"/var/run/lightnvr.pid\0".as_ptr() as *const ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*config).log_file as *mut ::core::ffi::c_char,
        b"/var/log/lightnvr.log\0".as_ptr() as *const ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    (*config).log_level = log_level_t::LOG_LEVEL_INFO.0 as ::core::ffi::c_int;
    (*config).syslog_enabled = r#false != 0;
    safe_strcpy(
        &raw mut (*config).syslog_ident as *mut ::core::ffi::c_char,
        b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    (*config).syslog_facility = LOG_USER;
    safe_strcpy(
        &raw mut (*config).storage_path as *mut ::core::ffi::c_char,
        b"/var/lib/lightnvr/recordings\0".as_ptr() as *const ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    (*config).storage_path_hls[0usize] = '\0' as ::core::ffi::c_char;
    (*config).max_storage_size = 0 as uint64_t;
    (*config).retention_days = 30 as ::core::ffi::c_int;
    (*config).auto_delete_oldest = r#true != 0;
    config_set_mp4_faststart(config, r#true != 0);
    (*config).storage_min_free_pct = 10 as ::core::ffi::c_int;
    (*config).storage_pressure_warning_pct = 20.0f64;
    (*config).storage_pressure_critical_pct = 10.0f64;
    (*config).storage_pressure_emergency_pct = 5.0f64;
    (*config).generate_thumbnails = r#true != 0;
    (*config).thumbnails_per_recording = 3 as ::core::ffi::c_int;
    (*config).record_mp4_directly = r#false != 0;
    safe_strcpy(
        &raw mut (*config).mp4_storage_path as *mut ::core::ffi::c_char,
        b"/var/lib/lightnvr/recordings/mp4\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*config).mp4_directory_format as *mut ::core::ffi::c_char,
        b"year_month_day\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        0 as size_t,
    );
    (*config).mp4_segment_duration = 900 as ::core::ffi::c_int;
    (*config).mp4_retention_days = 30 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut (*config).models_path as *mut ::core::ffi::c_char,
        b"/var/lib/lightnvr/models\0".as_ptr() as *const ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*config).api_detection_url as *mut ::core::ffi::c_char,
        b"http://localhost:8000/api/v1/detect\0".as_ptr() as *const ::core::ffi::c_char,
        MAX_URL_LENGTH as size_t,
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*config).api_detection_backend as *mut ::core::ffi::c_char,
        b"onnx\0".as_ptr() as *const ::core::ffi::c_char,
        32 as size_t,
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*config).api_detection_format as *mut ::core::ffi::c_char,
        API_DETECTION_FORMAT_NAME_LIGHT_OBJECT_DETECT.as_ptr(),
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*config).api_detection_detector_name as *mut ::core::ffi::c_char,
        b"default\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    (*config).default_detection_threshold = 50 as ::core::ffi::c_int;
    (*config).default_pre_detection_buffer = 5 as ::core::ffi::c_int;
    (*config).default_post_detection_buffer = 10 as ::core::ffi::c_int;
    (*config).detection_grace_period = 2 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut (*config).default_playback_transport as *mut ::core::ffi::c_char,
        b"auto\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    (*config).detection_engine.enabled = r#false != 0;
    (*config).detection_engine.num_threads = 1 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut (*config).detection_engine.delegate as *mut ::core::ffi::c_char,
        b"xnnpack\0".as_ptr() as *const ::core::ffi::c_char,
        16 as size_t,
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*config).db_path as *mut ::core::ffi::c_char,
        b"/var/lib/lightnvr/lightnvr.db\0".as_ptr() as *const ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    (*config).db_backup_interval_minutes = 0 as ::core::ffi::c_int;
    (*config).db_backup_retention_count = 6 as ::core::ffi::c_int;
    (*config).db_post_backup_script[0usize] = '\0' as ::core::ffi::c_char;
    (*config).db_startup_check = DB_STARTUP_CHECK_OFF;
    (*config).db_backup_verify = DB_BACKUP_VERIFY_FULL;
    (*config).web_port = 8080 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut (*config).web_bind_ip as *mut ::core::ffi::c_char,
        b"0.0.0.0\0".as_ptr() as *const ::core::ffi::c_char,
        32 as size_t,
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*config).web_root as *mut ::core::ffi::c_char,
        b"/var/lib/lightnvr/www\0".as_ptr() as *const ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    (*config).web_ssl_enabled = r#false != 0;
    (*config).web_ssl_cert_path[0usize] = '\0' as ::core::ffi::c_char;
    (*config).web_ssl_key_path[0usize] = '\0' as ::core::ffi::c_char;
    (*config).web_auth_enabled = r#true != 0;
    safe_strcpy(
        &raw mut (*config).web_username as *mut ::core::ffi::c_char,
        b"admin\0".as_ptr() as *const ::core::ffi::c_char,
        32 as size_t,
        0 as size_t,
    );
    (*config).web_password[0usize] = '\0' as ::core::ffi::c_char;
    (*config).audio_disabled = r#false != 0;
    (*config).auto_disabled = r#false != 0;
    (*config).webrtc_disabled = r#false != 0;
    (*config).hls_disabled = r#false != 0;
    (*config).mse_disabled = r#false != 0;
    (*config).auth_timeout_hours = 24 as ::core::ffi::c_int;
    (*config).auth_absolute_timeout_hours = 168 as ::core::ffi::c_int;
    (*config).trusted_device_days = 30 as ::core::ffi::c_int;
    (*config).trusted_proxy_cidrs[0usize] = '\0' as ::core::ffi::c_char;
    (*config).demo_mode = r#false != 0;
    (*config).force_mfa_on_login = r#false != 0;
    (*config).login_rate_limit_enabled = r#true != 0;
    (*config).login_rate_limit_max_attempts = 5 as ::core::ffi::c_int;
    (*config).login_rate_limit_window_seconds = 300 as ::core::ffi::c_int;
    (*config).web_compression_enabled = r#true != 0;
    (*config).web_use_minified_assets = r#true != 0;
    (*config).web_cache_max_age_html = 3600 as ::core::ffi::c_int;
    (*config).web_cache_max_age_css = 604800 as ::core::ffi::c_int;
    (*config).web_cache_max_age_js = 604800 as ::core::ffi::c_int;
    (*config).web_cache_max_age_images = 2592000 as ::core::ffi::c_int;
    (*config).web_cache_max_age_fonts = 2592000 as ::core::ffi::c_int;
    (*config).web_cache_max_age_default = 86400 as ::core::ffi::c_int;
    (*config).buffer_size = 1024 as ::core::ffi::c_int;
    (*config).use_swap = r#true != 0;
    safe_strcpy(
        &raw mut (*config).swap_file as *mut ::core::ffi::c_char,
        b"/var/lib/lightnvr/swap\0".as_ptr() as *const ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    (*config).swap_size = (128 as ::core::ffi::c_int as uint64_t)
        .wrapping_mul(1024 as uint64_t)
        .wrapping_mul(1024 as uint64_t);
    (*config).hw_accel_enabled = r#false != 0;
    memset(
        &raw mut (*config).hw_accel_device as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        32 as size_t,
    );
    (*config).go2rtc_enabled = r#true != 0;
    safe_strcpy(
        &raw mut (*config).go2rtc_binary_path as *mut ::core::ffi::c_char,
        GO2RTC_BINARY_PATH_RAW.as_ptr(),
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*config).go2rtc_config_dir as *mut ::core::ffi::c_char,
        GO2RTC_CONFIG_DIR_RAW.as_ptr(),
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    (*config).go2rtc_api_port = 1984 as ::core::ffi::c_int;
    (*config).go2rtc_rtsp_port = 8554 as ::core::ffi::c_int;
    (*config).go2rtc_force_native_hls = r#false != 0;
    (*config).go2rtc_proxy_max_inflight = 16 as ::core::ffi::c_int;
    (*config).go2rtc_webrtc_enabled = r#true != 0;
    (*config).go2rtc_webrtc_listen_port = 8555 as ::core::ffi::c_int;
    (*config).go2rtc_stun_enabled = r#true != 0;
    safe_strcpy(
        &raw mut (*config).go2rtc_stun_server as *mut ::core::ffi::c_char,
        b"stun.l.google.com:19302\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    (*config).go2rtc_external_ip[0usize] = '\0' as ::core::ffi::c_char;
    (*config).go2rtc_ice_servers[0usize] = '\0' as ::core::ffi::c_char;
    (*config).webrtc_connection_timeout_ms = 30000 as ::core::ffi::c_int;
    (*config).webrtc_ice_recovery_timeout_ms = 5000 as ::core::ffi::c_int;
    (*config).turn_enabled = r#false != 0;
    (*config).turn_server_url[0usize] = '\0' as ::core::ffi::c_char;
    (*config).turn_username[0usize] = '\0' as ::core::ffi::c_char;
    (*config).turn_password[0usize] = '\0' as ::core::ffi::c_char;
    (*config).onvif_discovery_enabled = r#false != 0;
    (*config).onvif_discovery_interval = 300 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut (*config).onvif_discovery_network as *mut ::core::ffi::c_char,
        b"auto\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*config).max_streams {
        (*(*config).streams.offset(i as isize)).detection_based_recording = r#false != 0;
        (*(*config).streams.offset(i as isize)).detection_model[0usize] =
            '\0' as ::core::ffi::c_char;
        (*(*config).streams.offset(i as isize)).detection_interval = 10 as ::core::ffi::c_int;
        (*(*config).streams.offset(i as isize)).detection_threshold = 0.5f32;
        (*(*config).streams.offset(i as isize)).pre_detection_buffer = 5 as ::core::ffi::c_int;
        (*(*config).streams.offset(i as isize)).post_detection_buffer = 10 as ::core::ffi::c_int;
        (*(*config).streams.offset(i as isize)).detection_api_url[0usize] =
            '\0' as ::core::ffi::c_char;
        safe_strcpy(
            &raw mut (*(*config).streams.offset(i as isize)).detection_object_filter
                as *mut ::core::ffi::c_char,
            b"none\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        (*(*config).streams.offset(i as isize)).streaming_enabled = r#true != 0;
        (*(*config).streams.offset(i as isize)).record_audio = r#false != 0;
        (*(*config).streams.offset(i as isize)).tier_critical_multiplier = 3.0f64;
        (*(*config).streams.offset(i as isize)).tier_important_multiplier = 2.0f64;
        (*(*config).streams.offset(i as isize)).tier_ephemeral_multiplier = 0.25f64;
        (*(*config).streams.offset(i as isize)).storage_priority = 5 as ::core::ffi::c_int;
        (*(*config).streams.offset(i as isize)).observation_evidence_enabled = r#false != 0;
        (*(*config).streams.offset(i as isize)).observation_labels[0usize] =
            '\0' as ::core::ffi::c_char;
        (*(*config).streams.offset(i as isize)).observation_max_age_days =
            OBSERVATION_DEFAULT_MAX_AGE_DAYS;
        (*(*config).streams.offset(i as isize)).observation_max_bytes =
            OBSERVATION_DEFAULT_MAX_BYTES as int64_t;
        i += 1;
    }
    (*config).mqtt_enabled = r#false != 0;
    (*config).mqtt_broker_host[0usize] = '\0' as ::core::ffi::c_char;
    (*config).mqtt_broker_port = 1883 as ::core::ffi::c_int;
    (*config).mqtt_username[0usize] = '\0' as ::core::ffi::c_char;
    (*config).mqtt_password[0usize] = '\0' as ::core::ffi::c_char;
    safe_strcpy(
        &raw mut (*config).mqtt_client_id as *mut ::core::ffi::c_char,
        b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*config).mqtt_topic_prefix as *mut ::core::ffi::c_char,
        b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    (*config).mqtt_tls_enabled = r#false != 0;
    (*config).mqtt_keepalive = 60 as ::core::ffi::c_int;
    (*config).mqtt_qos = 1 as ::core::ffi::c_int;
    (*config).mqtt_retain = r#false != 0;
    (*config).mqtt_ha_discovery = r#false != 0;
    safe_strcpy(
        &raw mut (*config).mqtt_ha_discovery_prefix as *mut ::core::ffi::c_char,
        b"homeassistant\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    (*config).mqtt_ha_snapshot_interval = 30 as ::core::ffi::c_int;
    system_health_policy_settings_defaults(&raw mut (*config).health);
}
unsafe extern "C" fn ensure_directories(mut config: *const config_t) -> ::core::ffi::c_int {
    if mkdir_recursive(&raw const (*config).storage_path as *const ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create storage directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).storage_path as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*config).storage_path_hls[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if mkdir_recursive(&raw const (*config).storage_path_hls as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to create HLS storage directory: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw const (*config).storage_path_hls as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Created HLS storage directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).storage_path_hls as *const ::core::ffi::c_char,
        );
    }
    if mkdir_recursive(&raw const (*config).models_path as *const ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create models directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).models_path as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if ensure_path(&raw const (*config).db_path as *const ::core::ffi::c_char) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create database directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).db_path as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if mkdir_recursive(&raw const (*config).web_root as *const ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create web root directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).web_root as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if ensure_path(&raw const (*config).log_file as *const ::core::ffi::c_char) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create log directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).log_file as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut cfg_log_fd: ::core::ffi::c_int = open(
        &raw const (*config).log_file as *const ::core::ffi::c_char,
        O_WRONLY | O_CREAT | O_APPEND,
        0o640 as ::core::ffi::c_int,
    );
    if cfg_log_fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Log file %s is not writable: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).log_file as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        if chmod_parent(
            &raw const (*config).log_file as *const ::core::ffi::c_char,
            0o755 as mode_t,
        ) != 0
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to change log directory permissions: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
    } else {
        close(cfg_log_fd);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn validate_config(mut config: *mut config_t) -> ::core::ffi::c_int {
    if config.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut health_error: [::core::ffi::c_char; 256] = [0; 256];
    if system_health_policy_validate_settings(
        &raw mut (*config).health,
        &raw mut health_error as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid health configuration: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut health_error as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*config).auth_absolute_timeout_hours < (*config).auth_timeout_hours {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"auth_absolute_timeout_hours (%d) is less than auth_timeout_hours (%d); clamping to the idle timeout\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*config).auth_absolute_timeout_hours,
            (*config).auth_timeout_hours,
        );
        (*config).auth_absolute_timeout_hours = (*config).auth_timeout_hours;
    }
    if strlen(&raw mut (*config).storage_path as *mut ::core::ffi::c_char) == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage path is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if strlen(&raw mut (*config).models_path as *mut ::core::ffi::c_char) == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Models path is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if strlen(&raw mut (*config).db_path as *mut ::core::ffi::c_char) == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database path is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*config).db_backup_interval_minutes < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"db_backup_interval_minutes (%d) is negative; clamping to 0\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*config).db_backup_interval_minutes,
        );
        (*config).db_backup_interval_minutes = 0 as ::core::ffi::c_int;
    }
    if (*config).db_backup_retention_count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"db_backup_retention_count (%d) is negative; clamping to 0\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*config).db_backup_retention_count,
        );
        (*config).db_backup_retention_count = 0 as ::core::ffi::c_int;
    }
    if (*config).db_startup_check < DB_STARTUP_CHECK_OFF
        || (*config).db_startup_check > DB_STARTUP_CHECK_FULL
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"db startup_check (%d) out of range [%d,%d]; using off\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*config).db_startup_check,
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
        (*config).db_startup_check = DB_STARTUP_CHECK_OFF;
    }
    if (*config).db_backup_verify < DB_BACKUP_VERIFY_OFF
        || (*config).db_backup_verify > DB_BACKUP_VERIFY_FULL
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"db backup_verify (%d) out of range [%d,%d]; using full\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*config).db_backup_verify,
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
        (*config).db_backup_verify = DB_BACKUP_VERIFY_FULL;
    }
    if (*config).storage_min_free_pct < 0 as ::core::ffi::c_int
        || (*config).storage_min_free_pct > 90 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"storage min_free_pct (%d) out of range [0,90]; clamping\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*config).storage_min_free_pct,
        );
        (*config).storage_min_free_pct = if (*config).storage_min_free_pct < 0 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            90 as ::core::ffi::c_int
        };
    }
    if (*config).storage_pressure_emergency_pct < 0.0f64
        || (*config).storage_pressure_emergency_pct >= (*config).storage_pressure_critical_pct
        || (*config).storage_pressure_critical_pct >= (*config).storage_pressure_warning_pct
        || (*config).storage_pressure_warning_pct > 100.0f64
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"storage pressure thresholds invalid (emergency=%.1f critical=%.1f warning=%.1f); resetting to defaults 5/10/20\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*config).storage_pressure_emergency_pct,
            (*config).storage_pressure_critical_pct,
            (*config).storage_pressure_warning_pct,
        );
        (*config).storage_pressure_emergency_pct = 5.0f64;
        (*config).storage_pressure_critical_pct = 10.0f64;
        (*config).storage_pressure_warning_pct = 20.0f64;
    }
    if strlen(&raw mut (*config).web_root as *mut ::core::ffi::c_char) == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Web root path is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*config).web_port <= 0 as ::core::ffi::c_int
        || (*config).web_port > 65535 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid web port: %d\0".as_ptr() as *const ::core::ffi::c_char,
            (*config).web_port,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*config).buffer_size <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid buffer size: %d\0".as_ptr() as *const ::core::ffi::c_char,
            (*config).buffer_size,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*config).use_swap as ::core::ffi::c_int != 0 && (*config).swap_size == 0 as uint64_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid swap size: %llu\0".as_ptr() as *const ::core::ffi::c_char,
            (*config).swap_size as ::core::ffi::c_ulonglong,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn config_set_detection_grace_period(
    mut config: *mut config_t,
    mut seconds: ::core::ffi::c_int,
) {
    if seconds < 0 as ::core::ffi::c_int {
        seconds = 0 as ::core::ffi::c_int;
    }
    if seconds > 60 as ::core::ffi::c_int {
        seconds = 60 as ::core::ffi::c_int;
    }
    (*config).detection_grace_period = seconds;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn config_set_detection_engine_threads(
    mut config: *mut config_t,
    mut threads: ::core::ffi::c_int,
) {
    if threads < 1 as ::core::ffi::c_int {
        threads = 1 as ::core::ffi::c_int;
    }
    if threads > 16 as ::core::ffi::c_int {
        threads = 16 as ::core::ffi::c_int;
    }
    (*config).detection_engine.num_threads = threads;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn config_set_detection_engine_delegate(
    mut config: *mut config_t,
    mut delegate: *const ::core::ffi::c_char,
) -> bool {
    if delegate.is_null()
        || strcmp(
            delegate,
            b"xnnpack\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            && strcmp(delegate, b"gpu\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
            && strcmp(delegate, b"none\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    safe_strcpy(
        &raw mut (*config).detection_engine.delegate as *mut ::core::ffi::c_char,
        delegate,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn config_ini_handler(
    mut user: *mut ::core::ffi::c_void,
    mut section: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut config: *mut config_t = user as *mut config_t;
    if strcmp(section, b"general\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(name, b"pid_file\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).pid_file as *mut ::core::ffi::c_char,
                value,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        } else if strcmp(name, b"log_file\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).log_file as *mut ::core::ffi::c_char,
                value,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        } else if strcmp(name, b"log_level\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).log_level = safe_atoi(value, 0 as ::core::ffi::c_int);
        } else if strcmp(
            name,
            b"syslog_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).syslog_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"syslog_ident\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).syslog_ident as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"syslog_facility\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if *(*__ctype_b_loc()).offset(*value.offset(0isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            {
                (*config).syslog_facility = safe_atoi(value, 0 as ::core::ffi::c_int);
            } else if strcmp(
                value,
                b"LOG_DAEMON\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*config).syslog_facility = LOG_DAEMON;
            } else if strcmp(
                value,
                b"LOG_LOCAL0\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*config).syslog_facility = LOG_LOCAL0;
            } else if strcmp(
                value,
                b"LOG_LOCAL1\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*config).syslog_facility = LOG_LOCAL1;
            } else if strcmp(
                value,
                b"LOG_LOCAL2\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*config).syslog_facility = LOG_LOCAL2;
            } else if strcmp(
                value,
                b"LOG_LOCAL3\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*config).syslog_facility = LOG_LOCAL3;
            } else if strcmp(
                value,
                b"LOG_LOCAL4\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*config).syslog_facility = LOG_LOCAL4;
            } else if strcmp(
                value,
                b"LOG_LOCAL5\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*config).syslog_facility = LOG_LOCAL5;
            } else if strcmp(
                value,
                b"LOG_LOCAL6\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*config).syslog_facility = LOG_LOCAL6;
            } else if strcmp(
                value,
                b"LOG_LOCAL7\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*config).syslog_facility = LOG_LOCAL7;
            } else {
                (*config).syslog_facility = LOG_USER;
            }
        }
    } else if strcmp(section, b"health\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        let mut parsed_bool: bool = false;
        if strcmp(name, b"enabled\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            if !parse_bool_strict(value, &raw mut parsed_bool) {
                return 0 as ::core::ffi::c_int;
            }
            (*config).health.enabled = parsed_bool;
        } else if strcmp(name, b"profile\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).health.profile as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"fast_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).health.fast_interval_seconds =
                safe_atoi(value, -1 as ::core::ffi::c_int) as uint32_t;
        } else if strcmp(
            name,
            b"normal_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).health.normal_interval_seconds =
                safe_atoi(value, -1 as ::core::ffi::c_int) as uint32_t;
        } else if strcmp(
            name,
            b"slow_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).health.slow_interval_seconds =
                safe_atoi(value, -1 as ::core::ffi::c_int) as uint32_t;
        } else if strcmp(
            name,
            b"device_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).health.device_interval_seconds =
                safe_atoi(value, -1 as ::core::ffi::c_int) as uint32_t;
        } else if strcmp(
            name,
            b"write_probe_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if !parse_bool_strict(value, &raw mut parsed_bool) {
                return 0 as ::core::ffi::c_int;
            }
            (*config).health.write_probe_enabled = parsed_bool;
        } else if strcmp(
            name,
            b"hardware_provider\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).health.hardware_provider as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"presence_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).health.presence_interval_seconds =
                safe_atoi(value, -1 as ::core::ffi::c_int) as uint32_t;
        } else if strcmp(
            name,
            b"incident_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).health.incident_retention_days =
                safe_atoi(value, -1 as ::core::ffi::c_int) as uint32_t;
        } else {
            return 0 as ::core::ffi::c_int;
        }
    } else if strcmp(section, b"storage\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(name, b"path\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).storage_path as *mut ::core::ffi::c_char,
                value,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        } else if strcmp(name, b"path_hls\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).storage_path_hls as *mut ::core::ffi::c_char,
                value,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        } else if strcmp(name, b"max_size\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).max_storage_size = strtoull(
                value,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                10 as ::core::ffi::c_int,
            ) as uint64_t;
        } else if strcmp(
            name,
            b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).retention_days = safe_atoi(value, 0 as ::core::ffi::c_int);
        } else if strcmp(
            name,
            b"auto_delete_oldest\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).auto_delete_oldest =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"mp4_faststart\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            config_set_mp4_faststart(
                config,
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int,
            );
        } else if strcmp(
            name,
            b"min_free_pct\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).storage_min_free_pct = safe_atoi(value, 10 as ::core::ffi::c_int);
        } else if strcmp(
            name,
            b"pressure_warning_pct\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).storage_pressure_warning_pct =
                strtod(value, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>());
        } else if strcmp(
            name,
            b"pressure_critical_pct\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).storage_pressure_critical_pct =
                strtod(value, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>());
        } else if strcmp(
            name,
            b"pressure_emergency_pct\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).storage_pressure_emergency_pct =
                strtod(value, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>());
        } else if strcmp(
            name,
            b"record_mp4_directly\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).record_mp4_directly =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(name, b"mp4_path\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).mp4_storage_path as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"mp4_directory_format\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if strcmp(value, b"flat\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcmp(
                    value,
                    b"year_month\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    value,
                    b"year_month_day\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                safe_strcpy(
                    &raw mut (*config).mp4_directory_format as *mut ::core::ffi::c_char,
                    value,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                    0 as size_t,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Ignoring invalid mp4_directory_format '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    value,
                );
            }
        } else if strcmp(
            name,
            b"mp4_segment_duration\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).mp4_segment_duration = safe_atoi(value, 0 as ::core::ffi::c_int);
        } else if strcmp(
            name,
            b"mp4_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).mp4_retention_days = safe_atoi(value, 0 as ::core::ffi::c_int);
        } else if strcmp(
            name,
            b"generate_thumbnails\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).generate_thumbnails =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"thumbnails_per_recording\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut n: ::core::ffi::c_int = safe_atoi(value, 3 as ::core::ffi::c_int);
            (*config).thumbnails_per_recording = if n <= 1 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            };
        }
    } else if strcmp(section, b"models\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(name, b"path\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).models_path as *mut ::core::ffi::c_char,
                value,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        }
    } else if strcmp(
        section,
        b"api_detection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(name, b"url\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).api_detection_url as *mut ::core::ffi::c_char,
                value,
                MAX_URL_LENGTH as size_t,
                0 as size_t,
            );
        } else if strcmp(name, b"backend\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).api_detection_backend as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                0 as size_t,
            );
        } else if strcmp(name, b"format\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            if !config_set_api_detection_format(config, value) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Unknown [api_detection] format '%s'; expected %s or %s, keeping %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    value,
                    b"light-object-detect\0".as_ptr() as *const ::core::ffi::c_char,
                    b"doods2\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*config).api_detection_format as *mut ::core::ffi::c_char,
                );
            }
        } else if strcmp(
            name,
            b"detector_name\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).api_detection_detector_name as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"detection_threshold\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).default_detection_threshold = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).default_detection_threshold < 0 as ::core::ffi::c_int {
                (*config).default_detection_threshold = 0 as ::core::ffi::c_int;
            }
            if (*config).default_detection_threshold > 100 as ::core::ffi::c_int {
                (*config).default_detection_threshold = 100 as ::core::ffi::c_int;
            }
        } else if strcmp(
            name,
            b"pre_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).default_pre_detection_buffer = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).default_pre_detection_buffer < 0 as ::core::ffi::c_int {
                (*config).default_pre_detection_buffer = 0 as ::core::ffi::c_int;
            }
            if (*config).default_pre_detection_buffer > 60 as ::core::ffi::c_int {
                (*config).default_pre_detection_buffer = 60 as ::core::ffi::c_int;
            }
        } else if strcmp(
            name,
            b"post_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).default_post_detection_buffer = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).default_post_detection_buffer < 0 as ::core::ffi::c_int {
                (*config).default_post_detection_buffer = 0 as ::core::ffi::c_int;
            }
            if (*config).default_post_detection_buffer > 300 as ::core::ffi::c_int {
                (*config).default_post_detection_buffer = 300 as ::core::ffi::c_int;
            }
        }
    } else if strcmp(
        section,
        b"detection_engine\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(name, b"enabled\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).detection_engine.enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    || strcmp(value, b"yes\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    || strcmp(value, b"on\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(name, b"threads\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            config_set_detection_engine_threads(config, safe_atoi(value, 1 as ::core::ffi::c_int));
        } else if strcmp(name, b"delegate\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            if !config_set_detection_engine_delegate(config, value) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Unknown detection_engine.delegate '%s'; using xnnpack\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    value,
                );
                config_set_detection_engine_delegate(
                    config,
                    b"xnnpack\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
    } else if strcmp(
        section,
        b"detection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(
            name,
            b"grace_period\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            config_set_detection_grace_period(config, safe_atoi(value, 0 as ::core::ffi::c_int));
        }
    } else if strcmp(
        section,
        b"database\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(name, b"path\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).db_path as *mut ::core::ffi::c_char,
                value,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"backup_interval_minutes\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).db_backup_interval_minutes = safe_atoi(value, 0 as ::core::ffi::c_int);
        } else if strcmp(
            name,
            b"backup_retention_count\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).db_backup_retention_count = safe_atoi(value, 0 as ::core::ffi::c_int);
        } else if strcmp(
            name,
            b"post_backup_script\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).db_post_backup_script as *mut ::core::ffi::c_char,
                value,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"startup_check\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if strcasecmp(value, b"off\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcasecmp(value, b"none\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                (*config).db_startup_check = DB_STARTUP_CHECK_OFF;
            } else if strcasecmp(value, b"full\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                (*config).db_startup_check = DB_STARTUP_CHECK_FULL;
            } else if strcasecmp(value, b"quick\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                (*config).db_startup_check = DB_STARTUP_CHECK_QUICK;
            } else {
                (*config).db_startup_check = safe_atoi(value, DB_STARTUP_CHECK_OFF);
            }
        } else if strcmp(
            name,
            b"backup_verify\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if strcasecmp(value, b"off\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcasecmp(value, b"none\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                (*config).db_backup_verify = DB_BACKUP_VERIFY_OFF;
            } else if strcasecmp(value, b"full\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                (*config).db_backup_verify = DB_BACKUP_VERIFY_FULL;
            } else if strcasecmp(value, b"quick\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                (*config).db_backup_verify = DB_BACKUP_VERIFY_QUICK;
            } else {
                (*config).db_backup_verify = safe_atoi(value, DB_BACKUP_VERIFY_FULL);
            }
        }
    } else if strcmp(section, b"web\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(name, b"port\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        {
            (*config).web_port = safe_atoi(value, 0 as ::core::ffi::c_int);
        } else if strcmp(name, b"bind_ip\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).web_bind_ip as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                0 as size_t,
            );
        } else if strcmp(name, b"root\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).web_root as *mut ::core::ffi::c_char,
                value,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"ssl_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).web_ssl_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"ssl_cert_path\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).web_ssl_cert_path as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"ssl_key_path\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).web_ssl_key_path as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"auth_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).web_auth_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(name, b"username\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).web_username as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                0 as size_t,
            );
        } else if strcmp(name, b"password\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).web_password as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"audio_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).audio_disabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"auto_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).auto_disabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"webrtc_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).webrtc_disabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"hls_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).hls_disabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"mse_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).mse_disabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"default_playback_transport\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if playback_transport_is_valid(value) {
                safe_strcpy(
                    &raw mut (*config).default_playback_transport as *mut ::core::ffi::c_char,
                    value,
                    ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
                    0 as size_t,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Unknown default playback transport '%s'; using auto\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    value,
                );
                safe_strcpy(
                    &raw mut (*config).default_playback_transport as *mut ::core::ffi::c_char,
                    b"auto\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
                    0 as size_t,
                );
            }
        } else if strcmp(
            name,
            b"auth_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).auth_timeout_hours = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).auth_timeout_hours < 1 as ::core::ffi::c_int {
                (*config).auth_timeout_hours = 1 as ::core::ffi::c_int;
            } else if (*config).auth_timeout_hours > INT_MAX / 3600 as ::core::ffi::c_int {
                (*config).auth_timeout_hours = INT_MAX / 3600 as ::core::ffi::c_int;
            }
        } else if strcmp(
            name,
            b"auth_absolute_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).auth_absolute_timeout_hours = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).auth_absolute_timeout_hours < 1 as ::core::ffi::c_int {
                (*config).auth_absolute_timeout_hours = 1 as ::core::ffi::c_int;
            } else if (*config).auth_absolute_timeout_hours > INT_MAX / 3600 as ::core::ffi::c_int {
                (*config).auth_absolute_timeout_hours = INT_MAX / 3600 as ::core::ffi::c_int;
            }
        } else if strcmp(
            name,
            b"trusted_device_days\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).trusted_device_days = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).trusted_device_days < 0 as ::core::ffi::c_int {
                (*config).trusted_device_days = 0 as ::core::ffi::c_int;
            } else if (*config).trusted_device_days > INT_MAX / 86400 as ::core::ffi::c_int {
                (*config).trusted_device_days = INT_MAX / 86400 as ::core::ffi::c_int;
            }
        } else if strcmp(
            name,
            b"trusted_proxy_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).trusted_proxy_cidrs as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                0 as size_t,
            );
        } else if strcmp(name, b"demo_mode\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).demo_mode = strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"force_mfa_on_login\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).force_mfa_on_login =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"login_rate_limit_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).login_rate_limit_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"login_rate_limit_max_attempts\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).login_rate_limit_max_attempts = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).login_rate_limit_max_attempts < 1 as ::core::ffi::c_int {
                (*config).login_rate_limit_max_attempts = 1 as ::core::ffi::c_int;
            }
        } else if strcmp(
            name,
            b"login_rate_limit_window_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).login_rate_limit_window_seconds = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).login_rate_limit_window_seconds < 10 as ::core::ffi::c_int {
                (*config).login_rate_limit_window_seconds = 10 as ::core::ffi::c_int;
            }
        } else if strcmp(
            name,
            b"web_thread_pool_size\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut v: ::core::ffi::c_int = safe_atoi(value, 0 as ::core::ffi::c_int);
            if v < 2 as ::core::ffi::c_int {
                v = 2 as ::core::ffi::c_int;
            }
            if v > 128 as ::core::ffi::c_int {
                v = 128 as ::core::ffi::c_int;
            }
            (*config).web_thread_pool_size = v;
        }
    } else if strcmp(section, b"streams\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(
            name,
            b"max_streams\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut new_max: ::core::ffi::c_int = safe_atoi(value, 0 as ::core::ffi::c_int);
            if new_max < 1 as ::core::ffi::c_int {
                new_max = 1 as ::core::ffi::c_int;
            }
            if new_max > MAX_STREAMS {
                new_max = MAX_STREAMS;
            }
            if new_max != (*config).max_streams {
                let mut p: *mut stream_config_t = realloc(
                    (*config).streams as *mut ::core::ffi::c_void,
                    (new_max as size_t).wrapping_mul(::core::mem::size_of::<stream_config_t>()),
                ) as *mut stream_config_t;
                if !p.is_null() {
                    if new_max > (*config).max_streams {
                        memset(
                            p.offset((*config).max_streams as isize) as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ((new_max - (*config).max_streams) as size_t)
                                .wrapping_mul(::core::mem::size_of::<stream_config_t>()),
                        );
                    }
                    (*config).streams = p;
                    (*config).max_streams = new_max;
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"max_streams set to %d\0".as_ptr() as *const ::core::ffi::c_char,
                        new_max,
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to realloc streams array for max_streams=%d, keeping %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        new_max,
                        (*config).max_streams,
                    );
                }
            }
        }
    } else if strstr(section, b"stream.\0".as_ptr() as *const ::core::ffi::c_char)
        == section as *mut ::core::ffi::c_char
    {
        static mut last_warned_section: [::core::ffi::c_char; 256] = [
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
        if strcmp(
            &raw mut last_warned_section as *mut ::core::ffi::c_char,
            section,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Ignoring stale INI section [%s]: stream config lives in the database. Remove this section from the config file.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                section,
            );
            safe_strcpy(
                &raw mut last_warned_section as *mut ::core::ffi::c_char,
                section,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        }
    } else if strcmp(section, b"memory\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(
            name,
            b"buffer_size\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).buffer_size = safe_atoi(value, 0 as ::core::ffi::c_int);
        } else if strcmp(name, b"use_swap\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).use_swap = strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int;
        } else if strcmp(name, b"swap_file\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).swap_file as *mut ::core::ffi::c_char,
                value,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        } else if strcmp(name, b"swap_size\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).swap_size = strtoull(
                value,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                10 as ::core::ffi::c_int,
            ) as uint64_t;
        }
    } else if strcmp(
        section,
        b"hardware\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if strcmp(
            name,
            b"hw_accel_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).hw_accel_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"hw_accel_device\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).hw_accel_device as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                0 as size_t,
            );
        }
    } else if strcmp(section, b"go2rtc\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(name, b"enabled\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).go2rtc_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"binary_path\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).go2rtc_binary_path as *mut ::core::ffi::c_char,
                value,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        } else if strcmp(name, b"config_dir\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).go2rtc_config_dir as *mut ::core::ffi::c_char,
                value,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
        } else if strcmp(name, b"api_port\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).go2rtc_api_port = safe_atoi(value, 0 as ::core::ffi::c_int);
        } else if strcmp(name, b"rtsp_port\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).go2rtc_rtsp_port = safe_atoi(value, 0 as ::core::ffi::c_int);
        } else if strcmp(
            name,
            b"webrtc_port\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                name,
                b"webrtc_listen_port\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            (*config).go2rtc_webrtc_listen_port = safe_atoi(value, 0 as ::core::ffi::c_int);
        } else if strcmp(
            name,
            b"webrtc_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).go2rtc_webrtc_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"stun_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).go2rtc_stun_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"stun_server\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).go2rtc_stun_server as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"external_ip\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).go2rtc_external_ip as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"ice_servers\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).go2rtc_ice_servers as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"webrtc_connection_timeout_ms\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).webrtc_connection_timeout_ms = safe_atoi(value, 30000 as ::core::ffi::c_int);
            if (*config).webrtc_connection_timeout_ms < 1000 as ::core::ffi::c_int {
                (*config).webrtc_connection_timeout_ms = 1000 as ::core::ffi::c_int;
            }
        } else if strcmp(
            name,
            b"webrtc_ice_recovery_timeout_ms\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).webrtc_ice_recovery_timeout_ms = safe_atoi(value, 5000 as ::core::ffi::c_int);
            if (*config).webrtc_ice_recovery_timeout_ms < 0 as ::core::ffi::c_int {
                (*config).webrtc_ice_recovery_timeout_ms = 0 as ::core::ffi::c_int;
            }
        } else if strcmp(
            name,
            b"force_native_hls\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).go2rtc_force_native_hls =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"proxy_max_inflight\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).go2rtc_proxy_max_inflight = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).go2rtc_proxy_max_inflight < 1 as ::core::ffi::c_int {
                (*config).go2rtc_proxy_max_inflight = 1 as ::core::ffi::c_int;
            }
            if (*config).go2rtc_proxy_max_inflight > 128 as ::core::ffi::c_int {
                (*config).go2rtc_proxy_max_inflight = 128 as ::core::ffi::c_int;
            }
        } else if strcmp(
            name,
            b"turn_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).turn_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"turn_server_url\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).turn_server_url as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"turn_username\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).turn_username as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"turn_password\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).turn_password as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
    } else if strcmp(section, b"onvif\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(
            name,
            b"discovery_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).onvif_discovery_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"discovery_interval\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).onvif_discovery_interval = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).onvif_discovery_interval < 30 as ::core::ffi::c_int {
                (*config).onvif_discovery_interval = 30 as ::core::ffi::c_int;
            }
            if (*config).onvif_discovery_interval > 3600 as ::core::ffi::c_int {
                (*config).onvif_discovery_interval = 3600 as ::core::ffi::c_int;
            }
        } else if strcmp(
            name,
            b"discovery_network\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).onvif_discovery_network as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
    } else if strcmp(section, b"mqtt\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(name, b"enabled\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).mqtt_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"broker_host\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).mqtt_broker_host as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"broker_port\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).mqtt_broker_port = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).mqtt_broker_port <= 0 as ::core::ffi::c_int
                || (*config).mqtt_broker_port > 65535 as ::core::ffi::c_int
            {
                (*config).mqtt_broker_port = 1883 as ::core::ffi::c_int;
            }
        } else if strcmp(name, b"username\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).mqtt_username as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else if strcmp(name, b"password\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).mqtt_password as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                0 as size_t,
            );
        } else if strcmp(name, b"client_id\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).mqtt_client_id as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"topic_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).mqtt_topic_prefix as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"tls_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).mqtt_tls_enabled =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(name, b"keepalive\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).mqtt_keepalive = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).mqtt_keepalive < 5 as ::core::ffi::c_int {
                (*config).mqtt_keepalive = 5 as ::core::ffi::c_int;
            }
            if (*config).mqtt_keepalive > 3600 as ::core::ffi::c_int {
                (*config).mqtt_keepalive = 3600 as ::core::ffi::c_int;
            }
        } else if strcmp(name, b"qos\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).mqtt_qos = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).mqtt_qos < 0 as ::core::ffi::c_int
                || (*config).mqtt_qos > 2 as ::core::ffi::c_int
            {
                (*config).mqtt_qos = 1 as ::core::ffi::c_int;
            }
        } else if strcmp(name, b"retain\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*config).mqtt_retain = strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"ha_discovery\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                name,
                b"ha_discovery_enabled\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            (*config).mqtt_ha_discovery =
                strcmp(value, b"true\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int;
        } else if strcmp(
            name,
            b"ha_discovery_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).mqtt_ha_discovery_prefix as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                0 as size_t,
            );
        } else if strcmp(
            name,
            b"ha_snapshot_interval\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*config).mqtt_ha_snapshot_interval = safe_atoi(value, 0 as ::core::ffi::c_int);
            if (*config).mqtt_ha_snapshot_interval < 0 as ::core::ffi::c_int {
                (*config).mqtt_ha_snapshot_interval = 0 as ::core::ffi::c_int;
            }
            if (*config).mqtt_ha_snapshot_interval > 300 as ::core::ffi::c_int {
                (*config).mqtt_ha_snapshot_interval = 300 as ::core::ffi::c_int;
            }
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn load_config_from_file(
    mut filename: *const ::core::ffi::c_char,
    mut config: *mut config_t,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = ini_parse(
        filename,
        Some(
            config_ini_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        config as *mut ::core::ffi::c_void,
    );
    if result < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not read config file %s\0".as_ptr() as *const ::core::ffi::c_char,
            filename,
        );
        return -1 as ::core::ffi::c_int;
    } else if result > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Error in config file %s at line %d\0".as_ptr() as *const ::core::ffi::c_char,
            filename,
            result,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_stream_configs(mut config: *mut config_t) -> ::core::ffi::c_int {
    if config.is_null() || (*config).streams.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if (*config).max_streams <= 0 as ::core::ffi::c_int || (*config).max_streams > MAX_STREAMS {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"load_stream_configs: invalid max_streams value (%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*config).max_streams,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut max_streams: size_t = (*config).max_streams as size_t;
    if max_streams > (SIZE_MAX as usize).wrapping_div(::core::mem::size_of::<stream_config_t>()) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"load_stream_configs: max_streams causes size overflow\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        (*config).streams as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stream_config_t>().wrapping_mul(max_streams),
    );
    let mut count: ::core::ffi::c_int = count_stream_configs();
    if count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to count stream configurations in database\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if count == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"No stream configurations found in database\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut load_capacity: size_t = if (count as size_t) < max_streams {
        count as size_t
    } else {
        max_streams
    };
    if count as size_t > max_streams {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"load_stream_configs: database has %d streams, truncating to configured limit %zu\0"
                .as_ptr() as *const ::core::ffi::c_char,
            count,
            max_streams,
        );
    }
    let mut db_streams: *mut stream_config_t =
        calloc(load_capacity, ::core::mem::size_of::<stream_config_t>()) as *mut stream_config_t;
    if db_streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"load_stream_configs: out of memory allocating %zu stream configs\0".as_ptr()
                as *const ::core::ffi::c_char,
            load_capacity,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut loaded: ::core::ffi::c_int =
        get_all_stream_configs(db_streams, load_capacity as ::core::ffi::c_int);
    if loaded < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to load stream configurations from database\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(db_streams as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < loaded {
        memcpy(
            (*config).streams.offset(i as isize) as *mut ::core::ffi::c_void,
            db_streams.offset(i as isize) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<stream_config_t>(),
        );
        i += 1;
    }
    free(db_streams as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Loaded %d stream configurations from database\0".as_ptr() as *const ::core::ffi::c_char,
        loaded,
    );
    return loaded;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn save_stream_configs(mut config: *mut config_t) -> ::core::ffi::c_int {
    if config.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut saved: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if begin_transaction() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to begin transaction for saving stream configurations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = count_stream_configs();
    if count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to count stream configurations in database\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        rollback_transaction();
        return -1 as ::core::ffi::c_int;
    }
    if count == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"No stream configurations to save\0".as_ptr() as *const ::core::ffi::c_char,
        );
        commit_transaction();
        return 0 as ::core::ffi::c_int;
    }
    let mut db_streams: *mut stream_config_t = calloc(
        (*config).max_streams as size_t,
        ::core::mem::size_of::<stream_config_t>(),
    ) as *mut stream_config_t;
    if db_streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"save_stream_configs: out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        rollback_transaction();
        return -1 as ::core::ffi::c_int;
    }
    let mut loaded: ::core::ffi::c_int = get_all_stream_configs(db_streams, (*config).max_streams);
    if loaded < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to load stream configurations from database\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(db_streams as *mut ::core::ffi::c_void);
        rollback_transaction();
        return -1 as ::core::ffi::c_int;
    }
    if loaded == count {
        let mut identical: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut config_named_streams: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*config).max_streams {
            if (*(*config).streams.offset(i as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                config_named_streams += 1;
            }
            i += 1;
        }
        if config_named_streams != loaded {
            identical = 0 as ::core::ffi::c_int;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*config).max_streams && identical != 0 {
            if (*(*config).streams.offset(i_0 as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < loaded {
                    if strcmp(
                        &raw mut (*(*config).streams.offset(i_0 as isize)).name
                            as *mut ::core::ffi::c_char,
                        &raw mut (*db_streams.offset(j as isize)).name as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        found = 1 as ::core::ffi::c_int;
                        break;
                    } else {
                        j += 1;
                    }
                }
                if found == 0 {
                    identical = 0 as ::core::ffi::c_int;
                }
            }
            i_0 += 1;
        }
        if identical != 0 {
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < (*config).max_streams {
                if (*(*config).streams.offset(i_1 as isize)).name[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                {
                    let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while j_0 < loaded {
                        if strcmp(
                            &raw mut (*(*config).streams.offset(i_1 as isize)).name
                                as *mut ::core::ffi::c_char,
                            &raw mut (*db_streams.offset(j_0 as isize)).name
                                as *mut ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            safe_strcpy(
                                &raw mut (*(*config).streams.offset(i_1 as isize)).camera_uuid
                                    as *mut ::core::ffi::c_char,
                                &raw mut (*db_streams.offset(j_0 as isize)).camera_uuid
                                    as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                                0 as size_t,
                            );
                            safe_strcpy(
                                &raw mut (*(*config).streams.offset(i_1 as isize)).location_uuid
                                    as *mut ::core::ffi::c_char,
                                &raw mut (*db_streams.offset(j_0 as isize)).location_uuid
                                    as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                                0 as size_t,
                            );
                            break;
                        } else {
                            j_0 += 1;
                        }
                    }
                }
                i_1 += 1;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream configurations unchanged, skipping update\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            free(db_streams as *mut ::core::ffi::c_void);
            commit_transaction();
            return loaded;
        }
    }
    if loaded > (*config).max_streams {
        loaded = (*config).max_streams;
    }
    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_2 < loaded {
        if delete_stream_config(
            &raw mut (*db_streams.offset(i_2 as isize)).name as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to delete stream configuration: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i_2 as isize)).name as *mut ::core::ffi::c_char,
            );
            free(db_streams as *mut ::core::ffi::c_void);
            rollback_transaction();
            return -1 as ::core::ffi::c_int;
        }
        i_2 += 1;
    }
    free(db_streams as *mut ::core::ffi::c_void);
    let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_3 < (*config).max_streams {
        if strlen(
            &raw mut (*(*config).streams.offset(i_3 as isize)).name as *mut ::core::ffi::c_char,
        ) > 0 as size_t
        {
            let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
            safe_strcpy(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                &raw mut (*(*config).streams.offset(i_3 as isize)).name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
            let mut result: uint64_t = add_stream_config((*config).streams.offset(i_3 as isize));
            if result == 0 as uint64_t {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to add stream configuration: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
                rollback_transaction();
                return -1 as ::core::ffi::c_int;
            }
            if get_stream_config_by_name(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                (*config).streams.offset(i_3 as isize),
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to reload stable identity for stream: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
                rollback_transaction();
                return -1 as ::core::ffi::c_int;
            }
            saved += 1;
        }
        i_3 += 1;
    }
    if commit_transaction() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to commit transaction for saving stream configurations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        rollback_transaction();
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Saved %d stream configurations to database\0".as_ptr() as *const ::core::ffi::c_char,
        saved,
    );
    return saved;
}
static mut g_custom_config_path: [::core::ffi::c_char; 512] = [
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
static mut g_loaded_config_path: [::core::ffi::c_char; 512] = [
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_custom_config_path(mut path: *const ::core::ffi::c_char) {
    if path.is_null() || *path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return;
    }
    let mut provided_len: size_t = strlen(path);
    if provided_len >= MAX_PATH_LENGTH as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Custom config path too long, ignoring\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !strstr(path, b"/../\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        || !strstr(path, b"/..\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        || strncmp(
            path,
            b"../\0".as_ptr() as *const ::core::ffi::c_char,
            3 as size_t,
        ) == 0 as ::core::ffi::c_int
        || strcmp(path, b"..\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Custom config path contains path traversal sequence, ignoring: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            path,
        );
        return;
    }
    safe_strcpy(
        &raw mut g_custom_config_path as *mut ::core::ffi::c_char,
        path,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Custom config path set to: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_custom_config_path as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_custom_config_path() -> *const ::core::ffi::c_char {
    return if g_custom_config_path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        &raw mut g_custom_config_path as *mut ::core::ffi::c_char
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_loaded_config_path() -> *const ::core::ffi::c_char {
    return if g_loaded_config_path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        &raw mut g_loaded_config_path as *mut ::core::ffi::c_char
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
}
unsafe extern "C" fn set_loaded_config_path(mut path: *const ::core::ffi::c_char) {
    if !path.is_null() && *path.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut g_loaded_config_path as *mut ::core::ffi::c_char,
            path,
            MAX_PATH_LENGTH as size_t,
            0 as size_t,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Loaded config path set to: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_loaded_config_path as *mut ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_config(mut config: *mut config_t) -> ::core::ffi::c_int {
    if config.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    load_default_config(config);
    let mut loaded: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if g_custom_config_path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if access(
            &raw mut g_custom_config_path as *mut ::core::ffi::c_char,
            R_OK,
        ) == 0 as ::core::ffi::c_int
        {
            let mut resolved_custom_path: [::core::ffi::c_char; 4096] = [0; 4096];
            let mut canon_path: *const ::core::ffi::c_char = realpath(
                &raw mut g_custom_config_path as *mut ::core::ffi::c_char,
                &raw mut resolved_custom_path as *mut ::core::ffi::c_char,
            );
            if canon_path.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to resolve config path '%s': %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut g_custom_config_path as *mut ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
            } else if load_config_from_file(canon_path, config) == 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Loaded configuration from custom path: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    canon_path,
                );
                set_loaded_config_path(canon_path);
                loaded = 1 as ::core::ffi::c_int;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to load configuration from custom path: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    canon_path,
                );
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Custom config file not accessible: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut g_custom_config_path as *mut ::core::ffi::c_char,
            );
        }
    }
    if loaded == 0 {
        let mut config_paths: [*const ::core::ffi::c_char; 4] = [
            b"./lightnvr.ini\0".as_ptr() as *const ::core::ffi::c_char,
            b"/etc/lightnvr/lightnvr.ini\0".as_ptr() as *const ::core::ffi::c_char,
            b"./config/lightnvr.ini\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !config_paths[i as usize].is_null() && loaded == 0 {
            if access(config_paths[i as usize], R_OK) == 0 as ::core::ffi::c_int {
                if load_config_from_file(config_paths[i as usize], config)
                    == 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Loaded configuration from %s\0".as_ptr() as *const ::core::ffi::c_char,
                        config_paths[i as usize],
                    );
                    set_loaded_config_path(config_paths[i as usize]);
                    loaded = 1 as ::core::ffi::c_int;
                    break;
                }
            }
            i += 1;
        }
    }
    if loaded == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"No configuration file found, using defaults\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    apply_env_overrides(config);
    if strlen(&raw mut (*config).web_root as *mut ::core::ffi::c_char) == 0 as size_t {
        safe_strcpy(
            &raw mut (*config).web_root as *mut ::core::ffi::c_char,
            b"/var/www/lightnvr\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Using web root: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*config).web_root as *mut ::core::ffi::c_char,
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
    if stat(
        &raw mut (*config).web_root as *mut ::core::ffi::c_char,
        &raw mut st,
    ) != 0 as ::core::ffi::c_int
        || !(st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Web root directory %s does not exist or is not a directory\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*config).web_root as *mut ::core::ffi::c_char,
        );
    }
    if validate_config(config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid configuration\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if ensure_directories(config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create required directories\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn save_config(
    mut config: *const config_t,
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid config parameter for save_config: config=%p\0".as_ptr()
                as *const ::core::ffi::c_char,
            config,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut save_path: *const ::core::ffi::c_char = path;
    if save_path.is_null()
        || *save_path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        save_path = get_loaded_config_path();
        if save_path.is_null() {
            save_path = get_custom_config_path();
        }
        if save_path.is_null() {
            static mut defaults: [*const ::core::ffi::c_char; 4] = [
                b"./lightnvr.ini\0".as_ptr() as *const ::core::ffi::c_char,
                b"/etc/lightnvr/lightnvr.ini\0".as_ptr() as *const ::core::ffi::c_char,
                b"./config/lightnvr.ini\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ];
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while !defaults[i as usize].is_null() {
                if access(defaults[i as usize], W_OK) == 0 as ::core::ffi::c_int {
                    save_path = defaults[i as usize];
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Using fallback config path for save: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        save_path,
                    );
                    break;
                } else {
                    i += 1;
                }
            }
        }
        if save_path.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"No path specified and no loaded config path available\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Attempting to save configuration to %s\0".as_ptr() as *const ::core::ffi::c_char,
        save_path,
    );
    let mut dir_path: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut dir_path as *mut ::core::ffi::c_char,
        save_path,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    let mut last_slash: *mut ::core::ffi::c_char = strrchr(
        &raw mut dir_path as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if !last_slash.is_null() {
        *last_slash = '\0' as ::core::ffi::c_char;
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
        if stat(&raw mut dir_path as *mut ::core::ffi::c_char, &raw mut st)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Directory %s does not exist, attempting to create it\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut dir_path as *mut ::core::ffi::c_char,
            );
            if mkdir_recursive(&raw mut dir_path as *mut ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to create directory %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut dir_path as *mut ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
                return -1 as ::core::ffi::c_int;
            }
        } else if !(st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Path %s exists but is not a directory\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut dir_path as *mut ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if access(&raw mut dir_path as *mut ::core::ffi::c_char, W_OK) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Directory %s is not writable: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut dir_path as *mut ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    let mut canonical_save_path: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut resolved_dir: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut validated_filename: [::core::ffi::c_char; 512] = [0; 512];
    let mut tmp: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut tmp as *mut ::core::ffi::c_char,
        save_path,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    let mut fname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut sl: *mut ::core::ffi::c_char = strrchr(
        &raw mut tmp as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if !sl.is_null() {
        fname = sl.offset(1 as ::core::ffi::c_int as isize);
        *sl = '\0' as ::core::ffi::c_char;
        if realpath(
            &raw mut tmp as *mut ::core::ffi::c_char,
            &raw mut resolved_dir as *mut ::core::ffi::c_char,
        )
        .is_null()
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cannot resolve config directory '%s': %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut tmp as *mut ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
    } else {
        fname = &raw mut tmp as *mut ::core::ffi::c_char;
        let mut cwd: [::core::ffi::c_char; 4096] = [0; 4096];
        if getcwd(
            &raw mut cwd as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        )
        .is_null()
            || realpath(
                &raw mut cwd as *mut ::core::ffi::c_char,
                &raw mut resolved_dir as *mut ::core::ffi::c_char,
            )
            .is_null()
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cannot resolve current working directory: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if *fname.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || !strchr(fname, '/' as ::core::ffi::c_int).is_null()
        || strcmp(fname, b"..\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strcmp(fname, b".\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid config filename: '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            fname,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut fname_len: size_t = strlen(fname);
    if fname_len < 4 as size_t
        || strcmp(
            fname
                .offset(fname_len as isize)
                .offset(-(4 as ::core::ffi::c_int as isize)),
            b".ini\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Config filename must end with .ini: '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            fname,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut n: ::core::ffi::c_int = snprintf(
        &raw mut canonical_save_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut resolved_dir as *mut ::core::ffi::c_char,
        fname,
    );
    if n < 0 as ::core::ffi::c_int
        || n as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Canonical config path too long\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut fname_written: ::core::ffi::c_int = snprintf(
        &raw mut validated_filename as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        fname,
    );
    if fname_written < 0 as ::core::ffi::c_int
        || fname_written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Config filename too long: '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            fname,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut dir_fd: ::core::ffi::c_int = open(
        &raw mut resolved_dir as *mut ::core::ffi::c_char,
        O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC,
    );
    if dir_fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not open config directory for writing: %s (error: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut resolved_dir as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut config_fd: ::core::ffi::c_int = openat(
        dir_fd,
        &raw mut validated_filename as *mut ::core::ffi::c_char,
        O_WRONLY | O_CREAT | O_NOFOLLOW | O_CLOEXEC | O_NONBLOCK,
        0o600 as ::core::ffi::c_int,
    );
    if config_fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not open config file for writing: %s (error: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut canonical_save_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(dir_fd);
        return -1 as ::core::ffi::c_int;
    }
    let mut config_stat: stat = stat {
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
    if fstat(config_fd, &raw mut config_stat) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not stat config file after opening: %s (error: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut canonical_save_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(config_fd);
        close(dir_fd);
        return -1 as ::core::ffi::c_int;
    }
    if !(config_stat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
        || config_stat.st_nlink != 1 as __nlink_t
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Config save target must be a regular file with one link: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut canonical_save_path as *mut ::core::ffi::c_char,
        );
        close(config_fd);
        close(dir_fd);
        return -1 as ::core::ffi::c_int;
    }
    let mut replacement_name: [::core::ffi::c_char; 96] = [
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
    ];
    if fchmod(config_fd, 0o600 as __mode_t) != 0 as ::core::ffi::c_int {
        let mut mode_error: ::core::ffi::c_int = *__errno_location();
        close(config_fd);
        config_fd = -1 as ::core::ffi::c_int;
        if mode_error == EPERM {
            static mut replacement_sequence: atomic_uint = 0 as ::core::ffi::c_uint;
            let mut attempt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while attempt < 100 as ::core::ffi::c_int {
                snprintf(
                    &raw mut replacement_name as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
                    b".lightnvr-config-%ld-%u.tmp\0".as_ptr() as *const ::core::ffi::c_char,
                    getpid() as ::core::ffi::c_long,
                    ::core::intrinsics::atomic_xadd::<
                        _,
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut replacement_sequence, 1 as ::core::ffi::c_uint),
                );
                config_fd = openat(
                    dir_fd,
                    &raw mut replacement_name as *mut ::core::ffi::c_char,
                    O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW | O_CLOEXEC,
                    0o600 as ::core::ffi::c_int,
                );
                if config_fd >= 0 as ::core::ffi::c_int || *__errno_location() != EEXIST {
                    break;
                }
                attempt += 1;
            }
        }
        if config_fd < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Could not prepare private config replacement: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            close(dir_fd);
            return -1 as ::core::ffi::c_int;
        }
    }
    if replacement_name[0usize] as ::core::ffi::c_int != 0
        && fchmod(config_fd, 0o600 as __mode_t) != 0 as ::core::ffi::c_int
        || ftruncate(config_fd, 0 as __off_t) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not prepare private config file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(config_fd);
        if replacement_name[0usize] != 0 {
            unlinkat(
                dir_fd,
                &raw mut replacement_name as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
        }
        close(dir_fd);
        return -1 as ::core::ffi::c_int;
    }
    let mut file: *mut FILE = fdopen(config_fd, b"w\0".as_ptr() as *const ::core::ffi::c_char);
    if file.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not open config file for writing: %s (error: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut canonical_save_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        close(config_fd);
        if replacement_name[0usize] != 0 {
            unlinkat(
                dir_fd,
                &raw mut replacement_name as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
        }
        close(dir_fd);
        return -1 as ::core::ffi::c_int;
    }
    fprintf(
        file,
        b"; LightNVR Configuration File (INI format)\n\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"[general]\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"pid_file = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).pid_file as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"log_file = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).log_file as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"log_level = %d  ; 0=ERROR, 1=WARN, 2=INFO, 3=DEBUG\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*config).log_level,
    );
    fprintf(
        file,
        b"syslog_enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).syslog_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"syslog_ident = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).syslog_ident as *const ::core::ffi::c_char,
    );
    let mut facility_name: *const ::core::ffi::c_char =
        b"LOG_USER\0".as_ptr() as *const ::core::ffi::c_char;
    match (*config).syslog_facility {
        LOG_USER => {
            facility_name = b"LOG_USER\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_DAEMON => {
            facility_name = b"LOG_DAEMON\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL0 => {
            facility_name = b"LOG_LOCAL0\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL1 => {
            facility_name = b"LOG_LOCAL1\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL2 => {
            facility_name = b"LOG_LOCAL2\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL3 => {
            facility_name = b"LOG_LOCAL3\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL4 => {
            facility_name = b"LOG_LOCAL4\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL5 => {
            facility_name = b"LOG_LOCAL5\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL6 => {
            facility_name = b"LOG_LOCAL6\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL7 => {
            facility_name = b"LOG_LOCAL7\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    fprintf(
        file,
        b"syslog_facility = %s  ; Syslog facility for system logging\n\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        facility_name,
    );
    fprintf(file, b"[health]\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        file,
        b"enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).health.enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"profile = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).health.profile as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"fast_interval_seconds = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).health.fast_interval_seconds,
    );
    fprintf(
        file,
        b"normal_interval_seconds = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).health.normal_interval_seconds,
    );
    fprintf(
        file,
        b"slow_interval_seconds = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).health.slow_interval_seconds,
    );
    fprintf(
        file,
        b"device_interval_seconds = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).health.device_interval_seconds,
    );
    fprintf(
        file,
        b"write_probe_enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).health.write_probe_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"hardware_provider = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).health.hardware_provider as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"presence_interval_seconds = %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).health.presence_interval_seconds,
    );
    fprintf(
        file,
        b"incident_retention_days = %u\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).health.incident_retention_days,
    );
    fprintf(
        file,
        b"[storage]\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"path = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).storage_path as *const ::core::ffi::c_char,
    );
    if (*config).storage_path_hls[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        fprintf(
            file,
            b"path_hls = %s  ; Dedicated path for HLS segments\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*config).storage_path_hls as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        file,
        b"max_size = %llu  ; 0 means unlimited, otherwise bytes\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*config).max_storage_size as ::core::ffi::c_ulonglong,
    );
    fprintf(
        file,
        b"retention_days = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).retention_days,
    );
    fprintf(
        file,
        b"auto_delete_oldest = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).auto_delete_oldest as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"mp4_faststart = %s  ; false avoids the post-segment rewrite (less I/O on NAS/HDD)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if config_get_mp4_faststart(config) as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"min_free_pct = %d  ; keep at least this %% of the volume free (0 disables capacity cap)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*config).storage_min_free_pct,
    );
    fprintf(
        file,
        b"pressure_warning_pct = %.1f\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).storage_pressure_warning_pct,
    );
    fprintf(
        file,
        b"pressure_critical_pct = %.1f\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).storage_pressure_critical_pct,
    );
    fprintf(
        file,
        b"pressure_emergency_pct = %.1f\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).storage_pressure_emergency_pct,
    );
    fprintf(
        file,
        b"; New recording format options\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"record_mp4_directly = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).record_mp4_directly as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if (*config).mp4_storage_path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        fprintf(
            file,
            b"mp4_path = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).mp4_storage_path as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        file,
        b"mp4_directory_format = %s  ; flat, year_month, or year_month_day\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*config).mp4_directory_format as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"mp4_segment_duration = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).mp4_segment_duration,
    );
    fprintf(
        file,
        b"mp4_retention_days = %d\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).mp4_retention_days,
    );
    fprintf(
        file,
        b"; Thumbnail preview settings\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"generate_thumbnails = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).generate_thumbnails as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"; Thumbnails per recording: 1 = single first-frame thumb (saves CPU on slow hardware); 3 = start/middle/end with hover cycling\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"thumbnails_per_recording = %d\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).thumbnails_per_recording,
    );
    fprintf(file, b"[models]\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        file,
        b"path = %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).models_path as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"[detection]\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"grace_period = %d  ; Seconds after last detection before entering post-buffer (0-60, default: 2)\n\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*config).detection_grace_period,
    );
    fprintf(
        file,
        b"[api_detection]\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"url = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).api_detection_url as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"backend = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).api_detection_backend as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"format = %s  ; Request format: light-object-detect (multipart upload) or doods2 (JSON body)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).api_detection_format as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"detector_name = %s  ; DOODS2 only: detector_name sent with each request\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*config).api_detection_detector_name as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"detection_threshold = %d  ; Default confidence threshold (0-100%%)\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*config).default_detection_threshold,
    );
    fprintf(
        file,
        b"pre_detection_buffer = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).default_pre_detection_buffer,
    );
    fprintf(
        file,
        b"post_detection_buffer = %d\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).default_post_detection_buffer,
    );
    fprintf(
        file,
        b"[detection_engine]\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"; In-process TFLite/LiteRT inference. Streams using a .tflite model_path\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"; will route through this engine. Per-model labels are read from a sidecar\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"; <basename>.labels.txt next to the model.\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).detection_engine.enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"threads = %d  ; Interpreter threads (1-16, default: 1)\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*config).detection_engine.num_threads,
    );
    fprintf(
        file,
        b"delegate = %s  ; xnnpack | gpu | none (default: xnnpack; gpu requires -DLITERT_WITH_GPU=ON)\n\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).detection_engine.delegate as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"[database]\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"path = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).db_path as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"backup_interval_minutes = %d  ; 0 disables periodic runtime backups\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*config).db_backup_interval_minutes,
    );
    fprintf(
        file,
        b"backup_retention_count = %d  ; Number of timestamped backups to keep\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*config).db_backup_retention_count,
    );
    fprintf(
        file,
        b"startup_check = %s  ; Boot consistency check: off (default), quick, or full\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        if (*config).db_startup_check == DB_STARTUP_CHECK_OFF {
            b"off\0".as_ptr() as *const ::core::ffi::c_char
        } else if (*config).db_startup_check == DB_STARTUP_CHECK_FULL {
            b"full\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"quick\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"backup_verify = %s  ; Post-copy backup verification: full (default), quick, or off\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if (*config).db_backup_verify == DB_BACKUP_VERIFY_OFF {
            b"off\0".as_ptr() as *const ::core::ffi::c_char
        } else if (*config).db_backup_verify == DB_BACKUP_VERIFY_QUICK {
            b"quick\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"full\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"post_backup_script = %s  ; Optional absolute path to executable hook\n\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*config).db_post_backup_script as *const ::core::ffi::c_char,
    );
    fprintf(file, b"[web]\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        file,
        b"web_thread_pool_size = %d  ; libuv UV_THREADPOOL_SIZE (default: 2x CPU cores; requires restart)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*config).web_thread_pool_size,
    );
    fprintf(
        file,
        b"port = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).web_port,
    );
    fprintf(
        file,
        b"bind_ip = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).web_bind_ip as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"root = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).web_root as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"ssl_enabled = %s  ; Serve HTTPS directly (needs a build with -DENABLE_SSL=ON)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if (*config).web_ssl_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"ssl_cert_path = %s  ; PEM certificate chain, leaf first\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*config).web_ssl_cert_path as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"ssl_key_path = %s  ; PEM private key (unencrypted)\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*config).web_ssl_key_path as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"auth_enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).web_auth_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"username = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).web_username as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"audio_disabled = %s  ; Instance-wide audio compliance policy\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        if (*config).audio_disabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"auto_disabled = %s  ; Hide Auto view and use the first enabled explicit transport\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if (*config).auto_disabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"webrtc_disabled = %s  ; Hide WebRTC view on dashboard\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        if (*config).webrtc_disabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"hls_disabled = %s     ; Hide HLS view on dashboard\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        if (*config).hls_disabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"mse_disabled = %s     ; Hide MSE view on dashboard\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        if (*config).mse_disabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"default_playback_transport = %s  ; Default transport profile for new streams\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*config).default_playback_transport as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"auth_timeout_hours = %d  ; Session idle timeout in hours (default: 24)\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*config).auth_timeout_hours,
    );
    fprintf(
        file,
        b"auth_absolute_timeout_hours = %d  ; Absolute session lifetime in hours (default: 168)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*config).auth_absolute_timeout_hours,
    );
    fprintf(
        file,
        b"trusted_device_days = %d  ; Remember trusted device for N days (0 disables, default: 30)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*config).trusted_device_days,
    );
    fprintf(
        file,
        b"trusted_proxy_cidrs = %s  ; Trusted reverse-proxy CIDRs for X-Forwarded-For (blank disables trust)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).trusted_proxy_cidrs as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"demo_mode = %s  ; Demo mode: allows unauthenticated viewer access\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        if (*config).demo_mode as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"force_mfa_on_login = %s  ; Require TOTP code with password at login (prevents password-only brute force)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if (*config).force_mfa_on_login as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"login_rate_limit_enabled = %s  ; Enable login rate limiting\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        if (*config).login_rate_limit_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"login_rate_limit_max_attempts = %d  ; Max login attempts before lockout (default: 5)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*config).login_rate_limit_max_attempts,
    );
    fprintf(
        file,
        b"login_rate_limit_window_seconds = %d  ; Rate limit window in seconds (default: 300)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*config).login_rate_limit_window_seconds,
    );
    fprintf(file, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        file,
        b"[streams]\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"max_streams = %d  ; Runtime stream slot limit (default: %d, ceiling: %d; requires restart)\n\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*config).max_streams,
        DEFAULT_MAX_STREAMS,
        MAX_STREAMS,
    );
    fprintf(file, b"[memory]\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        file,
        b"buffer_size = %d  ; Buffer size in KB\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).buffer_size,
    );
    fprintf(
        file,
        b"use_swap = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).use_swap as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"swap_file = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).swap_file as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"swap_size = %llu  ; Size in bytes\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).swap_size as ::core::ffi::c_ulonglong,
    );
    fprintf(
        file,
        b"[hardware]\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"hw_accel_enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).hw_accel_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"hw_accel_device = %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).hw_accel_device as *const ::core::ffi::c_char,
    );
    fprintf(file, b"[go2rtc]\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        file,
        b"enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).go2rtc_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"binary_path = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).go2rtc_binary_path as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"config_dir = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).go2rtc_config_dir as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"api_port = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).go2rtc_api_port,
    );
    fprintf(
        file,
        b"rtsp_port = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).go2rtc_rtsp_port,
    );
    fprintf(
        file,
        b"webrtc_enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).go2rtc_webrtc_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"webrtc_listen_port = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).go2rtc_webrtc_listen_port,
    );
    fprintf(
        file,
        b"stun_enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).go2rtc_stun_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"stun_server = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).go2rtc_stun_server as *const ::core::ffi::c_char,
    );
    if (*config).go2rtc_external_ip[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        fprintf(
            file,
            b"external_ip = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).go2rtc_external_ip as *const ::core::ffi::c_char,
        );
    }
    if (*config).go2rtc_ice_servers[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        fprintf(
            file,
            b"ice_servers = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).go2rtc_ice_servers as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        file,
        b"webrtc_connection_timeout_ms = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).webrtc_connection_timeout_ms,
    );
    fprintf(
        file,
        b"webrtc_ice_recovery_timeout_ms = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).webrtc_ice_recovery_timeout_ms,
    );
    fprintf(
        file,
        b"force_native_hls = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).go2rtc_force_native_hls as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"proxy_max_inflight = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).go2rtc_proxy_max_inflight,
    );
    fprintf(
        file,
        b"turn_enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).turn_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if (*config).turn_server_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        fprintf(
            file,
            b"turn_server_url = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).turn_server_url as *const ::core::ffi::c_char,
        );
    }
    if (*config).turn_username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        fprintf(
            file,
            b"turn_username = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).turn_username as *const ::core::ffi::c_char,
        );
    }
    if (*config).turn_password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        fprintf(
            file,
            b"turn_password = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).turn_password as *const ::core::ffi::c_char,
        );
    }
    fprintf(file, b"\n[mqtt]\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        file,
        b"enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).mqtt_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if (*config).mqtt_broker_host[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        fprintf(
            file,
            b"broker_host = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).mqtt_broker_host as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        file,
        b"broker_port = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).mqtt_broker_port,
    );
    if (*config).mqtt_username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        fprintf(
            file,
            b"username = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).mqtt_username as *const ::core::ffi::c_char,
        );
    }
    if (*config).mqtt_password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        fprintf(
            file,
            b"password = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).mqtt_password as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        file,
        b"client_id = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).mqtt_client_id as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"topic_prefix = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).mqtt_topic_prefix as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"tls_enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).mqtt_tls_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"keepalive = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).mqtt_keepalive,
    );
    fprintf(
        file,
        b"qos = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).mqtt_qos,
    );
    fprintf(
        file,
        b"retain = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).mqtt_retain as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"; Home Assistant MQTT auto-discovery\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"ha_discovery = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).mqtt_ha_discovery as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"ha_discovery_prefix = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).mqtt_ha_discovery_prefix as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"ha_snapshot_interval = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).mqtt_ha_snapshot_interval,
    );
    fprintf(
        file,
        b"\n[onvif]\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        file,
        b"discovery_enabled = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).onvif_discovery_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        file,
        b"discovery_interval = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).onvif_discovery_interval,
    );
    fprintf(
        file,
        b"discovery_network = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).onvif_discovery_network as *const ::core::ffi::c_char,
    );
    let mut write_failed: ::core::ffi::c_int = ferror(file);
    if replacement_name[0usize] as ::core::ffi::c_int != 0
        && (fflush(file) != 0 as ::core::ffi::c_int
            || fsync(fileno(file)) != 0 as ::core::ffi::c_int)
    {
        write_failed = 1 as ::core::ffi::c_int;
    }
    if fclose(file) != 0 as ::core::ffi::c_int {
        write_failed = 1 as ::core::ffi::c_int;
    }
    if write_failed == 0 && replacement_name[0usize] as ::core::ffi::c_int != 0 {
        let mut current: stat = stat {
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
        if fstatat(
            dir_fd,
            &raw mut validated_filename as *mut ::core::ffi::c_char,
            &raw mut current,
            AT_SYMLINK_NOFOLLOW,
        ) != 0 as ::core::ffi::c_int
            || !(current.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
            || current.st_nlink != 1 as __nlink_t
            || current.st_dev != config_stat.st_dev
            || current.st_ino != config_stat.st_ino
            || renameat(
                dir_fd,
                &raw mut replacement_name as *mut ::core::ffi::c_char,
                dir_fd,
                &raw mut validated_filename as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            write_failed = 1 as ::core::ffi::c_int;
        }
    }
    if replacement_name[0usize] != 0 {
        unlinkat(
            dir_fd,
            &raw mut replacement_name as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
    }
    close(dir_fd);
    if write_failed != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to write config file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut canonical_save_path as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_config(mut config: *const config_t) {
    if config.is_null() {
        return;
    }
    printf(b"LightNVR Configuration:\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(b"  General Settings:\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"    PID File: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).pid_file as *const ::core::ffi::c_char,
    );
    printf(
        b"    Log File: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).log_file as *const ::core::ffi::c_char,
    );
    printf(
        b"    Log Level: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).log_level,
    );
    printf(b"  Storage Settings:\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"    Storage Path: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).storage_path as *const ::core::ffi::c_char,
    );
    if (*config).storage_path_hls[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        printf(
            b"    HLS Storage Path: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).storage_path_hls as *const ::core::ffi::c_char,
        );
    }
    printf(
        b"    Max Storage Size: %llu bytes\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).max_storage_size as ::core::ffi::c_ulonglong,
    );
    printf(
        b"    Retention Days: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).retention_days,
    );
    printf(
        b"    Auto Delete Oldest: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).auto_delete_oldest as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    MP4 Faststart: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if config_get_mp4_faststart(config) as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(b"  Health Settings:\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"    Enabled: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).health.enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    Profile: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).health.profile as *const ::core::ffi::c_char,
    );
    printf(
        b"    Sampling intervals: %u/%u/%u/%u seconds\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).health.fast_interval_seconds,
        (*config).health.normal_interval_seconds,
        (*config).health.slow_interval_seconds,
        (*config).health.device_interval_seconds,
    );
    printf(b"  Models Settings:\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"    Models Path: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).models_path as *const ::core::ffi::c_char,
    );
    printf(b"  API Detection Settings:\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"    API URL: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).api_detection_url as *const ::core::ffi::c_char,
    );
    printf(b"  Database Settings:\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"    Database Path: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).db_path as *const ::core::ffi::c_char,
    );
    printf(
        b"    Backup Interval: %d minutes\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).db_backup_interval_minutes,
    );
    printf(
        b"    Backup Retention Count: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).db_backup_retention_count,
    );
    printf(
        b"    Startup Check: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).db_startup_check == DB_STARTUP_CHECK_OFF {
            b"off\0".as_ptr() as *const ::core::ffi::c_char
        } else if (*config).db_startup_check == DB_STARTUP_CHECK_FULL {
            b"full\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"quick\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    Backup Verification: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).db_backup_verify == DB_BACKUP_VERIFY_OFF {
            b"off\0".as_ptr() as *const ::core::ffi::c_char
        } else if (*config).db_backup_verify == DB_BACKUP_VERIFY_QUICK {
            b"quick\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"full\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    Post-backup Script: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).db_post_backup_script[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*config).db_post_backup_script as *const ::core::ffi::c_char
        } else {
            b"(disabled)\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(b"  Web Server Settings:\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"    Web Port: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).web_port,
    );
    printf(
        b"    Web Bind Address: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).web_bind_ip as *const ::core::ffi::c_char,
    );
    printf(
        b"    Web Root: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).web_root as *const ::core::ffi::c_char,
    );
    printf(
        b"    Web Auth Enabled: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).web_auth_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    Web Username: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).web_username as *const ::core::ffi::c_char,
    );
    printf(
        b"    Web Password: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"********\0".as_ptr() as *const ::core::ffi::c_char,
    );
    printf(
        b"    Audio Disabled: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).audio_disabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    Auto Disabled: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).auto_disabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    WebRTC Disabled: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).webrtc_disabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    HLS Disabled: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).hls_disabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    MSE Disabled: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).mse_disabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    Auth Idle Timeout: %d hours\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).auth_timeout_hours,
    );
    printf(
        b"    Auth Absolute Timeout: %d hours\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).auth_absolute_timeout_hours,
    );
    printf(
        b"    Trusted Device Lifetime: %d days\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).trusted_device_days,
    );
    printf(
        b"    Demo Mode: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).demo_mode as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    Force MFA on Login: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).force_mfa_on_login as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    Login Rate Limit: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).login_rate_limit_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    Login Rate Limit Max Attempts: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).login_rate_limit_max_attempts,
    );
    printf(
        b"    Login Rate Limit Window: %d seconds\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).login_rate_limit_window_seconds,
    );
    printf(b"  Stream Settings:\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"    Max Streams: %d (runtime) / %d (compile-time ceiling)\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*config).max_streams,
        MAX_STREAMS,
    );
    printf(
        b"  Web Thread Pool Size: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).web_thread_pool_size,
    );
    printf(b"  Memory Optimization:\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"    Buffer Size: %d KB\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).buffer_size,
    );
    printf(
        b"    Use Swap: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).use_swap as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    Swap File: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).swap_file as *const ::core::ffi::c_char,
    );
    printf(
        b"    Swap Size: %llu bytes\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*config).swap_size as ::core::ffi::c_ulonglong,
    );
    printf(b"  Hardware Acceleration:\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"    HW Accel Enabled: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if (*config).hw_accel_enabled as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"    HW Accel Device: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).hw_accel_device as *const ::core::ffi::c_char,
    );
    printf(b"  Stream Configurations:\n\0".as_ptr() as *const ::core::ffi::c_char);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*config).max_streams {
        if strlen(&raw mut (*(*config).streams.offset(i as isize)).name as *mut ::core::ffi::c_char)
            > 0 as size_t
        {
            printf(
                b"    Stream %d:\n\0".as_ptr() as *const ::core::ffi::c_char,
                i,
            );
            printf(
                b"      Name: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(*config).streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
            );
            printf(
                b"      URL: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(*config).streams.offset(i as isize)).url as *mut ::core::ffi::c_char,
            );
            printf(
                b"      Enabled: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if (*(*config).streams.offset(i as isize)).enabled as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            printf(
                b"      Resolution: %dx%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*(*config).streams.offset(i as isize)).width,
                (*(*config).streams.offset(i as isize)).height,
            );
            printf(
                b"      FPS: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*(*config).streams.offset(i as isize)).fps,
            );
            printf(
                b"      Codec: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(*config).streams.offset(i as isize)).codec as *mut ::core::ffi::c_char,
            );
            printf(
                b"      Priority: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*(*config).streams.offset(i as isize)).priority,
            );
            printf(
                b"      Record: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if (*(*config).streams.offset(i as isize)).record as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            printf(
                b"      Segment Duration: %d seconds\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*(*config).streams.offset(i as isize)).segment_duration,
            );
            printf(
                b"      Detection-based Recording: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if (*(*config).streams.offset(i as isize)).detection_based_recording
                    as ::core::ffi::c_int
                    != 0
                {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            printf(
                b"      Record Audio: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if (*(*config).streams.offset(i as isize)).record_audio as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            if (*(*config).streams.offset(i as isize)).detection_based_recording {
                printf(
                    b"      Detection Model: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    if (*(*config).streams.offset(i as isize)).detection_model[0usize]
                        as ::core::ffi::c_int
                        != 0
                    {
                        &raw mut (*(*config).streams.offset(i as isize)).detection_model
                            as *mut ::core::ffi::c_char
                    } else {
                        b"None\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                printf(
                    b"      Detection Interval: %d seconds\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(*config).streams.offset(i as isize)).detection_interval,
                );
                printf(
                    b"      Detection Threshold: %.2f\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*(*config).streams.offset(i as isize)).detection_threshold
                        as ::core::ffi::c_double,
                );
                printf(
                    b"      Pre-detection Buffer: %d seconds\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(*config).streams.offset(i as isize)).pre_detection_buffer,
                );
                printf(
                    b"      Post-detection Buffer: %d seconds\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(*config).streams.offset(i as isize)).post_detection_buffer,
                );
            }
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_detection_format_name(
    mut format: api_detection_format_t,
) -> *const ::core::ffi::c_char {
    match format {
        api_detection_format_t::API_DETECTION_FORMAT_DOODS2 => {
            return API_DETECTION_FORMAT_NAME_DOODS2.as_ptr();
        }
        api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT | _ => {
            return API_DETECTION_FORMAT_NAME_LIGHT_OBJECT_DETECT.as_ptr();
        }
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_detection_format_parse(
    mut name: *const ::core::ffi::c_char,
    mut out: *mut api_detection_format_t,
) -> bool {
    if name.is_null() || out.is_null() {
        return r#false != 0;
    }
    while *name as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *name as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        name = name.offset(1);
    }
    let mut len: size_t = strlen(name);
    while len > 0 as size_t
        && (*name.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == ' ' as ::core::ffi::c_int
            || *name.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                == '\t' as ::core::ffi::c_int)
    {
        len = len.wrapping_sub(1);
    }
    if len == 0 as size_t || len >= API_DETECTION_FORMAT_MAX as size_t {
        return r#false != 0;
    }
    let mut trimmed: [::core::ffi::c_char; 32] = [0; 32];
    memcpy(
        &raw mut trimmed as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        name as *const ::core::ffi::c_void,
        len,
    );
    trimmed[len] = '\0' as ::core::ffi::c_char;
    if strcasecmp(
        &raw mut trimmed as *mut ::core::ffi::c_char,
        API_DETECTION_FORMAT_NAME_LIGHT_OBJECT_DETECT.as_ptr(),
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            &raw mut trimmed as *mut ::core::ffi::c_char,
            b"light_object_detect\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            &raw mut trimmed as *mut ::core::ffi::c_char,
            b"lod\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        *out = api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT;
        return r#true != 0;
    }
    if strcasecmp(
        &raw mut trimmed as *mut ::core::ffi::c_char,
        API_DETECTION_FORMAT_NAME_DOODS2.as_ptr(),
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            &raw mut trimmed as *mut ::core::ffi::c_char,
            b"doods\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        *out = api_detection_format_t::API_DETECTION_FORMAT_DOODS2;
        return r#true != 0;
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn config_set_api_detection_format(
    mut config: *mut config_t,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    let mut format: api_detection_format_t =
        api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT;
    if config.is_null() || !api_detection_format_parse(name, &raw mut format) {
        return r#false != 0;
    }
    safe_strcpy(
        &raw mut (*config).api_detection_format as *mut ::core::ffi::c_char,
        api_detection_format_name(format),
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        0 as size_t,
    );
    return r#true != 0;
}
pub const GO2RTC_BINARY_PATH_RAW: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"/usr/local/bin/go2rtc\0")
};
pub const GO2RTC_CONFIG_DIR_RAW: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"/etc/lightnvr/go2rtc\0")
};
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
