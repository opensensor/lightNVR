unsafe extern "C" {
    pub type AVDictionary;
    pub type AVOptionRanges;
    pub type AVOption;
    pub type AVBuffer;
    pub type AVIAMFMixPresentation;
    pub type AVIAMFAudioElement;
    pub type AVCodecTag;
    pub type AVCodecInternal;
    unsafe fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn av_dict_free(m: *mut *mut AVDictionary);
    unsafe fn av_log_set_level(level: ::core::ffi::c_int);
    unsafe fn avio_seek(
        s: *mut AVIOContext,
        offset: int64_t,
        whence: ::core::ffi::c_int,
    ) -> int64_t;
    unsafe fn avio_flush(s: *mut AVIOContext);
    unsafe fn recording_file_open(
        context: *mut *mut AVIOContext,
        path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn recording_file_close(context: *mut *mut AVIOContext) -> ::core::ffi::c_int;
    unsafe fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn dirname(__path: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn av_strerror(
        errnum: ::core::ffi::c_int,
        errbuf: *mut ::core::ffi::c_char,
        errbuf_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn av_free(ptr: *mut ::core::ffi::c_void);
    unsafe fn av_rescale_q(a: int64_t, bq: AVRational, cq: AVRational) -> int64_t;
    unsafe fn avcodec_get_name(id: AVCodecID) -> *const ::core::ffi::c_char;
    unsafe fn av_packet_alloc() -> *mut AVPacket;
    unsafe fn av_packet_free(pkt: *mut *mut AVPacket);
    unsafe fn av_packet_ref(dst: *mut AVPacket, src: *const AVPacket) -> ::core::ffi::c_int;
    unsafe fn av_packet_unref(pkt: *mut AVPacket);
    unsafe fn av_packet_move_ref(dst: *mut AVPacket, src: *mut AVPacket);
    unsafe fn av_packet_rescale_ts(pkt: *mut AVPacket, tb_src: AVRational, tb_dst: AVRational);
    unsafe fn av_frame_alloc() -> *mut AVFrame;
    unsafe fn av_frame_free(frame: *mut *mut AVFrame);
    unsafe fn av_frame_unref(frame: *mut AVFrame);
    unsafe fn avcodec_parameters_free(par: *mut *mut AVCodecParameters);
    unsafe fn avcodec_parameters_copy(
        dst: *mut AVCodecParameters,
        src: *const AVCodecParameters,
    ) -> ::core::ffi::c_int;
    unsafe fn avcodec_find_decoder(id: AVCodecID) -> *const AVCodec;
    unsafe fn avformat_network_init() -> ::core::ffi::c_int;
    unsafe fn avformat_network_deinit() -> ::core::ffi::c_int;
    unsafe fn avformat_alloc_context() -> *mut AVFormatContext;
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
    unsafe fn avcodec_parameters_to_context(
        codec: *mut AVCodecContext,
        par: *const AVCodecParameters,
    ) -> ::core::ffi::c_int;
    unsafe fn avcodec_open2(
        avctx: *mut AVCodecContext,
        codec: *const AVCodec,
        options: *mut *mut AVDictionary,
    ) -> ::core::ffi::c_int;
    unsafe fn avcodec_send_packet(
        avctx: *mut AVCodecContext,
        avpkt: *const AVPacket,
    ) -> ::core::ffi::c_int;
    unsafe fn avcodec_receive_frame(
        avctx: *mut AVCodecContext,
        frame: *mut AVFrame,
    ) -> ::core::ffi::c_int;
    unsafe fn av_gettime() -> int64_t;
    unsafe fn av_gettime_relative() -> int64_t;
    unsafe fn av_usleep(usec: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    unsafe fn config_get_mp4_faststart(config: *const config_t) -> bool;
    static mut g_config: config_t;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn url_redact_for_logging(
        url: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn is_shutdown_initiated() -> bool;
    unsafe fn file_cache_release_due(
        released_through: off_t,
        current_position: off_t,
        interval_bytes: off_t,
    ) -> bool;
    unsafe fn file_cache_flush_and_release(
        fd: ::core::ffi::c_int,
        offset: off_t,
        length: off_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn stream_frame_rate_observe(
        measurement: *mut stream_frame_rate_measurement_t,
        now_us: int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn is_pcm_codec(codec_id: AVCodecID) -> bool;
    unsafe fn is_audio_codec_compatible_with_mp4(
        codec_id: AVCodecID,
        codec_name: *mut *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn transcode_pcm_to_aac(
        codec_params: *const AVCodecParameters,
        time_base: *const AVRational,
        stream_name: *const ::core::ffi::c_char,
        transcoded_params: *mut *mut AVCodecParameters,
    ) -> ::core::ffi::c_int;
    unsafe fn transcode_audio_packet(
        stream_name: *const ::core::ffi::c_char,
        in_pkt: *const AVPacket,
        out_pkt: *mut AVPacket,
        input_stream: *const AVStream,
    ) -> ::core::ffi::c_int;
    unsafe fn transcode_audio_flush(
        stream_name: *const ::core::ffi::c_char,
        packets_out: *mut *mut *mut AVPacket,
        packet_count_out: *mut ::core::ffi::c_int,
        discard_padding_out: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn cleanup_audio_transcoder(stream_name: *const ::core::ffi::c_char);
    unsafe fn recording_io_report_failure(
        resource: recording_io_resource_t,
        operation: recording_io_operation_t,
        error_code: ::core::ffi::c_int,
    );
    unsafe fn metrics_record_frame_from_source(
        stream_name: *const ::core::ffi::c_char,
        bytes: ::core::ffi::c_int,
        is_video: bool,
        source: metrics_frame_source_t,
    );
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
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
pub type off_t = __off_t;
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
    pub u: C2Rust_Unnamed,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
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
    pub params: C2Rust_Unnamed_0,
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
pub union C2Rust_Unnamed_0 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_1,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
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
    pub offsets: *mut C2Rust_Unnamed_2,
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
pub struct C2Rust_Unnamed_2 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
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
    pub observation_evidence_enabled: bool,
    pub observation_labels: [::core::ffi::c_char; 512],
    pub observation_max_age_days: ::core::ffi::c_int,
    pub observation_max_bytes: int64_t,
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
    pub go2rtc_external_ip: [::core::ffi::c_char; 256],
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub type atomic_int = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_frame_rate_measurement_t {
    pub started: bool,
    pub warmup_until_us: int64_t,
    pub window_start_us: int64_t,
    pub packets: uint64_t,
    pub fps: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp4_writer {
    pub output_path: [::core::ffi::c_char; 512],
    pub stream_name: [::core::ffi::c_char; 256],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub output_ctx: *mut AVFormatContext,
    pub reserved_output: *mut AVIOContext,
    pub video_stream_idx: ::core::ffi::c_int,
    pub has_audio: ::core::ffi::c_int,
    pub first_dts: int64_t,
    pub first_pts: int64_t,
    pub last_dts: int64_t,
    pub time_base: AVRational,
    pub is_initialized: ::core::ffi::c_int,
    pub creation_time: time_t,
    pub last_packet_time: time_t,
    pub audio: mp4_audio_state_t,
    pub mutex: pthread_mutex_t,
    pub current_recording_id: uint64_t,
    pub placement: storage_placement_t,
    pub segment_duration: ::core::ffi::c_int,
    pub last_rotation_time: time_t,
    pub waiting_for_keyframe: ::core::ffi::c_int,
    pub is_rotating: ::core::ffi::c_int,
    pub output_dir: [::core::ffi::c_char; 512],
    pub trigger_type: [::core::ffi::c_char; 16],
    pub schedule_restricted: ::core::ffi::c_int,
    pub start_time_corrected: bool,
    pub thread_ctx: *mut mp4_writer_thread_t,
    pub shutdown_component_id: ::core::ffi::c_int,
    pub pre_buffer_seconds: ::core::ffi::c_int,
    pub pending_audio_codecpar: *mut AVCodecParameters,
    pub pending_audio_time_base: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp4_writer_thread_t {
    pub thread: pthread_t,
    pub running: ::core::ffi::c_int,
    pub rtsp_url: [::core::ffi::c_char; 512],
    pub shutdown_requested: atomic_int,
    pub writer: *mut mp4_writer_t,
    pub segment_duration: ::core::ffi::c_int,
    pub last_segment_time: time_t,
    pub retry_count: ::core::ffi::c_int,
    pub last_retry_time: time_t,
    pub auto_restart: bool,
    pub force_reconnect: atomic_int,
    pub input_ctx: *mut AVFormatContext,
    pub segment_info: segment_info_t,
    pub context_mutex: pthread_mutex_t,
    pub video_params_detected: bool,
    pub segment_retry_count: ::core::ffi::c_int,
    pub last_segment_retry_time: time_t,
    pub segment_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segment_info_t {
    pub segment_index: ::core::ffi::c_int,
    pub has_audio: bool,
    pub last_frame_was_key: bool,
    pub pending_video_keyframe: *mut AVPacket,
    pub stream_name: [::core::ffi::c_char; 256],
    pub fps_measurement: stream_frame_rate_measurement_t,
}
pub type mp4_writer_t = mp4_writer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_placement_t {
    pub status: storage_placement_status_t,
    pub target_is_default: bool,
    pub target_uuid: [::core::ffi::c_char; 37],
    pub target_root: [::core::ffi::c_char; 512],
    pub object_key: [::core::ffi::c_char; 512],
    pub policy_uuid: [::core::ffi::c_char; 37],
    pub policy_version: int64_t,
    pub reason: [::core::ffi::c_char; 64],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct storage_placement_status_t(pub ::core::ffi::c_uint);
impl storage_placement_status_t {
    pub const STORAGE_PLACEMENT_READY: Self = Self(0);
    pub const STORAGE_PLACEMENT_PAUSED: Self = Self(1);
    pub const STORAGE_PLACEMENT_FAILED: Self = Self(2);
    pub const STORAGE_PLACEMENT_ERROR: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp4_audio_state_t {
    pub stream_idx: ::core::ffi::c_int,
    pub first_dts: int64_t,
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub initialized: ::core::ffi::c_int,
    pub time_base: AVRational,
    pub frame_size: ::core::ffi::c_int,
    pub mutex: pthread_mutex_t,
}
pub type record_segment_started_cb = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type record_segment_activity_cb = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct metrics_frame_source_t(pub ::core::ffi::c_uint);
impl metrics_frame_source_t {
    pub const METRICS_SOURCE_GENERIC: Self = Self(0);
    pub const METRICS_SOURCE_RECORDING: Self = Self(1);
    pub const METRICS_SOURCE_HLS: Self = Self(2);
    pub const METRICS_SOURCE_DETECTION: Self = Self(3);
    pub const METRICS_SOURCE_COUNT: Self = Self(4);
}
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AV_DICT_APPEND: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const AV_LOG_QUIET: ::core::ffi::c_int = -8 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn avio_tell(mut s: *mut AVIOContext) -> int64_t {
    return avio_seek(s, 0 as int64_t, SEEK_CUR);
}
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const AV_NOPTS_VALUE: int64_t = 0x8000000000000000 as ::core::ffi::c_ulong as int64_t;
pub const AVERROR_EOF: ::core::ffi::c_int = -((('E' as ::core::ffi::c_int
    | ('O' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
    | ('F' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
    as ::core::ffi::c_uint
    | (' ' as ::core::ffi::c_int as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
    as ::core::ffi::c_int);
pub const AVERROR_EXIT: ::core::ffi::c_int = -((('E' as ::core::ffi::c_int
    | ('X' as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
    | ('I' as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
    as ::core::ffi::c_uint
    | ('T' as ::core::ffi::c_int as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
    as ::core::ffi::c_int);
pub const AV_ERROR_MAX_STRING_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn av_q2d(mut a: AVRational) -> ::core::ffi::c_double {
    return a.num as ::core::ffi::c_double / a.den as ::core::ffi::c_double;
}
#[inline(always)]
unsafe extern "C" fn av_inv_q(mut q: AVRational) -> AVRational {
    let mut r: AVRational = AVRational {
        num: q.den,
        den: q.num,
    };
    return r;
}
pub const AV_PKT_FLAG_KEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MP4_DTS_MAX_VALUE: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
pub const MP4_DTS_WARNING_THRESHOLD: ::core::ffi::c_int = 0x70000000 as ::core::ffi::c_int;
pub const DTS_RESET_SAFE_VALUE: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const MAX_PACKET_DURATION_TIMEBASE_UNITS: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
pub const DIMENSION_PROBE_TIMEOUT_US: ::core::ffi::c_longlong = 60000000 as ::core::ffi::c_longlong;
pub const SHUTDOWN_KEYFRAME_WAIT_TIMEOUT_S: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KEYFRAME_WAIT_TIMEOUT_S: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const RECORDING_CACHE_RELEASE_INTERVAL_BYTES: ::core::ffi::c_longlong = 32
    as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong;
pub const TIMESTAMP_CONTINUITY_OFFSET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AUDIO_DTS_RESET_SAFE_VALUE: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
unsafe extern "C" fn calculate_frame_duration_from_stream(mut stream: *const AVStream) -> int64_t {
    if stream.is_null() {
        return 1 as int64_t;
    }
    if (*stream).avg_frame_rate.num > 0 as ::core::ffi::c_int
        && (*stream).avg_frame_rate.den > 0 as ::core::ffi::c_int
    {
        return av_rescale_q(
            1 as int64_t,
            av_inv_q((*stream).avg_frame_rate),
            (*stream).time_base,
        );
    }
    return 1 as int64_t;
}
unsafe extern "C" fn maybe_release_recording_file_cache(
    mut output_ctx: *mut AVFormatContext,
    mut cache_fd: *mut ::core::ffi::c_int,
    mut released_through: *mut off_t,
) {
    if output_ctx.is_null()
        || (*output_ctx).pb.is_null()
        || cache_fd.is_null()
        || *cache_fd < 0 as ::core::ffi::c_int
        || released_through.is_null()
    {
        return;
    }
    let mut position: int64_t = avio_tell((*output_ctx).pb);
    let mut current_position: off_t = position as off_t;
    if position < 0 as int64_t
        || current_position as int64_t != position
        || !file_cache_release_due(
            *released_through,
            current_position,
            RECORDING_CACHE_RELEASE_INTERVAL_BYTES as off_t,
        )
    {
        return;
    }
    avio_flush((*output_ctx).pb);
    let mut release_rc: ::core::ffi::c_int = file_cache_flush_and_release(
        *cache_fd,
        *released_through,
        current_position - *released_through,
    );
    if release_rc == 0 as ::core::ffi::c_int {
        *released_through = current_position;
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Disabling recording page-cache release after flush failed: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        strerror(release_rc),
    );
    close(*cache_fd);
    *cache_fd = -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn release_all_recording_file_cache(
    mut cache_fd: *mut ::core::ffi::c_int,
    mut output_file: *const ::core::ffi::c_char,
) {
    if cache_fd.is_null() || *cache_fd < 0 as ::core::ffi::c_int {
        return;
    }
    let mut release_rc: ::core::ffi::c_int =
        file_cache_flush_and_release(*cache_fd, 0 as off_t, 0 as off_t);
    if release_rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to release recording page cache for %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            if !output_file.is_null() {
                output_file
            } else {
                b"[unknown-file]\0".as_ptr() as *const ::core::ffi::c_char
            },
            strerror(release_rc),
        );
    }
    close(*cache_fd);
    *cache_fd = -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mp4_segment_rescale_packet_ts(
    mut pkt: *mut AVPacket,
    mut input_stream: *const AVStream,
    mut output_stream: *const AVStream,
) {
    if pkt.is_null() || input_stream.is_null() || output_stream.is_null() {
        return;
    }
    av_packet_rescale_ts(pkt, (*input_stream).time_base, (*output_stream).time_base);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mp4_segment_audio_error_requires_input_refresh(
    mut error: ::core::ffi::c_int,
) -> bool {
    return error == -22 as ::core::ffi::c_int;
}
unsafe extern "C" fn clamp_dts_pts_for_mp4(
    mut pkt: *mut AVPacket,
    mut reset_safe_val: int64_t,
    mut warning_thresh: int64_t,
    mut stream_label: *const ::core::ffi::c_char,
    mut last_dts: *mut int64_t,
    mut last_pts: *mut int64_t,
) {
    if (*pkt).dts == AV_NOPTS_VALUE {
        return;
    }
    if (*pkt).dts > MP4_DTS_MAX_VALUE as int64_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s DTS value exceeds MP4 format limit: %lld, resetting to safe value\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_label,
            (*pkt).dts as ::core::ffi::c_longlong,
        );
        let mut pts_dts_diff: int64_t = 0 as int64_t;
        if (*pkt).pts != AV_NOPTS_VALUE {
            pts_dts_diff = (*pkt).pts - (*pkt).dts;
        }
        (*pkt).dts = reset_safe_val;
        if (*pkt).pts != AV_NOPTS_VALUE {
            if pts_dts_diff >= 0 as int64_t && pts_dts_diff < 10000 as int64_t {
                (*pkt).pts = (*pkt).dts + pts_dts_diff;
            } else {
                (*pkt).pts = (*pkt).dts;
            }
        } else {
            (*pkt).pts = (*pkt).dts;
        }
        if !last_dts.is_null() {
            *last_dts = (*pkt).dts;
        }
        if !last_pts.is_null() {
            *last_pts = (*pkt).pts;
        }
    } else if (*pkt).dts > warning_thresh {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s DTS value approaching MP4 format limit: %lld, resetting to safe value\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_label,
            (*pkt).dts as ::core::ffi::c_longlong,
        );
        let mut pts_dts_diff_0: int64_t = 0 as int64_t;
        if (*pkt).pts != AV_NOPTS_VALUE {
            pts_dts_diff_0 = (*pkt).pts - (*pkt).dts;
        }
        (*pkt).dts = reset_safe_val;
        if (*pkt).pts != AV_NOPTS_VALUE {
            if pts_dts_diff_0 >= 0 as int64_t && pts_dts_diff_0 < 10000 as int64_t {
                (*pkt).pts = (*pkt).dts + pts_dts_diff_0;
            } else {
                (*pkt).pts = (*pkt).dts;
            }
        } else {
            (*pkt).pts = (*pkt).dts;
        }
        if !last_dts.is_null() {
            *last_dts = (*pkt).dts;
        }
        if !last_pts.is_null() {
            *last_pts = (*pkt).pts;
        }
    }
}
unsafe extern "C" fn interrupt_callback(mut ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    if is_shutdown_initiated() {
        return 1 as ::core::ffi::c_int;
    }
    if !ctx.is_null() {
        let mut shutdown_flag: *mut atomic_int = ctx as *mut atomic_int;
        if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            shutdown_flag,
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_terminal_storage_write_error(mut error: ::core::ffi::c_int) -> bool {
    return error == -28 as ::core::ffi::c_int
        || error == -5 as ::core::ffi::c_int
        || error == -30 as ::core::ffi::c_int;
}
unsafe extern "C" fn notify_recording_activity(
    mut activity_cb: record_segment_activity_cb,
    mut cb_ctx: *mut ::core::ffi::c_void,
) {
    if activity_cb.is_some() {
        activity_cb.expect("non-null function pointer")(cb_ctx);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mp4_segment_recorder_init() {
    avformat_network_init();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MP4 segment recorder initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn record_segment(
    mut rtsp_url: *const ::core::ffi::c_char,
    mut output_file: *const ::core::ffi::c_char,
    mut duration: ::core::ffi::c_int,
    mut has_audio: ::core::ffi::c_int,
    mut input_ctx_ptr: *mut *mut AVFormatContext,
    mut segment_info_ptr: *mut segment_info_t,
    mut started_cb: record_segment_started_cb,
    mut activity_cb: record_segment_activity_cb,
    mut cb_ctx: *mut ::core::ffi::c_void,
    mut shutdown_flag: *mut atomic_int,
) -> ::core::ffi::c_int {
    let mut consecutive_timestamp_errors: ::core::ffi::c_int = 0;
    let mut max_timestamp_errors: ::core::ffi::c_int = 0;
    let mut found_first_keyframe: bool = false;
    let mut waiting_for_final_keyframe: bool = false;
    let mut final_audio_error: ::core::ffi::c_int = 0;
    if g_config.audio_disabled {
        has_audio = 0 as ::core::ffi::c_int;
    }
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut input_ctx: *mut AVFormatContext = ::core::ptr::null_mut::<AVFormatContext>();
    let mut output_ctx: *mut AVFormatContext = ::core::ptr::null_mut::<AVFormatContext>();
    let mut opts: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
    let mut out_opts: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
    let mut pkt: *mut AVPacket = ::core::ptr::null_mut::<AVPacket>();
    let mut video_stream_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut audio_stream_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut needs_audio_transcoding: bool = r#false != 0;
    let mut reuse_input_context: bool = r#true != 0;
    let mut shutdown_detected: bool = r#false != 0;
    let mut recording_cache_fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut recording_cache_released_through: off_t = 0 as off_t;
    let mut out_video_stream: *mut AVStream = ::core::ptr::null_mut::<AVStream>();
    let mut out_audio_stream: *mut AVStream = ::core::ptr::null_mut::<AVStream>();
    let mut first_video_dts: int64_t = AV_NOPTS_VALUE;
    let mut first_video_pts: int64_t = AV_NOPTS_VALUE;
    let mut first_audio_dts: int64_t = AV_NOPTS_VALUE;
    let mut first_audio_pts: int64_t = AV_NOPTS_VALUE;
    let mut last_video_dts: int64_t = AV_NOPTS_VALUE;
    let mut last_video_pts: int64_t = AV_NOPTS_VALUE;
    let mut last_audio_dts: int64_t = AV_NOPTS_VALUE;
    let mut last_audio_pts: int64_t = AV_NOPTS_VALUE;
    let mut next_transcoded_audio_pts: int64_t = AV_NOPTS_VALUE;
    let mut audio_packet_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut video_packet_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut start_time: int64_t = 0 as int64_t;
    let mut segment_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut segment_end_dts: int64_t = AV_NOPTS_VALUE;
    let mut started_cb_called: bool = r#false != 0;
    let mut trailer_written: bool = r#false != 0;
    let mut waiting_start_time: int64_t = 0 as int64_t;
    if input_ctx_ptr.is_null() || segment_info_ptr.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters: input_ctx_ptr or segment_info_ptr is NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*input_ctx_ptr).is_null() && !(*segment_info_ptr).pending_video_keyframe.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Discarding pending keyframe because input context is not being reused (new connection)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        av_packet_free(&raw mut (*segment_info_ptr).pending_video_keyframe);
        (*segment_info_ptr).pending_video_keyframe = ::core::ptr::null_mut::<AVPacket>();
    }
    if (*input_ctx_ptr).is_null() {
        (*segment_info_ptr).fps_measurement = stream_frame_rate_measurement_t {
            started: false,
            warmup_until_us: 0,
            window_start_us: 0,
            packets: 0,
            fps: 0,
        };
        (*segment_info_ptr).segment_index = 0 as ::core::ffi::c_int;
        (*segment_info_ptr).last_frame_was_key = r#false != 0;
    }
    segment_index = (*segment_info_ptr).segment_index + 1 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting new segment with index %d\0".as_ptr() as *const ::core::ffi::c_char,
        segment_index,
    );
    let mut safe_rtsp_url: [::core::ffi::c_char; 512] = [0; 512];
    if url_redact_for_logging(
        rtsp_url,
        &raw mut safe_rtsp_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut safe_rtsp_url as *mut ::core::ffi::c_char,
            b"[invalid-url]\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Recording from %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut safe_rtsp_url as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Output file: %s\0".as_ptr() as *const ::core::ffi::c_char,
        output_file,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Duration: %d seconds\0".as_ptr() as *const ::core::ffi::c_char,
        duration,
    );
    '_cleanup: {
        if !(*input_ctx_ptr).is_null() {
            input_ctx = *input_ctx_ptr;
            *input_ctx_ptr = ::core::ptr::null_mut::<AVFormatContext>();
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Using existing input context\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*input_ctx).interrupt_callback.callback = Some(
                interrupt_callback
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            )
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
            (*input_ctx).interrupt_callback.opaque = shutdown_flag as *mut ::core::ffi::c_void;
        } else {
            input_ctx = avformat_alloc_context();
            if input_ctx.is_null() {
                recording_io_report_failure(
                    recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                    recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                    ENOMEM,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to allocate input context\0".as_ptr() as *const ::core::ffi::c_char,
                );
                ret = -1 as ::core::ffi::c_int;
                break '_cleanup;
            } else {
                (*input_ctx).interrupt_callback.callback = Some(
                    interrupt_callback
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
                )
                    as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
                (*input_ctx).interrupt_callback.opaque = shutdown_flag as *mut ::core::ffi::c_void;
                av_dict_set(
                    &raw mut opts,
                    b"rtsp_transport\0".as_ptr() as *const ::core::ffi::c_char,
                    b"tcp\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                av_dict_set(
                    &raw mut opts,
                    b"fflags\0".as_ptr() as *const ::core::ffi::c_char,
                    b"nobuffer\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                av_dict_set(
                    &raw mut opts,
                    b"fflags\0".as_ptr() as *const ::core::ffi::c_char,
                    b"+genpts\0".as_ptr() as *const ::core::ffi::c_char,
                    AV_DICT_APPEND,
                );
                av_dict_set(
                    &raw mut opts,
                    b"flags\0".as_ptr() as *const ::core::ffi::c_char,
                    b"low_delay\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                av_dict_set(
                    &raw mut opts,
                    b"max_delay\0".as_ptr() as *const ::core::ffi::c_char,
                    b"500000\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                av_dict_set(
                    &raw mut opts,
                    b"stimeout\0".as_ptr() as *const ::core::ffi::c_char,
                    b"5000000\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                av_dict_set(
                    &raw mut opts,
                    b"analyzeduration\0".as_ptr() as *const ::core::ffi::c_char,
                    b"5000000\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                av_dict_set(
                    &raw mut opts,
                    b"probesize\0".as_ptr() as *const ::core::ffi::c_char,
                    b"5242880\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Opening RTSP connection to %s (analyzeduration=5s, probesize=5MB)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut safe_rtsp_url as *mut ::core::ffi::c_char,
                );
                ret = avformat_open_input(
                    &raw mut input_ctx,
                    rtsp_url,
                    ::core::ptr::null::<AVInputFormat>(),
                    &raw mut opts,
                );
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
                    if ret == AVERROR_EXIT {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"RTSP open interrupted (AVERROR_EXIT) for %s \xE2\x80\x94 thread shutdown was requested during connection\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            &raw mut safe_rtsp_url as *mut ::core::ffi::c_char,
                        );
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to open RTSP input %s: %d (%s)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut safe_rtsp_url as *mut ::core::ffi::c_char,
                            ret,
                            &raw mut error_buf as *mut ::core::ffi::c_char,
                        );
                    }
                    if !input_ctx.is_null() {
                        avformat_free_context(input_ctx);
                        input_ctx = ::core::ptr::null_mut::<AVFormatContext>();
                    }
                    break '_cleanup;
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Probing stream info for %s ...\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut safe_rtsp_url as *mut ::core::ffi::c_char,
                    );
                    ret = avformat_find_stream_info(
                        input_ctx,
                        ::core::ptr::null_mut::<*mut AVDictionary>(),
                    );
                    if ret < 0 as ::core::ffi::c_int {
                        let mut err_buf: [::core::ffi::c_char; 64] = [
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
                            &raw mut err_buf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                        );
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to find stream info for %s: %d (%s)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut safe_rtsp_url as *mut ::core::ffi::c_char,
                            ret,
                            &raw mut err_buf as *mut ::core::ffi::c_char,
                        );
                        break '_cleanup;
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Stream info detected for %s: %d streams\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut safe_rtsp_url as *mut ::core::ffi::c_char,
                            (*input_ctx).nb_streams,
                        );
                    }
                }
            }
        }
        if input_ctx.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Input context is NULL, cannot proceed with recording\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            ret = -1 as ::core::ffi::c_int;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Input format: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*(*input_ctx).iformat).name,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Number of streams: %d\0".as_ptr() as *const ::core::ffi::c_char,
                (*input_ctx).nb_streams,
            );
            let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while i < (*input_ctx).nb_streams {
                let mut stream: *mut AVStream = *(*input_ctx).streams.offset(i as isize);
                if (*(*stream).codecpar).codec_type.0 == AVMediaType::AVMEDIA_TYPE_VIDEO.0
                    && video_stream_idx < 0 as ::core::ffi::c_int
                {
                    video_stream_idx = i as ::core::ffi::c_int;
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Found video stream: %d\0".as_ptr() as *const ::core::ffi::c_char,
                        i,
                    );
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"  Codec: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        avcodec_get_name((*(*stream).codecpar).codec_id),
                    );
                    if (*(*stream).codecpar).width == 0 as ::core::ffi::c_int
                        || (*(*stream).codecpar).height == 0 as ::core::ffi::c_int
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Video stream has unspecified dimensions (width=%d, height=%d) \xE2\x80\x94 will attempt decoder-based probing\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*(*stream).codecpar).width,
                            (*(*stream).codecpar).height,
                        );
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_DEBUG,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"  Resolution: %dx%d\0".as_ptr() as *const ::core::ffi::c_char,
                            (*(*stream).codecpar).width,
                            (*(*stream).codecpar).height,
                        );
                    }
                    if (*stream).avg_frame_rate.num != 0 && (*stream).avg_frame_rate.den != 0 {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_DEBUG,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"  Frame rate: %.2f fps\0".as_ptr() as *const ::core::ffi::c_char,
                            ((*stream).avg_frame_rate.num as ::core::ffi::c_float
                                / (*stream).avg_frame_rate.den as ::core::ffi::c_float)
                                as ::core::ffi::c_double,
                        );
                    }
                } else if (*(*stream).codecpar).codec_type.0 == AVMediaType::AVMEDIA_TYPE_AUDIO.0
                    && audio_stream_idx < 0 as ::core::ffi::c_int
                {
                    audio_stream_idx = i as ::core::ffi::c_int;
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Found audio stream: %d\0".as_ptr() as *const ::core::ffi::c_char,
                        i,
                    );
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"  Codec: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        avcodec_get_name((*(*stream).codecpar).codec_id),
                    );
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"  Sample rate: %d Hz\0".as_ptr() as *const ::core::ffi::c_char,
                        (*(*stream).codecpar).sample_rate,
                    );
                }
                i = i.wrapping_add(1);
            }
            if video_stream_idx < 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"No video stream found\0".as_ptr() as *const ::core::ffi::c_char,
                );
                ret = -1 as ::core::ffi::c_int;
            } else {
                let mut vstream: *mut AVStream =
                    *(*input_ctx).streams.offset(video_stream_idx as isize);
                if (*(*vstream).codecpar).width == 0 as ::core::ffi::c_int
                    || (*(*vstream).codecpar).height == 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Video dimensions 0x0 after stream probe \xE2\x80\x94 attempting decoder-based dimension detection from bitstream (up to 60s)...\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    let mut probe_decoder: *const AVCodec =
                        avcodec_find_decoder((*(*vstream).codecpar).codec_id);
                    if !probe_decoder.is_null() {
                        let mut probe_ctx: *mut AVCodecContext =
                            avcodec_alloc_context3(probe_decoder);
                        if !probe_ctx.is_null() {
                            avcodec_parameters_to_context(probe_ctx, (*vstream).codecpar);
                            if avcodec_open2(
                                probe_ctx,
                                probe_decoder,
                                ::core::ptr::null_mut::<*mut AVDictionary>(),
                            ) >= 0 as ::core::ffi::c_int
                            {
                                let mut probe_pkt: *mut AVPacket = av_packet_alloc();
                                let mut probe_frame: *mut AVFrame = av_frame_alloc();
                                if probe_pkt.is_null() || probe_frame.is_null() {
                                    recording_io_report_failure(
                                        recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                        recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                                        ENOMEM,
                                    );
                                }
                                if !probe_pkt.is_null() && !probe_frame.is_null() {
                                    let mut probe_start: int64_t = av_gettime();
                                    let mut probe_timeout: int64_t =
                                        DIMENSION_PROBE_TIMEOUT_US as int64_t;
                                    let mut dimensions_found: bool = r#false != 0;
                                    let mut probe_packets: ::core::ffi::c_int =
                                        0 as ::core::ffi::c_int;
                                    let mut probe_other_packets: ::core::ffi::c_int =
                                        0 as ::core::ffi::c_int;
                                    let mut audio_only_warned: bool = r#false != 0;
                                    let mut last_progress_log: int64_t = probe_start;
                                    while !dimensions_found
                                        && av_gettime() - probe_start < probe_timeout
                                    {
                                        if interrupt_callback(
                                            shutdown_flag as *mut ::core::ffi::c_void,
                                        ) != 0
                                        {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_INFO,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Dimension probe interrupted by shutdown\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            break;
                                        } else {
                                            let mut now: int64_t = av_gettime();
                                            let mut elapsed_us: int64_t = now - probe_start;
                                            if !audio_only_warned
                                                && elapsed_us >= 10000000 as int64_t
                                                && probe_packets == 0 as ::core::ffi::c_int
                                                && probe_other_packets > 0 as ::core::ffi::c_int
                                            {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_WARN,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Dimension probe: %d audio packets received but 0 video packets after 10s \xE2\x80\x94 go2rtc is likely waiting for a keyframe from the upstream camera. Continuing to probe on this connection (up to 60s total)...\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    probe_other_packets,
                                                );
                                                audio_only_warned = r#true != 0;
                                            }
                                            if now - last_progress_log >= 5000000 as int64_t {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Dimension probe: %d video pkts, %d other pkts, %.0fs elapsed\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    probe_packets,
                                                    probe_other_packets,
                                                    elapsed_us as ::core::ffi::c_double / 1000000.0f64,
                                                );
                                                last_progress_log = now;
                                            }
                                            let mut probe_ret: ::core::ffi::c_int =
                                                av_read_frame(input_ctx, probe_pkt);
                                            if probe_ret < 0 as ::core::ffi::c_int {
                                                if probe_ret == -11 as ::core::ffi::c_int {
                                                    notify_recording_activity(activity_cb, cb_ctx);
                                                    av_usleep(10000 as ::core::ffi::c_uint);
                                                } else {
                                                    let mut err_buf_0: [::core::ffi::c_char; 64] = [
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
                                                        probe_ret,
                                                        &raw mut err_buf_0
                                                            as *mut ::core::ffi::c_char,
                                                        ::core::mem::size_of::<
                                                            [::core::ffi::c_char; 64],
                                                        >(
                                                        ),
                                                    );
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_WARN,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Error reading packet during dimension probe: %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        &raw mut err_buf_0 as *mut ::core::ffi::c_char,
                                                    );
                                                    break;
                                                }
                                            } else {
                                                notify_recording_activity(activity_cb, cb_ctx);
                                                if (*probe_pkt).stream_index == video_stream_idx {
                                                    probe_packets += 1;
                                                    let mut send_ret: ::core::ffi::c_int =
                                                        avcodec_send_packet(probe_ctx, probe_pkt);
                                                    if send_ret >= 0 as ::core::ffi::c_int {
                                                        if (*probe_ctx).width
                                                            > 0 as ::core::ffi::c_int
                                                            && (*probe_ctx).height
                                                                > 0 as ::core::ffi::c_int
                                                        {
                                                            dimensions_found = r#true != 0;
                                                        } else {
                                                            let mut recv_ret: ::core::ffi::c_int =
                                                                avcodec_receive_frame(
                                                                    probe_ctx,
                                                                    probe_frame,
                                                                );
                                                            if recv_ret >= 0 as ::core::ffi::c_int {
                                                                if (*probe_ctx).width
                                                                    > 0 as ::core::ffi::c_int
                                                                    && (*probe_ctx).height
                                                                        > 0 as ::core::ffi::c_int
                                                                {
                                                                    dimensions_found = r#true != 0;
                                                                }
                                                                av_frame_unref(probe_frame);
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    probe_other_packets += 1;
                                                }
                                                av_packet_unref(probe_pkt);
                                            }
                                        }
                                    }
                                    if dimensions_found {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Probed video dimensions from bitstream: %dx%d (after %d video packets, %.1fs)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            (*probe_ctx).width,
                                            (*probe_ctx).height,
                                            probe_packets,
                                            (av_gettime() - probe_start) as ::core::ffi::c_double
                                                / 1000000.0f64,
                                        );
                                        (*(*vstream).codecpar).width = (*probe_ctx).width;
                                        (*(*vstream).codecpar).height = (*probe_ctx).height;
                                    } else {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_WARN,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Failed to probe video dimensions after %d video packets (%d other-stream), %.1fs\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            probe_packets,
                                            probe_other_packets,
                                            (av_gettime() - probe_start) as ::core::ffi::c_double
                                                / 1000000.0f64,
                                        );
                                    }
                                    av_frame_free(&raw mut probe_frame);
                                }
                                if !probe_pkt.is_null() {
                                    av_packet_free(&raw mut probe_pkt);
                                }
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Failed to open probe decoder for dimension detection\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            avcodec_free_context(&raw mut probe_ctx);
                        }
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"No decoder found for codec %s, cannot probe dimensions\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            avcodec_get_name((*(*vstream).codecpar).codec_id),
                        );
                    }
                }
                ret = avformat_alloc_output_context2(
                    &raw mut output_ctx,
                    ::core::ptr::null::<AVOutputFormat>(),
                    b"mp4\0".as_ptr() as *const ::core::ffi::c_char,
                    output_file,
                );
                if ret < 0 as ::core::ffi::c_int || output_ctx.is_null() {
                    recording_io_report_failure(
                        recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                        recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                        if ret < 0 as ::core::ffi::c_int {
                            ret
                        } else {
                            ENOMEM
                        },
                    );
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to create output context: %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ret,
                    );
                } else {
                    out_video_stream =
                        avformat_new_stream(output_ctx, ::core::ptr::null::<AVCodec>());
                    if out_video_stream.is_null() {
                        recording_io_report_failure(
                            recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                            recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                            ENOMEM,
                        );
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to create output video stream\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        ret = -1 as ::core::ffi::c_int;
                    } else {
                        ret = avcodec_parameters_copy(
                            (*out_video_stream).codecpar,
                            (**(*input_ctx).streams.offset(video_stream_idx as isize)).codecpar,
                        );
                        if ret < 0 as ::core::ffi::c_int {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Failed to copy video codec parameters: %d\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                ret,
                            );
                        } else {
                            (*(*out_video_stream).codecpar).codec_tag = 0 as uint32_t;
                            if (*(*out_video_stream).codecpar).extradata.is_null()
                                || (*(*out_video_stream).codecpar).extradata_size
                                    <= 0 as ::core::ffi::c_int
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Video stream has no extradata (SPS/PPS headers) \xE2\x80\x94 MP4 may be unplayable. This can happen when go2rtc has not yet received a keyframe from the camera.\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                );
                            }
                            if (*(*out_video_stream).codecpar).width == 0 as ::core::ffi::c_int
                                || (*(*out_video_stream).codecpar).height == 0 as ::core::ffi::c_int
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Video dimensions not set (width=%d, height=%d) \xE2\x80\x94 stream source not ready, closing connection and returning error so caller can retry with a fresh connection\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*(*out_video_stream).codecpar).width,
                                    (*(*out_video_stream).codecpar).height,
                                );
                                ret = -1 as ::core::ffi::c_int;
                            } else {
                                (*out_video_stream).time_base =
                                    (**(*input_ctx).streams.offset(video_stream_idx as isize))
                                        .time_base;
                                if audio_stream_idx >= 0 as ::core::ffi::c_int && has_audio != 0 {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_INFO,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Including audio stream in MP4 recording\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                    let mut codec_name: *const ::core::ffi::c_char =
                                        b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
                                    let mut is_compatible: bool =
                                        is_audio_codec_compatible_with_mp4(
                                            (*(**(*input_ctx)
                                                .streams
                                                .offset(audio_stream_idx as isize))
                                            .codecpar)
                                                .codec_id,
                                            &raw mut codec_name,
                                        );
                                    if !is_compatible {
                                        if is_pcm_codec(
                                            (*(**(*input_ctx)
                                                .streams
                                                .offset(audio_stream_idx as isize))
                                            .codecpar)
                                                .codec_id,
                                        ) {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_INFO,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Attempting to transcode %s audio to AAC for MP4 compatibility\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                codec_name,
                                            );
                                            let mut transcoded_params: *mut AVCodecParameters =
                                                ::core::ptr::null_mut::<AVCodecParameters>();
                                            let mut audio_tb: AVRational = (**(*input_ctx)
                                                .streams
                                                .offset(audio_stream_idx as isize))
                                            .time_base;
                                            let mut transcode_ret: ::core::ffi::c_int =
                                                transcode_pcm_to_aac(
                                                    (**(*input_ctx)
                                                        .streams
                                                        .offset(audio_stream_idx as isize))
                                                    .codecpar,
                                                    &raw mut audio_tb,
                                                    rtsp_url,
                                                    &raw mut transcoded_params,
                                                );
                                            if transcode_ret >= 0 as ::core::ffi::c_int
                                                && !transcoded_params.is_null()
                                            {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Successfully set up PCM-to-AAC transcoding for MP4 recording\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                );
                                                needs_audio_transcoding = r#true != 0;
                                                out_audio_stream = avformat_new_stream(
                                                    output_ctx,
                                                    ::core::ptr::null::<AVCodec>(),
                                                );
                                                if out_audio_stream.is_null() {
                                                    recording_io_report_failure(
                                                        recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                                        recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                                                        ENOMEM,
                                                    );
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_ERROR,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Failed to create output audio stream\0"
                                                            .as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    avcodec_parameters_free(
                                                        &raw mut transcoded_params,
                                                    );
                                                    ret = -1 as ::core::ffi::c_int;
                                                    break '_cleanup;
                                                } else {
                                                    ret = avcodec_parameters_copy(
                                                        (*out_audio_stream).codecpar,
                                                        transcoded_params,
                                                    );
                                                    avcodec_parameters_free(
                                                        &raw mut transcoded_params,
                                                    );
                                                    if ret < 0 as ::core::ffi::c_int {
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_ERROR,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Failed to copy transcoded audio codec parameters: %d\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                            ret,
                                                        );
                                                        break '_cleanup;
                                                    } else {
                                                        (*(*out_audio_stream).codecpar).codec_tag =
                                                            0 as uint32_t;
                                                        (*out_audio_stream).time_base =
                                                            if (*(*out_audio_stream).codecpar)
                                                                .sample_rate
                                                                > 0 as ::core::ffi::c_int
                                                            {
                                                                AVRational {
                                                                    num: 1 as ::core::ffi::c_int,
                                                                    den: (*(*out_audio_stream)
                                                                        .codecpar)
                                                                        .sample_rate,
                                                                }
                                                            } else {
                                                                (**(*input_ctx).streams.offset(
                                                                    audio_stream_idx as isize,
                                                                ))
                                                                .time_base
                                                            };
                                                    }
                                                }
                                            } else {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_ERROR,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Failed to transcode %s audio to AAC: %d \xE2\x80\x94 disabling audio\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    codec_name,
                                                    transcode_ret,
                                                );
                                                has_audio = 0 as ::core::ffi::c_int;
                                            }
                                        } else {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_WARN,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Audio codec %s is not compatible with MP4 and is not a PCM codec \xE2\x80\x94 disabling audio\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                codec_name,
                                            );
                                            has_audio = 0 as ::core::ffi::c_int;
                                        }
                                    } else {
                                        out_audio_stream = avformat_new_stream(
                                            output_ctx,
                                            ::core::ptr::null::<AVCodec>(),
                                        );
                                        if out_audio_stream.is_null() {
                                            recording_io_report_failure(
                                                recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                                recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                                                ENOMEM,
                                            );
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_ERROR,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Failed to create output audio stream\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            ret = -1 as ::core::ffi::c_int;
                                            break '_cleanup;
                                        } else {
                                            ret = avcodec_parameters_copy(
                                                (*out_audio_stream).codecpar,
                                                (**(*input_ctx)
                                                    .streams
                                                    .offset(audio_stream_idx as isize))
                                                .codecpar,
                                            );
                                            if ret < 0 as ::core::ffi::c_int {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_ERROR,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Failed to copy audio codec parameters: %d\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    ret,
                                                );
                                                break '_cleanup;
                                            } else {
                                                (*(*out_audio_stream).codecpar).codec_tag =
                                                    0 as uint32_t;
                                                (*out_audio_stream).time_base = (**(*input_ctx)
                                                    .streams
                                                    .offset(audio_stream_idx as isize))
                                                .time_base;
                                            }
                                        }
                                    }
                                }
                                if config_get_mp4_faststart(&raw mut g_config) {
                                    av_dict_set(
                                        &raw mut out_opts,
                                        b"movflags\0".as_ptr() as *const ::core::ffi::c_char,
                                        b"+faststart\0".as_ptr() as *const ::core::ffi::c_char,
                                        0 as ::core::ffi::c_int,
                                    );
                                }
                                if output_file.is_null()
                                    || *output_file.offset(0isize) as ::core::ffi::c_int
                                        == '\0' as ::core::ffi::c_int
                                {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_ERROR,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Invalid output file path (NULL or empty)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                    ret = -22 as ::core::ffi::c_int;
                                } else if output_ctx.is_null() {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_ERROR,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Output context is NULL, cannot open output file\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                    );
                                    ret = -22 as ::core::ffi::c_int;
                                } else {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_DEBUG,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Attempting to open output file: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        output_file,
                                    );
                                    ret =
                                        recording_file_open(&raw mut (*output_ctx).pb, output_file);
                                    if ret < 0 as ::core::ffi::c_int {
                                        recording_io_report_failure(
                                            recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
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
                                            b"Failed to open output file: %d (%s)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            ret,
                                            &raw mut error_buf_0 as *mut ::core::ffi::c_char,
                                        );
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_ERROR,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Output file path: %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            output_file,
                                        );
                                        let mut dir_path: *mut ::core::ffi::c_char =
                                            strdup(output_file);
                                        if !dir_path.is_null() {
                                            let mut dir: *mut ::core::ffi::c_char =
                                                dirname(dir_path);
                                            let mut dir_st: stat = stat {
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
                                            if stat(dir, &raw mut dir_st) != 0 as ::core::ffi::c_int
                                            {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_ERROR,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Directory does not exist: %s\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    dir_path,
                                                );
                                            } else if !(dir_st.st_mode & __S_IFMT as __mode_t
                                                == 0o40000 as __mode_t)
                                            {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_ERROR,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Path exists but is not a directory: %s\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    dir_path,
                                                );
                                            } else if access(dir_path, W_OK)
                                                != 0 as ::core::ffi::c_int
                                            {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_ERROR,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Directory is not writable: %s\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    dir_path,
                                                );
                                            }
                                            free(dir_path as *mut ::core::ffi::c_void);
                                        }
                                    } else {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_DEBUG,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Successfully opened output file: %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            output_file,
                                        );
                                        recording_cache_fd = open(output_file, O_RDWR | O_CLOEXEC);
                                        if recording_cache_fd < 0 as ::core::ffi::c_int {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_DEBUG,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Recording page-cache control unavailable for %s: %s\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                output_file,
                                                strerror(*__errno_location()),
                                            );
                                        }
                                        if (*(*out_video_stream).codecpar).width
                                            == 0 as ::core::ffi::c_int
                                            || (*(*out_video_stream).codecpar).height
                                                == 0 as ::core::ffi::c_int
                                        {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_ERROR,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Video dimensions still 0x0 before header write \xE2\x80\x94 aborting segment\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                            );
                                            ret = -1 as ::core::ffi::c_int;
                                        } else {
                                            ret = avformat_write_header(
                                                output_ctx,
                                                &raw mut out_opts,
                                            );
                                            if ret < 0 as ::core::ffi::c_int {
                                                recording_io_report_failure(
                                                    recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                                    recording_io_operation_t::RECORDING_IO_OPERATION_HEADER,
                                                    ret,
                                                );
                                                let mut error_buf_1: [::core::ffi::c_char; 64] = [
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
                                                    &raw mut error_buf_1
                                                        as *mut ::core::ffi::c_char,
                                                    AV_ERROR_MAX_STRING_SIZE as size_t,
                                                );
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_ERROR,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Failed to write header: %d (%s)\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    ret,
                                                    &raw mut error_buf_1
                                                        as *mut ::core::ffi::c_char,
                                                );
                                                if ret == -22 as ::core::ffi::c_int {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_ERROR,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Header write failed with EINVAL, likely due to invalid video parameters\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                    );
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_ERROR,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Video stream parameters: width=%d, height=%d, codec_id=%d\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        (*(*out_video_stream).codecpar).width,
                                                        (*(*out_video_stream).codecpar).height,
                                                        (*(*out_video_stream).codecpar).codec_id.0,
                                                    );
                                                    if !out_audio_stream.is_null() {
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_ERROR,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Audio stream parameters: codec_id=%d, sample_rate=%d, channels=%d\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                            (*(*out_audio_stream).codecpar).codec_id.0,
                                                            (*(*out_audio_stream).codecpar).sample_rate,
                                                            (*(*out_audio_stream).codecpar).ch_layout.nb_channels,
                                                        );
                                                        if (*(*out_audio_stream).codecpar)
                                                            .codec_id
                                                            .0
                                                            == AVCodecID::AV_CODEC_ID_PCM_MULAW.0
                                                        {
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"PCM \xCE\xBC-law (G.711 \xCE\xBC-law) audio codec is not compatible with MP4 format\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"Audio transcoding to AAC should be enabled automatically\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"If the issue persists, try disabling audio recording for this stream\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                        } else if (*(*out_audio_stream).codecpar)
                                                            .codec_id
                                                            .0
                                                            == AVCodecID::AV_CODEC_ID_PCM_ALAW.0
                                                        {
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"PCM A-law (G.711 A-law) audio codec is not compatible with MP4 format\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"Audio transcoding to AAC should be enabled automatically\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"If the issue persists, try disabling audio recording for this stream\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                        } else if (*(*out_audio_stream).codecpar)
                                                            .codec_id
                                                            .0
                                                            == AVCodecID::AV_CODEC_ID_PCM_S16LE.0
                                                        {
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"PCM signed 16-bit little-endian audio codec is not compatible with MP4 format\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"Audio transcoding to AAC should be enabled automatically\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"If the issue persists, try disabling audio recording for this stream\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                        } else if (*(*out_audio_stream).codecpar)
                                                            .codec_id
                                                            .0
                                                            == AVCodecID::AV_CODEC_ID_PCM_S16BE.0
                                                        {
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"PCM signed 16-bit big-endian audio codec is not compatible with MP4 format\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"Audio transcoding to AAC should be enabled automatically\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"If the issue persists, try disabling audio recording for this stream\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                        } else if (*(*out_audio_stream).codecpar)
                                                            .codec_id
                                                            .0
                                                            >= AVCodecID::AV_CODEC_ID_PCM_S16LE.0
                                                            && (*(*out_audio_stream).codecpar)
                                                                .codec_id
                                                                .0
                                                                <= AVCodecID::AV_CODEC_ID_PCM_LXF.0
                                                        {
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"PCM audio codec (codec_id=%d) is not compatible with MP4 format\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                (*(*out_audio_stream).codecpar).codec_id.0,
                                                            );
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"Audio transcoding to AAC should be enabled automatically\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"If the issue persists, try disabling audio recording for this stream\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Video packet time base: input=%d/%d, muxer=%d/%d\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    (**(*input_ctx).streams.offset(video_stream_idx as isize))
                                                        .time_base
                                                        .num,
                                                    (**(*input_ctx).streams.offset(video_stream_idx as isize))
                                                        .time_base
                                                        .den,
                                                    (*out_video_stream).time_base.num,
                                                    (*out_video_stream).time_base.den,
                                                );
                                                if !out_audio_stream.is_null()
                                                    && audio_stream_idx >= 0 as ::core::ffi::c_int
                                                {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_DEBUG,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Audio packet time base: input=%d/%d, muxer=%d/%d\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        (**(*input_ctx).streams.offset(audio_stream_idx as isize))
                                                            .time_base
                                                            .num,
                                                        (**(*input_ctx).streams.offset(audio_stream_idx as isize))
                                                            .time_base
                                                            .den,
                                                        (*out_audio_stream).time_base.num,
                                                        (*out_audio_stream).time_base.den,
                                                    );
                                                }
                                                pkt = av_packet_alloc() as *mut AVPacket;
                                                if pkt.is_null() {
                                                    recording_io_report_failure(
                                                        recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                                        recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                                                        ENOMEM,
                                                    );
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_ERROR,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Failed to allocate packet\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    ret = -12 as ::core::ffi::c_int;
                                                } else {
                                                    (*pkt).data =
                                                        ::core::ptr::null_mut::<uint8_t>();
                                                    (*pkt).size = 0 as ::core::ffi::c_int;
                                                    (*pkt).stream_index = -1 as ::core::ffi::c_int;
                                                    start_time = av_gettime();
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_INFO,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Recording started...\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    consecutive_timestamp_errors =
                                                        0 as ::core::ffi::c_int;
                                                    max_timestamp_errors = 5 as ::core::ffi::c_int;
                                                    found_first_keyframe = r#false != 0;
                                                    waiting_for_final_keyframe = r#false != 0;
                                                    if input_ctx.is_null() {
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_ERROR,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Input context is NULL before main recording loop, cannot proceed\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                        );
                                                        ret = -1 as ::core::ffi::c_int;
                                                    } else {
                                                        loop {
                                                            if !shutdown_detected
                                                                && !waiting_for_final_keyframe
                                                                && is_shutdown_initiated()
                                                                    as ::core::ffi::c_int
                                                                    != 0
                                                            {
                                                                _log_message_ctx(
                                                                    log_level_t::LOG_LEVEL_INFO,
                                                                    LOG_COMPONENT,
                                                                    _log_stream_name,
                                                                    b"Shutdown initiated, waiting for next key frame to end recording\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                );
                                                                waiting_for_final_keyframe =
                                                                    r#true != 0;
                                                                shutdown_detected = r#true != 0;
                                                            }
                                                            if duration > 0 as ::core::ffi::c_int
                                                                && !waiting_for_final_keyframe
                                                                && !shutdown_detected
                                                            {
                                                                let mut elapsed_seconds: int64_t =
                                                                    (av_gettime() - start_time)
                                                                        / 1000000 as int64_t;
                                                                if elapsed_seconds
                                                                    >= duration as int64_t
                                                                {
                                                                    _log_message_ctx(
                                                                        log_level_t::LOG_LEVEL_INFO,
                                                                        LOG_COMPONENT,
                                                                        _log_stream_name,
                                                                        b"Wall-clock duration limit of %d seconds reached, waiting for next key frame\0"
                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                        duration,
                                                                    );
                                                                    waiting_for_final_keyframe =
                                                                        r#true != 0;
                                                                }
                                                            }
                                                            let mut carried_keyframe: bool =
                                                                !(*segment_info_ptr)
                                                                    .pending_video_keyframe
                                                                    .is_null()
                                                                    && (*(*segment_info_ptr)
                                                                        .pending_video_keyframe)
                                                                        .size
                                                                        > 0 as ::core::ffi::c_int;
                                                            if !(*segment_info_ptr)
                                                                .pending_video_keyframe
                                                                .is_null()
                                                            {
                                                                if (*(*segment_info_ptr)
                                                                    .pending_video_keyframe)
                                                                    .size
                                                                    > 0 as ::core::ffi::c_int
                                                                {
                                                                    _log_message_ctx(
                                                                        log_level_t::LOG_LEVEL_DEBUG,
                                                                        LOG_COMPONENT,
                                                                        _log_stream_name,
                                                                        b"Using carried-over keyframe packet to start segment immediately (overlap mode)\0"
                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                    );
                                                                    av_packet_unref(pkt);
                                                                    av_packet_move_ref(
                                                                        pkt,
                                                                        (*segment_info_ptr)
                                                                            .pending_video_keyframe,
                                                                    );
                                                                    av_packet_free(
                                                                        &raw mut (*segment_info_ptr).pending_video_keyframe,
                                                                    );
                                                                    ret = 0 as ::core::ffi::c_int;
                                                                } else {
                                                                    av_packet_free(
                                                                        &raw mut (*segment_info_ptr).pending_video_keyframe,
                                                                    );
                                                                    (*segment_info_ptr)
                                                                        .pending_video_keyframe =
                                                                        ::core::ptr::null_mut::<
                                                                            AVPacket,
                                                                        >(
                                                                        );
                                                                    ret = av_read_frame(
                                                                        input_ctx, pkt,
                                                                    );
                                                                }
                                                            } else {
                                                                ret = av_read_frame(input_ctx, pkt);
                                                            }
                                                            if ret < 0 as ::core::ffi::c_int {
                                                                if ret == AVERROR_EOF {
                                                                    reuse_input_context =
                                                                        r#false != 0;
                                                                    _log_message_ctx(
                                                                        log_level_t::LOG_LEVEL_INFO,
                                                                        LOG_COMPONENT,
                                                                        _log_stream_name,
                                                                        b"End of stream reached for %s\0".as_ptr()
                                                                            as *const ::core::ffi::c_char,
                                                                        output_file,
                                                                    );
                                                                    break;
                                                                } else if ret == AVERROR_EXIT {
                                                                    shutdown_detected = r#true != 0;
                                                                    reuse_input_context =
                                                                        r#false != 0;
                                                                    _log_message_ctx(
                                                                        log_level_t::LOG_LEVEL_WARN,
                                                                        LOG_COMPONENT,
                                                                        _log_stream_name,
                                                                        b"RTSP read interrupted (AVERROR_EXIT) for %s \xE2\x80\x94 recording thread is being stopped\0"
                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                        output_file,
                                                                    );
                                                                    break;
                                                                } else if ret
                                                                    != -11 as ::core::ffi::c_int
                                                                {
                                                                    reuse_input_context =
                                                                        r#false != 0;
                                                                    let mut err_buf_1: [::core::ffi::c_char; 64] = [
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
                                                                        &raw mut err_buf_1 as *mut ::core::ffi::c_char,
                                                                        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                                                                    );
                                                                    _log_message_ctx(
                                                                        log_level_t::LOG_LEVEL_ERROR,
                                                                        LOG_COMPONENT,
                                                                        _log_stream_name,
                                                                        b"Error reading frame for %s: %d (%s)\0".as_ptr()
                                                                            as *const ::core::ffi::c_char,
                                                                        output_file,
                                                                        ret,
                                                                        &raw mut err_buf_1 as *mut ::core::ffi::c_char,
                                                                    );
                                                                    break;
                                                                } else {
                                                                    av_usleep(
                                                                        10000
                                                                            as ::core::ffi::c_uint,
                                                                    );
                                                                }
                                                            } else if !carried_keyframe
                                                                && interrupt_callback(
                                                                    shutdown_flag
                                                                        as *mut ::core::ffi::c_void,
                                                                ) != 0
                                                            {
                                                                av_packet_unref(pkt);
                                                                shutdown_detected = r#true != 0;
                                                                reuse_input_context = r#false != 0;
                                                                break;
                                                            } else {
                                                                notify_recording_activity(
                                                                    activity_cb,
                                                                    cb_ctx,
                                                                );
                                                                if (*pkt).stream_index
                                                                    == video_stream_idx
                                                                {
                                                                    if !carried_keyframe
                                                                        && (*pkt).size > 0 as ::core::ffi::c_int
                                                                    {
                                                                        stream_frame_rate_observe(
                                                                            &raw mut (*segment_info_ptr).fps_measurement,
                                                                            av_gettime_relative(),
                                                                        );
                                                                    }
                                                                    if (*segment_info_ptr)
                                                                        .stream_name[0usize]
                                                                        as ::core::ffi::c_int
                                                                        != '\0'
                                                                            as ::core::ffi::c_int
                                                                    {
                                                                        metrics_record_frame_from_source(
                                                                            &raw mut (*segment_info_ptr).stream_name
                                                                                as *mut ::core::ffi::c_char,
                                                                            (*pkt).size,
                                                                            r#true != 0,
                                                                            metrics_frame_source_t::METRICS_SOURCE_RECORDING,
                                                                        );
                                                                    }
                                                                    let mut is_keyframe: bool = (*pkt).flags & AV_PKT_FLAG_KEY
                                                                        != 0 as ::core::ffi::c_int;
                                                                    if !found_first_keyframe {
                                                                        if is_keyframe {
                                                                            found_first_keyframe =
                                                                                r#true != 0;
                                                                            if (*segment_info_ptr).last_frame_was_key
                                                                                as ::core::ffi::c_int != 0
                                                                                && segment_index > 0 as ::core::ffi::c_int
                                                                            {
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_INFO,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Previous segment ended with a key frame \xE2\x80\x94 starting new segment with overlap keyframe\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                );
                                                                            }
                                                                            _log_message_ctx(
                                                                                log_level_t::LOG_LEVEL_INFO,
                                                                                LOG_COMPONENT,
                                                                                _log_stream_name,
                                                                                b"Found first key frame, starting recording\0".as_ptr()
                                                                                    as *const ::core::ffi::c_char,
                                                                            );
                                                                            if !started_cb_called
                                                                                && started_cb
                                                                                    .is_some()
                                                                            {
                                                                                started_cb.expect("non-null function pointer")(cb_ctx);
                                                                                started_cb_called =
                                                                                    r#true != 0;
                                                                            }
                                                                            start_time =
                                                                                av_gettime();
                                                                        } else {
                                                                            av_packet_unref(pkt);
                                                                            continue;
                                                                        }
                                                                    }
                                                                    if first_video_dts
                                                                        == AV_NOPTS_VALUE
                                                                        && (*pkt).dts
                                                                            != AV_NOPTS_VALUE
                                                                    {
                                                                        first_video_dts =
                                                                            (*pkt).dts;
                                                                        first_video_pts = if (*pkt)
                                                                            .pts
                                                                            != AV_NOPTS_VALUE
                                                                        {
                                                                            (*pkt).pts
                                                                        } else {
                                                                            (*pkt).dts
                                                                        };
                                                                        _log_message_ctx(
                                                                            log_level_t::LOG_LEVEL_DEBUG,
                                                                            LOG_COMPONENT,
                                                                            _log_stream_name,
                                                                            b"First video DTS: %lld, PTS: %lld\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            first_video_dts as ::core::ffi::c_longlong,
                                                                            first_video_pts as ::core::ffi::c_longlong,
                                                                        );
                                                                        if duration > 0 as ::core::ffi::c_int
                                                                            && video_stream_idx >= 0 as ::core::ffi::c_int
                                                                        {
                                                                            let mut vs: *mut AVStream = *(*input_ctx)
                                                                                .streams
                                                                                .offset(video_stream_idx as isize);
                                                                            segment_end_dts = first_video_dts
                                                                                + av_rescale_q(
                                                                                    duration as int64_t,
                                                                                    AVRational {
                                                                                        num: 1 as ::core::ffi::c_int,
                                                                                        den: 1 as ::core::ffi::c_int,
                                                                                    },
                                                                                    (*vs).time_base,
                                                                                );
                                                                            _log_message_ctx(
                                                                                log_level_t::LOG_LEVEL_DEBUG,
                                                                                LOG_COMPONENT,
                                                                                _log_stream_name,
                                                                                b"Segment end DTS threshold: %lld (%ds in timebase {%d/%d})\0"
                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                segment_end_dts as ::core::ffi::c_longlong,
                                                                                duration,
                                                                                (*vs).time_base.num,
                                                                                (*vs).time_base.den,
                                                                            );
                                                                        }
                                                                    }
                                                                    if !waiting_for_final_keyframe
                                                                        && !shutdown_detected
                                                                        && segment_end_dts
                                                                            != AV_NOPTS_VALUE
                                                                        && (*pkt).dts
                                                                            != AV_NOPTS_VALUE
                                                                        && (*pkt).dts
                                                                            >= segment_end_dts
                                                                    {
                                                                        _log_message_ctx(
                                                                            log_level_t::LOG_LEVEL_INFO,
                                                                            LOG_COMPONENT,
                                                                            _log_stream_name,
                                                                            b"DTS segment boundary reached (pkt->dts %lld >= threshold %lld), checking current frame for segment close\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            (*pkt).dts as ::core::ffi::c_longlong,
                                                                            segment_end_dts as ::core::ffi::c_longlong,
                                                                        );
                                                                        waiting_for_final_keyframe = r#true != 0;
                                                                    }
                                                                    if waiting_for_final_keyframe {
                                                                        if waiting_start_time
                                                                            == 0 as int64_t
                                                                        {
                                                                            waiting_start_time =
                                                                                av_gettime();
                                                                        }
                                                                        let mut wait_time: int64_t = (av_gettime()
                                                                            - waiting_start_time) / 1000000 as int64_t;
                                                                        let mut keyframe_timeout_reached: bool = wait_time
                                                                            >= KEYFRAME_WAIT_TIMEOUT_S as int64_t;
                                                                        if is_keyframe as ::core::ffi::c_int != 0
                                                                            || shutdown_detected as ::core::ffi::c_int != 0
                                                                                && wait_time > SHUTDOWN_KEYFRAME_WAIT_TIMEOUT_S as int64_t
                                                                            || keyframe_timeout_reached as ::core::ffi::c_int != 0
                                                                        {
                                                                            if is_keyframe {
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_INFO,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Found final key frame, ending recording\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                );
                                                                                (*segment_info_ptr).last_frame_was_key = r#true != 0;
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Last frame was a key frame, next segment can start immediately (overlap mode)\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                );
                                                                                if (*segment_info_ptr).pending_video_keyframe.is_null() {
                                                                                    (*segment_info_ptr).pending_video_keyframe = av_packet_alloc()
                                                                                        as *mut AVPacket;
                                                                                }
                                                                                if !(*segment_info_ptr).pending_video_keyframe.is_null() {
                                                                                    av_packet_unref((*segment_info_ptr).pending_video_keyframe);
                                                                                    let mut ref_ret: ::core::ffi::c_int = av_packet_ref(
                                                                                        (*segment_info_ptr).pending_video_keyframe,
                                                                                        pkt,
                                                                                    );
                                                                                    if ref_ret < 0 as ::core::ffi::c_int {
                                                                                        _log_message_ctx(
                                                                                            log_level_t::LOG_LEVEL_WARN,
                                                                                            LOG_COMPONENT,
                                                                                            _log_stream_name,
                                                                                            b"Failed to store pending keyframe for next segment (ret=%d)\0"
                                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                                            ref_ret,
                                                                                        );
                                                                                        av_packet_free(
                                                                                            &raw mut (*segment_info_ptr).pending_video_keyframe,
                                                                                        );
                                                                                        (*segment_info_ptr).pending_video_keyframe = ::core::ptr::null_mut::<
                                                                                            AVPacket,
                                                                                        >();
                                                                                    } else {
                                                                                        _log_message_ctx(
                                                                                            log_level_t::LOG_LEVEL_DEBUG,
                                                                                            LOG_COMPONENT,
                                                                                            _log_stream_name,
                                                                                            b"Stored boundary keyframe for next segment start (overlap mode)\0"
                                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                                        );
                                                                                    }
                                                                                } else {
                                                                                    recording_io_report_failure(
                                                                                        recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                                                                        recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                                                                                        ENOMEM,
                                                                                    );
                                                                                    _log_message_ctx(
                                                                                        log_level_t::LOG_LEVEL_WARN,
                                                                                        LOG_COMPONENT,
                                                                                        _log_stream_name,
                                                                                        b"Failed to allocate pending keyframe packet for overlap mode\0"
                                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                                    );
                                                                                }
                                                                            } else {
                                                                                if keyframe_timeout_reached as ::core::ffi::c_int != 0
                                                                                    && !shutdown_detected
                                                                                {
                                                                                    _log_message_ctx(
                                                                                        log_level_t::LOG_LEVEL_WARN,
                                                                                        LOG_COMPONENT,
                                                                                        _log_stream_name,
                                                                                        b"Keyframe wait timeout after %lld s \xE2\x80\x94 camera has long keyframe interval? Cutting segment without final keyframe to enforce configured segment length.\0"
                                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                                        wait_time as ::core::ffi::c_longlong,
                                                                                    );
                                                                                } else {
                                                                                    _log_message_ctx(
                                                                                        log_level_t::LOG_LEVEL_INFO,
                                                                                        LOG_COMPONENT,
                                                                                        _log_stream_name,
                                                                                        b"Shutdown: waited %lld seconds for key frame, ending recording with non-key frame\0"
                                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                                        wait_time as ::core::ffi::c_longlong,
                                                                                    );
                                                                                }
                                                                                (*segment_info_ptr).last_frame_was_key = r#false != 0;
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Last frame was NOT a key frame, next segment will wait for a keyframe\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                );
                                                                            }
                                                                            if first_video_dts == AV_NOPTS_VALUE
                                                                                && (*pkt).dts != AV_NOPTS_VALUE
                                                                            {
                                                                                first_video_dts = (*pkt).dts;
                                                                                first_video_pts = if (*pkt).pts != AV_NOPTS_VALUE {
                                                                                    (*pkt).pts
                                                                                } else {
                                                                                    (*pkt).dts
                                                                                };
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"First video DTS: %lld, PTS: %lld\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                    first_video_dts as ::core::ffi::c_longlong,
                                                                                    first_video_pts as ::core::ffi::c_longlong,
                                                                                );
                                                                            }
                                                                            if segment_index == 0 as ::core::ffi::c_int {
                                                                                if (*pkt).dts != AV_NOPTS_VALUE
                                                                                    && first_video_dts != AV_NOPTS_VALUE
                                                                                {
                                                                                    (*pkt).dts -= first_video_dts;
                                                                                    if (*pkt).dts < 0 as int64_t {
                                                                                        (*pkt).dts = 0 as int64_t;
                                                                                    }
                                                                                }
                                                                                if (*pkt).pts != AV_NOPTS_VALUE
                                                                                    && first_video_pts != AV_NOPTS_VALUE
                                                                                {
                                                                                    (*pkt).pts -= first_video_pts;
                                                                                    if (*pkt).pts < 0 as int64_t {
                                                                                        (*pkt).pts = 0 as int64_t;
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if (*pkt).dts != AV_NOPTS_VALUE
                                                                                    && first_video_dts != AV_NOPTS_VALUE
                                                                                {
                                                                                    let mut relative_dts: int64_t = (*pkt).dts
                                                                                        - first_video_dts;
                                                                                    (*pkt).dts = relative_dts
                                                                                        + TIMESTAMP_CONTINUITY_OFFSET as int64_t;
                                                                                }
                                                                                if (*pkt).pts != AV_NOPTS_VALUE
                                                                                    && first_video_pts != AV_NOPTS_VALUE
                                                                                {
                                                                                    let mut relative_pts: int64_t = (*pkt).pts
                                                                                        - first_video_pts;
                                                                                    (*pkt).pts = relative_pts
                                                                                        + TIMESTAMP_CONTINUITY_OFFSET as int64_t;
                                                                                }
                                                                            }
                                                                            if (*pkt).pts != AV_NOPTS_VALUE
                                                                                && (*pkt).dts != AV_NOPTS_VALUE && (*pkt).pts < (*pkt).dts
                                                                            {
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Fixing video packet with PTS < DTS: PTS=%lld, DTS=%lld\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                    (*pkt).pts as ::core::ffi::c_longlong,
                                                                                    (*pkt).dts as ::core::ffi::c_longlong,
                                                                                );
                                                                                (*pkt).pts = (*pkt).dts;
                                                                            }
                                                                            if (*pkt).dts != AV_NOPTS_VALUE {
                                                                                clamp_dts_pts_for_mp4(
                                                                                    pkt,
                                                                                    DTS_RESET_SAFE_VALUE as int64_t,
                                                                                    MP4_DTS_WARNING_THRESHOLD as int64_t,
                                                                                    b"Video\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                    ::core::ptr::null_mut::<int64_t>(),
                                                                                    ::core::ptr::null_mut::<int64_t>(),
                                                                                );
                                                                            }
                                                                            if (*pkt).duration
                                                                                > MAX_PACKET_DURATION_TIMEBASE_UNITS as int64_t
                                                                            {
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_WARN,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Packet duration too large: %lld, capping at reasonable value\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                    (*pkt).duration as ::core::ffi::c_longlong,
                                                                                );
                                                                                let mut one_second: AVRational = AVRational {
                                                                                    num: 1 as ::core::ffi::c_int,
                                                                                    den: 1 as ::core::ffi::c_int,
                                                                                };
                                                                                let mut max_duration_in_stream_tb: int64_t = av_rescale_q(
                                                                                    1 as int64_t,
                                                                                    one_second,
                                                                                    (**(*input_ctx).streams.offset(video_stream_idx as isize))
                                                                                        .time_base,
                                                                                );
                                                                                if max_duration_in_stream_tb <= 0 as int64_t
                                                                                    || max_duration_in_stream_tb
                                                                                        > MAX_PACKET_DURATION_TIMEBASE_UNITS as int64_t
                                                                                {
                                                                                    max_duration_in_stream_tb = MAX_PACKET_DURATION_TIMEBASE_UNITS
                                                                                        as int64_t;
                                                                                }
                                                                                (*pkt).duration = max_duration_in_stream_tb;
                                                                            }
                                                                            if (*pkt).duration == 0 as int64_t
                                                                                || (*pkt).duration == AV_NOPTS_VALUE
                                                                            {
                                                                                (*pkt).duration = calculate_frame_duration_from_stream(
                                                                                    *(*input_ctx).streams.offset(video_stream_idx as isize),
                                                                                );
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Set final frame duration to %lld\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                    (*pkt).duration as ::core::ffi::c_longlong,
                                                                                );
                                                                            }
                                                                            if (*pkt).dts != AV_NOPTS_VALUE
                                                                                && last_video_dts != AV_NOPTS_VALUE
                                                                                && (*pkt).dts <= last_video_dts
                                                                            {
                                                                                let mut fixed_dts: int64_t = last_video_dts + 1 as int64_t;
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Fixing non-monotonic DTS in final frame: old=%lld, last=%lld, new=%lld\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                    (*pkt).dts as ::core::ffi::c_longlong,
                                                                                    last_video_dts as ::core::ffi::c_longlong,
                                                                                    fixed_dts as ::core::ffi::c_longlong,
                                                                                );
                                                                                if (*pkt).pts != AV_NOPTS_VALUE {
                                                                                    let mut pts_dts_diff: int64_t = (*pkt).pts - (*pkt).dts;
                                                                                    (*pkt).dts = fixed_dts;
                                                                                    (*pkt).pts = fixed_dts
                                                                                        + if pts_dts_diff > 0 as int64_t {
                                                                                            pts_dts_diff
                                                                                        } else {
                                                                                            0 as int64_t
                                                                                        };
                                                                                } else {
                                                                                    (*pkt).dts = fixed_dts;
                                                                                    (*pkt).pts = fixed_dts;
                                                                                }
                                                                            }
                                                                            (*pkt).stream_index = (*out_video_stream).index;
                                                                            mp4_segment_rescale_packet_ts(
                                                                                pkt,
                                                                                *(*input_ctx).streams.offset(video_stream_idx as isize),
                                                                                out_video_stream,
                                                                            );
                                                                            ret = av_interleaved_write_frame(output_ctx, pkt);
                                                                            if ret < 0 as ::core::ffi::c_int {
                                                                                recording_io_report_failure(
                                                                                    recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                                                                    recording_io_operation_t::RECORDING_IO_OPERATION_PACKET,
                                                                                    ret,
                                                                                );
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_ERROR,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Error writing final video frame: %d\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                    ret,
                                                                                );
                                                                                if is_terminal_storage_write_error(ret) {
                                                                                    _log_message_ctx(
                                                                                        log_level_t::LOG_LEVEL_ERROR,
                                                                                        LOG_COMPONENT,
                                                                                        _log_stream_name,
                                                                                        b"Non-recoverable write error on final frame (disk full, I/O error or read-only storage), stopping segment\0"
                                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                                    );
                                                                                    av_packet_unref(pkt);
                                                                                    break '_cleanup;
                                                                                }
                                                                            }
                                                                            av_packet_unref(pkt);
                                                                            break;
                                                                        }
                                                                    }
                                                                    if segment_index
                                                                        == 0 as ::core::ffi::c_int
                                                                    {
                                                                        if (*pkt).dts
                                                                            != AV_NOPTS_VALUE
                                                                            && first_video_dts
                                                                                != AV_NOPTS_VALUE
                                                                        {
                                                                            (*pkt).dts -=
                                                                                first_video_dts;
                                                                            if (*pkt).dts
                                                                                < 0 as int64_t
                                                                            {
                                                                                (*pkt).dts =
                                                                                    0 as int64_t;
                                                                            }
                                                                        }
                                                                        if (*pkt).pts
                                                                            != AV_NOPTS_VALUE
                                                                            && first_video_pts
                                                                                != AV_NOPTS_VALUE
                                                                        {
                                                                            (*pkt).pts -=
                                                                                first_video_pts;
                                                                            if (*pkt).pts
                                                                                < 0 as int64_t
                                                                            {
                                                                                (*pkt).pts =
                                                                                    0 as int64_t;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if (*pkt).dts
                                                                            != AV_NOPTS_VALUE
                                                                            && first_video_dts
                                                                                != AV_NOPTS_VALUE
                                                                        {
                                                                            let mut relative_dts_0: int64_t = (*pkt).dts
                                                                                - first_video_dts;
                                                                            (*pkt).dts = relative_dts_0
                                                                                + TIMESTAMP_CONTINUITY_OFFSET as int64_t;
                                                                        }
                                                                        if (*pkt).pts
                                                                            != AV_NOPTS_VALUE
                                                                            && first_video_pts
                                                                                != AV_NOPTS_VALUE
                                                                        {
                                                                            let mut relative_pts_0: int64_t = (*pkt).pts
                                                                                - first_video_pts;
                                                                            (*pkt).pts = relative_pts_0
                                                                                + TIMESTAMP_CONTINUITY_OFFSET as int64_t;
                                                                        }
                                                                    }
                                                                    if (*pkt).pts != AV_NOPTS_VALUE
                                                                        && (*pkt).dts
                                                                            != AV_NOPTS_VALUE
                                                                        && (*pkt).pts < (*pkt).dts
                                                                    {
                                                                        _log_message_ctx(
                                                                            log_level_t::LOG_LEVEL_DEBUG,
                                                                            LOG_COMPONENT,
                                                                            _log_stream_name,
                                                                            b"Fixing video packet with PTS < DTS: PTS=%lld, DTS=%lld\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            (*pkt).pts as ::core::ffi::c_longlong,
                                                                            (*pkt).dts as ::core::ffi::c_longlong,
                                                                        );
                                                                        (*pkt).pts = (*pkt).dts;
                                                                    }
                                                                    if (*pkt).dts != AV_NOPTS_VALUE
                                                                        && last_video_dts
                                                                            != AV_NOPTS_VALUE
                                                                        && (*pkt).dts
                                                                            <= last_video_dts
                                                                    {
                                                                        let mut fixed_dts_0: int64_t = last_video_dts
                                                                            + 1 as int64_t;
                                                                        _log_message_ctx(
                                                                            log_level_t::LOG_LEVEL_DEBUG,
                                                                            LOG_COMPONENT,
                                                                            _log_stream_name,
                                                                            b"Fixing non-monotonic DTS: old=%lld, last=%lld, new=%lld\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            (*pkt).dts as ::core::ffi::c_longlong,
                                                                            last_video_dts as ::core::ffi::c_longlong,
                                                                            fixed_dts_0 as ::core::ffi::c_longlong,
                                                                        );
                                                                        if (*pkt).pts
                                                                            != AV_NOPTS_VALUE
                                                                        {
                                                                            let mut pts_dts_diff_0: int64_t = (*pkt).pts - (*pkt).dts;
                                                                            (*pkt).dts =
                                                                                fixed_dts_0;
                                                                            (*pkt).pts = fixed_dts_0
                                                                                + if pts_dts_diff_0
                                                                                    > 0 as int64_t
                                                                                {
                                                                                    pts_dts_diff_0
                                                                                } else {
                                                                                    0 as int64_t
                                                                                };
                                                                        } else {
                                                                            (*pkt).dts =
                                                                                fixed_dts_0;
                                                                            (*pkt).pts =
                                                                                fixed_dts_0;
                                                                        }
                                                                    }
                                                                    if (*pkt).dts != AV_NOPTS_VALUE
                                                                    {
                                                                        last_video_dts = (*pkt).dts;
                                                                    }
                                                                    if (*pkt).pts != AV_NOPTS_VALUE
                                                                    {
                                                                        last_video_pts = (*pkt).pts;
                                                                    }
                                                                    if (*pkt).duration
                                                                        == 0 as int64_t
                                                                        || (*pkt).duration
                                                                            == AV_NOPTS_VALUE
                                                                    {
                                                                        (*pkt).duration = calculate_frame_duration_from_stream(
                                                                            *(*input_ctx).streams.offset(video_stream_idx as isize),
                                                                        );
                                                                        _log_message_ctx(
                                                                            log_level_t::LOG_LEVEL_DEBUG,
                                                                            LOG_COMPONENT,
                                                                            _log_stream_name,
                                                                            b"Set video packet duration to %lld\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            (*pkt).duration as ::core::ffi::c_longlong,
                                                                        );
                                                                    }
                                                                    (*pkt).stream_index =
                                                                        (*out_video_stream).index;
                                                                    mp4_segment_rescale_packet_ts(
                                                                        pkt,
                                                                        *(*input_ctx)
                                                                            .streams
                                                                            .offset(
                                                                                video_stream_idx
                                                                                    as isize,
                                                                            ),
                                                                        out_video_stream,
                                                                    );
                                                                    ret =
                                                                        av_interleaved_write_frame(
                                                                            output_ctx, pkt,
                                                                        );
                                                                    if ret < 0 as ::core::ffi::c_int
                                                                    {
                                                                        recording_io_report_failure(
                                                                            recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                                                            recording_io_operation_t::RECORDING_IO_OPERATION_PACKET,
                                                                            ret,
                                                                        );
                                                                        let mut error_buf_2: [::core::ffi::c_char; 64] = [
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
                                                                            &raw mut error_buf_2 as *mut ::core::ffi::c_char,
                                                                            AV_ERROR_MAX_STRING_SIZE as size_t,
                                                                        );
                                                                        _log_message_ctx(
                                                                            log_level_t::LOG_LEVEL_ERROR,
                                                                            LOG_COMPONENT,
                                                                            _log_stream_name,
                                                                            b"Error writing video frame: %d (%s)\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            ret,
                                                                            &raw mut error_buf_2 as *mut ::core::ffi::c_char,
                                                                        );
                                                                        if is_terminal_storage_write_error(ret) {
                                                                            _log_message_ctx(
                                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                                LOG_COMPONENT,
                                                                                _log_stream_name,
                                                                                b"Non-recoverable video write error (disk full, I/O error or read-only storage), stopping segment\0"
                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                            );
                                                                            break '_cleanup;
                                                                        } else if ret == -22 as ::core::ffi::c_int
                                                                            && !strstr(
                                                                                    &raw mut error_buf_2 as *mut ::core::ffi::c_char,
                                                                                    b"monoton\0".as_ptr() as *const ::core::ffi::c_char,
                                                                                )
                                                                                .is_null()
                                                                        {
                                                                            _log_message_ctx(
                                                                                log_level_t::LOG_LEVEL_WARN,
                                                                                LOG_COMPONENT,
                                                                                _log_stream_name,
                                                                                b"Detected timestamp error, will try to fix for next packet\0"
                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                            );
                                                                            consecutive_timestamp_errors += 1;
                                                                            if consecutive_timestamp_errors >= max_timestamp_errors {
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_WARN,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Too many consecutive timestamp errors (%d), resetting all timestamps\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                    consecutive_timestamp_errors,
                                                                                );
                                                                                first_video_dts = AV_NOPTS_VALUE;
                                                                                first_video_pts = AV_NOPTS_VALUE;
                                                                                last_video_dts = AV_NOPTS_VALUE;
                                                                                last_video_pts = AV_NOPTS_VALUE;
                                                                                first_audio_dts = AV_NOPTS_VALUE;
                                                                                first_audio_pts = AV_NOPTS_VALUE;
                                                                                last_audio_dts = AV_NOPTS_VALUE;
                                                                                last_audio_pts = AV_NOPTS_VALUE;
                                                                                consecutive_timestamp_errors = 0 as ::core::ffi::c_int;
                                                                            } else {
                                                                                last_video_dts
                                                                                    += 100 as ::core::ffi::c_int as int64_t
                                                                                        * consecutive_timestamp_errors as int64_t;
                                                                                last_video_pts
                                                                                    += 100 as ::core::ffi::c_int as int64_t
                                                                                        * consecutive_timestamp_errors as int64_t;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        consecutive_timestamp_errors = 0 as ::core::ffi::c_int;
                                                                        video_packet_count += 1;
                                                                        if video_packet_count % 300 as ::core::ffi::c_int
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            _log_message_ctx(
                                                                                log_level_t::LOG_LEVEL_DEBUG,
                                                                                LOG_COMPONENT,
                                                                                _log_stream_name,
                                                                                b"Processed %d video packets\0".as_ptr()
                                                                                    as *const ::core::ffi::c_char,
                                                                                video_packet_count,
                                                                            );
                                                                        }
                                                                        maybe_release_recording_file_cache(
                                                                            output_ctx,
                                                                            &raw mut recording_cache_fd,
                                                                            &raw mut recording_cache_released_through,
                                                                        );
                                                                    }
                                                                } else if has_audio != 0
                                                                    && !g_config.audio_disabled
                                                                    && audio_stream_idx
                                                                        >= 0 as ::core::ffi::c_int
                                                                    && (*pkt).stream_index
                                                                        == audio_stream_idx
                                                                    && !out_audio_stream.is_null()
                                                                {
                                                                    if !found_first_keyframe {
                                                                        av_packet_unref(pkt);
                                                                        continue;
                                                                    } else {
                                                                        if first_audio_dts
                                                                            == AV_NOPTS_VALUE
                                                                            && (*pkt).dts
                                                                                != AV_NOPTS_VALUE
                                                                        {
                                                                            first_audio_dts =
                                                                                (*pkt).dts;
                                                                            first_audio_pts = if (*pkt).pts != AV_NOPTS_VALUE {
                                                                                (*pkt).pts
                                                                            } else {
                                                                                (*pkt).dts
                                                                            };
                                                                            _log_message_ctx(
                                                                                log_level_t::LOG_LEVEL_DEBUG,
                                                                                LOG_COMPONENT,
                                                                                _log_stream_name,
                                                                                b"First audio DTS: %lld, PTS: %lld\0".as_ptr()
                                                                                    as *const ::core::ffi::c_char,
                                                                                first_audio_dts as ::core::ffi::c_longlong,
                                                                                first_audio_pts as ::core::ffi::c_longlong,
                                                                            );
                                                                        }
                                                                        if segment_index == 0 as ::core::ffi::c_int {
                                                                            if (*pkt).dts != AV_NOPTS_VALUE
                                                                                && first_audio_dts != AV_NOPTS_VALUE
                                                                            {
                                                                                (*pkt).dts -= first_audio_dts;
                                                                                if (*pkt).dts < 0 as int64_t {
                                                                                    (*pkt).dts = 0 as int64_t;
                                                                                }
                                                                            }
                                                                            if (*pkt).pts != AV_NOPTS_VALUE
                                                                                && first_audio_pts != AV_NOPTS_VALUE
                                                                            {
                                                                                (*pkt).pts -= first_audio_pts;
                                                                                if (*pkt).pts < 0 as int64_t {
                                                                                    (*pkt).pts = 0 as int64_t;
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if (*pkt).dts != AV_NOPTS_VALUE
                                                                                && first_audio_dts != AV_NOPTS_VALUE
                                                                            {
                                                                                let mut relative_dts_1: int64_t = (*pkt).dts
                                                                                    - first_audio_dts;
                                                                                (*pkt).dts = relative_dts_1
                                                                                    + TIMESTAMP_CONTINUITY_OFFSET as int64_t;
                                                                            }
                                                                            if (*pkt).pts != AV_NOPTS_VALUE
                                                                                && first_audio_pts != AV_NOPTS_VALUE
                                                                            {
                                                                                let mut relative_pts_1: int64_t = (*pkt).pts
                                                                                    - first_audio_pts;
                                                                                (*pkt).pts = relative_pts_1
                                                                                    + TIMESTAMP_CONTINUITY_OFFSET as int64_t;
                                                                            }
                                                                        }
                                                                        if audio_packet_count > 0 as ::core::ffi::c_int {
                                                                            if (*pkt).dts != AV_NOPTS_VALUE
                                                                                && (*pkt).dts <= last_audio_dts
                                                                            {
                                                                                let mut fixed_dts_1: int64_t = last_audio_dts
                                                                                    + 1 as int64_t;
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Fixing non-monotonic audio DTS: old=%lld, last=%lld, new=%lld\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                    (*pkt).dts as ::core::ffi::c_longlong,
                                                                                    last_audio_dts as ::core::ffi::c_longlong,
                                                                                    fixed_dts_1 as ::core::ffi::c_longlong,
                                                                                );
                                                                                (*pkt).dts = fixed_dts_1;
                                                                            }
                                                                            if (*pkt).pts != AV_NOPTS_VALUE
                                                                                && (*pkt).pts <= last_audio_pts
                                                                            {
                                                                                let mut fixed_pts: int64_t = last_audio_pts + 1 as int64_t;
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Fixing non-monotonic audio PTS: old=%lld, last=%lld, new=%lld\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                    (*pkt).pts as ::core::ffi::c_longlong,
                                                                                    last_audio_pts as ::core::ffi::c_longlong,
                                                                                    fixed_pts as ::core::ffi::c_longlong,
                                                                                );
                                                                                (*pkt).pts = fixed_pts;
                                                                            }
                                                                            if (*pkt).pts != AV_NOPTS_VALUE
                                                                                && (*pkt).dts != AV_NOPTS_VALUE && (*pkt).pts < (*pkt).dts
                                                                            {
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Fixing audio packet with PTS < DTS: PTS=%lld, DTS=%lld\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                    (*pkt).pts as ::core::ffi::c_longlong,
                                                                                    (*pkt).dts as ::core::ffi::c_longlong,
                                                                                );
                                                                                (*pkt).pts = (*pkt).dts;
                                                                            }
                                                                        }
                                                                        clamp_dts_pts_for_mp4(
                                                                            pkt,
                                                                            AUDIO_DTS_RESET_SAFE_VALUE as int64_t,
                                                                            MP4_DTS_WARNING_THRESHOLD as int64_t,
                                                                            b"Audio\0".as_ptr() as *const ::core::ffi::c_char,
                                                                            &raw mut last_audio_dts,
                                                                            &raw mut last_audio_pts,
                                                                        );
                                                                        if (*pkt).dts
                                                                            != AV_NOPTS_VALUE
                                                                        {
                                                                            last_audio_dts =
                                                                                (*pkt).dts;
                                                                        }
                                                                        if (*pkt).pts
                                                                            != AV_NOPTS_VALUE
                                                                        {
                                                                            last_audio_pts =
                                                                                (*pkt).pts;
                                                                        }
                                                                        if (*pkt).duration
                                                                            == 0 as int64_t
                                                                            || (*pkt).duration
                                                                                == AV_NOPTS_VALUE
                                                                        {
                                                                            let mut audio_stream: *mut AVStream = *(*input_ctx)
                                                                                .streams
                                                                                .offset(audio_stream_idx as isize);
                                                                            if (*(*audio_stream).codecpar).sample_rate
                                                                                > 0 as ::core::ffi::c_int
                                                                            {
                                                                                let mut nb_samples: ::core::ffi::c_int = 0
                                                                                    as ::core::ffi::c_int;
                                                                                if (*(*audio_stream).codecpar).ch_layout.nb_channels
                                                                                    > 0 as ::core::ffi::c_int
                                                                                    && (*(*audio_stream).codecpar).bits_per_coded_sample
                                                                                        > 0 as ::core::ffi::c_int
                                                                                {
                                                                                    let mut bytes_per_sample: ::core::ffi::c_int = (*(*audio_stream)
                                                                                        .codecpar)
                                                                                        .bits_per_coded_sample / 8 as ::core::ffi::c_int;
                                                                                    if bytes_per_sample > 0 as ::core::ffi::c_int {
                                                                                        nb_samples = (*pkt).size
                                                                                            / ((*(*audio_stream).codecpar).ch_layout.nb_channels
                                                                                                * bytes_per_sample);
                                                                                    }
                                                                                }
                                                                                if nb_samples > 0 as ::core::ffi::c_int {
                                                                                    (*pkt).duration = av_rescale_q(
                                                                                        nb_samples as int64_t,
                                                                                        AVRational {
                                                                                            num: 1 as ::core::ffi::c_int,
                                                                                            den: (*(*audio_stream).codecpar).sample_rate,
                                                                                        },
                                                                                        (*audio_stream).time_base,
                                                                                    );
                                                                                } else {
                                                                                    (*pkt).duration = av_rescale_q(
                                                                                        1024 as int64_t,
                                                                                        AVRational {
                                                                                            num: 1 as ::core::ffi::c_int,
                                                                                            den: (*(*audio_stream).codecpar).sample_rate,
                                                                                        },
                                                                                        (*audio_stream).time_base,
                                                                                    );
                                                                                }
                                                                            } else {
                                                                                (*pkt).duration = 1 as int64_t;
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Set default audio packet duration to 1\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                );
                                                                            }
                                                                        }
                                                                        (*pkt).stream_index =
                                                                            (*out_audio_stream)
                                                                                .index;
                                                                        if needs_audio_transcoding {
                                                                            let mut transcoded_pkt: *mut AVPacket = av_packet_alloc();
                                                                            if transcoded_pkt
                                                                                .is_null()
                                                                            {
                                                                                recording_io_report_failure(
                                                                                    recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                                                                    recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                                                                                    ENOMEM,
                                                                                );
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_ERROR,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Failed to allocate packet for transcoded audio\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                );
                                                                                av_packet_unref(
                                                                                    pkt,
                                                                                );
                                                                                ret = -12 as ::core::ffi::c_int;
                                                                                break '_cleanup;
                                                                            } else {
                                                                                let mut transcoder_key: *const ::core::ffi::c_char = if (*segment_info_ptr)
                                                                                    .stream_name[0usize] as ::core::ffi::c_int
                                                                                    != '\0' as ::core::ffi::c_int
                                                                                {
                                                                                    &raw mut (*segment_info_ptr).stream_name
                                                                                        as *mut ::core::ffi::c_char
                                                                                } else {
                                                                                    rtsp_url
                                                                                };
                                                                                let mut tc_ret: ::core::ffi::c_int = transcode_audio_packet(
                                                                                    transcoder_key,
                                                                                    pkt,
                                                                                    transcoded_pkt,
                                                                                    *(*input_ctx).streams.offset(audio_stream_idx as isize),
                                                                                );
                                                                                if tc_ret < 0 as ::core::ffi::c_int {
                                                                                    recording_io_report_failure(
                                                                                        recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                                                                        recording_io_operation_t::RECORDING_IO_OPERATION_PACKET,
                                                                                        tc_ret,
                                                                                    );
                                                                                    _log_message_ctx(
                                                                                        log_level_t::LOG_LEVEL_ERROR,
                                                                                        LOG_COMPONENT,
                                                                                        _log_stream_name,
                                                                                        b"Failed to transcode audio for %s: %d\0".as_ptr()
                                                                                            as *const ::core::ffi::c_char,
                                                                                        transcoder_key,
                                                                                        tc_ret,
                                                                                    );
                                                                                    av_packet_free(&raw mut transcoded_pkt);
                                                                                    av_packet_unref(pkt);
                                                                                    ret = tc_ret;
                                                                                    break '_cleanup;
                                                                                } else if (*transcoded_pkt).data.is_null()
                                                                                    || (*transcoded_pkt).size <= 0 as ::core::ffi::c_int
                                                                                {
                                                                                    av_packet_free(&raw mut transcoded_pkt);
                                                                                    av_packet_unref(pkt);
                                                                                    continue;
                                                                                } else {
                                                                                    while (*transcoded_pkt).size > 0 as ::core::ffi::c_int {
                                                                                        (*transcoded_pkt).stream_index = (*out_audio_stream).index;
                                                                                        if next_transcoded_audio_pts == AV_NOPTS_VALUE {
                                                                                            next_transcoded_audio_pts = if segment_index
                                                                                                <= 1 as ::core::ffi::c_int
                                                                                            {
                                                                                                -((*(*out_audio_stream).codecpar).initial_padding
                                                                                                    as int64_t)
                                                                                            } else {
                                                                                                0 as int64_t
                                                                                            };
                                                                                        }
                                                                                        (*transcoded_pkt).pts = next_transcoded_audio_pts;
                                                                                        (*transcoded_pkt).dts = next_transcoded_audio_pts;
                                                                                        (*transcoded_pkt).duration = (if (*(*out_audio_stream)
                                                                                            .codecpar)
                                                                                            .frame_size > 0 as ::core::ffi::c_int
                                                                                        {
                                                                                            (*(*out_audio_stream).codecpar).frame_size
                                                                                        } else {
                                                                                            1024 as ::core::ffi::c_int
                                                                                        }) as int64_t;
                                                                                        (*transcoded_pkt).time_base = (*out_audio_stream).time_base;
                                                                                        next_transcoded_audio_pts += (*transcoded_pkt).duration;
                                                                                        ret = av_interleaved_write_frame(
                                                                                            output_ctx,
                                                                                            transcoded_pkt,
                                                                                        );
                                                                                        if ret < 0 as ::core::ffi::c_int {
                                                                                            break;
                                                                                        }
                                                                                        ret = transcode_audio_packet(
                                                                                            transcoder_key,
                                                                                            ::core::ptr::null::<AVPacket>(),
                                                                                            transcoded_pkt,
                                                                                            *(*input_ctx).streams.offset(audio_stream_idx as isize),
                                                                                        );
                                                                                        if ret < 0 as ::core::ffi::c_int {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    av_packet_free(&raw mut transcoded_pkt);
                                                                                }
                                                                            }
                                                                        } else {
                                                                            mp4_segment_rescale_packet_ts(
                                                                                pkt,
                                                                                *(*input_ctx).streams.offset(audio_stream_idx as isize),
                                                                                out_audio_stream,
                                                                            );
                                                                            ret = av_interleaved_write_frame(output_ctx, pkt);
                                                                        }
                                                                        if ret < 0
                                                                            as ::core::ffi::c_int
                                                                        {
                                                                            recording_io_report_failure(
                                                                                recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                                                                recording_io_operation_t::RECORDING_IO_OPERATION_PACKET,
                                                                                ret,
                                                                            );
                                                                            let mut error_buf_3: [::core::ffi::c_char; 64] = [
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
                                                                                &raw mut error_buf_3 as *mut ::core::ffi::c_char,
                                                                                AV_ERROR_MAX_STRING_SIZE as size_t,
                                                                            );
                                                                            _log_message_ctx(
                                                                                log_level_t::LOG_LEVEL_ERROR,
                                                                                LOG_COMPONENT,
                                                                                _log_stream_name,
                                                                                b"Error writing audio frame: %d (%s)\0".as_ptr()
                                                                                    as *const ::core::ffi::c_char,
                                                                                ret,
                                                                                &raw mut error_buf_3 as *mut ::core::ffi::c_char,
                                                                            );
                                                                            if is_terminal_storage_write_error(ret) {
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_ERROR,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Non-recoverable audio write error (disk full, I/O error or read-only storage), stopping segment\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                );
                                                                                break '_cleanup;
                                                                            } else if mp4_segment_audio_error_requires_input_refresh(
                                                                                ret,
                                                                            ) {
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_WARN,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Audio mux state is invalid; disabling audio for the remainder of this segment and refreshing the RTSP input before the next segment\0"
                                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                                );
                                                                                has_audio = 0 as ::core::ffi::c_int;
                                                                                reuse_input_context = r#false != 0;
                                                                                ret = 0 as ::core::ffi::c_int;
                                                                            }
                                                                        } else {
                                                                            consecutive_timestamp_errors = 0 as ::core::ffi::c_int;
                                                                            audio_packet_count += 1;
                                                                            if audio_packet_count % 300 as ::core::ffi::c_int
                                                                                == 0 as ::core::ffi::c_int
                                                                            {
                                                                                _log_message_ctx(
                                                                                    log_level_t::LOG_LEVEL_DEBUG,
                                                                                    LOG_COMPONENT,
                                                                                    _log_stream_name,
                                                                                    b"Processed %d audio packets\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char,
                                                                                    audio_packet_count,
                                                                                );
                                                                            }
                                                                            maybe_release_recording_file_cache(
                                                                                output_ctx,
                                                                                &raw mut recording_cache_fd,
                                                                                &raw mut recording_cache_released_through,
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                                av_packet_unref(pkt);
                                                            }
                                                        }
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_INFO,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Recording segment complete (video packets: %d, audio packets: %d)\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                            video_packet_count,
                                                            audio_packet_count,
                                                        );
                                                        if video_packet_count
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_WARN,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"Segment recorded 0 video packets \xE2\x80\x94 treating as failure (RTSP connection likely dead)\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            if !input_ctx.is_null() {
                                                                avformat_close_input(
                                                                    &raw mut input_ctx,
                                                                );
                                                                input_ctx = ::core::ptr::null_mut::<
                                                                    AVFormatContext,
                                                                >(
                                                                );
                                                            }
                                                            *input_ctx_ptr = ::core::ptr::null_mut::<
                                                                AVFormatContext,
                                                            >(
                                                            );
                                                            ret = -1 as ::core::ffi::c_int;
                                                        } else {
                                                            final_audio_error =
                                                                0 as ::core::ffi::c_int;
                                                            if (shutdown_detected
                                                                as ::core::ffi::c_int
                                                                != 0
                                                                || !reuse_input_context)
                                                                && needs_audio_transcoding
                                                                    as ::core::ffi::c_int
                                                                    != 0
                                                                && !out_audio_stream.is_null()
                                                                && !output_ctx.is_null()
                                                            {
                                                                let mut transcoder_key_0: *const ::core::ffi::c_char = if (*segment_info_ptr)
                                                                    .stream_name[0usize] as ::core::ffi::c_int
                                                                    != '\0' as ::core::ffi::c_int
                                                                {
                                                                    &raw mut (*segment_info_ptr).stream_name
                                                                        as *mut ::core::ffi::c_char
                                                                } else {
                                                                    rtsp_url
                                                                };
                                                                let mut flush_packets: *mut *mut AVPacket = ::core::ptr::null_mut::<
                                                                    *mut AVPacket,
                                                                >();
                                                                let mut flush_count: ::core::ffi::c_int = 0
                                                                    as ::core::ffi::c_int;
                                                                let mut discard_padding: ::core::ffi::c_int = 0
                                                                    as ::core::ffi::c_int;
                                                                let mut flush_ret: ::core::ffi::c_int = transcode_audio_flush(
                                                                    transcoder_key_0,
                                                                    &raw mut flush_packets,
                                                                    &raw mut flush_count,
                                                                    &raw mut discard_padding,
                                                                );
                                                                if flush_ret
                                                                    < 0 as ::core::ffi::c_int
                                                                {
                                                                    _log_message_ctx(
                                                                        log_level_t::LOG_LEVEL_ERROR,
                                                                        LOG_COMPONENT,
                                                                        _log_stream_name,
                                                                        b"Failed to flush AAC transcoder for %s: %d\0".as_ptr()
                                                                            as *const ::core::ffi::c_char,
                                                                        transcoder_key_0,
                                                                        flush_ret,
                                                                    );
                                                                    final_audio_error = flush_ret;
                                                                } else {
                                                                    if next_transcoded_audio_pts
                                                                        == AV_NOPTS_VALUE
                                                                    {
                                                                        next_transcoded_audio_pts = if segment_index
                                                                            <= 1 as ::core::ffi::c_int
                                                                        {
                                                                            -((*(*out_audio_stream).codecpar).initial_padding
                                                                                as int64_t)
                                                                        } else {
                                                                            0 as int64_t
                                                                        };
                                                                    }
                                                                    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                    while i_0 < flush_count {
                                                                        let mut flush_pkt: *mut AVPacket = *flush_packets
                                                                            .offset(i_0 as isize);
                                                                        (*flush_pkt).stream_index =
                                                                            (*out_audio_stream)
                                                                                .index;
                                                                        (*flush_pkt).dts = next_transcoded_audio_pts;
                                                                        (*flush_pkt).pts =
                                                                            (*flush_pkt).dts;
                                                                        (*flush_pkt).duration = (if (*(*out_audio_stream).codecpar)
                                                                            .frame_size > 0 as ::core::ffi::c_int
                                                                        {
                                                                            (*(*out_audio_stream).codecpar).frame_size
                                                                        } else {
                                                                            1024 as ::core::ffi::c_int
                                                                        }) as int64_t;
                                                                        if i_0 == flush_count - 1 as ::core::ffi::c_int
                                                                            && discard_padding > 0 as ::core::ffi::c_int
                                                                            && (discard_padding as int64_t) < (*flush_pkt).duration
                                                                        {
                                                                            (*flush_pkt).duration -= discard_padding as int64_t;
                                                                            (*(*out_audio_stream).codecpar).trailing_padding = discard_padding;
                                                                        }
                                                                        (*flush_pkt).time_base =
                                                                            (*out_audio_stream)
                                                                                .time_base;
                                                                        next_transcoded_audio_pts += (*flush_pkt).duration;
                                                                        let mut write_ret: ::core::ffi::c_int = av_interleaved_write_frame(
                                                                            output_ctx,
                                                                            flush_pkt,
                                                                        );
                                                                        if write_ret < 0 as ::core::ffi::c_int {
                                                                            final_audio_error = write_ret;
                                                                            break;
                                                                        } else {
                                                                            i_0 += 1;
                                                                        }
                                                                    }
                                                                }
                                                                let mut i_1: ::core::ffi::c_int =
                                                                    0 as ::core::ffi::c_int;
                                                                while i_1 < flush_count {
                                                                    av_packet_free(
                                                                        flush_packets
                                                                            .offset(i_1 as isize),
                                                                    );
                                                                    i_1 += 1;
                                                                }
                                                                av_free(
                                                                    flush_packets
                                                                        as *mut ::core::ffi::c_void,
                                                                );
                                                            }
                                                            if !output_ctx.is_null()
                                                                && !(*output_ctx).pb.is_null()
                                                            {
                                                                ret = av_write_trailer(output_ctx);
                                                                if ret < 0 as ::core::ffi::c_int {
                                                                    recording_io_report_failure(
                                                                        recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                                                                        recording_io_operation_t::RECORDING_IO_OPERATION_TRAILER,
                                                                        ret,
                                                                    );
                                                                    _log_message_ctx(
                                                                        log_level_t::LOG_LEVEL_ERROR,
                                                                        LOG_COMPONENT,
                                                                        _log_stream_name,
                                                                        b"Failed to write trailer: %d\0".as_ptr()
                                                                            as *const ::core::ffi::c_char,
                                                                        ret,
                                                                    );
                                                                } else {
                                                                    trailer_written = r#true != 0;
                                                                    _log_message_ctx(
                                                                        log_level_t::LOG_LEVEL_DEBUG,
                                                                        LOG_COMPONENT,
                                                                        _log_stream_name,
                                                                        b"Successfully wrote trailer to output file\0".as_ptr()
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                }
                                                                if ret >= 0 as ::core::ffi::c_int
                                                                    && final_audio_error
                                                                        < 0 as ::core::ffi::c_int
                                                                {
                                                                    ret = final_audio_error;
                                                                }
                                                            }
                                                            (*segment_info_ptr).segment_index =
                                                                segment_index;
                                                            (*segment_info_ptr).has_audio =
                                                                has_audio != 0
                                                                    && audio_stream_idx
                                                                        >= 0 as ::core::ffi::c_int;
                                                            _log_message_ctx(
                                                                log_level_t::LOG_LEVEL_INFO,
                                                                LOG_COMPONENT,
                                                                _log_stream_name,
                                                                b"Saved segment info for next segment: index=%d, has_audio=%d, last_frame_was_key=%d\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                segment_index,
                                                                (has_audio != 0
                                                                    && audio_stream_idx >= 0 as ::core::ffi::c_int)
                                                                    as ::core::ffi::c_int,
                                                                (*segment_info_ptr).last_frame_was_key as ::core::ffi::c_int,
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting aggressive cleanup of FFmpeg resources\0".as_ptr() as *const ::core::ffi::c_char,
    );
    av_dict_free(&raw mut opts);
    av_dict_free(&raw mut out_opts);
    if !pkt.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Freeing packet during cleanup\0".as_ptr() as *const ::core::ffi::c_char,
        );
        av_packet_unref(pkt);
        av_packet_free(&raw mut pkt);
        pkt = ::core::ptr::null_mut::<AVPacket>();
    }
    if !input_ctx.is_null() && !(*input_ctx).pb.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Flushing input context\0".as_ptr() as *const ::core::ffi::c_char,
        );
        avio_flush((*input_ctx).pb);
    }
    if !output_ctx.is_null() && !(*output_ctx).pb.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Flushing output context\0".as_ptr() as *const ::core::ffi::c_char,
        );
        avio_flush((*output_ctx).pb);
    }
    if !output_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaning up output context\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !(*output_ctx).pb.is_null() && ret >= 0 as ::core::ffi::c_int && !trailer_written {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Writing trailer during cleanup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut trailer_ret: ::core::ffi::c_int = av_write_trailer(output_ctx);
            if trailer_ret < 0 as ::core::ffi::c_int {
                recording_io_report_failure(
                    recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                    recording_io_operation_t::RECORDING_IO_OPERATION_TRAILER,
                    trailer_ret,
                );
            }
        }
        if !(*output_ctx).pb.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Closing output file\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut close_ret: ::core::ffi::c_int = recording_file_close(&raw mut (*output_ctx).pb);
            if close_ret < 0 as ::core::ffi::c_int {
                recording_io_report_failure(
                    recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                    recording_io_operation_t::RECORDING_IO_OPERATION_CLOSE,
                    close_ret,
                );
            }
        }
        release_all_recording_file_cache(&raw mut recording_cache_fd, output_file);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Freeing output context\0".as_ptr() as *const ::core::ffi::c_char,
        );
        avformat_free_context(output_ctx);
        output_ctx = ::core::ptr::null_mut::<AVFormatContext>();
    }
    release_all_recording_file_cache(&raw mut recording_cache_fd, output_file);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Handling input context cleanup\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if ret >= 0 as ::core::ffi::c_int
        && reuse_input_context as ::core::ffi::c_int != 0
        && !shutdown_detected
    {
        *input_ctx_ptr = input_ctx;
        input_ctx = ::core::ptr::null_mut::<AVFormatContext>();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stored input context for reuse in next segment\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Closing input context due to error or refresh request\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        if !input_ctx.is_null() {
            if !(*input_ctx).pb.is_null() {
                avio_flush((*input_ctx).pb);
            }
            avformat_close_input(&raw mut input_ctx);
            input_ctx = ::core::ptr::null_mut::<AVFormatContext>();
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Input context is NULL, nothing to clean up\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Closed input context due to error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut transcoder_key_1: *const ::core::ffi::c_char =
            if (*segment_info_ptr).stream_name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                &raw mut (*segment_info_ptr).stream_name as *mut ::core::ffi::c_char
            } else {
                rtsp_url
            };
        cleanup_audio_transcoder(transcoder_key_1);
    }
    return ret;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mp4_segment_recorder_cleanup() {
    av_log_set_level(AV_LOG_QUIET);
    avformat_network_deinit();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MP4 segment recorder resources cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mp4_segment_recorder_write_packet(
    mut writer: *mut mp4_writer_t,
    mut pkt: *const AVPacket,
    mut input_stream: *const AVStream,
) -> ::core::ffi::c_int {
    if writer.is_null() || pkt.is_null() || input_stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters passed to mp4_segment_recorder_write_packet\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*writer).output_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Writer output context is NULL for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            if !(&raw mut (*writer).stream_name as *mut ::core::ffi::c_char).is_null() {
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char
            } else {
                b"unknown\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut out_pkt: *mut AVPacket = av_packet_alloc();
    if out_pkt.is_null() {
        recording_io_report_failure(
            recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
            recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
            ENOMEM,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate packet for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            if !(&raw mut (*writer).stream_name as *mut ::core::ffi::c_char).is_null() {
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char
            } else {
                b"unknown\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut ret: ::core::ffi::c_int = av_packet_ref(out_pkt, pkt);
    if ret < 0 as ::core::ffi::c_int {
        if ret == -12 as ::core::ffi::c_int {
            recording_io_report_failure(
                recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                ret,
            );
        }
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
            b"Failed to copy packet for stream %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if !(&raw mut (*writer).stream_name as *mut ::core::ffi::c_char).is_null() {
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char
            } else {
                b"unknown\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut error_buf as *mut ::core::ffi::c_char,
        );
        av_packet_free(&raw mut out_pkt);
        return ret;
    }
    if (*(*input_stream).codecpar).codec_type.0 == AVMediaType::AVMEDIA_TYPE_VIDEO.0 {
        (*out_pkt).stream_index = (*writer).video_stream_idx;
        if (*writer).first_dts == AV_NOPTS_VALUE && (*out_pkt).dts != AV_NOPTS_VALUE {
            (*writer).first_dts = (*out_pkt).dts;
            (*writer).first_pts = if (*out_pkt).pts != AV_NOPTS_VALUE {
                (*out_pkt).pts
            } else {
                (*out_pkt).dts
            };
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] First video DTS: %lld, PTS: %lld (for timestamp remapping)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
                (*writer).first_dts as ::core::ffi::c_longlong,
                (*writer).first_pts as ::core::ffi::c_longlong,
            );
        }
        if (*writer).first_dts != AV_NOPTS_VALUE && (*out_pkt).dts != AV_NOPTS_VALUE {
            (*out_pkt).dts -= (*writer).first_dts;
            if (*out_pkt).dts < 0 as int64_t {
                (*out_pkt).dts = 0 as int64_t;
            }
        }
        if (*writer).first_pts != AV_NOPTS_VALUE && (*out_pkt).pts != AV_NOPTS_VALUE {
            (*out_pkt).pts -= (*writer).first_pts;
            if (*out_pkt).pts < 0 as int64_t {
                (*out_pkt).pts = 0 as int64_t;
            }
        }
        if (*out_pkt).pts != AV_NOPTS_VALUE
            && (*out_pkt).dts != AV_NOPTS_VALUE
            && (*out_pkt).pts < (*out_pkt).dts
        {
            (*out_pkt).pts = (*out_pkt).dts;
        }
        if (*out_pkt).dts != AV_NOPTS_VALUE
            && (*writer).last_dts != AV_NOPTS_VALUE
            && (*out_pkt).dts > (*writer).last_dts
        {
            let mut threshold_sec: ::core::ffi::c_int =
                if (*writer).pre_buffer_seconds > 0 as ::core::ffi::c_int {
                    3 as ::core::ffi::c_int * (*writer).pre_buffer_seconds
                } else {
                    60 as ::core::ffi::c_int
                };
            let mut threshold_units: int64_t = av_rescale_q(
                threshold_sec as int64_t,
                AVRational {
                    num: 1 as ::core::ffi::c_int,
                    den: 1 as ::core::ffi::c_int,
                },
                (*input_stream).time_base,
            );
            let mut gap_units: int64_t = (*out_pkt).dts - (*writer).last_dts;
            if gap_units > threshold_units {
                let mut gap_sec: ::core::ffi::c_double =
                    gap_units as ::core::ffi::c_double * av_q2d((*input_stream).time_base);
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] DTS gap detector: %.2fs forward jump in %s (last_dts=%lld new_dts=%lld threshold=%ds) \xE2\x80\x94 MP4 timeline will be discontinuous\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
                    gap_sec,
                    &raw mut (*writer).output_path as *mut ::core::ffi::c_char,
                    (*writer).last_dts as ::core::ffi::c_longlong,
                    (*out_pkt).dts as ::core::ffi::c_longlong,
                    threshold_sec,
                );
            }
        }
        if (*out_pkt).dts != AV_NOPTS_VALUE
            && (*writer).last_dts != AV_NOPTS_VALUE
            && (*out_pkt).dts <= (*writer).last_dts
        {
            let mut fixed_dts: int64_t = (*writer).last_dts + 1 as int64_t;
            if (*out_pkt).pts != AV_NOPTS_VALUE {
                let mut pts_dts_diff: int64_t = (*out_pkt).pts - (*out_pkt).dts;
                (*out_pkt).dts = fixed_dts;
                (*out_pkt).pts = fixed_dts
                    + if pts_dts_diff > 0 as int64_t {
                        pts_dts_diff
                    } else {
                        0 as int64_t
                    };
            } else {
                (*out_pkt).dts = fixed_dts;
                (*out_pkt).pts = fixed_dts;
            }
        }
        (*writer).last_dts = (*out_pkt).dts;
        if (*out_pkt).duration == 0 as int64_t || (*out_pkt).duration == AV_NOPTS_VALUE {
            if (*input_stream).avg_frame_rate.num > 0 as ::core::ffi::c_int
                && (*input_stream).avg_frame_rate.den > 0 as ::core::ffi::c_int
            {
                (*out_pkt).duration = av_rescale_q(
                    1 as int64_t,
                    av_inv_q((*input_stream).avg_frame_rate),
                    (*input_stream).time_base,
                );
            } else {
                (*out_pkt).duration = 1 as int64_t;
            }
        }
        let mut out_stream: *mut AVStream = *(*(*writer).output_ctx)
            .streams
            .offset((*writer).video_stream_idx as isize);
        if !out_stream.is_null() {
            av_packet_rescale_ts(out_pkt, (*input_stream).time_base, (*out_stream).time_base);
        }
    } else if (*(*input_stream).codecpar).codec_type.0 == AVMediaType::AVMEDIA_TYPE_AUDIO.0 {
        if (*writer).audio.stream_idx >= 0 as ::core::ffi::c_int {
            (*out_pkt).stream_index = (*writer).audio.stream_idx;
            let mut packet_time_base: AVRational = if (*pkt).time_base.num > 0 as ::core::ffi::c_int
                && (*pkt).time_base.den > 0 as ::core::ffi::c_int
            {
                (*pkt).time_base
            } else {
                (*input_stream).time_base
            };
            let mut transcoded_pcm: bool =
                is_pcm_codec((*(*input_stream).codecpar).codec_id) as ::core::ffi::c_int != 0
                    && (*pkt).time_base.num > 0 as ::core::ffi::c_int
                    && (*pkt).time_base.den > 0 as ::core::ffi::c_int;
            if !transcoded_pcm
                && (*writer).audio.first_dts == AV_NOPTS_VALUE
                && (*out_pkt).dts != AV_NOPTS_VALUE
            {
                (*writer).audio.first_dts = (*out_pkt).dts;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] First audio DTS: %lld (for timestamp remapping)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*writer).stream_name as *mut ::core::ffi::c_char,
                    (*writer).audio.first_dts as ::core::ffi::c_longlong,
                );
            }
            if !transcoded_pcm
                && (*writer).audio.first_dts != AV_NOPTS_VALUE
                && (*out_pkt).dts != AV_NOPTS_VALUE
            {
                (*out_pkt).dts -= (*writer).audio.first_dts;
                if (*out_pkt).dts < 0 as int64_t {
                    (*out_pkt).dts = 0 as int64_t;
                }
            }
            if !transcoded_pcm
                && (*writer).audio.first_dts != AV_NOPTS_VALUE
                && (*out_pkt).pts != AV_NOPTS_VALUE
            {
                (*out_pkt).pts -= (*writer).audio.first_dts;
                if (*out_pkt).pts < 0 as int64_t {
                    (*out_pkt).pts = 0 as int64_t;
                }
            }
            if (*out_pkt).dts != AV_NOPTS_VALUE
                && (*writer).audio.last_dts != AV_NOPTS_VALUE
                && (*out_pkt).dts <= (*writer).audio.last_dts
            {
                let mut fixed_dts_0: int64_t = (*writer).audio.last_dts + 1 as int64_t;
                if (*out_pkt).pts != AV_NOPTS_VALUE {
                    let mut pts_dts_diff_0: int64_t = (*out_pkt).pts - (*out_pkt).dts;
                    (*out_pkt).dts = fixed_dts_0;
                    (*out_pkt).pts = fixed_dts_0
                        + if pts_dts_diff_0 > 0 as int64_t {
                            pts_dts_diff_0
                        } else {
                            0 as int64_t
                        };
                } else {
                    (*out_pkt).dts = fixed_dts_0;
                    (*out_pkt).pts = fixed_dts_0;
                }
            }
            (*writer).audio.last_dts = (*out_pkt).dts;
            if (*out_pkt).duration == 0 as int64_t || (*out_pkt).duration == AV_NOPTS_VALUE {
                if transcoded_pcm as ::core::ffi::c_int != 0
                    && (*(**(*(*writer).output_ctx)
                        .streams
                        .offset((*writer).audio.stream_idx as isize))
                    .codecpar)
                        .frame_size
                        > 0 as ::core::ffi::c_int
                    && (*(**(*(*writer).output_ctx)
                        .streams
                        .offset((*writer).audio.stream_idx as isize))
                    .codecpar)
                        .sample_rate
                        > 0 as ::core::ffi::c_int
                {
                    (*out_pkt).duration = (*(**(*(*writer).output_ctx)
                        .streams
                        .offset((*writer).audio.stream_idx as isize))
                    .codecpar)
                        .frame_size as int64_t;
                } else if (*(*input_stream).codecpar).frame_size > 0 as ::core::ffi::c_int
                    && (*(*input_stream).codecpar).sample_rate > 0 as ::core::ffi::c_int
                {
                    (*out_pkt).duration = av_rescale_q(
                        (*(*input_stream).codecpar).frame_size as int64_t,
                        AVRational {
                            num: 1 as ::core::ffi::c_int,
                            den: (*(*input_stream).codecpar).sample_rate,
                        },
                        (*input_stream).time_base,
                    );
                } else {
                    (*out_pkt).duration = 1 as int64_t;
                }
            }
            let mut out_stream_0: *mut AVStream = *(*(*writer).output_ctx)
                .streams
                .offset((*writer).audio.stream_idx as isize);
            if !out_stream_0.is_null() {
                av_packet_rescale_ts(out_pkt, packet_time_base, (*out_stream_0).time_base);
            }
        } else {
            av_packet_free(&raw mut out_pkt);
            return 0 as ::core::ffi::c_int;
        }
    } else {
        av_packet_free(&raw mut out_pkt);
        return 0 as ::core::ffi::c_int;
    }
    ret = av_interleaved_write_frame((*writer).output_ctx, out_pkt);
    if ret < 0 as ::core::ffi::c_int {
        recording_io_report_failure(
            recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
            recording_io_operation_t::RECORDING_IO_OPERATION_PACKET,
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
            b"Error writing frame for stream %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if !(&raw mut (*writer).stream_name as *mut ::core::ffi::c_char).is_null() {
                &raw mut (*writer).stream_name as *mut ::core::ffi::c_char
            } else {
                b"unknown\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut error_buf_0 as *mut ::core::ffi::c_char,
        );
    }
    av_packet_free(&raw mut out_pkt);
    return ret;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
