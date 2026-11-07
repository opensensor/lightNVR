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
    unsafe fn dlopen(
        __file: *const ::core::ffi::c_char,
        __mode: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn dlclose(__handle: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    unsafe fn dlsym(
        __handle: *mut ::core::ffi::c_void,
        __name: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn dlerror() -> *mut ::core::ffi::c_char;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
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
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
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
pub struct sod_realnet_functions_t {
    pub handle: *mut ::core::ffi::c_void,
    pub sod_realnet_create:
        Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub sod_realnet_load_model_from_mem: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            ::core::ffi::c_uint,
            *mut ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >,
    pub sod_realnet_model_config: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_uint,
            ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int,
    >,
    pub sod_realnet_detect: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_uchar,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub sod_realnet_destroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sod_box_dynamic {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub score: ::core::ffi::c_float,
    pub zName: *const ::core::ffi::c_char,
    pub pUserData: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sod_realnet_model_t {
    pub net: *mut ::core::ffi::c_void,
    pub threshold: ::core::ffi::c_float,
}
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const RTLD_LAZY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MAX_LABEL_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut sod_realnet_funcs: sod_realnet_functions_t = sod_realnet_functions_t {
    handle: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    sod_realnet_create: None,
    sod_realnet_load_model_from_mem: None,
    sod_realnet_model_config: None,
    sod_realnet_detect: None,
    sod_realnet_destroy: None,
};
static mut sod_realnet_available: bool = r#false != 0;
unsafe extern "C" fn init_sod_realnet_functions() -> bool {
    if !sod_realnet_funcs.handle.is_null() {
        return sod_realnet_available;
    }
    sod_realnet_funcs.handle = dlopen(
        b"libsod.so\0".as_ptr() as *const ::core::ffi::c_char,
        RTLD_LAZY,
    );
    if sod_realnet_funcs.handle.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"SOD library not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            dlerror(),
        );
        return r#false != 0;
    }
    sod_realnet_funcs.sod_realnet_create = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    >(dlsym(
        sod_realnet_funcs.handle,
        b"sod_realnet_create\0".as_ptr() as *const ::core::ffi::c_char,
    ));
    sod_realnet_funcs.sod_realnet_load_model_from_mem = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                ::core::ffi::c_uint,
                *mut ::core::ffi::c_uint,
            ) -> ::core::ffi::c_int,
        >,
    >(dlsym(
        sod_realnet_funcs.handle,
        b"sod_realnet_load_model_from_mem\0".as_ptr() as *const ::core::ffi::c_char,
    ));
    sod_realnet_funcs.sod_realnet_model_config = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_uint,
                ::core::ffi::c_int,
                ...
            ) -> ::core::ffi::c_int,
        >,
    >(dlsym(
        sod_realnet_funcs.handle,
        b"sod_realnet_model_config\0".as_ptr() as *const ::core::ffi::c_char,
    ));
    sod_realnet_funcs.sod_realnet_detect = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_uchar,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut *mut *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
    >(dlsym(
        sod_realnet_funcs.handle,
        b"sod_realnet_detect\0".as_ptr() as *const ::core::ffi::c_char,
    ));
    sod_realnet_funcs.sod_realnet_destroy = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    >(dlsym(
        sod_realnet_funcs.handle,
        b"sod_realnet_destroy\0".as_ptr() as *const ::core::ffi::c_char,
    ));
    if (&raw const sod_realnet_funcs.sod_realnet_create).read().is_some()
        && (&raw const sod_realnet_funcs.sod_realnet_load_model_from_mem).read().is_some()
        && (&raw const sod_realnet_funcs.sod_realnet_model_config).read().is_some()
        && (&raw const sod_realnet_funcs.sod_realnet_detect).read().is_some()
        && (&raw const sod_realnet_funcs.sod_realnet_destroy).read().is_some()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"SOD RealNet functions dynamically loaded\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sod_realnet_available = r#true != 0;
        return r#true != 0;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"SOD library found but missing required RealNet functions\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        dlclose(sod_realnet_funcs.handle);
        sod_realnet_funcs.handle = NULL;
        return r#false != 0;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_sod_realnet_model(
    mut model_path: *const ::core::ffi::c_char,
    mut threshold: ::core::ffi::c_float,
) -> *mut ::core::ffi::c_void {
    if !init_sod_realnet_functions() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"SOD RealNet functions not available\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    let mut net: *mut ::core::ffi::c_void = NULL;
    let mut rc: ::core::ffi::c_int = 0;
    rc = sod_realnet_funcs
        .sod_realnet_create
        .expect("non-null function pointer")(&raw mut net);
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create SOD RealNet handle: %d\0".as_ptr() as *const ::core::ffi::c_char,
            rc,
        );
        return NULL;
    }
    let mut handle: ::core::ffi::c_uint = 0;
    let mut fp: *mut FILE = fopen(model_path, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
    if fp.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open SOD RealNet model file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            model_path,
        );
        sod_realnet_funcs
            .sod_realnet_destroy
            .expect("non-null function pointer")(net);
        return NULL;
    }
    fseek(fp, 0 as ::core::ffi::c_long, SEEK_END);
    let mut file_size: ::core::ffi::c_long = ftell(fp);
    fseek(fp, 0 as ::core::ffi::c_long, SEEK_SET);
    let mut model_data: *mut ::core::ffi::c_void = malloc(file_size as size_t);
    if model_data.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for SOD RealNet model data\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fclose(fp);
        sod_realnet_funcs
            .sod_realnet_destroy
            .expect("non-null function pointer")(net);
        return NULL;
    }
    if fread(model_data, 1 as size_t, file_size as size_t, fp) != file_size as ::core::ffi::c_ulong
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to read SOD RealNet model data\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(model_data);
        fclose(fp);
        sod_realnet_funcs
            .sod_realnet_destroy
            .expect("non-null function pointer")(net);
        return NULL;
    }
    fclose(fp);
    rc = sod_realnet_funcs
        .sod_realnet_load_model_from_mem
        .expect("non-null function pointer")(
        net,
        model_data,
        file_size as ::core::ffi::c_uint,
        &raw mut handle,
    );
    free(model_data);
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to load SOD RealNet model: %s (error: %d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            model_path,
            rc,
        );
        sod_realnet_funcs
            .sod_realnet_destroy
            .expect("non-null function pointer")(net);
        return NULL;
    }
    let mut model: *mut sod_realnet_model_t =
        malloc(::core::mem::size_of::<sod_realnet_model_t>()) as *mut sod_realnet_model_t;
    if model.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for SOD RealNet model structure\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        sod_realnet_funcs
            .sod_realnet_destroy
            .expect("non-null function pointer")(net);
        return NULL;
    }
    (*model).net = net;
    (*model).threshold = threshold;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"SOD RealNet model loaded: %s\0".as_ptr() as *const ::core::ffi::c_char,
        model_path,
    );
    return model as *mut ::core::ffi::c_void;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_sod_realnet_model(mut model: *mut ::core::ffi::c_void) {
    if model.is_null() {
        return;
    }
    let mut m: *mut sod_realnet_model_t = model as *mut sod_realnet_model_t;
    if (&raw const sod_realnet_funcs.sod_realnet_destroy).read().is_some() {
        sod_realnet_funcs
            .sod_realnet_destroy
            .expect("non-null function pointer")((*m).net);
    }
    free(m as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_with_sod_realnet(
    mut model: *mut ::core::ffi::c_void,
    mut frame_data: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut channels: ::core::ffi::c_int,
    mut result: *mut detection_result_t,
) -> ::core::ffi::c_int {
    if model.is_null() || frame_data.is_null() || result.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if !init_sod_realnet_functions() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"SOD RealNet functions not available\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut m: *mut sod_realnet_model_t = model as *mut sod_realnet_model_t;
    (*result).count = 0 as ::core::ffi::c_int;
    let mut blob: *mut ::core::ffi::c_uchar = malloc(
        (width as size_t)
            .wrapping_mul(height as size_t)
            .wrapping_mul(channels as size_t),
    ) as *mut ::core::ffi::c_uchar;
    if blob.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for frame blob\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memcpy(
        blob as *mut ::core::ffi::c_void,
        frame_data as *const ::core::ffi::c_void,
        (width as size_t)
            .wrapping_mul(height as size_t)
            .wrapping_mul(channels as size_t),
    );
    let mut boxes: *mut ::core::ffi::c_void = NULL;
    let mut box_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = 0;
    rc = sod_realnet_funcs
        .sod_realnet_detect
        .expect("non-null function pointer")(
        (*m).net,
        blob,
        width,
        height,
        &raw mut boxes as *mut *mut *mut ::core::ffi::c_void,
        &raw mut box_count,
    );
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"SOD RealNet detection failed: %d\0".as_ptr() as *const ::core::ffi::c_char,
            rc,
        );
        free(blob as *mut ::core::ffi::c_void);
        blob = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        return -1 as ::core::ffi::c_int;
    }
    let mut valid_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < box_count && valid_count < MAX_DETECTIONS {
        let mut box_array: *mut sod_box_dynamic = boxes as *mut sod_box_dynamic;
        let mut score: ::core::ffi::c_float = (*box_array.offset(i as isize)).score;
        let mut name: *const ::core::ffi::c_char = (*box_array.offset(i as isize)).zName;
        let mut x: ::core::ffi::c_int = (*box_array.offset(i as isize)).x;
        let mut y: ::core::ffi::c_int = (*box_array.offset(i as isize)).y;
        let mut w: ::core::ffi::c_int = (*box_array.offset(i as isize)).w;
        let mut h: ::core::ffi::c_int = (*box_array.offset(i as isize)).h;
        if score >= (*m).threshold {
            safe_strcpy(
                &raw mut (*(&raw mut (*result).detections as *mut detection_t)
                    .offset(valid_count as isize))
                .label as *mut ::core::ffi::c_char,
                if !name.is_null() {
                    name
                } else {
                    b"face\0".as_ptr() as *const ::core::ffi::c_char
                },
                MAX_LABEL_LENGTH as size_t,
                0 as size_t,
            );
            (*result).detections[valid_count as usize].confidence =
                (score as ::core::ffi::c_double / 10.0f64) as ::core::ffi::c_float;
            if (*result).detections[valid_count as usize].confidence > 1.0f32 {
                (*result).detections[valid_count as usize].confidence = 1.0f32;
            }
            (*result).detections[valid_count as usize].x =
                x as ::core::ffi::c_float / width as ::core::ffi::c_float;
            (*result).detections[valid_count as usize].y =
                y as ::core::ffi::c_float / height as ::core::ffi::c_float;
            (*result).detections[valid_count as usize].width =
                w as ::core::ffi::c_float / width as ::core::ffi::c_float;
            (*result).detections[valid_count as usize].height =
                h as ::core::ffi::c_float / height as ::core::ffi::c_float;
            valid_count += 1;
        }
        i += 1;
    }
    (*result).count = valid_count;
    free(blob as *mut ::core::ffi::c_void);
    blob = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
