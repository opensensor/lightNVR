unsafe extern "C" {
    pub type sod_cnn;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn sws_freeContext(swsContext: *mut SwsContext);
    unsafe fn sws_getContext(
        srcW: ::core::ffi::c_int,
        srcH: ::core::ffi::c_int,
        srcFormat: AVPixelFormat,
        dstW: ::core::ffi::c_int,
        dstH: ::core::ffi::c_int,
        dstFormat: AVPixelFormat,
        flags: ::core::ffi::c_int,
        srcFilter: *mut SwsFilter,
        dstFilter: *mut SwsFilter,
        param: *const ::core::ffi::c_double,
    ) -> *mut SwsContext;
    unsafe fn sws_scale(
        c: *mut SwsContext,
        srcSlice: *const *const uint8_t,
        srcStride: *const ::core::ffi::c_int,
        srcSliceY: ::core::ffi::c_int,
        srcSliceH: ::core::ffi::c_int,
        dst: *const *mut uint8_t,
        dstStride: *const ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    unsafe fn sod_cnn_create(
        ppOut: *mut *mut sod_cnn,
        zArch: *const ::core::ffi::c_char,
        zModelPath: *const ::core::ffi::c_char,
        pzErr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sod_cnn_config(pNet: *mut sod_cnn, conf: SOD_CNN_CONFIG, ...) -> ::core::ffi::c_int;
    unsafe fn sod_cnn_predict(
        pNet: *mut sod_cnn,
        pInput: *mut ::core::ffi::c_float,
        paBox: *mut *mut sod_box,
        pnBox: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sod_cnn_destroy(pNet: *mut sod_cnn);
    unsafe fn sod_cnn_prepare_image(pNet: *mut sod_cnn, r#in: sod_img)
    -> *mut ::core::ffi::c_float;
    unsafe fn sod_cnn_get_network_size(
        pNet: *mut sod_cnn,
        pWidth: *mut ::core::ffi::c_int,
        pHeight: *mut ::core::ffi::c_int,
        pChannels: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sod_make_image(
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
    ) -> sod_img;
    unsafe fn sod_free_image(m: sod_img);
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
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __useconds_t = ::core::ffi::c_uint;
pub type int64_t = i64;
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
pub type uint8_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVClassCategory(pub ::core::ffi::c_uint);
impl AVClassCategory {
    pub const AV_CLASS_CATEGORY_NA: Self = Self(0);
    pub const AV_CLASS_CATEGORY_INPUT: Self = Self(1);
    pub const AV_CLASS_CATEGORY_OUTPUT: Self = Self(2);
    pub const AV_CLASS_CATEGORY_MUXER: Self = Self(3);
    pub const AV_CLASS_CATEGORY_DEMUXER: Self = Self(4);
    pub const AV_CLASS_CATEGORY_ENCODER: Self = Self(5);
    pub const AV_CLASS_CATEGORY_DECODER: Self = Self(6);
    pub const AV_CLASS_CATEGORY_FILTER: Self = Self(7);
    pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: Self = Self(8);
    pub const AV_CLASS_CATEGORY_SWSCALER: Self = Self(9);
    pub const AV_CLASS_CATEGORY_SWRESAMPLER: Self = Self(10);
    pub const AV_CLASS_CATEGORY_HWDEVICE: Self = Self(11);
    pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: Self = Self(40);
    pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: Self = Self(41);
    pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: Self = Self(42);
    pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: Self = Self(43);
    pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: Self = Self(44);
    pub const AV_CLASS_CATEGORY_DEVICE_INPUT: Self = Self(45);
    pub const AV_CLASS_CATEGORY_NB: Self = Self(46);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionRanges {
    pub range: *mut *mut AVOptionRange,
    pub nb_ranges: ::core::ffi::c_int,
    pub nb_components: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionRange {
    pub str: *const ::core::ffi::c_char,
    pub value_min: ::core::ffi::c_double,
    pub value_max: ::core::ffi::c_double,
    pub component_min: ::core::ffi::c_double,
    pub component_max: ::core::ffi::c_double,
    pub is_range: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const ::core::ffi::c_char,
    pub item_name:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const ::core::ffi::c_char>,
    pub option: *const AVOption,
    pub version: ::core::ffi::c_int,
    pub log_level_offset_offset: ::core::ffi::c_int,
    pub parent_log_context_offset: ::core::ffi::c_int,
    pub category: AVClassCategory,
    pub get_category: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> AVClassCategory>,
    pub query_ranges: Option<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub child_next: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub child_class_iterate:
        Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass>,
    pub state_flags_offset: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOption {
    pub name: *const ::core::ffi::c_char,
    pub help: *const ::core::ffi::c_char,
    pub offset: ::core::ffi::c_int,
    pub r#type: AVOptionType,
    pub default_val: C2Rust_Unnamed_0,
    pub min: ::core::ffi::c_double,
    pub max: ::core::ffi::c_double,
    pub flags: ::core::ffi::c_int,
    pub unit: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub i64: int64_t,
    pub dbl: ::core::ffi::c_double,
    pub str: *const ::core::ffi::c_char,
    pub q: AVRational,
    pub arr: *const AVOptionArrayDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionArrayDef {
    pub def: *const ::core::ffi::c_char,
    pub size_min: ::core::ffi::c_uint,
    pub size_max: ::core::ffi::c_uint,
    pub sep: ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVOptionType(pub ::core::ffi::c_uint);
impl AVOptionType {
    pub const AV_OPT_TYPE_FLAGS: Self = Self(1);
    pub const AV_OPT_TYPE_INT: Self = Self(2);
    pub const AV_OPT_TYPE_INT64: Self = Self(3);
    pub const AV_OPT_TYPE_DOUBLE: Self = Self(4);
    pub const AV_OPT_TYPE_FLOAT: Self = Self(5);
    pub const AV_OPT_TYPE_STRING: Self = Self(6);
    pub const AV_OPT_TYPE_RATIONAL: Self = Self(7);
    pub const AV_OPT_TYPE_BINARY: Self = Self(8);
    pub const AV_OPT_TYPE_DICT: Self = Self(9);
    pub const AV_OPT_TYPE_UINT64: Self = Self(10);
    pub const AV_OPT_TYPE_CONST: Self = Self(11);
    pub const AV_OPT_TYPE_IMAGE_SIZE: Self = Self(12);
    pub const AV_OPT_TYPE_PIXEL_FMT: Self = Self(13);
    pub const AV_OPT_TYPE_SAMPLE_FMT: Self = Self(14);
    pub const AV_OPT_TYPE_VIDEO_RATE: Self = Self(15);
    pub const AV_OPT_TYPE_DURATION: Self = Self(16);
    pub const AV_OPT_TYPE_COLOR: Self = Self(17);
    pub const AV_OPT_TYPE_BOOL: Self = Self(18);
    pub const AV_OPT_TYPE_CHLAYOUT: Self = Self(19);
    pub const AV_OPT_TYPE_UINT: Self = Self(20);
    pub const AV_OPT_TYPE_FLAG_ARRAY: Self = Self(65536);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVPixelFormat(pub ::core::ffi::c_int);
impl AVPixelFormat {
    pub const AV_PIX_FMT_NONE: Self = Self(-1);
    pub const AV_PIX_FMT_YUV420P: Self = Self(0);
    pub const AV_PIX_FMT_YUYV422: Self = Self(1);
    pub const AV_PIX_FMT_RGB24: Self = Self(2);
    pub const AV_PIX_FMT_BGR24: Self = Self(3);
    pub const AV_PIX_FMT_YUV422P: Self = Self(4);
    pub const AV_PIX_FMT_YUV444P: Self = Self(5);
    pub const AV_PIX_FMT_YUV410P: Self = Self(6);
    pub const AV_PIX_FMT_YUV411P: Self = Self(7);
    pub const AV_PIX_FMT_GRAY8: Self = Self(8);
    pub const AV_PIX_FMT_MONOWHITE: Self = Self(9);
    pub const AV_PIX_FMT_MONOBLACK: Self = Self(10);
    pub const AV_PIX_FMT_PAL8: Self = Self(11);
    pub const AV_PIX_FMT_YUVJ420P: Self = Self(12);
    pub const AV_PIX_FMT_YUVJ422P: Self = Self(13);
    pub const AV_PIX_FMT_YUVJ444P: Self = Self(14);
    pub const AV_PIX_FMT_UYVY422: Self = Self(15);
    pub const AV_PIX_FMT_UYYVYY411: Self = Self(16);
    pub const AV_PIX_FMT_BGR8: Self = Self(17);
    pub const AV_PIX_FMT_BGR4: Self = Self(18);
    pub const AV_PIX_FMT_BGR4_BYTE: Self = Self(19);
    pub const AV_PIX_FMT_RGB8: Self = Self(20);
    pub const AV_PIX_FMT_RGB4: Self = Self(21);
    pub const AV_PIX_FMT_RGB4_BYTE: Self = Self(22);
    pub const AV_PIX_FMT_NV12: Self = Self(23);
    pub const AV_PIX_FMT_NV21: Self = Self(24);
    pub const AV_PIX_FMT_ARGB: Self = Self(25);
    pub const AV_PIX_FMT_RGBA: Self = Self(26);
    pub const AV_PIX_FMT_ABGR: Self = Self(27);
    pub const AV_PIX_FMT_BGRA: Self = Self(28);
    pub const AV_PIX_FMT_GRAY16BE: Self = Self(29);
    pub const AV_PIX_FMT_GRAY16LE: Self = Self(30);
    pub const AV_PIX_FMT_YUV440P: Self = Self(31);
    pub const AV_PIX_FMT_YUVJ440P: Self = Self(32);
    pub const AV_PIX_FMT_YUVA420P: Self = Self(33);
    pub const AV_PIX_FMT_RGB48BE: Self = Self(34);
    pub const AV_PIX_FMT_RGB48LE: Self = Self(35);
    pub const AV_PIX_FMT_RGB565BE: Self = Self(36);
    pub const AV_PIX_FMT_RGB565LE: Self = Self(37);
    pub const AV_PIX_FMT_RGB555BE: Self = Self(38);
    pub const AV_PIX_FMT_RGB555LE: Self = Self(39);
    pub const AV_PIX_FMT_BGR565BE: Self = Self(40);
    pub const AV_PIX_FMT_BGR565LE: Self = Self(41);
    pub const AV_PIX_FMT_BGR555BE: Self = Self(42);
    pub const AV_PIX_FMT_BGR555LE: Self = Self(43);
    pub const AV_PIX_FMT_VAAPI: Self = Self(44);
    pub const AV_PIX_FMT_YUV420P16LE: Self = Self(45);
    pub const AV_PIX_FMT_YUV420P16BE: Self = Self(46);
    pub const AV_PIX_FMT_YUV422P16LE: Self = Self(47);
    pub const AV_PIX_FMT_YUV422P16BE: Self = Self(48);
    pub const AV_PIX_FMT_YUV444P16LE: Self = Self(49);
    pub const AV_PIX_FMT_YUV444P16BE: Self = Self(50);
    pub const AV_PIX_FMT_DXVA2_VLD: Self = Self(51);
    pub const AV_PIX_FMT_RGB444LE: Self = Self(52);
    pub const AV_PIX_FMT_RGB444BE: Self = Self(53);
    pub const AV_PIX_FMT_BGR444LE: Self = Self(54);
    pub const AV_PIX_FMT_BGR444BE: Self = Self(55);
    pub const AV_PIX_FMT_YA8: Self = Self(56);
    pub const AV_PIX_FMT_Y400A: Self = Self(56);
    pub const AV_PIX_FMT_GRAY8A: Self = Self(56);
    pub const AV_PIX_FMT_BGR48BE: Self = Self(57);
    pub const AV_PIX_FMT_BGR48LE: Self = Self(58);
    pub const AV_PIX_FMT_YUV420P9BE: Self = Self(59);
    pub const AV_PIX_FMT_YUV420P9LE: Self = Self(60);
    pub const AV_PIX_FMT_YUV420P10BE: Self = Self(61);
    pub const AV_PIX_FMT_YUV420P10LE: Self = Self(62);
    pub const AV_PIX_FMT_YUV422P10BE: Self = Self(63);
    pub const AV_PIX_FMT_YUV422P10LE: Self = Self(64);
    pub const AV_PIX_FMT_YUV444P9BE: Self = Self(65);
    pub const AV_PIX_FMT_YUV444P9LE: Self = Self(66);
    pub const AV_PIX_FMT_YUV444P10BE: Self = Self(67);
    pub const AV_PIX_FMT_YUV444P10LE: Self = Self(68);
    pub const AV_PIX_FMT_YUV422P9BE: Self = Self(69);
    pub const AV_PIX_FMT_YUV422P9LE: Self = Self(70);
    pub const AV_PIX_FMT_GBRP: Self = Self(71);
    pub const AV_PIX_FMT_GBR24P: Self = Self(71);
    pub const AV_PIX_FMT_GBRP9BE: Self = Self(72);
    pub const AV_PIX_FMT_GBRP9LE: Self = Self(73);
    pub const AV_PIX_FMT_GBRP10BE: Self = Self(74);
    pub const AV_PIX_FMT_GBRP10LE: Self = Self(75);
    pub const AV_PIX_FMT_GBRP16BE: Self = Self(76);
    pub const AV_PIX_FMT_GBRP16LE: Self = Self(77);
    pub const AV_PIX_FMT_YUVA422P: Self = Self(78);
    pub const AV_PIX_FMT_YUVA444P: Self = Self(79);
    pub const AV_PIX_FMT_YUVA420P9BE: Self = Self(80);
    pub const AV_PIX_FMT_YUVA420P9LE: Self = Self(81);
    pub const AV_PIX_FMT_YUVA422P9BE: Self = Self(82);
    pub const AV_PIX_FMT_YUVA422P9LE: Self = Self(83);
    pub const AV_PIX_FMT_YUVA444P9BE: Self = Self(84);
    pub const AV_PIX_FMT_YUVA444P9LE: Self = Self(85);
    pub const AV_PIX_FMT_YUVA420P10BE: Self = Self(86);
    pub const AV_PIX_FMT_YUVA420P10LE: Self = Self(87);
    pub const AV_PIX_FMT_YUVA422P10BE: Self = Self(88);
    pub const AV_PIX_FMT_YUVA422P10LE: Self = Self(89);
    pub const AV_PIX_FMT_YUVA444P10BE: Self = Self(90);
    pub const AV_PIX_FMT_YUVA444P10LE: Self = Self(91);
    pub const AV_PIX_FMT_YUVA420P16BE: Self = Self(92);
    pub const AV_PIX_FMT_YUVA420P16LE: Self = Self(93);
    pub const AV_PIX_FMT_YUVA422P16BE: Self = Self(94);
    pub const AV_PIX_FMT_YUVA422P16LE: Self = Self(95);
    pub const AV_PIX_FMT_YUVA444P16BE: Self = Self(96);
    pub const AV_PIX_FMT_YUVA444P16LE: Self = Self(97);
    pub const AV_PIX_FMT_VDPAU: Self = Self(98);
    pub const AV_PIX_FMT_XYZ12LE: Self = Self(99);
    pub const AV_PIX_FMT_XYZ12BE: Self = Self(100);
    pub const AV_PIX_FMT_NV16: Self = Self(101);
    pub const AV_PIX_FMT_NV20LE: Self = Self(102);
    pub const AV_PIX_FMT_NV20BE: Self = Self(103);
    pub const AV_PIX_FMT_RGBA64BE: Self = Self(104);
    pub const AV_PIX_FMT_RGBA64LE: Self = Self(105);
    pub const AV_PIX_FMT_BGRA64BE: Self = Self(106);
    pub const AV_PIX_FMT_BGRA64LE: Self = Self(107);
    pub const AV_PIX_FMT_YVYU422: Self = Self(108);
    pub const AV_PIX_FMT_YA16BE: Self = Self(109);
    pub const AV_PIX_FMT_YA16LE: Self = Self(110);
    pub const AV_PIX_FMT_GBRAP: Self = Self(111);
    pub const AV_PIX_FMT_GBRAP16BE: Self = Self(112);
    pub const AV_PIX_FMT_GBRAP16LE: Self = Self(113);
    pub const AV_PIX_FMT_QSV: Self = Self(114);
    pub const AV_PIX_FMT_MMAL: Self = Self(115);
    pub const AV_PIX_FMT_D3D11VA_VLD: Self = Self(116);
    pub const AV_PIX_FMT_CUDA: Self = Self(117);
    pub const AV_PIX_FMT_0RGB: Self = Self(118);
    pub const AV_PIX_FMT_RGB0: Self = Self(119);
    pub const AV_PIX_FMT_0BGR: Self = Self(120);
    pub const AV_PIX_FMT_BGR0: Self = Self(121);
    pub const AV_PIX_FMT_YUV420P12BE: Self = Self(122);
    pub const AV_PIX_FMT_YUV420P12LE: Self = Self(123);
    pub const AV_PIX_FMT_YUV420P14BE: Self = Self(124);
    pub const AV_PIX_FMT_YUV420P14LE: Self = Self(125);
    pub const AV_PIX_FMT_YUV422P12BE: Self = Self(126);
    pub const AV_PIX_FMT_YUV422P12LE: Self = Self(127);
    pub const AV_PIX_FMT_YUV422P14BE: Self = Self(128);
    pub const AV_PIX_FMT_YUV422P14LE: Self = Self(129);
    pub const AV_PIX_FMT_YUV444P12BE: Self = Self(130);
    pub const AV_PIX_FMT_YUV444P12LE: Self = Self(131);
    pub const AV_PIX_FMT_YUV444P14BE: Self = Self(132);
    pub const AV_PIX_FMT_YUV444P14LE: Self = Self(133);
    pub const AV_PIX_FMT_GBRP12BE: Self = Self(134);
    pub const AV_PIX_FMT_GBRP12LE: Self = Self(135);
    pub const AV_PIX_FMT_GBRP14BE: Self = Self(136);
    pub const AV_PIX_FMT_GBRP14LE: Self = Self(137);
    pub const AV_PIX_FMT_YUVJ411P: Self = Self(138);
    pub const AV_PIX_FMT_BAYER_BGGR8: Self = Self(139);
    pub const AV_PIX_FMT_BAYER_RGGB8: Self = Self(140);
    pub const AV_PIX_FMT_BAYER_GBRG8: Self = Self(141);
    pub const AV_PIX_FMT_BAYER_GRBG8: Self = Self(142);
    pub const AV_PIX_FMT_BAYER_BGGR16LE: Self = Self(143);
    pub const AV_PIX_FMT_BAYER_BGGR16BE: Self = Self(144);
    pub const AV_PIX_FMT_BAYER_RGGB16LE: Self = Self(145);
    pub const AV_PIX_FMT_BAYER_RGGB16BE: Self = Self(146);
    pub const AV_PIX_FMT_BAYER_GBRG16LE: Self = Self(147);
    pub const AV_PIX_FMT_BAYER_GBRG16BE: Self = Self(148);
    pub const AV_PIX_FMT_BAYER_GRBG16LE: Self = Self(149);
    pub const AV_PIX_FMT_BAYER_GRBG16BE: Self = Self(150);
    pub const AV_PIX_FMT_YUV440P10LE: Self = Self(151);
    pub const AV_PIX_FMT_YUV440P10BE: Self = Self(152);
    pub const AV_PIX_FMT_YUV440P12LE: Self = Self(153);
    pub const AV_PIX_FMT_YUV440P12BE: Self = Self(154);
    pub const AV_PIX_FMT_AYUV64LE: Self = Self(155);
    pub const AV_PIX_FMT_AYUV64BE: Self = Self(156);
    pub const AV_PIX_FMT_VIDEOTOOLBOX: Self = Self(157);
    pub const AV_PIX_FMT_P010LE: Self = Self(158);
    pub const AV_PIX_FMT_P010BE: Self = Self(159);
    pub const AV_PIX_FMT_GBRAP12BE: Self = Self(160);
    pub const AV_PIX_FMT_GBRAP12LE: Self = Self(161);
    pub const AV_PIX_FMT_GBRAP10BE: Self = Self(162);
    pub const AV_PIX_FMT_GBRAP10LE: Self = Self(163);
    pub const AV_PIX_FMT_MEDIACODEC: Self = Self(164);
    pub const AV_PIX_FMT_GRAY12BE: Self = Self(165);
    pub const AV_PIX_FMT_GRAY12LE: Self = Self(166);
    pub const AV_PIX_FMT_GRAY10BE: Self = Self(167);
    pub const AV_PIX_FMT_GRAY10LE: Self = Self(168);
    pub const AV_PIX_FMT_P016LE: Self = Self(169);
    pub const AV_PIX_FMT_P016BE: Self = Self(170);
    pub const AV_PIX_FMT_D3D11: Self = Self(171);
    pub const AV_PIX_FMT_GRAY9BE: Self = Self(172);
    pub const AV_PIX_FMT_GRAY9LE: Self = Self(173);
    pub const AV_PIX_FMT_GBRPF32BE: Self = Self(174);
    pub const AV_PIX_FMT_GBRPF32LE: Self = Self(175);
    pub const AV_PIX_FMT_GBRAPF32BE: Self = Self(176);
    pub const AV_PIX_FMT_GBRAPF32LE: Self = Self(177);
    pub const AV_PIX_FMT_DRM_PRIME: Self = Self(178);
    pub const AV_PIX_FMT_OPENCL: Self = Self(179);
    pub const AV_PIX_FMT_GRAY14BE: Self = Self(180);
    pub const AV_PIX_FMT_GRAY14LE: Self = Self(181);
    pub const AV_PIX_FMT_GRAYF32BE: Self = Self(182);
    pub const AV_PIX_FMT_GRAYF32LE: Self = Self(183);
    pub const AV_PIX_FMT_YUVA422P12BE: Self = Self(184);
    pub const AV_PIX_FMT_YUVA422P12LE: Self = Self(185);
    pub const AV_PIX_FMT_YUVA444P12BE: Self = Self(186);
    pub const AV_PIX_FMT_YUVA444P12LE: Self = Self(187);
    pub const AV_PIX_FMT_NV24: Self = Self(188);
    pub const AV_PIX_FMT_NV42: Self = Self(189);
    pub const AV_PIX_FMT_VULKAN: Self = Self(190);
    pub const AV_PIX_FMT_Y210BE: Self = Self(191);
    pub const AV_PIX_FMT_Y210LE: Self = Self(192);
    pub const AV_PIX_FMT_X2RGB10LE: Self = Self(193);
    pub const AV_PIX_FMT_X2RGB10BE: Self = Self(194);
    pub const AV_PIX_FMT_X2BGR10LE: Self = Self(195);
    pub const AV_PIX_FMT_X2BGR10BE: Self = Self(196);
    pub const AV_PIX_FMT_P210BE: Self = Self(197);
    pub const AV_PIX_FMT_P210LE: Self = Self(198);
    pub const AV_PIX_FMT_P410BE: Self = Self(199);
    pub const AV_PIX_FMT_P410LE: Self = Self(200);
    pub const AV_PIX_FMT_P216BE: Self = Self(201);
    pub const AV_PIX_FMT_P216LE: Self = Self(202);
    pub const AV_PIX_FMT_P416BE: Self = Self(203);
    pub const AV_PIX_FMT_P416LE: Self = Self(204);
    pub const AV_PIX_FMT_VUYA: Self = Self(205);
    pub const AV_PIX_FMT_RGBAF16BE: Self = Self(206);
    pub const AV_PIX_FMT_RGBAF16LE: Self = Self(207);
    pub const AV_PIX_FMT_VUYX: Self = Self(208);
    pub const AV_PIX_FMT_P012LE: Self = Self(209);
    pub const AV_PIX_FMT_P012BE: Self = Self(210);
    pub const AV_PIX_FMT_Y212BE: Self = Self(211);
    pub const AV_PIX_FMT_Y212LE: Self = Self(212);
    pub const AV_PIX_FMT_XV30BE: Self = Self(213);
    pub const AV_PIX_FMT_XV30LE: Self = Self(214);
    pub const AV_PIX_FMT_XV36BE: Self = Self(215);
    pub const AV_PIX_FMT_XV36LE: Self = Self(216);
    pub const AV_PIX_FMT_RGBF32BE: Self = Self(217);
    pub const AV_PIX_FMT_RGBF32LE: Self = Self(218);
    pub const AV_PIX_FMT_RGBAF32BE: Self = Self(219);
    pub const AV_PIX_FMT_RGBAF32LE: Self = Self(220);
    pub const AV_PIX_FMT_P212BE: Self = Self(221);
    pub const AV_PIX_FMT_P212LE: Self = Self(222);
    pub const AV_PIX_FMT_P412BE: Self = Self(223);
    pub const AV_PIX_FMT_P412LE: Self = Self(224);
    pub const AV_PIX_FMT_GBRAP14BE: Self = Self(225);
    pub const AV_PIX_FMT_GBRAP14LE: Self = Self(226);
    pub const AV_PIX_FMT_D3D12: Self = Self(227);
    pub const AV_PIX_FMT_AYUV: Self = Self(228);
    pub const AV_PIX_FMT_UYVA: Self = Self(229);
    pub const AV_PIX_FMT_VYU444: Self = Self(230);
    pub const AV_PIX_FMT_V30XBE: Self = Self(231);
    pub const AV_PIX_FMT_V30XLE: Self = Self(232);
    pub const AV_PIX_FMT_RGBF16BE: Self = Self(233);
    pub const AV_PIX_FMT_RGBF16LE: Self = Self(234);
    pub const AV_PIX_FMT_RGBA128BE: Self = Self(235);
    pub const AV_PIX_FMT_RGBA128LE: Self = Self(236);
    pub const AV_PIX_FMT_RGB96BE: Self = Self(237);
    pub const AV_PIX_FMT_RGB96LE: Self = Self(238);
    pub const AV_PIX_FMT_Y216BE: Self = Self(239);
    pub const AV_PIX_FMT_Y216LE: Self = Self(240);
    pub const AV_PIX_FMT_XV48BE: Self = Self(241);
    pub const AV_PIX_FMT_XV48LE: Self = Self(242);
    pub const AV_PIX_FMT_GBRPF16BE: Self = Self(243);
    pub const AV_PIX_FMT_GBRPF16LE: Self = Self(244);
    pub const AV_PIX_FMT_GBRAPF16BE: Self = Self(245);
    pub const AV_PIX_FMT_GBRAPF16LE: Self = Self(246);
    pub const AV_PIX_FMT_GRAYF16BE: Self = Self(247);
    pub const AV_PIX_FMT_GRAYF16LE: Self = Self(248);
    pub const AV_PIX_FMT_AMF_SURFACE: Self = Self(249);
    pub const AV_PIX_FMT_GRAY32BE: Self = Self(250);
    pub const AV_PIX_FMT_GRAY32LE: Self = Self(251);
    pub const AV_PIX_FMT_YAF32BE: Self = Self(252);
    pub const AV_PIX_FMT_YAF32LE: Self = Self(253);
    pub const AV_PIX_FMT_YAF16BE: Self = Self(254);
    pub const AV_PIX_FMT_YAF16LE: Self = Self(255);
    pub const AV_PIX_FMT_GBRAP32BE: Self = Self(256);
    pub const AV_PIX_FMT_GBRAP32LE: Self = Self(257);
    pub const AV_PIX_FMT_YUV444P10MSBBE: Self = Self(258);
    pub const AV_PIX_FMT_YUV444P10MSBLE: Self = Self(259);
    pub const AV_PIX_FMT_YUV444P12MSBBE: Self = Self(260);
    pub const AV_PIX_FMT_YUV444P12MSBLE: Self = Self(261);
    pub const AV_PIX_FMT_GBRP10MSBBE: Self = Self(262);
    pub const AV_PIX_FMT_GBRP10MSBLE: Self = Self(263);
    pub const AV_PIX_FMT_GBRP12MSBBE: Self = Self(264);
    pub const AV_PIX_FMT_GBRP12MSBLE: Self = Self(265);
    pub const AV_PIX_FMT_OHCODEC: Self = Self(266);
    pub const AV_PIX_FMT_NB: Self = Self(267);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SwsDither(pub ::core::ffi::c_uint);
impl SwsDither {
    pub const SWS_DITHER_NONE: Self = Self(0);
    pub const SWS_DITHER_AUTO: Self = Self(1);
    pub const SWS_DITHER_BAYER: Self = Self(2);
    pub const SWS_DITHER_ED: Self = Self(3);
    pub const SWS_DITHER_A_DITHER: Self = Self(4);
    pub const SWS_DITHER_X_DITHER: Self = Self(5);
    pub const SWS_DITHER_NB: Self = Self(6);
    pub const SWS_DITHER_MAX_ENUM: Self = Self(2147483647);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SwsAlphaBlend(pub ::core::ffi::c_uint);
impl SwsAlphaBlend {
    pub const SWS_ALPHA_BLEND_NONE: Self = Self(0);
    pub const SWS_ALPHA_BLEND_UNIFORM: Self = Self(1);
    pub const SWS_ALPHA_BLEND_CHECKERBOARD: Self = Self(2);
    pub const SWS_ALPHA_BLEND_NB: Self = Self(3);
    pub const SWS_ALPHA_BLEND_MAX_ENUM: Self = Self(2147483647);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SwsScaler(pub ::core::ffi::c_uint);
impl SwsScaler {
    pub const SWS_SCALE_AUTO: Self = Self(0);
    pub const SWS_SCALE_BILINEAR: Self = Self(1);
    pub const SWS_SCALE_BICUBIC: Self = Self(2);
    pub const SWS_SCALE_POINT: Self = Self(3);
    pub const SWS_SCALE_AREA: Self = Self(4);
    pub const SWS_SCALE_GAUSSIAN: Self = Self(5);
    pub const SWS_SCALE_SINC: Self = Self(6);
    pub const SWS_SCALE_LANCZOS: Self = Self(7);
    pub const SWS_SCALE_SPLINE: Self = Self(8);
    pub const SWS_SCALE_NB: Self = Self(9);
    pub const SWS_SCALE_MAX_ENUM: Self = Self(2147483647);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SwsBackend(pub ::core::ffi::c_uint);
impl SwsBackend {
    pub const SWS_BACKEND_LEGACY: Self = Self(1);
    pub const SWS_BACKEND_STABLE: Self = Self(1);
    pub const SWS_BACKEND_C: Self = Self(2);
    pub const SWS_BACKEND_MEMCPY: Self = Self(4);
    pub const SWS_BACKEND_X86: Self = Self(8);
    pub const SWS_BACKEND_AARCH64: Self = Self(16);
    pub const SWS_BACKEND_SPIRV: Self = Self(32);
    pub const SWS_BACKEND_UNSTABLE: Self = Self(62);
    pub const SWS_BACKEND_ALL: Self = Self(63);
    pub const SWS_BACKEND_MAX_ENUM: Self = Self(2147483647);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SwsFlags(pub ::core::ffi::c_uint);
impl SwsFlags {
    pub const SWS_STRICT: Self = Self(2048);
    pub const SWS_PRINT_INFO: Self = Self(4096);
    pub const SWS_FULL_CHR_H_INT: Self = Self(8192);
    pub const SWS_FULL_CHR_H_INP: Self = Self(16384);
    pub const SWS_ACCURATE_RND: Self = Self(262144);
    pub const SWS_BITEXACT: Self = Self(524288);
    pub const SWS_UNSTABLE: Self = Self(1048576);
    pub const SWS_DIRECT_BGR: Self = Self(32768);
    pub const SWS_ERROR_DIFFUSION: Self = Self(8388608);
    pub const SWS_FAST_BILINEAR: Self = Self(1);
    pub const SWS_BILINEAR: Self = Self(2);
    pub const SWS_BICUBIC: Self = Self(4);
    pub const SWS_X: Self = Self(8);
    pub const SWS_POINT: Self = Self(16);
    pub const SWS_AREA: Self = Self(32);
    pub const SWS_BICUBLIN: Self = Self(64);
    pub const SWS_GAUSS: Self = Self(128);
    pub const SWS_SINC: Self = Self(256);
    pub const SWS_LANCZOS: Self = Self(512);
    pub const SWS_SPLINE: Self = Self(1024);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwsContext {
    pub av_class: *const AVClass,
    pub opaque: *mut ::core::ffi::c_void,
    pub flags: ::core::ffi::c_uint,
    pub scaler_params: [::core::ffi::c_double; 2],
    pub threads: ::core::ffi::c_int,
    pub dither: SwsDither,
    pub alpha_blend: SwsAlphaBlend,
    pub gamma_flag: ::core::ffi::c_int,
    pub src_w: ::core::ffi::c_int,
    pub src_h: ::core::ffi::c_int,
    pub dst_w: ::core::ffi::c_int,
    pub dst_h: ::core::ffi::c_int,
    pub src_format: ::core::ffi::c_int,
    pub dst_format: ::core::ffi::c_int,
    pub src_range: ::core::ffi::c_int,
    pub dst_range: ::core::ffi::c_int,
    pub src_v_chr_pos: ::core::ffi::c_int,
    pub src_h_chr_pos: ::core::ffi::c_int,
    pub dst_v_chr_pos: ::core::ffi::c_int,
    pub dst_h_chr_pos: ::core::ffi::c_int,
    pub intent: ::core::ffi::c_int,
    pub scaler: SwsScaler,
    pub scaler_sub: SwsScaler,
    pub backends: SwsBackend,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwsVector {
    pub coeff: *mut ::core::ffi::c_double,
    pub length: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwsFilter {
    pub lumH: *mut SwsVector,
    pub lumV: *mut SwsVector,
    pub chrH: *mut SwsVector,
    pub chrV: *mut SwsVector,
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
pub type detection_model_t = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tflite_model_t {
    pub engine: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct model_t {
    pub r#type: [::core::ffi::c_char; 16],
    pub c2rust_unnamed: C2Rust_Unnamed_1,
    pub threshold: ::core::ffi::c_float,
    pub path: [::core::ffi::c_char; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub sod: *mut ::core::ffi::c_void,
    pub sod_realnet: *mut ::core::ffi::c_void,
    pub tflite: tflite_model_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SOD_CNN_CONFIG(pub ::core::ffi::c_uint);
impl SOD_CNN_CONFIG {
    pub const SOD_CNN_NETWORK_OUTPUT: Self = Self(1);
    pub const SOD_CNN_DETECTION_THRESHOLD: Self = Self(2);
    pub const SOD_CNN_NMS: Self = Self(3);
    pub const SOD_CNN_DETECTION_CLASSES: Self = Self(4);
    pub const SOD_CNN_RAND_SEED: Self = Self(5);
    pub const SOD_CNN_HIER_THRESHOLD: Self = Self(6);
    pub const SOD_CNN_TEMPERATURE: Self = Self(7);
    pub const SOD_CNN_LOG_CALLBACK: Self = Self(8);
    pub const SOD_RNN_CALLBACK: Self = Self(9);
    pub const SOD_RNN_TEXT_LENGTH: Self = Self(10);
    pub const SOD_RNN_DATA_LENGTH: Self = Self(11);
    pub const SOD_RNN_SEED: Self = Self(12);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sod_img {
    pub h: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub c: ::core::ffi::c_int,
    pub c2rust_unnamed: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub data: *mut ::core::ffi::c_float,
    pub zdata: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sod_box {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub score: ::core::ffi::c_float,
    pub zName: *const ::core::ffi::c_char,
    pub pUserData: *mut ::core::ffi::c_void,
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MAX_LABEL_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MODEL_TYPE_SOD: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"sod\0") };
static mut sod_available: bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_sod_detection_system() -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"SOD detection initialized with static linking\0".as_ptr() as *const ::core::ffi::c_char,
    );
    sod_available = r#true != 0;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_sod_detection_system() {
    sod_available = r#false != 0;
    usleep(100000 as ::core::ffi::c_int as __useconds_t);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"SOD detection system shutdown\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const MAX_CLEANED_MODELS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut cleaned_models: [*mut ::core::ffi::c_void; 32] = [
    NULL,
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
];
static mut cleaned_models_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut cleaned_models_mutex: pthread_mutex_t = pthread_mutex_t {
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
unsafe extern "C" fn is_model_already_cleaned(mut model_ptr: *const ::core::ffi::c_void) -> bool {
    let mut result: bool = r#false != 0;
    pthread_mutex_lock(&raw mut cleaned_models_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < cleaned_models_count {
        if cleaned_models[i as usize] == model_ptr as *mut ::core::ffi::c_void {
            result = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut cleaned_models_mutex);
    return result;
}
unsafe extern "C" fn add_to_cleaned_models(mut model_ptr: *mut ::core::ffi::c_void) {
    pthread_mutex_lock(&raw mut cleaned_models_mutex);
    if cleaned_models_count < MAX_CLEANED_MODELS {
        let c2rust_fresh0 = cleaned_models_count;
        cleaned_models_count += 1;
        cleaned_models[c2rust_fresh0 as usize] = model_ptr;
    } else {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < MAX_CLEANED_MODELS - 1 as ::core::ffi::c_int {
            cleaned_models[i as usize] = cleaned_models[(i + 1 as ::core::ffi::c_int) as usize];
            i += 1;
        }
        cleaned_models[(MAX_CLEANED_MODELS - 1 as ::core::ffi::c_int) as usize] = model_ptr;
    }
    pthread_mutex_unlock(&raw mut cleaned_models_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_sod_model(mut model: detection_model_t) {
    if model.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Attempted to clean up NULL SOD model\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut m: *mut model_t = model as *mut model_t;
    if strcmp(
        &raw mut (*m).r#type as *mut ::core::ffi::c_char,
        MODEL_TYPE_SOD.as_ptr(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Attempted to clean up non-SOD model: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*m).r#type as *mut ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleaning up SOD model: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*m).path as *mut ::core::ffi::c_char,
    );
    let mut sod_model_ptr: *mut ::core::ffi::c_void = (*m).c2rust_unnamed.sod;
    (*m).c2rust_unnamed.sod = NULL;
    if !sod_model_ptr.is_null() {
        if is_model_already_cleaned(sod_model_ptr) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Skipping cleanup of already cleaned SOD model: %p\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sod_model_ptr,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Destroying SOD model: %p\0".as_ptr() as *const ::core::ffi::c_char,
                sod_model_ptr,
            );
            add_to_cleaned_models(sod_model_ptr);
            let mut temp: *mut ::core::ffi::c_void = sod_model_ptr;
            sod_model_ptr = NULL;
            sod_cnn_destroy(temp as *mut sod_cnn);
        }
    }
    free(m as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"SOD model cleanup complete\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_sod_available() -> bool {
    return sod_available;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_sod_model(
    mut model_path: *const ::core::ffi::c_char,
    mut threshold: ::core::ffi::c_float,
) -> detection_model_t {
    if !sod_available {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"SOD library not available\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    let mut arch: *const ::core::ffi::c_char = b":face\0".as_ptr() as *const ::core::ffi::c_char;
    let mut filename: *const ::core::ffi::c_char = strrchr(model_path, '/' as ::core::ffi::c_int);
    if !filename.is_null() {
        filename = filename.offset(1);
    } else {
        filename = model_path;
    }
    if strcmp(
        filename,
        b"face_cnn.sod\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            filename,
            b"face.sod\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            filename,
            b"face_detection.sod\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Detected face model by exact filename match, using :face architecture: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            filename,
        );
    } else if strcmp(
        filename,
        b"tiny20.sod\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            filename,
            b"voc.sod\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            filename,
            b"voc_detection.sod\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        arch = b":voc\0".as_ptr() as *const ::core::ffi::c_char;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Detected VOC model by exact filename match, using :voc architecture: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            filename,
        );
    } else if strcmp(
        filename,
        b"tiny80.sod\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        arch = b":tiny80\0".as_ptr() as *const ::core::ffi::c_char;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Detected COCO-80 model by exact filename match, using :tiny80 architecture: %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            filename,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not determine model architecture from name, defaulting to :face for: %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            model_path,
        );
    }
    let mut sod_model: *mut ::core::ffi::c_void = NULL;
    let mut err_msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int = 0;
    let mut cnn_model: *mut sod_cnn = ::core::ptr::null_mut::<sod_cnn>();
    rc = sod_cnn_create(&raw mut cnn_model, arch, model_path, &raw mut err_msg);
    if rc != 0 as ::core::ffi::c_int || cnn_model.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to load SOD model: %s - %s\0".as_ptr() as *const ::core::ffi::c_char,
            model_path,
            if !err_msg.is_null() {
                err_msg
            } else {
                b"Unknown error\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return NULL;
    }
    sod_model = cnn_model as *mut ::core::ffi::c_void;
    if threshold <= 0.0f32 {
        threshold = 0.3f32;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using default threshold of 0.3 for model %s\0".as_ptr() as *const ::core::ffi::c_char,
            model_path,
        );
    }
    sod_cnn_config(
        cnn_model,
        SOD_CNN_CONFIG::SOD_CNN_DETECTION_THRESHOLD,
        threshold as ::core::ffi::c_double,
    );
    let mut model: *mut model_t = malloc(::core::mem::size_of::<model_t>()) as *mut model_t;
    if model.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for model structure\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        sod_cnn_destroy(sod_model as *mut sod_cnn);
        return NULL;
    }
    safe_strcpy(
        &raw mut (*model).r#type as *mut ::core::ffi::c_char,
        MODEL_TYPE_SOD.as_ptr(),
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    (*model).c2rust_unnamed.sod = sod_model;
    (*model).threshold = threshold;
    safe_strcpy(
        &raw mut (*model).path as *mut ::core::ffi::c_char,
        model_path,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"SOD model loaded: %s with threshold %.2f\0".as_ptr() as *const ::core::ffi::c_char,
        model_path,
        threshold as ::core::ffi::c_double,
    );
    return model as detection_model_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_with_sod_model(
    mut model: detection_model_t,
    mut frame_data: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut channels: ::core::ffi::c_int,
    mut result: *mut detection_result_t,
) -> ::core::ffi::c_int {
    if model.is_null() || frame_data.is_null() || result.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for detect_with_sod_model: model=%p, frame_data=%p, result=%p\0"
                .as_ptr() as *const ::core::ffi::c_char,
            model,
            frame_data,
            result,
        );
        return -1 as ::core::ffi::c_int;
    }
    if width <= 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
        || channels <= 0 as ::core::ffi::c_int
        || channels > 4 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid dimensions for detect_with_sod_model: width=%d, height=%d, channels=%d\0"
                .as_ptr() as *const ::core::ffi::c_char,
            width,
            height,
            channels,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut m: *mut model_t = model as *mut model_t;
    if strcmp(
        &raw mut (*m).r#type as *mut ::core::ffi::c_char,
        MODEL_TYPE_SOD.as_ptr(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid model type for detect_with_sod_model: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*m).r#type as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !sod_available {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"SOD library not available\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut net_w: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut net_h: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut net_c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut conv_width: ::core::ffi::c_int = width;
    let mut conv_height: ::core::ffi::c_int = height;
    let mut conv_source: *const ::core::ffi::c_uchar = frame_data;
    let mut resized_frame: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut resize_ctx: *mut SwsContext = ::core::ptr::null_mut::<SwsContext>();
    if channels == 3 as ::core::ffi::c_int
        && sod_cnn_get_network_size(
            (*m).c2rust_unnamed.sod as *mut sod_cnn,
            &raw mut net_w,
            &raw mut net_h,
            &raw mut net_c,
        ) == 0 as ::core::ffi::c_int
        && net_w > 0 as ::core::ffi::c_int
        && net_h > 0 as ::core::ffi::c_int
        && net_c == channels
    {
        let mut intermediate_w: ::core::ffi::c_int = net_w * 2 as ::core::ffi::c_int;
        let mut intermediate_h: ::core::ffi::c_int = net_h * 2 as ::core::ffi::c_int;
        if intermediate_w < width && intermediate_h < height {
            resize_ctx = sws_getContext(
                width,
                height,
                AVPixelFormat::AV_PIX_FMT_RGB24,
                intermediate_w,
                intermediate_h,
                AVPixelFormat::AV_PIX_FMT_RGB24,
                SwsFlags::SWS_AREA.0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<SwsFilter>(),
                ::core::ptr::null_mut::<SwsFilter>(),
                ::core::ptr::null::<::core::ffi::c_double>(),
            ) as *mut SwsContext;
            if !resize_ctx.is_null() {
                resized_frame = malloc(
                    (intermediate_w as size_t)
                        .wrapping_mul(intermediate_h as size_t)
                        .wrapping_mul(channels as size_t),
                ) as *mut uint8_t;
                if !resized_frame.is_null() {
                    let mut dst_data: [*mut uint8_t; 4] = [
                        resized_frame,
                        ::core::ptr::null_mut::<uint8_t>(),
                        ::core::ptr::null_mut::<uint8_t>(),
                        ::core::ptr::null_mut::<uint8_t>(),
                    ];
                    let mut dst_linesize: [::core::ffi::c_int; 4] = [
                        intermediate_w * channels,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    ];
                    let mut src_data: [*const uint8_t; 4] = [
                        frame_data as *const uint8_t,
                        ::core::ptr::null::<uint8_t>(),
                        ::core::ptr::null::<uint8_t>(),
                        ::core::ptr::null::<uint8_t>(),
                    ];
                    let mut src_linesize: [::core::ffi::c_int; 4] = [
                        width * channels,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    ];
                    sws_scale(
                        resize_ctx,
                        &raw mut src_data as *mut *const uint8_t,
                        &raw mut src_linesize as *mut ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        height,
                        &raw mut dst_data as *mut *mut uint8_t,
                        &raw mut dst_linesize as *mut ::core::ffi::c_int,
                    );
                    conv_width = intermediate_w;
                    conv_height = intermediate_h;
                    conv_source = resized_frame as *const ::core::ffi::c_uchar;
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to allocate pre-resize buffer (%dx%d), using full-resolution frame\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        intermediate_w,
                        intermediate_h,
                    );
                    sws_freeContext(resize_ctx);
                    resize_ctx = ::core::ptr::null_mut::<SwsContext>();
                }
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Step 1: Creating SOD image from frame data (dimensions: %dx%d, channels: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        conv_width,
        conv_height,
        channels,
    );
    let mut img: sod_img = sod_make_image(conv_width, conv_height, channels);
    if img.c2rust_unnamed.data.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create SOD image\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(resized_frame as *mut ::core::ffi::c_void);
        if !resize_ctx.is_null() {
            sws_freeContext(resize_ctx);
        }
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Step 2: Copying frame data to SOD image\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut pixel_count: size_t = (conv_width as size_t).wrapping_mul(conv_height as size_t);
    if pixel_count.wrapping_div(conv_width as size_t) != conv_height as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Integer overflow in image dimensions: width=%d, height=%d\0".as_ptr()
                as *const ::core::ffi::c_char,
            conv_width,
            conv_height,
        );
        sod_free_image(img);
        free(resized_frame as *mut ::core::ffi::c_void);
        if !resize_ctx.is_null() {
            sws_freeContext(resize_ctx);
        }
        return -1 as ::core::ffi::c_int;
    }
    let mut total_size: size_t = pixel_count.wrapping_mul(channels as size_t);
    if total_size.wrapping_div(pixel_count) != channels as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Integer overflow in total size calculation: width=%d, height=%d, channels=%d\0"
                .as_ptr() as *const ::core::ffi::c_char,
            conv_width,
            conv_height,
            channels,
        );
        sod_free_image(img);
        free(resized_frame as *mut ::core::ffi::c_void);
        if !resize_ctx.is_null() {
            sws_freeContext(resize_ctx);
        }
        return -1 as ::core::ffi::c_int;
    }
    let mut temp_buffer: *mut ::core::ffi::c_float =
        malloc(total_size.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()))
            as *mut ::core::ffi::c_float;
    if temp_buffer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate temporary buffer for image data conversion (size=%zu bytes)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            total_size.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()),
        );
        sod_free_image(img);
        free(resized_frame as *mut ::core::ffi::c_void);
        if !resize_ctx.is_null() {
            sws_freeContext(resize_ctx);
        }
        return -1 as ::core::ffi::c_int;
    }
    memset(
        temp_buffer as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        total_size.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()),
    );
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < channels {
        let mut h: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while h < conv_height {
            let mut w: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while w < conv_width {
                let mut c_offset: size_t = (c as size_t)
                    .wrapping_mul(conv_height as size_t)
                    .wrapping_mul(conv_width as size_t);
                let mut h_offset: size_t = (h as size_t).wrapping_mul(conv_width as size_t);
                let mut sod_idx: size_t = c_offset.wrapping_add(h_offset).wrapping_add(w as size_t);
                let mut row_offset: size_t = (h as size_t)
                    .wrapping_mul(conv_width as size_t)
                    .wrapping_mul(channels as size_t);
                let mut pixel_offset: size_t = (w as size_t).wrapping_mul(channels as size_t);
                let mut frame_idx: size_t = row_offset
                    .wrapping_add(pixel_offset)
                    .wrapping_add(c as size_t);
                if sod_idx < total_size && frame_idx < total_size {
                    *temp_buffer.offset(sod_idx as isize) = *conv_source.offset(frame_idx as isize)
                        as ::core::ffi::c_int
                        as ::core::ffi::c_float
                        / 255.0f32;
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Index out of bounds: sod_idx=%zu, frame_idx=%zu, total_size=%zu\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        sod_idx,
                        frame_idx,
                        total_size,
                    );
                }
                w += 1;
            }
            h += 1;
        }
        c += 1;
    }
    if !img.c2rust_unnamed.data.is_null() {
        memcpy(
            img.c2rust_unnamed.data as *mut ::core::ffi::c_void,
            temp_buffer as *const ::core::ffi::c_void,
            total_size.wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"SOD image data pointer is NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(temp_buffer as *mut ::core::ffi::c_void);
        sod_free_image(img);
        free(resized_frame as *mut ::core::ffi::c_void);
        if !resize_ctx.is_null() {
            sws_freeContext(resize_ctx);
        }
        return -1 as ::core::ffi::c_int;
    }
    free(temp_buffer as *mut ::core::ffi::c_void);
    free(resized_frame as *mut ::core::ffi::c_void);
    if !resize_ctx.is_null() {
        sws_freeContext(resize_ctx);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Step 3: Successfully copied frame data to SOD image\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Step 4: Preparing image for CNN detection with model=%p\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*m).c2rust_unnamed.sod,
    );
    let mut prepared_data: *mut ::core::ffi::c_float =
        ::core::ptr::null_mut::<::core::ffi::c_float>();
    if (*m).c2rust_unnamed.sod.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Model pointer is NULL before preparing image\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        sod_free_image(img);
        return -1 as ::core::ffi::c_int;
    }
    prepared_data = sod_cnn_prepare_image((*m).c2rust_unnamed.sod as *mut sod_cnn, img);
    if prepared_data.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare image for CNN detection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sod_free_image(img);
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Step 5: Successfully prepared image for CNN detection\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Step 6: Running CNN detection\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut boxes_ptr: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if (*m).c2rust_unnamed.sod.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Model pointer is NULL before prediction\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sod_free_image(img);
        return -1 as ::core::ffi::c_int;
    }
    let mut boxes: *mut sod_box = ::core::ptr::null_mut::<sod_box>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = sod_cnn_predict(
        (*m).c2rust_unnamed.sod as *mut sod_cnn,
        prepared_data,
        &raw mut boxes,
        &raw mut count,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Step 7: sod_cnn_predict returned with rc=%d, count=%d\0".as_ptr()
            as *const ::core::ffi::c_char,
        rc,
        count,
    );
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"CNN detection failed with error code: %d\0".as_ptr() as *const ::core::ffi::c_char,
            rc,
        );
        sod_free_image(img);
        return -1 as ::core::ffi::c_int;
    }
    if boxes.is_null() && count > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Boxes pointer is NULL but count is %d\0".as_ptr() as *const ::core::ffi::c_char,
            count,
        );
        count = 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Step 8: Processing detection results\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*result).count = 0 as ::core::ffi::c_int;
    let mut valid_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if count <= 0 as ::core::ffi::c_int || boxes.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"No detection boxes returned (count=%d, boxes=%p)\0".as_ptr()
                as *const ::core::ffi::c_char,
            count,
            boxes as *mut ::core::ffi::c_void,
        );
        sod_free_image(img);
        (*result).count = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Processing %d detection boxes\0".as_ptr() as *const ::core::ffi::c_char,
        count,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count && valid_count < MAX_DETECTIONS {
        let mut r#box: *mut sod_box = boxes.offset(i as isize);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Box %d: x=%d, y=%d, w=%d, h=%d, score=%.2f, name=%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            i,
            (*r#box).x,
            (*r#box).y,
            (*r#box).w,
            (*r#box).h,
            (*r#box).score as ::core::ffi::c_double,
            if !(*r#box).zName.is_null() {
                (*r#box).zName
            } else {
                b"unknown\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if (*r#box).x < 0 as ::core::ffi::c_int
            || (*r#box).y < 0 as ::core::ffi::c_int
            || (*r#box).w <= 0 as ::core::ffi::c_int
            || (*r#box).h <= 0 as ::core::ffi::c_int
            || (*r#box).x + (*r#box).w > conv_width
            || (*r#box).y + (*r#box).h > conv_height
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Box %d has invalid coordinates (x=%d, y=%d, w=%d, h=%d, img_w=%d, img_h=%d), skipping\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                i,
                (*r#box).x,
                (*r#box).y,
                (*r#box).w,
                (*r#box).h,
                conv_width,
                conv_height,
            );
        } else {
            if (*r#box).zName.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Box %d has NULL name, using 'unknown'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    i,
                );
            }
            let mut label: [::core::ffi::c_char; 32] = [0; 32];
            let mut name: *const ::core::ffi::c_char = if !(*r#box).zName.is_null() {
                (*r#box).zName
            } else {
                b"object\0".as_ptr() as *const ::core::ffi::c_char
            };
            if !name.is_null() && strlen(name) > 0 as size_t {
                safe_strcpy(
                    &raw mut label as *mut ::core::ffi::c_char,
                    name,
                    MAX_LABEL_LENGTH as size_t,
                    0 as size_t,
                );
            } else {
                safe_strcpy(
                    &raw mut label as *mut ::core::ffi::c_char,
                    b"object\0".as_ptr() as *const ::core::ffi::c_char,
                    MAX_LABEL_LENGTH as size_t,
                    0 as size_t,
                );
            }
            let mut confidence: ::core::ffi::c_float = (*r#box).score;
            if confidence > 1.0f32 {
                confidence = 1.0f32;
            }
            if confidence < 0.0f32 {
                confidence = 0.0f32;
            }
            let mut x: ::core::ffi::c_float = if conv_width > 0 as ::core::ffi::c_int {
                (*r#box).x as ::core::ffi::c_float / conv_width as ::core::ffi::c_float
            } else {
                0.0f32
            };
            let mut y: ::core::ffi::c_float = if conv_height > 0 as ::core::ffi::c_int {
                (*r#box).y as ::core::ffi::c_float / conv_height as ::core::ffi::c_float
            } else {
                0.0f32
            };
            let mut w_0: ::core::ffi::c_float = if conv_width > 0 as ::core::ffi::c_int {
                (*r#box).w as ::core::ffi::c_float / conv_width as ::core::ffi::c_float
            } else {
                0.0f32
            };
            let mut h_0: ::core::ffi::c_float = if conv_height > 0 as ::core::ffi::c_int {
                (*r#box).h as ::core::ffi::c_float / conv_height as ::core::ffi::c_float
            } else {
                0.0f32
            };
            x = if x < 0.0f32 {
                0.0f32
            } else if x > 1.0f32 {
                1.0f32
            } else {
                x
            };
            y = if y < 0.0f32 {
                0.0f32
            } else if y > 1.0f32 {
                1.0f32
            } else {
                y
            };
            w_0 = if w_0 < 0.0f32 {
                0.0f32
            } else if w_0 > 1.0f32 {
                1.0f32
            } else {
                w_0
            };
            h_0 = if h_0 < 0.0f32 {
                0.0f32
            } else if h_0 > 1.0f32 {
                1.0f32
            } else {
                h_0
            };
            if confidence < (*m).threshold {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Detection %d below threshold: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    i,
                    &raw mut label as *mut ::core::ffi::c_char,
                    (confidence * 100.0f32) as ::core::ffi::c_double,
                    x as ::core::ffi::c_double,
                    y as ::core::ffi::c_double,
                    w_0 as ::core::ffi::c_double,
                    h_0 as ::core::ffi::c_double,
                );
            } else {
                safe_strcpy(
                    &raw mut (*(&raw mut (*result).detections as *mut detection_t)
                        .offset(valid_count as isize))
                    .label as *mut ::core::ffi::c_char,
                    &raw mut label as *mut ::core::ffi::c_char,
                    MAX_LABEL_LENGTH as size_t,
                    0 as size_t,
                );
                (*result).detections[valid_count as usize].confidence = confidence;
                (*result).detections[valid_count as usize].x = x;
                (*result).detections[valid_count as usize].y = y;
                (*result).detections[valid_count as usize].width = w_0;
                (*result).detections[valid_count as usize].height = h_0;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Valid detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    valid_count,
                    &raw mut label as *mut ::core::ffi::c_char,
                    (confidence * 100.0f32) as ::core::ffi::c_double,
                    x as ::core::ffi::c_double,
                    y as ::core::ffi::c_double,
                    w_0 as ::core::ffi::c_double,
                    h_0 as ::core::ffi::c_double,
                );
                valid_count += 1;
            }
        }
        i += 1;
    }
    (*result).count = valid_count;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Detection found %d valid objects out of %d total\0".as_ptr()
            as *const ::core::ffi::c_char,
        valid_count,
        count,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Step 9: Freeing SOD image and prepared data\0".as_ptr() as *const ::core::ffi::c_char,
    );
    sod_free_image(img);
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
