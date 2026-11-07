unsafe extern "C" {
    pub type sod_cnn;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn is_shutdown_initiated() -> bool;
    unsafe fn get_total_memory_allocated() -> size_t;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn ends_with(
        str: *const ::core::ffi::c_char,
        suffix: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn init_sod_detection_system() -> ::core::ffi::c_int;
    unsafe fn shutdown_sod_detection_system();
    unsafe fn load_sod_model(
        model_path: *const ::core::ffi::c_char,
        threshold: ::core::ffi::c_float,
    ) -> detection_model_t;
    unsafe fn is_sod_available() -> bool;
    unsafe fn cleanup_sod_model(model: detection_model_t);
    unsafe fn load_sod_realnet_model(
        model_path: *const ::core::ffi::c_char,
        threshold: ::core::ffi::c_float,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn free_sod_realnet_model(model: *mut ::core::ffi::c_void);
    unsafe fn sod_cnn_destroy(pNet: *mut sod_cnn);
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
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
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
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
pub type detection_model_t = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct model_t {
    pub r#type: [::core::ffi::c_char; 16],
    pub c2rust_unnamed: C2Rust_Unnamed,
    pub threshold: ::core::ffi::c_float,
    pub path: [::core::ffi::c_char; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub sod: *mut ::core::ffi::c_void,
    pub sod_realnet: *mut ::core::ffi::c_void,
    pub tflite: tflite_model_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tflite_model_t {
    pub engine: *mut ::core::ffi::c_void,
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MODEL_TYPE_SOD: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"sod\0") };
pub const MODEL_TYPE_SOD_REALNET: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"sod_realnet\0") };
pub const MODEL_TYPE_TFLITE: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"tflite\0") };
pub const MODEL_TYPE_MOTION: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"motion\0") };
pub const MODEL_TYPE_API: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"api\0") };
pub const MODEL_TYPE_ONVIF: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"onvif\0") };
static mut in_shutdown_mode: bool = r#false != 0;
pub const MAX_MODEL_SIZE_MB: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
static mut initialized: bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_detection_model_system() -> ::core::ffi::c_int {
    if initialized {
        return 0 as ::core::ffi::c_int;
    }
    let mut sod_ret: ::core::ffi::c_int = init_sod_detection_system();
    if sod_ret != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize SOD detection system\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"In-process LiteRT detection engine NOT compiled in (rebuild with -DENABLE_LITERT=ON to enable)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    initialized = r#true != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Detection model system initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_detection_model_system() {
    if !initialized {
        return;
    }
    in_shutdown_mode = r#true != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Detection model system shutting down\0".as_ptr() as *const ::core::ffi::c_char,
    );
    shutdown_sod_detection_system();
    initialized = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Detection model system shutdown\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_model_supported(mut model_path: *const ::core::ffi::c_char) -> bool {
    if model_path.is_null() {
        return r#false != 0;
    }
    let mut ext: *const ::core::ffi::c_char = strrchr(model_path, '.' as ::core::ffi::c_int);
    if ext.is_null() {
        return r#false != 0;
    }
    if !strstr(
        model_path,
        b".realnet.sod\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        return is_sod_available();
    }
    if strcasecmp(ext, b".sod\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return is_sod_available();
    }
    if strcasecmp(ext, b".tflite\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_model_type(
    mut model_path: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if model_path.is_null() {
        return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if ends_with(
        model_path,
        b"api-detection\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        return MODEL_TYPE_API.as_ptr();
    }
    if strncmp(
        model_path,
        b"http://\0".as_ptr() as *const ::core::ffi::c_char,
        7 as size_t,
    ) == 0 as ::core::ffi::c_int
        || strncmp(
            model_path,
            b"https://\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        return MODEL_TYPE_API.as_ptr();
    }
    if ends_with(
        model_path,
        b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        return MODEL_TYPE_ONVIF.as_ptr();
    }
    if ends_with(
        model_path,
        b"motion\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        return MODEL_TYPE_MOTION.as_ptr();
    }
    let mut ext: *const ::core::ffi::c_char = strrchr(model_path, '.' as ::core::ffi::c_int);
    if ext.is_null() {
        return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if !strstr(
        model_path,
        b".realnet.sod\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        return if is_sod_available() as ::core::ffi::c_int != 0 {
            MODEL_TYPE_SOD_REALNET.as_ptr()
        } else {
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        };
    }
    if strcasecmp(ext, b".sod\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return if is_sod_available() as ::core::ffi::c_int != 0 {
            MODEL_TYPE_SOD.as_ptr()
        } else {
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        };
    }
    if strcasecmp(ext, b".tflite\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return MODEL_TYPE_TFLITE.as_ptr();
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn load_tflite_model(
    mut model_path: *const ::core::ffi::c_char,
    mut threshold: ::core::ffi::c_float,
) -> detection_model_t {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"LiteRT not compiled in; cannot load .tflite model %s (rebuild with -DENABLE_LITERT=ON)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        model_path,
    );
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_model_path(
    mut model: detection_model_t,
) -> *const ::core::ffi::c_char {
    if model.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut m: *const model_t = model as *const model_t;
    return &raw const (*m).path as *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_realnet_model_handle(
    mut model: detection_model_t,
) -> *mut ::core::ffi::c_void {
    if model.is_null() {
        return NULL;
    }
    let mut m: *mut model_t = model as *mut model_t;
    if strcmp(
        &raw mut (*m).r#type as *mut ::core::ffi::c_char,
        MODEL_TYPE_SOD_REALNET.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        return (*m).c2rust_unnamed.sod_realnet;
    }
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_model_type_from_handle(
    mut model: detection_model_t,
) -> *const ::core::ffi::c_char {
    if model.is_null() {
        return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }
    let mut m: *const model_t = model as *const model_t;
    return &raw const (*m).r#type as *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_old_detection_models(mut max_age: time_t) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Thread-local model approach: No global cache to clean. Current memory usage: %zu bytes\0"
            .as_ptr() as *const ::core::ffi::c_char,
        get_total_memory_allocated(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_detection_model(
    mut model_path: *const ::core::ffi::c_char,
    mut threshold: ::core::ffi::c_float,
) -> detection_model_t {
    if model_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid model path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    let mut is_api_detection: bool = ends_with(
        model_path,
        b"api-detection\0".as_ptr() as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || strncmp(
            model_path,
            b"http://\0".as_ptr() as *const ::core::ffi::c_char,
            7 as size_t,
        ) == 0 as ::core::ffi::c_int
        || strncmp(
            model_path,
            b"https://\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int;
    let mut is_onvif_detection: bool = ends_with(
        model_path,
        b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut is_motion_detection: bool = ends_with(
        model_path,
        b"motion\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if is_api_detection {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"API DETECTION: Using API for detection instead of a local model file\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if is_onvif_detection {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"ONVIF DETECTION: Using ONVIF for detection instead of a local model file\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if is_motion_detection {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MOTION DETECTION: Using built-in motion detection instead of a local model file\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
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
        if stat(model_path, &raw mut st) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"MODEL FILE NOT FOUND: %s\0".as_ptr() as *const ::core::ffi::c_char,
                model_path,
            );
            return NULL;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MODEL FILE EXISTS: %s\0".as_ptr() as *const ::core::ffi::c_char,
            model_path,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MODEL FILE SIZE: %ld bytes\0".as_ptr() as *const ::core::ffi::c_char,
            st.st_size,
        );
        let mut model_size_mb: ::core::ffi::c_double = st.st_size as ::core::ffi::c_double
            / (1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as ::core::ffi::c_double;
        if model_size_mb > MAX_MODEL_SIZE_MB as ::core::ffi::c_double {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Large model detected: %.1f MB (limit: %d MB)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                model_size_mb,
                50 as ::core::ffi::c_int,
            );
        }
    }
    let mut model_type: *const ::core::ffi::c_char = get_model_type(model_path);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MODEL TYPE: %s\0".as_ptr() as *const ::core::ffi::c_char,
        model_type,
    );
    let mut model: detection_model_t = NULL;
    if strcmp(model_type, MODEL_TYPE_API.as_ptr()) == 0 as ::core::ffi::c_int {
        let mut m: *mut model_t = malloc(::core::mem::size_of::<model_t>()) as *mut model_t;
        if !m.is_null() {
            safe_strcpy(
                &raw mut (*m).r#type as *mut ::core::ffi::c_char,
                MODEL_TYPE_API.as_ptr(),
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            (*m).c2rust_unnamed.sod = NULL;
            (*m).threshold = threshold;
            safe_strcpy(
                &raw mut (*m).path as *mut ::core::ffi::c_char,
                model_path,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
            model = m as detection_model_t;
        }
    } else if strcmp(model_type, MODEL_TYPE_ONVIF.as_ptr()) == 0 as ::core::ffi::c_int {
        let mut m_0: *mut model_t = malloc(::core::mem::size_of::<model_t>()) as *mut model_t;
        if !m_0.is_null() {
            safe_strcpy(
                &raw mut (*m_0).r#type as *mut ::core::ffi::c_char,
                MODEL_TYPE_ONVIF.as_ptr(),
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            (*m_0).c2rust_unnamed.sod = NULL;
            (*m_0).threshold = threshold;
            safe_strcpy(
                &raw mut (*m_0).path as *mut ::core::ffi::c_char,
                model_path,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
            model = m_0 as detection_model_t;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"ONVIF model created: %s\0".as_ptr() as *const ::core::ffi::c_char,
                model_path,
            );
        }
    } else if strcmp(model_type, MODEL_TYPE_MOTION.as_ptr()) == 0 as ::core::ffi::c_int {
        let mut m_1: *mut model_t = malloc(::core::mem::size_of::<model_t>()) as *mut model_t;
        if !m_1.is_null() {
            safe_strcpy(
                &raw mut (*m_1).r#type as *mut ::core::ffi::c_char,
                MODEL_TYPE_MOTION.as_ptr(),
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            (*m_1).c2rust_unnamed.sod = NULL;
            (*m_1).threshold = threshold;
            safe_strcpy(
                &raw mut (*m_1).path as *mut ::core::ffi::c_char,
                model_path,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
            model = m_1 as detection_model_t;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Built-in motion detection model handle created\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    } else if strcmp(model_type, MODEL_TYPE_SOD_REALNET.as_ptr()) == 0 as ::core::ffi::c_int {
        let mut realnet_model: *mut ::core::ffi::c_void =
            load_sod_realnet_model(model_path, threshold);
        if !realnet_model.is_null() {
            let mut m_2: *mut model_t = malloc(::core::mem::size_of::<model_t>()) as *mut model_t;
            if !m_2.is_null() {
                safe_strcpy(
                    &raw mut (*m_2).r#type as *mut ::core::ffi::c_char,
                    MODEL_TYPE_SOD_REALNET.as_ptr(),
                    ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                    0 as size_t,
                );
                (*m_2).c2rust_unnamed.sod_realnet = realnet_model;
                (*m_2).threshold = threshold;
                safe_strcpy(
                    &raw mut (*m_2).path as *mut ::core::ffi::c_char,
                    model_path,
                    MAX_PATH_LENGTH as size_t,
                    0 as size_t,
                );
                model = m_2 as detection_model_t;
            } else {
                free_sod_realnet_model(realnet_model);
            }
        }
    } else if strcmp(model_type, MODEL_TYPE_SOD.as_ptr()) == 0 as ::core::ffi::c_int {
        model = load_sod_model(model_path, threshold);
    } else if strcmp(model_type, MODEL_TYPE_TFLITE.as_ptr()) == 0 as ::core::ffi::c_int {
        model = load_tflite_model(model_path, threshold);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unsupported model type: %s\0".as_ptr() as *const ::core::ffi::c_char,
            model_type,
        );
    }
    if !model.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Successfully loaded model: %s\0".as_ptr() as *const ::core::ffi::c_char,
            model_path,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to load model: %s\0".as_ptr() as *const ::core::ffi::c_char,
            model_path,
        );
    }
    return model;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unload_detection_model(mut model: detection_model_t) {
    if model.is_null() {
        return;
    }
    let mut m: *mut model_t = model as *mut model_t;
    let mut model_path: [::core::ffi::c_char; 512] = [0; 512];
    if (*m).path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut model_path as *mut ::core::ffi::c_char,
            &raw mut (*m).path as *mut ::core::ffi::c_char,
            MAX_PATH_LENGTH as size_t,
            0 as size_t,
        );
    } else {
        strcpy(
            &raw mut model_path as *mut ::core::ffi::c_char,
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        &raw mut (*m).r#type as *mut ::core::ffi::c_char,
        MODEL_TYPE_API.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unloading API model: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut model_path as *mut ::core::ffi::c_char,
        );
        (*m).c2rust_unnamed.sod = NULL;
    } else if strcmp(
        &raw mut (*m).r#type as *mut ::core::ffi::c_char,
        MODEL_TYPE_ONVIF.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unloading ONVIF model: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut model_path as *mut ::core::ffi::c_char,
        );
        (*m).c2rust_unnamed.sod = NULL;
    } else if strcmp(
        &raw mut (*m).r#type as *mut ::core::ffi::c_char,
        MODEL_TYPE_MOTION.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unloading built-in motion detection model: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut model_path as *mut ::core::ffi::c_char,
        );
        (*m).c2rust_unnamed.sod = NULL;
    } else if strcmp(
        &raw mut (*m).r#type as *mut ::core::ffi::c_char,
        MODEL_TYPE_SOD.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using cleanup_sod_model for safer SOD model cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut model_copy: *mut model_t =
            malloc(::core::mem::size_of::<model_t>()) as *mut model_t;
        if !model_copy.is_null() {
            memcpy(
                model_copy as *mut ::core::ffi::c_void,
                m as *const ::core::ffi::c_void,
                ::core::mem::size_of::<model_t>(),
            );
            (*m).c2rust_unnamed.sod = NULL;
            free(m as *mut ::core::ffi::c_void);
            cleanup_sod_model(model_copy as detection_model_t);
            return;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to allocate memory for model copy, falling back to direct cleanup\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            let mut sod_model: *mut ::core::ffi::c_void = (*m).c2rust_unnamed.sod;
            if !sod_model.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Destroying SOD model\0".as_ptr() as *const ::core::ffi::c_char,
                );
                sod_cnn_destroy(sod_model as *mut sod_cnn);
                (*m).c2rust_unnamed.sod = NULL;
            }
        }
    } else if strcmp(
        &raw mut (*m).r#type as *mut ::core::ffi::c_char,
        MODEL_TYPE_SOD_REALNET.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        if !(*m).c2rust_unnamed.sod_realnet.is_null() {
            if is_shutdown_initiated() as ::core::ffi::c_int != 0
                || in_shutdown_mode as ::core::ffi::c_int != 0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"MEMORY LEAK FIX: Destroying SOD RealNet model during shutdown\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                free_sod_realnet_model((*m).c2rust_unnamed.sod_realnet);
            } else {
                free_sod_realnet_model((*m).c2rust_unnamed.sod_realnet);
            }
            (*m).c2rust_unnamed.sod_realnet = NULL;
        }
    } else {
        strcmp(
            &raw mut (*m).r#type as *mut ::core::ffi::c_char,
            MODEL_TYPE_TFLITE.as_ptr(),
        ) == 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unloading model: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut model_path as *mut ::core::ffi::c_char,
    );
    free(m as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn force_cleanup_model_cache() {
    in_shutdown_mode = r#true != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Thread-local model approach: No global cache to clean up\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
