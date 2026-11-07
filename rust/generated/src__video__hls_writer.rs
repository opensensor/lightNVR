unsafe extern "C" {
    pub type AVOptionRanges;
    pub type AVOption;
    pub type AVBuffer;
    pub type AVDictionary;
    pub type AVIAMFMixPresentation;
    pub type AVIAMFAudioElement;
    pub type AVCodecTag;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_timedlock(
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn av_strerror(
        errnum: ::core::ffi::c_int,
        errbuf: *mut ::core::ffi::c_char,
        errbuf_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn avcodec_get_name(id: AVCodecID) -> *const ::core::ffi::c_char;
    unsafe fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn av_dict_free(m: *mut *mut AVDictionary);
    unsafe fn av_packet_alloc() -> *mut AVPacket;
    unsafe fn av_packet_free(pkt: *mut *mut AVPacket);
    unsafe fn av_new_packet(pkt: *mut AVPacket, size: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn av_packet_ref(dst: *mut AVPacket, src: *const AVPacket) -> ::core::ffi::c_int;
    unsafe fn av_packet_rescale_ts(pkt: *mut AVPacket, tb_src: AVRational, tb_dst: AVRational);
    unsafe fn avcodec_parameters_free(par: *mut *mut AVCodecParameters);
    unsafe fn avcodec_parameters_copy(
        dst: *mut AVCodecParameters,
        src: *const AVCodecParameters,
    ) -> ::core::ffi::c_int;
    unsafe fn avio_open2(
        s: *mut *mut AVIOContext,
        url: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        int_cb: *const AVIOInterruptCB,
        options: *mut *mut AVDictionary,
    ) -> ::core::ffi::c_int;
    unsafe fn avio_closep(s: *mut *mut AVIOContext) -> ::core::ffi::c_int;
    unsafe fn avformat_free_context(s: *mut AVFormatContext);
    unsafe fn avformat_new_stream(s: *mut AVFormatContext, c: *const AVCodec) -> *mut AVStream;
    unsafe fn avformat_alloc_output_context2(
        ctx: *mut *mut AVFormatContext,
        oformat: *const AVOutputFormat,
        format_name: *const ::core::ffi::c_char,
        filename: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn avformat_write_header(
        s: *mut AVFormatContext,
        options: *mut *mut AVDictionary,
    ) -> ::core::ffi::c_int;
    unsafe fn av_interleaved_write_frame(
        s: *mut AVFormatContext,
        pkt: *mut AVPacket,
    ) -> ::core::ffi::c_int;
    unsafe fn av_write_trailer(s: *mut AVFormatContext) -> ::core::ffi::c_int;
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
    unsafe fn ensure_hls_directory(
        output_dir: *mut ::core::ffi::c_char,
        output_dir_size: size_t,
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn av_bsf_free(ctx: *mut *mut AVBSFContext);
    unsafe fn recording_io_report_failure(
        resource: recording_io_resource_t,
        operation: recording_io_operation_t,
        error_code: ::core::ffi::c_int,
    );
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
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type int64_t = i64;
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
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVMediaType(pub ::core::ffi::c_int);
impl AVMediaType {
    pub const AVMEDIA_TYPE_UNKNOWN: Self = Self(-1);
    pub const AVMEDIA_TYPE_VIDEO: Self = Self(0);
    pub const AVMEDIA_TYPE_AUDIO: Self = Self(1);
    pub const AVMEDIA_TYPE_DATA: Self = Self(2);
    pub const AVMEDIA_TYPE_SUBTITLE: Self = Self(3);
    pub const AVMEDIA_TYPE_ATTACHMENT: Self = Self(4);
    pub const AVMEDIA_TYPE_NB: Self = Self(5);
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVColorPrimaries(pub ::core::ffi::c_uint);
impl AVColorPrimaries {
    pub const AVCOL_PRI_RESERVED0: Self = Self(0);
    pub const AVCOL_PRI_BT709: Self = Self(1);
    pub const AVCOL_PRI_UNSPECIFIED: Self = Self(2);
    pub const AVCOL_PRI_RESERVED: Self = Self(3);
    pub const AVCOL_PRI_BT470M: Self = Self(4);
    pub const AVCOL_PRI_BT470BG: Self = Self(5);
    pub const AVCOL_PRI_SMPTE170M: Self = Self(6);
    pub const AVCOL_PRI_SMPTE240M: Self = Self(7);
    pub const AVCOL_PRI_FILM: Self = Self(8);
    pub const AVCOL_PRI_BT2020: Self = Self(9);
    pub const AVCOL_PRI_SMPTE428: Self = Self(10);
    pub const AVCOL_PRI_SMPTEST428_1: Self = Self(10);
    pub const AVCOL_PRI_SMPTE431: Self = Self(11);
    pub const AVCOL_PRI_SMPTE432: Self = Self(12);
    pub const AVCOL_PRI_EBU3213: Self = Self(22);
    pub const AVCOL_PRI_JEDEC_P22: Self = Self(22);
    pub const AVCOL_PRI_NB: Self = Self(23);
    pub const AVCOL_PRI_EXT_BASE: Self = Self(256);
    pub const AVCOL_PRI_V_GAMUT: Self = Self(256);
    pub const AVCOL_PRI_EXT_NB: Self = Self(257);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVColorTransferCharacteristic(pub ::core::ffi::c_uint);
impl AVColorTransferCharacteristic {
    pub const AVCOL_TRC_RESERVED0: Self = Self(0);
    pub const AVCOL_TRC_BT709: Self = Self(1);
    pub const AVCOL_TRC_UNSPECIFIED: Self = Self(2);
    pub const AVCOL_TRC_RESERVED: Self = Self(3);
    pub const AVCOL_TRC_GAMMA22: Self = Self(4);
    pub const AVCOL_TRC_GAMMA28: Self = Self(5);
    pub const AVCOL_TRC_SMPTE170M: Self = Self(6);
    pub const AVCOL_TRC_SMPTE240M: Self = Self(7);
    pub const AVCOL_TRC_LINEAR: Self = Self(8);
    pub const AVCOL_TRC_LOG: Self = Self(9);
    pub const AVCOL_TRC_LOG_SQRT: Self = Self(10);
    pub const AVCOL_TRC_IEC61966_2_4: Self = Self(11);
    pub const AVCOL_TRC_BT1361_ECG: Self = Self(12);
    pub const AVCOL_TRC_IEC61966_2_1: Self = Self(13);
    pub const AVCOL_TRC_BT2020_10: Self = Self(14);
    pub const AVCOL_TRC_BT2020_12: Self = Self(15);
    pub const AVCOL_TRC_SMPTE2084: Self = Self(16);
    pub const AVCOL_TRC_SMPTEST2084: Self = Self(16);
    pub const AVCOL_TRC_SMPTE428: Self = Self(17);
    pub const AVCOL_TRC_SMPTEST428_1: Self = Self(17);
    pub const AVCOL_TRC_ARIB_STD_B67: Self = Self(18);
    pub const AVCOL_TRC_NB: Self = Self(19);
    pub const AVCOL_TRC_EXT_BASE: Self = Self(256);
    pub const AVCOL_TRC_V_LOG: Self = Self(256);
    pub const AVCOL_TRC_EXT_NB: Self = Self(257);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVColorSpace(pub ::core::ffi::c_uint);
impl AVColorSpace {
    pub const AVCOL_SPC_RGB: Self = Self(0);
    pub const AVCOL_SPC_BT709: Self = Self(1);
    pub const AVCOL_SPC_UNSPECIFIED: Self = Self(2);
    pub const AVCOL_SPC_RESERVED: Self = Self(3);
    pub const AVCOL_SPC_FCC: Self = Self(4);
    pub const AVCOL_SPC_BT470BG: Self = Self(5);
    pub const AVCOL_SPC_SMPTE170M: Self = Self(6);
    pub const AVCOL_SPC_SMPTE240M: Self = Self(7);
    pub const AVCOL_SPC_YCGCO: Self = Self(8);
    pub const AVCOL_SPC_YCOCG: Self = Self(8);
    pub const AVCOL_SPC_BT2020_NCL: Self = Self(9);
    pub const AVCOL_SPC_BT2020_CL: Self = Self(10);
    pub const AVCOL_SPC_SMPTE2085: Self = Self(11);
    pub const AVCOL_SPC_CHROMA_DERIVED_NCL: Self = Self(12);
    pub const AVCOL_SPC_CHROMA_DERIVED_CL: Self = Self(13);
    pub const AVCOL_SPC_ICTCP: Self = Self(14);
    pub const AVCOL_SPC_IPT_C2: Self = Self(15);
    pub const AVCOL_SPC_YCGCO_RE: Self = Self(16);
    pub const AVCOL_SPC_YCGCO_RO: Self = Self(17);
    pub const AVCOL_SPC_NB: Self = Self(18);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVColorRange(pub ::core::ffi::c_uint);
impl AVColorRange {
    pub const AVCOL_RANGE_UNSPECIFIED: Self = Self(0);
    pub const AVCOL_RANGE_MPEG: Self = Self(1);
    pub const AVCOL_RANGE_JPEG: Self = Self(2);
    pub const AVCOL_RANGE_NB: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVChromaLocation(pub ::core::ffi::c_uint);
impl AVChromaLocation {
    pub const AVCHROMA_LOC_UNSPECIFIED: Self = Self(0);
    pub const AVCHROMA_LOC_LEFT: Self = Self(1);
    pub const AVCHROMA_LOC_CENTER: Self = Self(2);
    pub const AVCHROMA_LOC_TOPLEFT: Self = Self(3);
    pub const AVCHROMA_LOC_TOP: Self = Self(4);
    pub const AVCHROMA_LOC_BOTTOMLEFT: Self = Self(5);
    pub const AVCHROMA_LOC_BOTTOM: Self = Self(6);
    pub const AVCHROMA_LOC_NB: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVAlphaMode(pub ::core::ffi::c_uint);
impl AVAlphaMode {
    pub const AVALPHA_MODE_UNSPECIFIED: Self = Self(0);
    pub const AVALPHA_MODE_PREMULTIPLIED: Self = Self(1);
    pub const AVALPHA_MODE_STRAIGHT: Self = Self(2);
    pub const AVALPHA_MODE_NB: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVChannel(pub ::core::ffi::c_int);
impl AVChannel {
    pub const AV_CHAN_NONE: Self = Self(-1);
    pub const AV_CHAN_FRONT_LEFT: Self = Self(0);
    pub const AV_CHAN_FRONT_RIGHT: Self = Self(1);
    pub const AV_CHAN_FRONT_CENTER: Self = Self(2);
    pub const AV_CHAN_LOW_FREQUENCY: Self = Self(3);
    pub const AV_CHAN_BACK_LEFT: Self = Self(4);
    pub const AV_CHAN_BACK_RIGHT: Self = Self(5);
    pub const AV_CHAN_FRONT_LEFT_OF_CENTER: Self = Self(6);
    pub const AV_CHAN_FRONT_RIGHT_OF_CENTER: Self = Self(7);
    pub const AV_CHAN_BACK_CENTER: Self = Self(8);
    pub const AV_CHAN_SIDE_LEFT: Self = Self(9);
    pub const AV_CHAN_SIDE_RIGHT: Self = Self(10);
    pub const AV_CHAN_TOP_CENTER: Self = Self(11);
    pub const AV_CHAN_TOP_FRONT_LEFT: Self = Self(12);
    pub const AV_CHAN_TOP_FRONT_CENTER: Self = Self(13);
    pub const AV_CHAN_TOP_FRONT_RIGHT: Self = Self(14);
    pub const AV_CHAN_TOP_BACK_LEFT: Self = Self(15);
    pub const AV_CHAN_TOP_BACK_CENTER: Self = Self(16);
    pub const AV_CHAN_TOP_BACK_RIGHT: Self = Self(17);
    pub const AV_CHAN_STEREO_LEFT: Self = Self(29);
    pub const AV_CHAN_STEREO_RIGHT: Self = Self(30);
    pub const AV_CHAN_WIDE_LEFT: Self = Self(31);
    pub const AV_CHAN_WIDE_RIGHT: Self = Self(32);
    pub const AV_CHAN_SURROUND_DIRECT_LEFT: Self = Self(33);
    pub const AV_CHAN_SURROUND_DIRECT_RIGHT: Self = Self(34);
    pub const AV_CHAN_LOW_FREQUENCY_2: Self = Self(35);
    pub const AV_CHAN_TOP_SIDE_LEFT: Self = Self(36);
    pub const AV_CHAN_TOP_SIDE_RIGHT: Self = Self(37);
    pub const AV_CHAN_BOTTOM_FRONT_CENTER: Self = Self(38);
    pub const AV_CHAN_BOTTOM_FRONT_LEFT: Self = Self(39);
    pub const AV_CHAN_BOTTOM_FRONT_RIGHT: Self = Self(40);
    pub const AV_CHAN_SIDE_SURROUND_LEFT: Self = Self(41);
    pub const AV_CHAN_SIDE_SURROUND_RIGHT: Self = Self(42);
    pub const AV_CHAN_TOP_SURROUND_LEFT: Self = Self(43);
    pub const AV_CHAN_TOP_SURROUND_RIGHT: Self = Self(44);
    pub const AV_CHAN_BINAURAL_LEFT: Self = Self(61);
    pub const AV_CHAN_BINAURAL_RIGHT: Self = Self(62);
    pub const AV_CHAN_UNUSED: Self = Self(512);
    pub const AV_CHAN_UNKNOWN: Self = Self(768);
    pub const AV_CHAN_AMBISONIC_BASE: Self = Self(1024);
    pub const AV_CHAN_AMBISONIC_END: Self = Self(2047);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVChannelOrder(pub ::core::ffi::c_uint);
impl AVChannelOrder {
    pub const AV_CHANNEL_ORDER_UNSPEC: Self = Self(0);
    pub const AV_CHANNEL_ORDER_NATIVE: Self = Self(1);
    pub const AV_CHANNEL_ORDER_CUSTOM: Self = Self(2);
    pub const AV_CHANNEL_ORDER_AMBISONIC: Self = Self(3);
    pub const FF_CHANNEL_ORDER_NB: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelCustom {
    pub id: AVChannel,
    pub name: [::core::ffi::c_char; 16],
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelLayout {
    pub order: AVChannelOrder,
    pub nb_channels: ::core::ffi::c_int,
    pub u: C2Rust_Unnamed_0,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub mask: uint64_t,
    pub map: *mut AVChannelCustom,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVCodecID(pub ::core::ffi::c_uint);
impl AVCodecID {
    pub const AV_CODEC_ID_NONE: Self = Self(0);
    pub const AV_CODEC_ID_MPEG1VIDEO: Self = Self(1);
    pub const AV_CODEC_ID_MPEG2VIDEO: Self = Self(2);
    pub const AV_CODEC_ID_H261: Self = Self(3);
    pub const AV_CODEC_ID_H263: Self = Self(4);
    pub const AV_CODEC_ID_RV10: Self = Self(5);
    pub const AV_CODEC_ID_RV20: Self = Self(6);
    pub const AV_CODEC_ID_MJPEG: Self = Self(7);
    pub const AV_CODEC_ID_MJPEGB: Self = Self(8);
    pub const AV_CODEC_ID_LJPEG: Self = Self(9);
    pub const AV_CODEC_ID_SP5X: Self = Self(10);
    pub const AV_CODEC_ID_JPEGLS: Self = Self(11);
    pub const AV_CODEC_ID_MPEG4: Self = Self(12);
    pub const AV_CODEC_ID_RAWVIDEO: Self = Self(13);
    pub const AV_CODEC_ID_MSMPEG4V1: Self = Self(14);
    pub const AV_CODEC_ID_MSMPEG4V2: Self = Self(15);
    pub const AV_CODEC_ID_MSMPEG4V3: Self = Self(16);
    pub const AV_CODEC_ID_WMV1: Self = Self(17);
    pub const AV_CODEC_ID_WMV2: Self = Self(18);
    pub const AV_CODEC_ID_H263P: Self = Self(19);
    pub const AV_CODEC_ID_H263I: Self = Self(20);
    pub const AV_CODEC_ID_FLV1: Self = Self(21);
    pub const AV_CODEC_ID_SVQ1: Self = Self(22);
    pub const AV_CODEC_ID_SVQ3: Self = Self(23);
    pub const AV_CODEC_ID_DVVIDEO: Self = Self(24);
    pub const AV_CODEC_ID_HUFFYUV: Self = Self(25);
    pub const AV_CODEC_ID_CYUV: Self = Self(26);
    pub const AV_CODEC_ID_H264: Self = Self(27);
    pub const AV_CODEC_ID_INDEO3: Self = Self(28);
    pub const AV_CODEC_ID_VP3: Self = Self(29);
    pub const AV_CODEC_ID_THEORA: Self = Self(30);
    pub const AV_CODEC_ID_ASV1: Self = Self(31);
    pub const AV_CODEC_ID_ASV2: Self = Self(32);
    pub const AV_CODEC_ID_FFV1: Self = Self(33);
    pub const AV_CODEC_ID_4XM: Self = Self(34);
    pub const AV_CODEC_ID_VCR1: Self = Self(35);
    pub const AV_CODEC_ID_CLJR: Self = Self(36);
    pub const AV_CODEC_ID_MDEC: Self = Self(37);
    pub const AV_CODEC_ID_ROQ: Self = Self(38);
    pub const AV_CODEC_ID_INTERPLAY_VIDEO: Self = Self(39);
    pub const AV_CODEC_ID_XAN_WC3: Self = Self(40);
    pub const AV_CODEC_ID_XAN_WC4: Self = Self(41);
    pub const AV_CODEC_ID_RPZA: Self = Self(42);
    pub const AV_CODEC_ID_CINEPAK: Self = Self(43);
    pub const AV_CODEC_ID_WS_VQA: Self = Self(44);
    pub const AV_CODEC_ID_MSRLE: Self = Self(45);
    pub const AV_CODEC_ID_MSVIDEO1: Self = Self(46);
    pub const AV_CODEC_ID_IDCIN: Self = Self(47);
    pub const AV_CODEC_ID_8BPS: Self = Self(48);
    pub const AV_CODEC_ID_SMC: Self = Self(49);
    pub const AV_CODEC_ID_FLIC: Self = Self(50);
    pub const AV_CODEC_ID_TRUEMOTION1: Self = Self(51);
    pub const AV_CODEC_ID_VMDVIDEO: Self = Self(52);
    pub const AV_CODEC_ID_MSZH: Self = Self(53);
    pub const AV_CODEC_ID_ZLIB: Self = Self(54);
    pub const AV_CODEC_ID_QTRLE: Self = Self(55);
    pub const AV_CODEC_ID_TSCC: Self = Self(56);
    pub const AV_CODEC_ID_ULTI: Self = Self(57);
    pub const AV_CODEC_ID_QDRAW: Self = Self(58);
    pub const AV_CODEC_ID_VIXL: Self = Self(59);
    pub const AV_CODEC_ID_QPEG: Self = Self(60);
    pub const AV_CODEC_ID_PNG: Self = Self(61);
    pub const AV_CODEC_ID_PPM: Self = Self(62);
    pub const AV_CODEC_ID_PBM: Self = Self(63);
    pub const AV_CODEC_ID_PGM: Self = Self(64);
    pub const AV_CODEC_ID_PGMYUV: Self = Self(65);
    pub const AV_CODEC_ID_PAM: Self = Self(66);
    pub const AV_CODEC_ID_FFVHUFF: Self = Self(67);
    pub const AV_CODEC_ID_RV30: Self = Self(68);
    pub const AV_CODEC_ID_RV40: Self = Self(69);
    pub const AV_CODEC_ID_VC1: Self = Self(70);
    pub const AV_CODEC_ID_WMV3: Self = Self(71);
    pub const AV_CODEC_ID_LOCO: Self = Self(72);
    pub const AV_CODEC_ID_WNV1: Self = Self(73);
    pub const AV_CODEC_ID_AASC: Self = Self(74);
    pub const AV_CODEC_ID_INDEO2: Self = Self(75);
    pub const AV_CODEC_ID_FRAPS: Self = Self(76);
    pub const AV_CODEC_ID_TRUEMOTION2: Self = Self(77);
    pub const AV_CODEC_ID_BMP: Self = Self(78);
    pub const AV_CODEC_ID_CSCD: Self = Self(79);
    pub const AV_CODEC_ID_MMVIDEO: Self = Self(80);
    pub const AV_CODEC_ID_ZMBV: Self = Self(81);
    pub const AV_CODEC_ID_AVS: Self = Self(82);
    pub const AV_CODEC_ID_SMACKVIDEO: Self = Self(83);
    pub const AV_CODEC_ID_NUV: Self = Self(84);
    pub const AV_CODEC_ID_KMVC: Self = Self(85);
    pub const AV_CODEC_ID_FLASHSV: Self = Self(86);
    pub const AV_CODEC_ID_CAVS: Self = Self(87);
    pub const AV_CODEC_ID_JPEG2000: Self = Self(88);
    pub const AV_CODEC_ID_VMNC: Self = Self(89);
    pub const AV_CODEC_ID_VP5: Self = Self(90);
    pub const AV_CODEC_ID_VP6: Self = Self(91);
    pub const AV_CODEC_ID_VP6F: Self = Self(92);
    pub const AV_CODEC_ID_TARGA: Self = Self(93);
    pub const AV_CODEC_ID_DSICINVIDEO: Self = Self(94);
    pub const AV_CODEC_ID_TIERTEXSEQVIDEO: Self = Self(95);
    pub const AV_CODEC_ID_TIFF: Self = Self(96);
    pub const AV_CODEC_ID_GIF: Self = Self(97);
    pub const AV_CODEC_ID_DXA: Self = Self(98);
    pub const AV_CODEC_ID_DNXHD: Self = Self(99);
    pub const AV_CODEC_ID_THP: Self = Self(100);
    pub const AV_CODEC_ID_SGI: Self = Self(101);
    pub const AV_CODEC_ID_C93: Self = Self(102);
    pub const AV_CODEC_ID_BETHSOFTVID: Self = Self(103);
    pub const AV_CODEC_ID_PTX: Self = Self(104);
    pub const AV_CODEC_ID_TXD: Self = Self(105);
    pub const AV_CODEC_ID_VP6A: Self = Self(106);
    pub const AV_CODEC_ID_AMV: Self = Self(107);
    pub const AV_CODEC_ID_VB: Self = Self(108);
    pub const AV_CODEC_ID_PCX: Self = Self(109);
    pub const AV_CODEC_ID_SUNRAST: Self = Self(110);
    pub const AV_CODEC_ID_INDEO4: Self = Self(111);
    pub const AV_CODEC_ID_INDEO5: Self = Self(112);
    pub const AV_CODEC_ID_MIMIC: Self = Self(113);
    pub const AV_CODEC_ID_RL2: Self = Self(114);
    pub const AV_CODEC_ID_ESCAPE124: Self = Self(115);
    pub const AV_CODEC_ID_DIRAC: Self = Self(116);
    pub const AV_CODEC_ID_BFI: Self = Self(117);
    pub const AV_CODEC_ID_CMV: Self = Self(118);
    pub const AV_CODEC_ID_MOTIONPIXELS: Self = Self(119);
    pub const AV_CODEC_ID_TGV: Self = Self(120);
    pub const AV_CODEC_ID_TGQ: Self = Self(121);
    pub const AV_CODEC_ID_TQI: Self = Self(122);
    pub const AV_CODEC_ID_AURA: Self = Self(123);
    pub const AV_CODEC_ID_AURA2: Self = Self(124);
    pub const AV_CODEC_ID_V210X: Self = Self(125);
    pub const AV_CODEC_ID_TMV: Self = Self(126);
    pub const AV_CODEC_ID_V210: Self = Self(127);
    pub const AV_CODEC_ID_DPX: Self = Self(128);
    pub const AV_CODEC_ID_MAD: Self = Self(129);
    pub const AV_CODEC_ID_FRWU: Self = Self(130);
    pub const AV_CODEC_ID_FLASHSV2: Self = Self(131);
    pub const AV_CODEC_ID_CDGRAPHICS: Self = Self(132);
    pub const AV_CODEC_ID_R210: Self = Self(133);
    pub const AV_CODEC_ID_ANM: Self = Self(134);
    pub const AV_CODEC_ID_BINKVIDEO: Self = Self(135);
    pub const AV_CODEC_ID_IFF_ILBM: Self = Self(136);
    pub const AV_CODEC_ID_KGV1: Self = Self(137);
    pub const AV_CODEC_ID_YOP: Self = Self(138);
    pub const AV_CODEC_ID_VP8: Self = Self(139);
    pub const AV_CODEC_ID_PICTOR: Self = Self(140);
    pub const AV_CODEC_ID_ANSI: Self = Self(141);
    pub const AV_CODEC_ID_A64_MULTI: Self = Self(142);
    pub const AV_CODEC_ID_A64_MULTI5: Self = Self(143);
    pub const AV_CODEC_ID_R10K: Self = Self(144);
    pub const AV_CODEC_ID_MXPEG: Self = Self(145);
    pub const AV_CODEC_ID_LAGARITH: Self = Self(146);
    pub const AV_CODEC_ID_PRORES: Self = Self(147);
    pub const AV_CODEC_ID_JV: Self = Self(148);
    pub const AV_CODEC_ID_DFA: Self = Self(149);
    pub const AV_CODEC_ID_WMV3IMAGE: Self = Self(150);
    pub const AV_CODEC_ID_VC1IMAGE: Self = Self(151);
    pub const AV_CODEC_ID_UTVIDEO: Self = Self(152);
    pub const AV_CODEC_ID_BMV_VIDEO: Self = Self(153);
    pub const AV_CODEC_ID_VBLE: Self = Self(154);
    pub const AV_CODEC_ID_DXTORY: Self = Self(155);
    pub const AV_CODEC_ID_XWD: Self = Self(156);
    pub const AV_CODEC_ID_CDXL: Self = Self(157);
    pub const AV_CODEC_ID_XBM: Self = Self(158);
    pub const AV_CODEC_ID_ZEROCODEC: Self = Self(159);
    pub const AV_CODEC_ID_MSS1: Self = Self(160);
    pub const AV_CODEC_ID_MSA1: Self = Self(161);
    pub const AV_CODEC_ID_TSCC2: Self = Self(162);
    pub const AV_CODEC_ID_MTS2: Self = Self(163);
    pub const AV_CODEC_ID_CLLC: Self = Self(164);
    pub const AV_CODEC_ID_MSS2: Self = Self(165);
    pub const AV_CODEC_ID_VP9: Self = Self(166);
    pub const AV_CODEC_ID_AIC: Self = Self(167);
    pub const AV_CODEC_ID_ESCAPE130: Self = Self(168);
    pub const AV_CODEC_ID_G2M: Self = Self(169);
    pub const AV_CODEC_ID_WEBP: Self = Self(170);
    pub const AV_CODEC_ID_HNM4_VIDEO: Self = Self(171);
    pub const AV_CODEC_ID_HEVC: Self = Self(172);
    pub const AV_CODEC_ID_FIC: Self = Self(173);
    pub const AV_CODEC_ID_ALIAS_PIX: Self = Self(174);
    pub const AV_CODEC_ID_BRENDER_PIX: Self = Self(175);
    pub const AV_CODEC_ID_PAF_VIDEO: Self = Self(176);
    pub const AV_CODEC_ID_EXR: Self = Self(177);
    pub const AV_CODEC_ID_VP7: Self = Self(178);
    pub const AV_CODEC_ID_SANM: Self = Self(179);
    pub const AV_CODEC_ID_SGIRLE: Self = Self(180);
    pub const AV_CODEC_ID_MVC1: Self = Self(181);
    pub const AV_CODEC_ID_MVC2: Self = Self(182);
    pub const AV_CODEC_ID_HQX: Self = Self(183);
    pub const AV_CODEC_ID_TDSC: Self = Self(184);
    pub const AV_CODEC_ID_HQ_HQA: Self = Self(185);
    pub const AV_CODEC_ID_HAP: Self = Self(186);
    pub const AV_CODEC_ID_DDS: Self = Self(187);
    pub const AV_CODEC_ID_DXV: Self = Self(188);
    pub const AV_CODEC_ID_SCREENPRESSO: Self = Self(189);
    pub const AV_CODEC_ID_RSCC: Self = Self(190);
    pub const AV_CODEC_ID_AVS2: Self = Self(191);
    pub const AV_CODEC_ID_PGX: Self = Self(192);
    pub const AV_CODEC_ID_AVS3: Self = Self(193);
    pub const AV_CODEC_ID_MSP2: Self = Self(194);
    pub const AV_CODEC_ID_VVC: Self = Self(195);
    pub const AV_CODEC_ID_Y41P: Self = Self(196);
    pub const AV_CODEC_ID_AVRP: Self = Self(197);
    pub const AV_CODEC_ID_012V: Self = Self(198);
    pub const AV_CODEC_ID_AVUI: Self = Self(199);
    pub const AV_CODEC_ID_TARGA_Y216: Self = Self(200);
    pub const AV_CODEC_ID_YUV4: Self = Self(201);
    pub const AV_CODEC_ID_AVRN: Self = Self(202);
    pub const AV_CODEC_ID_CPIA: Self = Self(203);
    pub const AV_CODEC_ID_XFACE: Self = Self(204);
    pub const AV_CODEC_ID_SNOW: Self = Self(205);
    pub const AV_CODEC_ID_SMVJPEG: Self = Self(206);
    pub const AV_CODEC_ID_APNG: Self = Self(207);
    pub const AV_CODEC_ID_DAALA: Self = Self(208);
    pub const AV_CODEC_ID_CFHD: Self = Self(209);
    pub const AV_CODEC_ID_TRUEMOTION2RT: Self = Self(210);
    pub const AV_CODEC_ID_M101: Self = Self(211);
    pub const AV_CODEC_ID_MAGICYUV: Self = Self(212);
    pub const AV_CODEC_ID_SHEERVIDEO: Self = Self(213);
    pub const AV_CODEC_ID_YLC: Self = Self(214);
    pub const AV_CODEC_ID_PSD: Self = Self(215);
    pub const AV_CODEC_ID_PIXLET: Self = Self(216);
    pub const AV_CODEC_ID_SPEEDHQ: Self = Self(217);
    pub const AV_CODEC_ID_FMVC: Self = Self(218);
    pub const AV_CODEC_ID_SCPR: Self = Self(219);
    pub const AV_CODEC_ID_CLEARVIDEO: Self = Self(220);
    pub const AV_CODEC_ID_XPM: Self = Self(221);
    pub const AV_CODEC_ID_AV1: Self = Self(222);
    pub const AV_CODEC_ID_BITPACKED: Self = Self(223);
    pub const AV_CODEC_ID_MSCC: Self = Self(224);
    pub const AV_CODEC_ID_SRGC: Self = Self(225);
    pub const AV_CODEC_ID_SVG: Self = Self(226);
    pub const AV_CODEC_ID_GDV: Self = Self(227);
    pub const AV_CODEC_ID_FITS: Self = Self(228);
    pub const AV_CODEC_ID_IMM4: Self = Self(229);
    pub const AV_CODEC_ID_PROSUMER: Self = Self(230);
    pub const AV_CODEC_ID_MWSC: Self = Self(231);
    pub const AV_CODEC_ID_WCMV: Self = Self(232);
    pub const AV_CODEC_ID_RASC: Self = Self(233);
    pub const AV_CODEC_ID_HYMT: Self = Self(234);
    pub const AV_CODEC_ID_ARBC: Self = Self(235);
    pub const AV_CODEC_ID_AGM: Self = Self(236);
    pub const AV_CODEC_ID_LSCR: Self = Self(237);
    pub const AV_CODEC_ID_VP4: Self = Self(238);
    pub const AV_CODEC_ID_IMM5: Self = Self(239);
    pub const AV_CODEC_ID_MVDV: Self = Self(240);
    pub const AV_CODEC_ID_MVHA: Self = Self(241);
    pub const AV_CODEC_ID_CDTOONS: Self = Self(242);
    pub const AV_CODEC_ID_MV30: Self = Self(243);
    pub const AV_CODEC_ID_NOTCHLC: Self = Self(244);
    pub const AV_CODEC_ID_PFM: Self = Self(245);
    pub const AV_CODEC_ID_MOBICLIP: Self = Self(246);
    pub const AV_CODEC_ID_PHOTOCD: Self = Self(247);
    pub const AV_CODEC_ID_IPU: Self = Self(248);
    pub const AV_CODEC_ID_ARGO: Self = Self(249);
    pub const AV_CODEC_ID_CRI: Self = Self(250);
    pub const AV_CODEC_ID_SIMBIOSIS_IMX: Self = Self(251);
    pub const AV_CODEC_ID_SGA_VIDEO: Self = Self(252);
    pub const AV_CODEC_ID_GEM: Self = Self(253);
    pub const AV_CODEC_ID_VBN: Self = Self(254);
    pub const AV_CODEC_ID_JPEGXL: Self = Self(255);
    pub const AV_CODEC_ID_QOI: Self = Self(256);
    pub const AV_CODEC_ID_PHM: Self = Self(257);
    pub const AV_CODEC_ID_RADIANCE_HDR: Self = Self(258);
    pub const AV_CODEC_ID_WBMP: Self = Self(259);
    pub const AV_CODEC_ID_MEDIA100: Self = Self(260);
    pub const AV_CODEC_ID_VQC: Self = Self(261);
    pub const AV_CODEC_ID_PDV: Self = Self(262);
    pub const AV_CODEC_ID_EVC: Self = Self(263);
    pub const AV_CODEC_ID_RTV1: Self = Self(264);
    pub const AV_CODEC_ID_VMIX: Self = Self(265);
    pub const AV_CODEC_ID_LEAD: Self = Self(266);
    pub const AV_CODEC_ID_DNXUC: Self = Self(267);
    pub const AV_CODEC_ID_RV60: Self = Self(268);
    pub const AV_CODEC_ID_JPEGXL_ANIM: Self = Self(269);
    pub const AV_CODEC_ID_APV: Self = Self(270);
    pub const AV_CODEC_ID_PRORES_RAW: Self = Self(271);
    pub const AV_CODEC_ID_JPEGXS: Self = Self(272);
    pub const AV_CODEC_ID_WEBP_ANIM: Self = Self(273);
    pub const AV_CODEC_ID_FIRST_AUDIO: Self = Self(65536);
    pub const AV_CODEC_ID_PCM_S16LE: Self = Self(65536);
    pub const AV_CODEC_ID_PCM_S16BE: Self = Self(65537);
    pub const AV_CODEC_ID_PCM_U16LE: Self = Self(65538);
    pub const AV_CODEC_ID_PCM_U16BE: Self = Self(65539);
    pub const AV_CODEC_ID_PCM_S8: Self = Self(65540);
    pub const AV_CODEC_ID_PCM_U8: Self = Self(65541);
    pub const AV_CODEC_ID_PCM_MULAW: Self = Self(65542);
    pub const AV_CODEC_ID_PCM_ALAW: Self = Self(65543);
    pub const AV_CODEC_ID_PCM_S32LE: Self = Self(65544);
    pub const AV_CODEC_ID_PCM_S32BE: Self = Self(65545);
    pub const AV_CODEC_ID_PCM_U32LE: Self = Self(65546);
    pub const AV_CODEC_ID_PCM_U32BE: Self = Self(65547);
    pub const AV_CODEC_ID_PCM_S24LE: Self = Self(65548);
    pub const AV_CODEC_ID_PCM_S24BE: Self = Self(65549);
    pub const AV_CODEC_ID_PCM_U24LE: Self = Self(65550);
    pub const AV_CODEC_ID_PCM_U24BE: Self = Self(65551);
    pub const AV_CODEC_ID_PCM_S24DAUD: Self = Self(65552);
    pub const AV_CODEC_ID_PCM_ZORK: Self = Self(65553);
    pub const AV_CODEC_ID_PCM_S16LE_PLANAR: Self = Self(65554);
    pub const AV_CODEC_ID_PCM_DVD: Self = Self(65555);
    pub const AV_CODEC_ID_PCM_F32BE: Self = Self(65556);
    pub const AV_CODEC_ID_PCM_F32LE: Self = Self(65557);
    pub const AV_CODEC_ID_PCM_F64BE: Self = Self(65558);
    pub const AV_CODEC_ID_PCM_F64LE: Self = Self(65559);
    pub const AV_CODEC_ID_PCM_BLURAY: Self = Self(65560);
    pub const AV_CODEC_ID_PCM_LXF: Self = Self(65561);
    pub const AV_CODEC_ID_S302M: Self = Self(65562);
    pub const AV_CODEC_ID_PCM_S8_PLANAR: Self = Self(65563);
    pub const AV_CODEC_ID_PCM_S24LE_PLANAR: Self = Self(65564);
    pub const AV_CODEC_ID_PCM_S32LE_PLANAR: Self = Self(65565);
    pub const AV_CODEC_ID_PCM_S16BE_PLANAR: Self = Self(65566);
    pub const AV_CODEC_ID_PCM_S64LE: Self = Self(65567);
    pub const AV_CODEC_ID_PCM_S64BE: Self = Self(65568);
    pub const AV_CODEC_ID_PCM_F16LE: Self = Self(65569);
    pub const AV_CODEC_ID_PCM_F24LE: Self = Self(65570);
    pub const AV_CODEC_ID_PCM_VIDC: Self = Self(65571);
    pub const AV_CODEC_ID_PCM_SGA: Self = Self(65572);
    pub const AV_CODEC_ID_ADPCM_IMA_QT: Self = Self(69632);
    pub const AV_CODEC_ID_ADPCM_IMA_WAV: Self = Self(69633);
    pub const AV_CODEC_ID_ADPCM_IMA_DK3: Self = Self(69634);
    pub const AV_CODEC_ID_ADPCM_IMA_DK4: Self = Self(69635);
    pub const AV_CODEC_ID_ADPCM_IMA_WS: Self = Self(69636);
    pub const AV_CODEC_ID_ADPCM_IMA_SMJPEG: Self = Self(69637);
    pub const AV_CODEC_ID_ADPCM_MS: Self = Self(69638);
    pub const AV_CODEC_ID_ADPCM_4XM: Self = Self(69639);
    pub const AV_CODEC_ID_ADPCM_XA: Self = Self(69640);
    pub const AV_CODEC_ID_ADPCM_ADX: Self = Self(69641);
    pub const AV_CODEC_ID_ADPCM_EA: Self = Self(69642);
    pub const AV_CODEC_ID_ADPCM_G726: Self = Self(69643);
    pub const AV_CODEC_ID_ADPCM_CT: Self = Self(69644);
    pub const AV_CODEC_ID_ADPCM_SWF: Self = Self(69645);
    pub const AV_CODEC_ID_ADPCM_YAMAHA: Self = Self(69646);
    pub const AV_CODEC_ID_ADPCM_SBPRO_4: Self = Self(69647);
    pub const AV_CODEC_ID_ADPCM_SBPRO_3: Self = Self(69648);
    pub const AV_CODEC_ID_ADPCM_SBPRO_2: Self = Self(69649);
    pub const AV_CODEC_ID_ADPCM_THP: Self = Self(69650);
    pub const AV_CODEC_ID_ADPCM_IMA_AMV: Self = Self(69651);
    pub const AV_CODEC_ID_ADPCM_EA_R1: Self = Self(69652);
    pub const AV_CODEC_ID_ADPCM_EA_R3: Self = Self(69653);
    pub const AV_CODEC_ID_ADPCM_EA_R2: Self = Self(69654);
    pub const AV_CODEC_ID_ADPCM_IMA_EA_SEAD: Self = Self(69655);
    pub const AV_CODEC_ID_ADPCM_IMA_EA_EACS: Self = Self(69656);
    pub const AV_CODEC_ID_ADPCM_EA_XAS: Self = Self(69657);
    pub const AV_CODEC_ID_ADPCM_EA_MAXIS_XA: Self = Self(69658);
    pub const AV_CODEC_ID_ADPCM_IMA_ISS: Self = Self(69659);
    pub const AV_CODEC_ID_ADPCM_G722: Self = Self(69660);
    pub const AV_CODEC_ID_ADPCM_IMA_APC: Self = Self(69661);
    pub const AV_CODEC_ID_ADPCM_VIMA: Self = Self(69662);
    pub const AV_CODEC_ID_ADPCM_AFC: Self = Self(69663);
    pub const AV_CODEC_ID_ADPCM_IMA_OKI: Self = Self(69664);
    pub const AV_CODEC_ID_ADPCM_DTK: Self = Self(69665);
    pub const AV_CODEC_ID_ADPCM_IMA_RAD: Self = Self(69666);
    pub const AV_CODEC_ID_ADPCM_G726LE: Self = Self(69667);
    pub const AV_CODEC_ID_ADPCM_THP_LE: Self = Self(69668);
    pub const AV_CODEC_ID_ADPCM_PSX: Self = Self(69669);
    pub const AV_CODEC_ID_ADPCM_AICA: Self = Self(69670);
    pub const AV_CODEC_ID_ADPCM_IMA_DAT4: Self = Self(69671);
    pub const AV_CODEC_ID_ADPCM_MTAF: Self = Self(69672);
    pub const AV_CODEC_ID_ADPCM_AGM: Self = Self(69673);
    pub const AV_CODEC_ID_ADPCM_ARGO: Self = Self(69674);
    pub const AV_CODEC_ID_ADPCM_IMA_SSI: Self = Self(69675);
    pub const AV_CODEC_ID_ADPCM_ZORK: Self = Self(69676);
    pub const AV_CODEC_ID_ADPCM_IMA_APM: Self = Self(69677);
    pub const AV_CODEC_ID_ADPCM_IMA_ALP: Self = Self(69678);
    pub const AV_CODEC_ID_ADPCM_IMA_MTF: Self = Self(69679);
    pub const AV_CODEC_ID_ADPCM_IMA_CUNNING: Self = Self(69680);
    pub const AV_CODEC_ID_ADPCM_IMA_MOFLEX: Self = Self(69681);
    pub const AV_CODEC_ID_ADPCM_IMA_ACORN: Self = Self(69682);
    pub const AV_CODEC_ID_ADPCM_XMD: Self = Self(69683);
    pub const AV_CODEC_ID_ADPCM_IMA_XBOX: Self = Self(69684);
    pub const AV_CODEC_ID_ADPCM_SANYO: Self = Self(69685);
    pub const AV_CODEC_ID_ADPCM_IMA_HVQM4: Self = Self(69686);
    pub const AV_CODEC_ID_ADPCM_IMA_PDA: Self = Self(69687);
    pub const AV_CODEC_ID_ADPCM_N64: Self = Self(69688);
    pub const AV_CODEC_ID_ADPCM_IMA_HVQM2: Self = Self(69689);
    pub const AV_CODEC_ID_ADPCM_IMA_MAGIX: Self = Self(69690);
    pub const AV_CODEC_ID_ADPCM_PSXC: Self = Self(69691);
    pub const AV_CODEC_ID_ADPCM_CIRCUS: Self = Self(69692);
    pub const AV_CODEC_ID_ADPCM_IMA_ESCAPE: Self = Self(69693);
    pub const AV_CODEC_ID_AMR_NB: Self = Self(73728);
    pub const AV_CODEC_ID_AMR_WB: Self = Self(73729);
    pub const AV_CODEC_ID_RA_144: Self = Self(77824);
    pub const AV_CODEC_ID_RA_288: Self = Self(77825);
    pub const AV_CODEC_ID_ROQ_DPCM: Self = Self(81920);
    pub const AV_CODEC_ID_INTERPLAY_DPCM: Self = Self(81921);
    pub const AV_CODEC_ID_XAN_DPCM: Self = Self(81922);
    pub const AV_CODEC_ID_SOL_DPCM: Self = Self(81923);
    pub const AV_CODEC_ID_SDX2_DPCM: Self = Self(81924);
    pub const AV_CODEC_ID_GREMLIN_DPCM: Self = Self(81925);
    pub const AV_CODEC_ID_DERF_DPCM: Self = Self(81926);
    pub const AV_CODEC_ID_WADY_DPCM: Self = Self(81927);
    pub const AV_CODEC_ID_CBD2_DPCM: Self = Self(81928);
    pub const AV_CODEC_ID_MP2: Self = Self(86016);
    pub const AV_CODEC_ID_MP3: Self = Self(86017);
    pub const AV_CODEC_ID_AAC: Self = Self(86018);
    pub const AV_CODEC_ID_AC3: Self = Self(86019);
    pub const AV_CODEC_ID_DTS: Self = Self(86020);
    pub const AV_CODEC_ID_VORBIS: Self = Self(86021);
    pub const AV_CODEC_ID_DVAUDIO: Self = Self(86022);
    pub const AV_CODEC_ID_WMAV1: Self = Self(86023);
    pub const AV_CODEC_ID_WMAV2: Self = Self(86024);
    pub const AV_CODEC_ID_MACE3: Self = Self(86025);
    pub const AV_CODEC_ID_MACE6: Self = Self(86026);
    pub const AV_CODEC_ID_VMDAUDIO: Self = Self(86027);
    pub const AV_CODEC_ID_FLAC: Self = Self(86028);
    pub const AV_CODEC_ID_MP3ADU: Self = Self(86029);
    pub const AV_CODEC_ID_MP3ON4: Self = Self(86030);
    pub const AV_CODEC_ID_SHORTEN: Self = Self(86031);
    pub const AV_CODEC_ID_ALAC: Self = Self(86032);
    pub const AV_CODEC_ID_WESTWOOD_SND1: Self = Self(86033);
    pub const AV_CODEC_ID_GSM: Self = Self(86034);
    pub const AV_CODEC_ID_QDM2: Self = Self(86035);
    pub const AV_CODEC_ID_COOK: Self = Self(86036);
    pub const AV_CODEC_ID_TRUESPEECH: Self = Self(86037);
    pub const AV_CODEC_ID_TTA: Self = Self(86038);
    pub const AV_CODEC_ID_SMACKAUDIO: Self = Self(86039);
    pub const AV_CODEC_ID_QCELP: Self = Self(86040);
    pub const AV_CODEC_ID_WAVPACK: Self = Self(86041);
    pub const AV_CODEC_ID_DSICINAUDIO: Self = Self(86042);
    pub const AV_CODEC_ID_IMC: Self = Self(86043);
    pub const AV_CODEC_ID_MUSEPACK7: Self = Self(86044);
    pub const AV_CODEC_ID_MLP: Self = Self(86045);
    pub const AV_CODEC_ID_GSM_MS: Self = Self(86046);
    pub const AV_CODEC_ID_ATRAC3: Self = Self(86047);
    pub const AV_CODEC_ID_APE: Self = Self(86048);
    pub const AV_CODEC_ID_NELLYMOSER: Self = Self(86049);
    pub const AV_CODEC_ID_MUSEPACK8: Self = Self(86050);
    pub const AV_CODEC_ID_SPEEX: Self = Self(86051);
    pub const AV_CODEC_ID_WMAVOICE: Self = Self(86052);
    pub const AV_CODEC_ID_WMAPRO: Self = Self(86053);
    pub const AV_CODEC_ID_WMALOSSLESS: Self = Self(86054);
    pub const AV_CODEC_ID_ATRAC3P: Self = Self(86055);
    pub const AV_CODEC_ID_EAC3: Self = Self(86056);
    pub const AV_CODEC_ID_SIPR: Self = Self(86057);
    pub const AV_CODEC_ID_MP1: Self = Self(86058);
    pub const AV_CODEC_ID_TWINVQ: Self = Self(86059);
    pub const AV_CODEC_ID_TRUEHD: Self = Self(86060);
    pub const AV_CODEC_ID_MP4ALS: Self = Self(86061);
    pub const AV_CODEC_ID_ATRAC1: Self = Self(86062);
    pub const AV_CODEC_ID_BINKAUDIO_RDFT: Self = Self(86063);
    pub const AV_CODEC_ID_BINKAUDIO_DCT: Self = Self(86064);
    pub const AV_CODEC_ID_AAC_LATM: Self = Self(86065);
    pub const AV_CODEC_ID_QDMC: Self = Self(86066);
    pub const AV_CODEC_ID_CELT: Self = Self(86067);
    pub const AV_CODEC_ID_G723_1: Self = Self(86068);
    pub const AV_CODEC_ID_G729: Self = Self(86069);
    pub const AV_CODEC_ID_8SVX_EXP: Self = Self(86070);
    pub const AV_CODEC_ID_8SVX_FIB: Self = Self(86071);
    pub const AV_CODEC_ID_BMV_AUDIO: Self = Self(86072);
    pub const AV_CODEC_ID_RALF: Self = Self(86073);
    pub const AV_CODEC_ID_IAC: Self = Self(86074);
    pub const AV_CODEC_ID_ILBC: Self = Self(86075);
    pub const AV_CODEC_ID_OPUS: Self = Self(86076);
    pub const AV_CODEC_ID_COMFORT_NOISE: Self = Self(86077);
    pub const AV_CODEC_ID_TAK: Self = Self(86078);
    pub const AV_CODEC_ID_METASOUND: Self = Self(86079);
    pub const AV_CODEC_ID_PAF_AUDIO: Self = Self(86080);
    pub const AV_CODEC_ID_ON2AVC: Self = Self(86081);
    pub const AV_CODEC_ID_DSS_SP: Self = Self(86082);
    pub const AV_CODEC_ID_CODEC2: Self = Self(86083);
    pub const AV_CODEC_ID_FFWAVESYNTH: Self = Self(86084);
    pub const AV_CODEC_ID_SONIC: Self = Self(86085);
    pub const AV_CODEC_ID_SONIC_LS: Self = Self(86086);
    pub const AV_CODEC_ID_EVRC: Self = Self(86087);
    pub const AV_CODEC_ID_SMV: Self = Self(86088);
    pub const AV_CODEC_ID_DSD_LSBF: Self = Self(86089);
    pub const AV_CODEC_ID_DSD_MSBF: Self = Self(86090);
    pub const AV_CODEC_ID_DSD_LSBF_PLANAR: Self = Self(86091);
    pub const AV_CODEC_ID_DSD_MSBF_PLANAR: Self = Self(86092);
    pub const AV_CODEC_ID_4GV: Self = Self(86093);
    pub const AV_CODEC_ID_INTERPLAY_ACM: Self = Self(86094);
    pub const AV_CODEC_ID_XMA1: Self = Self(86095);
    pub const AV_CODEC_ID_XMA2: Self = Self(86096);
    pub const AV_CODEC_ID_DST: Self = Self(86097);
    pub const AV_CODEC_ID_ATRAC3AL: Self = Self(86098);
    pub const AV_CODEC_ID_ATRAC3PAL: Self = Self(86099);
    pub const AV_CODEC_ID_DOLBY_E: Self = Self(86100);
    pub const AV_CODEC_ID_APTX: Self = Self(86101);
    pub const AV_CODEC_ID_APTX_HD: Self = Self(86102);
    pub const AV_CODEC_ID_SBC: Self = Self(86103);
    pub const AV_CODEC_ID_ATRAC9: Self = Self(86104);
    pub const AV_CODEC_ID_HCOM: Self = Self(86105);
    pub const AV_CODEC_ID_ACELP_KELVIN: Self = Self(86106);
    pub const AV_CODEC_ID_MPEGH_3D_AUDIO: Self = Self(86107);
    pub const AV_CODEC_ID_SIREN: Self = Self(86108);
    pub const AV_CODEC_ID_HCA: Self = Self(86109);
    pub const AV_CODEC_ID_FASTAUDIO: Self = Self(86110);
    pub const AV_CODEC_ID_MSNSIREN: Self = Self(86111);
    pub const AV_CODEC_ID_DFPWM: Self = Self(86112);
    pub const AV_CODEC_ID_BONK: Self = Self(86113);
    pub const AV_CODEC_ID_MISC4: Self = Self(86114);
    pub const AV_CODEC_ID_APAC: Self = Self(86115);
    pub const AV_CODEC_ID_FTR: Self = Self(86116);
    pub const AV_CODEC_ID_WAVARC: Self = Self(86117);
    pub const AV_CODEC_ID_RKA: Self = Self(86118);
    pub const AV_CODEC_ID_AC4: Self = Self(86119);
    pub const AV_CODEC_ID_OSQ: Self = Self(86120);
    pub const AV_CODEC_ID_QOA: Self = Self(86121);
    pub const AV_CODEC_ID_LC3: Self = Self(86122);
    pub const AV_CODEC_ID_G728: Self = Self(86123);
    pub const AV_CODEC_ID_AHX: Self = Self(86124);
    pub const AV_CODEC_ID_APPLE_APAC: Self = Self(86125);
    pub const AV_CODEC_ID_FIRST_SUBTITLE: Self = Self(94208);
    pub const AV_CODEC_ID_DVD_SUBTITLE: Self = Self(94208);
    pub const AV_CODEC_ID_DVB_SUBTITLE: Self = Self(94209);
    pub const AV_CODEC_ID_TEXT: Self = Self(94210);
    pub const AV_CODEC_ID_XSUB: Self = Self(94211);
    pub const AV_CODEC_ID_SSA: Self = Self(94212);
    pub const AV_CODEC_ID_MOV_TEXT: Self = Self(94213);
    pub const AV_CODEC_ID_HDMV_PGS_SUBTITLE: Self = Self(94214);
    pub const AV_CODEC_ID_DVB_TELETEXT: Self = Self(94215);
    pub const AV_CODEC_ID_SRT: Self = Self(94216);
    pub const AV_CODEC_ID_MICRODVD: Self = Self(94217);
    pub const AV_CODEC_ID_EIA_608: Self = Self(94218);
    pub const AV_CODEC_ID_JACOSUB: Self = Self(94219);
    pub const AV_CODEC_ID_SAMI: Self = Self(94220);
    pub const AV_CODEC_ID_REALTEXT: Self = Self(94221);
    pub const AV_CODEC_ID_STL: Self = Self(94222);
    pub const AV_CODEC_ID_SUBVIEWER1: Self = Self(94223);
    pub const AV_CODEC_ID_SUBVIEWER: Self = Self(94224);
    pub const AV_CODEC_ID_SUBRIP: Self = Self(94225);
    pub const AV_CODEC_ID_WEBVTT: Self = Self(94226);
    pub const AV_CODEC_ID_MPL2: Self = Self(94227);
    pub const AV_CODEC_ID_VPLAYER: Self = Self(94228);
    pub const AV_CODEC_ID_PJS: Self = Self(94229);
    pub const AV_CODEC_ID_ASS: Self = Self(94230);
    pub const AV_CODEC_ID_HDMV_TEXT_SUBTITLE: Self = Self(94231);
    pub const AV_CODEC_ID_TTML: Self = Self(94232);
    pub const AV_CODEC_ID_ARIB_CAPTION: Self = Self(94233);
    pub const AV_CODEC_ID_IVTV_VBI: Self = Self(94234);
    pub const AV_CODEC_ID_FIRST_UNKNOWN: Self = Self(98304);
    pub const AV_CODEC_ID_TTF: Self = Self(98304);
    pub const AV_CODEC_ID_SCTE_35: Self = Self(98305);
    pub const AV_CODEC_ID_EPG: Self = Self(98306);
    pub const AV_CODEC_ID_BINTEXT: Self = Self(98307);
    pub const AV_CODEC_ID_XBIN: Self = Self(98308);
    pub const AV_CODEC_ID_IDF: Self = Self(98309);
    pub const AV_CODEC_ID_OTF: Self = Self(98310);
    pub const AV_CODEC_ID_SMPTE_KLV: Self = Self(98311);
    pub const AV_CODEC_ID_DVD_NAV: Self = Self(98312);
    pub const AV_CODEC_ID_TIMED_ID3: Self = Self(98313);
    pub const AV_CODEC_ID_BIN_DATA: Self = Self(98314);
    pub const AV_CODEC_ID_SMPTE_2038: Self = Self(98315);
    pub const AV_CODEC_ID_LCEVC: Self = Self(98316);
    pub const AV_CODEC_ID_SMPTE_436M_ANC: Self = Self(98317);
    pub const AV_CODEC_ID_PROBE: Self = Self(102400);
    pub const AV_CODEC_ID_MPEG2TS: Self = Self(131072);
    pub const AV_CODEC_ID_MPEG4SYSTEMS: Self = Self(131073);
    pub const AV_CODEC_ID_FFMETADATA: Self = Self(135168);
    pub const AV_CODEC_ID_WRAPPED_AVFRAME: Self = Self(135169);
    pub const AV_CODEC_ID_VNULL: Self = Self(135170);
    pub const AV_CODEC_ID_ANULL: Self = Self(135171);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVFieldOrder(pub ::core::ffi::c_uint);
impl AVFieldOrder {
    pub const AV_FIELD_UNKNOWN: Self = Self(0);
    pub const AV_FIELD_PROGRESSIVE: Self = Self(1);
    pub const AV_FIELD_TT: Self = Self(2);
    pub const AV_FIELD_BB: Self = Self(3);
    pub const AV_FIELD_TB: Self = Self(4);
    pub const AV_FIELD_BT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVDiscard(pub ::core::ffi::c_int);
impl AVDiscard {
    pub const AVDISCARD_NONE: Self = Self(-16);
    pub const AVDISCARD_DEFAULT: Self = Self(0);
    pub const AVDISCARD_NONREF: Self = Self(8);
    pub const AVDISCARD_BIDIR: Self = Self(16);
    pub const AVDISCARD_NONINTRA: Self = Self(24);
    pub const AVDISCARD_NONKEY: Self = Self(32);
    pub const AVDISCARD_ALL: Self = Self(48);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVPacketSideDataType(pub ::core::ffi::c_uint);
impl AVPacketSideDataType {
    pub const AV_PKT_DATA_PALETTE: Self = Self(0);
    pub const AV_PKT_DATA_NEW_EXTRADATA: Self = Self(1);
    pub const AV_PKT_DATA_PARAM_CHANGE: Self = Self(2);
    pub const AV_PKT_DATA_H263_MB_INFO: Self = Self(3);
    pub const AV_PKT_DATA_REPLAYGAIN: Self = Self(4);
    pub const AV_PKT_DATA_DISPLAYMATRIX: Self = Self(5);
    pub const AV_PKT_DATA_STEREO3D: Self = Self(6);
    pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: Self = Self(7);
    pub const AV_PKT_DATA_QUALITY_STATS: Self = Self(8);
    pub const AV_PKT_DATA_FALLBACK_TRACK: Self = Self(9);
    pub const AV_PKT_DATA_CPB_PROPERTIES: Self = Self(10);
    pub const AV_PKT_DATA_SKIP_SAMPLES: Self = Self(11);
    pub const AV_PKT_DATA_JP_DUALMONO: Self = Self(12);
    pub const AV_PKT_DATA_STRINGS_METADATA: Self = Self(13);
    pub const AV_PKT_DATA_SUBTITLE_POSITION: Self = Self(14);
    pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: Self = Self(15);
    pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: Self = Self(16);
    pub const AV_PKT_DATA_WEBVTT_SETTINGS: Self = Self(17);
    pub const AV_PKT_DATA_METADATA_UPDATE: Self = Self(18);
    pub const AV_PKT_DATA_MPEGTS_STREAM_ID: Self = Self(19);
    pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: Self = Self(20);
    pub const AV_PKT_DATA_SPHERICAL: Self = Self(21);
    pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: Self = Self(22);
    pub const AV_PKT_DATA_A53_CC: Self = Self(23);
    pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: Self = Self(24);
    pub const AV_PKT_DATA_ENCRYPTION_INFO: Self = Self(25);
    pub const AV_PKT_DATA_AFD: Self = Self(26);
    pub const AV_PKT_DATA_PRFT: Self = Self(27);
    pub const AV_PKT_DATA_ICC_PROFILE: Self = Self(28);
    pub const AV_PKT_DATA_DOVI_CONF: Self = Self(29);
    pub const AV_PKT_DATA_S12M_TIMECODE: Self = Self(30);
    pub const AV_PKT_DATA_DYNAMIC_HDR10_PLUS: Self = Self(31);
    pub const AV_PKT_DATA_IAMF_MIX_GAIN_PARAM: Self = Self(32);
    pub const AV_PKT_DATA_IAMF_DEMIXING_INFO_PARAM: Self = Self(33);
    pub const AV_PKT_DATA_IAMF_RECON_GAIN_INFO_PARAM: Self = Self(34);
    pub const AV_PKT_DATA_AMBIENT_VIEWING_ENVIRONMENT: Self = Self(35);
    pub const AV_PKT_DATA_FRAME_CROPPING: Self = Self(36);
    pub const AV_PKT_DATA_LCEVC: Self = Self(37);
    pub const AV_PKT_DATA_3D_REFERENCE_DISPLAYS: Self = Self(38);
    pub const AV_PKT_DATA_RTCP_SR: Self = Self(39);
    pub const AV_PKT_DATA_EXIF: Self = Self(40);
    pub const AV_PKT_DATA_DYNAMIC_HDR_SMPTE_2094_APP5: Self = Self(41);
    pub const AV_PKT_DATA_HEVC_CONF: Self = Self(42);
    pub const AV_PKT_DATA_NB: Self = Self(43);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub r#type: AVPacketSideDataType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: ::core::ffi::c_int,
    pub stream_index: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: ::core::ffi::c_int,
    pub duration: int64_t,
    pub pos: int64_t,
    pub opaque: *mut ::core::ffi::c_void,
    pub opaque_ref: *mut AVBufferRef,
    pub time_base: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParameters {
    pub codec_type: AVMediaType,
    pub codec_id: AVCodecID,
    pub codec_tag: uint32_t,
    pub extradata: *mut uint8_t,
    pub extradata_size: ::core::ffi::c_int,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub bit_rate: int64_t,
    pub bits_per_coded_sample: ::core::ffi::c_int,
    pub bits_per_raw_sample: ::core::ffi::c_int,
    pub profile: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub sample_aspect_ratio: AVRational,
    pub framerate: AVRational,
    pub field_order: AVFieldOrder,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub video_delay: ::core::ffi::c_int,
    pub ch_layout: AVChannelLayout,
    pub sample_rate: ::core::ffi::c_int,
    pub block_align: ::core::ffi::c_int,
    pub frame_size: ::core::ffi::c_int,
    pub initial_padding: ::core::ffi::c_int,
    pub trailing_padding: ::core::ffi::c_int,
    pub seek_preroll: ::core::ffi::c_int,
    pub alpha_mode: AVAlphaMode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOInterruptCB {
    pub callback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVIODataMarkerType(pub ::core::ffi::c_uint);
impl AVIODataMarkerType {
    pub const AVIO_DATA_MARKER_HEADER: Self = Self(0);
    pub const AVIO_DATA_MARKER_SYNC_POINT: Self = Self(1);
    pub const AVIO_DATA_MARKER_BOUNDARY_POINT: Self = Self(2);
    pub const AVIO_DATA_MARKER_UNKNOWN: Self = Self(3);
    pub const AVIO_DATA_MARKER_TRAILER: Self = Self(4);
    pub const AVIO_DATA_MARKER_FLUSH_POINT: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOContext {
    pub av_class: *const AVClass,
    pub buffer: *mut ::core::ffi::c_uchar,
    pub buffer_size: ::core::ffi::c_int,
    pub buf_ptr: *mut ::core::ffi::c_uchar,
    pub buf_end: *mut ::core::ffi::c_uchar,
    pub opaque: *mut ::core::ffi::c_void,
    pub read_packet: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut uint8_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub write_packet: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const uint8_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub seek: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, int64_t, ::core::ffi::c_int) -> int64_t,
    >,
    pub pos: int64_t,
    pub eof_reached: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
    pub write_flag: ::core::ffi::c_int,
    pub max_packet_size: ::core::ffi::c_int,
    pub min_packet_size: ::core::ffi::c_int,
    pub checksum: ::core::ffi::c_ulong,
    pub checksum_ptr: *mut ::core::ffi::c_uchar,
    pub update_checksum: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_ulong,
            *const uint8_t,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_ulong,
    >,
    pub read_pause: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub read_seek: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            int64_t,
            ::core::ffi::c_int,
        ) -> int64_t,
    >,
    pub seekable: ::core::ffi::c_int,
    pub direct: ::core::ffi::c_int,
    pub protocol_whitelist: *const ::core::ffi::c_char,
    pub protocol_blacklist: *const ::core::ffi::c_char,
    pub write_data_type: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const uint8_t,
            ::core::ffi::c_int,
            AVIODataMarkerType,
            int64_t,
        ) -> ::core::ffi::c_int,
    >,
    pub ignore_boundary_point: ::core::ffi::c_int,
    pub buf_ptr_max: *mut ::core::ffi::c_uchar,
    pub bytes_read: int64_t,
    pub bytes_written: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProfile {
    pub profile: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodec {
    pub name: *const ::core::ffi::c_char,
    pub long_name: *const ::core::ffi::c_char,
    pub r#type: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: ::core::ffi::c_int,
    pub max_lowres: uint8_t,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFormatContext {
    pub av_class: *const AVClass,
    pub iformat: *const AVInputFormat,
    pub oformat: *const AVOutputFormat,
    pub priv_data: *mut ::core::ffi::c_void,
    pub pb: *mut AVIOContext,
    pub ctx_flags: ::core::ffi::c_int,
    pub nb_streams: ::core::ffi::c_uint,
    pub streams: *mut *mut AVStream,
    pub nb_stream_groups: ::core::ffi::c_uint,
    pub stream_groups: *mut *mut AVStreamGroup,
    pub nb_chapters: ::core::ffi::c_uint,
    pub chapters: *mut *mut AVChapter,
    pub url: *mut ::core::ffi::c_char,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub bit_rate: int64_t,
    pub packet_size: ::core::ffi::c_uint,
    pub max_delay: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub probesize: int64_t,
    pub max_analyze_duration: int64_t,
    pub key: *const uint8_t,
    pub keylen: ::core::ffi::c_int,
    pub nb_programs: ::core::ffi::c_uint,
    pub programs: *mut *mut AVProgram,
    pub video_codec_id: AVCodecID,
    pub audio_codec_id: AVCodecID,
    pub subtitle_codec_id: AVCodecID,
    pub data_codec_id: AVCodecID,
    pub metadata: *mut AVDictionary,
    pub start_time_realtime: int64_t,
    pub fps_probe_size: ::core::ffi::c_int,
    pub error_recognition: ::core::ffi::c_int,
    pub interrupt_callback: AVIOInterruptCB,
    pub debug: ::core::ffi::c_int,
    pub max_streams: ::core::ffi::c_int,
    pub max_index_size: ::core::ffi::c_uint,
    pub max_picture_buffer: ::core::ffi::c_uint,
    pub max_interleave_delta: int64_t,
    pub max_ts_probe: ::core::ffi::c_int,
    pub max_chunk_duration: ::core::ffi::c_int,
    pub max_chunk_size: ::core::ffi::c_int,
    pub max_probe_packets: ::core::ffi::c_int,
    pub strict_std_compliance: ::core::ffi::c_int,
    pub event_flags: ::core::ffi::c_int,
    pub avoid_negative_ts: ::core::ffi::c_int,
    pub audio_preload: ::core::ffi::c_int,
    pub use_wallclock_as_timestamps: ::core::ffi::c_int,
    pub skip_estimate_duration_from_pts: ::core::ffi::c_int,
    pub avio_flags: ::core::ffi::c_int,
    pub duration_estimation_method: AVDurationEstimationMethod,
    pub skip_initial_bytes: int64_t,
    pub correct_ts_overflow: ::core::ffi::c_uint,
    pub seek2any: ::core::ffi::c_int,
    pub flush_packets: ::core::ffi::c_int,
    pub probe_score: ::core::ffi::c_int,
    pub format_probesize: ::core::ffi::c_int,
    pub codec_whitelist: *mut ::core::ffi::c_char,
    pub format_whitelist: *mut ::core::ffi::c_char,
    pub protocol_whitelist: *mut ::core::ffi::c_char,
    pub protocol_blacklist: *mut ::core::ffi::c_char,
    pub io_repositioned: ::core::ffi::c_int,
    pub video_codec: *const AVCodec,
    pub audio_codec: *const AVCodec,
    pub subtitle_codec: *const AVCodec,
    pub data_codec: *const AVCodec,
    pub metadata_header_padding: ::core::ffi::c_int,
    pub opaque: *mut ::core::ffi::c_void,
    pub control_message_cb: av_format_control_message,
    pub output_ts_offset: int64_t,
    pub dump_separator: *mut uint8_t,
    pub io_open: Option<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut *mut AVIOContext,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut AVDictionary,
        ) -> ::core::ffi::c_int,
    >,
    pub io_close2:
        Option<unsafe extern "C" fn(*mut AVFormatContext, *mut AVIOContext) -> ::core::ffi::c_int>,
    pub duration_probesize: int64_t,
    pub name: *mut ::core::ffi::c_char,
}
pub type av_format_control_message = Option<
    unsafe extern "C" fn(
        *mut AVFormatContext,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
        size_t,
    ) -> ::core::ffi::c_int,
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVDurationEstimationMethod(pub ::core::ffi::c_uint);
impl AVDurationEstimationMethod {
    pub const AVFMT_DURATION_FROM_PTS: Self = Self(0);
    pub const AVFMT_DURATION_FROM_STREAM: Self = Self(1);
    pub const AVFMT_DURATION_FROM_BITRATE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProgram {
    pub id: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub discard: AVDiscard,
    pub stream_index: *mut ::core::ffi::c_uint,
    pub nb_stream_indexes: ::core::ffi::c_uint,
    pub metadata: *mut AVDictionary,
    pub program_num: ::core::ffi::c_int,
    pub pmt_pid: ::core::ffi::c_int,
    pub pcr_pid: ::core::ffi::c_int,
    pub pmt_version: ::core::ffi::c_int,
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChapter {
    pub id: int64_t,
    pub time_base: AVRational,
    pub start: int64_t,
    pub end: int64_t,
    pub metadata: *mut AVDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStreamGroup {
    pub av_class: *const AVClass,
    pub priv_data: *mut ::core::ffi::c_void,
    pub index: ::core::ffi::c_uint,
    pub id: int64_t,
    pub r#type: AVStreamGroupParamsType,
    pub params: C2Rust_Unnamed_1,
    pub metadata: *mut AVDictionary,
    pub nb_streams: ::core::ffi::c_uint,
    pub streams: *mut *mut AVStream,
    pub disposition: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStream {
    pub av_class: *const AVClass,
    pub index: ::core::ffi::c_int,
    pub id: ::core::ffi::c_int,
    pub codecpar: *mut AVCodecParameters,
    pub priv_data: *mut ::core::ffi::c_void,
    pub time_base: AVRational,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub nb_frames: int64_t,
    pub disposition: ::core::ffi::c_int,
    pub discard: AVDiscard,
    pub sample_aspect_ratio: AVRational,
    pub metadata: *mut AVDictionary,
    pub avg_frame_rate: AVRational,
    pub attached_pic: AVPacket,
    pub event_flags: ::core::ffi::c_int,
    pub r_frame_rate: AVRational,
    pub pts_wrap_bits: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub iamf_audio_element: *mut AVIAMFAudioElement,
    pub iamf_mix_presentation: *mut AVIAMFMixPresentation,
    pub tile_grid: *mut AVStreamGroupTileGrid,
    pub layered_video: *mut AVStreamGroupLayeredVideo,
    pub lcevc: *mut AVStreamGroupLayeredVideo,
    pub tref: *mut AVStreamGroupTREF,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStreamGroupTREF {
    pub av_class: *const AVClass,
    pub metadata_index: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStreamGroupLayeredVideo {
    pub av_class: *const AVClass,
    pub c2rust_unnamed: C2Rust_Unnamed_2,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub el_index: ::core::ffi::c_uint,
    pub lcevc_index: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStreamGroupTileGrid {
    pub av_class: *const AVClass,
    pub nb_tiles: ::core::ffi::c_uint,
    pub coded_width: ::core::ffi::c_int,
    pub coded_height: ::core::ffi::c_int,
    pub offsets: *mut C2Rust_Unnamed_3,
    pub background: [uint8_t; 4],
    pub horizontal_offset: ::core::ffi::c_int,
    pub vertical_offset: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub idx: ::core::ffi::c_uint,
    pub horizontal: ::core::ffi::c_int,
    pub vertical: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVStreamGroupParamsType(pub ::core::ffi::c_uint);
impl AVStreamGroupParamsType {
    pub const AV_STREAM_GROUP_PARAMS_NONE: Self = Self(0);
    pub const AV_STREAM_GROUP_PARAMS_IAMF_AUDIO_ELEMENT: Self = Self(1);
    pub const AV_STREAM_GROUP_PARAMS_IAMF_MIX_PRESENTATION: Self = Self(2);
    pub const AV_STREAM_GROUP_PARAMS_TILE_GRID: Self = Self(3);
    pub const AV_STREAM_GROUP_PARAMS_LCEVC: Self = Self(4);
    pub const AV_STREAM_GROUP_PARAMS_TREF: Self = Self(5);
    pub const AV_STREAM_GROUP_PARAMS_DOLBY_VISION: Self = Self(6);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOutputFormat {
    pub name: *const ::core::ffi::c_char,
    pub long_name: *const ::core::ffi::c_char,
    pub mime_type: *const ::core::ffi::c_char,
    pub extensions: *const ::core::ffi::c_char,
    pub audio_codec: AVCodecID,
    pub video_codec: AVCodecID,
    pub subtitle_codec: AVCodecID,
    pub flags: ::core::ffi::c_int,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVInputFormat {
    pub name: *const ::core::ffi::c_char,
    pub long_name: *const ::core::ffi::c_char,
    pub flags: ::core::ffi::c_int,
    pub extensions: *const ::core::ffi::c_char,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub mime_type: *const ::core::ffi::c_char,
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
pub struct AVBSFContext {
    pub av_class: *const AVClass,
    pub filter: *const AVBitStreamFilter,
    pub priv_data: *mut ::core::ffi::c_void,
    pub par_in: *mut AVCodecParameters,
    pub par_out: *mut AVCodecParameters,
    pub time_base_in: AVRational,
    pub time_base_out: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBitStreamFilter {
    pub name: *const ::core::ffi::c_char,
    pub codec_ids: *const AVCodecID,
    pub priv_class: *const AVClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_dts_info_t {
    pub first_dts: int64_t,
    pub last_dts: int64_t,
    pub time_base: AVRational,
    pub initialized: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hls_writer_t {
    pub output_dir: [::core::ffi::c_char; 512],
    pub stream_name: [::core::ffi::c_char; 256],
    pub segment_duration: ::core::ffi::c_int,
    pub output_ctx: *mut AVFormatContext,
    pub initialized: ::core::ffi::c_int,
    pub last_cleanup_time: time_t,
    pub dts_tracker: stream_dts_info_t,
    pub audio_dts_tracker: stream_dts_info_t,
    pub video_out_index: ::core::ffi::c_int,
    pub audio_out_index: ::core::ffi::c_int,
    pub dts_jump_count: ::core::ffi::c_int,
    pub bsf_ctx: *mut AVBSFContext,
    pub thread_ctx: *mut ::core::ffi::c_void,
    pub mutex: pthread_mutex_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct recording_io_operation_t(pub ::core::ffi::c_uint);
impl recording_io_operation_t {
    pub const RECORDING_IO_OPERATION_ALLOCATE: Self = Self(0);
    pub const RECORDING_IO_OPERATION_OPEN: Self = Self(1);
    pub const RECORDING_IO_OPERATION_HEADER: Self = Self(2);
    pub const RECORDING_IO_OPERATION_PACKET: Self = Self(3);
    pub const RECORDING_IO_OPERATION_TRAILER: Self = Self(4);
    pub const RECORDING_IO_OPERATION_CLOSE: Self = Self(5);
    pub const RECORDING_IO_OPERATION_FILESYSTEM: Self = Self(6);
    pub const RECORDING_IO_OPERATION_COUNT: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct recording_io_resource_t(pub ::core::ffi::c_uint);
impl recording_io_resource_t {
    pub const RECORDING_IO_RESOURCE_RECORDING: Self = Self(0);
    pub const RECORDING_IO_RESOURCE_HLS: Self = Self(1);
    pub const RECORDING_IO_RESOURCE_COUNT: Self = Self(2);
}
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AV_NOPTS_VALUE: int64_t = 0x8000000000000000 as ::core::ffi::c_ulong as int64_t;
pub const AV_TIME_BASE: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
pub const AV_ERROR_MAX_STRING_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const AV_PKT_FLAG_KEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const AVIO_FLAG_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hls_writer_create(
    mut output_dir: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut segment_duration: ::core::ffi::c_int,
) -> *mut hls_writer_t {
    let mut existing_writer: *mut hls_writer_t = find_hls_writer_by_stream_name(stream_name);
    if !existing_writer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"HLS writer for stream %s already exists. Closing existing writer to prevent duplicates.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        hls_writer_close(existing_writer);
        usleep(100000 as ::core::ffi::c_int as __useconds_t);
    }
    let mut writer: *mut hls_writer_t =
        calloc(1 as size_t, ::core::mem::size_of::<hls_writer_t>()) as *mut hls_writer_t;
    if writer.is_null() {
        recording_io_report_failure(
            recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
            recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
            ENOMEM,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate HLS writer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<hls_writer_t>();
    }
    safe_strcpy(
        &raw mut (*writer).output_dir as *mut ::core::ffi::c_char,
        output_dir,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        stream_name,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    (*writer).video_out_index = -1 as ::core::ffi::c_int;
    (*writer).audio_out_index = -1 as ::core::ffi::c_int;
    if segment_duration < 1 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"HLS segment duration too low (%d), setting to 1 second minimum\0".as_ptr()
                as *const ::core::ffi::c_char,
            segment_duration,
        );
        segment_duration = 1 as ::core::ffi::c_int;
    } else if segment_duration > 10 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"HLS segment duration too high (%d), capping at 10 seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            segment_duration,
        );
        segment_duration = 10 as ::core::ffi::c_int;
    }
    (*writer).segment_duration = segment_duration;
    (*writer).last_cleanup_time = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_init(
        &raw mut (*writer).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    );
    if ensure_hls_directory(
        &raw mut (*writer).output_dir as *mut ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
    ) != 0
    {
        let mut directory_error: ::core::ffi::c_int = *__errno_location();
        recording_io_report_failure(
            recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
            recording_io_operation_t::RECORDING_IO_OPERATION_FILESYSTEM,
            directory_error,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to ensure HLS output directory exists: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).output_dir as *mut ::core::ffi::c_char,
        );
        pthread_mutex_destroy(&raw mut (*writer).mutex);
        free(writer as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<hls_writer_t>();
    }
    let mut output_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut output_path as *mut ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        b"%s/index.m3u8\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*writer).output_dir as *mut ::core::ffi::c_char,
    );
    let mut ret: ::core::ffi::c_int = avformat_alloc_output_context2(
        &raw mut (*writer).output_ctx,
        ::core::ptr::null::<AVOutputFormat>(),
        b"hls\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut output_path as *mut ::core::ffi::c_char,
    );
    if ret < 0 as ::core::ffi::c_int {
        recording_io_report_failure(
            recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
            recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
            ret,
        );
        let mut error_buf: [::core::ffi::c_char; 64] = [
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
        av_strerror(
            ret,
            &raw mut error_buf as *mut ::core::ffi::c_char,
            AV_ERROR_MAX_STRING_SIZE as size_t,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate output context for HLS: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut error_buf as *mut ::core::ffi::c_char,
        );
        pthread_mutex_destroy(&raw mut (*writer).mutex);
        free(writer as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<hls_writer_t>();
    }
    let mut options: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
    let mut hls_time: [::core::ffi::c_char; 16] = [0; 16];
    snprintf(
        &raw mut hls_time as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        segment_duration,
    );
    av_dict_set(
        &raw mut options,
        b"hls_time\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut hls_time as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut options,
        b"hls_list_size\0".as_ptr() as *const ::core::ffi::c_char,
        b"6\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut options,
        b"hls_segment_type\0".as_ptr() as *const ::core::ffi::c_char,
        b"mpegts\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut options,
        b"hls_flags\0".as_ptr() as *const ::core::ffi::c_char,
        b"delete_segments+independent_segments+program_date_time+temp_file\0".as_ptr()
            as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    let mut force_key_frames: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut force_key_frames as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"expr:gte(t,n_forced*%d)\0".as_ptr() as *const ::core::ffi::c_char,
        segment_duration,
    );
    av_dict_set(
        &raw mut options,
        b"force_key_frames\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut force_key_frames as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut options,
        b"start_number\0".as_ptr() as *const ::core::ffi::c_char,
        b"0\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut options,
        b"flush_packets\0".as_ptr() as *const ::core::ffi::c_char,
        b"1\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut options,
        b"avoid_negative_ts\0".as_ptr() as *const ::core::ffi::c_char,
        b"make_non_negative\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    let mut segment_format: [::core::ffi::c_char; 544] = [0; 544];
    snprintf(
        &raw mut segment_format as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 544]>(),
        b"%s/segment_%%d.ts\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*writer).output_dir as *mut ::core::ffi::c_char,
    );
    av_dict_set(
        &raw mut options,
        b"hls_segment_filename\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut segment_format as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"HLS writer options for stream %s (optimized for stability and compatibility):\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"  hls_time: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut hls_time as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"  hls_list_size: 6\0".as_ptr() as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"  hls_flags: delete_segments+independent_segments+program_date_time+temp_file\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"  hls_segment_type: mpegts\0".as_ptr() as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"  force_key_frames: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut force_key_frames as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"  start_number: 0\0".as_ptr() as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"  hls_segment_filename: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut segment_format as *mut ::core::ffi::c_char,
    );
    ret = avio_open2(
        &raw mut (*(*writer).output_ctx).pb,
        &raw mut output_path as *mut ::core::ffi::c_char,
        AVIO_FLAG_WRITE,
        ::core::ptr::null::<AVIOInterruptCB>(),
        &raw mut options,
    );
    if ret < 0 as ::core::ffi::c_int {
        recording_io_report_failure(
            recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
            recording_io_operation_t::RECORDING_IO_OPERATION_OPEN,
            ret,
        );
        let mut error_buf_0: [::core::ffi::c_char; 64] = [
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
        av_strerror(
            ret,
            &raw mut error_buf_0 as *mut ::core::ffi::c_char,
            AV_ERROR_MAX_STRING_SIZE as size_t,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open HLS output file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut error_buf_0 as *mut ::core::ffi::c_char,
        );
        av_dict_free(&raw mut options);
        avformat_free_context((*writer).output_ctx);
        pthread_mutex_destroy(&raw mut (*writer).mutex);
        free(writer as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<hls_writer_t>();
    }
    av_dict_free(&raw mut options);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Created HLS writer for stream %s at %s with segment duration %d seconds\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
        output_dir,
        segment_duration,
    );
    register_hls_writer(writer);
    return writer;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hls_writer_audio_supported(
    mut par: *const AVCodecParameters,
) -> ::core::ffi::c_int {
    return (!par.is_null()
        && (*par).codec_type.0 == AVMediaType::AVMEDIA_TYPE_AUDIO.0
        && (*par).codec_id.0 == AVCodecID::AV_CODEC_ID_AAC.0
        && (*par).extradata_size > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hls_writer_initialize(
    mut writer: *mut hls_writer_t,
    mut input_stream: *const AVStream,
) -> ::core::ffi::c_int {
    return hls_writer_initialize_av(writer, input_stream, ::core::ptr::null::<AVStream>());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hls_writer_initialize_av(
    mut writer: *mut hls_writer_t,
    mut input_stream: *const AVStream,
    mut audio_stream: *const AVStream,
) -> ::core::ffi::c_int {
    if writer.is_null() || input_stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for hls_writer_initialize\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*writer).output_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Output context is NULL in hls_writer_initialize\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*writer).initialized != 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut out_stream: *mut AVStream =
        avformat_new_stream((*writer).output_ctx, ::core::ptr::null::<AVCodec>());
    if out_stream.is_null() {
        recording_io_report_failure(
            recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
            recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
            ENOMEM,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create output stream for HLS\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut ret: ::core::ffi::c_int =
        avcodec_parameters_copy((*out_stream).codecpar, (*input_stream).codecpar);
    if ret < 0 as ::core::ffi::c_int {
        let mut error_buf: [::core::ffi::c_char; 64] = [
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
        av_strerror(
            ret,
            &raw mut error_buf as *mut ::core::ffi::c_char,
            AV_ERROR_MAX_STRING_SIZE as size_t,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to copy codec parameters: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut error_buf as *mut ::core::ffi::c_char,
        );
        return ret;
    }
    (*out_stream).time_base = (*input_stream).time_base;
    (*writer).video_out_index = (*out_stream).index;
    if (*(*input_stream).codecpar).codec_id.0 == AVCodecID::AV_CODEC_ID_H264.0 {
        (*(*out_stream).codecpar).codec_tag = 0 as uint32_t;
        if !(*writer).output_ctx.is_null() {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while (i as ::core::ffi::c_uint) < (*(*writer).output_ctx).nb_streams {
                let mut stream: *mut AVStream = *(*(*writer).output_ctx).streams.offset(i as isize);
                if !stream.is_null() {
                    (*(*stream).codecpar).codec_tag = 0 as uint32_t;
                }
                i += 1;
            }
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Set correct codec parameters for H.264 in HLS for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s is not H.264, using default codec parameters\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
    }
    (*writer).audio_out_index = -1 as ::core::ffi::c_int;
    if !audio_stream.is_null() && hls_writer_audio_supported((*audio_stream).codecpar) != 0 {
        let mut audio_out: *mut AVStream =
            avformat_new_stream((*writer).output_ctx, ::core::ptr::null::<AVCodec>());
        if audio_out.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to create HLS audio output stream for %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        ret = avcodec_parameters_copy((*audio_out).codecpar, (*audio_stream).codecpar);
        if ret < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to copy audio codec parameters for HLS stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            );
            return ret;
        }
        (*(*audio_out).codecpar).codec_tag = 0 as uint32_t;
        (*audio_out).time_base = (*audio_stream).time_base;
        (*writer).audio_out_index = (*audio_out).index;
        (*(*writer).output_ctx).max_interleave_delta = AV_TIME_BASE as int64_t;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"HLS stream %s: muxing AAC audio (%d Hz, %d ch) as output stream %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            (*(*audio_stream).codecpar).sample_rate,
            (*(*audio_stream).codecpar).ch_layout.nb_channels,
            (*writer).audio_out_index,
        );
    } else if !audio_stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"HLS stream %s: skipping unsupported audio codec %s (only AAC with codec config is passed through)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            avcodec_get_name((*(*audio_stream).codecpar).codec_id),
        );
    }
    let mut options: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
    ret = avformat_write_header((*writer).output_ctx, &raw mut options);
    if ret < 0 as ::core::ffi::c_int {
        recording_io_report_failure(
            recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
            recording_io_operation_t::RECORDING_IO_OPERATION_HEADER,
            ret,
        );
        let mut error_buf_0: [::core::ffi::c_char; 64] = [
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
        av_strerror(
            ret,
            &raw mut error_buf_0 as *mut ::core::ffi::c_char,
            AV_ERROR_MAX_STRING_SIZE as size_t,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to write HLS header: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut error_buf_0 as *mut ::core::ffi::c_char,
        );
        av_dict_free(&raw mut options);
        return ret;
    }
    av_dict_free(&raw mut options);
    (*writer).dts_tracker.initialized = 0 as ::core::ffi::c_int;
    (*writer).dts_tracker.first_dts = 0 as int64_t;
    (*writer).dts_tracker.last_dts = 0 as int64_t;
    (*writer).dts_tracker.time_base = (*out_stream).time_base;
    (*writer).audio_dts_tracker.initialized = 0 as ::core::ffi::c_int;
    (*writer).audio_dts_tracker.first_dts = 0 as int64_t;
    (*writer).audio_dts_tracker.last_dts = 0 as int64_t;
    if (*writer).audio_out_index >= 0 as ::core::ffi::c_int {
        (*writer).audio_dts_tracker.time_base = (**(*(*writer).output_ctx)
            .streams
            .offset((*writer).audio_out_index as isize))
        .time_base;
    }
    (*writer).dts_jump_count = 0 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Initialized HLS writer for stream %s with DTS tracking\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
    );
    (*writer).initialized = 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hls_writer_write_packet(
    mut writer: *mut hls_writer_t,
    mut pkt: *const AVPacket,
    mut input_stream: *const AVStream,
) -> ::core::ffi::c_int {
    if writer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"hls_writer_write_packet: NULL writer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if pkt.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"hls_writer_write_packet: NULL packet for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if input_stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"hls_writer_write_packet: NULL input stream for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*writer).output_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"hls_writer_write_packet: Writer for stream %s has been closed\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if now - (*writer).last_cleanup_time >= 10 as time_t {
        if ensure_hls_directory(
            &raw mut (*writer).output_dir as *mut ::core::ffi::c_char,
            MAX_PATH_LENGTH as size_t,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        ) != 0
        {
            let mut directory_error: ::core::ffi::c_int = *__errno_location();
            recording_io_report_failure(
                recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
                recording_io_operation_t::RECORDING_IO_OPERATION_FILESYSTEM,
                directory_error,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to ensure HLS output directory exists: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*writer).output_dir as *mut ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*writer).last_cleanup_time = now;
    }
    let is_audio: ::core::ffi::c_int = (!(*input_stream).codecpar.is_null()
        && (*(*input_stream).codecpar).codec_type.0 == AVMediaType::AVMEDIA_TYPE_AUDIO.0)
        as ::core::ffi::c_int;
    if (*writer).initialized == 0 {
        if is_audio != 0 {
            return 0 as ::core::ffi::c_int;
        }
        let mut ret: ::core::ffi::c_int = hls_writer_initialize(writer, input_stream);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    if is_audio != 0 && (*writer).audio_out_index < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*writer).output_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"hls_writer_write_packet: Writer for stream %s has been closed after initialization\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut out_pkt_ptr: *mut AVPacket = av_packet_alloc();
    if out_pkt_ptr.is_null() {
        recording_io_report_failure(
            recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
            recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
            ENOMEM,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate packet for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*pkt).data.is_null() || (*pkt).size <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid packet data for stream %s (data=%p, size=%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            (*pkt).data,
            (*pkt).size,
        );
        av_packet_free(&raw mut out_pkt_ptr);
        return -1 as ::core::ffi::c_int;
    }
    let mut packet_ref_ret: ::core::ffi::c_int = av_packet_ref(out_pkt_ptr, pkt);
    if packet_ref_ret < 0 as ::core::ffi::c_int {
        if packet_ref_ret == -12 as ::core::ffi::c_int {
            recording_io_report_failure(
                recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
                recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                packet_ref_ret,
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to reference packet for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
        av_packet_free(&raw mut out_pkt_ptr);
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if (*(*input_stream).codecpar).codec_id.0 == AVCodecID::AV_CODEC_ID_H264.0 {
        let mut has_start_code: bool = r#false != 0;
        if (*out_pkt_ptr).size >= 4 as ::core::ffi::c_int {
            has_start_code = *(*out_pkt_ptr).data.offset(0isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && *(*out_pkt_ptr).data.offset(1isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && *(*out_pkt_ptr).data.offset(2isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                && *(*out_pkt_ptr).data.offset(3isize) as ::core::ffi::c_int
                    == 0x1 as ::core::ffi::c_int;
        }
        if !has_start_code {
            let mut new_pkt_ptr: *mut AVPacket = av_packet_alloc();
            if new_pkt_ptr.is_null() {
                recording_io_report_failure(
                    recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
                    recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                    ENOMEM,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to allocate new packet for H.264 conversion for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
                );
                av_packet_free(&raw mut out_pkt_ptr);
                return -1 as ::core::ffi::c_int;
            }
            let mut packet_alloc_ret: ::core::ffi::c_int =
                av_new_packet(new_pkt_ptr, (*out_pkt_ptr).size + 4 as ::core::ffi::c_int);
            if packet_alloc_ret < 0 as ::core::ffi::c_int {
                if packet_alloc_ret == -12 as ::core::ffi::c_int {
                    recording_io_report_failure(
                        recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
                        recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                        packet_alloc_ret,
                    );
                }
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to allocate new packet for H.264 conversion for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
                );
                av_packet_free(&raw mut new_pkt_ptr);
                av_packet_free(&raw mut out_pkt_ptr);
                return -1 as ::core::ffi::c_int;
            }
            *(*new_pkt_ptr).data.offset(0isize) = 0 as uint8_t;
            *(*new_pkt_ptr).data.offset(1isize) = 0 as uint8_t;
            *(*new_pkt_ptr).data.offset(2isize) = 0 as uint8_t;
            *(*new_pkt_ptr).data.offset(3isize) = 0x1 as uint8_t;
            memcpy(
                (*new_pkt_ptr).data.offset(4 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                (*out_pkt_ptr).data as *const ::core::ffi::c_void,
                (*out_pkt_ptr).size as size_t,
            );
            (*new_pkt_ptr).pts = (*out_pkt_ptr).pts;
            (*new_pkt_ptr).dts = (*out_pkt_ptr).dts;
            (*new_pkt_ptr).flags = (*out_pkt_ptr).flags;
            (*new_pkt_ptr).stream_index = (*out_pkt_ptr).stream_index;
            (*new_pkt_ptr).duration = (*out_pkt_ptr).duration;
            (*new_pkt_ptr).pos = (*out_pkt_ptr).pos;
            av_packet_free(&raw mut out_pkt_ptr);
            out_pkt_ptr = new_pkt_ptr;
            new_pkt_ptr = ::core::ptr::null_mut::<AVPacket>();
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Added H.264 start code for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            );
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using original packet for non-H264 stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
    }
    if (*out_pkt_ptr).pts == AV_NOPTS_VALUE && (*out_pkt_ptr).dts == AV_NOPTS_VALUE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Both PTS and DTS are invalid for packet in stream %s, skipping packet\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
        av_packet_free(&raw mut out_pkt_ptr);
        return 0 as ::core::ffi::c_int;
    }
    if (*out_pkt_ptr).pts == AV_NOPTS_VALUE {
        (*out_pkt_ptr).pts = (*out_pkt_ptr).dts;
    } else if (*out_pkt_ptr).dts == AV_NOPTS_VALUE {
        (*out_pkt_ptr).dts = (*out_pkt_ptr).pts;
    }
    if (*out_pkt_ptr).pts < (*out_pkt_ptr).dts {
        (*out_pkt_ptr).pts = (*out_pkt_ptr).dts;
    }
    let mut out_index: ::core::ffi::c_int = if is_audio != 0 {
        (*writer).audio_out_index
    } else {
        (*writer).video_out_index
    };
    let mut dts_tracker: *mut stream_dts_info_t = if is_audio != 0 {
        &raw mut (*writer).audio_dts_tracker
    } else {
        &raw mut (*writer).dts_tracker
    };
    if (*writer).output_ctx.is_null()
        || (*(*writer).output_ctx).streams.is_null()
        || out_index < 0 as ::core::ffi::c_int
        || out_index >= (*(*writer).output_ctx).nb_streams as ::core::ffi::c_int
        || (*(*(*writer).output_ctx).streams.offset(out_index as isize)).is_null()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"hls_writer_write_packet: Writer context invalid for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
        av_packet_free(&raw mut out_pkt_ptr);
        return -1 as ::core::ffi::c_int;
    }
    let mut out_stream: *mut AVStream = *(*(*writer).output_ctx).streams.offset(out_index as isize);
    (*out_pkt_ptr).stream_index = (*out_stream).index;
    av_packet_rescale_ts(
        out_pkt_ptr,
        (*input_stream).time_base,
        (*out_stream).time_base,
    );
    if (*out_pkt_ptr).pts < (*out_pkt_ptr).dts {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Fixing HLS packet with PTS < DTS after rescaling: PTS=%lld, DTS=%lld\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*out_pkt_ptr).pts as ::core::ffi::c_longlong,
            (*out_pkt_ptr).dts as ::core::ffi::c_longlong,
        );
        (*out_pkt_ptr).pts = (*out_pkt_ptr).dts;
    }
    if (*out_pkt_ptr).pts != AV_NOPTS_VALUE && (*out_pkt_ptr).dts != AV_NOPTS_VALUE {
        let mut pts_dts_diff: int64_t = (*out_pkt_ptr).pts - (*out_pkt_ptr).dts;
        if pts_dts_diff > 90000 as ::core::ffi::c_int as int64_t * 10 as int64_t {
            (*out_pkt_ptr).pts =
                (*out_pkt_ptr).dts + 90000 as ::core::ffi::c_int as int64_t * 5 as int64_t;
        }
    }
    if (*dts_tracker).initialized == 0 {
        (*dts_tracker).first_dts = (*out_pkt_ptr).dts;
        (*dts_tracker).last_dts = (*out_pkt_ptr).dts;
        (*dts_tracker).time_base = (*out_stream).time_base;
        (*dts_tracker).initialized = 1 as ::core::ffi::c_int;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Initialized DTS tracker for stream %s: first_dts=%lld\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            (*out_pkt_ptr).dts as ::core::ffi::c_longlong,
        );
    } else {
        if (*out_pkt_ptr).dts <= (*dts_tracker).last_dts {
            let mut old_dts: int64_t = (*out_pkt_ptr).dts;
            (*out_pkt_ptr).dts = (*dts_tracker).last_dts + 1 as int64_t;
            if (*out_pkt_ptr).pts < (*out_pkt_ptr).dts {
                (*out_pkt_ptr).pts = (*out_pkt_ptr).dts;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Fixed non-monotonic DTS for stream %s: old_dts=%lld, new_dts=%lld, last_dts=%lld\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
                old_dts as ::core::ffi::c_longlong,
                (*out_pkt_ptr).dts as ::core::ffi::c_longlong,
                (*dts_tracker).last_dts as ::core::ffi::c_longlong,
            );
            (*writer).dts_jump_count += 1;
            if (*writer).dts_jump_count % 10 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stream %s has had %d DTS corrections - stream may have timestamp issues\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
                    (*writer).dts_jump_count,
                );
            }
        }
        (*dts_tracker).last_dts = (*out_pkt_ptr).dts;
    }
    let mut is_key_frame: bool =
        is_audio == 0 && (*out_pkt_ptr).flags & AV_PKT_FLAG_KEY != 0 as ::core::ffi::c_int;
    if is_key_frame {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Writing key frame to HLS for stream %s: pts=%lld, dts=%lld, size=%d\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            (*out_pkt_ptr).pts as ::core::ffi::c_longlong,
            (*out_pkt_ptr).dts as ::core::ffi::c_longlong,
            (*out_pkt_ptr).size,
        );
    }
    result = av_interleaved_write_frame((*writer).output_ctx, out_pkt_ptr);
    av_packet_free(&raw mut out_pkt_ptr);
    if result < 0 as ::core::ffi::c_int {
        recording_io_report_failure(
            recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
            recording_io_operation_t::RECORDING_IO_OPERATION_PACKET,
            result,
        );
        let mut error_buf: [::core::ffi::c_char; 64] = [
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
        av_strerror(
            result,
            &raw mut error_buf as *mut ::core::ffi::c_char,
            AV_ERROR_MAX_STRING_SIZE as size_t,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Error writing HLS packet for stream %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            &raw mut error_buf as *mut ::core::ffi::c_char,
        );
        if !strstr(
            &raw mut error_buf as *mut ::core::ffi::c_char,
            b"No such file or directory\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            ensure_hls_directory(
                &raw mut (*writer).output_dir as *mut ::core::ffi::c_char,
                MAX_PATH_LENGTH as size_t,
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            );
        } else if !strstr(
            &raw mut error_buf as *mut ::core::ffi::c_char,
            b"Invalid data found when processing input\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Ignoring invalid packet data for stream %s and continuing\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            );
            result = 0 as ::core::ffi::c_int;
        }
    } else {
        (*writer).last_cleanup_time = now;
    }
    return result;
}
pub const MAX_HLS_WRITERS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut g_hls_writers: [*mut hls_writer_t; 32] = [
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
    ::core::ptr::null_mut::<hls_writer_t>(),
];
static mut g_hls_writers_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut g_hls_writers_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_hls_writer_by_stream_name(
    mut stream_name: *const ::core::ffi::c_char,
) -> *mut hls_writer_t {
    if stream_name.is_null() {
        return ::core::ptr::null_mut::<hls_writer_t>();
    }
    pthread_mutex_lock(&raw mut g_hls_writers_mutex);
    let mut existing_writer: *mut hls_writer_t = ::core::ptr::null_mut::<hls_writer_t>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_HLS_WRITERS {
        if !g_hls_writers[i as usize].is_null()
            && !(&raw mut (**(&raw mut g_hls_writers as *mut *mut hls_writer_t).offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char)
                .is_null()
            && strcmp(
                &raw mut (**(&raw mut g_hls_writers as *mut *mut hls_writer_t).offset(i as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            existing_writer = g_hls_writers[i as usize];
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut g_hls_writers_mutex);
    return existing_writer;
}
unsafe extern "C" fn register_hls_writer(mut writer: *mut hls_writer_t) {
    if writer.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut g_hls_writers_mutex);
    let mut already_registered: bool = r#false != 0;
    let mut empty_slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut duplicate_stream_name: bool = r#false != 0;
    let mut existing_writer: *const hls_writer_t = ::core::ptr::null::<hls_writer_t>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_HLS_WRITERS {
        if g_hls_writers[i as usize] == writer {
            already_registered = r#true != 0;
            break;
        } else {
            if !g_hls_writers[i as usize].is_null()
                && !(&raw mut (**(&raw mut g_hls_writers as *mut *mut hls_writer_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char)
                    .is_null()
                && !(&raw mut (*writer).stream_name as *mut ::core::ffi::c_char).is_null()
                && strcmp(
                    &raw mut (**(&raw mut g_hls_writers as *mut *mut hls_writer_t)
                        .offset(i as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                    &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                duplicate_stream_name = r#true != 0;
                existing_writer = g_hls_writers[i as usize];
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Found duplicate HLS writer for stream %s during registration\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
                );
            }
            if g_hls_writers[i as usize].is_null() && empty_slot == -1 as ::core::ffi::c_int {
                empty_slot = i;
            }
            i += 1;
        }
    }
    if duplicate_stream_name as ::core::ffi::c_int != 0 && !existing_writer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Multiple HLS writers detected for stream %s. This may cause issues.\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
        );
    }
    if !already_registered && empty_slot != -1 as ::core::ffi::c_int {
        g_hls_writers[empty_slot as usize] = writer;
        g_hls_writers_count += 1;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Registered HLS writer %p for stream %s for global tracking (total: %d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            writer as *mut ::core::ffi::c_void,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            g_hls_writers_count,
        );
    }
    pthread_mutex_unlock(&raw mut g_hls_writers_mutex);
}
unsafe extern "C" fn unregister_hls_writer(mut writer: *mut hls_writer_t) {
    if writer.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut g_hls_writers_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_HLS_WRITERS {
        if g_hls_writers[i as usize] == writer {
            g_hls_writers[i as usize] = ::core::ptr::null_mut::<hls_writer_t>();
            g_hls_writers_count -= 1;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Unregistered HLS writer %p for stream %s from global tracking (remaining: %d)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                writer as *mut ::core::ffi::c_void,
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
                g_hls_writers_count,
            );
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut g_hls_writers_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_all_hls_writers() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleaning up all HLS writers...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    pthread_mutex_lock(&raw mut g_hls_writers_mutex);
    let mut writers_to_close: [*mut hls_writer_t; 32] = [
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
        ::core::ptr::null_mut::<hls_writer_t>(),
    ];
    let mut writers_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_HLS_WRITERS {
        if !g_hls_writers[i as usize].is_null() {
            let c2rust_fresh0 = writers_count;
            writers_count += 1;
            writers_to_close[c2rust_fresh0 as usize] = g_hls_writers[i as usize];
            g_hls_writers[i as usize] = ::core::ptr::null_mut::<hls_writer_t>();
        }
        i += 1;
    }
    g_hls_writers_count = 0 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut g_hls_writers_mutex);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < writers_count {
        if !writers_to_close[i_0 as usize].is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Closing HLS writer %d/%d during global cleanup: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i_0 + 1 as ::core::ffi::c_int,
                writers_count,
                &raw mut (**(&raw mut writers_to_close as *mut *mut hls_writer_t)
                    .offset(i_0 as isize))
                .stream_name as *mut ::core::ffi::c_char,
            );
            hls_writer_close(writers_to_close[i_0 as usize]);
        }
        i_0 += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Completed cleanup of %d HLS writers\0".as_ptr() as *const ::core::ffi::c_char,
        writers_count,
    );
}
#[thread_local]
static mut in_writer_close: bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hls_writer_close(mut writer: *mut hls_writer_t) {
    if writer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Attempted to close NULL HLS writer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if in_writer_close {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Recursive call to hls_writer_close detected, aborting to prevent double free\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    in_writer_close = r#true != 0;
    static mut close_mutex: pthread_mutex_t = pthread_mutex_t {
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
    let mut close_timeout: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut close_timeout);
    close_timeout.tv_sec += 10 as __time_t;
    let mut close_mutex_result: ::core::ffi::c_int =
        pthread_mutex_timedlock(&raw mut close_mutex, &raw mut close_timeout);
    if close_mutex_result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Timeout waiting for HLS close mutex (error: %s), proceeding with close anyway\0"
                .as_ptr() as *const ::core::ffi::c_char,
            strerror(close_mutex_result),
        );
    }
    ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if (*writer).stream_name[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
            MAX_STREAM_NAME as size_t,
            0 as size_t,
        );
    } else {
        strcpy(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting to close HLS writer for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    if !(*writer).thread_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Clearing thread context reference for stream %s during writer close\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        (*writer).thread_ctx = NULL;
    }
    let mut timeout: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut timeout);
    timeout.tv_sec += 5 as __time_t;
    ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
    let mut mutex_result: ::core::ffi::c_int =
        pthread_mutex_timedlock(&raw mut (*writer).mutex, &raw mut timeout);
    if mutex_result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not acquire HLS writer mutex for stream %s within timeout, proceeding with forced close (error: %s)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            strerror(mutex_result),
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Successfully acquired mutex for HLS writer for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    if (*writer).output_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Attempted to close already closed HLS writer for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        if mutex_result == 0 as ::core::ffi::c_int {
            ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
            pthread_mutex_unlock(&raw mut (*writer).mutex);
        }
        in_writer_close = r#false != 0;
        if close_mutex_result == 0 as ::core::ffi::c_int {
            pthread_mutex_unlock(&raw mut close_mutex);
        }
        return;
    }
    let mut local_output_ctx: *mut AVFormatContext = ::core::ptr::null_mut::<AVFormatContext>();
    if !(*writer).output_ctx.is_null() {
        local_output_ctx = (*writer).output_ctx;
        (*writer).output_ctx = ::core::ptr::null_mut::<AVFormatContext>();
        (*writer).initialized = 0 as ::core::ffi::c_int;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Output context became NULL during close for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    if mutex_result == 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut (*writer).mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Released mutex for HLS writer for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    usleep(50000 as __useconds_t);
    if !local_output_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Writing trailer for HLS writer for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        let mut context_valid: bool = r#false != 0;
        if (*local_output_ctx).nb_streams > 0 as ::core::ffi::c_uint {
            if !(*local_output_ctx).oformat.is_null() && !(*local_output_ctx).pb.is_null() {
                let mut all_streams_valid: bool = r#true != 0;
                let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                while i < (*local_output_ctx).nb_streams {
                    if (*(*local_output_ctx).streams.offset(i as isize)).is_null()
                        || (**(*local_output_ctx).streams.offset(i as isize))
                            .codecpar
                            .is_null()
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Invalid stream %d in context for stream %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            i,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                        all_streams_valid = r#false != 0;
                        break;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
                if all_streams_valid {
                    context_valid = r#true != 0;
                }
            }
        }
        if context_valid {
            let mut ret: ::core::ffi::c_int = av_write_trailer(local_output_ctx);
            if ret < 0 as ::core::ffi::c_int {
                recording_io_report_failure(
                    recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
                    recording_io_operation_t::RECORDING_IO_OPERATION_TRAILER,
                    ret,
                );
                let mut error_buf: [::core::ffi::c_char; 64] = [
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
                av_strerror(
                    ret,
                    &raw mut error_buf as *mut ::core::ffi::c_char,
                    AV_ERROR_MAX_STRING_SIZE as size_t,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Error writing trailer for HLS writer for stream %s: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                    &raw mut error_buf as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Successfully wrote trailer for HLS writer for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Skipping trailer write for stream %s: invalid context state\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
        if !(*local_output_ctx).pb.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Closing AVIO context for HLS writer for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
            let mut pb_to_close: *mut AVIOContext = (*local_output_ctx).pb;
            (*local_output_ctx).pb = ::core::ptr::null_mut::<AVIOContext>();
            let mut close_ret: ::core::ffi::c_int = avio_closep(&raw mut pb_to_close);
            if close_ret < 0 as ::core::ffi::c_int {
                recording_io_report_failure(
                    recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
                    recording_io_operation_t::RECORDING_IO_OPERATION_CLOSE,
                    close_ret,
                );
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Successfully closed AVIO context for HLS writer for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
        usleep(100000 as ::core::ffi::c_int as __useconds_t);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Freeing format context for HLS writer for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        avformat_free_context(local_output_ctx);
        local_output_ctx = ::core::ptr::null_mut::<AVFormatContext>();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Successfully freed format context for HLS writer for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    if !(*writer).bsf_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Freeing bitstream filter context for HLS writer for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        let mut bsf_to_free: *mut AVBSFContext = (*writer).bsf_ctx;
        (*writer).bsf_ctx = ::core::ptr::null_mut::<AVBSFContext>();
        av_bsf_free(&raw mut bsf_to_free);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Successfully freed bitstream filter context for HLS writer for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    if mutex_result == 0 as ::core::ffi::c_int {
        let mut destroy_result: ::core::ffi::c_int =
            pthread_mutex_destroy(&raw mut (*writer).mutex);
        if destroy_result != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to destroy mutex for HLS writer for stream %s: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
                strerror(destroy_result),
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Successfully destroyed mutex for HLS writer for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Closed HLS writer for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    if !(*writer).output_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Output context still exists during final cleanup for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        if (*writer).initialized != 0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Writing trailer for stream %s during cleanup\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
            let mut trailer_ret: ::core::ffi::c_int = av_write_trailer((*writer).output_ctx);
            if trailer_ret < 0 as ::core::ffi::c_int {
                recording_io_report_failure(
                    recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
                    recording_io_operation_t::RECORDING_IO_OPERATION_TRAILER,
                    trailer_ret,
                );
            }
            (*writer).initialized = 0 as ::core::ffi::c_int;
        }
        if !(*(*writer).output_ctx).pb.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Closing AVIO context during final cleanup for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
            let mut close_ret_0: ::core::ffi::c_int =
                avio_closep(&raw mut (*(*writer).output_ctx).pb);
            if close_ret_0 < 0 as ::core::ffi::c_int {
                recording_io_report_failure(
                    recording_io_resource_t::RECORDING_IO_RESOURCE_HLS,
                    recording_io_operation_t::RECORDING_IO_OPERATION_CLOSE,
                    close_ret_0,
                );
            }
        }
        let mut i_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while i_0 < (*(*writer).output_ctx).nb_streams {
            if !(*(*(*writer).output_ctx).streams.offset(i_0 as isize)).is_null() {
                if !(**(*(*writer).output_ctx).streams.offset(i_0 as isize))
                    .codecpar
                    .is_null()
                {
                    avcodec_parameters_free(
                        &raw mut (**(*(*writer).output_ctx).streams.offset(i_0 as isize)).codecpar,
                    );
                }
                if !(**(*(*writer).output_ctx).streams.offset(i_0 as isize))
                    .metadata
                    .is_null()
                {
                    av_dict_free(
                        &raw mut (**(*(*writer).output_ctx).streams.offset(i_0 as isize)).metadata,
                    );
                }
            }
            i_0 = i_0.wrapping_add(1);
        }
        avformat_free_context((*writer).output_ctx);
        (*writer).output_ctx = ::core::ptr::null_mut::<AVFormatContext>();
    }
    if !(*writer).bsf_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Bitstream filter context still exists during final cleanup for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        av_bsf_free(&raw mut (*writer).bsf_ctx);
        (*writer).bsf_ctx = ::core::ptr::null_mut::<AVBSFContext>();
    }
    (*writer).dts_tracker.first_dts = 0 as int64_t;
    (*writer).dts_tracker.last_dts = 0 as int64_t;
    (*writer).dts_tracker.initialized = 0 as ::core::ffi::c_int;
    (*writer).audio_dts_tracker.initialized = 0 as ::core::ffi::c_int;
    unregister_hls_writer(writer);
    free(writer as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Freed HLS writer structure for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    in_writer_close = r#false != 0;
    if close_mutex_result == 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut close_mutex);
    }
}
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
