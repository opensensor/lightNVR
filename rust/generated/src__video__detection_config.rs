use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn get_total_memory_allocated() -> size_t;
    unsafe fn get_peak_memory_allocated() -> size_t;
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
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub struct detection_config_t {
    pub buffer_pool_size: ::core::ffi::c_int,
    pub concurrent_detections: ::core::ffi::c_int,
    pub buffer_allocation_retries: ::core::ffi::c_int,
    pub downscale_factor_default: ::core::ffi::c_int,
    pub downscale_factor_cnn: ::core::ffi::c_int,
    pub downscale_factor_realnet: ::core::ffi::c_int,
    pub threshold_cnn: ::core::ffi::c_float,
    pub threshold_cnn_embedded: ::core::ffi::c_float,
    pub threshold_realnet: ::core::ffi::c_float,
    pub save_frames_for_debug: bool,
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
#[unsafe(no_mangle)]
pub static mut default_config: detection_config_t = detection_config_t {
    buffer_pool_size: 0,
    concurrent_detections: MAX_STREAMS,
    buffer_allocation_retries: 0,
    downscale_factor_default: 1 as ::core::ffi::c_int,
    downscale_factor_cnn: 2 as ::core::ffi::c_int,
    downscale_factor_realnet: 1 as ::core::ffi::c_int,
    threshold_cnn: 0.3f32,
    threshold_cnn_embedded: 0.3f32,
    threshold_realnet: 5.0f32,
    save_frames_for_debug: r#false != 0,
};
#[unsafe(no_mangle)]
pub static mut embedded_config: detection_config_t = detection_config_t {
    buffer_pool_size: 0,
    concurrent_detections: 2 as ::core::ffi::c_int,
    buffer_allocation_retries: 0,
    downscale_factor_default: 1 as ::core::ffi::c_int,
    downscale_factor_cnn: 2 as ::core::ffi::c_int,
    downscale_factor_realnet: 1 as ::core::ffi::c_int,
    threshold_cnn: 0.3f32,
    threshold_cnn_embedded: 0.3f32,
    threshold_realnet: 5.0f32,
    save_frames_for_debug: r#false != 0,
};
static mut current_config: *mut detection_config_t = ::core::ptr::null_mut::<detection_config_t>();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_detection_config() -> ::core::ffi::c_int {
    if !current_config.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    current_config = &raw mut default_config;
    let mut meminfo: *mut FILE = fopen(
        b"/proc/meminfo\0".as_ptr() as *const ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !meminfo.is_null() {
        let mut line: [::core::ffi::c_char; 256] = [0; 256];
        let mut total_mem_kb: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
        while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
            meminfo,
        )
        .is_null()
        {
            if strncmp(
                &raw mut line as *mut ::core::ffi::c_char,
                b"MemTotal:\0".as_ptr() as *const ::core::ffi::c_char,
                9 as size_t,
            ) != 0 as ::core::ffi::c_int
            {
                continue;
            }
            sscanf(
                &raw mut line as *mut ::core::ffi::c_char,
                b"MemTotal: %lu\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut total_mem_kb,
            );
            break;
        }
        fclose(meminfo);
        if total_mem_kb > 0 as ::core::ffi::c_ulong
            && total_mem_kb
                < (512 as ::core::ffi::c_ulong).wrapping_mul(1024 as ::core::ffi::c_ulong)
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Detected embedded device with %lu KB RAM, using embedded configuration\0".as_ptr()
                    as *const ::core::ffi::c_char,
                total_mem_kb,
            );
            current_config = &raw mut embedded_config;
        }
    }
    let mut num_cores: ::core::ffi::c_int =
        sysconf(C2Rust_Unnamed::_SC_NPROCESSORS_ONLN.0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if num_cores > 0 as ::core::ffi::c_int && num_cores <= 2 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Detected embedded device with %d CPU cores, using embedded configuration\0".as_ptr()
                as *const ::core::ffi::c_char,
            num_cores,
        );
        current_config = &raw mut embedded_config;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Detection configuration initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_detection_config() -> *mut detection_config_t {
    if current_config.is_null() {
        init_detection_config();
    }
    return current_config;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_detection_config(
    mut config: *const detection_config_t,
) -> ::core::ffi::c_int {
    if config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid configuration\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*config).concurrent_detections <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid configuration parameters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if current_config == &raw mut default_config {
        memcpy(
            &raw mut default_config as *mut ::core::ffi::c_void,
            config as *const ::core::ffi::c_void,
            ::core::mem::size_of::<detection_config_t>(),
        );
    } else if current_config == &raw mut embedded_config {
        memcpy(
            &raw mut embedded_config as *mut ::core::ffi::c_void,
            config as *const ::core::ffi::c_void,
            ::core::mem::size_of::<detection_config_t>(),
        );
    } else {
        if current_config.is_null() {
            current_config =
                malloc(::core::mem::size_of::<detection_config_t>()) as *mut detection_config_t;
            if current_config.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to allocate memory for configuration\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        memcpy(
            current_config as *mut ::core::ffi::c_void,
            config as *const ::core::ffi::c_void,
            ::core::mem::size_of::<detection_config_t>(),
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Detection configuration updated\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_detection_memory_usage(
    mut total_memory: *mut size_t,
    mut peak_memory: *mut size_t,
) {
    if !total_memory.is_null() {
        *total_memory = get_total_memory_allocated();
    }
    if !peak_memory.is_null() {
        *peak_memory = get_peak_memory_allocated();
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
