use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    unsafe fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn dirname(__path: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn log_get_thread_component() -> *const ::core::ffi::c_char;
    unsafe fn log_get_thread_stream() -> *const ::core::ffi::c_char;
    unsafe fn mkdir_recursive(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
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
pub type time_t = __time_t;
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
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub log_file: *mut FILE,
    pub log_filename: [::core::ffi::c_char; 256],
    pub mutex: pthread_mutex_t,
    pub initialized: ::core::ffi::c_int,
}
pub type cJSON_bool = ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut json_logger: C2Rust_Unnamed_0 = unsafe {
    C2Rust_Unnamed_0 {
        log_file: ::core::ptr::null_mut::<FILE>(),
        log_filename: ::core::mem::transmute::<
            [u8; 256],
            [::core::ffi::c_char; 256],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0 as ::core::ffi::c_int,
                __count: 0 as ::core::ffi::c_uint,
                __owner: 0 as ::core::ffi::c_int,
                __nusers: 0 as ::core::ffi::c_uint,
                __kind: C2Rust_Unnamed::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
                __spins: 0 as ::core::ffi::c_short,
                __glibc_reserved: 0 as ::core::ffi::c_short,
                __list: __pthread_internal_list {
                    __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                    __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
                },
            },
        },
        initialized: 0 as ::core::ffi::c_int,
    }
};
static mut json_log_level_strings: [*const ::core::ffi::c_char; 4] = [
    b"error\0".as_ptr() as *const ::core::ffi::c_char,
    b"warning\0".as_ptr() as *const ::core::ffi::c_char,
    b"info\0".as_ptr() as *const ::core::ffi::c_char,
    b"debug\0".as_ptr() as *const ::core::ffi::c_char,
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_json_logger(
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if filename.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if pthread_mutex_init(
        &raw mut json_logger.mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"Failed to initialize JSON logger mutex\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut json_logger.mutex);
    let mut dir_path: *mut ::core::ffi::c_char = strdup(filename);
    if dir_path.is_null() {
        pthread_mutex_unlock(&raw mut json_logger.mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut dir: *const ::core::ffi::c_char = dirname(dir_path);
    if mkdir_recursive(dir) != 0 as ::core::ffi::c_int {
        free(dir_path as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut json_logger.mutex);
        return -1 as ::core::ffi::c_int;
    }
    free(dir_path as *mut ::core::ffi::c_void);
    let mut jlog_fd: ::core::ffi::c_int = open(
        filename,
        O_WRONLY | O_CREAT | O_APPEND,
        0o640 as ::core::ffi::c_int,
    );
    json_logger.log_file = if jlog_fd >= 0 as ::core::ffi::c_int {
        fdopen(jlog_fd, b"a\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<FILE>()
    };
    if json_logger.log_file.is_null() {
        if jlog_fd >= 0 as ::core::ffi::c_int {
            close(jlog_fd);
        }
        pthread_mutex_unlock(&raw mut json_logger.mutex);
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut json_logger.log_filename as *mut ::core::ffi::c_char,
        filename,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    json_logger.initialized = 1 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut json_logger.mutex);
    let mut now: time_t = 0;
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
    let mut tm_info: *const tm = ::core::ptr::null::<tm>();
    let mut timestamp: [::core::ffi::c_char; 32] = [0; 32];
    time(&raw mut now);
    tm_info = localtime_r(&raw mut now, &raw mut tm_buf);
    strftime(
        &raw mut timestamp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%Y-%m-%dT%H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
        tm_info,
    );
    write_json_log(
        log_level_t::LOG_LEVEL_INFO,
        &raw mut timestamp as *mut ::core::ffi::c_char,
        b"JSON logger initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_json_logger() {
    pthread_mutex_lock(&raw mut json_logger.mutex);
    if !json_logger.log_file.is_null() {
        fclose(json_logger.log_file);
        json_logger.log_file = ::core::ptr::null_mut::<FILE>();
    }
    json_logger.initialized = 0 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut json_logger.mutex);
    pthread_mutex_destroy(&raw mut json_logger.mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_json_log(
    mut level: log_level_t,
    mut timestamp: *const ::core::ffi::c_char,
    mut message: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if json_logger.initialized == 0 || json_logger.log_file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if level.0 < log_level_t::LOG_LEVEL_ERROR.0 || level.0 > log_level_t::LOG_LEVEL_DEBUG.0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut log_entry: *mut cJSON = cJSON_CreateObject();
    if log_entry.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    cJSON_AddStringToObject(
        log_entry,
        b"timestamp\0".as_ptr() as *const ::core::ffi::c_char,
        timestamp,
    );
    cJSON_AddStringToObject(
        log_entry,
        b"level\0".as_ptr() as *const ::core::ffi::c_char,
        json_log_level_strings[level.0 as usize],
    );
    let mut component: *const ::core::ffi::c_char = log_get_thread_component();
    let mut stream_name: *const ::core::ffi::c_char = log_get_thread_stream();
    if !component.is_null()
        && *component.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        cJSON_AddStringToObject(
            log_entry,
            b"component\0".as_ptr() as *const ::core::ffi::c_char,
            component,
        );
    }
    if !stream_name.is_null()
        && *stream_name.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        cJSON_AddStringToObject(
            log_entry,
            b"stream\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
    }
    cJSON_AddStringToObject(
        log_entry,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        message,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(log_entry);
    cJSON_Delete(log_entry);
    if json_str.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut json_logger.mutex);
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if fprintf(
        json_logger.log_file,
        b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        json_str,
    ) < 0 as ::core::ffi::c_int
    {
        result = -1 as ::core::ffi::c_int;
    }
    fflush(json_logger.log_file);
    pthread_mutex_unlock(&raw mut json_logger.mutex);
    free(json_str as *mut ::core::ffi::c_void);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_json_logs(
    mut min_level: *const ::core::ffi::c_char,
    mut last_timestamp: *const ::core::ffi::c_char,
    mut logs: *mut *mut *mut ::core::ffi::c_char,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if json_logger.initialized == 0 || json_logger.log_file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *logs = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    *count = 0 as ::core::ffi::c_int;
    let mut min_value: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    if strcmp(min_level, b"error\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        min_value = 0 as ::core::ffi::c_int;
    } else if strcmp(
        min_level,
        b"warning\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        min_value = 1 as ::core::ffi::c_int;
    } else if strcmp(min_level, b"info\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        min_value = 2 as ::core::ffi::c_int;
    } else if strcmp(min_level, b"debug\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        min_value = 3 as ::core::ffi::c_int;
    }
    let mut file: *mut FILE = fopen(
        &raw mut json_logger.log_filename as *mut ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    fseek(file, 0 as ::core::ffi::c_long, SEEK_END);
    let mut file_size: ::core::ffi::c_long = ftell(file);
    if file_size < 0 as ::core::ffi::c_long {
        fclose(file);
        return -1 as ::core::ffi::c_int;
    }
    let max_size: ::core::ffi::c_long = 100 as ::core::ffi::c_long * 1024 as ::core::ffi::c_long;
    let mut read_size: ::core::ffi::c_long = file_size;
    let mut offset: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    if file_size > max_size {
        read_size = max_size;
        offset = file_size - max_size;
    }
    if read_size == 0 as ::core::ffi::c_long {
        fclose(file);
        return 0 as ::core::ffi::c_int;
    }
    let mut buffer: *mut ::core::ffi::c_char =
        malloc((read_size as size_t).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if buffer.is_null() {
        fclose(file);
        return -1 as ::core::ffi::c_int;
    }
    fseek(file, offset, SEEK_SET);
    let mut bytes_read: size_t = fread(
        buffer as *mut ::core::ffi::c_void,
        1 as size_t,
        read_size as size_t,
        file,
    ) as size_t;
    if bytes_read > read_size as size_t {
        bytes_read = read_size as size_t;
    }
    *buffer.offset(bytes_read as isize) = '\0' as ::core::ffi::c_char;
    fclose(file);
    let mut line_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while i < bytes_read {
        if *buffer.offset(i as isize) as ::core::ffi::c_int == '\n' as ::core::ffi::c_int {
            line_count += 1;
        }
        i = i.wrapping_add(1);
    }
    if bytes_read > 0 as size_t
        && *buffer.offset(bytes_read.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            != '\n' as ::core::ffi::c_int
    {
        line_count += 1;
    }
    if line_count == 0 as ::core::ffi::c_int {
        free(buffer as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    let mut log_lines: *mut *mut ::core::ffi::c_char = malloc(
        (line_count as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
    ) as *mut *mut ::core::ffi::c_char;
    if log_lines.is_null() {
        free(buffer as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut line: *mut ::core::ffi::c_char = strtok_r(
        buffer,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    let mut log_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut compare_timestamp: ::core::ffi::c_int = (!last_timestamp.is_null()
        && *last_timestamp.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    while !line.is_null() && log_index < line_count {
        let mut log_entry: *mut cJSON = cJSON_Parse(line);
        if !log_entry.is_null() {
            let mut timestamp_json: *mut cJSON = cJSON_GetObjectItem(
                log_entry,
                b"timestamp\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut level_json: *mut cJSON =
                cJSON_GetObjectItem(log_entry, b"level\0".as_ptr() as *const ::core::ffi::c_char);
            let mut message_json: *mut cJSON = cJSON_GetObjectItem(
                log_entry,
                b"message\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !timestamp_json.is_null()
                && cJSON_IsString(timestamp_json) != 0
                && !level_json.is_null()
                && cJSON_IsString(level_json) != 0
                && !message_json.is_null()
                && cJSON_IsString(message_json) != 0
            {
                let mut timestamp: *const ::core::ffi::c_char = (*timestamp_json).valuestring;
                let mut level: *const ::core::ffi::c_char = (*level_json).valuestring;
                if compare_timestamp != 0
                    && strcmp(timestamp, last_timestamp) <= 0 as ::core::ffi::c_int
                {
                    cJSON_Delete(log_entry);
                    line = strtok_r(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut saveptr,
                    );
                    continue;
                } else {
                    let mut level_value: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
                    if strcmp(level, b"error\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        level_value = 0 as ::core::ffi::c_int;
                    } else if strcmp(level, b"warning\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        level_value = 1 as ::core::ffi::c_int;
                    } else if strcmp(level, b"info\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        level_value = 2 as ::core::ffi::c_int;
                    } else if strcmp(level, b"debug\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        level_value = 3 as ::core::ffi::c_int;
                    }
                    if level_value <= min_value {
                        *log_lines.offset(log_index as isize) = strdup(line);
                        if (*log_lines.offset(log_index as isize)).is_null() {
                            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_0 < log_index {
                                free(*log_lines.offset(i_0 as isize) as *mut ::core::ffi::c_void);
                                i_0 += 1;
                            }
                            free(log_lines as *mut ::core::ffi::c_void);
                            free(buffer as *mut ::core::ffi::c_void);
                            cJSON_Delete(log_entry);
                            return -1 as ::core::ffi::c_int;
                        }
                        log_index += 1;
                    }
                }
            }
            cJSON_Delete(log_entry);
        }
        line = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b"\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    *logs = log_lines;
    *count = log_index;
    free(buffer as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn json_log_rotate(
    mut max_size: size_t,
    mut max_files: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if json_logger.initialized == 0
        || json_logger.log_filename[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut json_logger.mutex);
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
        &raw mut json_logger.log_filename as *mut ::core::ffi::c_char,
        &raw mut st,
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(&raw mut json_logger.mutex);
        return -1 as ::core::ffi::c_int;
    }
    if (st.st_size as size_t) < max_size {
        pthread_mutex_unlock(&raw mut json_logger.mutex);
        return 0 as ::core::ffi::c_int;
    }
    if !json_logger.log_file.is_null() {
        fclose(json_logger.log_file);
        json_logger.log_file = ::core::ptr::null_mut::<FILE>();
    }
    let mut old_path: [::core::ffi::c_char; 512] = [0; 512];
    let mut new_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut old_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s.%d\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut json_logger.log_filename as *mut ::core::ffi::c_char,
        max_files,
    );
    unlink(&raw mut old_path as *mut ::core::ffi::c_char);
    let mut i: ::core::ffi::c_int = max_files - 1 as ::core::ffi::c_int;
    while i > 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut old_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s.%d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut json_logger.log_filename as *mut ::core::ffi::c_char,
            i,
        );
        snprintf(
            &raw mut new_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s.%d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut json_logger.log_filename as *mut ::core::ffi::c_char,
            i + 1 as ::core::ffi::c_int,
        );
        rename(
            &raw mut old_path as *mut ::core::ffi::c_char,
            &raw mut new_path as *mut ::core::ffi::c_char,
        );
        i -= 1;
    }
    snprintf(
        &raw mut new_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s.1\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut json_logger.log_filename as *mut ::core::ffi::c_char,
    );
    rename(
        &raw mut json_logger.log_filename as *mut ::core::ffi::c_char,
        &raw mut new_path as *mut ::core::ffi::c_char,
    );
    let mut jrot_fd: ::core::ffi::c_int = open(
        &raw mut json_logger.log_filename as *mut ::core::ffi::c_char,
        O_WRONLY | O_CREAT | O_APPEND,
        0o640 as ::core::ffi::c_int,
    );
    json_logger.log_file = if jrot_fd >= 0 as ::core::ffi::c_int {
        fdopen(jrot_fd, b"a\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<FILE>()
    };
    if json_logger.log_file.is_null() {
        if jrot_fd >= 0 as ::core::ffi::c_int {
            close(jrot_fd);
        }
        pthread_mutex_unlock(&raw mut json_logger.mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut json_logger.mutex);
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
