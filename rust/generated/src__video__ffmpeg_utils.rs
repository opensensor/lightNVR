use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type AVBuffer;
    pub type AVDictionary;
    pub type AVIAMFMixPresentation;
    pub type AVIAMFAudioElement;
    pub type AVCodecTag;
    pub type AVCodecInternal;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    unsafe fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn av_strerror(
        errnum: ::core::ffi::c_int,
        errbuf: *mut ::core::ffi::c_char,
        errbuf_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn av_freep(ptr: *mut ::core::ffi::c_void);
    unsafe fn av_rescale_q(a: int64_t, bq: AVRational, cq: AVRational) -> int64_t;
    unsafe fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn av_dict_free(m: *mut *mut AVDictionary);
    unsafe fn av_packet_alloc() -> *mut AVPacket;
    unsafe fn av_packet_free(pkt: *mut *mut AVPacket);
    unsafe fn av_packet_unref(pkt: *mut AVPacket);
    unsafe fn av_frame_alloc() -> *mut AVFrame;
    unsafe fn av_frame_free(frame: *mut *mut AVFrame);
    unsafe fn av_frame_get_buffer(
        frame: *mut AVFrame,
        align: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn av_frame_make_writable(frame: *mut AVFrame) -> ::core::ffi::c_int;
    unsafe fn avcodec_parameters_copy(
        dst: *mut AVCodecParameters,
        src: *const AVCodecParameters,
    ) -> ::core::ffi::c_int;
    unsafe fn avio_flush(s: *mut AVIOContext);
    unsafe fn avio_open(
        s: *mut *mut AVIOContext,
        url: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn avio_closep(s: *mut *mut AVIOContext) -> ::core::ffi::c_int;
    unsafe fn avcodec_find_encoder(id: AVCodecID) -> *const AVCodec;
    unsafe fn avformat_network_init() -> ::core::ffi::c_int;
    unsafe fn avformat_network_deinit() -> ::core::ffi::c_int;
    unsafe fn avformat_free_context(s: *mut AVFormatContext);
    unsafe fn avformat_new_stream(s: *mut AVFormatContext, c: *const AVCodec) -> *mut AVStream;
    unsafe fn avformat_alloc_output_context2(
        ctx: *mut *mut AVFormatContext,
        oformat: *const AVOutputFormat,
        format_name: *const ::core::ffi::c_char,
        filename: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn avformat_open_input(
        ps: *mut *mut AVFormatContext,
        url: *const ::core::ffi::c_char,
        fmt: *const AVInputFormat,
        options: *mut *mut AVDictionary,
    ) -> ::core::ffi::c_int;
    unsafe fn avformat_find_stream_info(
        ic: *mut AVFormatContext,
        options: *mut *mut AVDictionary,
    ) -> ::core::ffi::c_int;
    unsafe fn av_read_frame(s: *mut AVFormatContext, pkt: *mut AVPacket) -> ::core::ffi::c_int;
    unsafe fn avformat_close_input(s: *mut *mut AVFormatContext);
    unsafe fn avformat_write_header(
        s: *mut AVFormatContext,
        options: *mut *mut AVDictionary,
    ) -> ::core::ffi::c_int;
    unsafe fn av_interleaved_write_frame(
        s: *mut AVFormatContext,
        pkt: *mut AVPacket,
    ) -> ::core::ffi::c_int;
    unsafe fn av_write_trailer(s: *mut AVFormatContext) -> ::core::ffi::c_int;
    unsafe fn avcodec_alloc_context3(codec: *const AVCodec) -> *mut AVCodecContext;
    unsafe fn avcodec_free_context(avctx: *mut *mut AVCodecContext);
    unsafe fn avcodec_open2(
        avctx: *mut AVCodecContext,
        codec: *const AVCodec,
        options: *mut *mut AVDictionary,
    ) -> ::core::ffi::c_int;
    unsafe fn avcodec_send_frame(
        avctx: *mut AVCodecContext,
        frame: *const AVFrame,
    ) -> ::core::ffi::c_int;
    unsafe fn avcodec_receive_packet(
        avctx: *mut AVCodecContext,
        avpkt: *mut AVPacket,
    ) -> ::core::ffi::c_int;
    unsafe fn avcodec_flush_buffers(avctx: *mut AVCodecContext);
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
    unsafe fn config_get_mp4_faststart(config: *const config_t) -> bool;
    static mut g_config: config_t;
    unsafe fn url_redact_for_logging(
        url: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn ffmpeg_leak_detector_init();
    unsafe fn ffmpeg_leak_detector_cleanup();
    unsafe fn ffmpeg_track_allocation(
        ptr: *mut ::core::ffi::c_void,
        r#type: *const ::core::ffi::c_char,
        location: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
    );
    unsafe fn ffmpeg_untrack_allocation(ptr: *const ::core::ffi::c_void);
    unsafe fn ffmpeg_dump_allocations();
    unsafe fn ffmpeg_force_cleanup_all();
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
pub type uint8_t = u8;
pub type uint16_t = u16;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVPictureType(pub ::core::ffi::c_uint);
impl AVPictureType {
    pub const AV_PICTURE_TYPE_NONE: Self = Self(0);
    pub const AV_PICTURE_TYPE_I: Self = Self(1);
    pub const AV_PICTURE_TYPE_P: Self = Self(2);
    pub const AV_PICTURE_TYPE_B: Self = Self(3);
    pub const AV_PICTURE_TYPE_S: Self = Self(4);
    pub const AV_PICTURE_TYPE_SI: Self = Self(5);
    pub const AV_PICTURE_TYPE_SP: Self = Self(6);
    pub const AV_PICTURE_TYPE_BI: Self = Self(7);
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
    pub default_val: C2Rust_Unnamed,
    pub min: ::core::ffi::c_double,
    pub max: ::core::ffi::c_double,
    pub flags: ::core::ffi::c_int,
    pub unit: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
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
pub struct AVSampleFormat(pub ::core::ffi::c_int);
impl AVSampleFormat {
    pub const AV_SAMPLE_FMT_NONE: Self = Self(-1);
    pub const AV_SAMPLE_FMT_U8: Self = Self(0);
    pub const AV_SAMPLE_FMT_S16: Self = Self(1);
    pub const AV_SAMPLE_FMT_S32: Self = Self(2);
    pub const AV_SAMPLE_FMT_FLT: Self = Self(3);
    pub const AV_SAMPLE_FMT_DBL: Self = Self(4);
    pub const AV_SAMPLE_FMT_U8P: Self = Self(5);
    pub const AV_SAMPLE_FMT_S16P: Self = Self(6);
    pub const AV_SAMPLE_FMT_S32P: Self = Self(7);
    pub const AV_SAMPLE_FMT_FLTP: Self = Self(8);
    pub const AV_SAMPLE_FMT_DBLP: Self = Self(9);
    pub const AV_SAMPLE_FMT_S64: Self = Self(10);
    pub const AV_SAMPLE_FMT_S64P: Self = Self(11);
    pub const AV_SAMPLE_FMT_NB: Self = Self(12);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVAudioServiceType(pub ::core::ffi::c_uint);
impl AVAudioServiceType {
    pub const AV_AUDIO_SERVICE_TYPE_MAIN: Self = Self(0);
    pub const AV_AUDIO_SERVICE_TYPE_EFFECTS: Self = Self(1);
    pub const AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED: Self = Self(2);
    pub const AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED: Self = Self(3);
    pub const AV_AUDIO_SERVICE_TYPE_DIALOGUE: Self = Self(4);
    pub const AV_AUDIO_SERVICE_TYPE_COMMENTARY: Self = Self(5);
    pub const AV_AUDIO_SERVICE_TYPE_EMERGENCY: Self = Self(6);
    pub const AV_AUDIO_SERVICE_TYPE_VOICE_OVER: Self = Self(7);
    pub const AV_AUDIO_SERVICE_TYPE_KARAOKE: Self = Self(8);
    pub const AV_AUDIO_SERVICE_TYPE_NB: Self = Self(9);
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
pub struct AVFrameSideData {
    pub r#type: AVFrameSideDataType,
    pub data: *mut uint8_t,
    pub size: size_t,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVFrameSideDataType(pub ::core::ffi::c_uint);
impl AVFrameSideDataType {
    pub const AV_FRAME_DATA_PANSCAN: Self = Self(0);
    pub const AV_FRAME_DATA_A53_CC: Self = Self(1);
    pub const AV_FRAME_DATA_STEREO3D: Self = Self(2);
    pub const AV_FRAME_DATA_MATRIXENCODING: Self = Self(3);
    pub const AV_FRAME_DATA_DOWNMIX_INFO: Self = Self(4);
    pub const AV_FRAME_DATA_REPLAYGAIN: Self = Self(5);
    pub const AV_FRAME_DATA_DISPLAYMATRIX: Self = Self(6);
    pub const AV_FRAME_DATA_AFD: Self = Self(7);
    pub const AV_FRAME_DATA_MOTION_VECTORS: Self = Self(8);
    pub const AV_FRAME_DATA_SKIP_SAMPLES: Self = Self(9);
    pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: Self = Self(10);
    pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: Self = Self(11);
    pub const AV_FRAME_DATA_GOP_TIMECODE: Self = Self(12);
    pub const AV_FRAME_DATA_SPHERICAL: Self = Self(13);
    pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: Self = Self(14);
    pub const AV_FRAME_DATA_ICC_PROFILE: Self = Self(15);
    pub const AV_FRAME_DATA_S12M_TIMECODE: Self = Self(16);
    pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: Self = Self(17);
    pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: Self = Self(18);
    pub const AV_FRAME_DATA_VIDEO_ENC_PARAMS: Self = Self(19);
    pub const AV_FRAME_DATA_SEI_UNREGISTERED: Self = Self(20);
    pub const AV_FRAME_DATA_FILM_GRAIN_PARAMS: Self = Self(21);
    pub const AV_FRAME_DATA_DETECTION_BBOXES: Self = Self(22);
    pub const AV_FRAME_DATA_DOVI_RPU_BUFFER: Self = Self(23);
    pub const AV_FRAME_DATA_DOVI_METADATA: Self = Self(24);
    pub const AV_FRAME_DATA_DYNAMIC_HDR_VIVID: Self = Self(25);
    pub const AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT: Self = Self(26);
    pub const AV_FRAME_DATA_VIDEO_HINT: Self = Self(27);
    pub const AV_FRAME_DATA_LCEVC: Self = Self(28);
    pub const AV_FRAME_DATA_VIEW_ID: Self = Self(29);
    pub const AV_FRAME_DATA_3D_REFERENCE_DISPLAYS: Self = Self(30);
    pub const AV_FRAME_DATA_EXIF: Self = Self(31);
    pub const AV_FRAME_DATA_DYNAMIC_HDR_SMPTE_2094_APP5: Self = Self(32);
    pub const AV_FRAME_DATA_IAMF_MIX_GAIN_PARAM: Self = Self(33);
    pub const AV_FRAME_DATA_IAMF_DEMIXING_INFO_PARAM: Self = Self(34);
    pub const AV_FRAME_DATA_IAMF_RECON_GAIN_INFO_PARAM: Self = Self(35);
    pub const AV_FRAME_DATA_RAW_COLOR_PARAMS: Self = Self(36);
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
pub struct AVFrame {
    pub data: [*mut uint8_t; 8],
    pub linesize: [::core::ffi::c_int; 8],
    pub extended_data: *mut *mut uint8_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub nb_samples: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: int64_t,
    pub pkt_dts: int64_t,
    pub time_base: AVRational,
    pub quality: ::core::ffi::c_int,
    pub opaque: *mut ::core::ffi::c_void,
    pub repeat_pict: ::core::ffi::c_int,
    pub sample_rate: ::core::ffi::c_int,
    pub buf: [*mut AVBufferRef; 8],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: ::core::ffi::c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: int64_t,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: ::core::ffi::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: size_t,
    pub crop_bottom: size_t,
    pub crop_left: size_t,
    pub crop_right: size_t,
    pub private_ref: *mut ::core::ffi::c_void,
    pub ch_layout: AVChannelLayout,
    pub duration: int64_t,
    pub alpha_mode: AVAlphaMode,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecDescriptor {
    pub id: AVCodecID,
    pub r#type: AVMediaType,
    pub name: *const ::core::ffi::c_char,
    pub long_name: *const ::core::ffi::c_char,
    pub props: ::core::ffi::c_int,
    pub mime_types: *const *const ::core::ffi::c_char,
    pub profiles: *const AVProfile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RcOverride {
    pub start_frame: ::core::ffi::c_int,
    pub end_frame: ::core::ffi::c_int,
    pub qscale: ::core::ffi::c_int,
    pub quality_factor: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: ::core::ffi::c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    pub codec_tag: ::core::ffi::c_uint,
    pub priv_data: *mut ::core::ffi::c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut ::core::ffi::c_void,
    pub bit_rate: int64_t,
    pub flags: ::core::ffi::c_int,
    pub flags2: ::core::ffi::c_int,
    pub extradata: *mut uint8_t,
    pub extradata_size: ::core::ffi::c_int,
    pub time_base: AVRational,
    pub pkt_timebase: AVRational,
    pub framerate: AVRational,
    pub delay: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub coded_width: ::core::ffi::c_int,
    pub coded_height: ::core::ffi::c_int,
    pub sample_aspect_ratio: AVRational,
    pub pix_fmt: AVPixelFormat,
    pub sw_pix_fmt: AVPixelFormat,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub field_order: AVFieldOrder,
    pub refs: ::core::ffi::c_int,
    pub has_b_frames: ::core::ffi::c_int,
    pub slice_flags: ::core::ffi::c_int,
    pub draw_horiz_band: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVFrame,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub get_format:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat>,
    pub max_b_frames: ::core::ffi::c_int,
    pub b_quant_factor: ::core::ffi::c_float,
    pub b_quant_offset: ::core::ffi::c_float,
    pub i_quant_factor: ::core::ffi::c_float,
    pub i_quant_offset: ::core::ffi::c_float,
    pub lumi_masking: ::core::ffi::c_float,
    pub temporal_cplx_masking: ::core::ffi::c_float,
    pub spatial_cplx_masking: ::core::ffi::c_float,
    pub p_masking: ::core::ffi::c_float,
    pub dark_masking: ::core::ffi::c_float,
    pub nsse_weight: ::core::ffi::c_int,
    pub me_cmp: ::core::ffi::c_int,
    pub me_sub_cmp: ::core::ffi::c_int,
    pub mb_cmp: ::core::ffi::c_int,
    pub ildct_cmp: ::core::ffi::c_int,
    pub dia_size: ::core::ffi::c_int,
    pub last_predictor_count: ::core::ffi::c_int,
    pub me_pre_cmp: ::core::ffi::c_int,
    pub pre_dia_size: ::core::ffi::c_int,
    pub me_subpel_quality: ::core::ffi::c_int,
    pub me_range: ::core::ffi::c_int,
    pub mb_decision: ::core::ffi::c_int,
    pub intra_matrix: *mut uint16_t,
    pub inter_matrix: *mut uint16_t,
    pub chroma_intra_matrix: *mut uint16_t,
    pub intra_dc_precision: ::core::ffi::c_int,
    pub mb_lmin: ::core::ffi::c_int,
    pub mb_lmax: ::core::ffi::c_int,
    pub bidir_refine: ::core::ffi::c_int,
    pub keyint_min: ::core::ffi::c_int,
    pub gop_size: ::core::ffi::c_int,
    pub mv0_threshold: ::core::ffi::c_int,
    pub slices: ::core::ffi::c_int,
    pub sample_rate: ::core::ffi::c_int,
    pub sample_fmt: AVSampleFormat,
    pub ch_layout: AVChannelLayout,
    pub frame_size: ::core::ffi::c_int,
    pub block_align: ::core::ffi::c_int,
    pub cutoff: ::core::ffi::c_int,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub initial_padding: ::core::ffi::c_int,
    pub trailing_padding: ::core::ffi::c_int,
    pub seek_preroll: ::core::ffi::c_int,
    pub get_buffer2: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVFrame,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub bit_rate_tolerance: ::core::ffi::c_int,
    pub global_quality: ::core::ffi::c_int,
    pub compression_level: ::core::ffi::c_int,
    pub qcompress: ::core::ffi::c_float,
    pub qblur: ::core::ffi::c_float,
    pub qmin: ::core::ffi::c_int,
    pub qmax: ::core::ffi::c_int,
    pub max_qdiff: ::core::ffi::c_int,
    pub rc_buffer_size: ::core::ffi::c_int,
    pub rc_override_count: ::core::ffi::c_int,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: int64_t,
    pub rc_min_rate: int64_t,
    pub rc_max_available_vbv_use: ::core::ffi::c_float,
    pub rc_min_vbv_overflow_use: ::core::ffi::c_float,
    pub rc_initial_buffer_occupancy: ::core::ffi::c_int,
    pub trellis: ::core::ffi::c_int,
    pub stats_out: *mut ::core::ffi::c_char,
    pub stats_in: *mut ::core::ffi::c_char,
    pub workaround_bugs: ::core::ffi::c_int,
    pub strict_std_compliance: ::core::ffi::c_int,
    pub error_concealment: ::core::ffi::c_int,
    pub debug: ::core::ffi::c_int,
    pub err_recognition: ::core::ffi::c_int,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut ::core::ffi::c_void,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: ::core::ffi::c_int,
    pub extra_hw_frames: ::core::ffi::c_int,
    pub error: [uint64_t; 8],
    pub dct_algo: ::core::ffi::c_int,
    pub idct_algo: ::core::ffi::c_int,
    pub bits_per_coded_sample: ::core::ffi::c_int,
    pub bits_per_raw_sample: ::core::ffi::c_int,
    pub thread_count: ::core::ffi::c_int,
    pub thread_type: ::core::ffi::c_int,
    pub active_thread_type: ::core::ffi::c_int,
    pub execute: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub execute2: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub profile: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub skip_alpha: ::core::ffi::c_int,
    pub skip_top: ::core::ffi::c_int,
    pub skip_bottom: ::core::ffi::c_int,
    pub lowres: ::core::ffi::c_int,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub sub_charenc: *mut ::core::ffi::c_char,
    pub sub_charenc_mode: ::core::ffi::c_int,
    pub subtitle_header_size: ::core::ffi::c_int,
    pub subtitle_header: *mut uint8_t,
    pub dump_separator: *mut uint8_t,
    pub codec_whitelist: *mut ::core::ffi::c_char,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: ::core::ffi::c_int,
    pub export_side_data: ::core::ffi::c_int,
    pub max_pixels: int64_t,
    pub apply_cropping: ::core::ffi::c_int,
    pub discard_damaged_percentage: ::core::ffi::c_int,
    pub max_samples: int64_t,
    pub get_encode_buffer: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub frame_num: int64_t,
    pub side_data_prefer_packet: *mut ::core::ffi::c_int,
    pub nb_side_data_prefer_packet: ::core::ffi::c_uint,
    pub decoded_side_data: *mut *mut AVFrameSideData,
    pub nb_decoded_side_data: ::core::ffi::c_int,
    pub alpha_mode: AVAlphaMode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWAccel {
    pub name: *const ::core::ffi::c_char,
    pub r#type: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: ::core::ffi::c_int,
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
pub struct jpeg_encoder_cache {
    pub codec_ctx: *mut AVCodecContext,
    pub frame: *mut AVFrame,
    pub pkt: *mut AVPacket,
    pub sws_ctx: *mut SwsContext,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub channels: ::core::ffi::c_int,
    pub quality: ::core::ffi::c_int,
    pub src_pix_fmt: AVPixelFormat,
    pub mutex: pthread_mutex_t,
}
pub type jpeg_encoder_cache_t = jpeg_encoder_cache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_encoder_cache_entry_t {
    pub encoder: *mut jpeg_encoder_cache_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub channels: ::core::ffi::c_int,
    pub quality: ::core::ffi::c_int,
    pub last_used: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_t {
    pub pid_file: [::core::ffi::c_char; 512],
    pub log_file: [::core::ffi::c_char; 512],
    pub log_level: ::core::ffi::c_int,
    pub syslog_enabled: bool,
    pub syslog_ident: [::core::ffi::c_char; 64],
    pub syslog_facility: ::core::ffi::c_int,
    pub storage_path: [::core::ffi::c_char; 512],
    pub storage_path_hls: [::core::ffi::c_char; 512],
    pub max_storage_size: uint64_t,
    pub retention_days: ::core::ffi::c_int,
    pub auto_delete_oldest: bool,
    pub mp4_faststart: bool,
    pub storage_min_free_pct: ::core::ffi::c_int,
    pub storage_pressure_warning_pct: ::core::ffi::c_double,
    pub storage_pressure_critical_pct: ::core::ffi::c_double,
    pub storage_pressure_emergency_pct: ::core::ffi::c_double,
    pub generate_thumbnails: bool,
    pub thumbnails_per_recording: ::core::ffi::c_int,
    pub record_mp4_directly: bool,
    pub mp4_storage_path: [::core::ffi::c_char; 512],
    pub mp4_directory_format: [::core::ffi::c_char; 32],
    pub mp4_segment_duration: ::core::ffi::c_int,
    pub mp4_retention_days: ::core::ffi::c_int,
    pub models_path: [::core::ffi::c_char; 512],
    pub api_detection_url: [::core::ffi::c_char; 512],
    pub api_detection_backend: [::core::ffi::c_char; 32],
    pub api_detection_format: [::core::ffi::c_char; 32],
    pub api_detection_detector_name: [::core::ffi::c_char; 64],
    pub default_detection_threshold: ::core::ffi::c_int,
    pub default_pre_detection_buffer: ::core::ffi::c_int,
    pub default_post_detection_buffer: ::core::ffi::c_int,
    pub default_playback_transport: [::core::ffi::c_char; 24],
    pub detection_grace_period: ::core::ffi::c_int,
    pub db_path: [::core::ffi::c_char; 512],
    pub db_backup_interval_minutes: ::core::ffi::c_int,
    pub db_backup_retention_count: ::core::ffi::c_int,
    pub db_post_backup_script: [::core::ffi::c_char; 512],
    pub db_startup_check: ::core::ffi::c_int,
    pub db_backup_verify: ::core::ffi::c_int,
    pub web_thread_pool_size: ::core::ffi::c_int,
    pub web_port: ::core::ffi::c_int,
    pub web_bind_ip: [::core::ffi::c_char; 32],
    pub web_root: [::core::ffi::c_char; 512],
    pub web_ssl_enabled: bool,
    pub web_ssl_cert_path: [::core::ffi::c_char; 512],
    pub web_ssl_key_path: [::core::ffi::c_char; 512],
    pub web_auth_enabled: bool,
    pub web_username: [::core::ffi::c_char; 32],
    pub web_password: [::core::ffi::c_char; 32],
    pub audio_disabled: bool,
    pub auto_disabled: bool,
    pub webrtc_disabled: bool,
    pub hls_disabled: bool,
    pub mse_disabled: bool,
    pub auth_timeout_hours: ::core::ffi::c_int,
    pub auth_absolute_timeout_hours: ::core::ffi::c_int,
    pub trusted_device_days: ::core::ffi::c_int,
    pub trusted_proxy_cidrs: [::core::ffi::c_char; 1024],
    pub demo_mode: bool,
    pub force_mfa_on_login: bool,
    pub login_rate_limit_enabled: bool,
    pub login_rate_limit_max_attempts: ::core::ffi::c_int,
    pub login_rate_limit_window_seconds: ::core::ffi::c_int,
    pub web_compression_enabled: bool,
    pub web_use_minified_assets: bool,
    pub web_cache_max_age_html: ::core::ffi::c_int,
    pub web_cache_max_age_css: ::core::ffi::c_int,
    pub web_cache_max_age_js: ::core::ffi::c_int,
    pub web_cache_max_age_images: ::core::ffi::c_int,
    pub web_cache_max_age_fonts: ::core::ffi::c_int,
    pub web_cache_max_age_default: ::core::ffi::c_int,
    pub onvif_discovery_enabled: bool,
    pub onvif_discovery_interval: ::core::ffi::c_int,
    pub onvif_discovery_network: [::core::ffi::c_char; 64],
    pub max_streams: ::core::ffi::c_int,
    pub streams: *mut stream_config_t,
    pub buffer_size: ::core::ffi::c_int,
    pub use_swap: bool,
    pub swap_file: [::core::ffi::c_char; 512],
    pub swap_size: uint64_t,
    pub memory_constrained: bool,
    pub hw_accel_enabled: bool,
    pub hw_accel_device: [::core::ffi::c_char; 32],
    pub go2rtc_enabled: bool,
    pub go2rtc_binary_path: [::core::ffi::c_char; 512],
    pub go2rtc_config_dir: [::core::ffi::c_char; 512],
    pub go2rtc_api_port: ::core::ffi::c_int,
    pub go2rtc_rtsp_port: ::core::ffi::c_int,
    pub go2rtc_force_native_hls: bool,
    pub go2rtc_proxy_max_inflight: ::core::ffi::c_int,
    pub go2rtc_webrtc_enabled: bool,
    pub go2rtc_webrtc_listen_port: ::core::ffi::c_int,
    pub go2rtc_stun_enabled: bool,
    pub go2rtc_stun_server: [::core::ffi::c_char; 256],
    pub go2rtc_external_ip: [::core::ffi::c_char; 64],
    pub go2rtc_ice_servers: [::core::ffi::c_char; 512],
    pub webrtc_connection_timeout_ms: ::core::ffi::c_int,
    pub webrtc_ice_recovery_timeout_ms: ::core::ffi::c_int,
    pub turn_enabled: bool,
    pub turn_server_url: [::core::ffi::c_char; 256],
    pub turn_username: [::core::ffi::c_char; 64],
    pub turn_password: [::core::ffi::c_char; 64],
    pub mqtt_enabled: bool,
    pub mqtt_broker_host: [::core::ffi::c_char; 256],
    pub mqtt_broker_port: ::core::ffi::c_int,
    pub mqtt_username: [::core::ffi::c_char; 64],
    pub mqtt_password: [::core::ffi::c_char; 128],
    pub mqtt_client_id: [::core::ffi::c_char; 64],
    pub mqtt_topic_prefix: [::core::ffi::c_char; 128],
    pub mqtt_tls_enabled: bool,
    pub mqtt_keepalive: ::core::ffi::c_int,
    pub mqtt_qos: ::core::ffi::c_int,
    pub mqtt_retain: bool,
    pub mqtt_ha_discovery: bool,
    pub mqtt_ha_discovery_prefix: [::core::ffi::c_char; 128],
    pub mqtt_ha_snapshot_interval: ::core::ffi::c_int,
    pub health: system_health_policy_settings_t,
    pub detection_engine: detection_engine_config_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_engine_config_t {
    pub enabled: bool,
    pub num_threads: ::core::ffi::c_int,
    pub delegate: [::core::ffi::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_policy_settings_t {
    pub enabled: bool,
    pub profile: [::core::ffi::c_char; 16],
    pub fast_interval_seconds: uint32_t,
    pub normal_interval_seconds: uint32_t,
    pub slow_interval_seconds: uint32_t,
    pub device_interval_seconds: uint32_t,
    pub write_probe_enabled: bool,
    pub hardware_provider: [::core::ffi::c_char; 16],
    pub presence_interval_seconds: uint32_t,
    pub incident_retention_days: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_config_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 256],
    pub url: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_int,
    pub codec: [::core::ffi::c_char; 16],
    pub priority: ::core::ffi::c_int,
    pub record: bool,
    pub segment_duration: ::core::ffi::c_int,
    pub detection_based_recording: bool,
    pub detection_model: [::core::ffi::c_char; 512],
    pub detection_interval: ::core::ffi::c_int,
    pub detection_threshold: ::core::ffi::c_float,
    pub pre_detection_buffer: ::core::ffi::c_int,
    pub post_detection_buffer: ::core::ffi::c_int,
    pub detection_api_url: [::core::ffi::c_char; 512],
    pub detection_object_filter: [::core::ffi::c_char; 16],
    pub detection_object_filter_list: [::core::ffi::c_char; 256],
    pub streaming_enabled: bool,
    pub protocol: stream_protocol_t,
    pub record_audio: bool,
    pub audio_voice_enhancement: bool,
    pub onvif_username: [::core::ffi::c_char; 64],
    pub onvif_password: [::core::ffi::c_char; 64],
    pub onvif_profile: [::core::ffi::c_char; 64],
    pub onvif_port: ::core::ffi::c_int,
    pub onvif_discovery_enabled: bool,
    pub is_onvif: bool,
    pub backchannel_enabled: bool,
    pub retention_days: ::core::ffi::c_int,
    pub detection_retention_days: ::core::ffi::c_int,
    pub max_storage_mb: ::core::ffi::c_int,
    pub tier_critical_multiplier: ::core::ffi::c_double,
    pub tier_important_multiplier: ::core::ffi::c_double,
    pub tier_ephemeral_multiplier: ::core::ffi::c_double,
    pub storage_priority: ::core::ffi::c_int,
    pub ptz_enabled: bool,
    pub ptz_max_x: ::core::ffi::c_int,
    pub ptz_max_y: ::core::ffi::c_int,
    pub ptz_max_z: ::core::ffi::c_int,
    pub ptz_has_home: bool,
    pub record_on_schedule: bool,
    pub recording_schedule: [uint8_t; 168],
    pub detection_record_on_schedule: bool,
    pub detection_recording_schedule: [uint8_t; 168],
    pub tags: [::core::ffi::c_char; 256],
    pub admin_url: [::core::ffi::c_char; 512],
    pub privacy_mode: bool,
    pub motion_trigger_source: [::core::ffi::c_char; 256],
    pub go2rtc_source_override: [::core::ffi::c_char; 8192],
    pub sub_stream_url: [::core::ffi::c_char; 512],
    pub detection_url: [::core::ffi::c_char; 512],
    pub publish_url: [::core::ffi::c_char; 512],
    pub playback_transport: [::core::ffi::c_char; 24],
    pub eptz_config: [::core::ffi::c_char; 1024],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
pub const FF_QP2LAMBDA: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const AV_NOPTS_VALUE: int64_t = 0x8000000000000000 as ::core::ffi::c_ulong as int64_t;
pub const AV_ERROR_MAX_STRING_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const AVIO_FLAG_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AVFMT_NOFILE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const AV_CODEC_FLAG_QSCALE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffmpeg_frame_to_rgb24_padded(
    mut frame: *const AVFrame,
    mut rgb_size_out: *mut size_t,
) -> *mut uint8_t {
    if !rgb_size_out.is_null() {
        *rgb_size_out = 0 as size_t;
    }
    if frame.is_null()
        || (*frame).width <= 0 as ::core::ffi::c_int
        || (*frame).height <= 0 as ::core::ffi::c_int
        || (*frame).format == AVPixelFormat::AV_PIX_FMT_NONE.0
        || !(*frame).hw_frames_ctx.is_null()
        || (*frame).data[0usize].is_null()
    {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    let mut sws_ctx: *mut SwsContext = sws_getContext(
        (*frame).width,
        (*frame).height,
        AVPixelFormat((*frame).format),
        (*frame).width,
        (*frame).height,
        AVPixelFormat::AV_PIX_FMT_RGB24,
        SwsFlags::SWS_BILINEAR.0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<SwsFilter>(),
        ::core::ptr::null_mut::<SwsFilter>(),
        ::core::ptr::null::<::core::ffi::c_double>(),
    );
    if sws_ctx.is_null() {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    let mut rgb_size: size_t = ((*frame).width as size_t)
        .wrapping_mul((*frame).height as size_t)
        .wrapping_mul(3 as size_t);
    let mut rgb_buf: *mut uint8_t =
        malloc(rgb_size.wrapping_add(RGB24_SWS_TAIL_PADDING as size_t)) as *mut uint8_t;
    if rgb_buf.is_null() {
        sws_freeContext(sws_ctx);
        return ::core::ptr::null_mut::<uint8_t>();
    }
    memset(
        rgb_buf.offset(rgb_size as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        RGB24_SWS_TAIL_PADDING as size_t,
    );
    let mut rgb_data: [*mut uint8_t; 4] = [
        rgb_buf,
        ::core::ptr::null_mut::<uint8_t>(),
        ::core::ptr::null_mut::<uint8_t>(),
        ::core::ptr::null_mut::<uint8_t>(),
    ];
    let mut rgb_linesize: [::core::ffi::c_int; 4] = [
        (*frame).width * 3 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut rows: ::core::ffi::c_int = sws_scale(
        sws_ctx,
        &raw const (*frame).data as *const *mut uint8_t as *const *const uint8_t,
        &raw const (*frame).linesize as *const ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        (*frame).height,
        &raw mut rgb_data as *mut *mut uint8_t,
        &raw mut rgb_linesize as *mut ::core::ffi::c_int,
    );
    sws_freeContext(sws_ctx);
    if rows <= 0 as ::core::ffi::c_int {
        free(rgb_buf as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<uint8_t>();
    }
    if !rgb_size_out.is_null() {
        *rgb_size_out = rgb_size;
    }
    return rgb_buf;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_ffmpeg_error(
    mut err: ::core::ffi::c_int,
    mut message: *const ::core::ffi::c_char,
) {
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
        err,
        &raw mut error_buf as *mut ::core::ffi::c_char,
        AV_ERROR_MAX_STRING_SIZE as size_t,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
        message,
        &raw mut error_buf as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_ffmpeg() {
    avformat_network_init();
    ffmpeg_leak_detector_init();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"FFmpeg initialized with leak detection\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_ffmpeg() {
    ffmpeg_force_cleanup_all();
    ffmpeg_leak_detector_cleanup();
    avformat_network_deinit();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"FFmpeg cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_packet_cleanup(mut pkt_ptr: *mut *mut AVPacket) {
    if !pkt_ptr.is_null() && !(*pkt_ptr).is_null() {
        let mut pkt_to_free: *mut AVPacket = *pkt_ptr;
        *pkt_ptr = ::core::ptr::null_mut::<AVPacket>();
        ffmpeg_untrack_allocation(pkt_to_free as *mut ::core::ffi::c_void);
        av_packet_unref(pkt_to_free);
        av_packet_free(&raw mut pkt_to_free);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Safely cleaned up packet\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_avformat_cleanup(mut ctx_ptr: *mut *mut AVFormatContext) {
    if ctx_ptr.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"NULL pointer passed to safe_avformat_cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*ctx_ptr).is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"NULL AVFormatContext passed to safe_avformat_cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ctx_to_close: *mut AVFormatContext = *ctx_ptr;
    *ctx_ptr = ::core::ptr::null_mut::<AVFormatContext>();
    ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
    if ctx_to_close.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Context became NULL during cleanup\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    ffmpeg_untrack_allocation(ctx_to_close as *mut ::core::ffi::c_void);
    if !(*ctx_to_close).pb.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Flushing I/O buffers during cleanup\0".as_ptr() as *const ::core::ffi::c_char,
        );
        avio_flush((*ctx_to_close).pb);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No I/O context available during cleanup\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*ctx_to_close).nb_streams > 0 as ::core::ffi::c_uint && !(*ctx_to_close).streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Closing input context with %d streams\0".as_ptr() as *const ::core::ffi::c_char,
            (*ctx_to_close).nb_streams,
        );
        avformat_close_input(&raw mut ctx_to_close);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Context has no streams, using direct free\0".as_ptr() as *const ::core::ffi::c_char,
        );
        avformat_free_context(ctx_to_close);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Safely cleaned up AVFormatContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn comprehensive_ffmpeg_cleanup(
    mut input_ctx: *mut *mut AVFormatContext,
    mut codec_ctx: *mut *mut AVCodecContext,
    mut packet: *mut *mut AVPacket,
    mut frame: *mut *mut AVFrame,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting comprehensive FFmpeg resource cleanup\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !frame.is_null() && !(*frame).is_null() {
        let mut frame_to_free: *mut AVFrame = *frame;
        *frame = ::core::ptr::null_mut::<AVFrame>();
        ffmpeg_untrack_allocation(frame_to_free as *mut ::core::ffi::c_void);
        av_frame_free(&raw mut frame_to_free);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaned up AVFrame\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !packet.is_null() {
        safe_packet_cleanup(packet);
    }
    if !codec_ctx.is_null() && !(*codec_ctx).is_null() {
        let mut codec_to_free: *mut AVCodecContext = *codec_ctx;
        *codec_ctx = ::core::ptr::null_mut::<AVCodecContext>();
        ffmpeg_untrack_allocation(codec_to_free as *mut ::core::ffi::c_void);
        avcodec_flush_buffers(codec_to_free);
        avcodec_free_context(&raw mut codec_to_free);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaned up AVCodecContext\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !input_ctx.is_null() && !(*input_ctx).is_null() {
        safe_avformat_cleanup(input_ctx);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Comprehensive FFmpeg resource cleanup completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn periodic_ffmpeg_reset(
    mut input_ctx_ptr: *mut *mut AVFormatContext,
    mut url: *const ::core::ffi::c_char,
    mut protocol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if input_ctx_ptr.is_null() || url.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters passed to periodic_ffmpeg_reset\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut safe_url: [::core::ffi::c_char; 512] = [0; 512];
    if url_redact_for_logging(
        url,
        &raw mut safe_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut safe_url as *mut ::core::ffi::c_char,
            b"[invalid-url]\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Performing periodic FFmpeg resource reset for URL: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut safe_url as *mut ::core::ffi::c_char,
    );
    ffmpeg_dump_allocations();
    if !(*input_ctx_ptr).is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Closing existing input context during reset\0".as_ptr() as *const ::core::ffi::c_char,
        );
        safe_avformat_cleanup(input_ctx_ptr);
    }
    av_freep(NULL);
    let mut new_ctx: *mut AVFormatContext = ::core::ptr::null_mut::<AVFormatContext>();
    let mut options: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
    if protocol == stream_protocol_t::STREAM_PROTOCOL_TCP.0 as ::core::ffi::c_int {
        av_dict_set(
            &raw mut options,
            b"rtsp_transport\0".as_ptr() as *const ::core::ffi::c_char,
            b"tcp\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
    }
    av_dict_set(
        &raw mut options,
        b"stimeout\0".as_ptr() as *const ::core::ffi::c_char,
        b"5000000\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut options,
        b"reconnect\0".as_ptr() as *const ::core::ffi::c_char,
        b"1\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut options,
        b"reconnect_streamed\0".as_ptr() as *const ::core::ffi::c_char,
        b"1\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut options,
        b"reconnect_delay_max\0".as_ptr() as *const ::core::ffi::c_char,
        b"2\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    let mut ret: ::core::ffi::c_int = avformat_open_input(
        &raw mut new_ctx,
        url,
        ::core::ptr::null::<AVInputFormat>(),
        &raw mut options,
    );
    av_dict_free(&raw mut options);
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
            b"Failed to open input during reset: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut error_buf as *mut ::core::ffi::c_char,
        );
        return ret;
    }
    ret = avformat_find_stream_info(new_ctx, ::core::ptr::null_mut::<*mut AVDictionary>());
    if ret < 0 as ::core::ffi::c_int {
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
            b"Failed to find stream info during reset: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut error_buf_0 as *mut ::core::ffi::c_char,
        );
        avformat_close_input(&raw mut new_ctx);
        return ret;
    }
    ffmpeg_track_allocation(
        new_ctx as *mut ::core::ffi::c_void,
        b"AVFormatContext\0".as_ptr() as *const ::core::ffi::c_char,
        b"src/video/ffmpeg_utils.c\0".as_ptr() as *const ::core::ffi::c_char,
        315 as ::core::ffi::c_int,
    );
    *input_ctx_ptr = new_ctx;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Successfully reset FFmpeg resources\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffmpeg_encode_jpeg(
    mut frame_data: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut channels: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_int,
    mut output_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut qscale: ::core::ffi::c_int = 0;
    let mut src_data: [*const uint8_t; 4] = [::core::ptr::null::<uint8_t>(); 4];
    let mut src_linesize: [::core::ffi::c_int; 4] = [0; 4];
    if frame_data.is_null()
        || output_path.is_null()
        || width <= 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for JPEG encoding\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut codec_ctx: *mut AVCodecContext = ::core::ptr::null_mut::<AVCodecContext>();
    let mut frame: *mut AVFrame = ::core::ptr::null_mut::<AVFrame>();
    let mut pkt: *mut AVPacket = ::core::ptr::null_mut::<AVPacket>();
    let mut sws_ctx: *mut SwsContext = ::core::ptr::null_mut::<SwsContext>();
    let mut outfile: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut src_pix_fmt: AVPixelFormat = AVPixelFormat::AV_PIX_FMT_YUV420P;
    match channels {
        1 => {
            src_pix_fmt = AVPixelFormat::AV_PIX_FMT_GRAY8;
        }
        3 => {
            src_pix_fmt = AVPixelFormat::AV_PIX_FMT_RGB24;
        }
        4 => {
            src_pix_fmt = AVPixelFormat::AV_PIX_FMT_RGBA;
        }
        _ => {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Unsupported channel count: %d\0".as_ptr() as *const ::core::ffi::c_char,
                channels,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    let mut codec: *const AVCodec = avcodec_find_encoder(AVCodecID::AV_CODEC_ID_MJPEG);
    if codec.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MJPEG encoder not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    codec_ctx = avcodec_alloc_context3(codec);
    if codec_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate codec context\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ret = -1 as ::core::ffi::c_int;
    } else {
        (*codec_ctx).width = width;
        (*codec_ctx).height = height;
        (*codec_ctx).time_base = AVRational {
            num: 1 as ::core::ffi::c_int,
            den: 25 as ::core::ffi::c_int,
        };
        (*codec_ctx).pix_fmt = AVPixelFormat::AV_PIX_FMT_YUVJ420P;
        qscale = 31 as ::core::ffi::c_int
            - quality * 30 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
        if qscale < 1 as ::core::ffi::c_int {
            qscale = 1 as ::core::ffi::c_int;
        }
        if qscale > 31 as ::core::ffi::c_int {
            qscale = 31 as ::core::ffi::c_int;
        }
        (*codec_ctx).flags |= AV_CODEC_FLAG_QSCALE;
        (*codec_ctx).global_quality = qscale * FF_QP2LAMBDA;
        ret = avcodec_open2(
            codec_ctx,
            codec,
            ::core::ptr::null_mut::<*mut AVDictionary>(),
        );
        if ret < 0 as ::core::ffi::c_int {
            log_ffmpeg_error(
                ret,
                b"Failed to open MJPEG codec\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            frame = av_frame_alloc();
            if frame.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to allocate frame\0".as_ptr() as *const ::core::ffi::c_char,
                );
                ret = -1 as ::core::ffi::c_int;
            } else {
                (*frame).format = (*codec_ctx).pix_fmt.0;
                (*frame).width = width;
                (*frame).height = height;
                ret = av_frame_get_buffer(frame, 0 as ::core::ffi::c_int);
                if ret < 0 as ::core::ffi::c_int {
                    log_ffmpeg_error(
                        ret,
                        b"Failed to allocate frame buffer\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    sws_ctx = sws_getContext(
                        width,
                        height,
                        src_pix_fmt,
                        width,
                        height,
                        (*codec_ctx).pix_fmt,
                        SwsFlags::SWS_BILINEAR.0 as ::core::ffi::c_int,
                        ::core::ptr::null_mut::<SwsFilter>(),
                        ::core::ptr::null_mut::<SwsFilter>(),
                        ::core::ptr::null::<::core::ffi::c_double>(),
                    ) as *mut SwsContext;
                    if sws_ctx.is_null() {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to create scaling context\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        ret = -1 as ::core::ffi::c_int;
                    } else {
                        src_data = [
                            frame_data as *const uint8_t,
                            ::core::ptr::null::<uint8_t>(),
                            ::core::ptr::null::<uint8_t>(),
                            ::core::ptr::null::<uint8_t>(),
                        ];
                        src_linesize = [
                            width * channels,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        ];
                        sws_scale(
                            sws_ctx,
                            &raw mut src_data as *mut *const uint8_t,
                            &raw mut src_linesize as *mut ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            height,
                            &raw mut (*frame).data as *mut *mut uint8_t,
                            &raw mut (*frame).linesize as *mut ::core::ffi::c_int,
                        );
                        pkt = av_packet_alloc();
                        if pkt.is_null() {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Failed to allocate packet\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            ret = -1 as ::core::ffi::c_int;
                        } else {
                            ret = avcodec_send_frame(codec_ctx, frame);
                            if ret < 0 as ::core::ffi::c_int {
                                log_ffmpeg_error(
                                    ret,
                                    b"Failed to send frame for encoding\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            } else {
                                ret = avcodec_receive_packet(codec_ctx, pkt);
                                if ret < 0 as ::core::ffi::c_int {
                                    log_ffmpeg_error(
                                        ret,
                                        b"Failed to receive encoded packet\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                } else {
                                    let mut jpeg_fd: ::core::ffi::c_int = open(
                                        output_path,
                                        O_WRONLY | O_CREAT | O_TRUNC,
                                        0o640 as ::core::ffi::c_int,
                                    );
                                    outfile = if jpeg_fd >= 0 as ::core::ffi::c_int {
                                        fdopen(
                                            jpeg_fd,
                                            b"wb\0".as_ptr() as *const ::core::ffi::c_char,
                                        )
                                    } else {
                                        ::core::ptr::null_mut::<FILE>()
                                    };
                                    if outfile.is_null() {
                                        if jpeg_fd >= 0 as ::core::ffi::c_int {
                                            close(jpeg_fd);
                                        }
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_ERROR,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Failed to open output file %s: %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            output_path,
                                            strerror(*__errno_location()),
                                        );
                                        ret = -1 as ::core::ffi::c_int;
                                    } else if fwrite(
                                        (*pkt).data as *const ::core::ffi::c_void,
                                        1 as size_t,
                                        (*pkt).size as size_t,
                                        outfile,
                                    ) as size_t
                                        != (*pkt).size as size_t
                                    {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_ERROR,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Failed to write JPEG data to file\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        ret = -1 as ::core::ffi::c_int;
                                    } else {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_DEBUG,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Encoded JPEG: %dx%d, %d bytes -> %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            width,
                                            height,
                                            (*pkt).size,
                                            output_path,
                                        );
                                        ret = 0 as ::core::ffi::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !outfile.is_null() {
        fclose(outfile);
    }
    if !pkt.is_null() {
        av_packet_free(&raw mut pkt);
    }
    if !sws_ctx.is_null() {
        sws_freeContext(sws_ctx);
    }
    if !frame.is_null() {
        av_frame_free(&raw mut frame);
    }
    if !codec_ctx.is_null() {
        avcodec_free_context(&raw mut codec_ctx);
    }
    return ret;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jpeg_encoder_cache_create(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut channels: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_int,
) -> *mut jpeg_encoder_cache_t {
    if width <= 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
        || channels < 1 as ::core::ffi::c_int
        || channels > 4 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for JPEG encoder cache: %dx%d, %d channels\0".as_ptr()
                as *const ::core::ffi::c_char,
            width,
            height,
            channels,
        );
        return ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
    }
    let mut encoder: *mut jpeg_encoder_cache_t =
        calloc(1 as size_t, ::core::mem::size_of::<jpeg_encoder_cache_t>())
            as *mut jpeg_encoder_cache_t;
    if encoder.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate JPEG encoder cache\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
    }
    (*encoder).width = width;
    (*encoder).height = height;
    (*encoder).channels = channels;
    (*encoder).quality = quality;
    pthread_mutex_init(
        &raw mut (*encoder).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    );
    match channels {
        1 => {
            (*encoder).src_pix_fmt = AVPixelFormat::AV_PIX_FMT_GRAY8;
        }
        3 => {
            (*encoder).src_pix_fmt = AVPixelFormat::AV_PIX_FMT_RGB24;
        }
        4 => {
            (*encoder).src_pix_fmt = AVPixelFormat::AV_PIX_FMT_RGBA;
        }
        _ => {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Unsupported channel count: %d\0".as_ptr() as *const ::core::ffi::c_char,
                channels,
            );
            free(encoder as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
        }
    }
    let mut codec: *const AVCodec = avcodec_find_encoder(AVCodecID::AV_CODEC_ID_MJPEG);
    if codec.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"MJPEG encoder not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(encoder as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
    }
    (*encoder).codec_ctx = avcodec_alloc_context3(codec);
    if (*encoder).codec_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate codec context for JPEG cache\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(encoder as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
    }
    (*(*encoder).codec_ctx).width = width;
    (*(*encoder).codec_ctx).height = height;
    (*(*encoder).codec_ctx).time_base = AVRational {
        num: 1 as ::core::ffi::c_int,
        den: 25 as ::core::ffi::c_int,
    };
    (*(*encoder).codec_ctx).pix_fmt = AVPixelFormat::AV_PIX_FMT_YUVJ420P;
    let mut qscale: ::core::ffi::c_int =
        31 as ::core::ffi::c_int - quality * 30 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
    if qscale < 1 as ::core::ffi::c_int {
        qscale = 1 as ::core::ffi::c_int;
    }
    if qscale > 31 as ::core::ffi::c_int {
        qscale = 31 as ::core::ffi::c_int;
    }
    (*(*encoder).codec_ctx).flags |= AV_CODEC_FLAG_QSCALE;
    (*(*encoder).codec_ctx).global_quality = qscale * FF_QP2LAMBDA;
    let mut ret: ::core::ffi::c_int = avcodec_open2(
        (*encoder).codec_ctx,
        codec,
        ::core::ptr::null_mut::<*mut AVDictionary>(),
    );
    if ret < 0 as ::core::ffi::c_int {
        log_ffmpeg_error(
            ret,
            b"Failed to open MJPEG codec for cache\0".as_ptr() as *const ::core::ffi::c_char,
        );
        avcodec_free_context(&raw mut (*encoder).codec_ctx);
        free(encoder as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
    }
    (*encoder).frame = av_frame_alloc();
    if (*encoder).frame.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate frame for JPEG cache\0".as_ptr() as *const ::core::ffi::c_char,
        );
        avcodec_free_context(&raw mut (*encoder).codec_ctx);
        free(encoder as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
    }
    (*(*encoder).frame).format = (*(*encoder).codec_ctx).pix_fmt.0;
    (*(*encoder).frame).width = width;
    (*(*encoder).frame).height = height;
    ret = av_frame_get_buffer((*encoder).frame, 0 as ::core::ffi::c_int);
    if ret < 0 as ::core::ffi::c_int {
        log_ffmpeg_error(
            ret,
            b"Failed to allocate frame buffer for JPEG cache\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        av_frame_free(&raw mut (*encoder).frame);
        avcodec_free_context(&raw mut (*encoder).codec_ctx);
        free(encoder as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
    }
    (*encoder).sws_ctx = sws_getContext(
        width,
        height,
        (*encoder).src_pix_fmt,
        width,
        height,
        (*(*encoder).codec_ctx).pix_fmt,
        SwsFlags::SWS_BILINEAR.0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<SwsFilter>(),
        ::core::ptr::null_mut::<SwsFilter>(),
        ::core::ptr::null::<::core::ffi::c_double>(),
    ) as *mut SwsContext;
    if (*encoder).sws_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create scaling context for JPEG cache\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        av_frame_free(&raw mut (*encoder).frame);
        avcodec_free_context(&raw mut (*encoder).codec_ctx);
        free(encoder as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
    }
    (*encoder).pkt = av_packet_alloc();
    if (*encoder).pkt.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate packet for JPEG cache\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sws_freeContext((*encoder).sws_ctx);
        av_frame_free(&raw mut (*encoder).frame);
        avcodec_free_context(&raw mut (*encoder).codec_ctx);
        free(encoder as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Created cached JPEG encoder: %dx%d, %d channels, quality %d\0".as_ptr()
            as *const ::core::ffi::c_char,
        width,
        height,
        channels,
        quality,
    );
    return encoder;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jpeg_encoder_cache_encode(
    mut encoder: *mut jpeg_encoder_cache_t,
    mut frame_data: *const ::core::ffi::c_uchar,
    mut output_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if encoder.is_null() || frame_data.is_null() || output_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for cached JPEG encode\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*encoder).mutex);
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut outfile: *mut FILE = ::core::ptr::null_mut::<FILE>();
    ret = av_frame_make_writable((*encoder).frame);
    if ret < 0 as ::core::ffi::c_int {
        log_ffmpeg_error(
            ret,
            b"Failed to make frame writable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut (*encoder).mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut src_data: [*const uint8_t; 4] = [
        frame_data as *const uint8_t,
        ::core::ptr::null::<uint8_t>(),
        ::core::ptr::null::<uint8_t>(),
        ::core::ptr::null::<uint8_t>(),
    ];
    let mut src_linesize: [::core::ffi::c_int; 4] = [
        (*encoder).width * (*encoder).channels,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    sws_scale(
        (*encoder).sws_ctx,
        &raw mut src_data as *mut *const uint8_t,
        &raw mut src_linesize as *mut ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        (*encoder).height,
        &raw mut (*(*encoder).frame).data as *mut *mut uint8_t,
        &raw mut (*(*encoder).frame).linesize as *mut ::core::ffi::c_int,
    );
    av_packet_unref((*encoder).pkt);
    ret = avcodec_send_frame((*encoder).codec_ctx, (*encoder).frame);
    if ret < 0 as ::core::ffi::c_int {
        log_ffmpeg_error(
            ret,
            b"Failed to send frame for cached encoding\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut (*encoder).mutex);
        return -1 as ::core::ffi::c_int;
    }
    ret = avcodec_receive_packet((*encoder).codec_ctx, (*encoder).pkt);
    if ret < 0 as ::core::ffi::c_int {
        log_ffmpeg_error(
            ret,
            b"Failed to receive encoded packet from cache\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut (*encoder).mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut jpeg_fd2: ::core::ffi::c_int = open(
        output_path,
        O_WRONLY | O_CREAT | O_TRUNC,
        0o640 as ::core::ffi::c_int,
    );
    outfile = if jpeg_fd2 >= 0 as ::core::ffi::c_int {
        fdopen(jpeg_fd2, b"wb\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<FILE>()
    };
    if outfile.is_null() {
        if jpeg_fd2 >= 0 as ::core::ffi::c_int {
            close(jpeg_fd2);
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open output file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            output_path,
            strerror(*__errno_location()),
        );
        pthread_mutex_unlock(&raw mut (*encoder).mutex);
        return -1 as ::core::ffi::c_int;
    }
    if fwrite(
        (*(*encoder).pkt).data as *const ::core::ffi::c_void,
        1 as size_t,
        (*(*encoder).pkt).size as size_t,
        outfile,
    ) as size_t
        != (*(*encoder).pkt).size as size_t
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to write JPEG data to file\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fclose(outfile);
        pthread_mutex_unlock(&raw mut (*encoder).mutex);
        return -1 as ::core::ffi::c_int;
    }
    fclose(outfile);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cached JPEG encode: %dx%d, %d bytes -> %s\0".as_ptr() as *const ::core::ffi::c_char,
        (*encoder).width,
        (*encoder).height,
        (*(*encoder).pkt).size,
        output_path,
    );
    pthread_mutex_unlock(&raw mut (*encoder).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jpeg_encoder_cache_encode_to_memory(
    mut encoder: *mut jpeg_encoder_cache_t,
    mut frame_data: *const ::core::ffi::c_uchar,
    mut out_data: *mut *mut ::core::ffi::c_uchar,
    mut out_size: *mut size_t,
) -> ::core::ffi::c_int {
    if encoder.is_null() || frame_data.is_null() || out_data.is_null() || out_size.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for cached JPEG encode to memory\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*encoder).mutex);
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ret = av_frame_make_writable((*encoder).frame);
    if ret < 0 as ::core::ffi::c_int {
        log_ffmpeg_error(
            ret,
            b"Failed to make frame writable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut (*encoder).mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut src_data: [*const uint8_t; 4] = [
        frame_data as *const uint8_t,
        ::core::ptr::null::<uint8_t>(),
        ::core::ptr::null::<uint8_t>(),
        ::core::ptr::null::<uint8_t>(),
    ];
    let mut src_linesize: [::core::ffi::c_int; 4] = [
        (*encoder).width * (*encoder).channels,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    sws_scale(
        (*encoder).sws_ctx,
        &raw mut src_data as *mut *const uint8_t,
        &raw mut src_linesize as *mut ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        (*encoder).height,
        &raw mut (*(*encoder).frame).data as *mut *mut uint8_t,
        &raw mut (*(*encoder).frame).linesize as *mut ::core::ffi::c_int,
    );
    av_packet_unref((*encoder).pkt);
    ret = avcodec_send_frame((*encoder).codec_ctx, (*encoder).frame);
    if ret < 0 as ::core::ffi::c_int {
        log_ffmpeg_error(
            ret,
            b"Failed to send frame for cached encoding to memory\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut (*encoder).mutex);
        return -1 as ::core::ffi::c_int;
    }
    ret = avcodec_receive_packet((*encoder).codec_ctx, (*encoder).pkt);
    if ret < 0 as ::core::ffi::c_int {
        log_ffmpeg_error(
            ret,
            b"Failed to receive encoded packet from cache\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut (*encoder).mutex);
        return -1 as ::core::ffi::c_int;
    }
    *out_data = malloc((*(*encoder).pkt).size as size_t) as *mut ::core::ffi::c_uchar;
    if (*out_data).is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for JPEG output\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut (*encoder).mutex);
        return -1 as ::core::ffi::c_int;
    }
    memcpy(
        *out_data as *mut ::core::ffi::c_void,
        (*(*encoder).pkt).data as *const ::core::ffi::c_void,
        (*(*encoder).pkt).size as size_t,
    );
    *out_size = (*(*encoder).pkt).size as size_t;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cached JPEG encode to memory: %dx%d, %d bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (*encoder).width,
        (*encoder).height,
        (*(*encoder).pkt).size,
    );
    pthread_mutex_unlock(&raw mut (*encoder).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jpeg_encoder_cache_destroy(mut encoder: *mut jpeg_encoder_cache_t) {
    if encoder.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut (*encoder).mutex);
    if !(*encoder).pkt.is_null() {
        av_packet_free(&raw mut (*encoder).pkt);
    }
    if !(*encoder).sws_ctx.is_null() {
        sws_freeContext((*encoder).sws_ctx);
    }
    if !(*encoder).frame.is_null() {
        av_frame_free(&raw mut (*encoder).frame);
    }
    if !(*encoder).codec_ctx.is_null() {
        avcodec_free_context(&raw mut (*encoder).codec_ctx);
    }
    pthread_mutex_unlock(&raw mut (*encoder).mutex);
    pthread_mutex_destroy(&raw mut (*encoder).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Destroyed cached JPEG encoder: %dx%d\0".as_ptr() as *const ::core::ffi::c_char,
        (*encoder).width,
        (*encoder).height,
    );
    free(encoder as *mut ::core::ffi::c_void);
}
pub const MAX_JPEG_ENCODER_CACHE: ::core::ffi::c_int = MAX_STREAMS;
static mut jpeg_encoder_global_cache: [jpeg_encoder_cache_entry_t; 1024] =
    [jpeg_encoder_cache_entry_t {
        encoder: ::core::ptr::null_mut::<jpeg_encoder_cache_t>(),
        width: 0,
        height: 0,
        channels: 0,
        quality: 0,
        last_used: 0,
    }; 1024];
static mut jpeg_encoder_global_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_4::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut jpeg_encoder_cache_initialized: bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jpeg_encoder_get_cached(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut channels: ::core::ffi::c_int,
    mut quality: ::core::ffi::c_int,
) -> *mut jpeg_encoder_cache_t {
    pthread_mutex_lock(&raw mut jpeg_encoder_global_mutex);
    if !jpeg_encoder_cache_initialized {
        memset(
            &raw mut jpeg_encoder_global_cache as *mut jpeg_encoder_cache_entry_t
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[jpeg_encoder_cache_entry_t; 1024]>(),
        );
        jpeg_encoder_cache_initialized = r#true != 0;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut oldest_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut oldest_time: time_t = now;
    let mut free_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_JPEG_ENCODER_CACHE {
        if !jpeg_encoder_global_cache[i as usize].encoder.is_null() {
            if jpeg_encoder_global_cache[i as usize].width == width
                && jpeg_encoder_global_cache[i as usize].height == height
                && jpeg_encoder_global_cache[i as usize].channels == channels
                && jpeg_encoder_global_cache[i as usize].quality == quality
            {
                jpeg_encoder_global_cache[i as usize].last_used = now;
                let mut encoder: *mut jpeg_encoder_cache_t =
                    jpeg_encoder_global_cache[i as usize].encoder;
                pthread_mutex_unlock(&raw mut jpeg_encoder_global_mutex);
                return encoder;
            }
            if jpeg_encoder_global_cache[i as usize].last_used < oldest_time {
                oldest_time = jpeg_encoder_global_cache[i as usize].last_used;
                oldest_idx = i;
            }
        } else if free_idx < 0 as ::core::ffi::c_int {
            free_idx = i;
        }
        i += 1;
    }
    let mut target_idx: ::core::ffi::c_int = 0;
    if free_idx >= 0 as ::core::ffi::c_int {
        target_idx = free_idx;
    } else if oldest_idx >= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Evicting cached JPEG encoder: %dx%d to make room for %dx%d\0".as_ptr()
                as *const ::core::ffi::c_char,
            jpeg_encoder_global_cache[oldest_idx as usize].width,
            jpeg_encoder_global_cache[oldest_idx as usize].height,
            width,
            height,
        );
        jpeg_encoder_cache_destroy(jpeg_encoder_global_cache[oldest_idx as usize].encoder);
        jpeg_encoder_global_cache[oldest_idx as usize].encoder =
            ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
        target_idx = oldest_idx;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No available slot for JPEG encoder cache\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut jpeg_encoder_global_mutex);
        return ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
    }
    let mut new_encoder: *mut jpeg_encoder_cache_t =
        jpeg_encoder_cache_create(width, height, channels, quality);
    if !new_encoder.is_null() {
        jpeg_encoder_global_cache[target_idx as usize].encoder = new_encoder;
        jpeg_encoder_global_cache[target_idx as usize].width = width;
        jpeg_encoder_global_cache[target_idx as usize].height = height;
        jpeg_encoder_global_cache[target_idx as usize].channels = channels;
        jpeg_encoder_global_cache[target_idx as usize].quality = quality;
        jpeg_encoder_global_cache[target_idx as usize].last_used = now;
    }
    pthread_mutex_unlock(&raw mut jpeg_encoder_global_mutex);
    return new_encoder;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jpeg_encoder_cleanup_all() {
    pthread_mutex_lock(&raw mut jpeg_encoder_global_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_JPEG_ENCODER_CACHE {
        if !jpeg_encoder_global_cache[i as usize].encoder.is_null() {
            jpeg_encoder_cache_destroy(jpeg_encoder_global_cache[i as usize].encoder);
            jpeg_encoder_global_cache[i as usize].encoder =
                ::core::ptr::null_mut::<jpeg_encoder_cache_t>();
        }
        i += 1;
    }
    jpeg_encoder_cache_initialized = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleaned up all cached JPEG encoders\0".as_ptr() as *const ::core::ffi::c_char,
    );
    pthread_mutex_unlock(&raw mut jpeg_encoder_global_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffmpeg_concat_ts_to_mp4(
    mut segment_paths: *mut *const ::core::ffi::c_char,
    mut segment_count: ::core::ffi::c_int,
    mut output_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if segment_paths.is_null() || segment_count <= 0 as ::core::ffi::c_int || output_path.is_null()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for TS concatenation\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut output_ctx: *mut AVFormatContext = ::core::ptr::null_mut::<AVFormatContext>();
    let mut input_contexts: *mut *mut AVFormatContext =
        ::core::ptr::null_mut::<*mut AVFormatContext>();
    let mut stream_mapping: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut last_pts: *mut int64_t = ::core::ptr::null_mut::<int64_t>();
    let mut last_dts: *mut int64_t = ::core::ptr::null_mut::<int64_t>();
    let mut pts_offset: *mut int64_t = ::core::ptr::null_mut::<int64_t>();
    let mut dts_offset: *mut int64_t = ::core::ptr::null_mut::<int64_t>();
    let mut first_pts: *mut int64_t = ::core::ptr::null_mut::<int64_t>();
    let mut first_dts: *mut int64_t = ::core::ptr::null_mut::<int64_t>();
    let mut first_packet: *mut bool = ::core::ptr::null_mut::<bool>();
    let mut output_stream_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut opts: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
    input_contexts = calloc(
        segment_count as size_t,
        ::core::mem::size_of::<*mut AVFormatContext>(),
    ) as *mut *mut AVFormatContext;
    if input_contexts.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate input contexts array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    ret = avformat_alloc_output_context2(
        &raw mut output_ctx,
        ::core::ptr::null::<AVOutputFormat>(),
        b"mp4\0".as_ptr() as *const ::core::ffi::c_char,
        output_path,
    );
    '_cleanup: {
        if ret < 0 as ::core::ffi::c_int || output_ctx.is_null() {
            log_ffmpeg_error(
                ret,
                b"Failed to create output context\0".as_ptr() as *const ::core::ffi::c_char,
            );
            ret = -1 as ::core::ffi::c_int;
        } else {
            ret = avformat_open_input(
                input_contexts.offset(0isize),
                *segment_paths.offset(0isize),
                ::core::ptr::null::<AVInputFormat>(),
                ::core::ptr::null_mut::<*mut AVDictionary>(),
            );
            if ret < 0 as ::core::ffi::c_int {
                log_ffmpeg_error(
                    ret,
                    b"Failed to open first input segment\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                ret = avformat_find_stream_info(
                    *input_contexts.offset(0isize),
                    ::core::ptr::null_mut::<*mut AVDictionary>(),
                );
                if ret < 0 as ::core::ffi::c_int {
                    log_ffmpeg_error(
                        ret,
                        b"Failed to find stream info\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    output_stream_count =
                        (**input_contexts.offset(0isize)).nb_streams as ::core::ffi::c_int;
                    stream_mapping = calloc(
                        output_stream_count as size_t,
                        ::core::mem::size_of::<::core::ffi::c_int>(),
                    ) as *mut ::core::ffi::c_int;
                    last_pts = calloc(
                        output_stream_count as size_t,
                        ::core::mem::size_of::<int64_t>(),
                    ) as *mut int64_t;
                    last_dts = calloc(
                        output_stream_count as size_t,
                        ::core::mem::size_of::<int64_t>(),
                    ) as *mut int64_t;
                    pts_offset = calloc(
                        output_stream_count as size_t,
                        ::core::mem::size_of::<int64_t>(),
                    ) as *mut int64_t;
                    dts_offset = calloc(
                        output_stream_count as size_t,
                        ::core::mem::size_of::<int64_t>(),
                    ) as *mut int64_t;
                    first_pts = calloc(
                        output_stream_count as size_t,
                        ::core::mem::size_of::<int64_t>(),
                    ) as *mut int64_t;
                    first_dts = calloc(
                        output_stream_count as size_t,
                        ::core::mem::size_of::<int64_t>(),
                    ) as *mut int64_t;
                    first_packet = calloc(
                        output_stream_count as size_t,
                        ::core::mem::size_of::<bool>(),
                    ) as *mut bool;
                    if stream_mapping.is_null()
                        || last_pts.is_null()
                        || last_dts.is_null()
                        || pts_offset.is_null()
                        || dts_offset.is_null()
                        || first_pts.is_null()
                        || first_dts.is_null()
                        || first_packet.is_null()
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to allocate stream tracking arrays\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        ret = -1 as ::core::ffi::c_int;
                    } else {
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i < output_stream_count {
                            let mut in_stream: *mut AVStream =
                                *(**input_contexts.offset(0isize)).streams.offset(i as isize);
                            let mut out_stream: *mut AVStream =
                                avformat_new_stream(output_ctx, ::core::ptr::null::<AVCodec>());
                            if out_stream.is_null() {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_ERROR,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Failed to create output stream\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                ret = -1 as ::core::ffi::c_int;
                                break '_cleanup;
                            } else {
                                ret = avcodec_parameters_copy(
                                    (*out_stream).codecpar,
                                    (*in_stream).codecpar,
                                );
                                if ret < 0 as ::core::ffi::c_int {
                                    log_ffmpeg_error(
                                        ret,
                                        b"Failed to copy codec parameters\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                    break '_cleanup;
                                } else {
                                    (*(*out_stream).codecpar).codec_tag = 0 as uint32_t;
                                    (*out_stream).time_base = (*in_stream).time_base;
                                    *stream_mapping.offset(i as isize) = i;
                                    *last_pts.offset(i as isize) = 0 as int64_t;
                                    *last_dts.offset(i as isize) = 0 as int64_t;
                                    *pts_offset.offset(i as isize) = 0 as int64_t;
                                    *dts_offset.offset(i as isize) = 0 as int64_t;
                                    i += 1;
                                }
                            }
                        }
                        if config_get_mp4_faststart(&raw mut g_config) {
                            av_dict_set(
                                &raw mut opts,
                                b"movflags\0".as_ptr() as *const ::core::ffi::c_char,
                                b"+faststart\0".as_ptr() as *const ::core::ffi::c_char,
                                0 as ::core::ffi::c_int,
                            );
                        }
                        if (*(*output_ctx).oformat).flags & AVFMT_NOFILE == 0 {
                            ret =
                                avio_open(&raw mut (*output_ctx).pb, output_path, AVIO_FLAG_WRITE);
                            if ret < 0 as ::core::ffi::c_int {
                                log_ffmpeg_error(
                                    ret,
                                    b"Failed to open output file\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                break '_cleanup;
                            }
                        }
                        ret = avformat_write_header(output_ctx, &raw mut opts);
                        av_dict_free(&raw mut opts);
                        opts = ::core::ptr::null_mut::<AVDictionary>();
                        if ret < 0 as ::core::ffi::c_int {
                            log_ffmpeg_error(
                                ret,
                                b"Failed to write header\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        } else {
                            let mut seg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while seg < segment_count {
                                's_255: {
                                    if seg > 0 as ::core::ffi::c_int {
                                        ret = avformat_open_input(
                                            input_contexts.offset(seg as isize),
                                            *segment_paths.offset(seg as isize),
                                            ::core::ptr::null::<AVInputFormat>(),
                                            ::core::ptr::null_mut::<*mut AVDictionary>(),
                                        );
                                        if ret < 0 as ::core::ffi::c_int {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_WARN,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Failed to open segment %s, skipping\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                *segment_paths.offset(seg as isize),
                                            );
                                            break 's_255;
                                        } else {
                                            ret = avformat_find_stream_info(
                                                *input_contexts.offset(seg as isize),
                                                ::core::ptr::null_mut::<*mut AVDictionary>(),
                                            );
                                            if ret < 0 as ::core::ffi::c_int {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_WARN,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Failed to find stream info for segment %s, skipping\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    *segment_paths.offset(seg as isize),
                                                );
                                                avformat_close_input(
                                                    input_contexts.offset(seg as isize),
                                                );
                                                break 's_255;
                                            }
                                        }
                                    }
                                    let mut pkt: *mut AVPacket = av_packet_alloc();
                                    if pkt.is_null() {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_ERROR,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Failed to allocate packet\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        ret = -1 as ::core::ffi::c_int;
                                        break '_cleanup;
                                    } else {
                                        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                        while i_0 < output_stream_count {
                                            *first_pts.offset(i_0 as isize) = AV_NOPTS_VALUE;
                                            *first_dts.offset(i_0 as isize) = AV_NOPTS_VALUE;
                                            *first_packet.offset(i_0 as isize) = r#true != 0;
                                            i_0 += 1;
                                        }
                                        while av_read_frame(
                                            *input_contexts.offset(seg as isize),
                                            pkt,
                                        ) >= 0 as ::core::ffi::c_int
                                        {
                                            let mut stream_idx: ::core::ffi::c_int =
                                                (*pkt).stream_index;
                                            if stream_idx >= output_stream_count {
                                                av_packet_unref(pkt);
                                            } else {
                                                let mut in_stream_0: *mut AVStream =
                                                    *(**input_contexts.offset(seg as isize))
                                                        .streams
                                                        .offset(stream_idx as isize);
                                                let mut out_stream_0: *mut AVStream =
                                                    *(*output_ctx)
                                                        .streams
                                                        .offset(stream_idx as isize);
                                                if *first_packet.offset(stream_idx as isize) {
                                                    *first_pts.offset(stream_idx as isize) =
                                                        (*pkt).pts;
                                                    *first_dts.offset(stream_idx as isize) =
                                                        (*pkt).dts;
                                                    *first_packet.offset(stream_idx as isize) =
                                                        r#false != 0;
                                                }
                                                if (*pkt).pts != AV_NOPTS_VALUE {
                                                    (*pkt).pts = av_rescale_q(
                                                        (*pkt).pts
                                                            - *first_pts
                                                                .offset(stream_idx as isize),
                                                        (*in_stream_0).time_base,
                                                        (*out_stream_0).time_base,
                                                    ) + *pts_offset
                                                        .offset(stream_idx as isize);
                                                }
                                                if (*pkt).dts != AV_NOPTS_VALUE {
                                                    (*pkt).dts = av_rescale_q(
                                                        (*pkt).dts
                                                            - *first_dts
                                                                .offset(stream_idx as isize),
                                                        (*in_stream_0).time_base,
                                                        (*out_stream_0).time_base,
                                                    ) + *dts_offset
                                                        .offset(stream_idx as isize);
                                                }
                                                if (*pkt).pts != AV_NOPTS_VALUE
                                                    && (*pkt).dts != AV_NOPTS_VALUE
                                                    && (*pkt).dts > (*pkt).pts
                                                {
                                                    (*pkt).dts = (*pkt).pts;
                                                }
                                                if (*pkt).pts != AV_NOPTS_VALUE {
                                                    *last_pts.offset(stream_idx as isize) =
                                                        (*pkt).pts;
                                                }
                                                if (*pkt).dts != AV_NOPTS_VALUE {
                                                    *last_dts.offset(stream_idx as isize) =
                                                        (*pkt).dts;
                                                }
                                                (*pkt).duration = av_rescale_q(
                                                    (*pkt).duration,
                                                    (*in_stream_0).time_base,
                                                    (*out_stream_0).time_base,
                                                );
                                                (*pkt).pos = -1 as int64_t;
                                                ret = av_interleaved_write_frame(output_ctx, pkt);
                                                if ret < 0 as ::core::ffi::c_int {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_WARN,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Failed to write packet from segment %d\0"
                                                            .as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        seg,
                                                    );
                                                }
                                                av_packet_unref(pkt);
                                            }
                                        }
                                        av_packet_free(&raw mut pkt);
                                        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                        while i_1 < output_stream_count {
                                            let mut out_stream_1: *mut AVStream =
                                                *(*output_ctx).streams.offset(i_1 as isize);
                                            let mut gap: int64_t = av_rescale_q(
                                                1 as int64_t,
                                                AVRational {
                                                    num: 1 as ::core::ffi::c_int,
                                                    den: 90000 as ::core::ffi::c_int,
                                                },
                                                (*out_stream_1).time_base,
                                            );
                                            *pts_offset.offset(i_1 as isize) =
                                                *last_pts.offset(i_1 as isize) + gap;
                                            *dts_offset.offset(i_1 as isize) =
                                                *last_dts.offset(i_1 as isize) + gap;
                                            i_1 += 1;
                                        }
                                        avformat_close_input(input_contexts.offset(seg as isize));
                                    }
                                }
                                seg += 1;
                            }
                            ret = av_write_trailer(output_ctx);
                            if ret < 0 as ::core::ffi::c_int {
                                log_ffmpeg_error(
                                    ret,
                                    b"Failed to write trailer\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Successfully concatenated %d segments to %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    segment_count,
                                    output_path,
                                );
                                ret = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    if !input_contexts.is_null() {
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < segment_count {
            if !(*input_contexts.offset(i_2 as isize)).is_null() {
                avformat_close_input(input_contexts.offset(i_2 as isize));
            }
            i_2 += 1;
        }
        free(input_contexts as *mut ::core::ffi::c_void);
    }
    if !output_ctx.is_null() {
        if (*(*output_ctx).oformat).flags & AVFMT_NOFILE == 0 && !(*output_ctx).pb.is_null() {
            avio_closep(&raw mut (*output_ctx).pb);
        }
        avformat_free_context(output_ctx);
    }
    if !opts.is_null() {
        av_dict_free(&raw mut opts);
    }
    free(stream_mapping as *mut ::core::ffi::c_void);
    free(last_pts as *mut ::core::ffi::c_void);
    free(last_dts as *mut ::core::ffi::c_void);
    free(pts_offset as *mut ::core::ffi::c_void);
    free(dts_offset as *mut ::core::ffi::c_void);
    free(first_pts as *mut ::core::ffi::c_void);
    free(first_dts as *mut ::core::ffi::c_void);
    free(first_packet as *mut ::core::ffi::c_void);
    return ret;
}
pub const RGB24_SWS_TAIL_PADDING: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
