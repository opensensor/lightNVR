unsafe extern "C" {
    pub type faac_encoder;
    unsafe fn faac_params_init(params: *mut faac_params, caller_size: uint32_t) -> faac_status;
    unsafe fn faac_encoder_open(
        config: *const faac_params,
        out: *mut *mut faac_encoder,
    ) -> faac_status;
    unsafe fn faac_encoder_close(encoder: *mut *mut faac_encoder) -> faac_status;
    unsafe fn faac_encoder_get_info(
        encoder: *const faac_encoder,
        out_info: *mut faac_encoder_info,
    ) -> faac_status;
    unsafe fn faac_encoder_asc(
        encoder: *const faac_encoder,
        buf: *mut *const uint8_t,
        len: *mut uint32_t,
    ) -> faac_status;
    unsafe fn faac_encoder_encode(
        encoder: *mut faac_encoder,
        r#in: *const ::core::ffi::c_void,
        in_samples: uint32_t,
        out: *mut uint8_t,
        out_cap: uint32_t,
        bytes_written: *mut uint32_t,
    ) -> faac_status;
    unsafe fn faac_strerror(status: faac_status) -> *const ::core::ffi::c_char;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct faac_status(pub ::core::ffi::c_int);
impl faac_status {
    pub const FAAC_OK: Self = Self(0);
    pub const FAAC_ERR_INVALID_ARGUMENT: Self = Self(-1);
    pub const FAAC_ERR_UNSUPPORTED: Self = Self(-2);
    pub const FAAC_ERR_NO_MEMORY: Self = Self(-3);
    pub const FAAC_ERR_OUTPUT_TOO_SMALL: Self = Self(-4);
    pub const FAAC_ERR_INPUT_OVERFLOW: Self = Self(-5);
    pub const FAAC_ERR_INTERNAL: Self = Self(-6);
    pub const FAAC_STATUS_MAX: Self = Self(2147483647);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct faac_object_type(pub ::core::ffi::c_uint);
impl faac_object_type {
    pub const FAAC_OBJ_AUTO: Self = Self(0);
    pub const FAAC_OBJ_LC: Self = Self(2);
    pub const FAAC_OBJ_HE_AAC_V1: Self = Self(5);
    pub const FAAC_OBJ_HE_AAC_V2: Self = Self(29);
    pub const FAAC_OBJ_MAX: Self = Self(2147483647);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct faac_stereo_mode(pub ::core::ffi::c_uint);
impl faac_stereo_mode {
    pub const FAAC_STEREO_LR: Self = Self(0);
    pub const FAAC_STEREO_MS: Self = Self(1);
    pub const FAAC_STEREO_IS: Self = Self(2);
    pub const FAAC_STEREO_AUTO: Self = Self(3);
    pub const FAAC_STEREO_MAX: Self = Self(2147483647);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct faac_shortctl_mode(pub ::core::ffi::c_uint);
impl faac_shortctl_mode {
    pub const FAAC_SHORTCTL_NORMAL: Self = Self(0);
    pub const FAAC_SHORTCTL_NOSHORT: Self = Self(1);
    pub const FAAC_SHORTCTL_NOLONG: Self = Self(2);
    pub const FAAC_SHORTCTL_MAX: Self = Self(2147483647);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct faac_stream_format(pub ::core::ffi::c_uint);
impl faac_stream_format {
    pub const FAAC_STREAM_RAW: Self = Self(0);
    pub const FAAC_STREAM_ADTS: Self = Self(1);
    pub const FAAC_STREAM_ADTS_MPEG2: Self = Self(2);
    pub const FAAC_STREAM_MAX: Self = Self(2147483647);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct faac_rate_control(pub ::core::ffi::c_uint);
impl faac_rate_control {
    pub const FAAC_RC_ABR: Self = Self(0);
    pub const FAAC_RC_VBR: Self = Self(1);
    pub const FAAC_RC_CBR: Self = Self(2);
    pub const FAAC_RC_MAX: Self = Self(2147483647);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct faac_input_format(pub ::core::ffi::c_uint);
impl faac_input_format {
    pub const FAAC_INPUT_NULL: Self = Self(0);
    pub const FAAC_INPUT_16BIT: Self = Self(1);
    pub const FAAC_INPUT_24BIT: Self = Self(2);
    pub const FAAC_INPUT_24IN32: Self = Self(3);
    pub const FAAC_INPUT_FLOAT: Self = Self(4);
    pub const FAAC_INPUT_MAX: Self = Self(2147483647);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct faac_channel_layout(pub ::core::ffi::c_uint);
impl faac_channel_layout {
    pub const FAAC_LAYOUT_AAC: Self = Self(0);
    pub const FAAC_LAYOUT_WAV: Self = Self(1);
    pub const FAAC_LAYOUT_MAX: Self = Self(2147483647);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct faac_params {
    pub struct_size: uint32_t,
    pub sample_rate: uint32_t,
    pub channels: uint32_t,
    pub object_type: faac_object_type,
    pub stereo_mode: faac_stereo_mode,
    pub flags: uint32_t,
    pub bit_rate: uint32_t,
    pub bandwidth: uint32_t,
    pub quality: uint32_t,
    pub stream_format: faac_stream_format,
    pub input_format: faac_input_format,
    pub short_control: faac_shortctl_mode,
    pub channel_layout: faac_channel_layout,
    pub peak_bit_rate: uint32_t,
    pub rate_control: faac_rate_control,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct faac_encoder_info {
    pub struct_size: uint32_t,
    pub frame_samples: uint32_t,
    pub max_output_bytes: uint32_t,
    pub object_type: faac_object_type,
    pub stream_format: faac_stream_format,
    pub bit_rate: uint32_t,
    pub bandwidth: uint32_t,
    pub quality: uint32_t,
    pub encoder_delay: uint32_t,
    pub asc_size: uint32_t,
    pub asc: [uint8_t; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct faac_adapter {
    pub enc: *mut faac_encoder,
    pub info: faac_encoder_info,
    pub asc_buf: *mut uint8_t,
    pub asc_size: size_t,
    pub num_channels: uint32_t,
    pub sample_rate: uint32_t,
}
pub type faac_adapter_t = faac_adapter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct faac_encoder_config {
    pub sample_rate: uint32_t,
    pub num_channels: uint32_t,
    pub bit_rate_per_channel: uint32_t,
    pub object_type: faac_object_type,
}
pub type faac_encoder_config_t = faac_encoder_config;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const LOG_COMPONENT: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"FAACEncoder\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn faac_adapter_open(
    mut config: *const faac_encoder_config_t,
) -> *mut faac_adapter_t {
    if config.is_null()
        || (*config).sample_rate == 0 as uint32_t
        || (*config).num_channels == 0 as uint32_t
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"faac_adapter_open: invalid configuration parameters\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<faac_adapter_t>();
    }
    let mut adapter: *mut faac_adapter_t =
        calloc(1 as size_t, ::core::mem::size_of::<faac_adapter_t>()) as *mut faac_adapter_t;
    if adapter.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"faac_adapter_open: memory allocation failed for adapter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<faac_adapter_t>();
    }
    (*adapter).num_channels = (*config).num_channels;
    (*adapter).sample_rate = (*config).sample_rate;
    let mut params: faac_params = faac_params {
        struct_size: 0,
        sample_rate: 0,
        channels: 0,
        object_type: faac_object_type::FAAC_OBJ_AUTO,
        stereo_mode: faac_stereo_mode::FAAC_STEREO_LR,
        flags: 0,
        bit_rate: 0,
        bandwidth: 0,
        quality: 0,
        stream_format: faac_stream_format::FAAC_STREAM_RAW,
        input_format: faac_input_format::FAAC_INPUT_NULL,
        short_control: faac_shortctl_mode::FAAC_SHORTCTL_NORMAL,
        channel_layout: faac_channel_layout::FAAC_LAYOUT_AAC,
        peak_bit_rate: 0,
        rate_control: faac_rate_control::FAAC_RC_ABR,
    };
    let mut status: faac_status = faac_params_init(
        &raw mut params,
        ::core::mem::size_of::<faac_params>() as uint32_t,
    );
    if status.0 != faac_status::FAAC_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"faac_params_init failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            faac_strerror(status),
        );
        free(adapter as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<faac_adapter_t>();
    }
    params.sample_rate = (*config).sample_rate;
    params.channels = (*config).num_channels;
    params.stream_format = faac_stream_format::FAAC_STREAM_RAW;
    params.input_format = faac_input_format::FAAC_INPUT_16BIT;
    params.object_type = (*config).object_type;
    let mut br_per_ch: uint32_t = (*config).bit_rate_per_channel;
    if br_per_ch == 0 as uint32_t {
        br_per_ch = (if (*config).sample_rate >= 32000 as uint32_t {
            64000 as ::core::ffi::c_int
        } else {
            32000 as ::core::ffi::c_int
        }) as uint32_t;
    }
    if br_per_ch > (UINT32_MAX as uint32_t).wrapping_div((*config).num_channels) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"faac_adapter_open: requested bitrate overflow (%u per channel x %u channels)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            br_per_ch,
            (*config).num_channels,
        );
        free(adapter as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<faac_adapter_t>();
    }
    params.bit_rate = br_per_ch.wrapping_mul((*config).num_channels);
    params.rate_control = faac_rate_control::FAAC_RC_ABR;
    status = faac_encoder_open(&raw mut params, &raw mut (*adapter).enc);
    if status.0 != faac_status::FAAC_OK.0 || (*adapter).enc.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"faac_encoder_open failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if status.0 != faac_status::FAAC_OK.0 {
                faac_strerror(status)
            } else {
                b"NULL encoder handle\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        free(adapter as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<faac_adapter_t>();
    }
    (*adapter).info.struct_size = ::core::mem::size_of::<faac_encoder_info>() as uint32_t;
    status = faac_encoder_get_info((*adapter).enc, &raw mut (*adapter).info);
    if status.0 != faac_status::FAAC_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"faac_encoder_get_info failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            faac_strerror(status),
        );
        faac_encoder_close(&raw mut (*adapter).enc);
        free(adapter as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<faac_adapter_t>();
    }
    let mut asc_ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut asc_len: uint32_t = 0 as uint32_t;
    status = faac_encoder_asc((*adapter).enc, &raw mut asc_ptr, &raw mut asc_len);
    if status.0 != faac_status::FAAC_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"faac_encoder_asc failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            faac_strerror(status),
        );
        faac_encoder_close(&raw mut (*adapter).enc);
        free(adapter as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<faac_adapter_t>();
    } else if asc_ptr.is_null() || asc_len == 0 as uint32_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"faac_encoder_asc returned invalid AudioSpecificConfig (ptr=%p, len=%u)\0".as_ptr()
                as *const ::core::ffi::c_char,
            asc_ptr as *const ::core::ffi::c_void,
            asc_len,
        );
        faac_encoder_close(&raw mut (*adapter).enc);
        free(adapter as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<faac_adapter_t>();
    }
    (*adapter).asc_buf = malloc(asc_len as size_t) as *mut uint8_t;
    if (*adapter).asc_buf.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"faac_adapter_open: memory allocation failed for AudioSpecificConfig\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        faac_encoder_close(&raw mut (*adapter).enc);
        free(adapter as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<faac_adapter_t>();
    }
    memcpy(
        (*adapter).asc_buf as *mut ::core::ffi::c_void,
        asc_ptr as *const ::core::ffi::c_void,
        asc_len as size_t,
    );
    (*adapter).asc_size = asc_len as size_t;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Opened FAAC encoder: %u Hz, %u ch, bitrate %u bps, frame_samples %u, delay %u, obj_type %d\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*adapter).sample_rate,
        (*config).num_channels,
        (*adapter).info.bit_rate,
        (*adapter).info.frame_samples,
        (*adapter).info.encoder_delay,
        (*adapter).info.object_type.0 as ::core::ffi::c_int,
    );
    return adapter;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn faac_adapter_get_asc(
    mut adapter: *mut faac_adapter_t,
    mut asc_out: *mut *const uint8_t,
    mut asc_size_out: *mut size_t,
) -> ::core::ffi::c_int {
    if adapter.is_null() || asc_out.is_null() || asc_size_out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *asc_out = (*adapter).asc_buf;
    *asc_size_out = (*adapter).asc_size;
    return if !(*adapter).asc_buf.is_null() && (*adapter).asc_size > 0 as size_t {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn faac_adapter_get_frame_samples(
    mut adapter: *mut faac_adapter_t,
) -> uint32_t {
    return if !adapter.is_null() {
        (*adapter).info.frame_samples
    } else {
        1024 as uint32_t
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn faac_adapter_get_max_output_bytes(
    mut adapter: *mut faac_adapter_t,
) -> uint32_t {
    return if !adapter.is_null() {
        (*adapter).info.max_output_bytes
    } else {
        2048 as uint32_t
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn faac_adapter_get_sample_rate(
    mut adapter: *mut faac_adapter_t,
) -> uint32_t {
    return if !adapter.is_null() {
        (*adapter).sample_rate
    } else {
        0 as uint32_t
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn faac_adapter_get_object_type(
    mut adapter: *mut faac_adapter_t,
) -> faac_object_type {
    return faac_object_type(if !adapter.is_null() {
        (*adapter).info.object_type.0
    } else {
        faac_object_type::FAAC_OBJ_AUTO.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn faac_adapter_get_encoder_delay(
    mut adapter: *mut faac_adapter_t,
) -> uint32_t {
    return if !adapter.is_null() {
        (*adapter).info.encoder_delay
    } else {
        0 as uint32_t
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn faac_adapter_encode(
    mut adapter: *mut faac_adapter_t,
    mut pcm_data: *const ::core::ffi::c_void,
    mut total_samples: uint32_t,
    mut out_buf: *mut uint8_t,
    mut out_cap: uint32_t,
    mut bytes_written: *mut uint32_t,
) -> ::core::ffi::c_int {
    if adapter.is_null() || (*adapter).enc.is_null() || bytes_written.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut status: faac_status = faac_encoder_encode(
        (*adapter).enc,
        pcm_data,
        total_samples,
        out_buf,
        out_cap,
        bytes_written,
    );
    if status.0 != faac_status::FAAC_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"faac_encoder_encode error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            faac_strerror(status),
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn faac_adapter_close(mut adapter_ptr: *mut *mut faac_adapter_t) {
    if adapter_ptr.is_null() || (*adapter_ptr).is_null() {
        return;
    }
    let mut adapter: *mut faac_adapter_t = *adapter_ptr;
    if !(*adapter).enc.is_null() {
        faac_encoder_close(&raw mut (*adapter).enc);
    }
    if !(*adapter).asc_buf.is_null() {
        free((*adapter).asc_buf as *mut ::core::ffi::c_void);
    }
    free(adapter as *mut ::core::ffi::c_void);
    *adapter_ptr = ::core::ptr::null_mut::<faac_adapter_t>();
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
