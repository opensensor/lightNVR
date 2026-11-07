unsafe extern "C" {
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn db_get_system_setting(
        key: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_char,
        out_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_set_system_setting(
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
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
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const EVENT_INSTALLATION_UUID_KEY: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"event_installation_uuid\0")
};
static mut identity_mutex: pthread_mutex_t = pthread_mutex_t {
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
};
static mut identity_initialized: bool = r#false != 0;
static mut installation_source: [::core::ffi::c_char; 128] = [0; 128];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_identity_init() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut identity_mutex);
    if identity_initialized {
        pthread_mutex_unlock(&raw mut identity_mutex);
        return 0 as ::core::ffi::c_int;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [
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
    ];
    let mut read_result: ::core::ffi::c_int = db_get_system_setting(
        EVENT_INSTALLATION_UUID_KEY.as_ptr(),
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>() as ::core::ffi::c_int,
    );
    if read_result != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(&raw mut uuid as *mut ::core::ffi::c_char)
    {
        if read_result == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Replacing invalid persisted event installation UUID\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
            || db_set_system_setting(
                EVENT_INSTALLATION_UUID_KEY.as_ptr(),
                &raw mut uuid as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to create persistent event installation identity\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            pthread_mutex_unlock(&raw mut identity_mutex);
            return -1 as ::core::ffi::c_int;
        }
    }
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut installation_source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        b"urn:lightnvr:%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
    {
        installation_source[0usize] = '\0' as ::core::ffi::c_char;
        pthread_mutex_unlock(&raw mut identity_mutex);
        return -1 as ::core::ffi::c_int;
    }
    identity_initialized = r#true != 0;
    pthread_mutex_unlock(&raw mut identity_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_identity_get_source(
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    if output.is_null() || output_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut identity_mutex);
    if !identity_initialized
        || installation_source[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strlen(&raw mut installation_source as *mut ::core::ffi::c_char) >= output_size
    {
        *output.offset(0isize) = '\0' as ::core::ffi::c_char;
        pthread_mutex_unlock(&raw mut identity_mutex);
        return -1 as ::core::ffi::c_int;
    }
    snprintf(
        output,
        output_size,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut installation_source as *mut ::core::ffi::c_char,
    );
    pthread_mutex_unlock(&raw mut identity_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_identity_shutdown() {
    pthread_mutex_lock(&raw mut identity_mutex);
    memset(
        &raw mut installation_source as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    );
    identity_initialized = r#false != 0;
    pthread_mutex_unlock(&raw mut identity_mutex);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
