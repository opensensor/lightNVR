unsafe extern "C" {
    pub type AVOptionRanges;
    pub type AVOption;
    pub type AVBuffer;
    pub type AVDictionary;
    pub type AVIAMFMixPresentation;
    pub type AVIAMFAudioElement;
    pub type AVCodecTag;
    pub type AVCodecInternal;
    pub type stream_handle_s;
    unsafe fn remove(__filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn av_strerror(
        errnum: ::core::ffi::c_int,
        errbuf: *mut ::core::ffi::c_char,
        errbuf_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn av_free(ptr: *mut ::core::ffi::c_void);
    unsafe fn av_log_set_level(level: ::core::ffi::c_int);
    unsafe fn av_packet_alloc() -> *mut AVPacket;
    unsafe fn av_packet_free(pkt: *mut *mut AVPacket);
    unsafe fn av_packet_unref(pkt: *mut AVPacket);
    unsafe fn avformat_network_deinit() -> ::core::ffi::c_int;
    unsafe fn av_read_frame(s: *mut AVFormatContext, pkt: *mut AVPacket) -> ::core::ffi::c_int;
    unsafe fn avformat_close_input(s: *mut *mut AVFormatContext);
    unsafe fn av_usleep(usec: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn log_set_thread_context(
        component: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    );
    static mut g_config: config_t;
    unsafe fn url_apply_credentials(
        url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn rtsp_probe_server(
        url: *const ::core::ffi::c_char,
        host: *mut ::core::ffi::c_char,
        host_size: size_t,
        port: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn register_component(
        name: *const ::core::ffi::c_char,
        r#type: component_type_t,
        context: *mut ::core::ffi::c_void,
        priority: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn update_component_state(component_id: ::core::ffi::c_int, state: component_state_t);
    unsafe fn is_shutdown_initiated() -> bool;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn metrics_record_frame_from_source(
        stream_name: *const ::core::ffi::c_char,
        bytes: ::core::ffi::c_int,
        is_video: bool,
        source: metrics_frame_source_t,
    );
    unsafe fn go2rtc_integration_is_using_go2rtc_for_hls(
        stream_name: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn go2rtc_get_rtsp_url(
        stream_name: *const ::core::ffi::c_char,
        url: *mut ::core::ffi::c_char,
        url_size: size_t,
    ) -> bool;
    unsafe fn get_stream_config(
        handle: stream_handle_t,
        config: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_by_name(name: *const ::core::ffi::c_char) -> stream_handle_t;
    unsafe fn get_stream_state_by_name(
        name: *const ::core::ffi::c_char,
    ) -> *mut stream_state_manager_t;
    unsafe fn stream_state_add_ref(
        state: *mut stream_state_manager_t,
        component: stream_component_t,
    ) -> ::core::ffi::c_int;
    unsafe fn stream_state_release_ref(
        state: *mut stream_state_manager_t,
        component: stream_component_t,
    ) -> ::core::ffi::c_int;
    unsafe fn is_stream_state_stopping(state: *mut stream_state_manager_t) -> bool;
    unsafe fn set_stream_callbacks_enabled(
        state: *mut stream_state_manager_t,
        enabled: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn are_stream_callbacks_enabled(state: *mut stream_state_manager_t) -> bool;
    unsafe fn hls_writer_create(
        output_dir: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
        segment_duration: ::core::ffi::c_int,
    ) -> *mut hls_writer_t;
    unsafe fn hls_writer_initialize_av(
        writer: *mut hls_writer_t,
        video_stream: *const AVStream,
        audio_stream: *const AVStream,
    ) -> ::core::ffi::c_int;
    unsafe fn hls_writer_write_packet(
        writer: *mut hls_writer_t,
        pkt: *const AVPacket,
        input_stream: *const AVStream,
    ) -> ::core::ffi::c_int;
    unsafe fn hls_writer_close(writer: *mut hls_writer_t);
    unsafe fn cleanup_all_hls_writers();
    unsafe fn is_stream_stopping(stream_name: *const ::core::ffi::c_char) -> bool;
    unsafe fn mark_stream_stopping(stream_name: *const ::core::ffi::c_char);
    unsafe fn unmark_stream_stopping(stream_name: *const ::core::ffi::c_char);
    unsafe fn ensure_hls_directory(
        output_dir: *mut ::core::ffi::c_char,
        output_dir_size: size_t,
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn clear_stream_hls_segments(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn open_input_stream(
        input_ctx: *mut *mut AVFormatContext,
        url: *const ::core::ffi::c_char,
        protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn find_video_stream_index(input_ctx: *const AVFormatContext) -> ::core::ffi::c_int;
    unsafe fn find_audio_stream_index(input_ctx: *const AVFormatContext) -> ::core::ffi::c_int;
    unsafe fn start_hls_stream(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn stop_hls_stream(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn comprehensive_ffmpeg_cleanup(
        input_ctx: *mut *mut AVFormatContext,
        codec_ctx: *mut *mut AVCodecContext,
        packet: *mut *mut AVPacket,
        frame: *mut *mut AVFrame,
    );
    unsafe fn reset_timestamp_tracker(stream_name: *const ::core::ffi::c_char);
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
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
    pub const PTHREAD_CREATE_JOINABLE: Self = Self(0);
    pub const PTHREAD_CREATE_DETACHED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int_fast64_t = ::core::ffi::c_long;
pub type atomic_bool = bool;
pub type atomic_int = ::core::ffi::c_int;
pub type atomic_int_fast64_t = int_fast64_t;
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
    pub u: C2Rust_Unnamed_1,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
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
    pub params: C2Rust_Unnamed_2,
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
pub union C2Rust_Unnamed_2 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_3,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
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
    pub offsets: *mut C2Rust_Unnamed_4,
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
pub struct C2Rust_Unnamed_4 {
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct component_state_t(pub ::core::ffi::c_uint);
impl component_state_t {
    pub const COMPONENT_RUNNING: Self = Self(0);
    pub const COMPONENT_STOPPING: Self = Self(1);
    pub const COMPONENT_STOPPED: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct component_type_t(pub ::core::ffi::c_uint);
impl component_type_t {
    pub const COMPONENT_DETECTION_THREAD: Self = Self(0);
    pub const COMPONENT_SERVER_THREAD: Self = Self(1);
    pub const COMPONENT_HLS_WRITER: Self = Self(2);
    pub const COMPONENT_MP4_WRITER: Self = Self(3);
    pub const COMPONENT_OTHER: Self = Self(4);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hls_unified_thread_ctx_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub rtsp_url: [::core::ffi::c_char; 512],
    pub output_path: [::core::ffi::c_char; 512],
    pub thread: pthread_t,
    pub running: atomic_int,
    pub shutdown_component_id: ::core::ffi::c_int,
    pub protocol: ::core::ffi::c_int,
    pub segment_duration: ::core::ffi::c_int,
    pub audio_enabled: ::core::ffi::c_int,
    pub writer: *mut hls_writer_t,
    pub last_packet_time: atomic_int_fast64_t,
    pub connection_valid: atomic_int,
    pub consecutive_failures: atomic_int,
    pub thread_state: atomic_int,
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
pub struct context_deletion_info_t {
    pub ctx: *mut ::core::ffi::c_void,
    pub pending_deletion: atomic_int,
    pub thread_exited: atomic_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_state_manager_t {
    pub name: [::core::ffi::c_char; 256],
    pub state: stream_state_t,
    pub features: stream_features_t,
    pub protocol_state: stream_protocol_state_t,
    pub timestamp_state: stream_timestamp_state_t,
    pub stats: stream_stats_t,
    pub config: stream_config_t,
    pub mutex: pthread_mutex_t,
    pub state_mutex: pthread_mutex_t,
    pub ref_count: ::core::ffi::c_int,
    pub component_refs: [::core::ffi::c_int; 6],
    pub reader_ctx: *mut ::core::ffi::c_void,
    pub hls_ctx: *mut ::core::ffi::c_void,
    pub mp4_ctx: *mut ::core::ffi::c_void,
    pub detection_ctx: *mut ::core::ffi::c_void,
    pub callbacks_enabled: bool,
    pub last_error_message: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_stats_t {
    pub bytes_received: uint64_t,
    pub frames_received: uint64_t,
    pub frames_dropped: uint64_t,
    pub errors: uint64_t,
    pub reconnects: uint64_t,
    pub bitrate: ::core::ffi::c_double,
    pub fps: ::core::ffi::c_double,
    pub uptime: uint64_t,
    pub last_frame_time: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_timestamp_state_t {
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub expected_next_pts: int64_t,
    pub pts_discontinuity_count: int64_t,
    pub timestamps_initialized: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_protocol_state_t {
    pub protocol: stream_protocol_t,
    pub is_multicast: bool,
    pub reconnect_attempts: ::core::ffi::c_int,
    pub last_reconnect_time: int64_t,
    pub buffer_size: ::core::ffi::c_int,
    pub timeout_ms: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_features_t {
    pub streaming_enabled: bool,
    pub recording_enabled: bool,
    pub detection_enabled: bool,
    pub motion_detection_enabled: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_state_t(pub ::core::ffi::c_uint);
impl stream_state_t {
    pub const STREAM_STATE_INACTIVE: Self = Self(0);
    pub const STREAM_STATE_STARTING: Self = Self(1);
    pub const STREAM_STATE_ACTIVE: Self = Self(2);
    pub const STREAM_STATE_STOPPING: Self = Self(3);
    pub const STREAM_STATE_ERROR: Self = Self(4);
    pub const STREAM_STATE_RECONNECTING: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct hls_thread_state_t(pub ::core::ffi::c_uint);
impl hls_thread_state_t {
    pub const HLS_THREAD_INITIALIZING: Self = Self(0);
    pub const HLS_THREAD_CONNECTING: Self = Self(1);
    pub const HLS_THREAD_RUNNING: Self = Self(2);
    pub const HLS_THREAD_RECONNECTING: Self = Self(3);
    pub const HLS_THREAD_STOPPING: Self = Self(4);
    pub const HLS_THREAD_STOPPED: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_component_t(pub ::core::ffi::c_uint);
impl stream_component_t {
    pub const STREAM_COMPONENT_READER: Self = Self(0);
    pub const STREAM_COMPONENT_HLS: Self = Self(1);
    pub const STREAM_COMPONENT_MP4: Self = Self(2);
    pub const STREAM_COMPONENT_DETECTION: Self = Self(3);
    pub const STREAM_COMPONENT_API: Self = Self(4);
    pub const STREAM_COMPONENT_OTHER: Self = Self(5);
    pub const STREAM_COMPONENT_COUNT: Self = Self(6);
}
pub type stream_handle_t = *mut stream_handle_s;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const AV_ERROR_MAX_STRING_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const AV_LOG_QUIET: ::core::ffi::c_int = -8 as ::core::ffi::c_int;
pub const AV_PKT_FLAG_KEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
static mut ffmpeg_memory_cleanup_registered: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn global_ffmpeg_cleanup() {
    static mut in_global_cleanup: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if in_global_cleanup != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Already in global FFmpeg cleanup, skipping to prevent double free\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    in_global_cleanup = 1 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Global FFmpeg cleanup called during program exit\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Calling avformat_network_deinit() during global cleanup\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    avformat_network_deinit();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Setting FFmpeg log level to quiet during global cleanup\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    av_log_set_level(AV_LOG_QUIET);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Using safer approach to release FFmpeg memory during global cleanup\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut dummy: *mut ::core::ffi::c_void = av_malloc(16 as size_t);
    if !dummy.is_null() {
        av_free(dummy);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Global FFmpeg cleanup completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    in_global_cleanup = 0 as ::core::ffi::c_int;
}
pub const MAX_PACKET_TIMEOUT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const BASE_RECONNECT_DELAY_MS: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const MAX_RECONNECT_DELAY_MS: ::core::ffi::c_int = 30000 as ::core::ffi::c_int;
pub const MAX_FREED_CONTEXTS: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
static mut freed_contexts: [*mut ::core::ffi::c_void; 100] = [
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
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ::core::ptr::null_mut::<::core::ffi::c_void>(),
];
static mut freed_contexts_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut freed_contexts_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_0::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut pending_deletions: [context_deletion_info_t; 1024] = [
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
    context_deletion_info_t {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pending_deletion: 0,
        thread_exited: 0,
    },
];
static mut pending_deletions_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_0::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
pub const MAX_THREAD_EXIT_WAIT_US: ::core::ffi::c_int = 2000000 as ::core::ffi::c_int;
pub const MEMORY_GUARD_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MEMORY_GUARD_PATTERN: ::core::ffi::c_int = 0xfe as ::core::ffi::c_int;
pub const WATCHDOG_CHECK_INTERVAL_SEC: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const WATCHDOG_MAX_RESTART_ATTEMPTS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const WATCHDOG_RESTART_COOLDOWN_SEC: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
unsafe extern "C" fn hls_guarded_malloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut total_size: size_t = size.wrapping_add(
        (2 as ::core::ffi::c_int as size_t).wrapping_mul(MEMORY_GUARD_SIZE as size_t),
    );
    let mut mem: *mut ::core::ffi::c_uchar = malloc(total_size) as *mut ::core::ffi::c_uchar;
    if mem.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory of size %zu\0".as_ptr() as *const ::core::ffi::c_char,
            size,
        );
        return NULL;
    }
    memset(
        mem as *mut ::core::ffi::c_void,
        MEMORY_GUARD_PATTERN,
        MEMORY_GUARD_SIZE as size_t,
    );
    memset(
        mem.offset(MEMORY_GUARD_SIZE as isize).offset(size as isize) as *mut ::core::ffi::c_void,
        MEMORY_GUARD_PATTERN,
        MEMORY_GUARD_SIZE as size_t,
    );
    return mem.offset(MEMORY_GUARD_SIZE as isize) as *mut ::core::ffi::c_void;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hls_guarded_free(
    mut ptr: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    if ptr.is_null() {
        return NULL;
    }
    let mut mem: *mut ::core::ffi::c_uchar =
        (ptr as *mut ::core::ffi::c_uchar).offset(-(MEMORY_GUARD_SIZE as isize));
    let mut guard_corrupted: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MEMORY_GUARD_SIZE {
        if *mem.offset(i as isize) as ::core::ffi::c_int != MEMORY_GUARD_PATTERN {
            guard_corrupted = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    if guard_corrupted {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory corruption detected: guard bytes have been overwritten\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    free(mem as *mut ::core::ffi::c_void);
    return NULL;
}
unsafe extern "C" fn is_context_already_freed(mut ctx: *mut ::core::ffi::c_void) -> bool {
    if ctx.is_null() {
        return r#false != 0;
    }
    let mut result: bool = r#false != 0;
    pthread_mutex_lock(&raw mut freed_contexts_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < freed_contexts_count {
        if freed_contexts[i as usize] == ctx {
            result = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut freed_contexts_mutex);
    return result;
}
unsafe extern "C" fn mark_context_pending_deletion(mut ctx: *mut ::core::ffi::c_void) {
    pthread_mutex_lock(&raw mut pending_deletions_mutex);
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if pending_deletions[i as usize].ctx == ctx {
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
                    .offset(i as isize))
                .pending_deletion,
                1 as ::core::ffi::c_int,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
                    .offset(i as isize))
                .thread_exited,
                0 as ::core::ffi::c_int,
            );
            slot = i;
            break;
        } else {
            if pending_deletions[i as usize].ctx.is_null() && slot == -1 as ::core::ffi::c_int {
                slot = i;
            }
            i += 1;
        }
    }
    if slot != -1 as ::core::ffi::c_int && pending_deletions[slot as usize].ctx != ctx {
        pending_deletions[slot as usize].ctx = ctx;
        *&raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
            .offset(slot as isize))
        .pending_deletion = 1 as ::core::ffi::c_int;
        *&raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
            .offset(slot as isize))
        .thread_exited = 0 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut pending_deletions_mutex);
}
unsafe extern "C" fn is_context_pending_deletion(mut ctx: *const ::core::ffi::c_void) -> bool {
    if ctx.is_null() {
        return r#false != 0;
    }
    let mut result: bool = r#false != 0;
    pthread_mutex_lock(&raw mut pending_deletions_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if pending_deletions[i as usize].ctx == ctx as *mut ::core::ffi::c_void
            && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
                    .offset(i as isize))
                .pending_deletion,
            ) != 0
        {
            result = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut pending_deletions_mutex);
    return result;
}
unsafe extern "C" fn mark_thread_exited(mut ctx: *mut ::core::ffi::c_void) {
    if ctx.is_null() {
        return;
    }
    if is_context_already_freed(ctx) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Attempted to mark thread as exited for already freed context %p\0".as_ptr()
                as *const ::core::ffi::c_char,
            ctx,
        );
        return;
    }
    pthread_mutex_lock(&raw mut pending_deletions_mutex);
    let mut found: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if pending_deletions[i as usize].ctx == ctx {
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
                    .offset(i as isize))
                .thread_exited,
                1 as ::core::ffi::c_int,
            );
            found = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Marked thread as exited for context %p\0".as_ptr() as *const ::core::ffi::c_char,
                ctx,
            );
            break;
        } else {
            i += 1;
        }
    }
    if !found {
        let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < g_config.max_streams {
            if pending_deletions[i_0 as usize].ctx.is_null() {
                slot = i_0;
                break;
            } else {
                i_0 += 1;
            }
        }
        if slot != -1 as ::core::ffi::c_int {
            if !is_context_already_freed(ctx) {
                pending_deletions[slot as usize].ctx = ctx;
                *&raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
                    .offset(slot as isize))
                .pending_deletion = 0 as ::core::ffi::c_int;
                *&raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
                    .offset(slot as isize))
                .thread_exited = 1 as ::core::ffi::c_int;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Added context %p to pending deletions list and marked thread as exited\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    ctx,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Context %p was freed during mark_thread_exited, not adding to list\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ctx,
                );
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"No empty slot found in pending deletions list for context %p\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ctx,
            );
        }
    }
    pthread_mutex_unlock(&raw mut pending_deletions_mutex);
}
unsafe extern "C" fn has_thread_exited(mut ctx: *const ::core::ffi::c_void) -> bool {
    let mut result: bool = r#false != 0;
    pthread_mutex_lock(&raw mut pending_deletions_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if pending_deletions[i as usize].ctx == ctx as *mut ::core::ffi::c_void {
            result = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
                    .offset(i as isize))
                .thread_exited,
            ) != 0;
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut pending_deletions_mutex);
    return result;
}
unsafe extern "C" fn wait_for_thread_exit(mut ctx: *mut ::core::ffi::c_void) {
    if ctx.is_null() {
        return;
    }
    if is_context_already_freed(ctx) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Context %p is already freed, skipping thread exit wait\0".as_ptr()
                as *const ::core::ffi::c_char,
            ctx,
        );
        mark_thread_exited(ctx);
        return;
    }
    let mut wait_time: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let sleep_interval: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
    if has_thread_exited(ctx) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Thread for context %p has already exited\0".as_ptr() as *const ::core::ffi::c_char,
            ctx,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Waiting for thread to exit for context %p\0".as_ptr() as *const ::core::ffi::c_char,
        ctx,
    );
    ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
    while wait_time < MAX_THREAD_EXIT_WAIT_US {
        if is_context_already_freed(ctx) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Context %p was freed during wait_for_thread_exit\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ctx,
            );
            mark_thread_exited(ctx);
            return;
        }
        if has_thread_exited(ctx) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Thread for context %p has exited after waiting %d ms\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ctx,
                wait_time / 1000 as ::core::ffi::c_int,
            );
            return;
        }
        usleep(sleep_interval as __useconds_t);
        wait_time += sleep_interval;
        if wait_time % 100000 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Still waiting for thread to exit for context %p (%d ms elapsed)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ctx,
                wait_time / 1000 as ::core::ffi::c_int,
            );
            if !has_thread_exited(ctx) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Re-signaling thread for context %p to exit\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ctx,
                );
                mark_context_pending_deletion(ctx);
                ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>(
                );
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Timeout waiting for thread to exit for context %p after %d ms\0".as_ptr()
            as *const ::core::ffi::c_char,
        ctx,
        2000000 as ::core::ffi::c_int / 1000 as ::core::ffi::c_int,
    );
    if !is_context_already_freed(ctx) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Forcing thread exited status for context %p to prevent deadlock\0".as_ptr()
                as *const ::core::ffi::c_char,
            ctx,
        );
        mark_thread_exited(ctx);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Context %p was freed during wait_for_thread_exit timeout\0".as_ptr()
                as *const ::core::ffi::c_char,
            ctx,
        );
    };
}
unsafe extern "C" fn clear_context_pending_deletion(mut ctx: *const ::core::ffi::c_void) {
    pthread_mutex_lock(&raw mut pending_deletions_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if pending_deletions[i as usize].ctx == ctx as *mut ::core::ffi::c_void {
            pending_deletions[i as usize].ctx = NULL;
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
                    .offset(i as isize))
                .pending_deletion,
                0 as ::core::ffi::c_int,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
                    .offset(i as isize))
                .thread_exited,
                0 as ::core::ffi::c_int,
            );
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut pending_deletions_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_context_as_freed(mut ctx: *mut ::core::ffi::c_void) {
    if ctx.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut freed_contexts_mutex);
    let mut already_freed: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < freed_contexts_count {
        if freed_contexts[i as usize] == ctx {
            already_freed = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    if already_freed {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Context %p is already marked as freed\0".as_ptr() as *const ::core::ffi::c_char,
            ctx,
        );
        pthread_mutex_unlock(&raw mut freed_contexts_mutex);
        return;
    }
    if freed_contexts_count >= MAX_FREED_CONTEXTS {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < MAX_FREED_CONTEXTS - 1 as ::core::ffi::c_int {
            freed_contexts[i_0 as usize] = freed_contexts[(i_0 + 1 as ::core::ffi::c_int) as usize];
            i_0 += 1;
        }
        freed_contexts_count = MAX_FREED_CONTEXTS - 1 as ::core::ffi::c_int;
    }
    let c2rust_fresh1 = freed_contexts_count;
    freed_contexts_count += 1;
    freed_contexts[c2rust_fresh1 as usize] = ctx;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Marked context %p as freed\0".as_ptr() as *const ::core::ffi::c_char,
        ctx,
    );
    pthread_mutex_unlock(&raw mut freed_contexts_mutex);
}
#[thread_local]
static mut in_safe_cleanup: bool = r#false != 0;
unsafe extern "C" fn safe_cleanup_resources(
    mut input_ctx: *mut *mut AVFormatContext,
    mut pkt: *mut *mut AVPacket,
    mut writer: *mut *mut hls_writer_t,
) {
    if in_safe_cleanup {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Recursive call to safe_cleanup_resources detected, aborting to prevent double free\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    in_safe_cleanup = r#true != 0;
    if input_ctx.is_null() && pkt.is_null() && writer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"All parameters to safe_cleanup_resources are NULL, nothing to clean up\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        in_safe_cleanup = r#false != 0;
        return;
    }
    if !pkt.is_null() {
        let mut pkt_to_free: *mut AVPacket = ::core::ptr::null_mut::<AVPacket>();
        if !(*pkt).is_null() {
            pkt_to_free = *pkt;
            *pkt = ::core::ptr::null_mut::<AVPacket>();
            ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Safely unreferencing packet during cleanup\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            av_packet_unref(pkt_to_free);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Safely freeing packet during cleanup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
            av_packet_free(&raw mut pkt_to_free);
        }
    }
    if !input_ctx.is_null() {
        let mut ctx_to_close: *mut AVFormatContext = ::core::ptr::null_mut::<AVFormatContext>();
        if !(*input_ctx).is_null() {
            ctx_to_close = *input_ctx;
            *input_ctx = ::core::ptr::null_mut::<AVFormatContext>();
            ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Safely closing input context during cleanup\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Using comprehensive cleanup for input context\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            let mut ctx_to_cleanup: *mut AVFormatContext = ctx_to_close;
            ctx_to_close = ::core::ptr::null_mut::<AVFormatContext>();
            comprehensive_ffmpeg_cleanup(
                &raw mut ctx_to_cleanup,
                ::core::ptr::null_mut::<*mut AVCodecContext>(),
                ::core::ptr::null_mut::<*mut AVPacket>(),
                ::core::ptr::null_mut::<*mut AVFrame>(),
            );
            if !ctx_to_cleanup.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to clean up context, forcing NULL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                ctx_to_cleanup = ::core::ptr::null_mut::<AVFormatContext>();
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Successfully cleaned up input context\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    if !writer.is_null() {
        let mut writer_to_free: *mut hls_writer_t =
            ::core::intrinsics::atomic_xchg::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                writer,
                ::core::ptr::null_mut::<hls_writer_t>(),
            );
        if !writer_to_free.is_null() {
            hls_writer_close(writer_to_free);
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Completed safe cleanup of resources\0".as_ptr() as *const ::core::ffi::c_char,
    );
    in_safe_cleanup = r#false != 0;
}
unsafe extern "C" fn calculate_reconnect_delay(
    mut attempt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if attempt <= 0 as ::core::ffi::c_int {
        return BASE_RECONNECT_DELAY_MS;
    }
    let mut delay: ::core::ffi::c_int = BASE_RECONNECT_DELAY_MS;
    while attempt > 1 as ::core::ffi::c_int && delay < MAX_RECONNECT_DELAY_MS {
        if delay >= MAX_RECONNECT_DELAY_MS / 2 as ::core::ffi::c_int {
            return MAX_RECONNECT_DELAY_MS;
        }
        delay *= 2 as ::core::ffi::c_int;
        attempt -= 1;
    }
    return delay;
}
unsafe extern "C" fn interruptible_sleep_ms(
    mut total_ms: ::core::ffi::c_int,
    mut ctx: *mut hls_unified_thread_ctx_t,
    mut state: *mut stream_state_manager_t,
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let interval_ms: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
    let mut elapsed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while elapsed < total_ms {
        let mut sleep_ms: ::core::ffi::c_int = if total_ms - elapsed < interval_ms {
            total_ms - elapsed
        } else {
            interval_ms
        };
        av_usleep((sleep_ms * 1000 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        elapsed += sleep_ms;
        if is_context_already_freed(ctx as *mut ::core::ffi::c_void) as ::core::ffi::c_int != 0
            || is_context_pending_deletion(ctx as *const ::core::ffi::c_void) as ::core::ffi::c_int
                != 0
            || ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).running,
            ) == 0
            || is_shutdown_initiated() as ::core::ffi::c_int != 0
            || !state.is_null()
                && (is_stream_state_stopping(state) as ::core::ffi::c_int != 0
                    || !are_stream_callbacks_enabled(state))
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Interruptible sleep for %s interrupted after %d/%d ms due to %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                elapsed,
                total_ms,
                if is_context_already_freed(ctx as *mut ::core::ffi::c_void) as ::core::ffi::c_int
                    != 0
                {
                    b"context already freed\0".as_ptr() as *const ::core::ffi::c_char
                } else if is_context_pending_deletion(ctx as *const ::core::ffi::c_void)
                    as ::core::ffi::c_int
                    != 0
                {
                    b"context pending deletion\0".as_ptr() as *const ::core::ffi::c_char
                } else if ::core::intrinsics::atomic_load::<
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(&raw mut (*ctx).running)
                    == 0
                {
                    b"running flag cleared\0".as_ptr() as *const ::core::ffi::c_char
                } else if is_shutdown_initiated() as ::core::ffi::c_int != 0 {
                    b"system shutdown\0".as_ptr() as *const ::core::ffi::c_char
                } else if !state.is_null()
                    && is_stream_state_stopping(state) as ::core::ffi::c_int != 0
                {
                    b"stream state STOPPING\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"callbacks disabled\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hls_unified_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut ctx: *mut hls_unified_thread_ctx_t = arg as *mut hls_unified_thread_ctx_t;
    let mut input_ctx: *mut AVFormatContext = ::core::ptr::null_mut::<AVFormatContext>();
    let mut pkt: *mut AVPacket = ::core::ptr::null_mut::<AVPacket>();
    let mut video_stream_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut audio_stream_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0;
    let mut thread_state: hls_thread_state_t = hls_thread_state_t::HLS_THREAD_INITIALIZING;
    let mut reconnect_attempt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut reconnect_delay_ms: ::core::ffi::c_int = BASE_RECONNECT_DELAY_MS;
    let mut last_packet_time: time_t = 0 as time_t;
    if ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"NULL context passed to unified HLS thread\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    if is_context_pending_deletion(ctx as *const ::core::ffi::c_void) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Context is already marked for deletion, exiting thread\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    if is_shutdown_initiated() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unified HLS thread exiting immediately due to system shutdown\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        if !ctx.is_null() {
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).running,
                0 as ::core::ffi::c_int,
            );
            if (*ctx).shutdown_component_id >= 0 as ::core::ffi::c_int {
                update_component_state(
                    (*ctx).shutdown_component_id,
                    component_state_t::COMPONENT_STOPPED,
                );
            }
        }
        return NULL;
    }
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    log_set_thread_context(
        b"HLSWriter\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting unified HLS thread for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).running,
    ) == 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unified HLS thread for %s started but already marked as not running\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        return NULL;
    }
    let mut state: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut stream_name as *mut ::core::ffi::c_char);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not find stream state for %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).running,
            0 as ::core::ffi::c_int,
        );
        return NULL;
    }
    let mut component_name: [::core::ffi::c_char; 128] = [0; 128];
    snprintf(
        &raw mut component_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        b"hls_unified_%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    (*ctx).shutdown_component_id = register_component(
        &raw mut component_name as *mut ::core::ffi::c_char,
        component_type_t::COMPONENT_HLS_WRITER,
        ctx as *mut ::core::ffi::c_void,
        60 as ::core::ffi::c_int,
    );
    if (*ctx).shutdown_component_id >= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Registered unified HLS thread %s with shutdown coordinator (ID: %d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            (*ctx).shutdown_component_id,
        );
    }
    pkt = av_packet_alloc() as *mut AVPacket;
    if pkt.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate packet for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).running,
            0 as ::core::ffi::c_int,
        );
        if (*ctx).shutdown_component_id >= 0 as ::core::ffi::c_int {
            update_component_state(
                (*ctx).shutdown_component_id,
                component_state_t::COMPONENT_STOPPED,
            );
        }
        return NULL;
    }
    if !state.is_null()
        && !(*state).hls_ctx.is_null()
        && (*state).hls_ctx != (*ctx).writer as *mut ::core::ffi::c_void
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream state for %s already has an HLS writer. Replacing it.\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    let mut seg_duration: ::core::ffi::c_int = if (*ctx).segment_duration > 0 as ::core::ffi::c_int
    {
        (*ctx).segment_duration
    } else {
        2 as ::core::ffi::c_int
    };
    (*ctx).writer = hls_writer_create(
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        seg_duration,
    );
    if (*ctx).writer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create HLS writer for %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        safe_cleanup_resources(
            ::core::ptr::null_mut::<*mut AVFormatContext>(),
            &raw mut pkt,
            ::core::ptr::null_mut::<*mut hls_writer_t>(),
        );
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).running,
            0 as ::core::ffi::c_int,
        );
        if (*ctx).shutdown_component_id >= 0 as ::core::ffi::c_int {
            update_component_state(
                (*ctx).shutdown_component_id,
                component_state_t::COMPONENT_STOPPED,
            );
        }
        return NULL;
    }
    if !state.is_null() {
        (*state).hls_ctx = (*ctx).writer as *mut ::core::ffi::c_void;
    }
    let mut has_written_packets: bool = r#false != 0;
    let mut ctx_safe: *mut hls_unified_thread_ctx_t = ctx;
    let mut local_rtsp_url: [::core::ffi::c_char; 512] = [0; 512];
    let mut local_protocol: ::core::ffi::c_int = 0;
    let mut input_stream: *const AVStream = ::core::ptr::null::<AVStream>();
    let mut is_audio_pkt: bool = false;
    let mut now: time_t = 0;
    let mut reconnect_rtsp_url: [::core::ffi::c_char; 512] = [0; 512];
    let mut reconnect_protocol: ::core::ffi::c_int = 0;
    while !ctx.is_null()
        && !ctx_safe.is_null()
        && !is_context_already_freed(ctx_safe as *mut ::core::ffi::c_void)
        && !is_context_pending_deletion(ctx_safe as *const ::core::ffi::c_void)
    {
        ctx_safe = ctx;
        if ctx_safe.is_null()
            || is_context_already_freed(ctx_safe as *mut ::core::ffi::c_void) as ::core::ffi::c_int
                != 0
            || is_context_pending_deletion(ctx_safe as *const ::core::ffi::c_void)
                as ::core::ffi::c_int
                != 0
            || !ctx_safe.is_null()
                && ::core::intrinsics::atomic_load::<
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(&raw mut (*ctx_safe).running)
                    == 0
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Unified HLS thread for %s stopping due to %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
                if ctx_safe.is_null() {
                    b"context is NULL\0".as_ptr() as *const ::core::ffi::c_char
                } else if is_context_already_freed(ctx_safe as *mut ::core::ffi::c_void)
                    as ::core::ffi::c_int
                    != 0
                {
                    b"context already freed\0".as_ptr() as *const ::core::ffi::c_char
                } else if is_context_pending_deletion(ctx_safe as *const ::core::ffi::c_void)
                    as ::core::ffi::c_int
                    != 0
                {
                    b"context pending deletion\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"running flag cleared\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            break;
        } else {
            if !ctx_safe.is_null()
                && !is_context_already_freed(ctx_safe as *mut ::core::ffi::c_void)
                && !is_context_pending_deletion(ctx_safe as *const ::core::ffi::c_void)
            {
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx_safe).thread_state,
                    thread_state.0 as ::core::ffi::c_int,
                );
            }
            if is_shutdown_initiated() as ::core::ffi::c_int != 0
                || is_stream_state_stopping(state) as ::core::ffi::c_int != 0
                || !are_stream_callbacks_enabled(state)
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Unified HLS thread for %s stopping due to %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                    if is_shutdown_initiated() as ::core::ffi::c_int != 0 {
                        b"system shutdown\0".as_ptr() as *const ::core::ffi::c_char
                    } else if is_stream_state_stopping(state) as ::core::ffi::c_int != 0 {
                        b"stream state STOPPING\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"callbacks disabled\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
            }
            if !state.is_null() && !(*state).features.streaming_enabled {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Unified HLS thread for %s stopping due to streaming disabled\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
                thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
            }
            match thread_state {
                hls_thread_state_t::HLS_THREAD_INITIALIZING => {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Initializing unified HLS thread for stream %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                    );
                    thread_state = hls_thread_state_t::HLS_THREAD_CONNECTING;
                    reconnect_attempt = 0 as ::core::ffi::c_int;
                }
                hls_thread_state_t::HLS_THREAD_CONNECTING => {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Connecting to stream %s (attempt %d)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        reconnect_attempt + 1 as ::core::ffi::c_int,
                    );
                    safe_cleanup_resources(
                        &raw mut input_ctx,
                        ::core::ptr::null_mut::<*mut AVPacket>(),
                        ::core::ptr::null_mut::<*mut hls_writer_t>(),
                    );
                    if strncmp(
                        &raw mut (*ctx).rtsp_url as *mut ::core::ffi::c_char,
                        b"rtsp://\0".as_ptr() as *const ::core::ffi::c_char,
                        7 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut host: [::core::ffi::c_char; 256] = [
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
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
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
                        let mut port: ::core::ffi::c_int = 554 as ::core::ffi::c_int;
                        if rtsp_probe_server(
                            &raw mut (*ctx).rtsp_url as *mut ::core::ffi::c_char,
                            &raw mut host as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                            &raw mut port,
                        ) < 0 as ::core::ffi::c_int
                        {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Failed to connect to RTSP server: %s:%d\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut host as *mut ::core::ffi::c_char,
                                port,
                            );
                            ::core::intrinsics::atomic_store::<
                                _,
                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                            >(
                                &raw mut (*ctx).connection_valid, 0 as ::core::ffi::c_int
                            );
                            reconnect_attempt += 1;
                            reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Will retry connection to stream %s in %d ms (attempt %d)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                reconnect_delay_ms,
                                reconnect_attempt + 1 as ::core::ffi::c_int,
                            );
                            if interruptible_sleep_ms(
                                reconnect_delay_ms,
                                ctx,
                                state,
                                &raw mut stream_name as *mut ::core::ffi::c_char,
                            ) == 0
                            {
                                continue;
                            }
                            thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                            continue;
                        }
                    }
                    if is_shutdown_initiated() {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Skipping initial connection for stream %s during shutdown\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                        thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                    } else if ctx.is_null()
                        || is_context_pending_deletion(ctx as *const ::core::ffi::c_void)
                            as ::core::ffi::c_int
                            != 0
                        || is_context_already_freed(ctx as *mut ::core::ffi::c_void)
                            as ::core::ffi::c_int
                            != 0
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Context for stream %s is no longer valid, skipping connection attempt\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                        thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                    } else {
                        local_rtsp_url = [0; 512];
                        local_protocol = 0;
                        if !ctx.is_null()
                            && (*ctx).rtsp_url[0usize] as ::core::ffi::c_int
                                != '\0' as ::core::ffi::c_int
                        {
                            safe_strcpy(
                                &raw mut local_rtsp_url as *mut ::core::ffi::c_char,
                                &raw mut (*ctx).rtsp_url as *mut ::core::ffi::c_char,
                                MAX_PATH_LENGTH as size_t,
                                0 as size_t,
                            );
                            local_protocol = (*ctx).protocol;
                            input_ctx = ::core::ptr::null_mut::<AVFormatContext>();
                            ::core::intrinsics::atomic_fence::<
                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                            >();
                            ret = open_input_stream(
                                &raw mut input_ctx,
                                &raw mut local_rtsp_url as *mut ::core::ffi::c_char,
                                local_protocol,
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
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_ERROR,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Failed to connect to stream %s: %s (error code: %d)\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                    &raw mut error_buf as *mut ::core::ffi::c_char,
                                    ret,
                                );
                                if !input_ctx.is_null() {
                                    let mut ctx_to_cleanup: *mut AVFormatContext = input_ctx;
                                    input_ctx = ::core::ptr::null_mut::<AVFormatContext>();
                                    ::core::intrinsics::atomic_fence::<
                                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                                    >();
                                    comprehensive_ffmpeg_cleanup(
                                        &raw mut ctx_to_cleanup,
                                        ::core::ptr::null_mut::<*mut AVCodecContext>(),
                                        ::core::ptr::null_mut::<*mut AVPacket>(),
                                        ::core::ptr::null_mut::<*mut AVFrame>(),
                                    );
                                    if !ctx_to_cleanup.is_null() {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_WARN,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Failed to clean up context, forcing NULL\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        ctx_to_cleanup = ::core::ptr::null_mut::<AVFormatContext>();
                                    }
                                }
                                if is_shutdown_initiated() {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_INFO,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Stopping initial connection attempts for stream %s during shutdown\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                    );
                                    thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                                } else if !ctx.is_null()
                                    && !is_context_pending_deletion(
                                        ctx as *const ::core::ffi::c_void,
                                    )
                                    && !is_context_already_freed(ctx as *mut ::core::ffi::c_void)
                                {
                                    ::core::intrinsics::atomic_store::<
                                        _,
                                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                                    >(
                                        &raw mut (*ctx).connection_valid, 0 as ::core::ffi::c_int
                                    );
                                    reconnect_attempt += 1;
                                    reconnect_delay_ms =
                                        calculate_reconnect_delay(reconnect_attempt);
                                    if reconnect_attempt > 1000 as ::core::ffi::c_int {
                                        reconnect_attempt = 1000 as ::core::ffi::c_int;
                                    }
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_INFO,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Will retry connection to stream %s in %d ms (attempt %d)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                        reconnect_delay_ms,
                                        reconnect_attempt + 1 as ::core::ffi::c_int,
                                    );
                                    if interruptible_sleep_ms(
                                        reconnect_delay_ms,
                                        ctx,
                                        state,
                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                    ) == 0
                                    {
                                        continue;
                                    }
                                    thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                                } else {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_WARN,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Context for stream %s is no longer valid, skipping connection state update\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                    );
                                    thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                                }
                            } else {
                                if !input_ctx.is_null() {
                                    (*input_ctx).interrupt_callback.opaque =
                                        &raw mut (*ctx).running as *mut ::core::ffi::c_void;
                                }
                                if input_ctx.is_null() {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_ERROR,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Input context is NULL for stream %s despite successful open\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                    );
                                    ::core::intrinsics::atomic_store::<
                                        _,
                                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                                    >(
                                        &raw mut (*ctx).connection_valid, 0 as ::core::ffi::c_int
                                    );
                                    reconnect_attempt += 1;
                                } else {
                                    video_stream_idx = find_video_stream_index(input_ctx);
                                    audio_stream_idx = if (*ctx).audio_enabled != 0 {
                                        find_audio_stream_index(input_ctx)
                                    } else {
                                        -1 as ::core::ffi::c_int
                                    };
                                    if video_stream_idx == -1 as ::core::ffi::c_int {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_ERROR,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"No video stream found for stream %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                        );
                                        comprehensive_ffmpeg_cleanup(
                                            &raw mut input_ctx,
                                            ::core::ptr::null_mut::<*mut AVCodecContext>(),
                                            ::core::ptr::null_mut::<*mut AVPacket>(),
                                            ::core::ptr::null_mut::<*mut AVFrame>(),
                                        );
                                        ::core::intrinsics::atomic_store::<
                                            _,
                                            { ::core::intrinsics::AtomicOrdering::SeqCst },
                                        >(
                                            &raw mut (*ctx).connection_valid,
                                            0 as ::core::ffi::c_int,
                                        );
                                        reconnect_attempt += 1;
                                        reconnect_delay_ms =
                                            calculate_reconnect_delay(reconnect_attempt);
                                        if reconnect_attempt > 1000 as ::core::ffi::c_int {
                                            reconnect_attempt = 1000 as ::core::ffi::c_int;
                                        }
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Will retry connection to stream %s in %d ms (attempt %d)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                            reconnect_delay_ms,
                                            reconnect_attempt + 1 as ::core::ffi::c_int,
                                        );
                                        if interruptible_sleep_ms(
                                            reconnect_delay_ms,
                                            ctx,
                                            state,
                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                        ) == 0
                                        {
                                            continue;
                                        }
                                        thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                                    } else {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Stream %s has %d streams:\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                            (*input_ctx).nb_streams,
                                        );
                                        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                                        while i < (*input_ctx).nb_streams {
                                            let mut stream: *mut AVStream =
                                                *(*input_ctx).streams.offset(i as isize);
                                            let mut codecpar: *mut AVCodecParameters =
                                                (*stream).codecpar;
                                            if (*codecpar).codec_type.0
                                                == AVMediaType::AVMEDIA_TYPE_VIDEO.0
                                            {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Stream %d: Video stream detected (codec: %d, width: %d, height: %d)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    i,
                                                    (*codecpar).codec_id.0,
                                                    (*codecpar).width,
                                                    (*codecpar).height,
                                                );
                                            } else if (*codecpar).codec_type.0
                                                == AVMediaType::AVMEDIA_TYPE_AUDIO.0
                                            {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Stream %d: Audio stream detected (codec: %d, channels: %d, sample_rate: %d)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    i,
                                                    (*codecpar).codec_id.0,
                                                    (*codecpar).ch_layout.nb_channels,
                                                    (*codecpar).sample_rate,
                                                );
                                            } else {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Stream %d: Other stream type detected (codec_type: %d)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    i,
                                                    (*codecpar).codec_type.0,
                                                );
                                            }
                                            i = i.wrapping_add(1);
                                        }
                                        if !is_shutdown_initiated()
                                            && !(*(*input_ctx)
                                                .streams
                                                .offset(video_stream_idx as isize))
                                            .is_null()
                                        {
                                            if !ctx.is_null()
                                                && !is_context_pending_deletion(
                                                    ctx as *const ::core::ffi::c_void,
                                                )
                                                && !is_context_already_freed(
                                                    ctx as *mut ::core::ffi::c_void,
                                                )
                                                && !(*ctx).writer.is_null()
                                            {
                                                ret = hls_writer_initialize_av(
                                                    (*ctx).writer,
                                                    *(*input_ctx)
                                                        .streams
                                                        .offset(video_stream_idx as isize),
                                                    if audio_stream_idx >= 0 as ::core::ffi::c_int {
                                                        *(*input_ctx)
                                                            .streams
                                                            .offset(audio_stream_idx as isize)
                                                    } else {
                                                        ::core::ptr::null_mut::<AVStream>()
                                                    },
                                                );
                                                if ret < 0 as ::core::ffi::c_int {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_ERROR,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Failed to initialize HLS writer for stream %s\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                                    );
                                                    comprehensive_ffmpeg_cleanup(
                                                        &raw mut input_ctx,
                                                        ::core::ptr::null_mut::<*mut AVCodecContext>(
                                                        ),
                                                        ::core::ptr::null_mut::<*mut AVPacket>(),
                                                        ::core::ptr::null_mut::<*mut AVFrame>(),
                                                    );
                                                    if !ctx.is_null()
                                                        && !is_context_pending_deletion(
                                                            ctx as *const ::core::ffi::c_void,
                                                        )
                                                        && !is_context_already_freed(
                                                            ctx as *mut ::core::ffi::c_void,
                                                        )
                                                    {
                                                        ::core::intrinsics::atomic_store::<
                                                            _,
                                                            {
                                                                ::core::intrinsics::AtomicOrdering::SeqCst
                                                            },
                                                        >(
                                                            &raw mut (*ctx).connection_valid,
                                                            0 as ::core::ffi::c_int,
                                                        );
                                                        reconnect_attempt += 1;
                                                        reconnect_delay_ms =
                                                            calculate_reconnect_delay(
                                                                reconnect_attempt,
                                                            );
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_INFO,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Will retry connection to stream %s in %d ms (attempt %d)\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                                            reconnect_delay_ms,
                                                            reconnect_attempt + 1 as ::core::ffi::c_int,
                                                        );
                                                        if interruptible_sleep_ms(
                                                            reconnect_delay_ms,
                                                            ctx,
                                                            state,
                                                            &raw mut stream_name
                                                                as *mut ::core::ffi::c_char,
                                                        ) == 0
                                                        {
                                                            continue;
                                                        }
                                                        thread_state =
                                                            hls_thread_state_t::HLS_THREAD_STOPPING;
                                                        continue;
                                                    } else {
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_WARN,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Context for stream %s is no longer valid, skipping connection state update\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                                        );
                                                        thread_state =
                                                            hls_thread_state_t::HLS_THREAD_STOPPING;
                                                        continue;
                                                    }
                                                }
                                            } else {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_WARN,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Context for stream %s is no longer valid or writer is NULL, skipping HLS writer initialization\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                                );
                                                thread_state =
                                                    hls_thread_state_t::HLS_THREAD_STOPPING;
                                                continue;
                                            }
                                        }
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Successfully connected to stream %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                        );
                                        thread_state = hls_thread_state_t::HLS_THREAD_RUNNING;
                                        reconnect_attempt = 0 as ::core::ffi::c_int;
                                        if is_context_already_freed(ctx as *mut ::core::ffi::c_void)
                                            as ::core::ffi::c_int
                                            != 0
                                            || is_context_pending_deletion(
                                                ctx as *const ::core::ffi::c_void,
                                            )
                                                as ::core::ffi::c_int
                                                != 0
                                        {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_WARN,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Context for stream %s is no longer valid, exiting thread\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                            );
                                            thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                                        } else {
                                            ::core::intrinsics::atomic_store::<
                                                _,
                                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                                            >(
                                                &raw mut (*ctx).connection_valid,
                                                1 as ::core::ffi::c_int,
                                            );
                                            ::core::intrinsics::atomic_store::<
                                                _,
                                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                                            >(
                                                &raw mut (*ctx).consecutive_failures,
                                                0 as ::core::ffi::c_int,
                                            );
                                            last_packet_time =
                                                time(::core::ptr::null_mut::<time_t>());
                                            ::core::intrinsics::atomic_store::<
                                                _,
                                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                                            >(
                                                &raw mut (*ctx).last_packet_time, last_packet_time
                                            );
                                        }
                                    }
                                }
                            }
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Invalid RTSP URL for stream %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut stream_name as *mut ::core::ffi::c_char,
                            );
                            thread_state = hls_thread_state_t::HLS_THREAD_RECONNECTING;
                            reconnect_attempt += 1;
                        }
                    }
                }
                hls_thread_state_t::HLS_THREAD_RUNNING => {
                    if is_context_already_freed(ctx as *mut ::core::ffi::c_void)
                        as ::core::ffi::c_int
                        != 0
                        || is_context_pending_deletion(ctx as *const ::core::ffi::c_void)
                            as ::core::ffi::c_int
                            != 0
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Context for stream %s is no longer valid, exiting thread\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                        thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                    } else if ::core::intrinsics::atomic_load::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*ctx).running)
                        == 0
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Unified HLS thread for %s detected shutdown before read\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                        thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                    } else {
                        ret = av_read_frame(input_ctx, pkt);
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
                                b"Error reading from stream %s: %s (code: %d)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                &raw mut error_buf_0 as *mut ::core::ffi::c_char,
                                ret,
                            );
                            av_packet_unref(pkt);
                            thread_state = hls_thread_state_t::HLS_THREAD_RECONNECTING;
                            reconnect_attempt = 1 as ::core::ffi::c_int;
                            ::core::intrinsics::atomic_store::<
                                _,
                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                            >(
                                &raw mut (*ctx).connection_valid, 0 as ::core::ffi::c_int
                            );
                            ::core::intrinsics::atomic_xadd::<
                                _,
                                _,
                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                            >(
                                &raw mut (*ctx).consecutive_failures,
                                1 as ::core::ffi::c_int,
                            );
                        } else {
                            input_stream = ::core::ptr::null::<AVStream>();
                            if (*pkt).stream_index >= 0 as ::core::ffi::c_int
                                && ((*pkt).stream_index as ::core::ffi::c_uint)
                                    < (*input_ctx).nb_streams
                            {
                                input_stream =
                                    *(*input_ctx).streams.offset((*pkt).stream_index as isize);
                                if (*pkt).data.is_null() || (*pkt).size <= 0 as ::core::ffi::c_int {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_WARN,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Invalid packet (null data or zero size) for stream %s\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                    );
                                    av_packet_unref(pkt);
                                } else {
                                    is_audio_pkt = audio_stream_idx >= 0 as ::core::ffi::c_int
                                        && (*pkt).stream_index == audio_stream_idx;
                                    if (*pkt).stream_index == video_stream_idx
                                        || is_audio_pkt as ::core::ffi::c_int != 0
                                    {
                                        if !is_audio_pkt {
                                            metrics_record_frame_from_source(
                                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                                (*pkt).size,
                                                r#true != 0,
                                                metrics_frame_source_t::METRICS_SOURCE_HLS,
                                            );
                                        }
                                        if ctx.is_null()
                                            || is_context_pending_deletion(
                                                ctx as *const ::core::ffi::c_void,
                                            )
                                                as ::core::ffi::c_int
                                                != 0
                                            || is_context_already_freed(
                                                ctx as *mut ::core::ffi::c_void,
                                            )
                                                as ::core::ffi::c_int
                                                != 0
                                        {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_WARN,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Context for stream %s is no longer valid, skipping packet and exiting thread\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                            );
                                            thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                                            continue;
                                        } else if (*ctx).writer.is_null() {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_WARN,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Writer is NULL for stream %s during packet processing, skipping packet\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                            );
                                            continue;
                                        } else {
                                            let mut writer: *mut hls_writer_t = (*ctx).writer;
                                            ::core::intrinsics::atomic_fence::<
                                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                                            >();
                                            if writer.is_null() {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_WARN,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Writer became NULL for stream %s after memory barrier, skipping packet\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                                );
                                                continue;
                                            } else {
                                                pthread_mutex_lock(&raw mut (*writer).mutex);
                                                if (*ctx).writer != writer {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_WARN,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Writer changed for stream %s during mutex lock, releasing mutex and skipping packet\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                                    );
                                                    pthread_mutex_unlock(&raw mut (*writer).mutex);
                                                    continue;
                                                } else {
                                                    if (*writer).initialized == 0 {
                                                        hls_writer_initialize_av(
                                                            writer,
                                                            *(*input_ctx)
                                                                .streams
                                                                .offset(video_stream_idx as isize),
                                                            if audio_stream_idx
                                                                >= 0 as ::core::ffi::c_int
                                                            {
                                                                *(*input_ctx).streams.offset(
                                                                    audio_stream_idx as isize,
                                                                )
                                                            } else {
                                                                ::core::ptr::null_mut::<AVStream>()
                                                            },
                                                        );
                                                    }
                                                    ret = hls_writer_write_packet(
                                                        writer,
                                                        pkt,
                                                        input_stream,
                                                    );
                                                    let mut is_key_frame: bool = !is_audio_pkt
                                                        && (*pkt).flags & AV_PKT_FLAG_KEY
                                                            != 0 as ::core::ffi::c_int;
                                                    if is_key_frame as ::core::ffi::c_int != 0
                                                        && ret >= 0 as ::core::ffi::c_int
                                                    {
                                                        _log_message_ctx(
                                                            log_level_t::LOG_LEVEL_DEBUG,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Processed video key frame for stream %s\0".as_ptr()
                                                                as *const ::core::ffi::c_char,
                                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                                        );
                                                    }
                                                    if ret < 0 as ::core::ffi::c_int {
                                                        let mut error_buf_1: [::core::ffi::c_char;
                                                            64] = [
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
                                                            log_level_t::LOG_LEVEL_WARN,
                                                            LOG_COMPONENT,
                                                            _log_stream_name,
                                                            b"Error writing %s packet to HLS for stream %s: %s\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                            if is_audio_pkt as ::core::ffi::c_int != 0 {
                                                                b"audio\0".as_ptr() as *const ::core::ffi::c_char
                                                            } else {
                                                                b"video\0".as_ptr() as *const ::core::ffi::c_char
                                                            },
                                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                                            &raw mut error_buf_1 as *mut ::core::ffi::c_char,
                                                        );
                                                    } else {
                                                        if !is_audio_pkt {
                                                            last_packet_time = time(
                                                                ::core::ptr::null_mut::<time_t>(),
                                                            );
                                                            ::core::intrinsics::atomic_store::<
                                                                _,
                                                                {
                                                                    ::core::intrinsics::AtomicOrdering::SeqCst
                                                                },
                                                            >(
                                                                &raw mut (*ctx).last_packet_time,
                                                                last_packet_time,
                                                            );
                                                            ::core::intrinsics::atomic_store::<
                                                                _,
                                                                {
                                                                    ::core::intrinsics::AtomicOrdering::SeqCst
                                                                },
                                                            >(
                                                                &raw mut (*ctx)
                                                                    .consecutive_failures,
                                                                0 as ::core::ffi::c_int,
                                                            );
                                                            ::core::intrinsics::atomic_store::<
                                                                _,
                                                                {
                                                                    ::core::intrinsics::AtomicOrdering::SeqCst
                                                                },
                                                            >(
                                                                &raw mut (*ctx).connection_valid,
                                                                1 as ::core::ffi::c_int,
                                                            );
                                                        }
                                                        has_written_packets = r#true != 0;
                                                    }
                                                    pthread_mutex_unlock(&raw mut (*writer).mutex);
                                                }
                                            }
                                        }
                                    } else if (*pkt).stream_index >= 0 as ::core::ffi::c_int
                                        && ((*pkt).stream_index as ::core::ffi::c_uint)
                                            < (*input_ctx).nb_streams
                                    {
                                        let mut stream_0: *const AVStream = *(*input_ctx)
                                            .streams
                                            .offset((*pkt).stream_index as isize);
                                        let mut codecpar_0: *const AVCodecParameters =
                                            (*stream_0).codecpar;
                                        if (*codecpar_0).codec_type.0
                                            == AVMediaType::AVMEDIA_TYPE_AUDIO.0
                                        {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_DEBUG,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Skipping audio packet for stream %s (stream index: %d)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                                (*pkt).stream_index,
                                            );
                                        } else {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_DEBUG,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Skipping non-video packet for stream %s (stream index: %d, type: %d)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                                (*pkt).stream_index,
                                                (*codecpar_0).codec_type.0,
                                            );
                                        }
                                    } else {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_WARN,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Skipping packet with invalid stream index %d for stream %s\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            (*pkt).stream_index,
                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                        );
                                    }
                                    av_packet_unref(pkt);
                                    now = time(::core::ptr::null_mut::<time_t>());
                                    if now - last_packet_time > MAX_PACKET_TIMEOUT as time_t {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_ERROR,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"No video packets successfully written to HLS for stream %s for %ld seconds, reconnecting\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                            now - last_packet_time,
                                        );
                                        thread_state = hls_thread_state_t::HLS_THREAD_RECONNECTING;
                                        reconnect_attempt = 1 as ::core::ffi::c_int;
                                        if !ctx.is_null()
                                            && !is_context_pending_deletion(
                                                ctx as *const ::core::ffi::c_void,
                                            )
                                            && !is_context_already_freed(
                                                ctx as *mut ::core::ffi::c_void,
                                            )
                                        {
                                            ::core::intrinsics::atomic_store::<
                                                _,
                                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                                            >(
                                                &raw mut (*ctx).connection_valid,
                                                0 as ::core::ffi::c_int,
                                            );
                                            ::core::intrinsics::atomic_xadd::<
                                                _,
                                                _,
                                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                                            >(
                                                &raw mut (*ctx).consecutive_failures,
                                                1 as ::core::ffi::c_int,
                                            );
                                        } else {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_WARN,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Context for stream %s is no longer valid, skipping connection state update\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                            );
                                            thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                                        }
                                    }
                                }
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Invalid stream index %d for stream %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*pkt).stream_index,
                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                );
                                av_packet_unref(pkt);
                            }
                        }
                    }
                }
                hls_thread_state_t::HLS_THREAD_RECONNECTING => {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Reconnecting to stream %s (attempt %d)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        reconnect_attempt,
                    );
                    safe_cleanup_resources(
                        &raw mut input_ctx,
                        ::core::ptr::null_mut::<*mut AVPacket>(),
                        ::core::ptr::null_mut::<*mut hls_writer_t>(),
                    );
                    reconnect_delay_ms = calculate_reconnect_delay(reconnect_attempt);
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Waiting %d ms before reconnecting to stream %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        reconnect_delay_ms,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                    );
                    if interruptible_sleep_ms(
                        reconnect_delay_ms,
                        ctx,
                        state,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                    ) != 0
                    {
                        thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                    } else if is_shutdown_initiated() {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Skipping reconnection for stream %s during shutdown\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                        thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                    } else if ctx.is_null()
                        || is_context_pending_deletion(ctx as *const ::core::ffi::c_void)
                            as ::core::ffi::c_int
                            != 0
                        || is_context_already_freed(ctx as *mut ::core::ffi::c_void)
                            as ::core::ffi::c_int
                            != 0
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Context for stream %s is no longer valid, skipping reconnection attempt\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                        thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                    } else {
                        reconnect_rtsp_url = [0; 512];
                        reconnect_protocol = 0;
                        if !ctx.is_null()
                            && (*ctx).rtsp_url[0usize] as ::core::ffi::c_int
                                != '\0' as ::core::ffi::c_int
                        {
                            safe_strcpy(
                                &raw mut reconnect_rtsp_url as *mut ::core::ffi::c_char,
                                &raw mut (*ctx).rtsp_url as *mut ::core::ffi::c_char,
                                MAX_PATH_LENGTH as size_t,
                                0 as size_t,
                            );
                            reconnect_protocol = (*ctx).protocol;
                            input_ctx = ::core::ptr::null_mut::<AVFormatContext>();
                            ::core::intrinsics::atomic_fence::<
                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                            >();
                            ret = open_input_stream(
                                &raw mut input_ctx,
                                &raw mut reconnect_rtsp_url as *mut ::core::ffi::c_char,
                                reconnect_protocol,
                            );
                            if ret < 0 as ::core::ffi::c_int {
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
                                    b"Failed to reconnect to stream %s: %s (error code: %d)\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                    &raw mut error_buf_2 as *mut ::core::ffi::c_char,
                                    ret,
                                );
                                if !input_ctx.is_null() {
                                    let mut ctx_to_cleanup_0: *mut AVFormatContext = input_ctx;
                                    input_ctx = ::core::ptr::null_mut::<AVFormatContext>();
                                    ::core::intrinsics::atomic_fence::<
                                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                                    >();
                                    comprehensive_ffmpeg_cleanup(
                                        &raw mut ctx_to_cleanup_0,
                                        ::core::ptr::null_mut::<*mut AVCodecContext>(),
                                        ::core::ptr::null_mut::<*mut AVPacket>(),
                                        ::core::ptr::null_mut::<*mut AVFrame>(),
                                    );
                                    if !ctx_to_cleanup_0.is_null() {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_WARN,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Failed to clean up context, forcing NULL\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        ctx_to_cleanup_0 =
                                            ::core::ptr::null_mut::<AVFormatContext>();
                                    }
                                }
                                if is_shutdown_initiated() {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_INFO,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Stopping reconnection attempts for stream %s during shutdown\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                    );
                                    thread_state = hls_thread_state_t::HLS_THREAD_STOPPING;
                                } else {
                                    reconnect_attempt += 1;
                                    if reconnect_attempt > 1000 as ::core::ffi::c_int {
                                        reconnect_attempt = 1000 as ::core::ffi::c_int;
                                    }
                                }
                            } else {
                                if !input_ctx.is_null() {
                                    (*input_ctx).interrupt_callback.opaque =
                                        &raw mut (*ctx).running as *mut ::core::ffi::c_void;
                                }
                                if input_ctx.is_null() {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_ERROR,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Input context is NULL for stream %s despite successful reconnect\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                    );
                                    reconnect_attempt += 1;
                                } else {
                                    video_stream_idx = find_video_stream_index(input_ctx);
                                    audio_stream_idx = if (*ctx).audio_enabled != 0 {
                                        find_audio_stream_index(input_ctx)
                                    } else {
                                        -1 as ::core::ffi::c_int
                                    };
                                    if video_stream_idx == -1 as ::core::ffi::c_int {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_ERROR,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"No video stream found for stream %s during reconnection\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                        );
                                        avformat_close_input(&raw mut input_ctx);
                                        reconnect_attempt += 1;
                                        if reconnect_attempt > 1000 as ::core::ffi::c_int {
                                            reconnect_attempt = 1000 as ::core::ffi::c_int;
                                        }
                                    } else {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"Stream %s has %d streams during reconnection:\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            &raw mut stream_name as *mut ::core::ffi::c_char,
                                            (*input_ctx).nb_streams,
                                        );
                                        let mut i_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                                        while i_0 < (*input_ctx).nb_streams {
                                            let mut stream_1: *mut AVStream =
                                                *(*input_ctx).streams.offset(i_0 as isize);
                                            let mut codecpar_1: *mut AVCodecParameters =
                                                (*stream_1).codecpar;
                                            if (*codecpar_1).codec_type.0
                                                == AVMediaType::AVMEDIA_TYPE_VIDEO.0
                                            {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Stream %d: Video stream detected (codec: %d, width: %d, height: %d)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    i_0,
                                                    (*codecpar_1).codec_id.0,
                                                    (*codecpar_1).width,
                                                    (*codecpar_1).height,
                                                );
                                            } else if (*codecpar_1).codec_type.0
                                                == AVMediaType::AVMEDIA_TYPE_AUDIO.0
                                            {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Stream %d: Audio stream detected (codec: %d, channels: %d, sample_rate: %d)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    i_0,
                                                    (*codecpar_1).codec_id.0,
                                                    (*codecpar_1).ch_layout.nb_channels,
                                                    (*codecpar_1).sample_rate,
                                                );
                                            } else {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Stream %d: Other stream type detected (codec_type: %d)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    i_0,
                                                    (*codecpar_1).codec_type.0,
                                                );
                                            }
                                            i_0 = i_0.wrapping_add(1);
                                        }
                                        if input_ctx.is_null()
                                            || (*input_ctx).nb_streams == 0 as ::core::ffi::c_uint
                                        {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_ERROR,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Reconnection to stream %s failed: Invalid input context or no streams\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                            );
                                            if !input_ctx.is_null() {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_WARN,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Cleaning up invalid input context for stream %s\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                                );
                                                safe_cleanup_resources(
                                                    &raw mut input_ctx,
                                                    ::core::ptr::null_mut::<*mut AVPacket>(),
                                                    ::core::ptr::null_mut::<*mut hls_writer_t>(),
                                                );
                                            }
                                            reconnect_attempt += 1;
                                            if reconnect_attempt > 1000 as ::core::ffi::c_int {
                                                reconnect_attempt = 1000 as ::core::ffi::c_int;
                                            }
                                        } else {
                                            _log_message_ctx(
                                                log_level_t::LOG_LEVEL_INFO,
                                                LOG_COMPONENT,
                                                _log_stream_name,
                                                b"Successfully reconnected to stream %s after %d attempts\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                                reconnect_attempt,
                                            );
                                            if has_written_packets as ::core::ffi::c_int != 0
                                                && !(*ctx).writer.is_null()
                                            {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Closing old HLS writer for stream %s after reconnection (has_written_packets=true)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                                );
                                                let mut old_writer: *mut hls_writer_t =
                                                    ::core::intrinsics::atomic_xchg::<
                                                        _,
                                                        {
                                                            ::core::intrinsics::AtomicOrdering::SeqCst
                                                        },
                                                    >(
                                                        &raw mut (*ctx).writer,
                                                        ::core::ptr::null_mut::<hls_writer_t>(),
                                                    );
                                                if !old_writer.is_null() {
                                                    hls_writer_close(old_writer);
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_INFO,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Successfully closed old HLS writer for stream %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                                    );
                                                }
                                                let mut reconnect_seg_duration: ::core::ffi::c_int =
                                                    if (*ctx).segment_duration
                                                        > 0 as ::core::ffi::c_int
                                                    {
                                                        (*ctx).segment_duration
                                                    } else {
                                                        2 as ::core::ffi::c_int
                                                    };
                                                (*ctx).writer = hls_writer_create(
                                                    &raw mut (*ctx).output_path
                                                        as *mut ::core::ffi::c_char,
                                                    &raw mut stream_name
                                                        as *mut ::core::ffi::c_char,
                                                    reconnect_seg_duration,
                                                );
                                                if (*ctx).writer.is_null() {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_ERROR,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Failed to create new HLS writer for %s after reconnection\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                                    );
                                                    reconnect_attempt += 1;
                                                    thread_state =
                                                        hls_thread_state_t::HLS_THREAD_RECONNECTING;
                                                    continue;
                                                } else {
                                                    _log_message_ctx(
                                                        log_level_t::LOG_LEVEL_INFO,
                                                        LOG_COMPONENT,
                                                        _log_stream_name,
                                                        b"Created new HLS writer for stream %s after reconnection\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                                    );
                                                    has_written_packets = r#false != 0;
                                                }
                                            } else if !has_written_packets {
                                                _log_message_ctx(
                                                    log_level_t::LOG_LEVEL_INFO,
                                                    LOG_COMPONENT,
                                                    _log_stream_name,
                                                    b"Keeping existing HLS writer for stream %s (no packets written yet)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                                );
                                            }
                                            thread_state = hls_thread_state_t::HLS_THREAD_RUNNING;
                                            reconnect_attempt = 0 as ::core::ffi::c_int;
                                            ::core::intrinsics::atomic_store::<
                                                _,
                                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                                            >(
                                                &raw mut (*ctx).connection_valid,
                                                1 as ::core::ffi::c_int,
                                            );
                                            ::core::intrinsics::atomic_store::<
                                                _,
                                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                                            >(
                                                &raw mut (*ctx).consecutive_failures,
                                                0 as ::core::ffi::c_int,
                                            );
                                            last_packet_time =
                                                time(::core::ptr::null_mut::<time_t>());
                                            ::core::intrinsics::atomic_store::<
                                                _,
                                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                                            >(
                                                &raw mut (*ctx).last_packet_time, last_packet_time
                                            );
                                        }
                                    }
                                }
                            }
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Invalid RTSP URL for stream %s during reconnection\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut stream_name as *mut ::core::ffi::c_char,
                            );
                            thread_state = hls_thread_state_t::HLS_THREAD_RECONNECTING;
                            reconnect_attempt += 1;
                        }
                    }
                }
                hls_thread_state_t::HLS_THREAD_STOPPING => {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Stopping unified HLS thread for stream %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                    );
                    ::core::intrinsics::atomic_store::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*ctx).running, 0 as ::core::ffi::c_int);
                    ::core::intrinsics::atomic_store::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*ctx).connection_valid, 0 as ::core::ffi::c_int);
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Cleaning up resources for stream %s before stopping\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                    );
                    safe_cleanup_resources(
                        &raw mut input_ctx,
                        &raw mut pkt,
                        ::core::ptr::null_mut::<*mut hls_writer_t>(),
                    );
                    if !(*ctx).writer.is_null() {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Closing HLS writer for stream %s during shutdown\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                        let mut writer_to_cleanup: *mut hls_writer_t =
                            ::core::intrinsics::atomic_xchg::<
                                _,
                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                            >(
                                &raw mut (*ctx).writer,
                                ::core::ptr::null_mut::<hls_writer_t>(),
                            );
                        if !writer_to_cleanup.is_null() {
                            hls_writer_close(writer_to_cleanup);
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Successfully closed HLS writer for stream %s during shutdown\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut stream_name as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                    if (*ctx).shutdown_component_id >= 0 as ::core::ffi::c_int {
                        update_component_state(
                            (*ctx).shutdown_component_id,
                            component_state_t::COMPONENT_STOPPED,
                        );
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Updated component state to STOPPED for stream %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                    }
                    thread_state = hls_thread_state_t::HLS_THREAD_STOPPED;
                }
                hls_thread_state_t::HLS_THREAD_STOPPED => {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Unified HLS thread for %s in STOPPED state but still in main loop\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                    );
                    ::core::intrinsics::atomic_store::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*ctx).running, 0 as ::core::ffi::c_int);
                }
                _ => {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Unified HLS thread for %s in unknown state: %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        thread_state.0,
                    );
                    ::core::intrinsics::atomic_store::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*ctx).running, 0 as ::core::ffi::c_int);
                }
            }
        }
    }
    let mut ctx_for_exit: *mut hls_unified_thread_ctx_t = ctx;
    ctx = ::core::ptr::null_mut::<hls_unified_thread_ctx_t>();
    let mut context_valid_for_exit: bool = !ctx_for_exit.is_null()
        && !is_context_already_freed(ctx_for_exit as *mut ::core::ffi::c_void)
        && !is_context_pending_deletion(ctx_for_exit as *const ::core::ffi::c_void);
    if !input_ctx.is_null() || !pkt.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Thread for stream %s exited loop without proper cleanup, cleaning up now\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        safe_cleanup_resources(
            &raw mut input_ctx,
            &raw mut pkt,
            ::core::ptr::null_mut::<*mut hls_writer_t>(),
        );
    }
    if !ctx_for_exit.is_null() {
        let mut writer_to_cleanup_0: *mut hls_writer_t = ::core::ptr::null_mut::<hls_writer_t>();
        if context_valid_for_exit {
            if !(*ctx_for_exit).writer.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Writer for stream %s still exists after loop exit, cleaning up now\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
                writer_to_cleanup_0 = ::core::intrinsics::atomic_xchg::<
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(
                    &raw mut (*ctx_for_exit).writer,
                    ::core::ptr::null_mut::<hls_writer_t>(),
                );
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Context for stream %s is no longer valid, skipping writer cleanup\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
        if !writer_to_cleanup_0.is_null() {
            hls_writer_close(writer_to_cleanup_0);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Successfully closed HLS writer for stream %s after loop exit\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
        mark_thread_exited(ctx_for_exit as *mut ::core::ffi::c_void);
        if context_valid_for_exit {
            if !is_context_already_freed(ctx_for_exit as *mut ::core::ffi::c_void)
                && !is_context_pending_deletion(ctx_for_exit as *const ::core::ffi::c_void)
            {
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx_for_exit).connection_valid,
                    0 as ::core::ffi::c_int,
                );
                if (*ctx_for_exit).shutdown_component_id >= 0 as ::core::ffi::c_int {
                    update_component_state(
                        (*ctx_for_exit).shutdown_component_id,
                        component_state_t::COMPONENT_STOPPED,
                    );
                }
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Context for stream %s is no longer valid, skipping connection_valid update\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Updated component state to STOPPED for stream %s after loop exit\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
    }
    if !state.is_null() {
        let mut writer_ptr: *const hls_writer_t = ::core::ptr::null::<hls_writer_t>();
        if !ctx.is_null() && !is_context_already_freed(ctx as *mut ::core::ffi::c_void) {
            writer_ptr = (*ctx).writer;
        }
        if !writer_ptr.is_null() && (*state).hls_ctx == writer_ptr as *mut ::core::ffi::c_void {
            (*state).hls_ctx = NULL;
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleaning up all resources for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    if !input_ctx.is_null() || !pkt.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Final cleanup of FFmpeg resources for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        if !input_ctx.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cleaning up input context for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
        if !pkt.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cleaning up packet for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
        safe_cleanup_resources(
            &raw mut input_ctx,
            &raw mut pkt,
            ::core::ptr::null_mut::<*mut hls_writer_t>(),
        );
    }
    let mut writer_to_cleanup_1: *mut hls_writer_t = ::core::ptr::null_mut::<hls_writer_t>();
    let mut shutdown_id: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut ctx_local: *mut hls_unified_thread_ctx_t = ctx;
    ctx = ::core::ptr::null_mut::<hls_unified_thread_ctx_t>();
    if !ctx_local.is_null()
        && !is_context_already_freed(ctx_local as *mut ::core::ffi::c_void)
        && !is_context_pending_deletion(ctx_local as *const ::core::ffi::c_void)
    {
        let mut writer_ptr_0: *const hls_writer_t = ::core::ptr::null::<hls_writer_t>();
        writer_ptr_0 = ::core::intrinsics::atomic_load::<
            *mut hls_writer_t,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut (*ctx_local).writer);
        if !writer_ptr_0.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cleaning up HLS writer for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
            writer_to_cleanup_1 = ::core::intrinsics::atomic_xchg::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*ctx_local).writer,
                ::core::ptr::null_mut::<hls_writer_t>(),
            );
            if !writer_to_cleanup_1.is_null()
                && !(&raw mut (*writer_to_cleanup_1).stream_name as *mut ::core::ffi::c_char)
                    .is_null()
            {
                let mut writer_stream_name: [::core::ffi::c_char; 256] = [0; 256];
                safe_strcpy(
                    &raw mut writer_stream_name as *mut ::core::ffi::c_char,
                    &raw mut (*writer_to_cleanup_1).stream_name as *mut ::core::ffi::c_char,
                    MAX_STREAM_NAME as size_t,
                    0 as size_t,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Preparing to clean up HLS writer for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut writer_stream_name as *mut ::core::ffi::c_char,
                );
            }
        }
        shutdown_id = (*ctx_local).shutdown_component_id;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Context for stream %s is no longer valid during cleanup (already cleaned up)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"About to clean up remaining resources for stream %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
    if !writer_to_cleanup_1.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Writer is valid for stream %s, cleaning up\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Closing HLS writer for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        hls_writer_close(writer_to_cleanup_1);
        writer_to_cleanup_1 = ::core::ptr::null_mut::<hls_writer_t>();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Successfully closed HLS writer for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No writer to clean up for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    if shutdown_id >= 0 as ::core::ffi::c_int {
        update_component_state(shutdown_id, component_state_t::COMPONENT_STOPPED);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Updated unified HLS thread %s state to STOPPED in shutdown coordinator\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    if !ctx_local.is_null() {
        mark_thread_exited(ctx_local as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Final marking of thread as exited for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    unmark_stream_stopping(&raw mut stream_name as *mut ::core::ffi::c_char);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unmarked stream %s as stopping before thread exit\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    if !ctx_local.is_null() {
        ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
        let mut ctx_to_free: *mut hls_unified_thread_ctx_t = ctx_local;
        ctx_local = ::core::ptr::null_mut::<hls_unified_thread_ctx_t>();
        ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
        let mut context_valid: bool = r#true != 0;
        if (*ctx_to_free).stream_name[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Context has empty stream_name, may be corrupted\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            context_valid = r#false != 0;
        }
        let mut ptr: *const ::core::ffi::c_uchar = ctx_to_free as *const ::core::ffi::c_uchar;
        let mut all_zeros: bool = r#true != 0;
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < 16 as ::core::ffi::c_int
            && (i_1 as usize) < ::core::mem::size_of::<hls_unified_thread_ctx_t>()
        {
            if *ptr.offset(i_1 as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                all_zeros = r#false != 0;
                break;
            } else {
                i_1 += 1;
            }
        }
        if all_zeros {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Context memory appears to be zeroed out, may have been freed already\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            context_valid = r#false != 0;
        }
        if context_valid {
            if is_context_already_freed(ctx_to_free as *mut ::core::ffi::c_void) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Context for stream %s has already been freed, skipping\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Freeing context for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
                mark_context_pending_deletion(ctx_to_free as *mut ::core::ffi::c_void);
                wait_for_thread_exit(ctx_to_free as *mut ::core::ffi::c_void);
                mark_context_as_freed(ctx_to_free as *mut ::core::ffi::c_void);
                clear_context_pending_deletion(ctx_to_free as *const ::core::ffi::c_void);
                hls_guarded_free(ctx_to_free as *mut ::core::ffi::c_void);
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Successfully freed context for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Skipping cleanup of potentially invalid context for stream %s (already cleaned up)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Context is NULL during cleanup for stream %s (already cleaned up)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unified HLS thread for stream %s exited\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    if !input_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Input context still exists at thread exit for stream %s, forcing cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        let mut ctx_to_cleanup_1: *mut AVFormatContext = input_ctx;
        input_ctx = ::core::ptr::null_mut::<AVFormatContext>();
        comprehensive_ffmpeg_cleanup(
            &raw mut ctx_to_cleanup_1,
            ::core::ptr::null_mut::<*mut AVCodecContext>(),
            ::core::ptr::null_mut::<*mut AVPacket>(),
            ::core::ptr::null_mut::<*mut AVFrame>(),
        );
    }
    if !pkt.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Packet still exists at thread exit for stream %s, forcing cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        let mut pkt_to_cleanup: *mut AVPacket = pkt;
        pkt = ::core::ptr::null_mut::<AVPacket>();
        if !pkt_to_cleanup.is_null() {
            av_packet_unref(pkt_to_cleanup);
            av_packet_free(&raw mut pkt_to_cleanup);
        }
    }
    if !is_shutdown_initiated()
        && !is_stream_stopping(&raw mut stream_name as *mut ::core::ffi::c_char)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Using safer approach to release FFmpeg memory during thread exit\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut dummy: *mut ::core::ffi::c_void = av_malloc(16 as size_t);
        if !dummy.is_null() {
            av_free(dummy);
        }
        av_log_set_level(AV_LOG_QUIET);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Skipping aggressive FFmpeg cleanup during shutdown or stream deletion to prevent crashes\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !ctx_for_exit.is_null()
        && !is_context_already_freed(ctx_for_exit as *mut ::core::ffi::c_void)
    {
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx_for_exit).thread_state,
            hls_thread_state_t::HLS_THREAD_STOPPED.0 as ::core::ffi::c_int,
        );
        ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
        mark_thread_exited(ctx_for_exit as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Thread for stream %s marked as STOPPED\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Context for stream %s is no longer valid, skipping final thread exit marking\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unified HLS thread for stream %s has completed all cleanup steps\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    return NULL;
}
#[unsafe(no_mangle)]
pub static mut unified_contexts_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_0::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
#[unsafe(no_mangle)]
pub static mut unified_contexts: [*mut hls_unified_thread_ctx_t; 1024] =
    [::core::ptr::null_mut::<hls_unified_thread_ctx_t>(); 1024];
static mut hls_watchdog_thread: pthread_t = 0;
static mut watchdog_running: atomic_bool = false;
static mut watchdog_restart_count: atomic_int = 0 as ::core::ffi::c_int;
static mut last_restart_time: [time_t; 1024] = [
    0 as time_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
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
static mut restart_attempts: [::core::ffi::c_int; 1024] = [0 as ::core::ffi::c_int; 1024];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_hls_unified_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if is_shutdown_initiated() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot start HLS stream %s during shutdown\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: stream_handle_t = get_stream_by_name(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut config: stream_config_t = stream_config_t {
        camera_uuid: [0; 37],
        location_uuid: [0; 37],
        name: [0; 256],
        url: [0; 512],
        enabled: false,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        priority: 0,
        record: false,
        segment_duration: 0,
        detection_based_recording: false,
        detection_model: [0; 512],
        detection_interval: 0,
        detection_threshold: 0.,
        pre_detection_buffer: 0,
        post_detection_buffer: 0,
        detection_api_url: [0; 512],
        detection_object_filter: [0; 16],
        detection_object_filter_list: [0; 256],
        streaming_enabled: false,
        protocol: stream_protocol_t::STREAM_PROTOCOL_TCP,
        record_audio: false,
        audio_voice_enhancement: false,
        onvif_username: [0; 64],
        onvif_password: [0; 64],
        onvif_profile: [0; 64],
        onvif_port: 0,
        onvif_discovery_enabled: false,
        is_onvif: false,
        backchannel_enabled: false,
        retention_days: 0,
        detection_retention_days: 0,
        max_storage_mb: 0,
        tier_critical_multiplier: 0.,
        tier_important_multiplier: 0.,
        tier_ephemeral_multiplier: 0.,
        storage_priority: 0,
        ptz_enabled: false,
        ptz_max_x: 0,
        ptz_max_y: 0,
        ptz_max_z: 0,
        ptz_has_home: false,
        record_on_schedule: false,
        recording_schedule: [0; 168],
        detection_record_on_schedule: false,
        detection_recording_schedule: [0; 168],
        tags: [0; 256],
        admin_url: [0; 512],
        privacy_mode: false,
        motion_trigger_source: [0; 256],
        go2rtc_source_override: [0; 8192],
        sub_stream_url: [0; 512],
        detection_url: [0; 512],
        publish_url: [0; 512],
        playback_transport: [0; 24],
        eptz_config: [0; 1024],
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    if get_stream_config(stream, &raw mut config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get config for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = get_stream_state_by_name(stream_name);
    if state.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream state not found for %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    if is_stream_state_stopping(state) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot start HLS stream %s while it is in the process of being stopped\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    stream_state_add_ref(state, stream_component_t::STREAM_COMPONENT_HLS);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Added HLS reference to stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    pthread_mutex_lock(&raw mut unified_contexts_mutex);
    let mut running_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut running_indices: [::core::ffi::c_int; 1024] = [0; 1024];
    let mut valid_connection_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if !unified_contexts[i as usize].is_null()
            && strcmp(
                &raw mut (**(&raw mut unified_contexts as *mut *mut hls_unified_thread_ctx_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            let c2rust_fresh0 = running_count;
            running_count += 1;
            running_indices[c2rust_fresh0 as usize] = i;
            if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (**(&raw mut unified_contexts as *mut *mut hls_unified_thread_ctx_t)
                    .offset(i as isize))
                .connection_valid,
            ) != 0
            {
                valid_connection_idx = i;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Found running HLS context for stream %s with valid connection at index %d\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    stream_name,
                    i,
                );
            }
        }
        i += 1;
    }
    if running_count > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Found %d running HLS contexts for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            running_count,
            stream_name,
        );
        let mut best_context_idx: ::core::ffi::c_int = 0;
        if valid_connection_idx >= 0 as ::core::ffi::c_int {
            best_context_idx = valid_connection_idx;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Keeping HLS context with valid connection for stream %s at index %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                best_context_idx,
            );
        } else {
            best_context_idx = running_indices[0usize];
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"No valid connections found, keeping first HLS context for stream %s at index %d\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                best_context_idx,
            );
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < running_count {
            let mut idx: ::core::ffi::c_int = running_indices[i_0 as usize];
            if idx != best_context_idx {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Stopping duplicate HLS context at index %d for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    idx,
                    stream_name,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (**(&raw mut unified_contexts as *mut *mut hls_unified_thread_ctx_t)
                        .offset(idx as isize))
                    .running,
                    0 as ::core::ffi::c_int,
                );
            }
            i_0 += 1;
        }
        if best_context_idx >= 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Using existing HLS context for stream %s at index %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                best_context_idx,
            );
            pthread_mutex_unlock(&raw mut unified_contexts_mutex);
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < g_config.max_streams {
        if unified_contexts[i_1 as usize].is_null() {
            slot = i_1;
            break;
        } else {
            i_1 += 1;
        }
    }
    if slot == -1 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No slot available for new HLS stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut unified_contexts_mutex);
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Clearing any existing HLS segments for stream %s before starting\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    clear_stream_hls_segments(stream_name);
    let mut ctx: *mut hls_unified_thread_ctx_t = hls_guarded_malloc(::core::mem::size_of::<
        hls_unified_thread_ctx_t,
    >()) as *mut hls_unified_thread_ctx_t;
    if ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for unified HLS context\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<hls_unified_thread_ctx_t>(),
    );
    safe_strcpy(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        stream_name,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    let mut actual_url: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut actual_url as *mut ::core::ffi::c_char,
        &raw mut config.url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    if go2rtc_integration_is_using_go2rtc_for_hls(stream_name) {
        let mut rtsp_retries: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
        let mut rtsp_url_success: bool = r#false != 0;
        while rtsp_retries > 0 as ::core::ffi::c_int && !rtsp_url_success {
            if go2rtc_get_rtsp_url(
                stream_name,
                &raw mut actual_url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            ) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Using go2rtc RTSP URL for HLS streaming on stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                );
                rtsp_url_success = r#true != 0;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to get go2rtc RTSP URL for stream %s (retries left: %d)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                    rtsp_retries - 1 as ::core::ffi::c_int,
                );
                rtsp_retries -= 1;
                if rtsp_retries > 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Waiting before retrying to get RTSP URL for stream %s...\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        stream_name,
                    );
                    pthread_mutex_unlock(&raw mut unified_contexts_mutex);
                    sleep(3 as ::core::ffi::c_uint);
                    pthread_mutex_lock(&raw mut unified_contexts_mutex);
                }
            }
        }
        if !rtsp_url_success {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to get go2rtc RTSP URL for stream %s after multiple attempts, falling back to original URL\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
        }
        if rtsp_url_success as ::core::ffi::c_int != 0
            && (!config.record_audio || g_config.audio_disabled as ::core::ffi::c_int != 0)
        {
            let mut url_len: size_t = strlen(&raw mut actual_url as *mut ::core::ffi::c_char);
            let mut suffix: *const ::core::ffi::c_char =
                b"?video\0".as_ptr() as *const ::core::ffi::c_char;
            let mut suffix_len: size_t = strlen(suffix);
            if url_len.wrapping_add(suffix_len)
                < ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
            {
                safe_strcat(
                    &raw mut actual_url as *mut ::core::ffi::c_char,
                    suffix,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Audio recording disabled for %s, using video-only go2rtc RTSP URL for HLS\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    stream_name,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"RTSP URL too long to append ?video selector for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                );
            }
        }
    }
    if !go2rtc_integration_is_using_go2rtc_for_hls(stream_name)
        || strcmp(
            &raw mut actual_url as *mut ::core::ffi::c_char,
            &raw mut config.url as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        let mut credentialed_url: [::core::ffi::c_char; 512] = [0; 512];
        if url_apply_credentials(
            &raw mut actual_url as *mut ::core::ffi::c_char,
            if config.onvif_username[0usize] as ::core::ffi::c_int != 0 {
                &raw mut config.onvif_username as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            if config.onvif_password[0usize] as ::core::ffi::c_int != 0 {
                &raw mut config.onvif_password as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            &raw mut credentialed_url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut actual_url as *mut ::core::ffi::c_char,
                &raw mut credentialed_url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
    }
    safe_strcpy(
        &raw mut (*ctx).rtsp_url as *mut ::core::ffi::c_char,
        &raw mut actual_url as *mut ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    (*ctx).protocol = config.protocol.0 as ::core::ffi::c_int;
    (*ctx).audio_enabled = (config.record_audio as ::core::ffi::c_int != 0
        && !g_config.audio_disabled) as ::core::ffi::c_int;
    if config.segment_duration > 0 as ::core::ffi::c_int
        && config.segment_duration <= 10 as ::core::ffi::c_int
    {
        (*ctx).segment_duration = config.segment_duration;
    } else {
        (*ctx).segment_duration = 2 as ::core::ffi::c_int;
    }
    if ensure_hls_directory(
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        stream_name,
    ) != 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create output directory %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
        );
        hls_guarded_free(ctx as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Created HLS directory with full permissions: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
    );
    let mut test_file: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut test_file as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/.test_write\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
    );
    let mut test_fd: ::core::ffi::c_int = open(
        &raw mut test_file as *mut ::core::ffi::c_char,
        O_WRONLY | O_CREAT | O_TRUNC,
        0o600 as ::core::ffi::c_int,
    );
    if test_fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Directory is not writable: %s (error: %s)\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        hls_guarded_free(ctx as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    close(test_fd);
    remove(&raw mut test_file as *mut ::core::ffi::c_char);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Verified HLS directory is writable: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).running,
        1 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).connection_valid,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).consecutive_failures,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).last_packet_time,
        time(::core::ptr::null_mut::<time_t>()),
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).thread_state,
        hls_thread_state_t::HLS_THREAD_INITIALIZING.0 as ::core::ffi::c_int,
    );
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    pthread_attr_init(&raw mut attr);
    pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
    );
    let mut thread_result: ::core::ffi::c_int = pthread_create(
        &raw mut (*ctx).thread,
        &raw mut attr,
        Some(
            hls_unified_thread_func
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        ctx as *mut ::core::ffi::c_void,
    );
    pthread_attr_destroy(&raw mut attr);
    if thread_result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create unified HLS thread for %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        hls_guarded_free(ctx as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    unified_contexts[slot as usize] = ctx;
    pthread_mutex_unlock(&raw mut unified_contexts_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Started unified HLS thread for %s in slot %d\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        slot,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn restart_hls_unified_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Force restarting HLS stream for %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    if is_shutdown_initiated() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot restart HLS stream %s during shutdown\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut state: *mut stream_state_manager_t = get_stream_state_by_name(stream_name);
    if !state.is_null() && is_stream_state_stopping(state) as ::core::ffi::c_int != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot restart HLS stream %s while it is in the process of being stopped\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Clearing HLS segments for stream %s before restart\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    clear_stream_hls_segments(stream_name);
    let mut stop_result: ::core::ffi::c_int = stop_hls_stream(stream_name);
    if stop_result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to stop HLS stream %s for restart, continuing anyway\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
    }
    usleep(500000 as ::core::ffi::c_int as __useconds_t);
    let mut start_result: ::core::ffi::c_int = start_hls_stream(stream_name);
    if start_result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to restart HLS stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Successfully restarted HLS stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_hls_unified_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Attempting to stop HLS stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    let mut state: *mut stream_state_manager_t = get_stream_state_by_name(stream_name);
    if !state.is_null() {
        let mut ctx_found: bool = r#false != 0;
        pthread_mutex_lock(&raw mut unified_contexts_mutex);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < g_config.max_streams {
            if !unified_contexts[i as usize].is_null()
                && strcmp(
                    &raw mut (**(&raw mut unified_contexts as *mut *mut hls_unified_thread_ctx_t)
                        .offset(i as isize))
                    .stream_name as *mut ::core::ffi::c_char,
                    stream_name,
                ) == 0 as ::core::ffi::c_int
            {
                ctx_found = r#true != 0;
                break;
            } else {
                i += 1;
            }
        }
        pthread_mutex_unlock(&raw mut unified_contexts_mutex);
        if ctx_found {
            set_stream_callbacks_enabled(state, r#false != 0);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Disabled callbacks for stream %s during HLS shutdown\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s not found in HLS contexts, not disabling callbacks\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        }
    }
    pthread_mutex_lock(&raw mut unified_contexts_mutex);
    let mut ctx: *mut hls_unified_thread_ctx_t =
        ::core::ptr::null_mut::<hls_unified_thread_ctx_t>();
    let mut index: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut contexts_found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut indices: [::core::ffi::c_int; 1024] = [0; 1024];
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < g_config.max_streams {
        if !unified_contexts[i_0 as usize].is_null()
            && (*unified_contexts[i_0 as usize]).stream_name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            && strcmp(
                &raw mut (**(&raw mut unified_contexts as *mut *mut hls_unified_thread_ctx_t)
                    .offset(i_0 as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            if contexts_found == 0 as ::core::ffi::c_int {
                ctx = unified_contexts[i_0 as usize];
                index = i_0;
            }
            indices[contexts_found as usize] = i_0;
            contexts_found += 1;
            found = 1 as ::core::ffi::c_int;
        }
        i_0 += 1;
    }
    if contexts_found > 1 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Found %d HLS contexts for stream %s during stop operation\0".as_ptr()
                as *const ::core::ffi::c_char,
            contexts_found,
            stream_name,
        );
        let mut i_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i_1 < contexts_found {
            let mut idx: ::core::ffi::c_int = indices[i_1 as usize];
            if !unified_contexts[idx as usize].is_null() && unified_contexts[idx as usize] != ctx {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Marking additional HLS context %d for stream %s as stopping\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    i_1,
                    stream_name,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (**(&raw mut unified_contexts as *mut *mut hls_unified_thread_ctx_t)
                        .offset(idx as isize))
                    .running,
                    0 as ::core::ffi::c_int,
                );
            }
            i_1 += 1;
        }
    }
    if found == 0 {
        pthread_mutex_unlock(&raw mut unified_contexts_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"HLS stream %s not found for stopping\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).running,
    ) == 0
    {
        pthread_mutex_unlock(&raw mut unified_contexts_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"HLS stream %s is already stopped\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return 0 as ::core::ffi::c_int;
    }
    mark_stream_stopping(stream_name);
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).running,
        0 as ::core::ffi::c_int,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Marked HLS stream %s as stopping (index: %d)\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        index,
    );
    reset_timestamp_tracker(stream_name);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Reset timestamp tracker for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    pthread_mutex_unlock(&raw mut unified_contexts_mutex);
    usleep(100000 as ::core::ffi::c_int as __useconds_t);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Waiting for detached thread for stream %s to exit on its own\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    let mut wait_attempts: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    while wait_attempts > 0 as ::core::ffi::c_int {
        if is_stream_stopping(stream_name) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Thread for stream %s is still stopping, waiting... (%d attempts left)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                wait_attempts,
            );
            usleep(500000 as ::core::ffi::c_int as __useconds_t);
            wait_attempts -= 1;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Thread for stream %s has completed its shutdown\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
            break;
        }
    }
    pthread_mutex_lock(&raw mut unified_contexts_mutex);
    if index >= 0 as ::core::ffi::c_int
        && index < MAX_STREAMS
        && unified_contexts[index as usize] == ctx
    {
        mark_context_pending_deletion(ctx as *mut ::core::ffi::c_void);
        ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
        if !ctx.is_null() && !is_context_already_freed(ctx as *mut ::core::ffi::c_void) {
            let mut writer_stream_name: [::core::ffi::c_char; 256] = [0; 256];
            safe_strcpy(
                &raw mut writer_stream_name as *mut ::core::ffi::c_char,
                stream_name,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
            let mut writer_to_cleanup: *const hls_writer_t = ::core::ptr::null::<hls_writer_t>();
            if !(*ctx).writer.is_null() {
                writer_to_cleanup = ::core::intrinsics::atomic_xchg::<
            *mut hls_writer_t,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(
                    &raw mut (*ctx).writer,
                    ::core::ptr::null_mut::<hls_writer_t>(),
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Clearing writer reference in context for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut writer_stream_name as *mut ::core::ffi::c_char,
                );
            }
            if !writer_to_cleanup.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Writer reference cleared for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut writer_stream_name as *mut ::core::ffi::c_char,
                );
            }
        }
        unified_contexts[index as usize] = ::core::ptr::null_mut::<hls_unified_thread_ctx_t>();
        pthread_mutex_unlock(&raw mut unified_contexts_mutex);
        if !ctx.is_null() {
            if is_context_already_freed(ctx as *mut ::core::ffi::c_void) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Context for stream %s has already been freed, skipping\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                );
                return 0 as ::core::ffi::c_int;
            }
            ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
            let mut ctx_to_free: *mut hls_unified_thread_ctx_t = ctx;
            ctx = ::core::ptr::null_mut::<hls_unified_thread_ctx_t>();
            ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
            let mut context_valid: bool = r#true != 0;
            if (*ctx_to_free).stream_name[0usize] as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Context has empty stream_name, may be corrupted\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                context_valid = r#false != 0;
            }
            let mut ptr: *const ::core::ffi::c_uchar = ctx_to_free as *const ::core::ffi::c_uchar;
            let mut all_zeros: bool = r#true != 0;
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2 < 16 as ::core::ffi::c_int
                && (i_2 as usize) < ::core::mem::size_of::<hls_unified_thread_ctx_t>()
            {
                if *ptr.offset(i_2 as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    all_zeros = r#false != 0;
                    break;
                } else {
                    i_2 += 1;
                }
            }
            if all_zeros {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Context memory appears to be zeroed out, may have been freed already\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                context_valid = r#false != 0;
            }
            if context_valid {
                if is_context_already_freed(ctx_to_free as *mut ::core::ffi::c_void) {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Context for stream %s has already been freed, skipping\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        stream_name,
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Freeing context for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                        stream_name,
                    );
                    mark_context_pending_deletion(ctx_to_free as *mut ::core::ffi::c_void);
                    ::core::intrinsics::atomic_fence::<
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >();
                    usleep(50000 as __useconds_t);
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Waiting for thread to exit for context %p (stream %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ctx_to_free,
                        stream_name,
                    );
                    wait_for_thread_exit(ctx_to_free as *mut ::core::ffi::c_void);
                    mark_context_as_freed(ctx_to_free as *mut ::core::ffi::c_void);
                    clear_context_pending_deletion(ctx_to_free as *const ::core::ffi::c_void);
                    hls_guarded_free(ctx_to_free as *mut ::core::ffi::c_void);
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Successfully freed context for stream %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        stream_name,
                    );
                }
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Skipping cleanup of potentially invalid context for stream %s (already cleaned up)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    stream_name,
                );
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Context is NULL during cleanup for stream %s (already cleaned up)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Successfully cleaned up resources for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
    } else {
        pthread_mutex_unlock(&raw mut unified_contexts_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Context for stream %s was modified during cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
    }
    pthread_mutex_lock(&raw mut unified_contexts_mutex);
    let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_3 < g_config.max_streams {
        if !unified_contexts[i_3 as usize].is_null()
            && (*unified_contexts[i_3 as usize]).stream_name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            && strcmp(
                &raw mut (**(&raw mut unified_contexts as *mut *mut hls_unified_thread_ctx_t)
                    .offset(i_3 as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Found additional HLS context for stream %s after primary context cleanup\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (**(&raw mut unified_contexts as *mut *mut hls_unified_thread_ctx_t)
                    .offset(i_3 as isize))
                .running,
                0 as ::core::ffi::c_int,
            );
            let mut extra_ctx: *mut hls_unified_thread_ctx_t = unified_contexts[i_3 as usize];
            unified_contexts[i_3 as usize] = ::core::ptr::null_mut::<hls_unified_thread_ctx_t>();
            mark_context_as_freed(extra_ctx as *mut ::core::ffi::c_void);
            hls_guarded_free(extra_ctx as *mut ::core::ffi::c_void);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cleaned up additional HLS context for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
        }
        i_3 += 1;
    }
    pthread_mutex_unlock(&raw mut unified_contexts_mutex);
    unmark_stream_stopping(stream_name);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stopped HLS stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    if !state.is_null() {
        ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
        set_stream_callbacks_enabled(state, r#true != 0);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Re-enabled callbacks for stream %s after HLS shutdown\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        usleep(100000 as ::core::ffi::c_int as __useconds_t);
        ::core::intrinsics::atomic_fence::<{ ::core::intrinsics::AtomicOrdering::SeqCst }>();
        stream_state_release_ref(state, stream_component_t::STREAM_COMPONENT_HLS);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Released HLS reference to stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_hls_stream_active(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut unified_contexts_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if !unified_contexts[i as usize].is_null()
            && strcmp(
                &raw mut (**(&raw mut unified_contexts as *mut *mut hls_unified_thread_ctx_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
            && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (**(&raw mut unified_contexts as *mut *mut hls_unified_thread_ctx_t)
                    .offset(i as isize))
                .running,
            ) != 0
            && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (**(&raw mut unified_contexts as *mut *mut hls_unified_thread_ctx_t)
                    .offset(i as isize))
                .connection_valid,
            ) != 0
        {
            pthread_mutex_unlock(&raw mut unified_contexts_mutex);
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    pthread_mutex_unlock(&raw mut unified_contexts_mutex);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn init_freed_contexts_tracking() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Initializing freed contexts tracking system\0".as_ptr() as *const ::core::ffi::c_char,
    );
    pthread_mutex_lock(&raw mut freed_contexts_mutex);
    memset(
        &raw mut freed_contexts as *mut *mut ::core::ffi::c_void as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[*mut ::core::ffi::c_void; 100]>(),
    );
    freed_contexts_count = 0 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut freed_contexts_mutex);
    pthread_mutex_lock(&raw mut pending_deletions_mutex);
    memset(
        &raw mut pending_deletions as *mut context_deletion_info_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[context_deletion_info_t; 1024]>(),
    );
    pthread_mutex_unlock(&raw mut pending_deletions_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Freed contexts tracking system initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn cleanup_freed_contexts_tracking() {
    pthread_mutex_lock(&raw mut pending_deletions_mutex);
    let mut has_pending: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if !pending_deletions[i as usize].ctx.is_null()
            && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
                    .offset(i as isize))
                .pending_deletion,
            ) != 0
        {
            has_pending = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Waiting for thread to exit for context %p\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pending_deletions[i as usize].ctx,
            );
            if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*(&raw mut pending_deletions as *mut context_deletion_info_t)
                    .offset(i as isize))
                .thread_exited,
            ) == 0
            {
                pthread_mutex_unlock(&raw mut pending_deletions_mutex);
                wait_for_thread_exit(pending_deletions[i as usize].ctx);
                pthread_mutex_lock(&raw mut pending_deletions_mutex);
            }
        }
        i += 1;
    }
    if has_pending {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"All pending deletions have been processed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    memset(
        &raw mut pending_deletions as *mut context_deletion_info_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[context_deletion_info_t; 1024]>(),
    );
    pthread_mutex_unlock(&raw mut pending_deletions_mutex);
    pthread_mutex_lock(&raw mut freed_contexts_mutex);
    memset(
        &raw mut freed_contexts as *mut *mut ::core::ffi::c_void as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[*mut ::core::ffi::c_void; 100]>(),
    );
    freed_contexts_count = 0 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut freed_contexts_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleaned up freed contexts tracking system\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_hls_unified_thread_system() {
    static mut in_system_cleanup: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if in_system_cleanup != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Already in HLS unified thread system cleanup, skipping to prevent double free\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    in_system_cleanup = 1 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleaning up HLS unified thread system...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    stop_hls_watchdog();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Performing final FFmpeg resource cleanup during system shutdown\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    cleanup_all_hls_writers();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Using safer approach to release FFmpeg memory during system shutdown\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut dummy: *mut ::core::ffi::c_void = av_malloc(16 as size_t);
    if !dummy.is_null() {
        av_free(dummy);
    }
    av_log_set_level(AV_LOG_QUIET);
    if ffmpeg_memory_cleanup_registered == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Registering global FFmpeg cleanup function with atexit()\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        atexit(Some(global_ffmpeg_cleanup as unsafe extern "C" fn() -> ()));
        ffmpeg_memory_cleanup_registered = 1 as ::core::ffi::c_int;
    }
    cleanup_freed_contexts_tracking();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"HLS unified thread system cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
    in_system_cleanup = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn init_ffmpeg_memory_cleanup() {
    if ffmpeg_memory_cleanup_registered == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Initializing FFmpeg memory cleanup system\0".as_ptr() as *const ::core::ffi::c_char,
        );
        atexit(Some(global_ffmpeg_cleanup as unsafe extern "C" fn() -> ()));
        ffmpeg_memory_cleanup_registered = 1 as ::core::ffi::c_int;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"FFmpeg memory cleanup system initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn is_thread_running(mut ctx: *mut hls_unified_thread_ctx_t) -> bool {
    if ctx.is_null() {
        return r#false != 0;
    }
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).running,
    ) == 0
    {
        return r#false != 0;
    }
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).thread_state,
    ) == hls_thread_state_t::HLS_THREAD_STOPPED.0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut last_packet: time_t = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut (*ctx).last_packet_time);
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if now - last_packet > MAX_PACKET_TIMEOUT as time_t * 3 as time_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Thread for stream %s has not received a packet in %ld seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            now - last_packet,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn hls_watchdog_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"HLSWatchdog\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"HLS watchdog thread started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    while ::core::sync::atomic::AtomicBool::from_ptr(&raw mut watchdog_running).load(::core::sync::atomic::Ordering::SeqCst) {
        sleep(WATCHDOG_CHECK_INTERVAL_SEC as ::core::ffi::c_uint);
        if !::core::sync::atomic::AtomicBool::from_ptr(&raw mut watchdog_running).load(::core::sync::atomic::Ordering::SeqCst) || is_shutdown_initiated() as ::core::ffi::c_int != 0
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"HLS watchdog thread stopping\0".as_ptr() as *const ::core::ffi::c_char,
            );
            break;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"HLS watchdog checking thread status...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            pthread_mutex_lock(&raw mut unified_contexts_mutex);
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < g_config.max_streams {
                let mut ctx: *mut hls_unified_thread_ctx_t = unified_contexts[i as usize];
                if !ctx.is_null() {
                    if !is_context_pending_deletion(ctx as *const ::core::ffi::c_void) {
                        if !is_thread_running(ctx) {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"HLS thread for stream %s has exited unexpectedly\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                            );
                            let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
                            if now - last_restart_time[i as usize]
                                < WATCHDOG_RESTART_COOLDOWN_SEC as time_t
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Cooldown period not elapsed for stream %s, not restarting yet\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                                );
                            } else if restart_attempts[i as usize] >= WATCHDOG_MAX_RESTART_ATTEMPTS
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_ERROR,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Maximum restart attempts (%d) exceeded for stream %s, not restarting\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                                );
                            } else {
                                let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
                                safe_strcpy(
                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                                    MAX_STREAM_NAME as size_t,
                                    0 as size_t,
                                );
                                pthread_mutex_unlock(&raw mut unified_contexts_mutex);
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Attempting to restart HLS thread for stream %s (attempt %d)\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                    restart_attempts[i as usize] + 1 as ::core::ffi::c_int,
                                );
                                stop_hls_unified_stream(
                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                );
                                usleep(1000000 as ::core::ffi::c_int as __useconds_t);
                                let mut result: ::core::ffi::c_int = start_hls_unified_stream(
                                    &raw mut stream_name as *mut ::core::ffi::c_char,
                                );
                                if result == 0 as ::core::ffi::c_int {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_INFO,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Successfully restarted HLS thread for stream %s\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                    );
                                    pthread_mutex_lock(&raw mut unified_contexts_mutex);
                                    last_restart_time[i as usize] =
                                        time(::core::ptr::null_mut::<time_t>());
                                    restart_attempts[i as usize] += 1;
                                    ::core::intrinsics::atomic_xadd::<
                                        _,
                                        _,
                                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                                    >(
                                        &raw mut watchdog_restart_count, 1 as ::core::ffi::c_int
                                    );
                                    pthread_mutex_unlock(&raw mut unified_contexts_mutex);
                                } else {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_ERROR,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Failed to restart HLS thread for stream %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        &raw mut stream_name as *mut ::core::ffi::c_char,
                                    );
                                    pthread_mutex_lock(&raw mut unified_contexts_mutex);
                                    last_restart_time[i as usize] =
                                        time(::core::ptr::null_mut::<time_t>());
                                    restart_attempts[i as usize] += 1;
                                    pthread_mutex_unlock(&raw mut unified_contexts_mutex);
                                }
                                pthread_mutex_lock(&raw mut unified_contexts_mutex);
                            }
                        } else {
                            let mut now_0: time_t = time(::core::ptr::null_mut::<time_t>());
                            if restart_attempts[i as usize] > 0 as ::core::ffi::c_int
                                && now_0 - last_restart_time[i as usize]
                                    > WATCHDOG_RESTART_COOLDOWN_SEC as time_t * 2 as time_t
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Resetting restart attempts for stream %s after stable period\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                                );
                                restart_attempts[i as usize] = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
                i += 1;
            }
            pthread_mutex_unlock(&raw mut unified_contexts_mutex);
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"HLS watchdog thread exited\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return NULL;
}
unsafe extern "C" fn start_hls_watchdog() {
    if ::core::sync::atomic::AtomicBool::from_ptr(&raw mut watchdog_running).load(::core::sync::atomic::Ordering::SeqCst) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"HLS watchdog thread is already running\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    memset(
        &raw mut last_restart_time as *mut time_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[time_t; 1024]>(),
    );
    memset(
        &raw mut restart_attempts as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_int; 1024]>(),
    );
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut watchdog_running).store(true, ::core::sync::atomic::Ordering::SeqCst);
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut watchdog_restart_count,
        0 as ::core::ffi::c_int,
    );
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    pthread_attr_init(&raw mut attr);
    pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
    );
    let mut result: ::core::ffi::c_int = pthread_create(
        &raw mut hls_watchdog_thread,
        &raw mut attr,
        Some(
            hls_watchdog_thread_func
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    );
    pthread_attr_destroy(&raw mut attr);
    if result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create HLS watchdog thread\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ::core::sync::atomic::AtomicBool::from_ptr(&raw mut watchdog_running).store(false, ::core::sync::atomic::Ordering::SeqCst);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Started HLS watchdog thread\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_hls_watchdog() {
    if !::core::sync::atomic::AtomicBool::from_ptr(&raw mut watchdog_running).load(::core::sync::atomic::Ordering::SeqCst) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"HLS watchdog thread is not running\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut watchdog_running).store(false, ::core::sync::atomic::Ordering::SeqCst);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Signaled HLS watchdog thread to stop\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_hls_watchdog_restart_count() -> ::core::ffi::c_int {
    return ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut watchdog_restart_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_hls_unified_thread_system() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Initializing HLS unified thread system...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    init_freed_contexts_tracking();
    init_ffmpeg_memory_cleanup();
    start_hls_watchdog();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"HLS unified thread system initialized with watchdog monitoring\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
