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
    unsafe fn get_detection_config() -> *mut detection_config_t;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
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
pub type uint8_t = u8;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MODEL_TYPE_SOD: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"sod\0") };
pub const MODEL_TYPE_SOD_REALNET: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"sod_realnet\0") };
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_embedded_device() -> bool {
    static mut checked: bool = r#false != 0;
    static mut is_embedded: bool = r#false != 0;
    if checked {
        return is_embedded;
    }
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
                b"Detected embedded device with %lu KB RAM\0".as_ptr()
                    as *const ::core::ffi::c_char,
                total_mem_kb,
            );
            is_embedded = r#true != 0;
            checked = r#true != 0;
            return r#true != 0;
        }
    }
    let mut num_cores: ::core::ffi::c_int =
        sysconf(C2Rust_Unnamed::_SC_NPROCESSORS_ONLN.0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if num_cores > 0 as ::core::ffi::c_int && num_cores <= 2 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Detected embedded device with %d CPU cores\0".as_ptr() as *const ::core::ffi::c_char,
            num_cores,
        );
        is_embedded = r#true != 0;
        checked = r#true != 0;
        return r#true != 0;
    }
    checked = r#true != 0;
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_downscale_factor(
    mut model_type: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut config: *mut detection_config_t = get_detection_config();
    if config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get detection configuration\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut embedded: bool = is_embedded_device();
    if !embedded {
        if strcmp(model_type, MODEL_TYPE_SOD.as_ptr()) == 0 as ::core::ffi::c_int {
            return (*config).downscale_factor_cnn;
        } else if strcmp(model_type, MODEL_TYPE_SOD_REALNET.as_ptr()) == 0 as ::core::ffi::c_int {
            return (*config).downscale_factor_realnet;
        } else {
            return (*config).downscale_factor_default;
        }
    }
    if strcmp(model_type, MODEL_TYPE_SOD.as_ptr()) == 0 as ::core::ffi::c_int {
        let mut downscale: ::core::ffi::c_int = (*config).downscale_factor_cnn;
        let mut meminfo: *mut FILE = fopen(
            b"/proc/meminfo\0".as_ptr() as *const ::core::ffi::c_char,
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !meminfo.is_null() {
            let mut line: [::core::ffi::c_char; 256] = [0; 256];
            let mut available_mem_kb: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
            while !fgets(
                &raw mut line as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
                meminfo,
            )
            .is_null()
            {
                if strncmp(
                    &raw mut line as *mut ::core::ffi::c_char,
                    b"MemAvailable:\0".as_ptr() as *const ::core::ffi::c_char,
                    13 as size_t,
                ) != 0 as ::core::ffi::c_int
                {
                    continue;
                }
                sscanf(
                    &raw mut line as *mut ::core::ffi::c_char,
                    b"MemAvailable: %lu\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut available_mem_kb,
                );
                break;
            }
            fclose(meminfo);
            if available_mem_kb > 0 as ::core::ffi::c_ulong
                && available_mem_kb
                    < (50 as ::core::ffi::c_ulong).wrapping_mul(1024 as ::core::ffi::c_ulong)
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Very low memory available (%lu KB), using more aggressive downscaling\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    available_mem_kb,
                );
                downscale = 4 as ::core::ffi::c_int;
            }
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using downscale factor %d for CNN model on embedded device\0".as_ptr()
                as *const ::core::ffi::c_char,
            downscale,
        );
        return downscale;
    } else if strcmp(model_type, MODEL_TYPE_SOD_REALNET.as_ptr()) == 0 as ::core::ffi::c_int {
        return (*config).downscale_factor_realnet;
    }
    return (*config).downscale_factor_default;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_detection_threshold(
    mut model_type: *const ::core::ffi::c_char,
    mut configured_threshold: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    let mut config: *mut detection_config_t = get_detection_config();
    if config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get detection configuration\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0.3f32;
    }
    if configured_threshold > 0.0f32 {
        return configured_threshold;
    }
    let mut embedded: bool = is_embedded_device();
    if strcmp(model_type, MODEL_TYPE_SOD_REALNET.as_ptr()) == 0 as ::core::ffi::c_int {
        return (*config).threshold_realnet;
    } else if strcmp(model_type, MODEL_TYPE_SOD.as_ptr()) == 0 as ::core::ffi::c_int {
        if embedded {
            return (*config).threshold_cnn_embedded;
        } else {
            return (*config).threshold_cnn;
        }
    }
    return (*config).threshold_cnn;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn downscale_frame(
    mut src_data: *const uint8_t,
    mut src_width: ::core::ffi::c_int,
    mut src_height: ::core::ffi::c_int,
    mut src_channels: ::core::ffi::c_int,
    mut dst_data: *mut uint8_t,
    mut dst_width: ::core::ffi::c_int,
    mut dst_height: ::core::ffi::c_int,
    mut dst_channels: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if src_data.is_null()
        || dst_data.is_null()
        || src_width <= 0 as ::core::ffi::c_int
        || src_height <= 0 as ::core::ffi::c_int
        || dst_width <= 0 as ::core::ffi::c_int
        || dst_height <= 0 as ::core::ffi::c_int
        || src_channels <= 0 as ::core::ffi::c_int
        || dst_channels <= 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for downscale_frame\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if src_channels != dst_channels {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Source and destination channels must match\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut scale_x: ::core::ffi::c_float =
        src_width as ::core::ffi::c_float / dst_width as ::core::ffi::c_float;
    let mut scale_y: ::core::ffi::c_float =
        src_height as ::core::ffi::c_float / dst_height as ::core::ffi::c_float;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < dst_height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < dst_width {
            let mut src_x: ::core::ffi::c_float = x as ::core::ffi::c_float * scale_x;
            let mut src_y: ::core::ffi::c_float = y as ::core::ffi::c_float * scale_y;
            let mut src_x_int: ::core::ffi::c_int = src_x as ::core::ffi::c_int;
            let mut src_y_int: ::core::ffi::c_int = src_y as ::core::ffi::c_int;
            let mut src_x_frac: ::core::ffi::c_float = src_x - src_x_int as ::core::ffi::c_float;
            let mut src_y_frac: ::core::ffi::c_float = src_y - src_y_int as ::core::ffi::c_float;
            let mut src_x_int_p1: ::core::ffi::c_int =
                if (src_x_int + 1 as ::core::ffi::c_int) < src_width {
                    src_x_int + 1 as ::core::ffi::c_int
                } else {
                    src_x_int
                };
            let mut src_y_int_p1: ::core::ffi::c_int =
                if (src_y_int + 1 as ::core::ffi::c_int) < src_height {
                    src_y_int + 1 as ::core::ffi::c_int
                } else {
                    src_y_int
                };
            let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while c < dst_channels {
                let mut p00: uint8_t = *src_data
                    .offset(((src_y_int * src_width + src_x_int) * src_channels + c) as isize);
                let mut p01: uint8_t = *src_data
                    .offset(((src_y_int * src_width + src_x_int_p1) * src_channels + c) as isize);
                let mut p10: uint8_t = *src_data
                    .offset(((src_y_int_p1 * src_width + src_x_int) * src_channels + c) as isize);
                let mut p11: uint8_t = *src_data.offset(
                    ((src_y_int_p1 * src_width + src_x_int_p1) * src_channels + c) as isize,
                );
                let mut top: ::core::ffi::c_float = p00 as ::core::ffi::c_float
                    * (1.0f32 - src_x_frac)
                    + p01 as ::core::ffi::c_float * src_x_frac;
                let mut bottom: ::core::ffi::c_float = p10 as ::core::ffi::c_float
                    * (1.0f32 - src_x_frac)
                    + p11 as ::core::ffi::c_float * src_x_frac;
                let mut value: ::core::ffi::c_float = top
                    * (1 as ::core::ffi::c_int as ::core::ffi::c_float - src_y_frac)
                    + bottom * src_y_frac;
                *dst_data.offset(((y * dst_width + x) * dst_channels + c) as isize) =
                    value as uint8_t;
                c += 1;
            }
            x += 1;
        }
        y += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn calculate_detection_memory_requirements(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut channels: ::core::ffi::c_int,
    mut model_type: *const ::core::ffi::c_char,
) -> size_t {
    let mut downscale_factor: ::core::ffi::c_int = get_downscale_factor(model_type);
    let mut downscaled_width: ::core::ffi::c_int = width / downscale_factor;
    let mut downscaled_height: ::core::ffi::c_int = height / downscale_factor;
    downscaled_width = downscaled_width / 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
    downscaled_height = downscaled_height / 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
    let mut frame_size: size_t = (downscaled_width as size_t)
        .wrapping_mul(downscaled_height as size_t)
        .wrapping_mul(channels as size_t);
    let mut model_size: size_t = 0 as size_t;
    if strcmp(model_type, MODEL_TYPE_SOD.as_ptr()) == 0 as ::core::ffi::c_int {
        model_size = (20 as ::core::ffi::c_int as size_t)
            .wrapping_mul(1024 as size_t)
            .wrapping_mul(1024 as size_t);
    } else if strcmp(model_type, MODEL_TYPE_SOD_REALNET.as_ptr()) == 0 as ::core::ffi::c_int {
        model_size = (5 as ::core::ffi::c_int as size_t)
            .wrapping_mul(1024 as size_t)
            .wrapping_mul(1024 as size_t);
    }
    let mut config: *const detection_config_t = get_detection_config();
    let mut buffer_pool_size: size_t = if !config.is_null() {
        ((*config).buffer_pool_size as size_t).wrapping_mul(frame_size)
    } else {
        (4 as size_t).wrapping_mul(frame_size)
    };
    return frame_size
        .wrapping_add(model_size)
        .wrapping_add(buffer_pool_size);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
