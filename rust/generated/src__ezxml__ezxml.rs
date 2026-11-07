use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    unsafe fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    unsafe fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    unsafe fn madvise(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __advice: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
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
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
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
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ezxml {
    pub name: *mut ::core::ffi::c_char,
    pub attr: *mut *mut ::core::ffi::c_char,
    pub txt: *mut ::core::ffi::c_char,
    pub off: size_t,
    pub next: ezxml_t,
    pub sibling: ezxml_t,
    pub ordered: ezxml_t,
    pub child: ezxml_t,
    pub parent: ezxml_t,
    pub flags: ::core::ffi::c_short,
}
pub type ezxml_t = *mut ezxml;
pub type ezxml_root_t = *mut ezxml_root;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ezxml_root {
    pub xml: ezxml,
    pub cur: ezxml_t,
    pub m: *mut ::core::ffi::c_char,
    pub len: size_t,
    pub u: *mut ::core::ffi::c_char,
    pub s: *mut ::core::ffi::c_char,
    pub e: *mut ::core::ffi::c_char,
    pub ent: *mut *mut ::core::ffi::c_char,
    pub attr: *mut *mut *mut ::core::ffi::c_char,
    pub pi: *mut *mut *mut ::core::ffi::c_char,
    pub standalone: ::core::ffi::c_short,
    pub err: [::core::ffi::c_char; 128],
}
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MADV_NORMAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MADV_SEQUENTIAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAP_FAILED: *mut ::core::ffi::c_void = ::core::ptr::with_exposed_provenance_mut::<
    ::core::ffi::c_void,
>(-1 as ::core::ffi::c_int as usize);
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EZXML_BUFSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const EZXML_NAMEM: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const EZXML_TXTM: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const EZXML_DUP: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const EZXML_WS: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"\t\r\n \0") };
pub const EZXML_ERRL: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut EZXML_NIL: [*mut ::core::ffi::c_char; 1] =
    [::core::ptr::null_mut::<::core::ffi::c_char>()];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_child(
    mut xml: ezxml_t,
    mut name: *const ::core::ffi::c_char,
) -> ezxml_t {
    xml = if !xml.is_null() {
        (*xml).child
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    while !xml.is_null() && strcmp(name, (*xml).name) != 0 {
        xml = (*xml).sibling;
    }
    return xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_idx(mut xml: ezxml_t, mut idx: ::core::ffi::c_int) -> ezxml_t {
    while !xml.is_null() && idx != 0 {
        xml = (*xml).next;
        idx -= 1;
    }
    return xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_attr(
    mut xml: ezxml_t,
    mut attr: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut root: ezxml_root_t = xml as ezxml_root_t;
    if xml.is_null() || (*xml).attr.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    while !(*(*xml).attr.offset(i as isize)).is_null()
        && strcmp(attr, *(*xml).attr.offset(i as isize)) != 0
    {
        i += 2 as ::core::ffi::c_int;
    }
    if !(*(*xml).attr.offset(i as isize)).is_null() {
        return *(*xml).attr.offset((i + 1 as ::core::ffi::c_int) as isize);
    }
    while !(*root).xml.parent.is_null() {
        root = (*root).xml.parent as ezxml_root_t;
    }
    i = 0 as ::core::ffi::c_int;
    while !(*(*root).attr.offset(i as isize)).is_null()
        && strcmp(
            (*xml).name,
            *(*(*root).attr.offset(i as isize)).offset(0isize),
        ) != 0
    {
        i += 1;
    }
    if (*(*root).attr.offset(i as isize)).is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    while !(*(*(*root).attr.offset(i as isize)).offset(j as isize)).is_null()
        && strcmp(attr, *(*(*root).attr.offset(i as isize)).offset(j as isize)) != 0
    {
        j += 3 as ::core::ffi::c_int;
    }
    return if !(*(*(*root).attr.offset(i as isize)).offset(j as isize)).is_null() {
        *(*(*root).attr.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_vget(mut xml: ezxml_t, mut ap: ::core::ffi::VaList) -> ezxml_t {
    let mut name: *mut ::core::ffi::c_char = ap.arg::<*mut ::core::ffi::c_char>();
    let mut idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if !name.is_null() && *name as ::core::ffi::c_int != 0 {
        idx = ap.arg::<::core::ffi::c_int>();
        xml = ezxml_child(xml, name);
    }
    return if idx < 0 as ::core::ffi::c_int {
        xml
    } else {
        ezxml_vget(ezxml_idx(xml, idx), ap.clone())
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_get(mut xml: ezxml_t, mut c2rust_args: ...) -> ezxml_t {
    let mut ap: ::core::ffi::VaList;
    let mut r: ezxml_t = ::core::ptr::null_mut::<ezxml>();
    ap = c2rust_args.clone();
    r = ezxml_vget(xml, ap.clone());
    return r;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_pi(
    mut xml: ezxml_t,
    mut target: *const ::core::ffi::c_char,
) -> *mut *const ::core::ffi::c_char {
    let mut root: ezxml_root_t = xml as ezxml_root_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if root.is_null() {
        return &raw mut EZXML_NIL as *mut *mut ::core::ffi::c_char
            as *mut *const ::core::ffi::c_char;
    }
    while !(*root).xml.parent.is_null() {
        root = (*root).xml.parent as ezxml_root_t;
    }
    while !(*(*root).pi.offset(i as isize)).is_null()
        && strcmp(target, *(*(*root).pi.offset(i as isize)).offset(0isize)) != 0
    {
        i += 1;
    }
    return (if !(*(*root).pi.offset(i as isize)).is_null() {
        (*(*root).pi.offset(i as isize)).offset(1 as ::core::ffi::c_int as isize)
    } else {
        &raw mut EZXML_NIL as *mut *mut ::core::ffi::c_char
    }) as *mut *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_err(
    mut root: ezxml_root_t,
    mut s: *mut ::core::ffi::c_char,
    mut err: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> ezxml_t {
    let mut ap: ::core::ffi::VaList;
    let mut line: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fmt: [::core::ffi::c_char; 128] = [0; 128];
    t = (*root).s;
    while t < s {
        if *t as ::core::ffi::c_int == '\n' as ::core::ffi::c_int {
            line += 1;
        }
        t = t.offset(1);
    }
    snprintf(
        &raw mut fmt as *mut ::core::ffi::c_char,
        EZXML_ERRL as size_t,
        b"[error near line %d]: %s\0".as_ptr() as *const ::core::ffi::c_char,
        line,
        err,
    );
    ap = c2rust_args.clone();
    vsnprintf(
        &raw mut (*root).err as *mut ::core::ffi::c_char,
        EZXML_ERRL as size_t,
        &raw mut fmt as *mut ::core::ffi::c_char,
        ap.clone(),
    );
    return &raw mut (*root).xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_decode(
    mut s: *mut ::core::ffi::c_char,
    mut ent: *mut *mut ::core::ffi::c_char,
    mut t: ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut e: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = s;
    let mut m: *mut ::core::ffi::c_char = s;
    let mut b: ::core::ffi::c_long = 0;
    let mut c: ::core::ffi::c_long = 0;
    let mut d: ::core::ffi::c_long = 0;
    let mut l: ::core::ffi::c_long = 0;
    while *s != 0 {
        while *s as ::core::ffi::c_int == '\r' as ::core::ffi::c_int {
            let c2rust_fresh5 = s;
            s = s.offset(1);
            *c2rust_fresh5 = '\n' as ::core::ffi::c_char;
            if *s as ::core::ffi::c_int == '\n' as ::core::ffi::c_int {
                memmove(
                    s as *mut ::core::ffi::c_void,
                    s.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                    strlen(s),
                );
            }
        }
        s = s.offset(1);
    }
    s = r;
    loop {
        while *s as ::core::ffi::c_int != 0
            && *s as ::core::ffi::c_int != '&' as ::core::ffi::c_int
            && (*s as ::core::ffi::c_int != '%' as ::core::ffi::c_int
                || t as ::core::ffi::c_int != '%' as ::core::ffi::c_int)
            && *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
        {
            s = s.offset(1);
        }
        if *s == 0 {
            break;
        }
        if t as ::core::ffi::c_int != 'c' as ::core::ffi::c_int
            && strncmp(
                s,
                b"&#\0".as_ptr() as *const ::core::ffi::c_char,
                2 as size_t,
            ) == 0
        {
            if *s.offset(2isize) as ::core::ffi::c_int == 'x' as ::core::ffi::c_int {
                c = strtol(
                    s.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut e,
                    16 as ::core::ffi::c_int,
                );
            } else {
                c = strtol(
                    s.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut e,
                    10 as ::core::ffi::c_int,
                );
            }
            if c == 0 || *e as ::core::ffi::c_int != ';' as ::core::ffi::c_int {
                s = s.offset(1);
            } else {
                if c < 0x80 as ::core::ffi::c_long {
                    let c2rust_fresh6 = s;
                    s = s.offset(1);
                    *c2rust_fresh6 = c as ::core::ffi::c_char;
                } else {
                    b = 0 as ::core::ffi::c_long;
                    d = c;
                    while d != 0 {
                        b += 1;
                        d /= 2 as ::core::ffi::c_long;
                    }
                    b = (b - 2 as ::core::ffi::c_long) / 5 as ::core::ffi::c_long;
                    let c2rust_fresh7 = s;
                    s = s.offset(1);
                    *c2rust_fresh7 = (((0xff as ::core::ffi::c_int) << 7 as ::core::ffi::c_long - b)
                        as ::core::ffi::c_long
                        | c >> 6 as ::core::ffi::c_long * b)
                        as ::core::ffi::c_char;
                    while b != 0 {
                        b -= 1;
                        let c2rust_fresh8 = s;
                        s = s.offset(1);
                        *c2rust_fresh8 = (0x80 as ::core::ffi::c_long
                            | c >> 6 as ::core::ffi::c_long * b & 0x3f as ::core::ffi::c_long)
                            as ::core::ffi::c_char;
                    }
                }
                memmove(
                    s as *mut ::core::ffi::c_void,
                    strchr(s, ';' as ::core::ffi::c_int).offset(1 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    strlen(strchr(s, ';' as ::core::ffi::c_int)),
                );
            }
        } else if *s as ::core::ffi::c_int == '&' as ::core::ffi::c_int
            && (t as ::core::ffi::c_int == '&' as ::core::ffi::c_int
                || t as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || t as ::core::ffi::c_int == '*' as ::core::ffi::c_int)
            || *s as ::core::ffi::c_int == '%' as ::core::ffi::c_int
                && t as ::core::ffi::c_int == '%' as ::core::ffi::c_int
        {
            b = 0 as ::core::ffi::c_long;
            while !(*ent.offset(b as isize)).is_null()
                && strncmp(
                    s.offset(1 as ::core::ffi::c_int as isize),
                    *ent.offset(b as isize),
                    strlen(*ent.offset(b as isize)),
                ) != 0
            {
                b += 2 as ::core::ffi::c_long;
            }
            let c2rust_fresh9 = b;
            b += 1;
            if !(*ent.offset(c2rust_fresh9 as isize)).is_null() {
                c = strlen(*ent.offset(b as isize)) as ::core::ffi::c_long;
                e = strchr(s, ';' as ::core::ffi::c_int);
                if c as isize - 1isize > e.offset_from(s) {
                    d = s.offset_from(r) as ::core::ffi::c_long;
                    l = ((d + c) as size_t).wrapping_add(strlen(e)) as ::core::ffi::c_long;
                    r = (if r == m {
                        strcpy(malloc(l as size_t) as *mut ::core::ffi::c_char, r)
                            as *mut ::core::ffi::c_void
                    } else {
                        realloc(r as *mut ::core::ffi::c_void, l as size_t)
                    }) as *mut ::core::ffi::c_char;
                    s = r.offset(d as isize);
                    e = strchr(s, ';' as ::core::ffi::c_int);
                }
                memmove(
                    s.offset(c as isize) as *mut ::core::ffi::c_void,
                    e.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                    strlen(e),
                );
                strncpy(s, *ent.offset(b as isize), c as size_t);
            } else {
                s = s.offset(1);
            }
        } else if (t as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || t as ::core::ffi::c_int == '*' as ::core::ffi::c_int)
            && *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            let c2rust_fresh10 = s;
            s = s.offset(1);
            *c2rust_fresh10 = ' ' as ::core::ffi::c_char;
        } else {
            s = s.offset(1);
        }
    }
    if t as ::core::ffi::c_int == '*' as ::core::ffi::c_int {
        s = r;
        while *s != 0 {
            l = strspn(s, b" \0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_long;
            if l != 0 {
                memmove(
                    s as *mut ::core::ffi::c_void,
                    s.offset(l as isize) as *const ::core::ffi::c_void,
                    strlen(s.offset(l as isize)).wrapping_add(1 as size_t),
                );
            }
            while *s as ::core::ffi::c_int != 0
                && *s as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
            {
                s = s.offset(1);
            }
            s = s.offset(1);
        }
        s = s.offset(-1);
        if s >= r && *s as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            *s = '\0' as ::core::ffi::c_char;
        }
    }
    return r;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_open_tag(
    mut root: ezxml_root_t,
    mut name: *mut ::core::ffi::c_char,
    mut attr: *mut *mut ::core::ffi::c_char,
) {
    let mut xml: ezxml_t = (*root).cur;
    if !(*xml).name.is_null() {
        xml = ezxml_add_child(xml, name, strlen((*xml).txt));
    } else {
        (*xml).name = name;
    }
    (*xml).attr = attr;
    (*root).cur = xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_char_content(
    mut root: ezxml_root_t,
    mut s: *mut ::core::ffi::c_char,
    mut len: size_t,
    mut t: ::core::ffi::c_char,
) {
    let mut xml: ezxml_t = (*root).cur;
    let mut m: *mut ::core::ffi::c_char = s;
    let mut l: size_t = 0;
    if xml.is_null() || (*xml).name.is_null() || len == 0 {
        return;
    }
    *s.offset(len as isize) = '\0' as ::core::ffi::c_char;
    s = ezxml_decode(s, (*root).ent, t);
    len = strlen(s).wrapping_add(1 as size_t);
    if *(*xml).txt == 0 {
        (*xml).txt = s;
    } else {
        (*xml).txt = (if (*xml).flags as ::core::ffi::c_int & EZXML_TXTM != 0 {
            l = strlen((*xml).txt);
            realloc((*xml).txt as *mut ::core::ffi::c_void, l.wrapping_add(len))
        } else {
            l = strlen((*xml).txt);
            strcpy(
                malloc(l.wrapping_add(len)) as *mut ::core::ffi::c_char,
                (*xml).txt,
            ) as *mut ::core::ffi::c_void
        }) as *mut ::core::ffi::c_char;
        strcpy((*xml).txt.offset(l as isize), s);
        if s != m {
            free(s as *mut ::core::ffi::c_void);
        }
    }
    if (*xml).txt != m {
        ezxml_set_flag(xml, EZXML_TXTM as ::core::ffi::c_short);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_close_tag(
    mut root: ezxml_root_t,
    mut name: *mut ::core::ffi::c_char,
    mut s: *mut ::core::ffi::c_char,
) -> ezxml_t {
    if (*root).cur.is_null()
        || (*(*root).cur).name.is_null()
        || strcmp(name, (*(*root).cur).name) != 0
    {
        return ezxml_err(
            root,
            s,
            b"unexpected closing tag </%s>\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
    }
    (*root).cur = (*(*root).cur).parent;
    return ::core::ptr::null_mut::<ezxml>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_ent_ok(
    mut name: *mut ::core::ffi::c_char,
    mut s: *mut ::core::ffi::c_char,
    mut ent: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    loop {
        while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != '&' as ::core::ffi::c_int
        {
            s = s.offset(1);
        }
        if *s == 0 {
            return 1 as ::core::ffi::c_int;
        }
        if strncmp(
            s.offset(1 as ::core::ffi::c_int as isize),
            name,
            strlen(name),
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while !(*ent.offset(i as isize)).is_null()
            && strncmp(
                *ent.offset(i as isize),
                s.offset(1 as ::core::ffi::c_int as isize),
                strlen(*ent.offset(i as isize)),
            ) != 0
        {
            i += 2 as ::core::ffi::c_int;
        }
        if !(*ent.offset(i as isize)).is_null()
            && ezxml_ent_ok(
                name,
                *ent.offset((i + 1 as ::core::ffi::c_int) as isize),
                ent,
            ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        s = s.offset(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_proc_inst(
    mut root: ezxml_root_t,
    mut s: *mut ::core::ffi::c_char,
    mut len: size_t,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut target: *mut ::core::ffi::c_char = s;
    *s.offset(len as isize) = '\0' as ::core::ffi::c_char;
    s = s.offset(strcspn(s, EZXML_WS.as_ptr()) as isize);
    if *s != 0 {
        *s = '\0' as ::core::ffi::c_char;
        s = s.offset(
            strspn(
                s.offset(1 as ::core::ffi::c_int as isize),
                EZXML_WS.as_ptr(),
            )
            .wrapping_add(1 as ::core::ffi::c_ulong) as isize,
        );
    }
    if strcmp(target, b"xml\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        s = strstr(s, b"standalone\0".as_ptr() as *const ::core::ffi::c_char);
        if !s.is_null()
            && strncmp(
                s.offset(strspn(
                    s.offset(10 as ::core::ffi::c_int as isize),
                    b"\t\r\n ='\"\0".as_ptr() as *const ::core::ffi::c_char,
                ) as isize)
                    .offset(10 as ::core::ffi::c_int as isize),
                b"yes\0".as_ptr() as *const ::core::ffi::c_char,
                3 as size_t,
            ) == 0
        {
            (*root).standalone = 1 as ::core::ffi::c_short;
        }
        return;
    }
    if (*(*root).pi.offset(0isize)).is_null() {
        (*root).pi = malloc(::core::mem::size_of::<*mut *mut ::core::ffi::c_char>())
            as *mut *mut *mut ::core::ffi::c_char;
        *(*root).pi = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    while !(*(*root).pi.offset(i as isize)).is_null()
        && strcmp(target, *(*(*root).pi.offset(i as isize)).offset(0isize)) != 0
    {
        i += 1;
    }
    if (*(*root).pi.offset(i as isize)).is_null() {
        (*root).pi = realloc(
            (*root).pi as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<*mut *mut ::core::ffi::c_char>()
                .wrapping_mul((i + 2 as ::core::ffi::c_int) as size_t),
        ) as *mut *mut *mut ::core::ffi::c_char;
        *(*root).pi.offset(i as isize) =
            malloc(::core::mem::size_of::<*mut ::core::ffi::c_char>().wrapping_mul(3 as size_t))
                as *mut *mut ::core::ffi::c_char;
        *(*(*root).pi.offset(i as isize)).offset(0isize) = target;
        *(*root).pi.offset((i + 1 as ::core::ffi::c_int) as isize) =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        *(*(*root).pi.offset(i as isize)).offset(1isize) =
            *(*root).pi.offset((i + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_char;
        *(*(*root).pi.offset(i as isize)).offset(2isize) =
            strdup(b"\0".as_ptr() as *const ::core::ffi::c_char);
    }
    while !(*(*(*root).pi.offset(i as isize)).offset(j as isize)).is_null() {
        j += 1;
    }
    *(*root).pi.offset(i as isize) = realloc(
        *(*root).pi.offset(i as isize) as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>()
            .wrapping_mul((j + 3 as ::core::ffi::c_int) as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    *(*(*root).pi.offset(i as isize)).offset((j + 2 as ::core::ffi::c_int) as isize) = realloc(
        *(*(*root).pi.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_void,
        (j + 1 as ::core::ffi::c_int) as size_t,
    )
        as *mut ::core::ffi::c_char;
    strcpy(
        (*(*(*root).pi.offset(i as isize)).offset((j + 2 as ::core::ffi::c_int) as isize))
            .offset(j as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        if !(*root).xml.name.is_null() {
            b">\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"<\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    *(*(*root).pi.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize) =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    *(*(*root).pi.offset(i as isize)).offset(j as isize) = s;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_internal_dtd(
    mut root: ezxml_root_t,
    mut s: *mut ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_short {
    let mut q: ::core::ffi::c_char = 0;
    let mut c: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ent: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut pe: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut ent_tmp: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    pe = memcpy(
        malloc(::core::mem::size_of::<[*mut ::core::ffi::c_char; 1]>()),
        &raw mut EZXML_NIL as *mut *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[*mut ::core::ffi::c_char; 1]>(),
    ) as *mut *mut ::core::ffi::c_char;
    *s.offset(len as isize) = '\0' as ::core::ffi::c_char;
    while !s.is_null() {
        while *s as ::core::ffi::c_int != 0
            && *s as ::core::ffi::c_int != '<' as ::core::ffi::c_int
            && *s as ::core::ffi::c_int != '%' as ::core::ffi::c_int
        {
            s = s.offset(1);
        }
        if *s == 0 {
            break;
        }
        if strncmp(
            s,
            b"<!ENTITY\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0
        {
            s = s.offset(
                strspn(
                    s.offset(8 as ::core::ffi::c_int as isize),
                    EZXML_WS.as_ptr(),
                )
                .wrapping_add(8 as ::core::ffi::c_ulong) as isize,
            );
            c = s;
            n = s.offset(strspn(s, b"\t\r\n %\0".as_ptr() as *const ::core::ffi::c_char) as isize);
            s = n.offset(strcspn(n, EZXML_WS.as_ptr()) as isize);
            *s = ';' as ::core::ffi::c_char;
            v = s
                .offset(strspn(
                    s.offset(1 as ::core::ffi::c_int as isize),
                    EZXML_WS.as_ptr(),
                ) as isize)
                .offset(1 as ::core::ffi::c_int as isize);
            let c2rust_fresh11 = v;
            v = v.offset(1);
            q = *c2rust_fresh11;
            if q as ::core::ffi::c_int != '"' as ::core::ffi::c_int
                && q as ::core::ffi::c_int != '\'' as ::core::ffi::c_int
            {
                s = strchr(s, '>' as ::core::ffi::c_int);
            } else {
                i = 0 as ::core::ffi::c_int;
                ent = if *c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                    pe
                } else {
                    (*root).ent
                };
                while !(*ent.offset(i as isize)).is_null() {
                    i += 1;
                }
                ent_tmp = realloc(
                    ent as *mut ::core::ffi::c_void,
                    ((i + 3 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
                ) as *mut *mut ::core::ffi::c_char;
                if ent_tmp.is_null() {
                    free(ent as *mut ::core::ffi::c_void);
                    break;
                } else {
                    ent = ent_tmp;
                    if *c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                        pe = ent;
                    } else {
                        (*root).ent = ent;
                    }
                    s = s.offset(1);
                    *s = '\0' as ::core::ffi::c_char;
                    s = strchr(v, q as ::core::ffi::c_int);
                    if !s.is_null() {
                        let c2rust_fresh12 = s;
                        s = s.offset(1);
                        *c2rust_fresh12 = '\0' as ::core::ffi::c_char;
                    }
                    *ent.offset((i + 1 as ::core::ffi::c_int) as isize) =
                        ezxml_decode(v, pe, '%' as ::core::ffi::c_char);
                    *ent.offset((i + 2 as ::core::ffi::c_int) as isize) =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    if ezxml_ent_ok(n, *ent.offset((i + 1 as ::core::ffi::c_int) as isize), ent)
                        == 0
                    {
                        if *ent.offset((i + 1 as ::core::ffi::c_int) as isize) != v {
                            free(*ent.offset((i + 1 as ::core::ffi::c_int) as isize)
                                as *mut ::core::ffi::c_void);
                        }
                        ezxml_err(
                            root,
                            v,
                            b"circular entity declaration &%s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            n,
                        );
                        break;
                    } else {
                        *ent.offset(i as isize) = n;
                    }
                }
            }
        } else if strncmp(
            s,
            b"<!ATTLIST\0".as_ptr() as *const ::core::ffi::c_char,
            9 as size_t,
        ) == 0
        {
            t = s
                .offset(strspn(
                    s.offset(9 as ::core::ffi::c_int as isize),
                    EZXML_WS.as_ptr(),
                ) as isize)
                .offset(9 as ::core::ffi::c_int as isize);
            if *t == 0 {
                ezxml_err(
                    root,
                    t,
                    b"unclosed <!ATTLIST\0".as_ptr() as *const ::core::ffi::c_char,
                );
                break;
            } else {
                s = t.offset(
                    strcspn(t, b"\t\r\n >\0".as_ptr() as *const ::core::ffi::c_char) as isize,
                );
                if *s as ::core::ffi::c_int == '>' as ::core::ffi::c_int {
                    continue;
                }
                *s = '\0' as ::core::ffi::c_char;
                i = 0 as ::core::ffi::c_int;
                while !(*(*root).attr.offset(i as isize)).is_null()
                    && strcmp(n, *(*(*root).attr.offset(i as isize)).offset(0isize)) != 0
                {
                    i += 1;
                }
                loop {
                    s = s.offset(1);
                    n = s.offset(strspn(s, EZXML_WS.as_ptr()) as isize);
                    if !(*n as ::core::ffi::c_int != 0
                        && *n as ::core::ffi::c_int != '>' as ::core::ffi::c_int)
                    {
                        break;
                    }
                    s = n.offset(strcspn(n, EZXML_WS.as_ptr()) as isize);
                    if *s != 0 {
                        *s = '\0' as ::core::ffi::c_char;
                        s = s.offset(
                            strspn(
                                s.offset(1 as ::core::ffi::c_int as isize),
                                EZXML_WS.as_ptr(),
                            )
                            .wrapping_add(1 as ::core::ffi::c_ulong)
                                as isize,
                        );
                        c = (if strncmp(
                            s,
                            b"CDATA\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as size_t,
                        ) != 0
                        {
                            b"*\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b" \0".as_ptr() as *const ::core::ffi::c_char
                        }) as *mut ::core::ffi::c_char;
                        if strncmp(
                            s,
                            b"NOTATION\0".as_ptr() as *const ::core::ffi::c_char,
                            8 as size_t,
                        ) == 0
                        {
                            s = s.offset(
                                strspn(
                                    s.offset(8 as ::core::ffi::c_int as isize),
                                    EZXML_WS.as_ptr(),
                                )
                                .wrapping_add(8 as ::core::ffi::c_ulong)
                                    as isize,
                            );
                        }
                        s = if *s as ::core::ffi::c_int == '(' as ::core::ffi::c_int {
                            strchr(s, ')' as ::core::ffi::c_int)
                        } else {
                            s.offset(strcspn(s, EZXML_WS.as_ptr()) as isize)
                        };
                        if s.is_null() {
                            ezxml_err(
                                root,
                                t,
                                b"malformed <!ATTLIST\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            break;
                        } else {
                            s = s.offset(strspn(
                                s,
                                b"\t\r\n )\0".as_ptr() as *const ::core::ffi::c_char,
                            ) as isize);
                            if strncmp(
                                s,
                                b"#FIXED\0".as_ptr() as *const ::core::ffi::c_char,
                                6 as size_t,
                            ) == 0
                            {
                                s = s.offset(
                                    strspn(
                                        s.offset(6 as ::core::ffi::c_int as isize),
                                        EZXML_WS.as_ptr(),
                                    )
                                    .wrapping_add(6 as ::core::ffi::c_ulong)
                                        as isize,
                                );
                            }
                            if *s as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
                                s = s.offset(
                                    strcspn(s, b"\t\r\n >\0".as_ptr() as *const ::core::ffi::c_char)
                                        .wrapping_sub(1 as ::core::ffi::c_ulong)
                                        as isize,
                                );
                                if *c as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
                                    continue;
                                }
                                v = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            } else if (*s as ::core::ffi::c_int == '"' as ::core::ffi::c_int
                                || *s as ::core::ffi::c_int == '\'' as ::core::ffi::c_int)
                                && {
                                    v = s.offset(1 as ::core::ffi::c_int as isize);
                                    s = strchr(v, *s as ::core::ffi::c_int);
                                    !s.is_null()
                                }
                            {
                                *s = '\0' as ::core::ffi::c_char;
                            } else {
                                ezxml_err(
                                    root,
                                    t,
                                    b"malformed <!ATTLIST\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                break;
                            }
                            if (*(*root).attr.offset(i as isize)).is_null() {
                                (*root).attr =
                                    (if i == 0 {
                                        malloc((2 as size_t).wrapping_mul(::core::mem::size_of::<
                                            *mut *mut ::core::ffi::c_char,
                                        >(
                                        )))
                                    } else {
                                        realloc(
                                        (*root).attr as *mut ::core::ffi::c_void,
                                        ((i + 2 as ::core::ffi::c_int) as size_t).wrapping_mul(
                                            ::core::mem::size_of::<*mut *mut ::core::ffi::c_char>(),
                                        ),
                                    )
                                    })
                                        as *mut *mut *mut ::core::ffi::c_char;
                                *(*root).attr.offset(i as isize) = malloc(
                                    (2 as size_t)
                                        .wrapping_mul(::core::mem::size_of::<
                                            *mut ::core::ffi::c_char,
                                        >()),
                                )
                                    as *mut *mut ::core::ffi::c_char;
                                *(*(*root).attr.offset(i as isize)).offset(0isize) = t;
                                *(*root).attr.offset((i + 1 as ::core::ffi::c_int) as isize) =
                                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
                                *(*(*root).attr.offset(i as isize)).offset(1isize) =
                                    *(*root).attr.offset((i + 1 as ::core::ffi::c_int) as isize)
                                        as *mut ::core::ffi::c_char;
                            }
                            j = 1 as ::core::ffi::c_int;
                            while !(*(*(*root).attr.offset(i as isize)).offset(j as isize))
                                .is_null()
                            {
                                j += 3 as ::core::ffi::c_int;
                            }
                            *(*root).attr.offset(i as isize) = realloc(
                                *(*root).attr.offset(i as isize) as *mut ::core::ffi::c_void,
                                ((j + 4 as ::core::ffi::c_int) as size_t).wrapping_mul(
                                    ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
                                ),
                            )
                                as *mut *mut ::core::ffi::c_char;
                            *(*(*root).attr.offset(i as isize))
                                .offset((j + 3 as ::core::ffi::c_int) as isize) =
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                            *(*(*root).attr.offset(i as isize))
                                .offset((j + 2 as ::core::ffi::c_int) as isize) = c;
                            *(*(*root).attr.offset(i as isize))
                                .offset((j + 1 as ::core::ffi::c_int) as isize) = if !v.is_null() {
                                ezxml_decode(v, (*root).ent, *c)
                            } else {
                                ::core::ptr::null_mut::<::core::ffi::c_char>()
                            };
                            *(*(*root).attr.offset(i as isize)).offset(j as isize) = n;
                        }
                    } else {
                        ezxml_err(
                            root,
                            t,
                            b"malformed <!ATTLIST\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        break;
                    }
                }
            }
        } else if strncmp(
            s,
            b"<!--\0".as_ptr() as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0
        {
            s = strstr(
                s.offset(4 as ::core::ffi::c_int as isize),
                b"-->\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if strncmp(
            s,
            b"<?\0".as_ptr() as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0
        {
            c = s.offset(2 as ::core::ffi::c_int as isize);
            s = strstr(c, b"?>\0".as_ptr() as *const ::core::ffi::c_char);
            if !s.is_null() {
                let c2rust_fresh13 = s;
                s = s.offset(1);
                ezxml_proc_inst(root, c, c2rust_fresh13.offset_from(c) as size_t);
            }
        } else if *s as ::core::ffi::c_int == '<' as ::core::ffi::c_int {
            s = strchr(s, '>' as ::core::ffi::c_int);
        } else {
            let c2rust_fresh14 = s;
            s = s.offset(1);
            if *c2rust_fresh14 as ::core::ffi::c_int == '%' as ::core::ffi::c_int
                && (*root).standalone == 0
            {
                break;
            }
        }
    }
    free(pe as *mut ::core::ffi::c_void);
    return (*(&raw mut (*root).err as *mut ::core::ffi::c_char) == 0) as ::core::ffi::c_int
        as ::core::ffi::c_short;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_str2utf8(
    mut s: *mut *mut ::core::ffi::c_char,
    mut len: *mut size_t,
) -> *mut ::core::ffi::c_char {
    let mut u: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut l: size_t = 0 as size_t;
    let mut sl: size_t = 0;
    let mut max: size_t = *len;
    let mut c: ::core::ffi::c_long = 0;
    let mut d: ::core::ffi::c_long = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut be: ::core::ffi::c_int = if **s as ::core::ffi::c_int == -2 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else if **s as ::core::ffi::c_int == -1 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
    if be == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    u = malloc(max) as *mut ::core::ffi::c_char;
    sl = 2 as size_t;
    while sl < (*len).wrapping_sub(1 as size_t) {
        c = (if be != 0 {
            (*(*s).offset(sl as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(*s).offset(sl.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int
        } else {
            (*(*s).offset(sl.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(*s).offset(sl as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        }) as ::core::ffi::c_long;
        if c >= 0xd800 as ::core::ffi::c_long && c <= 0xdfff as ::core::ffi::c_long && {
            sl = sl.wrapping_add(2 as size_t);
            sl < (*len).wrapping_sub(1 as size_t)
        } {
            d = (if be != 0 {
                (*(*s).offset(sl as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(*s).offset(sl.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int
            } else {
                (*(*s).offset(sl.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(*s).offset(sl as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
            }) as ::core::ffi::c_long;
            c = ((c & 0x3ff as ::core::ffi::c_long) << 10 as ::core::ffi::c_int
                | d & 0x3ff as ::core::ffi::c_long)
                + 0x10000 as ::core::ffi::c_long;
        }
        while l.wrapping_add(6 as size_t) > max {
            max = max.wrapping_add(EZXML_BUFSIZE as size_t);
            let mut utmp: *mut ::core::ffi::c_char =
                realloc(u as *mut ::core::ffi::c_void, max) as *mut ::core::ffi::c_char;
            if utmp.is_null() {
                free(u as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            u = utmp;
        }
        if c < 0x80 as ::core::ffi::c_long {
            let c2rust_fresh15 = l;
            l = l.wrapping_add(1);
            *u.offset(c2rust_fresh15 as isize) = c as ::core::ffi::c_char;
        } else {
            b = 0 as ::core::ffi::c_int;
            d = c;
            while d != 0 {
                b += 1;
                d /= 2 as ::core::ffi::c_long;
            }
            b = (b - 2 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int;
            let c2rust_fresh16 = l;
            l = l.wrapping_add(1);
            *u.offset(c2rust_fresh16 as isize) =
                (((0xff as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - b)
                    as ::core::ffi::c_long
                    | c >> 6 as ::core::ffi::c_int * b) as ::core::ffi::c_char;
            while b != 0 {
                b -= 1;
                let c2rust_fresh17 = l;
                l = l.wrapping_add(1);
                *u.offset(c2rust_fresh17 as isize) = (0x80 as ::core::ffi::c_long
                    | c >> 6 as ::core::ffi::c_int * b & 0x3f as ::core::ffi::c_long)
                    as ::core::ffi::c_char;
            }
        }
        sl = sl.wrapping_add(2 as size_t);
    }
    *len = l;
    *s = realloc(u as *mut ::core::ffi::c_void, *len) as *mut ::core::ffi::c_char;
    return *s;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_free_attr(mut attr: *mut *mut ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut m: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if attr.is_null() || attr == &raw mut EZXML_NIL as *mut *mut ::core::ffi::c_char {
        return;
    }
    while !(*attr.offset(i as isize)).is_null() {
        i += 2 as ::core::ffi::c_int;
    }
    m = *attr.offset((i + 1 as ::core::ffi::c_int) as isize);
    i = 0 as ::core::ffi::c_int;
    while *m.offset(i as isize) != 0 {
        if *m.offset(i as isize) as ::core::ffi::c_int & EZXML_NAMEM != 0 {
            free(*attr.offset((i * 2 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_void);
        }
        if *m.offset(i as isize) as ::core::ffi::c_int & EZXML_TXTM != 0 {
            free(
                *attr.offset((i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_void,
            );
        }
        i += 1;
    }
    free(m as *mut ::core::ffi::c_void);
    free(attr as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_parse_str(
    mut s: *mut ::core::ffi::c_char,
    mut len: size_t,
) -> ezxml_t {
    let mut root: ezxml_root_t =
        ezxml_new(::core::ptr::null::<::core::ffi::c_char>()) as ezxml_root_t;
    let mut q: ::core::ffi::c_char = 0;
    let mut e: ::core::ffi::c_char = 0;
    let mut d: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut attr: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut a: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut l: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    (*root).m = s;
    if len == 0 {
        return ezxml_err(
            root,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b"root tag missing\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    (*root).u = ezxml_str2utf8(&raw mut s, &raw mut len);
    (*root).s = s;
    (*root).e = (*root).s.offset(len as isize);
    e = *s.offset(len.wrapping_sub(1 as size_t) as isize);
    *s.offset(len.wrapping_sub(1 as size_t) as isize) = '\0' as ::core::ffi::c_char;
    while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != '<' as ::core::ffi::c_int {
        s = s.offset(1);
    }
    if *s == 0 {
        return ezxml_err(
            root,
            s,
            b"root tag missing\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    loop {
        attr = &raw mut EZXML_NIL as *mut *mut ::core::ffi::c_char;
        s = s.offset(1);
        d = s;
        if *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalpha.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            || *s as ::core::ffi::c_int == '_' as ::core::ffi::c_int
            || *s as ::core::ffi::c_int == ':' as ::core::ffi::c_int
            || (*s as ::core::ffi::c_int) < '\0' as ::core::ffi::c_int
        {
            if (*root).cur.is_null() {
                return ezxml_err(
                    root,
                    d,
                    b"markup outside of root element\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            s = s
                .offset(strcspn(s, b"\t\r\n />\0".as_ptr() as *const ::core::ffi::c_char) as isize);
            while *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            {
                let c2rust_fresh0 = s;
                s = s.offset(1);
                *c2rust_fresh0 = '\0' as ::core::ffi::c_char;
            }
            if *s as ::core::ffi::c_int != 0
                && *s as ::core::ffi::c_int != '/' as ::core::ffi::c_int
                && *s as ::core::ffi::c_int != '>' as ::core::ffi::c_int
            {
                i = 0 as ::core::ffi::c_int;
                loop {
                    a = *(*root).attr.offset(i as isize);
                    if !(!a.is_null() && strcmp(*a.offset(0isize), d) != 0) {
                        break;
                    }
                    i += 1;
                }
            }
            l = 0 as ::core::ffi::c_int;
            while *s as ::core::ffi::c_int != 0
                && *s as ::core::ffi::c_int != '/' as ::core::ffi::c_int
                && *s as ::core::ffi::c_int != '>' as ::core::ffi::c_int
            {
                attr = (if l != 0 {
                    realloc(
                        attr as *mut ::core::ffi::c_void,
                        ((l + 4 as ::core::ffi::c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
                    )
                } else {
                    malloc(
                        (4 as size_t)
                            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
                    )
                }) as *mut *mut ::core::ffi::c_char;
                *attr.offset((l + 3 as ::core::ffi::c_int) as isize) = (if l != 0 {
                    realloc(
                        *attr.offset((l + 1 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_void,
                        (l / 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as size_t,
                    )
                } else {
                    malloc(2 as size_t)
                })
                    as *mut ::core::ffi::c_char;
                strcpy(
                    (*attr.offset((l + 3 as ::core::ffi::c_int) as isize))
                        .offset((l / 2 as ::core::ffi::c_int) as isize),
                    b" \0".as_ptr() as *const ::core::ffi::c_char,
                );
                *attr.offset((l + 2 as ::core::ffi::c_int) as isize) =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                *attr.offset((l + 1 as ::core::ffi::c_int) as isize) =
                    b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                *attr.offset(l as isize) = s;
                s = s.offset(
                    strcspn(s, b"\t\r\n =/>\0".as_ptr() as *const ::core::ffi::c_char) as isize,
                );
                if *s as ::core::ffi::c_int == '=' as ::core::ffi::c_int
                    || *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        != 0
                {
                    let c2rust_fresh1 = s;
                    s = s.offset(1);
                    *c2rust_fresh1 = '\0' as ::core::ffi::c_char;
                    s = s.offset(
                        strspn(s, b"\t\r\n =\0".as_ptr() as *const ::core::ffi::c_char) as isize,
                    );
                    q = *s;
                    if q as ::core::ffi::c_int == '"' as ::core::ffi::c_int
                        || q as ::core::ffi::c_int == '\'' as ::core::ffi::c_int
                    {
                        s = s.offset(1);
                        *attr.offset((l + 1 as ::core::ffi::c_int) as isize) = s;
                        while *s as ::core::ffi::c_int != 0
                            && *s as ::core::ffi::c_int != q as ::core::ffi::c_int
                        {
                            s = s.offset(1);
                        }
                        if *s != 0 {
                            let c2rust_fresh2 = s;
                            s = s.offset(1);
                            *c2rust_fresh2 = '\0' as ::core::ffi::c_char;
                        } else {
                            ezxml_free_attr(attr);
                            return ezxml_err(
                                root,
                                d,
                                b"missing %c\0".as_ptr() as *const ::core::ffi::c_char,
                                q as ::core::ffi::c_int,
                            );
                        }
                        j = 1 as ::core::ffi::c_int;
                        while !a.is_null()
                            && !(*a.offset(j as isize)).is_null()
                            && strcmp(*a.offset(j as isize), *attr.offset(l as isize)) != 0
                        {
                            j += 3 as ::core::ffi::c_int;
                        }
                        *attr.offset((l + 1 as ::core::ffi::c_int) as isize) = ezxml_decode(
                            *attr.offset((l + 1 as ::core::ffi::c_int) as isize),
                            (*root).ent,
                            (if !a.is_null() && !(*a.offset(j as isize)).is_null() {
                                **a.offset((j + 2 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                            } else {
                                ' ' as ::core::ffi::c_int
                            }) as ::core::ffi::c_char,
                        );
                        if *attr.offset((l + 1 as ::core::ffi::c_int) as isize) < d
                            || *attr.offset((l + 1 as ::core::ffi::c_int) as isize) > s
                        {
                            *(*attr.offset((l + 3 as ::core::ffi::c_int) as isize))
                                .offset((l / 2 as ::core::ffi::c_int) as isize) =
                                EZXML_TXTM as ::core::ffi::c_char;
                        }
                    }
                }
                while *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    != 0
                {
                    s = s.offset(1);
                }
                l += 2 as ::core::ffi::c_int;
            }
            if *s as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                let c2rust_fresh3 = s;
                s = s.offset(1);
                *c2rust_fresh3 = '\0' as ::core::ffi::c_char;
                if *s as ::core::ffi::c_int != 0
                    && *s as ::core::ffi::c_int != '>' as ::core::ffi::c_int
                    || *s == 0 && e as ::core::ffi::c_int != '>' as ::core::ffi::c_int
                {
                    if l != 0 {
                        ezxml_free_attr(attr);
                    }
                    return ezxml_err(
                        root,
                        d,
                        b"missing >\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                ezxml_open_tag(root, d, attr);
                ezxml_close_tag(root, d, s);
            } else {
                q = *s;
                if q as ::core::ffi::c_int == '>' as ::core::ffi::c_int
                    || *s == 0 && e as ::core::ffi::c_int == '>' as ::core::ffi::c_int
                {
                    *s = '\0' as ::core::ffi::c_char;
                    ezxml_open_tag(root, d, attr);
                    *s = q;
                } else {
                    if l != 0 {
                        ezxml_free_attr(attr);
                    }
                    return ezxml_err(
                        root,
                        d,
                        b"missing >\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
        } else if *s as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            d = s.offset(1 as ::core::ffi::c_int as isize);
            s = s.offset(
                strcspn(d, b"\t\r\n >\0".as_ptr() as *const ::core::ffi::c_char)
                    .wrapping_add(1 as ::core::ffi::c_ulong) as isize,
            );
            q = *s;
            if q == 0 && e as ::core::ffi::c_int != '>' as ::core::ffi::c_int {
                return ezxml_err(
                    root,
                    d,
                    b"missing >\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            *s = '\0' as ::core::ffi::c_char;
            if !ezxml_close_tag(root, d, s).is_null() {
                return &raw mut (*root).xml;
            }
            *s = q;
            if *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            {
                s = s.offset(strspn(s, EZXML_WS.as_ptr()) as isize);
            }
        } else if strncmp(
            s,
            b"!--\0".as_ptr() as *const ::core::ffi::c_char,
            3 as size_t,
        ) == 0
        {
            s = strstr(
                s.offset(3 as ::core::ffi::c_int as isize),
                b"--\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if s.is_null()
                || {
                    s = s.offset(2 as ::core::ffi::c_int as isize);
                    *s as ::core::ffi::c_int != '>' as ::core::ffi::c_int
                        && *s as ::core::ffi::c_int != 0
                }
                || *s == 0 && e as ::core::ffi::c_int != '>' as ::core::ffi::c_int
            {
                return ezxml_err(
                    root,
                    d,
                    b"unclosed <!--\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        } else if strncmp(
            s,
            b"![CDATA[\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0
        {
            s = strstr(s, b"]]>\0".as_ptr() as *const ::core::ffi::c_char);
            if !s.is_null() {
                s = s.offset(2 as ::core::ffi::c_int as isize);
                ezxml_char_content(
                    root,
                    d.offset(8 as ::core::ffi::c_int as isize),
                    (s.offset_from(d) - 10isize) as size_t,
                    'c' as ::core::ffi::c_char,
                );
            } else {
                return ezxml_err(
                    root,
                    d,
                    b"unclosed <![CDATA[\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        } else if strncmp(
            s,
            b"!DOCTYPE\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0
        {
            l = 0 as ::core::ffi::c_int;
            while *s as ::core::ffi::c_int != 0
                && (l == 0 && *s as ::core::ffi::c_int != '>' as ::core::ffi::c_int
                    || l != 0
                        && (*s as ::core::ffi::c_int != ']' as ::core::ffi::c_int
                            || *s
                                .offset(strspn(
                                    s.offset(1 as ::core::ffi::c_int as isize),
                                    EZXML_WS.as_ptr(),
                                ) as isize)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                != '>' as ::core::ffi::c_int))
            {
                s = s.offset(
                    strcspn(
                        s.offset(1 as ::core::ffi::c_int as isize),
                        b"[]>\0".as_ptr() as *const ::core::ffi::c_char,
                    )
                    .wrapping_add(1 as ::core::ffi::c_ulong) as isize,
                );
                l = if *s as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
                    1 as ::core::ffi::c_int
                } else {
                    l
                };
            }
            if *s == 0 && e as ::core::ffi::c_int != '>' as ::core::ffi::c_int {
                return ezxml_err(
                    root,
                    d,
                    b"unclosed <!DOCTYPE\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            d = if l != 0 {
                strchr(d, '[' as ::core::ffi::c_int).offset(1 as ::core::ffi::c_int as isize)
            } else {
                d
            };
            if l != 0 && {
                let c2rust_fresh4 = s;
                s = s.offset(1);
                ezxml_internal_dtd(root, d, c2rust_fresh4.offset_from(d) as size_t) == 0
            } {
                return &raw mut (*root).xml;
            }
        } else if *s as ::core::ffi::c_int == '?' as ::core::ffi::c_int {
            loop {
                s = strchr(s, '?' as ::core::ffi::c_int);
                if !(!s.is_null()
                    && {
                        s = s.offset(1);
                        *s as ::core::ffi::c_int != 0
                    }
                    && *s as ::core::ffi::c_int != '>' as ::core::ffi::c_int)
                {
                    break;
                }
            }
            if s.is_null() || *s == 0 && e as ::core::ffi::c_int != '>' as ::core::ffi::c_int {
                return ezxml_err(
                    root,
                    d,
                    b"unclosed <?\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                ezxml_proc_inst(
                    root,
                    d.offset(1 as ::core::ffi::c_int as isize),
                    (s.offset_from(d) - 2isize) as size_t,
                );
            }
        } else {
            return ezxml_err(
                root,
                d,
                b"unexpected <\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if s.is_null() || *s == 0 {
            break;
        }
        *s = '\0' as ::core::ffi::c_char;
        s = s.offset(1);
        d = s;
        if *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != '<' as ::core::ffi::c_int {
            while *s as ::core::ffi::c_int != 0
                && *s as ::core::ffi::c_int != '<' as ::core::ffi::c_int
            {
                s = s.offset(1);
            }
            if *s == 0 {
                break;
            }
            ezxml_char_content(
                root,
                d,
                s.offset_from(d) as size_t,
                '&' as ::core::ffi::c_char,
            );
        } else if *s == 0 {
            break;
        }
    }
    if (*root).cur.is_null() {
        return &raw mut (*root).xml;
    } else if (*(*root).cur).name.is_null() {
        return ezxml_err(
            root,
            d,
            b"root tag missing\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        return ezxml_err(
            root,
            d,
            b"unclosed tag <%s>\0".as_ptr() as *const ::core::ffi::c_char,
            (*(*root).cur).name,
        );
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_parse_fp(mut fp: *mut FILE) -> ezxml_t {
    let mut root: ezxml_root_t = ::core::ptr::null_mut::<ezxml_root>();
    let mut l: size_t = 0;
    let mut len: size_t = 0 as size_t;
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    s = malloc(EZXML_BUFSIZE as size_t) as *mut ::core::ffi::c_char;
    if s.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    loop {
        l = fread(
            s.offset(len as isize) as *mut ::core::ffi::c_void,
            1 as size_t,
            EZXML_BUFSIZE as size_t,
            fp,
        ) as size_t;
        len = len.wrapping_add(l);
        if l == EZXML_BUFSIZE as size_t {
            let mut stmp: *mut ::core::ffi::c_char = realloc(
                s as *mut ::core::ffi::c_void,
                len.wrapping_add(EZXML_BUFSIZE as size_t),
            ) as *mut ::core::ffi::c_char;
            if stmp.is_null() {
                free(s as *mut ::core::ffi::c_void);
                s = ::core::ptr::null_mut::<::core::ffi::c_char>();
            } else {
                s = stmp;
            }
        }
        if !(!s.is_null() && l == EZXML_BUFSIZE as size_t) {
            break;
        }
    }
    if s.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    root = ezxml_parse_str(s, len) as ezxml_root_t;
    (*root).len = -1 as ::core::ffi::c_int as size_t;
    return &raw mut (*root).xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_parse_fd(mut fd: ::core::ffi::c_int) -> ezxml_t {
    let mut root: ezxml_root_t = ::core::ptr::null_mut::<ezxml_root>();
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
    let mut l: size_t = 0;
    let mut m: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<ezxml>();
    }
    fstat(fd, &raw mut st);
    l = (st.st_size + sysconf(C2Rust_Unnamed_0::_SC_PAGESIZE.0 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_long
        & !(sysconf(C2Rust_Unnamed_0::_SC_PAGESIZE.0 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_long)) as size_t;
    m = mmap(
        NULL,
        l,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE,
        fd,
        0 as __off_t,
    );
    if m != MAP_FAILED {
        madvise(m, l, MADV_SEQUENTIAL);
        root = ezxml_parse_str(m as *mut ::core::ffi::c_char, st.st_size as size_t) as ezxml_root_t;
        (*root).len = l;
        madvise(m, (*root).len, MADV_NORMAL);
    } else {
        m = malloc(st.st_size as size_t);
        l = read(fd, m, st.st_size as size_t) as size_t;
        root = ezxml_parse_str(m as *mut ::core::ffi::c_char, l) as ezxml_root_t;
        (*root).len = -1 as ::core::ffi::c_int as size_t;
    }
    return &raw mut (*root).xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_parse_file(mut file: *const ::core::ffi::c_char) -> ezxml_t {
    let mut fd: ::core::ffi::c_int = open(file, O_RDONLY, 0 as ::core::ffi::c_int);
    let mut xml: ezxml_t = ezxml_parse_fd(fd);
    if fd >= 0 as ::core::ffi::c_int {
        close(fd);
    }
    return xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_ampencode(
    mut s: *const ::core::ffi::c_char,
    mut len: size_t,
    mut dst: *mut *mut ::core::ffi::c_char,
    mut dlen: *mut size_t,
    mut max: *mut size_t,
    mut a: ::core::ffi::c_short,
) -> *mut ::core::ffi::c_char {
    let mut e: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    e = s.offset(len as isize);
    while s != e {
        while (*dlen).wrapping_add(10 as size_t) > *max {
            *max = (*max).wrapping_add(EZXML_BUFSIZE as size_t);
            *dst = realloc(*dst as *mut ::core::ffi::c_void, *max) as *mut ::core::ffi::c_char;
        }
        match *s as ::core::ffi::c_int {
            0 => return *dst,
            38 => {
                *dlen = (*dlen).wrapping_add(sprintf(
                    (*dst).offset(*dlen as isize),
                    b"&amp;\0".as_ptr() as *const ::core::ffi::c_char,
                ) as size_t);
            }
            60 => {
                *dlen = (*dlen).wrapping_add(sprintf(
                    (*dst).offset(*dlen as isize),
                    b"&lt;\0".as_ptr() as *const ::core::ffi::c_char,
                ) as size_t);
            }
            62 => {
                *dlen = (*dlen).wrapping_add(sprintf(
                    (*dst).offset(*dlen as isize),
                    b"&gt;\0".as_ptr() as *const ::core::ffi::c_char,
                ) as size_t);
            }
            34 => {
                *dlen = (*dlen).wrapping_add(sprintf(
                    (*dst).offset(*dlen as isize),
                    if a as ::core::ffi::c_int != 0 {
                        b"&quot;\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                ) as size_t);
            }
            10 => {
                *dlen = (*dlen).wrapping_add(sprintf(
                    (*dst).offset(*dlen as isize),
                    if a as ::core::ffi::c_int != 0 {
                        b"&#xA;\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\n\0".as_ptr() as *const ::core::ffi::c_char
                    },
                ) as size_t);
            }
            9 => {
                *dlen = (*dlen).wrapping_add(sprintf(
                    (*dst).offset(*dlen as isize),
                    if a as ::core::ffi::c_int != 0 {
                        b"&#x9;\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\t\0".as_ptr() as *const ::core::ffi::c_char
                    },
                ) as size_t);
            }
            13 => {
                *dlen = (*dlen).wrapping_add(sprintf(
                    (*dst).offset(*dlen as isize),
                    b"&#xD;\0".as_ptr() as *const ::core::ffi::c_char,
                ) as size_t);
            }
            _ => {
                let c2rust_fresh18 = *dlen;
                *dlen = (*dlen).wrapping_add(1);
                *(*dst).offset(c2rust_fresh18 as isize) = *s;
            }
        }
        s = s.offset(1);
    }
    return *dst;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_toxml_r(
    mut xml: ezxml_t,
    mut s: *mut *mut ::core::ffi::c_char,
    mut len: *mut size_t,
    mut max: *mut size_t,
    mut start: size_t,
    mut attr: *mut *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut txt: *mut ::core::ffi::c_char = (if !(*xml).parent.is_null() {
        (*(*xml).parent).txt
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    }) as *mut ::core::ffi::c_char;
    let mut off: size_t = 0 as size_t;
    *s = ezxml_ampencode(
        txt.offset(start as isize),
        (*xml).off.wrapping_sub(start),
        s,
        len,
        max,
        0 as ::core::ffi::c_short,
    );
    while (*len)
        .wrapping_add(strlen((*xml).name))
        .wrapping_add(4 as size_t)
        > *max
    {
        *max = (*max).wrapping_add(EZXML_BUFSIZE as size_t);
        *s = realloc(*s as *mut ::core::ffi::c_void, *max) as *mut ::core::ffi::c_char;
    }
    *len = (*len).wrapping_add(sprintf(
        (*s).offset(*len as isize),
        b"<%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*xml).name,
    ) as size_t);
    i = 0 as ::core::ffi::c_int;
    while !(*(*xml).attr.offset(i as isize)).is_null() {
        if ezxml_attr(xml, *(*xml).attr.offset(i as isize))
            == *(*xml).attr.offset((i + 1 as ::core::ffi::c_int) as isize)
        {
            while (*len)
                .wrapping_add(strlen(*(*xml).attr.offset(i as isize)))
                .wrapping_add(7 as size_t)
                > *max
            {
                *max = (*max).wrapping_add(EZXML_BUFSIZE as size_t);
                *s = realloc(*s as *mut ::core::ffi::c_void, *max) as *mut ::core::ffi::c_char;
            }
            *len = (*len).wrapping_add(sprintf(
                (*s).offset(*len as isize),
                b" %s=\"\0".as_ptr() as *const ::core::ffi::c_char,
                *(*xml).attr.offset(i as isize),
            ) as size_t);
            ezxml_ampencode(
                *(*xml).attr.offset((i + 1 as ::core::ffi::c_int) as isize),
                -1 as ::core::ffi::c_int as size_t,
                s,
                len,
                max,
                1 as ::core::ffi::c_short,
            );
            *len = (*len).wrapping_add(sprintf(
                (*s).offset(*len as isize),
                b"\"\0".as_ptr() as *const ::core::ffi::c_char,
            ) as size_t);
        }
        i += 2 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while !(*attr.offset(i as isize)).is_null()
        && strcmp(*(*attr.offset(i as isize)).offset(0isize), (*xml).name) != 0
    {
        i += 1;
    }
    j = 1 as ::core::ffi::c_int;
    while !(*attr.offset(i as isize)).is_null()
        && !(*(*attr.offset(i as isize)).offset(j as isize)).is_null()
    {
        if !((*(*attr.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize)).is_null()
            || ezxml_attr(xml, *(*attr.offset(i as isize)).offset(j as isize))
                != *(*attr.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize))
        {
            while (*len)
                .wrapping_add(strlen(*(*attr.offset(i as isize)).offset(j as isize)))
                .wrapping_add(7 as size_t)
                > *max
            {
                *max = (*max).wrapping_add(EZXML_BUFSIZE as size_t);
                *s = realloc(*s as *mut ::core::ffi::c_void, *max) as *mut ::core::ffi::c_char;
            }
            *len = (*len).wrapping_add(sprintf(
                (*s).offset(*len as isize),
                b" %s=\"\0".as_ptr() as *const ::core::ffi::c_char,
                *(*attr.offset(i as isize)).offset(j as isize),
            ) as size_t);
            ezxml_ampencode(
                *(*attr.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize),
                -1 as ::core::ffi::c_int as size_t,
                s,
                len,
                max,
                1 as ::core::ffi::c_short,
            );
            *len = (*len).wrapping_add(sprintf(
                (*s).offset(*len as isize),
                b"\"\0".as_ptr() as *const ::core::ffi::c_char,
            ) as size_t);
        }
        j += 3 as ::core::ffi::c_int;
    }
    *len = (*len).wrapping_add(sprintf(
        (*s).offset(*len as isize),
        b">\0".as_ptr() as *const ::core::ffi::c_char,
    ) as size_t);
    *s = if !(*xml).child.is_null() {
        ezxml_toxml_r((*xml).child, s, len, max, 0 as size_t, attr)
    } else {
        ezxml_ampencode(
            (*xml).txt,
            -1 as ::core::ffi::c_int as size_t,
            s,
            len,
            max,
            0 as ::core::ffi::c_short,
        )
    };
    while (*len)
        .wrapping_add(strlen((*xml).name))
        .wrapping_add(4 as size_t)
        > *max
    {
        *max = (*max).wrapping_add(EZXML_BUFSIZE as size_t);
        *s = realloc(*s as *mut ::core::ffi::c_void, *max) as *mut ::core::ffi::c_char;
    }
    *len = (*len).wrapping_add(sprintf(
        (*s).offset(*len as isize),
        b"</%s>\0".as_ptr() as *const ::core::ffi::c_char,
        (*xml).name,
    ) as size_t);
    while off < (*xml).off && *txt.offset(off as isize) as ::core::ffi::c_int != 0 {
        off = off.wrapping_add(1);
    }
    return if !(*xml).ordered.is_null() {
        ezxml_toxml_r((*xml).ordered, s, len, max, off, attr)
    } else {
        ezxml_ampencode(
            txt.offset(off as isize),
            -1 as ::core::ffi::c_int as size_t,
            s,
            len,
            max,
            0 as ::core::ffi::c_short,
        )
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_toxml(mut xml: ezxml_t) -> *mut ::core::ffi::c_char {
    let mut p: ezxml_t = if !xml.is_null() {
        (*xml).parent
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    let mut o: ezxml_t = if !xml.is_null() {
        (*xml).ordered
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    let mut root: ezxml_root_t = xml as ezxml_root_t;
    let mut len: size_t = 0 as size_t;
    let mut max: size_t = EZXML_BUFSIZE as size_t;
    let mut s: *mut ::core::ffi::c_char = strcpy(
        malloc(max) as *mut ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    if xml.is_null() || (*xml).name.is_null() {
        return realloc(s as *mut ::core::ffi::c_void, len.wrapping_add(1 as size_t))
            as *mut ::core::ffi::c_char;
    }
    while !(*root).xml.parent.is_null() {
        root = (*root).xml.parent as ezxml_root_t;
    }
    i = 0 as ::core::ffi::c_int;
    while p.is_null() && !(*(*root).pi.offset(i as isize)).is_null() {
        k = 2 as ::core::ffi::c_int;
        while !(*(*(*root).pi.offset(i as isize)).offset((k - 1 as ::core::ffi::c_int) as isize))
            .is_null()
        {
            k += 1;
        }
        j = 1 as ::core::ffi::c_int;
        loop {
            n = *(*(*root).pi.offset(i as isize)).offset(j as isize);
            if n.is_null() {
                break;
            }
            if *(*(*(*root).pi.offset(i as isize)).offset(k as isize))
                .offset((j - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != '>' as ::core::ffi::c_int
            {
                t = *(*(*root).pi.offset(i as isize)).offset(0isize);
                while len
                    .wrapping_add(strlen(t))
                    .wrapping_add(strlen(n))
                    .wrapping_add(7 as size_t)
                    > max
                {
                    max = max.wrapping_add(EZXML_BUFSIZE as size_t);
                    let mut stmp: *mut ::core::ffi::c_char =
                        realloc(s as *mut ::core::ffi::c_void, max) as *mut ::core::ffi::c_char;
                    if stmp.is_null() {
                        free(s as *mut ::core::ffi::c_void);
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    s = stmp;
                }
                len = len.wrapping_add(sprintf(
                    s.offset(len as isize),
                    b"<?%s%s%s?>\n\0".as_ptr() as *const ::core::ffi::c_char,
                    t,
                    if *n as ::core::ffi::c_int != 0 {
                        b" \0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    n,
                ) as size_t);
            }
            j += 1;
        }
        i += 1;
    }
    (*xml).ordered = ::core::ptr::null_mut::<ezxml>();
    (*xml).parent = (*xml).ordered;
    s = ezxml_toxml_r(
        xml,
        &raw mut s,
        &raw mut len,
        &raw mut max,
        0 as size_t,
        (*root).attr,
    );
    (*xml).parent = p;
    (*xml).ordered = o;
    i = 0 as ::core::ffi::c_int;
    while p.is_null() && !(*(*root).pi.offset(i as isize)).is_null() {
        k = 2 as ::core::ffi::c_int;
        while !(*(*(*root).pi.offset(i as isize)).offset((k - 1 as ::core::ffi::c_int) as isize))
            .is_null()
        {
            k += 1;
        }
        j = 1 as ::core::ffi::c_int;
        loop {
            n = *(*(*root).pi.offset(i as isize)).offset(j as isize);
            if n.is_null() {
                break;
            }
            if *(*(*(*root).pi.offset(i as isize)).offset(k as isize))
                .offset((j - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != '<' as ::core::ffi::c_int
            {
                t = *(*(*root).pi.offset(i as isize)).offset(0isize);
                while len
                    .wrapping_add(strlen(t))
                    .wrapping_add(strlen(n))
                    .wrapping_add(7 as size_t)
                    > max
                {
                    max = max.wrapping_add(EZXML_BUFSIZE as size_t);
                    let mut stmp_0: *mut ::core::ffi::c_char =
                        realloc(s as *mut ::core::ffi::c_void, max) as *mut ::core::ffi::c_char;
                    if stmp_0.is_null() {
                        free(s as *mut ::core::ffi::c_void);
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    s = stmp_0;
                }
                len = len.wrapping_add(sprintf(
                    s.offset(len as isize),
                    b"\n<?%s%s%s?>\0".as_ptr() as *const ::core::ffi::c_char,
                    t,
                    if *n as ::core::ffi::c_int != 0 {
                        b" \0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    n,
                ) as size_t);
            }
            j += 1;
        }
        i += 1;
    }
    return realloc(s as *mut ::core::ffi::c_void, len.wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_free(mut xml: ezxml_t) {
    let mut root: ezxml_root_t = xml as ezxml_root_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut a: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if xml.is_null() {
        return;
    }
    ezxml_free((*xml).child);
    ezxml_free((*xml).ordered);
    if (*xml).parent.is_null() {
        i = 10 as ::core::ffi::c_int;
        while !(*(*root).ent.offset(i as isize)).is_null() {
            s = *(*root).ent.offset((i + 1 as ::core::ffi::c_int) as isize);
            if s < (*root).s || s > (*root).e {
                free(s as *mut ::core::ffi::c_void);
            }
            i += 2 as ::core::ffi::c_int;
        }
        free((*root).ent as *mut ::core::ffi::c_void);
        i = 0 as ::core::ffi::c_int;
        loop {
            a = *(*root).attr.offset(i as isize);
            if a.is_null() {
                break;
            }
            j = 1 as ::core::ffi::c_int;
            loop {
                let c2rust_fresh19 = j;
                j += 1;
                if (*a.offset(c2rust_fresh19 as isize)).is_null() {
                    break;
                }
                if !(*a.offset(j as isize)).is_null()
                    && (*a.offset(j as isize) < (*root).s || *a.offset(j as isize) > (*root).e)
                {
                    free(*a.offset(j as isize) as *mut ::core::ffi::c_void);
                }
                j += 2 as ::core::ffi::c_int;
            }
            free(a as *mut ::core::ffi::c_void);
            i += 1;
        }
        if !(*(*root).attr.offset(0isize)).is_null() {
            free((*root).attr as *mut ::core::ffi::c_void);
        }
        i = 0 as ::core::ffi::c_int;
        while !(*(*root).pi.offset(i as isize)).is_null() {
            j = 1 as ::core::ffi::c_int;
            while !(*(*(*root).pi.offset(i as isize)).offset(j as isize)).is_null() {
                j += 1;
            }
            free(
                *(*(*root).pi.offset(i as isize)).offset((j + 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_void,
            );
            free(*(*root).pi.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        if !(*(*root).pi.offset(0isize)).is_null() {
            free((*root).pi as *mut ::core::ffi::c_void);
        }
        if (*root).len == -1 as ::core::ffi::c_int as size_t {
            free((*root).m as *mut ::core::ffi::c_void);
        } else if (*root).len != 0 {
            munmap((*root).m as *mut ::core::ffi::c_void, (*root).len);
        }
        if !(*root).u.is_null() {
            free((*root).u as *mut ::core::ffi::c_void);
        }
    }
    ezxml_free_attr((*xml).attr);
    if (*xml).flags as ::core::ffi::c_int & EZXML_TXTM != 0 {
        free((*xml).txt as *mut ::core::ffi::c_void);
    }
    if (*xml).flags as ::core::ffi::c_int & EZXML_NAMEM != 0 {
        free((*xml).name as *mut ::core::ffi::c_void);
    }
    free(xml as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_error(mut xml: ezxml_t) -> *const ::core::ffi::c_char {
    while !xml.is_null() && !(*xml).parent.is_null() {
        xml = (*xml).parent;
    }
    return if !xml.is_null() {
        &raw mut (*(xml as ezxml_root_t)).err as *mut ::core::ffi::c_char
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_new(mut name: *const ::core::ffi::c_char) -> ezxml_t {
    static mut ent: [*mut ::core::ffi::c_char; 11] = [
        b"lt;\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"&#60;\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"gt;\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"&#62;\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"quot;\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"&#34;\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"apos;\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"&#39;\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"amp;\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"&#38;\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ];
    let mut root: ezxml_root_t = memset(
        malloc(::core::mem::size_of::<ezxml_root>()),
        '\0' as ::core::ffi::c_int,
        ::core::mem::size_of::<ezxml_root>(),
    ) as ezxml_root_t;
    (*root).xml.name = name as *mut ::core::ffi::c_char;
    (*root).cur = &raw mut (*root).xml as ezxml_t;
    (*root).xml.txt = b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    strcpy(
        &raw mut (*root).err as *mut ::core::ffi::c_char,
        (*root).xml.txt,
    );
    (*root).ent = memcpy(
        malloc(::core::mem::size_of::<[*mut ::core::ffi::c_char; 11]>()),
        &raw mut ent as *mut *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[*mut ::core::ffi::c_char; 11]>(),
    ) as *mut *mut ::core::ffi::c_char;
    (*root).xml.attr = &raw mut EZXML_NIL as *mut *mut ::core::ffi::c_char;
    (*root).pi = (*root).xml.attr as *mut *mut *mut ::core::ffi::c_char;
    (*root).attr = (*root).pi;
    return &raw mut (*root).xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_insert(
    mut xml: ezxml_t,
    mut dest: ezxml_t,
    mut off: size_t,
) -> ezxml_t {
    let mut cur: ezxml_t = ::core::ptr::null_mut::<ezxml>();
    let mut prev: ezxml_t = ::core::ptr::null_mut::<ezxml>();
    let mut head: ezxml_t = ::core::ptr::null_mut::<ezxml>();
    (*xml).ordered = ::core::ptr::null_mut::<ezxml>();
    (*xml).sibling = (*xml).ordered;
    (*xml).next = (*xml).sibling;
    (*xml).off = off;
    (*xml).parent = dest;
    head = (*dest).child;
    if !head.is_null() {
        if (*head).off <= off {
            cur = head;
            while !(*cur).ordered.is_null() && (*(*cur).ordered).off <= off {
                cur = (*cur).ordered;
            }
            (*xml).ordered = (*cur).ordered;
            (*cur).ordered = xml;
        } else {
            (*xml).ordered = head;
            (*dest).child = xml;
        }
        cur = head;
        prev = ::core::ptr::null_mut::<ezxml>();
        while !cur.is_null() && strcmp((*cur).name, (*xml).name) != 0 {
            prev = cur;
            cur = (*cur).sibling;
        }
        if !cur.is_null() && (*cur).off <= off {
            while !(*cur).next.is_null() && (*(*cur).next).off <= off {
                cur = (*cur).next;
            }
            (*xml).next = (*cur).next;
            (*cur).next = xml;
        } else {
            if !prev.is_null() && !cur.is_null() {
                (*prev).sibling = (*cur).sibling;
            }
            (*xml).next = cur;
            cur = head;
            prev = ::core::ptr::null_mut::<ezxml>();
            while !cur.is_null() && (*cur).off <= off {
                prev = cur;
                cur = (*cur).sibling;
            }
            (*xml).sibling = cur;
            if !prev.is_null() {
                (*prev).sibling = xml;
            }
        }
    } else {
        (*dest).child = xml;
    }
    return xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_add_child(
    mut xml: ezxml_t,
    mut name: *const ::core::ffi::c_char,
    mut off: size_t,
) -> ezxml_t {
    let mut child: ezxml_t = ::core::ptr::null_mut::<ezxml>();
    if xml.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    child = memset(
        malloc(::core::mem::size_of::<ezxml>()),
        '\0' as ::core::ffi::c_int,
        ::core::mem::size_of::<ezxml>(),
    ) as ezxml_t;
    (*child).name = name as *mut ::core::ffi::c_char;
    (*child).attr = &raw mut EZXML_NIL as *mut *mut ::core::ffi::c_char;
    (*child).txt = b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    return ezxml_insert(child, xml, off);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_set_txt(
    mut xml: ezxml_t,
    mut txt: *const ::core::ffi::c_char,
) -> ezxml_t {
    if xml.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    if (*xml).flags as ::core::ffi::c_int & EZXML_TXTM != 0 {
        free((*xml).txt as *mut ::core::ffi::c_void);
    }
    (*xml).flags = ((*xml).flags as ::core::ffi::c_int & !EZXML_TXTM) as ::core::ffi::c_short;
    (*xml).txt = txt as *mut ::core::ffi::c_char;
    return xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_set_attr(
    mut xml: ezxml_t,
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> ezxml_t {
    let mut l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0;
    if xml.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    while !(*(*xml).attr.offset(l as isize)).is_null()
        && strcmp(*(*xml).attr.offset(l as isize), name) != 0
    {
        l += 2 as ::core::ffi::c_int;
    }
    if (*(*xml).attr.offset(l as isize)).is_null() {
        if value.is_null() {
            return xml;
        }
        if (*xml).attr == &raw mut EZXML_NIL as *mut *mut ::core::ffi::c_char {
            (*xml).attr = malloc(
                (4 as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
            ) as *mut *mut ::core::ffi::c_char;
            *(*xml).attr.offset(1isize) = strdup(b"\0".as_ptr() as *const ::core::ffi::c_char);
        } else {
            (*xml).attr = realloc(
                (*xml).attr as *mut ::core::ffi::c_void,
                ((l + 4 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
            ) as *mut *mut ::core::ffi::c_char;
        }
        *(*xml).attr.offset(l as isize) = name as *mut ::core::ffi::c_char;
        *(*xml).attr.offset((l + 2 as ::core::ffi::c_int) as isize) =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        c = strlen(*(*xml).attr.offset((l + 1 as ::core::ffi::c_int) as isize))
            as ::core::ffi::c_int;
        *(*xml).attr.offset((l + 3 as ::core::ffi::c_int) as isize) = realloc(
            *(*xml).attr.offset((l + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_void,
            (c + 2 as ::core::ffi::c_int) as size_t,
        )
            as *mut ::core::ffi::c_char;
        strcpy(
            (*(*xml).attr.offset((l + 3 as ::core::ffi::c_int) as isize)).offset(c as isize),
            b" \0".as_ptr() as *const ::core::ffi::c_char,
        );
        if (*xml).flags as ::core::ffi::c_int & EZXML_DUP != 0 {
            *(*(*xml).attr.offset((l + 3 as ::core::ffi::c_int) as isize)).offset(c as isize) =
                EZXML_NAMEM as ::core::ffi::c_char;
        }
    } else if (*xml).flags as ::core::ffi::c_int & EZXML_DUP != 0 {
        free(name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    }
    c = l;
    while !(*(*xml).attr.offset(c as isize)).is_null() {
        c += 2 as ::core::ffi::c_int;
    }
    if *(*(*xml).attr.offset((c + 1 as ::core::ffi::c_int) as isize))
        .offset((l / 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
        & EZXML_TXTM
        != 0
    {
        free(
            *(*xml).attr.offset((l + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_void
        );
    }
    if (*xml).flags as ::core::ffi::c_int & EZXML_DUP != 0 {
        *(*(*xml).attr.offset((c + 1 as ::core::ffi::c_int) as isize))
            .offset((l / 2 as ::core::ffi::c_int) as isize) =
            (*(*(*xml).attr.offset((c + 1 as ::core::ffi::c_int) as isize))
                .offset((l / 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                | EZXML_TXTM) as ::core::ffi::c_char;
    } else {
        *(*(*xml).attr.offset((c + 1 as ::core::ffi::c_int) as isize))
            .offset((l / 2 as ::core::ffi::c_int) as isize) =
            (*(*(*xml).attr.offset((c + 1 as ::core::ffi::c_int) as isize))
                .offset((l / 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                & !EZXML_TXTM) as ::core::ffi::c_char;
    }
    if !value.is_null() {
        *(*xml).attr.offset((l + 1 as ::core::ffi::c_int) as isize) =
            value as *mut ::core::ffi::c_char;
    } else {
        if *(*(*xml).attr.offset((c + 1 as ::core::ffi::c_int) as isize))
            .offset((l / 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & EZXML_NAMEM
            != 0
        {
            free(*(*xml).attr.offset(l as isize) as *mut ::core::ffi::c_void);
        }
        memmove(
            (*xml).attr.offset(l as isize) as *mut ::core::ffi::c_void,
            (*xml)
                .attr
                .offset(l as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            ((c - l + 2 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
        );
        (*xml).attr = realloc(
            (*xml).attr as *mut ::core::ffi::c_void,
            ((c + 2 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
        ) as *mut *mut ::core::ffi::c_char;
        memmove(
            (*(*xml).attr.offset((c + 1 as ::core::ffi::c_int) as isize))
                .offset((l / 2 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_void,
            (*(*xml).attr.offset((c + 1 as ::core::ffi::c_int) as isize))
                .offset((l / 2 as ::core::ffi::c_int) as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (c / 2 as ::core::ffi::c_int - l / 2 as ::core::ffi::c_int) as size_t,
        );
    }
    (*xml).flags = ((*xml).flags as ::core::ffi::c_int & !EZXML_DUP) as ::core::ffi::c_short;
    return xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_set_flag(
    mut xml: ezxml_t,
    mut flag: ::core::ffi::c_short,
) -> ezxml_t {
    if !xml.is_null() {
        (*xml).flags = ((*xml).flags as ::core::ffi::c_int | flag as ::core::ffi::c_int)
            as ::core::ffi::c_short;
    }
    return xml;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ezxml_cut(mut xml: ezxml_t) -> ezxml_t {
    let mut cur: ezxml_t = ::core::ptr::null_mut::<ezxml>();
    if xml.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    if !(*xml).next.is_null() {
        (*(*xml).next).sibling = (*xml).sibling;
    }
    if !(*xml).parent.is_null() {
        cur = (*(*xml).parent).child;
        if cur == xml {
            (*(*xml).parent).child = (*xml).ordered;
        } else {
            while (*cur).ordered != xml {
                cur = (*cur).ordered;
            }
            (*cur).ordered = (*(*cur).ordered).ordered;
            cur = (*(*xml).parent).child;
            if strcmp((*cur).name, (*xml).name) != 0 {
                while strcmp((*(*cur).sibling).name, (*xml).name) != 0 {
                    cur = (*cur).sibling;
                }
                if (*cur).sibling == xml {
                    (*cur).sibling = if !(*xml).next.is_null() {
                        (*xml).next
                    } else {
                        (*(*cur).sibling).sibling
                    };
                } else {
                    cur = (*cur).sibling;
                }
            }
            while !(*cur).next.is_null() && (*cur).next != xml {
                cur = (*cur).next;
            }
            if !(*cur).next.is_null() {
                (*cur).next = (*(*cur).next).next;
            }
        }
    }
    (*xml).next = ::core::ptr::null_mut::<ezxml>();
    (*xml).sibling = (*xml).next;
    (*xml).ordered = (*xml).sibling;
    return xml;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
