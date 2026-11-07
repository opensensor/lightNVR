use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    unsafe fn httpd_authorize_global_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_process_get_override_path() -> *const ::core::ffi::c_char;
    unsafe fn go2rtc_process_get_listener_policy(
        policy: *mut ::core::ffi::c_char,
        capacity: size_t,
    ) -> bool;
    unsafe fn go2rtc_process_get_config_path() -> *const ::core::ffi::c_char;
    unsafe fn go2rtc_process_try_get_pid(pid: *mut ::core::ffi::c_int) -> bool;
    unsafe fn yaml_redact_alloc(
        src: *const ::core::ffi::c_char,
        len: size_t,
        out_len: *mut size_t,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn yaml_redact_is_available() -> ::core::ffi::c_int;
    unsafe fn db_get_system_setting_alloc(
        key: *const ::core::ffi::c_char,
        out: *mut *mut ::core::ffi::c_char,
        out_len: *mut size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
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
    unsafe fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
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
pub type uint64_t = u64;
pub type uintmax_t = ::libc::uintmax_t;
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
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
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SystemAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const EFBIG: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_go2rtc_memory_usage(
    mut memory_usage: *mut ::core::ffi::c_ulonglong,
) -> bool {
    if memory_usage.is_null() {
        return r#false != 0;
    }
    *memory_usage = 0 as ::core::ffi::c_ulonglong;
    let mut pid: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if !go2rtc_process_try_get_pid(&raw mut pid) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc lifecycle busy; memory usage temporarily unavailable\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if pid <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"No go2rtc process found (PID: %d); memory usage is zero\0".as_ptr()
                as *const ::core::ffi::c_char,
            pid,
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
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to open %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut status_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    }
    let mut line: [::core::ffi::c_char; 256] = [0; 256];
    let mut vm_rss: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
        status_file,
    )
    .is_null()
    {
        if strncmp(
            &raw mut line as *mut ::core::ffi::c_char,
            b"VmRSS:\0".as_ptr() as *const ::core::ffi::c_char,
            6 as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            continue;
        }
        let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        vm_rss = strtoul(
            (&raw mut line as *mut ::core::ffi::c_char).offset(6 as ::core::ffi::c_int as isize),
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
        b"go2rtc memory usage (PID %d): %llu bytes\0".as_ptr() as *const ::core::ffi::c_char,
        pid,
        *memory_usage,
    );
    return r#true != 0;
}
pub const EFFECTIVE_CONFIG_FILE_CAP: ::core::ffi::c_int =
    1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
unsafe extern "C" fn read_file_alloc(
    mut path: *const ::core::ffi::c_char,
    mut out: *mut *mut ::core::ffi::c_char,
    mut out_len: *mut size_t,
) -> ::core::ffi::c_int {
    *out = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *out_len = 0 as size_t;
    let mut fd: ::core::ffi::c_int = open(path, O_RDONLY | O_CLOEXEC);
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() == ENOENT {
            return 0 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"effective-config: open %s failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
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
    if fstat(fd, &raw mut st) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"effective-config: fstat %s failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
        close(fd);
        return -1 as ::core::ffi::c_int;
    }
    if st.st_size as size_t > EFFECTIVE_CONFIG_FILE_CAP as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"effective-config: %s is %lld bytes (cap %d) \xE2\x80\x94 refusing\0".as_ptr()
                as *const ::core::ffi::c_char,
            path,
            st.st_size as ::core::ffi::c_longlong,
            1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int,
        );
        close(fd);
        return -1 as ::core::ffi::c_int;
    }
    let mut buf: *mut ::core::ffi::c_char =
        malloc((st.st_size as size_t).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if buf.is_null() {
        close(fd);
        return -1 as ::core::ffi::c_int;
    }
    let mut total: size_t = 0 as size_t;
    while total < st.st_size as size_t {
        let mut n: ssize_t = read(
            fd,
            buf.offset(total as isize) as *mut ::core::ffi::c_void,
            (st.st_size as size_t).wrapping_sub(total),
        );
        if n < 0 as ssize_t {
            if *__errno_location() == EINTR {
                continue;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"effective-config: read %s failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                path,
                strerror(*__errno_location()),
            );
            free(buf as *mut ::core::ffi::c_void);
            close(fd);
            return -1 as ::core::ffi::c_int;
        } else {
            if n == 0 as ssize_t {
                break;
            }
            total = total.wrapping_add(n as size_t);
        }
    }
    close(fd);
    if total != st.st_size as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"effective-config: %s shrank during read (got %zu of %lld) \xE2\x80\x94 refusing to return truncated content\0"
                .as_ptr() as *const ::core::ffi::c_char,
            path,
            total,
            st.st_size as ::core::ffi::c_longlong,
        );
        free(buf as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    *buf.offset(total as isize) = '\0' as ::core::ffi::c_char;
    *out = buf;
    *out_len = total;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn add_warning(
    mut warnings: *mut cJSON,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut msg: [::core::ffi::c_char; 256] = [0; 256];
    let mut ap: ::core::ffi::VaList;
    ap = c2rust_args.clone();
    vsnprintf(
        &raw mut msg as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        fmt,
        ap.clone(),
    );
    cJSON_AddItemToArray(
        warnings,
        cJSON_CreateString(&raw mut msg as *mut ::core::ffi::c_char),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_system_go2rtc_effective_config(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/system/go2rtc/effective-config\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut warnings: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || warnings.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(warnings);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut base_path: *const ::core::ffi::c_char = go2rtc_process_get_config_path();
    let mut override_path: *const ::core::ffi::c_char = go2rtc_process_get_override_path();
    let mut initialized: ::core::ffi::c_int = !base_path.is_null() as ::core::ffi::c_int;
    cJSON_AddBoolToObject(
        root,
        b"go2rtc_initialized\0".as_ptr() as *const ::core::ffi::c_char,
        initialized,
    );
    cJSON_AddBoolToObject(
        root,
        b"redaction_available\0".as_ptr() as *const ::core::ffi::c_char,
        yaml_redact_is_available(),
    );
    let mut base_loaded: bool = r#false != 0;
    let mut override_loaded: bool = r#false != 0;
    let mut base_yaml: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut base_len: size_t = 0 as size_t;
    if !base_path.is_null() {
        let mut rc: ::core::ffi::c_int =
            read_file_alloc(base_path, &raw mut base_yaml, &raw mut base_len);
        if rc == 1 as ::core::ffi::c_int {
            base_loaded = r#true != 0;
        } else if rc == 0 as ::core::ffi::c_int {
            add_warning(
                warnings,
                b"go2rtc.yaml not present at %s (go2rtc not started yet?)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                base_path,
            );
        } else if rc < 0 as ::core::ffi::c_int {
            add_warning(
                warnings,
                b"failed to read go2rtc.yaml at %s \xE2\x80\x94 see server log\0".as_ptr()
                    as *const ::core::ffi::c_char,
                base_path,
            );
        }
    } else {
        add_warning(
            warnings,
            b"go2rtc process manager not initialized; base config unavailable\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut base_redacted: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !base_yaml.is_null() {
        let mut out_len: size_t = 0 as size_t;
        base_redacted = yaml_redact_alloc(base_yaml, base_len, &raw mut out_len);
        if base_redacted.is_null() {
            add_warning(
                warnings,
                b"redaction failed for base config \xE2\x80\x94 content omitted\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                root,
                b"base\0".as_ptr() as *const ::core::ffi::c_char,
                b"[REDACTION_FAILED]\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            cJSON_AddStringToObject(
                root,
                b"base\0".as_ptr() as *const ::core::ffi::c_char,
                base_redacted,
            );
        }
    } else {
        cJSON_AddStringToObject(
            root,
            b"base\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut override_yaml: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut override_len: size_t = 0 as size_t;
    if !override_path.is_null() {
        let mut rc_0: ::core::ffi::c_int =
            read_file_alloc(override_path, &raw mut override_yaml, &raw mut override_len);
        if rc_0 == 1 as ::core::ffi::c_int {
            override_loaded = r#true != 0;
        } else if rc_0 < 0 as ::core::ffi::c_int {
            add_warning(
                warnings,
                b"failed to read override.yaml at %s \xE2\x80\x94 see server log\0".as_ptr()
                    as *const ::core::ffi::c_char,
                override_path,
            );
        }
    }
    let mut override_redacted: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !override_yaml.is_null() {
        let mut out_len_0: size_t = 0 as size_t;
        override_redacted = yaml_redact_alloc(override_yaml, override_len, &raw mut out_len_0);
        if override_redacted.is_null() {
            add_warning(
                warnings,
                b"redaction failed for override \xE2\x80\x94 content omitted\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                root,
                b"override\0".as_ptr() as *const ::core::ffi::c_char,
                b"[REDACTION_FAILED]\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            cJSON_AddStringToObject(
                root,
                b"override\0".as_ptr() as *const ::core::ffi::c_char,
                override_redacted,
            );
        }
    } else {
        cJSON_AddStringToObject(
            root,
            b"override\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut listener_policy: [::core::ffi::c_char; 512] = [0; 512];
    let mut managed_policy: bool = go2rtc_process_get_listener_policy(
        &raw mut listener_policy as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    );
    cJSON_AddStringToObject(
        root,
        b"managed_listener_policy\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut listener_policy as *mut ::core::ffi::c_char,
    );
    if managed_policy {
        add_warning(
            warnings,
            b"Managed listener policy is applied after overrides: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut listener_policy as *mut ::core::ffi::c_char,
        );
    } else {
        add_warning(
            warnings,
            b"No managed listener policy recorded. External go2rtc services must restrict their own HTTP and RTSP listeners.\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut order: *mut cJSON = cJSON_CreateArray();
    if !order.is_null() {
        if base_loaded {
            cJSON_AddItemToArray(
                order,
                cJSON_CreateString(b"go2rtc.yaml\0".as_ptr() as *const ::core::ffi::c_char),
            );
        }
        if override_loaded {
            cJSON_AddItemToArray(
                order,
                cJSON_CreateString(b"override.yaml\0".as_ptr() as *const ::core::ffi::c_char),
            );
        }
        if managed_policy {
            cJSON_AddItemToArray(
                order,
                cJSON_CreateString(
                    b"managed_listener_policy\0".as_ptr() as *const ::core::ffi::c_char
                ),
            );
        }
        cJSON_AddItemToObject(
            root,
            b"merged_source_order\0".as_ptr() as *const ::core::ffi::c_char,
            order,
        );
    }
    cJSON_AddBoolToObject(
        root,
        b"override_in_use\0".as_ptr() as *const ::core::ffi::c_char,
        override_loaded as cJSON_bool,
    );
    cJSON_AddItemToObject(
        root,
        b"warnings\0".as_ptr() as *const ::core::ffi::c_char,
        warnings,
    );
    let mut body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    free(base_yaml as *mut ::core::ffi::c_void);
    free(base_redacted as *mut ::core::ffi::c_void);
    free(override_yaml as *mut ::core::ffi::c_void);
    free(override_redacted as *mut ::core::ffi::c_void);
    if body.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, body);
    free(body as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn try_read_file(
    mut path: *const ::core::ffi::c_char,
    mut out: *mut *mut ::core::ffi::c_char,
    mut out_len: *mut size_t,
    mut read_error: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *out = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *out_len = 0 as size_t;
    *read_error = 0 as ::core::ffi::c_int;
    let mut fd: ::core::ffi::c_int = open(path, O_RDONLY | O_CLOEXEC | O_NOFOLLOW | O_NONBLOCK);
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() == ENOENT {
            return 0 as ::core::ffi::c_int;
        }
        *read_error = *__errno_location();
        return -1 as ::core::ffi::c_int;
    }
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
    if fstat(fd, &raw mut st) != 0 as ::core::ffi::c_int {
        *read_error = *__errno_location();
        close(fd);
        return -1 as ::core::ffi::c_int;
    }
    if !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
        || st.st_size < 0 as __off_t
        || st.st_size as uintmax_t > EFFECTIVE_CONFIG_FILE_CAP as uintmax_t
    {
        *read_error = if !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
            EINVAL
        } else if st.st_size < 0 as __off_t {
            EIO
        } else {
            EFBIG
        };
        close(fd);
        return -1 as ::core::ffi::c_int;
    }
    let mut buf: *mut ::core::ffi::c_char =
        malloc((st.st_size as size_t).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if buf.is_null() {
        *read_error = ENOMEM;
        close(fd);
        return -1 as ::core::ffi::c_int;
    }
    let mut total: size_t = 0 as size_t;
    while total < st.st_size as size_t {
        let mut n: ssize_t = read(
            fd,
            buf.offset(total as isize) as *mut ::core::ffi::c_void,
            (st.st_size as size_t).wrapping_sub(total),
        );
        if n < 0 as ssize_t {
            if *__errno_location() == EINTR {
                continue;
            }
            *read_error = *__errno_location();
            free(buf as *mut ::core::ffi::c_void);
            close(fd);
            return -1 as ::core::ffi::c_int;
        } else {
            if n == 0 as ssize_t {
                break;
            }
            total = total.wrapping_add(n as size_t);
        }
    }
    if close(fd) != 0 as ::core::ffi::c_int {
        *read_error = *__errno_location();
        free(buf as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    if total != st.st_size as size_t {
        *read_error = EIO;
        free(buf as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    *buf.offset(total as isize) = '\0' as ::core::ffi::c_char;
    *out = buf;
    *out_len = total;
    return 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_system_go2rtc_override_status(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/system/go2rtc/override-status\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut override_path: *const ::core::ffi::c_char = go2rtc_process_get_override_path();
    cJSON_AddStringToObject(
        root,
        b"expected_file_path\0".as_ptr() as *const ::core::ffi::c_char,
        if !override_path.is_null() {
            override_path
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut db_content: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut db_len: size_t = 0 as size_t;
    let mut db_rc: ::core::ffi::c_int = db_get_system_setting_alloc(
        b"go2rtc_config_override\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut db_content,
        &raw mut db_len,
    );
    let mut db_has_content: bool =
        db_rc == 0 as ::core::ffi::c_int && !db_content.is_null() && db_len > 0 as size_t;
    cJSON_AddNumberToObject(
        root,
        b"db_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (if db_has_content as ::core::ffi::c_int != 0 {
            db_len
        } else {
            0 as size_t
        }) as ::core::ffi::c_double,
    );
    let mut file_content: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut file_len: size_t = 0 as size_t;
    let mut file_errno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut file_rc: ::core::ffi::c_int = if !override_path.is_null() {
        try_read_file(
            override_path,
            &raw mut file_content,
            &raw mut file_len,
            &raw mut file_errno,
        )
    } else {
        -1 as ::core::ffi::c_int
    };
    let mut file_present: bool = file_rc == 1 as ::core::ffi::c_int;
    cJSON_AddBoolToObject(
        root,
        b"file_present\0".as_ptr() as *const ::core::ffi::c_char,
        file_present as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        root,
        b"file_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (if file_present as ::core::ffi::c_int != 0 {
            file_len as ::core::ffi::c_long
        } else {
            -1 as ::core::ffi::c_long
        }) as ::core::ffi::c_double,
    );
    if file_rc < 0 as ::core::ffi::c_int {
        let mut file_error: *const ::core::ffi::c_char = if override_path.is_null() {
            b"no override path\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            strerror(file_errno)
        };
        cJSON_AddStringToObject(
            root,
            b"file_read_error\0".as_ptr() as *const ::core::ffi::c_char,
            file_error,
        );
    }
    let mut content_matches: bool = r#false != 0;
    if db_has_content as ::core::ffi::c_int != 0
        && file_present as ::core::ffi::c_int != 0
        && db_len == file_len
    {
        content_matches = memcmp(
            db_content as *const ::core::ffi::c_void,
            file_content as *const ::core::ffi::c_void,
            db_len,
        ) == 0 as ::core::ffi::c_int;
    } else if !db_has_content && file_rc == 0 as ::core::ffi::c_int {
        content_matches = r#true != 0;
    }
    cJSON_AddBoolToObject(
        root,
        b"content_matches\0".as_ptr() as *const ::core::ffi::c_char,
        content_matches as cJSON_bool,
    );
    let mut reason: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut reason_len: size_t = 0 as size_t;
    let mut rc_reason: ::core::ffi::c_int = db_get_system_setting_alloc(
        b"go2rtc_config_override_disabled_reason\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut reason,
        &raw mut reason_len,
    );
    let mut quarantine_active: bool =
        rc_reason == 0 as ::core::ffi::c_int && !reason.is_null() && reason_len > 0 as size_t;
    cJSON_AddBoolToObject(
        root,
        b"quarantine_active\0".as_ptr() as *const ::core::ffi::c_char,
        quarantine_active as cJSON_bool,
    );
    if quarantine_active {
        cJSON_AddStringToObject(
            root,
            b"quarantine_reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
    }
    free(reason as *mut ::core::ffi::c_void);
    let mut quar_copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut quar_copy_len: size_t = 0 as size_t;
    let mut rc_quar: ::core::ffi::c_int = db_get_system_setting_alloc(
        b"go2rtc_config_override_quarantined\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut quar_copy,
        &raw mut quar_copy_len,
    );
    let mut has_quar_copy: bool =
        rc_quar == 0 as ::core::ffi::c_int && !quar_copy.is_null() && quar_copy_len > 0 as size_t;
    cJSON_AddNumberToObject(
        root,
        b"quarantined_copy_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (if has_quar_copy as ::core::ffi::c_int != 0 {
            quar_copy_len
        } else {
            0 as size_t
        }) as ::core::ffi::c_double,
    );
    free(quar_copy as *mut ::core::ffi::c_void);
    let mut validated: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut validated_len: size_t = 0 as size_t;
    let mut rc_v: ::core::ffi::c_int = db_get_system_setting_alloc(
        b"go2rtc_override_validated_version\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut validated,
        &raw mut validated_len,
    );
    if rc_v == 0 as ::core::ffi::c_int && !validated.is_null() {
        cJSON_AddStringToObject(
            root,
            b"validated_version\0".as_ptr() as *const ::core::ffi::c_char,
            validated,
        );
    } else {
        cJSON_AddStringToObject(
            root,
            b"validated_version\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    free(validated as *mut ::core::ffi::c_void);
    let mut pid: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut pid_known: bool = go2rtc_process_try_get_pid(&raw mut pid);
    if !pid_known {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc lifecycle busy; runtime state temporarily unavailable\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddBoolToObject(
        root,
        b"process_state_known\0".as_ptr() as *const ::core::ffi::c_char,
        pid_known as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        root,
        b"process_running\0".as_ptr() as *const ::core::ffi::c_char,
        (pid > 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    cJSON_AddNumberToObject(
        root,
        b"process_pid\0".as_ptr() as *const ::core::ffi::c_char,
        if pid > 0 as ::core::ffi::c_int {
            pid as ::core::ffi::c_double
        } else {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        },
    );
    free(db_content as *mut ::core::ffi::c_void);
    free(file_content as *mut ::core::ffi::c_void);
    let mut body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if body.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, body);
    free(body as *mut ::core::ffi::c_void);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
