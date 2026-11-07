use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type AVDictionary;
    pub type AVOptionRanges;
    pub type AVOption;
    pub type AVBuffer;
    pub type AVIAMFMixPresentation;
    pub type AVIAMFAudioElement;
    pub type AVCodecTag;
    pub type AVCodecInternal;
    pub type stream_handle_s;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn av_dict_free(m: *mut *mut AVDictionary);
    unsafe fn av_log_set_level(level: ::core::ffi::c_int);
    unsafe fn recording_file_open(
        context: *mut *mut AVIOContext,
        path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn recording_io_report_failure(
        resource: recording_io_resource_t,
        operation: recording_io_operation_t,
        error_code: ::core::ffi::c_int,
    );
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn difftime(__time1: time_t, __time0: time_t) -> ::core::ffi::c_double;
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn av_strerror(
        errnum: ::core::ffi::c_int,
        errbuf: *mut ::core::ffi::c_char,
        errbuf_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn av_packet_alloc() -> *mut AVPacket;
    unsafe fn av_packet_free(pkt: *mut *mut AVPacket);
    unsafe fn av_packet_unref(pkt: *mut AVPacket);
    unsafe fn av_frame_alloc() -> *mut AVFrame;
    unsafe fn av_frame_free(frame: *mut *mut AVFrame);
    unsafe fn av_frame_unref(frame: *mut AVFrame);
    unsafe fn avcodec_parameters_alloc() -> *mut AVCodecParameters;
    unsafe fn avcodec_parameters_free(par: *mut *mut AVCodecParameters);
    unsafe fn avcodec_parameters_copy(
        dst: *mut AVCodecParameters,
        src: *const AVCodecParameters,
    ) -> ::core::ffi::c_int;
    unsafe fn avcodec_find_decoder(id: AVCodecID) -> *const AVCodec;
    unsafe fn avformat_alloc_context() -> *mut AVFormatContext;
    unsafe fn avformat_free_context(s: *mut AVFormatContext);
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
    unsafe fn avcodec_flush_buffers(avctx: *mut AVCodecContext);
    unsafe fn avcodec_descriptor_get(id: AVCodecID) -> *const AVCodecDescriptor;
    unsafe fn av_gettime_relative() -> int64_t;
    unsafe fn av_usleep(usec: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    unsafe fn log_ffmpeg_error(err: ::core::ffi::c_int, message: *const ::core::ffi::c_char);
    unsafe fn ffmpeg_frame_to_rgb24_padded(
        frame: *const AVFrame,
        rgb_size_out: *mut size_t,
    ) -> *mut uint8_t;
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
    unsafe fn mkdir_recursive(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn is_shutdown_initiated() -> bool;
    unsafe fn event_producer_publish_detection(
        camera_uuid: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
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
    unsafe fn init_packet_buffer_pool(memory_limit_mb: size_t) -> ::core::ffi::c_int;
    unsafe fn calculate_packet_buffer_pool_size() -> size_t;
    unsafe fn cleanup_packet_buffer_pool();
    unsafe fn create_packet_buffer(
        stream_name: *const ::core::ffi::c_char,
        buffer_seconds: ::core::ffi::c_int,
        mode: buffer_mode_t,
    ) -> *mut packet_buffer_t;
    unsafe fn destroy_packet_buffer(buffer: *mut packet_buffer_t);
    unsafe fn packet_buffer_add_packet(
        buffer: *mut packet_buffer_t,
        packet: *const AVPacket,
        timestamp: time_t,
    ) -> ::core::ffi::c_int;
    unsafe fn packet_buffer_flush(
        buffer: *mut packet_buffer_t,
        callback: Option<
            unsafe extern "C" fn(*const AVPacket, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        >,
        user_data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn packet_buffer_trim_before(
        buffer: *mut packet_buffer_t,
        cutoff: time_t,
        video_stream_index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn packet_buffer_clear(buffer: *mut packet_buffer_t);
    unsafe fn packet_buffer_get_stats(
        buffer: *mut packet_buffer_t,
        count: *mut ::core::ffi::c_int,
        memory_usage: *mut size_t,
        duration: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn stream_frame_rate_select(
        average: AVRational,
        nominal: AVRational,
        provisional: *mut bool,
    ) -> ::core::ffi::c_double;
    unsafe fn stream_frame_rate_round(fps: ::core::ffi::c_double) -> ::core::ffi::c_int;
    unsafe fn stream_frame_rate_observe(
        measurement: *mut stream_frame_rate_measurement_t,
        now_us: int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn load_detection_model(
        model_path: *const ::core::ffi::c_char,
        threshold: ::core::ffi::c_float,
    ) -> detection_model_t;
    unsafe fn unload_detection_model(model: detection_model_t);
    unsafe fn mp4_writer_create(
        output_path: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    ) -> *mut mp4_writer_t;
    unsafe fn mp4_writer_write_packet(
        writer: *mut mp4_writer_t,
        pkt: *const AVPacket,
        input_stream: *const AVStream,
    ) -> ::core::ffi::c_int;
    unsafe fn mp4_writer_close(writer: *mut mp4_writer_t);
    unsafe fn mp4_writer_set_audio(writer: *mut mp4_writer_t, enable: ::core::ffi::c_int);
    unsafe fn storage_placement_select(
        stream_name: *const ::core::ffi::c_char,
        placement: *mut storage_placement_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_config(
        handle: stream_handle_t,
        config: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_by_name(name: *const ::core::ffi::c_char) -> stream_handle_t;
    unsafe fn db_detection_engines_list(
        stream_name: *const ::core::ffi::c_char,
        engines: *mut stream_detection_engine_t,
        capacity: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn detect_motion_onvif(
        onvif_url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        result: *mut detection_result_t,
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn detect_objects(
        model: detection_model_t,
        frame_data: *const ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        channels: ::core::ffi::c_int,
        result: *mut detection_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn api_detection_options_from_config(options: *mut api_detection_options_t);
    unsafe fn api_detection_options_apply_json(
        options: *mut api_detection_options_t,
        config_json: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn detect_objects_api_with_options(
        api_url: *const ::core::ffi::c_char,
        frame_data: *const ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        channels: ::core::ffi::c_int,
        result: *mut detection_result_t,
        stream_name: *const ::core::ffi::c_char,
        threshold: ::core::ffi::c_float,
        recording_id: uint64_t,
        frame_timestamp: time_t,
        options: *const api_detection_options_t,
    ) -> ::core::ffi::c_int;
    unsafe fn configure_motion_detection(
        stream_name: *const ::core::ffi::c_char,
        sensitivity: ::core::ffi::c_float,
        min_motion_area: ::core::ffi::c_float,
        cooldown_time: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn detect_motion(
        stream_name: *const ::core::ffi::c_char,
        frame_data: *const ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        channels: ::core::ffi::c_int,
        frame_time: time_t,
        result: *mut detection_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn set_motion_detection_enabled(
        stream_name: *const ::core::ffi::c_char,
        enabled: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn mp4_writer_initialize(
        writer: *mut mp4_writer_t,
        pkt: *const AVPacket,
        input_stream: *const AVStream,
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
    unsafe fn set_audio_voice_enhancement(stream_name: *const ::core::ffi::c_char, enabled: bool);
    unsafe fn get_current_recording_id_for_stream(
        stream_name: *const ::core::ffi::c_char,
    ) -> uint64_t;
    unsafe fn stream_has_continuous_writer(stream_name: *const ::core::ffi::c_char) -> bool;
    unsafe fn build_mp4_recording_directory_for_placement(
        config: *const config_t,
        placement: *const storage_placement_t,
        stream_name: *const ::core::ffi::c_char,
        timestamp: time_t,
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn filter_detections_by_zones(
        stream_name: *const ::core::ffi::c_char,
        result: *mut detection_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn filter_detections_by_stream_objects(
        stream_name: *const ::core::ffi::c_char,
        result: *mut detection_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_streaming_config() -> *mut config_t;
    unsafe fn get_stream_state_by_name(
        name: *const ::core::ffi::c_char,
    ) -> *mut stream_state_manager_t;
    unsafe fn handle_stream_error(
        state: *mut stream_state_manager_t,
        error_code: ::core::ffi::c_int,
        error_message: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_stream_get_rtsp_url(
        stream_id: *const ::core::ffi::c_char,
        buffer: *mut ::core::ffi::c_char,
        buffer_size: size_t,
    ) -> bool;
    unsafe fn go2rtc_snapshot_cleanup_thread();
    unsafe fn go2rtc_integration_reload_stream_config(
        stream_name: *const ::core::ffi::c_char,
        new_url: *const ::core::ffi::c_char,
        new_username: *const ::core::ffi::c_char,
        new_password: *const ::core::ffi::c_char,
        new_backchannel_enabled: ::core::ffi::c_int,
        new_protocol: ::core::ffi::c_int,
        new_record_audio: ::core::ffi::c_int,
    ) -> bool;
    unsafe fn add_recording_metadata(metadata: *const recording_metadata_t) -> uint64_t;
    unsafe fn update_recording_metadata(
        id: uint64_t,
        end_time: time_t,
        size_bytes: uint64_t,
        is_complete: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn update_recording_start_time(id: uint64_t, start_time: time_t) -> ::core::ffi::c_int;
    unsafe fn store_detections_in_db_for_camera(
        camera_uuid: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
        timestamp: time_t,
        recording_id: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn update_detections_recording_id(
        stream_name: *const ::core::ffi::c_char,
        recording_id: uint64_t,
        since_time: time_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn update_stream_video_params(
        stream_name: *const ::core::ffi::c_char,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        fps: ::core::ffi::c_int,
        codec: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn url_apply_credentials(
        url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn url_build_onvif_service_url(
        url: *const ::core::ffi::c_char,
        onvif_port: ::core::ffi::c_int,
        service_path: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn url_redact_for_logging(
        url: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn update_stream_storage_cache_add_recording(
        stream_name: *const ::core::ffi::c_char,
        size_bytes: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn metrics_record_frame_from_source(
        stream_name: *const ::core::ffi::c_char,
        bytes: ::core::ffi::c_int,
        is_video: bool,
        source: metrics_frame_source_t,
    );
    unsafe fn metrics_record_segment_complete(
        stream_name: *const ::core::ffi::c_char,
        start_time: time_t,
        end_time: time_t,
        bytes: uint64_t,
    );
    unsafe fn metrics_set_recording_active(stream_name: *const ::core::ffi::c_char, active: bool);
    unsafe fn metrics_set_configured_fps(
        stream_name: *const ::core::ffi::c_char,
        fps: ::core::ffi::c_double,
    );
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
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
pub type __useconds_t = ::core::ffi::c_uint;
pub type __clockid_t = ::core::ffi::c_int;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int_fast64_t = ::core::ffi::c_long;
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
pub struct api_detection_format_t(pub ::core::ffi::c_uint);
impl api_detection_format_t {
    pub const API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT: Self = Self(0);
    pub const API_DETECTION_FORMAT_DOODS2: Self = Self(1);
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
pub type atomic_bool = bool;
pub type atomic_int = ::core::ffi::c_int;
pub type atomic_llong = ::core::ffi::c_longlong;
pub type atomic_int_fast64_t = int_fast64_t;
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
pub struct buffer_mode_t(pub ::core::ffi::c_uint);
impl buffer_mode_t {
    pub const BUFFER_MODE_MEMORY: Self = Self(0);
    pub const BUFFER_MODE_DISK: Self = Self(1);
    pub const BUFFER_MODE_HYBRID: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffered_packet_t {
    pub packet: *mut AVPacket,
    pub timestamp: time_t,
    pub pts: int64_t,
    pub dts: int64_t,
    pub stream_index: ::core::ffi::c_int,
    pub is_keyframe: bool,
    pub data_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct packet_buffer_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub buffer_seconds: ::core::ffi::c_int,
    pub max_packets: ::core::ffi::c_int,
    pub mode: buffer_mode_t,
    pub packets: *mut buffered_packet_t,
    pub head: ::core::ffi::c_int,
    pub tail: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub total_packets_buffered: uint64_t,
    pub total_packets_dropped: uint64_t,
    pub total_bytes_buffered: uint64_t,
    pub current_memory_usage: size_t,
    pub peak_memory_usage: size_t,
    pub oldest_packet_time: time_t,
    pub newest_packet_time: time_t,
    pub disk_buffer_path: [::core::ffi::c_char; 512],
    pub disk_buffer_file: *mut FILE,
    pub mutex: pthread_mutex_t,
    pub mutex_initialized: bool,
    pub active: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_frame_rate_measurement_t {
    pub started: bool,
    pub warmup_until_us: int64_t,
    pub window_start_us: int64_t,
    pub packets: uint64_t,
    pub fps: ::core::ffi::c_int,
}
pub type detection_model_t = *mut ::core::ffi::c_void;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_status_t(pub ::core::ffi::c_uint);
impl stream_status_t {
    pub const STREAM_STATUS_UNKNOWN: Self = Self(0);
    pub const STREAM_STATUS_STOPPED: Self = Self(1);
    pub const STREAM_STATUS_STARTING: Self = Self(2);
    pub const STREAM_STATUS_RUNNING: Self = Self(3);
    pub const STREAM_STATUS_STOPPING: Self = Self(4);
    pub const STREAM_STATUS_ERROR: Self = Self(5);
    pub const STREAM_STATUS_RECONNECTING: Self = Self(6);
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
pub type stream_handle_t = *mut stream_handle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_detection_engine_t {
    pub id: uint64_t,
    pub engine_key: [::core::ffi::c_char; 64],
    pub engine_type: [::core::ffi::c_char; 16],
    pub model_path: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub threshold: ::core::ffi::c_float,
    pub interval_seconds: ::core::ffi::c_int,
    pub sort_order: ::core::ffi::c_int,
    pub config_json: [::core::ffi::c_char; 1024],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct unified_detection_state_t(pub ::core::ffi::c_uint);
impl unified_detection_state_t {
    pub const UDT_STATE_INITIALIZING: Self = Self(0);
    pub const UDT_STATE_CONNECTING: Self = Self(1);
    pub const UDT_STATE_BUFFERING: Self = Self(2);
    pub const UDT_STATE_RECORDING: Self = Self(3);
    pub const UDT_STATE_POST_BUFFER: Self = Self(4);
    pub const UDT_STATE_RECONNECTING: Self = Self(5);
    pub const UDT_STATE_STOPPING: Self = Self(6);
    pub const UDT_STATE_STOPPED: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unified_detection_ctx_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub rtsp_url: [::core::ffi::c_char; 512],
    pub output_dir: [::core::ffi::c_char; 512],
    pub thread: pthread_t,
    pub running: atomic_int,
    pub state: atomic_int,
    pub shutdown_component_id: ::core::ffi::c_int,
    pub slot_idx: ::core::ffi::c_int,
    pub model_path: [::core::ffi::c_char; 512],
    pub model: detection_model_t,
    pub detection_threshold: ::core::ffi::c_float,
    pub detection_interval: ::core::ffi::c_int,
    pub pre_buffer_seconds: ::core::ffi::c_int,
    pub post_buffer_seconds: ::core::ffi::c_int,
    pub segment_duration: ::core::ffi::c_int,
    pub packet_buffer: *mut packet_buffer_t,
    pub mp4_writer: *mut mp4_writer_t,
    pub current_recording_path: [::core::ffi::c_char; 512],
    pub placement: storage_placement_t,
    pub current_recording_id: uint64_t,
    pub last_detection_time: atomic_llong,
    pub last_detection_check_time: atomic_llong,
    pub post_buffer_end_time: atomic_llong,
    pub log_counter: atomic_int,
    pub model_load_failed: bool,
    pub engines: [stream_detection_engine_t; 8],
    pub engine_models: [detection_model_t; 8],
    pub engine_load_failed: [bool; 8],
    pub engine_last_run: [time_t; 8],
    pub engine_count: ::core::ffi::c_int,
    pub recording_grace_interval: ::core::ffi::c_int,
    pub has_onvif_engine: bool,
    pub last_packet_time: atomic_int_fast64_t,
    pub consecutive_failures: atomic_int,
    pub reconnect_attempt: ::core::ffi::c_int,
    pub record_audio: bool,
    pub audio_voice_enhancement: bool,
    pub external_motion_trigger: atomic_int,
    pub annotation_only: bool,
    pub onvif_thread: pthread_t,
    pub onvif_thread_running: atomic_int,
    pub onvif_motion_detected: atomic_int,
    pub onvif_motion_timestamp: atomic_llong,
    pub onvif_motion_onset: atomic_llong,
    pub onvif_onset_handled: ::core::ffi::c_longlong,
    pub onvif_url_cached: [::core::ffi::c_char; 512],
    pub onvif_username_cached: [::core::ffi::c_char; 64],
    pub onvif_password_cached: [::core::ffi::c_char; 64],
    pub detection_stream_thread: pthread_t,
    pub detection_stream_thread_running: atomic_int,
    pub detection_stream_connected: atomic_int,
    pub detection_stream_result: atomic_int,
    pub detection_stream_pending: detection_result_t,
    pub detection_stream_pending_ts: time_t,
    pub detection_stream_result_mutex: pthread_mutex_t,
    pub detection_stream_url: [::core::ffi::c_char; 512],
    pub input_ctx: *mut AVFormatContext,
    pub decoder_ctx: *mut AVCodecContext,
    pub video_stream_idx: ::core::ffi::c_int,
    pub audio_stream_idx: ::core::ffi::c_int,
    pub input_io_deadline_us: int64_t,
    pub detection_io_deadline_us: int64_t,
    pub total_packets_processed: uint64_t,
    pub total_detections: uint64_t,
    pub total_recordings: uint64_t,
    pub fps_is_provisional: atomic_bool,
    pub fps_measurement: stream_frame_rate_measurement_t,
    pub stream_connect_time: time_t,
    pub first_video_pts: int64_t,
    pub first_video_pts_set: bool,
    pub stream_is_live: bool,
    pub mutex: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_metadata_t {
    pub id: uint64_t,
    pub stream_name: [::core::ffi::c_char; 64],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub file_path: [::core::ffi::c_char; 512],
    pub storage_target_uuid: [::core::ffi::c_char; 37],
    pub object_key: [::core::ffi::c_char; 512],
    pub placement_reason: [::core::ffi::c_char; 64],
    pub storage_policy_version: int64_t,
    pub start_time: time_t,
    pub end_time: time_t,
    pub size_bytes: uint64_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_int,
    pub codec: [::core::ffi::c_char; 16],
    pub is_complete: bool,
    pub playback_issue: [::core::ffi::c_char; 32],
    pub trigger_type: [::core::ffi::c_char; 16],
    pub protected: bool,
    pub retention_override_days: ::core::ffi::c_int,
    pub retention_tier: ::core::ffi::c_int,
    pub disk_pressure_eligible: bool,
    pub schedule_restricted: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_detection_options_t {
    pub format: api_detection_format_t,
    pub backend: [::core::ffi::c_char; 32],
    pub detector_name: [::core::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flush_callback_ctx_t {
    pub ctx: *mut unified_detection_ctx_t,
    pub packets_written: ::core::ffi::c_int,
    pub found_keyframe: bool,
    pub writer_initialized: bool,
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
pub const AV_LOG_QUIET: ::core::ffi::c_int = -8 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AV_NOPTS_VALUE: int64_t = 0x8000000000000000 as ::core::ffi::c_ulong as int64_t;
pub const AV_TIME_BASE: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn av_q2d(mut a: AVRational) -> ::core::ffi::c_double {
    return a.num as ::core::ffi::c_double / a.den as ::core::ffi::c_double;
}
pub const AV_PKT_FLAG_KEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const AV_CODEC_PROP_INTRA_ONLY: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MAX_BUFFER_SECONDS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const MIN_BUFFER_SECONDS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PACKET_BUFFER_TRIGGER_HEADROOM_SECONDS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const STREAM_FRAME_RATE_DEFAULT: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const MAX_DETECTION_ENGINES_PER_STREAM: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MAX_UNIFIED_DETECTION_THREADS: ::core::ffi::c_int = MAX_STREAMS;
pub const BASE_RECONNECT_DELAY_MS: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const MAX_RECONNECT_DELAY_MS: ::core::ffi::c_int = 30000 as ::core::ffi::c_int;
pub const MAX_PACKET_TIMEOUT_SEC: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const OPEN_TIMEOUT_US: ::core::ffi::c_longlong =
    5 as ::core::ffi::c_longlong * AV_TIME_BASE as ::core::ffi::c_longlong;
pub const READ_TIMEOUT_US: int64_t = MAX_PACKET_TIMEOUT_SEC as int64_t * AV_TIME_BASE as int64_t;
pub const DEFAULT_DETECTION_INTERVAL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ONVIF_MOTION_HOLD_SECS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const ONVIF_MIN_POLL_INTERVAL_MS: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
static mut DEFAULT_MOTION_SENSITIVITY: ::core::ffi::c_float = 0.15f32;
static mut DEFAULT_MOTION_MIN_AREA_RATIO: ::core::ffi::c_float = 0.005f32;
static mut DEFAULT_MOTION_COOLDOWN_SECS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
static mut detection_contexts: [*mut unified_detection_ctx_t; 1024] = [
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
    ::core::ptr::null_mut::<unified_detection_ctx_t>(),
];
static mut contexts_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut system_initialized: bool = r#false != 0;
unsafe extern "C" fn get_actual_api_url(
    mut stream_name: *const ::core::ffi::c_char,
    mut model_path: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut actual_api_url: *const ::core::ffi::c_char = model_path;
    if !model_path.is_null()
        && strcmp(
            model_path,
            b"api-detection\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        if (&raw mut g_config.api_detection_url as *mut ::core::ffi::c_char).is_null()
            || g_config.api_detection_url[0usize] as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] api_detection_url is not configured for api-detection model_path\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
            );
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        actual_api_url = &raw mut g_config.api_detection_url as *mut ::core::ffi::c_char;
    }
    return actual_api_url;
}
unsafe extern "C" fn resolve_api_detection_options(
    mut ctx: *const unified_detection_ctx_t,
    mut options: *mut api_detection_options_t,
) {
    api_detection_options_from_config(options);
    if ctx.is_null() || (*ctx).engine_count != 1 as ::core::ffi::c_int {
        return;
    }
    let mut engine: *const stream_detection_engine_t =
        (&raw const (*ctx).engines as *const stream_detection_engine_t).offset(0isize);
    if strcmp(
        &raw const (*engine).engine_type as *const ::core::ffi::c_char,
        b"api\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        || (*engine).config_json[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return;
    }
    if api_detection_options_apply_json(
        options,
        &raw const (*engine).config_json as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Detection engine %s has an invalid api detection config; using global settings\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw const (*ctx).stream_name as *const ::core::ffi::c_char,
            &raw const (*engine).engine_key as *const ::core::ffi::c_char,
        );
        api_detection_options_from_config(options);
    }
}
unsafe extern "C" fn detection_link_recording_id(
    mut ctx: *mut unified_detection_ctx_t,
) -> uint64_t {
    if ctx.is_null() {
        return 0 as uint64_t;
    }
    let mut detection_id: uint64_t = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut (*ctx).current_recording_id);
    if detection_id != 0 as uint64_t {
        return detection_id;
    }
    return get_current_recording_id_for_stream(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn should_annotate_continuous(mut ctx: *mut unified_detection_ctx_t) -> bool {
    if ctx.is_null() {
        return r#false != 0;
    }
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).current_recording_id,
    ) != 0 as uint64_t
    {
        return r#false != 0;
    }
    if !(*ctx).annotation_only {
        return r#false != 0;
    }
    return stream_has_continuous_writer(&raw mut (*ctx).stream_name as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn log_stream_info_failure(
    mut stream_name: *const ::core::ffi::c_char,
    mut source: *const ::core::ffi::c_char,
    mut format: *const AVFormatContext,
    mut ret: ::core::ffi::c_int,
    mut deadline_us: int64_t,
    mut cancelled: bool,
) {
    let mut error: [::core::ffi::c_char; 64] = [0; 64];
    if ret < 0 as ::core::ffi::c_int {
        av_strerror(
            ret,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        );
    } else {
        safe_strcpy(
            &raw mut error as *mut ::core::ffi::c_char,
            b"probe completed but was interrupted\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    let mut timed_out: bool = deadline_us > 0 as int64_t && av_gettime_relative() >= deadline_us;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] %s: Failed to find stream info: %s (ret=%d, timeout=%s, cancelled=%s, streams=%u)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        source,
        &raw mut error as *mut ::core::ffi::c_char,
        ret,
        if timed_out as ::core::ffi::c_int != 0 {
            b"yes\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"no\0".as_ptr() as *const ::core::ffi::c_char
        },
        if cancelled as ::core::ffi::c_int != 0 {
            b"yes\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"no\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !format.is_null() {
            (*format).nb_streams
        } else {
            0 as ::core::ffi::c_uint
        },
    );
    if format.is_null() {
        return;
    }
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < (*format).nb_streams {
        let mut stream: *const AVStream = *(*format).streams.offset(i as isize);
        if (*(*stream).codecpar).codec_type.0 == AVMediaType::AVMEDIA_TYPE_VIDEO.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] %s probe video: codec=%d, size=%dx%d, avg=%d/%d, nominal=%d/%d, time_base=%d/%d\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                source,
                (*(*stream).codecpar).codec_id.0,
                (*(*stream).codecpar).width,
                (*(*stream).codecpar).height,
                (*stream).avg_frame_rate.num,
                (*stream).avg_frame_rate.den,
                (*stream).r_frame_rate.num,
                (*stream).r_frame_rate.den,
                (*stream).time_base.num,
                (*stream).time_base.den,
            );
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ffmpeg_interrupt_callback(
    mut opaque: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut unified_detection_ctx_t = opaque as *mut unified_detection_ctx_t;
    if ctx.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).running,
    ) == 0
        || is_shutdown_initiated() as ::core::ffi::c_int != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    return ((*ctx).input_io_deadline_us > 0 as int64_t
        && av_gettime_relative() >= (*ctx).input_io_deadline_us) as ::core::ffi::c_int;
}
unsafe extern "C" fn is_api_detection(mut model_path: *const ::core::ffi::c_char) -> bool {
    if model_path.is_null()
        || *model_path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if strcmp(
        model_path,
        b"api-detection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return r#true != 0;
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
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn is_motion_detection_model(mut model_path: *const ::core::ffi::c_char) -> bool {
    if model_path.is_null()
        || *model_path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    return strcmp(
        model_path,
        b"motion\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_onvif_detection_model(mut model_path: *const ::core::ffi::c_char) -> bool {
    if model_path.is_null()
        || *model_path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    return strcmp(
        model_path,
        b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn runtime_has_engine_type(
    mut ctx: *const unified_detection_ctx_t,
    mut engine_type: *const ::core::ffi::c_char,
) -> bool {
    if ctx.is_null() || engine_type.is_null() {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*ctx).engine_count {
        if (*ctx).engines[i as usize].enabled as ::core::ffi::c_int != 0
            && strcmp(
                &raw const (*(&raw const (*ctx).engines as *const stream_detection_engine_t)
                    .offset(i as isize))
                .engine_type as *const ::core::ffi::c_char,
                engine_type,
            ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn unload_runtime_models(mut ctx: *mut unified_detection_ctx_t) {
    if ctx.is_null() {
        return;
    }
    if !(*ctx).model.is_null() {
        unload_detection_model((*ctx).model);
        (*ctx).model = NULL as detection_model_t;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*ctx).engine_count {
        if !(*ctx).engine_models[i as usize].is_null() {
            unload_detection_model((*ctx).engine_models[i as usize]);
            (*ctx).engine_models[i as usize] = NULL as detection_model_t;
        }
        i += 1;
    }
}
unsafe extern "C" fn load_runtime_engine_config(
    mut ctx: *mut unified_detection_ctx_t,
    mut legacy_model: *const ::core::ffi::c_char,
    mut legacy_threshold: ::core::ffi::c_float,
    mut legacy_interval: ::core::ffi::c_int,
) {
    let mut stored: [stream_detection_engine_t; 8] = [stream_detection_engine_t {
        id: 0,
        engine_key: [0; 64],
        engine_type: [0; 16],
        model_path: [0; 512],
        enabled: false,
        threshold: 0.,
        interval_seconds: 0,
        sort_order: 0,
        config_json: [0; 1024],
    }; 8];
    let mut count: ::core::ffi::c_int = db_detection_engines_list(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        &raw mut stored as *mut stream_detection_engine_t,
        MAX_DETECTION_ENGINES_PER_STREAM as size_t,
    );
    (*ctx).engine_count = 0 as ::core::ffi::c_int;
    (*ctx).recording_grace_interval = legacy_interval;
    if count > 0 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < count {
            if stored[i as usize].enabled {
                let c2rust_fresh1 = (*ctx).engine_count;
                (*ctx).engine_count += 1;
                (*ctx).engines[c2rust_fresh1 as usize] = stored[i as usize];
            }
            i += 1;
        }
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*ctx).engine_count {
        let mut engine: *mut stream_detection_engine_t =
            (&raw mut (*ctx).engines as *mut stream_detection_engine_t).offset(i_0 as isize);
        if strcmp(
            &raw mut (*engine).engine_key as *mut ::core::ffi::c_char,
            b"legacy-primary\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && !legacy_model.is_null()
            && *legacy_model.offset(0isize) as ::core::ffi::c_int != 0
        {
            safe_strcpy(
                &raw mut (*engine).model_path as *mut ::core::ffi::c_char,
                legacy_model,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
        if (*engine).model_path[0usize] == 0
            && (strcmp(
                &raw mut (*engine).engine_type as *mut ::core::ffi::c_char,
                b"motion\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut (*engine).engine_type as *mut ::core::ffi::c_char,
                    b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int)
        {
            safe_strcpy(
                &raw mut (*engine).model_path as *mut ::core::ffi::c_char,
                &raw mut (*engine).engine_type as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
        if strcmp(
            &raw mut (*engine).engine_type as *mut ::core::ffi::c_char,
            b"object\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && (*engine).model_path[0usize] as ::core::ffi::c_int != 0
            && (*engine).model_path[0usize] as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        {
            let mut resolved: [::core::ffi::c_char; 512] = [0; 512];
            let mut base: *const ::core::ffi::c_char =
                if g_config.models_path[0usize] as ::core::ffi::c_int != 0 {
                    &raw mut g_config.models_path as *mut ::core::ffi::c_char
                } else {
                    b"/etc/lightnvr/models\0".as_ptr() as *const ::core::ffi::c_char
                };
            let mut written: ::core::ffi::c_int = snprintf(
                &raw mut resolved as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
                base,
                &raw mut (*engine).model_path as *mut ::core::ffi::c_char,
            );
            if written > 0 as ::core::ffi::c_int
                && (written as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
            {
                safe_strcpy(
                    &raw mut (*engine).model_path as *mut ::core::ffi::c_char,
                    &raw mut resolved as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    0 as size_t,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] Detection engine %s model path is too long\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    &raw mut (*engine).engine_key as *mut ::core::ffi::c_char,
                );
            }
        }
        i_0 += 1;
    }
    if (*ctx).engine_count == 0 as ::core::ffi::c_int
        && !legacy_model.is_null()
        && *legacy_model.offset(0isize) as ::core::ffi::c_int != 0
    {
        let mut engine_0: *mut stream_detection_engine_t =
            (&raw mut (*ctx).engines as *mut stream_detection_engine_t).offset(0isize);
        memset(
            engine_0 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stream_detection_engine_t>(),
        );
        safe_strcpy(
            &raw mut (*engine_0).engine_key as *mut ::core::ffi::c_char,
            b"legacy-primary\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*engine_0).model_path as *mut ::core::ffi::c_char,
            legacy_model,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        let mut r#type: *const ::core::ffi::c_char =
            if is_motion_detection_model(legacy_model) as ::core::ffi::c_int != 0 {
                b"motion\0".as_ptr() as *const ::core::ffi::c_char
            } else if is_onvif_detection_model(legacy_model) as ::core::ffi::c_int != 0 {
                b"onvif\0".as_ptr() as *const ::core::ffi::c_char
            } else if is_api_detection(legacy_model) as ::core::ffi::c_int != 0 {
                b"api\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"object\0".as_ptr() as *const ::core::ffi::c_char
            };
        safe_strcpy(
            &raw mut (*engine_0).engine_type as *mut ::core::ffi::c_char,
            r#type,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        (*engine_0).enabled = r#true != 0;
        (*engine_0).threshold = legacy_threshold;
        (*engine_0).interval_seconds = legacy_interval;
        (*ctx).engine_count = 1 as ::core::ffi::c_int;
    }
    let mut minimum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut maximum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < (*ctx).engine_count {
        let mut interval: ::core::ffi::c_int =
            if (*ctx).engines[i_1 as usize].interval_seconds > 0 as ::core::ffi::c_int {
                (*ctx).engines[i_1 as usize].interval_seconds
            } else {
                DEFAULT_DETECTION_INTERVAL
            };
        if minimum == 0 as ::core::ffi::c_int || interval < minimum {
            minimum = interval;
        }
        if interval > maximum {
            maximum = interval;
        }
        i_1 += 1;
    }
    if minimum > 0 as ::core::ffi::c_int {
        (*ctx).detection_interval = minimum;
    }
    (*ctx).recording_grace_interval = if maximum > 0 as ::core::ffi::c_int {
        maximum
    } else {
        (*ctx).detection_interval
    };
    (*ctx).has_onvif_engine =
        runtime_has_engine_type(ctx, b"onvif\0".as_ptr() as *const ::core::ffi::c_char);
    if (*ctx).engine_count == 1 as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut (*ctx).model_path as *mut ::core::ffi::c_char,
            &raw mut (*(&raw mut (*ctx).engines as *mut stream_detection_engine_t).offset(0isize))
                .model_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        (*ctx).detection_threshold = (*ctx).engines[0usize].threshold;
    }
}
unsafe extern "C" fn extract_onvif_base_url(
    mut stream_url: *const ::core::ffi::c_char,
    mut onvif_port: ::core::ffi::c_int,
    mut onvif_url: *mut ::core::ffi::c_char,
    mut onvif_url_size: size_t,
) {
    if stream_url.is_null() || onvif_url.is_null() || onvif_url_size == 0 as size_t {
        return;
    }
    *onvif_url.offset(0isize) = '\0' as ::core::ffi::c_char;
    url_build_onvif_service_url(
        stream_url,
        onvif_port,
        ::core::ptr::null::<::core::ffi::c_char>(),
        onvif_url,
        onvif_url_size,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_unified_detection_system() -> ::core::ffi::c_int {
    if system_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unified detection system already initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut contexts_mutex);
    memset(
        &raw mut detection_contexts as *mut *mut unified_detection_ctx_t
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[*mut unified_detection_ctx_t; 1024]>(),
    );
    let mut memory_limit: size_t = calculate_packet_buffer_pool_size();
    if init_packet_buffer_pool(memory_limit) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize packet buffer pool\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut contexts_mutex);
        return -1 as ::core::ffi::c_int;
    }
    system_initialized = r#true != 0;
    pthread_mutex_unlock(&raw mut contexts_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unified detection system initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_unified_detection_system() {
    if !system_initialized {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Shutting down unified detection system\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut already_stopped_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut threads_to_stop: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pthread_mutex_lock(&raw mut contexts_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_UNIFIED_DETECTION_THREADS {
        if !detection_contexts[i as usize].is_null() {
            let mut ctx: *mut unified_detection_ctx_t = detection_contexts[i as usize];
            let mut current_state: unified_detection_state_t =
                unified_detection_state_t(::core::intrinsics::atomic_load::<
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(&raw mut (*ctx).state)
                    as ::core::ffi::c_uint);
            if current_state.0 == unified_detection_state_t::UDT_STATE_STOPPED.0 {
                already_stopped_count += 1;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Unified detection thread %s already stopped (state=%d)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    current_state.0,
                );
            } else {
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx).running,
                    0 as ::core::ffi::c_int,
                );
                if current_state.0 != unified_detection_state_t::UDT_STATE_STOPPING.0 {
                    ::core::intrinsics::atomic_store::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(
                        &raw mut (*ctx).state,
                        unified_detection_state_t::UDT_STATE_STOPPING.0 as ::core::ffi::c_int,
                    );
                }
                threads_to_stop += 1;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Signaled unified detection thread %s to stop (was state=%d)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    current_state.0,
                );
            }
        }
        i += 1;
    }
    pthread_mutex_unlock(&raw mut contexts_mutex);
    if already_stopped_count > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Found %d unified detection threads already stopped\0".as_ptr()
                as *const ::core::ffi::c_char,
            already_stopped_count,
        );
    }
    let mut max_wait_iterations: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut wait_iter: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while wait_iter < max_wait_iterations {
        let mut all_stopped: bool = r#true != 0;
        pthread_mutex_lock(&raw mut contexts_mutex);
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < MAX_UNIFIED_DETECTION_THREADS {
            if !detection_contexts[i_0 as usize].is_null() {
                let mut state: unified_detection_state_t =
                    unified_detection_state_t(::core::intrinsics::atomic_load::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(
                        &raw mut (**(&raw mut detection_contexts
                            as *mut *mut unified_detection_ctx_t)
                            .offset(i_0 as isize))
                        .state,
                    ) as ::core::ffi::c_uint);
                if state.0 != unified_detection_state_t::UDT_STATE_STOPPED.0 {
                    all_stopped = r#false != 0;
                    break;
                }
            }
            i_0 += 1;
        }
        pthread_mutex_unlock(&raw mut contexts_mutex);
        if all_stopped {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"All unified detection threads have stopped\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            break;
        } else {
            usleep(100000 as ::core::ffi::c_int as __useconds_t);
            if wait_iter == max_wait_iterations - 1 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Timeout waiting for unified detection threads to stop, proceeding with cleanup\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            wait_iter += 1;
        }
    }
    pthread_mutex_lock(&raw mut contexts_mutex);
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < MAX_UNIFIED_DETECTION_THREADS {
        if !detection_contexts[i_1 as usize].is_null() {
            let mut ctx_0: *mut unified_detection_ctx_t = detection_contexts[i_1 as usize];
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cleaning up unified detection context for %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx_0).stream_name as *mut ::core::ffi::c_char,
            );
            if (*ctx_0).has_onvif_engine {
                stop_onvif_detection_thread(ctx_0);
            }
            stop_detection_stream_thread(ctx_0);
            if !(*ctx_0).packet_buffer.is_null() {
                destroy_packet_buffer((*ctx_0).packet_buffer);
                (*ctx_0).packet_buffer = ::core::ptr::null_mut::<packet_buffer_t>();
            }
            if !(*ctx_0).mp4_writer.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"MP4 writer still active during shutdown cleanup for %s - closing without database update\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*ctx_0).stream_name as *mut ::core::ffi::c_char,
                );
                mp4_writer_close((*ctx_0).mp4_writer);
                (*ctx_0).mp4_writer = ::core::ptr::null_mut::<mp4_writer_t>();
            }
            unload_runtime_models(ctx_0);
            if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx_0).state,
            ) == unified_detection_state_t::UDT_STATE_STOPPED.0 as ::core::ffi::c_int
            {
                pthread_mutex_destroy(&raw mut (*ctx_0).mutex);
                pthread_mutex_destroy(&raw mut (*ctx_0).detection_stream_result_mutex);
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Skipping mutex destroy for %s - thread may still be running\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*ctx_0).stream_name as *mut ::core::ffi::c_char,
                );
            }
            free(ctx_0 as *mut ::core::ffi::c_void);
            detection_contexts[i_1 as usize] = ::core::ptr::null_mut::<unified_detection_ctx_t>();
        }
        i_1 += 1;
    }
    cleanup_packet_buffer_pool();
    system_initialized = r#false != 0;
    pthread_mutex_unlock(&raw mut contexts_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unified detection system shutdown complete\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn state_to_string(
    mut state: unified_detection_state_t,
) -> *const ::core::ffi::c_char {
    match state {
        unified_detection_state_t::UDT_STATE_INITIALIZING => {
            return b"INITIALIZING\0".as_ptr() as *const ::core::ffi::c_char;
        }
        unified_detection_state_t::UDT_STATE_CONNECTING => {
            return b"CONNECTING\0".as_ptr() as *const ::core::ffi::c_char;
        }
        unified_detection_state_t::UDT_STATE_BUFFERING => {
            return b"BUFFERING\0".as_ptr() as *const ::core::ffi::c_char;
        }
        unified_detection_state_t::UDT_STATE_RECORDING => {
            return b"RECORDING\0".as_ptr() as *const ::core::ffi::c_char;
        }
        unified_detection_state_t::UDT_STATE_POST_BUFFER => {
            return b"POST_BUFFER\0".as_ptr() as *const ::core::ffi::c_char;
        }
        unified_detection_state_t::UDT_STATE_RECONNECTING => {
            return b"RECONNECTING\0".as_ptr() as *const ::core::ffi::c_char;
        }
        unified_detection_state_t::UDT_STATE_STOPPING => {
            return b"STOPPING\0".as_ptr() as *const ::core::ffi::c_char;
        }
        unified_detection_state_t::UDT_STATE_STOPPED => {
            return b"STOPPED\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"UNKNOWN\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
unsafe extern "C" fn find_context_by_name(
    mut stream_name: *const ::core::ffi::c_char,
) -> *mut unified_detection_ctx_t {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_UNIFIED_DETECTION_THREADS {
        if !detection_contexts[i as usize].is_null()
            && strcmp(
                &raw mut (**(&raw mut detection_contexts as *mut *mut unified_detection_ctx_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            return detection_contexts[i as usize];
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<unified_detection_ctx_t>();
}
unsafe extern "C" fn find_empty_slot() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_UNIFIED_DETECTION_THREADS {
        if detection_contexts[i as usize].is_null() {
            return i;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn onvif_detection_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut ctx: *mut unified_detection_ctx_t = arg as *mut unified_detection_ctx_t;
    let mut safe_onvif_url: [::core::ffi::c_char; 512] = [0; 512];
    if url_redact_for_logging(
        &raw mut (*ctx).onvif_url_cached as *mut ::core::ffi::c_char,
        &raw mut safe_onvif_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut safe_onvif_url as *mut ::core::ffi::c_char,
            b"[invalid-url]\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] ONVIF detection thread started (url=%s)\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        &raw mut safe_onvif_url as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] ONVIF detection thread auth=%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        if (*ctx).onvif_username_cached[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    while ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).onvif_thread_running,
    ) != 0
    {
        let mut result: detection_result_t = detection_result_t {
            count: 0,
            detections: [detection_t {
                label: [0; 32],
                confidence: 0.,
                x: 0.,
                y: 0.,
                width: 0.,
                height: 0.,
                track_id: 0,
                zone_id: [0; 32],
            }; 20],
        };
        memset(
            &raw mut result as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<detection_result_t>(),
        );
        let mut poll_start: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_MONOTONIC, &raw mut poll_start);
        let mut ret: ::core::ffi::c_int = detect_motion_onvif(
            &raw mut (*ctx).onvif_url_cached as *mut ::core::ffi::c_char,
            &raw mut (*ctx).onvif_username_cached as *mut ::core::ffi::c_char,
            &raw mut (*ctx).onvif_password_cached as *mut ::core::ffi::c_char,
            &raw mut result,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).onvif_thread_running,
        ) == 0
        {
            break;
        }
        if ret == 0 as ::core::ffi::c_int && result.count > 0 as ::core::ffi::c_int {
            let mut seen: ::core::ffi::c_longlong =
                time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_longlong;
            if ::core::intrinsics::atomic_xchg::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).onvif_motion_detected,
                1 as ::core::ffi::c_int,
            ) == 0 as ::core::ffi::c_int
            {
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx).onvif_motion_onset,
                    seen,
                );
            }
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).onvif_motion_timestamp,
                seen,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] ONVIF thread: %d motion event(s) detected\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                result.count,
            );
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 20 as ::core::ffi::c_int
                && ::core::intrinsics::atomic_load::<
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(&raw mut (*ctx).onvif_thread_running)
                    != 0
            {
                av_usleep(100000 as ::core::ffi::c_int as ::core::ffi::c_uint);
                i += 1;
            }
        } else {
            let mut last_ts: ::core::ffi::c_longlong = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*ctx).onvif_motion_timestamp
            );
            if last_ts == 0 as ::core::ffi::c_longlong
                || time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_longlong - last_ts
                    >= ONVIF_MOTION_HOLD_SECS as ::core::ffi::c_longlong
            {
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx).onvif_motion_detected,
                    0 as ::core::ffi::c_int,
                );
            }
            if ret == ONVIF_DETECT_RETRY {
                let mut remaining_ms: ::core::ffi::c_int = ONVIF_PULL_DROP_RETRY_DELAY_MS;
                while remaining_ms > 0 as ::core::ffi::c_int
                    && ::core::intrinsics::atomic_load::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*ctx).onvif_thread_running)
                        != 0
                {
                    let mut sleep_ms: ::core::ffi::c_int =
                        if remaining_ms < 100 as ::core::ffi::c_int {
                            remaining_ms
                        } else {
                            100 as ::core::ffi::c_int
                        };
                    av_usleep((sleep_ms * 1000 as ::core::ffi::c_int) as ::core::ffi::c_uint);
                    remaining_ms -= sleep_ms;
                }
            } else if ret != 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] ONVIF thread: detect_motion_onvif error %d \xE2\x80\x93 backing off 5 s before retry\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    ret,
                );
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < 50 as ::core::ffi::c_int
                    && ::core::intrinsics::atomic_load::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*ctx).onvif_thread_running)
                        != 0
                {
                    av_usleep(100000 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    i_0 += 1;
                }
            } else {
                let mut poll_end: timespec = timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                clock_gettime(CLOCK_MONOTONIC, &raw mut poll_end);
                let mut elapsed_ms: ::core::ffi::c_long = (poll_end.tv_sec - poll_start.tv_sec)
                    * 1000 as ::core::ffi::c_long
                    + (poll_end.tv_nsec - poll_start.tv_nsec) / 1000000 as ::core::ffi::c_long;
                let mut remaining_ms_0: ::core::ffi::c_long =
                    ONVIF_MIN_POLL_INTERVAL_MS as ::core::ffi::c_long - elapsed_ms;
                let mut slept: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
                while slept < remaining_ms_0
                    && ::core::intrinsics::atomic_load::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*ctx).onvif_thread_running)
                        != 0
                {
                    av_usleep(100000 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    slept += 100 as ::core::ffi::c_long;
                }
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] ONVIF detection thread exiting\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
    return NULL;
}
unsafe extern "C" fn start_onvif_detection_thread(
    mut ctx: *mut unified_detection_ctx_t,
) -> ::core::ffi::c_int {
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).onvif_thread_running,
        1 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).onvif_motion_detected,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).onvif_motion_timestamp,
        0 as ::core::ffi::c_longlong,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).onvif_motion_onset,
        0 as ::core::ffi::c_longlong,
    );
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    pthread_attr_init(&raw mut attr);
    pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed::PTHREAD_CREATE_JOINABLE.0 as ::core::ffi::c_int,
    );
    let mut ret: ::core::ffi::c_int = pthread_create(
        &raw mut (*ctx).onvif_thread,
        &raw mut attr,
        Some(
            onvif_detection_thread_func
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        ctx as *mut ::core::ffi::c_void,
    );
    pthread_attr_destroy(&raw mut attr);
    if ret != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to create ONVIF detection thread: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            strerror(ret),
        );
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).onvif_thread_running,
            0 as ::core::ffi::c_int,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] ONVIF detection thread created\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn stop_onvif_detection_thread(mut ctx: *mut unified_detection_ctx_t) {
    let mut expected: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let c2rust_result = ::core::intrinsics::atomic_cxchg::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(
        &raw mut (*ctx).onvif_thread_running,
        *&raw mut expected,
        0 as ::core::ffi::c_int,
    );
    *&raw mut expected = c2rust_result.0;
    if !c2rust_result.1 {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Requesting ONVIF detection thread to stop\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
    pthread_join(
        (*ctx).onvif_thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] ONVIF detection thread joined\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn detection_stream_backoff(
    mut ctx: *mut unified_detection_ctx_t,
    mut delay_ms: *mut ::core::ffi::c_int,
) {
    let mut remaining: ::core::ffi::c_int = *delay_ms;
    while remaining > 0 as ::core::ffi::c_int
        && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).detection_stream_thread_running,
        ) != 0
    {
        let mut slice: ::core::ffi::c_int = if remaining > 100 as ::core::ffi::c_int {
            100 as ::core::ffi::c_int
        } else {
            remaining
        };
        av_usleep((slice as ::core::ffi::c_uint).wrapping_mul(1000 as ::core::ffi::c_uint));
        remaining -= slice;
    }
    *delay_ms *= 2 as ::core::ffi::c_int;
    if *delay_ms > MAX_RECONNECT_DELAY_MS {
        *delay_ms = MAX_RECONNECT_DELAY_MS;
    }
}
unsafe extern "C" fn detection_stream_interrupt_cb(
    mut opaque: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut unified_detection_ctx_t = opaque as *mut unified_detection_ctx_t;
    if ctx.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).detection_stream_thread_running,
    ) == 0
        || ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).running,
        ) == 0
        || is_shutdown_initiated() as ::core::ffi::c_int != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    return ((*ctx).detection_io_deadline_us > 0 as int64_t
        && av_gettime_relative() >= (*ctx).detection_io_deadline_us)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn detection_stream_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut ctx: *mut unified_detection_ctx_t = arg as *mut unified_detection_ctx_t;
    let mut safe_detection_url: [::core::ffi::c_char; 512] = [0; 512];
    if url_redact_for_logging(
        &raw mut (*ctx).detection_stream_url as *mut ::core::ffi::c_char,
        &raw mut safe_detection_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut safe_detection_url as *mut ::core::ffi::c_char,
            b"[invalid-url]\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Detection stream thread started (url=%s)\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        &raw mut safe_detection_url as *mut ::core::ffi::c_char,
    );
    let mut reconnect_delay_ms: ::core::ffi::c_int = BASE_RECONNECT_DELAY_MS;
    while ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).detection_stream_thread_running,
    ) != 0
    {
        let mut fmt_ctx: *mut AVFormatContext = avformat_alloc_context();
        if fmt_ctx.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Detection stream thread: avformat_alloc_context failed\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            );
            detection_stream_backoff(ctx, &raw mut reconnect_delay_ms);
        } else {
            (*fmt_ctx).interrupt_callback.callback = Some(
                detection_stream_interrupt_cb
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            )
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
            (*fmt_ctx).interrupt_callback.opaque = ctx as *mut ::core::ffi::c_void;
            let mut opts: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
            av_dict_set(
                &raw mut opts,
                b"rtsp_transport\0".as_ptr() as *const ::core::ffi::c_char,
                b"tcp\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            av_dict_set(
                &raw mut opts,
                b"timeout\0".as_ptr() as *const ::core::ffi::c_char,
                b"5000000\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            av_dict_set(
                &raw mut opts,
                b"rw_timeout\0".as_ptr() as *const ::core::ffi::c_char,
                b"5000000\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            av_dict_set(
                &raw mut opts,
                b"analyzeduration\0".as_ptr() as *const ::core::ffi::c_char,
                b"2000000\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            av_dict_set(
                &raw mut opts,
                b"probesize\0".as_ptr() as *const ::core::ffi::c_char,
                b"2000000\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            av_dict_set(
                &raw mut opts,
                b"protocol_whitelist\0".as_ptr() as *const ::core::ffi::c_char,
                b"udp,rtp,rtsp,rtsps,tcp,tls,https,http\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            (*ctx).detection_io_deadline_us =
                (av_gettime_relative() as ::core::ffi::c_longlong + OPEN_TIMEOUT_US) as int64_t;
            let mut ret: ::core::ffi::c_int = avformat_open_input(
                &raw mut fmt_ctx,
                &raw mut (*ctx).detection_stream_url as *mut ::core::ffi::c_char,
                ::core::ptr::null::<AVInputFormat>(),
                &raw mut opts,
            );
            av_dict_free(&raw mut opts);
            if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).detection_stream_thread_running,
            ) == 0
            {
                if !fmt_ctx.is_null() {
                    avformat_close_input(&raw mut fmt_ctx);
                }
                break;
            } else if ret < 0 as ::core::ffi::c_int {
                if !fmt_ctx.is_null() {
                    avformat_close_input(&raw mut fmt_ctx);
                }
                let mut errbuf: [::core::ffi::c_char; 128] = [0; 128];
                av_strerror(
                    ret,
                    &raw mut errbuf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] Detection stream thread: cannot open %s: %s \xE2\x80\x94 retry in %d ms\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    &raw mut safe_detection_url as *mut ::core::ffi::c_char,
                    &raw mut errbuf as *mut ::core::ffi::c_char,
                    reconnect_delay_ms,
                );
                detection_stream_backoff(ctx, &raw mut reconnect_delay_ms);
            } else {
                (*ctx).detection_io_deadline_us = av_gettime_relative() + READ_TIMEOUT_US;
                ret = avformat_find_stream_info(
                    fmt_ctx,
                    ::core::ptr::null_mut::<*mut AVDictionary>(),
                );
                if ret < 0 as ::core::ffi::c_int
                    || detection_stream_interrupt_cb(ctx as *mut ::core::ffi::c_void) != 0
                {
                    log_stream_info_failure(
                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                        b"Detection stream\0".as_ptr() as *const ::core::ffi::c_char,
                        fmt_ctx,
                        ret,
                        (*ctx).detection_io_deadline_us,
                        ::core::intrinsics::atomic_load::<
                            _,
                            { ::core::intrinsics::AtomicOrdering::SeqCst },
                        >(&raw mut (*ctx).detection_stream_thread_running)
                            == 0
                            || ::core::intrinsics::atomic_load::<
                                _,
                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                            >(&raw mut (*ctx).running)
                                == 0
                            || is_shutdown_initiated() as ::core::ffi::c_int != 0,
                    );
                    avformat_close_input(&raw mut fmt_ctx);
                    detection_stream_backoff(ctx, &raw mut reconnect_delay_ms);
                } else {
                    let mut video_idx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                    while i < (*fmt_ctx).nb_streams {
                        if (*(**(*fmt_ctx).streams.offset(i as isize)).codecpar)
                            .codec_type
                            .0
                            == AVMediaType::AVMEDIA_TYPE_VIDEO.0
                        {
                            video_idx = i as ::core::ffi::c_int;
                            break;
                        } else {
                            i = i.wrapping_add(1);
                        }
                    }
                    if video_idx < 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"[%s] Detection stream thread: no video stream in %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                            &raw mut safe_detection_url as *mut ::core::ffi::c_char,
                        );
                        avformat_close_input(&raw mut fmt_ctx);
                        detection_stream_backoff(ctx, &raw mut reconnect_delay_ms);
                    } else {
                        let mut decoder: *const AVCodec = avcodec_find_decoder(
                            (*(**(*fmt_ctx).streams.offset(video_idx as isize)).codecpar).codec_id,
                        );
                        let mut dec_ctx: *mut AVCodecContext = if !decoder.is_null() {
                            avcodec_alloc_context3(decoder)
                        } else {
                            ::core::ptr::null_mut::<AVCodecContext>()
                        };
                        if dec_ctx.is_null() {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"[%s] Detection stream thread: no decoder available\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                            );
                            avformat_close_input(&raw mut fmt_ctx);
                            detection_stream_backoff(ctx, &raw mut reconnect_delay_ms);
                        } else {
                            avcodec_parameters_to_context(
                                dec_ctx,
                                (**(*fmt_ctx).streams.offset(video_idx as isize)).codecpar,
                            );
                            if avcodec_open2(
                                dec_ctx,
                                decoder,
                                ::core::ptr::null_mut::<*mut AVDictionary>(),
                            ) < 0 as ::core::ffi::c_int
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"[%s] Detection stream thread: failed to open decoder\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                                );
                                avcodec_free_context(&raw mut dec_ctx);
                                avformat_close_input(&raw mut fmt_ctx);
                                detection_stream_backoff(ctx, &raw mut reconnect_delay_ms);
                            } else {
                                let mut last_check: time_t = 0 as time_t;
                                let mut pkt: *mut AVPacket = av_packet_alloc();
                                let mut frame: *mut AVFrame = av_frame_alloc();
                                let mut stream_ok: bool = !pkt.is_null() && !frame.is_null();
                                let mut codec_desc: *const AVCodecDescriptor =
                                    avcodec_descriptor_get((*dec_ctx).codec_id);
                                let mut is_intra_only: bool = !codec_desc.is_null()
                                    && (*codec_desc).props & AV_CODEC_PROP_INTRA_ONLY
                                        != 0 as ::core::ffi::c_int;
                                if stream_ok {
                                    ::core::intrinsics::atomic_store::<
                                        _,
                                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                                    >(
                                        &raw mut (*ctx).detection_stream_connected,
                                        1 as ::core::ffi::c_int,
                                    );
                                    reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_INFO,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"[%s] Detection stream thread connected\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                                    );
                                } else {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_WARN,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"[%s] Detection stream thread: failed to allocate packet/frame\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                                    );
                                }
                                while ::core::intrinsics::atomic_load::<
                                    _,
                                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                                >(
                                    &raw mut (*ctx).detection_stream_thread_running
                                ) != 0
                                    && stream_ok as ::core::ffi::c_int != 0
                                {
                                    (*ctx).detection_io_deadline_us =
                                        av_gettime_relative() + READ_TIMEOUT_US;
                                    let mut rd: ::core::ffi::c_int = av_read_frame(fmt_ctx, pkt);
                                    let mut timed_out: bool =
                                        av_gettime_relative() >= (*ctx).detection_io_deadline_us;
                                    (*ctx).detection_io_deadline_us = 0 as int64_t;
                                    if rd < 0 as ::core::ffi::c_int
                                        || timed_out as ::core::ffi::c_int != 0
                                    {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_WARN,
                                            LOG_COMPONENT,
                                            _log_stream_name,
                                            b"[%s] Detection stream thread: %s, reconnecting\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                                            if timed_out as ::core::ffi::c_int != 0 {
                                                b"I/O deadline exceeded\0".as_ptr()
                                                    as *const ::core::ffi::c_char
                                            } else {
                                                b"read error\0".as_ptr()
                                                    as *const ::core::ffi::c_char
                                            },
                                        );
                                        stream_ok = r#false != 0;
                                        break;
                                    } else if (*pkt).stream_index != video_idx {
                                        av_packet_unref(pkt);
                                    } else {
                                        let mut now: time_t =
                                            time(::core::ptr::null_mut::<time_t>());
                                        let mut interval_elapsed: bool =
                                            now - last_check >= (*ctx).detection_interval as time_t;
                                        if is_intra_only as ::core::ffi::c_int != 0
                                            && !interval_elapsed
                                        {
                                            av_packet_unref(pkt);
                                        } else if avcodec_send_packet(dec_ctx, pkt)
                                            < 0 as ::core::ffi::c_int
                                        {
                                            av_packet_unref(pkt);
                                        } else {
                                            av_packet_unref(pkt);
                                            while avcodec_receive_frame(dec_ctx, frame)
                                                == 0 as ::core::ffi::c_int
                                            {
                                                now = time(::core::ptr::null_mut::<time_t>());
                                                if now - last_check
                                                    >= (*ctx).detection_interval as time_t
                                                {
                                                    last_check = now;
                                                    let mut result: detection_result_t =
                                                        detection_result_t {
                                                            count: 0,
                                                            detections: [detection_t {
                                                                label: [0; 32],
                                                                confidence: 0.,
                                                                x: 0.,
                                                                y: 0.,
                                                                width: 0.,
                                                                height: 0.,
                                                                track_id: 0,
                                                                zone_id: [0; 32],
                                                            };
                                                                20],
                                                        };
                                                    let mut hit: bool = detect_on_decoded_frame(
                                                        ctx,
                                                        frame,
                                                        now,
                                                        &raw mut result,
                                                    );
                                                    if hit {
                                                        pthread_mutex_lock(
                                                            &raw mut (*ctx)
                                                                .detection_stream_result_mutex,
                                                        );
                                                        (*ctx).detection_stream_pending = result;
                                                        (*ctx).detection_stream_pending_ts = now;
                                                        pthread_mutex_unlock(
                                                            &raw mut (*ctx)
                                                                .detection_stream_result_mutex,
                                                        );
                                                        ::core::intrinsics::atomic_store::<
                                                            _,
                                                            {
                                                                ::core::intrinsics::AtomicOrdering::SeqCst
                                                            },
                                                        >(
                                                            &raw mut (*ctx).detection_stream_result,
                                                            1 as ::core::ffi::c_int,
                                                        );
                                                    }
                                                }
                                                av_frame_unref(frame);
                                            }
                                        }
                                    }
                                }
                                ::core::intrinsics::atomic_store::<
                                    _,
                                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                                >(
                                    &raw mut (*ctx).detection_stream_connected,
                                    0 as ::core::ffi::c_int,
                                );
                                ::core::intrinsics::atomic_store::<
                                    _,
                                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                                >(
                                    &raw mut (*ctx).detection_stream_result,
                                    0 as ::core::ffi::c_int,
                                );
                                (*ctx).detection_io_deadline_us = (av_gettime_relative()
                                    as ::core::ffi::c_longlong
                                    + OPEN_TIMEOUT_US)
                                    as int64_t;
                                if !pkt.is_null() {
                                    av_packet_free(&raw mut pkt);
                                }
                                if !frame.is_null() {
                                    av_frame_free(&raw mut frame);
                                }
                                avcodec_free_context(&raw mut dec_ctx);
                                avformat_close_input(&raw mut fmt_ctx);
                                (*ctx).detection_io_deadline_us = 0 as int64_t;
                                if ::core::intrinsics::atomic_load::<
                                    _,
                                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                                >(
                                    &raw mut (*ctx).detection_stream_thread_running
                                ) != 0
                                    && !stream_ok
                                {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_INFO,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"[%s] Detection stream thread: reconnecting in %d ms\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                                        reconnect_delay_ms,
                                    );
                                    detection_stream_backoff(ctx, &raw mut reconnect_delay_ms);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Detection stream thread exiting\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
    return NULL;
}
unsafe extern "C" fn start_detection_stream_thread(
    mut ctx: *mut unified_detection_ctx_t,
) -> ::core::ffi::c_int {
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).detection_stream_thread_running,
        1 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).detection_stream_connected,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).detection_stream_result,
        0 as ::core::ffi::c_int,
    );
    memset(
        &raw mut (*ctx).detection_stream_pending as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    (*ctx).detection_stream_pending_ts = 0 as time_t;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    pthread_attr_init(&raw mut attr);
    pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed::PTHREAD_CREATE_JOINABLE.0 as ::core::ffi::c_int,
    );
    let mut ret: ::core::ffi::c_int = pthread_create(
        &raw mut (*ctx).detection_stream_thread,
        &raw mut attr,
        Some(
            detection_stream_thread_func
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        ctx as *mut ::core::ffi::c_void,
    );
    pthread_attr_destroy(&raw mut attr);
    if ret != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to create detection stream thread: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            strerror(ret),
        );
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).detection_stream_thread_running,
            0 as ::core::ffi::c_int,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Detection stream thread created\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn stop_detection_stream_thread(mut ctx: *mut unified_detection_ctx_t) {
    let mut expected: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let c2rust_result = ::core::intrinsics::atomic_cxchg::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(
        &raw mut (*ctx).detection_stream_thread_running,
        *&raw mut expected,
        0 as ::core::ffi::c_int,
    );
    *&raw mut expected = c2rust_result.0;
    if !c2rust_result.1 {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Requesting detection stream thread to stop\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
    pthread_join(
        (*ctx).detection_stream_thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Detection stream thread joined\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn detection_should_record_audio(
    mut config: *const stream_config_t,
    mut global_cfg: *const config_t,
) -> bool {
    return (*config).record_audio as ::core::ffi::c_int != 0
        && !(!global_cfg.is_null() && (*global_cfg).audio_disabled as ::core::ffi::c_int != 0);
}
unsafe extern "C" fn resolve_detection_rtsp_url(
    mut ctx: *mut unified_detection_ctx_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut config: *const stream_config_t,
) {
    if !go2rtc_stream_get_rtsp_url(
        stream_name,
        &raw mut (*ctx).rtsp_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) {
        if url_apply_credentials(
            &raw const (*config).url as *const ::core::ffi::c_char,
            if (*config).onvif_username[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*config).onvif_username as *const ::core::ffi::c_char
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            },
            if (*config).onvif_password[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*config).onvif_password as *const ::core::ffi::c_char
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            },
            &raw mut (*ctx).rtsp_url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*ctx).rtsp_url as *mut ::core::ffi::c_char,
                &raw const (*config).url as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
    } else if !(*ctx).record_audio {
        safe_strcat(
            &raw mut (*ctx).rtsp_url as *mut ::core::ffi::c_char,
            b"?video\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_unified_detection_thread(
    mut stream_name: *const ::core::ffi::c_char,
    mut model_path: *const ::core::ffi::c_char,
    mut threshold: ::core::ffi::c_float,
    mut pre_buffer_seconds: ::core::ffi::c_int,
    mut post_buffer_seconds: ::core::ffi::c_int,
    mut annotation_only: bool,
) -> ::core::ffi::c_int {
    if stream_name.is_null() || model_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for start_unified_detection_thread\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !system_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unified detection system not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut contexts_mutex);
    let mut existing: *mut unified_detection_ctx_t = find_context_by_name(stream_name);
    if !existing.is_null()
        && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*existing).running,
        ) != 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unified detection already running for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        pthread_mutex_unlock(&raw mut contexts_mutex);
        return 0 as ::core::ffi::c_int;
    }
    let mut slot: ::core::ffi::c_int = find_empty_slot();
    if slot < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No available slots for unified detection thread\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut contexts_mutex);
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
        pthread_mutex_unlock(&raw mut contexts_mutex);
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
    };
    if get_stream_config(stream, &raw mut config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get config for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        pthread_mutex_unlock(&raw mut contexts_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut ctx: *mut unified_detection_ctx_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<unified_detection_ctx_t>(),
    ) as *mut unified_detection_ctx_t;
    if ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate unified detection context\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut contexts_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut global_cfg: *mut config_t = get_streaming_config();
    safe_strcpy(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        stream_name,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*ctx).camera_uuid as *mut ::core::ffi::c_char,
        &raw mut config.camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*ctx).model_path as *mut ::core::ffi::c_char,
        model_path,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    (*ctx).detection_threshold = threshold;
    (*ctx).pre_buffer_seconds = if pre_buffer_seconds > 0 as ::core::ffi::c_int {
        pre_buffer_seconds
    } else {
        10 as ::core::ffi::c_int
    };
    (*ctx).post_buffer_seconds = if post_buffer_seconds > 0 as ::core::ffi::c_int {
        post_buffer_seconds
    } else {
        5 as ::core::ffi::c_int
    };
    (*ctx).segment_duration =
        if !global_cfg.is_null() && (*global_cfg).mp4_segment_duration > 0 as ::core::ffi::c_int {
            (*global_cfg).mp4_segment_duration
        } else {
            30 as ::core::ffi::c_int
        };
    (*ctx).detection_interval = if config.detection_interval > 0 as ::core::ffi::c_int {
        config.detection_interval
    } else {
        DEFAULT_DETECTION_INTERVAL
    };
    load_runtime_engine_config(ctx, model_path, threshold, (*ctx).detection_interval);
    (*ctx).record_audio = detection_should_record_audio(&raw mut config, global_cfg);
    (*ctx).audio_voice_enhancement = config.audio_voice_enhancement;
    (*ctx).annotation_only = annotation_only;
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).external_motion_trigger,
        0 as ::core::ffi::c_int,
    );
    (*ctx).stream_connect_time = time(::core::ptr::null_mut::<time_t>());
    (*ctx).first_video_pts = AV_NOPTS_VALUE;
    (*ctx).first_video_pts_set = r#false != 0;
    (*ctx).stream_is_live = r#false != 0;
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).last_detection_check_time,
        time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_longlong,
    );
    if annotation_only {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Detection will annotate continuous recordings instead of writing separate detection clips; clips are only created while continuous recording is stopped\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
    }
    resolve_detection_rtsp_url(ctx, stream_name, &raw mut config);
    if runtime_has_engine_type(ctx, b"motion\0".as_ptr() as *const ::core::ffi::c_char) {
        configure_motion_detection(
            stream_name,
            DEFAULT_MOTION_SENSITIVITY,
            DEFAULT_MOTION_MIN_AREA_RATIO,
            DEFAULT_MOTION_COOLDOWN_SECS,
        );
        set_motion_detection_enabled(stream_name, r#true != 0);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Built-in motion detection enabled (sensitivity=%.2f, confidence threshold=%.0f%%)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_name,
            DEFAULT_MOTION_SENSITIVITY as ::core::ffi::c_double,
            (threshold * 100.0f32) as ::core::ffi::c_double,
        );
    }
    if pthread_mutex_init(
        &raw mut (*ctx).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize mutex for unified detection context\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(ctx as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut contexts_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if pthread_mutex_init(
        &raw mut (*ctx).detection_stream_result_mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize detection stream result mutex for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        pthread_mutex_destroy(&raw mut (*ctx).mutex);
        free(ctx as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut contexts_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut buffer_window: ::core::ffi::c_int = (*ctx).pre_buffer_seconds;
    if buffer_window < MIN_BUFFER_SECONDS {
        buffer_window = MIN_BUFFER_SECONDS;
    }
    if buffer_window > MAX_BUFFER_SECONDS {
        buffer_window = MAX_BUFFER_SECONDS;
    }
    if (*ctx).has_onvif_engine {
        buffer_window += PACKET_BUFFER_TRIGGER_HEADROOM_SECONDS;
    }
    (*ctx).packet_buffer = create_packet_buffer(
        stream_name,
        buffer_window,
        buffer_mode_t::BUFFER_MODE_MEMORY,
    );
    if (*ctx).packet_buffer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create pre-detection buffer for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        pthread_mutex_destroy(&raw mut (*ctx).detection_stream_result_mutex);
        pthread_mutex_destroy(&raw mut (*ctx).mutex);
        free(ctx as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut contexts_mutex);
        return -1 as ::core::ffi::c_int;
    }
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).running,
        1 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).state,
        unified_detection_state_t::UDT_STATE_INITIALIZING.0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).last_packet_time,
        time(::core::ptr::null_mut::<time_t>()),
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).consecutive_failures,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).onvif_thread_running,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).onvif_motion_detected,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).onvif_motion_timestamp,
        0 as ::core::ffi::c_longlong,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).onvif_motion_onset,
        0 as ::core::ffi::c_longlong,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).detection_stream_thread_running,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).detection_stream_connected,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).detection_stream_result,
        0 as ::core::ffi::c_int,
    );
    memset(
        &raw mut (*ctx).detection_stream_pending as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    (*ctx).detection_stream_pending_ts = 0 as time_t;
    if (*ctx).has_onvif_engine {
        extract_onvif_base_url(
            &raw mut config.url as *mut ::core::ffi::c_char,
            config.onvif_port,
            &raw mut (*ctx).onvif_url_cached as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        );
        safe_strcpy(
            &raw mut (*ctx).onvif_username_cached as *mut ::core::ffi::c_char,
            &raw mut config.onvif_username as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*ctx).onvif_password_cached as *mut ::core::ffi::c_char,
            &raw mut config.onvif_password as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        if (*ctx).onvif_url_cached[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Cannot start ONVIF detection thread: could not derive an ONVIF URL from the configured stream\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
        } else if start_onvif_detection_thread(ctx) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] ONVIF detection thread could not be started; ONVIF-triggered recording is disabled for this stream\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
        }
    }
    if config.detection_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut (*ctx).detection_stream_url as *mut ::core::ffi::c_char,
            &raw mut config.detection_url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        if start_detection_stream_thread(ctx) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Detection stream thread could not be started; falling back to keyframe-based detection on the main stream\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
        } else {
            let mut safe_detection_url: [::core::ffi::c_char; 512] = [0; 512];
            if url_redact_for_logging(
                &raw mut (*ctx).detection_stream_url as *mut ::core::ffi::c_char,
                &raw mut safe_detection_url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            ) != 0 as ::core::ffi::c_int
            {
                safe_strcpy(
                    &raw mut safe_detection_url as *mut ::core::ffi::c_char,
                    b"[invalid-url]\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    0 as size_t,
                );
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Detection stream thread started for url=%s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                &raw mut safe_detection_url as *mut ::core::ffi::c_char,
            );
        }
    }
    (*ctx).slot_idx = slot;
    detection_contexts[slot as usize] = ctx;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    pthread_attr_init(&raw mut attr);
    pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
    );
    let mut result: ::core::ffi::c_int = pthread_create(
        &raw mut (*ctx).thread,
        &raw mut attr,
        Some(
            unified_detection_thread_func
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        ctx as *mut ::core::ffi::c_void,
    );
    pthread_attr_destroy(&raw mut attr);
    if result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create unified detection thread for %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            strerror(result),
        );
        stop_onvif_detection_thread(ctx);
        stop_detection_stream_thread(ctx);
        destroy_packet_buffer((*ctx).packet_buffer);
        pthread_mutex_destroy(&raw mut (*ctx).detection_stream_result_mutex);
        pthread_mutex_destroy(&raw mut (*ctx).mutex);
        free(ctx as *mut ::core::ffi::c_void);
        detection_contexts[slot as usize] = ::core::ptr::null_mut::<unified_detection_ctx_t>();
        pthread_mutex_unlock(&raw mut contexts_mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut contexts_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Started unified detection thread for stream %s (model=%s, threshold=%.2f, interval=%d, pre-buffer=%ds, post-buffer=%ds, segment=%ds)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        &raw mut (*ctx).model_path as *mut ::core::ffi::c_char,
        (*ctx).detection_threshold as ::core::ffi::c_double,
        (*ctx).detection_interval,
        (*ctx).pre_buffer_seconds,
        (*ctx).post_buffer_seconds,
        (*ctx).segment_duration,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_unified_detection_thread(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut contexts_mutex);
    let mut ctx: *mut unified_detection_ctx_t = find_context_by_name(stream_name);
    if ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"No unified detection thread found for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        pthread_mutex_unlock(&raw mut contexts_mutex);
        return -1 as ::core::ffi::c_int;
    }
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).running,
        0 as ::core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).state,
        unified_detection_state_t::UDT_STATE_STOPPING.0 as ::core::ffi::c_int,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Signaled unified detection thread for %s to stop\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    pthread_mutex_unlock(&raw mut contexts_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_unified_detection_running(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if stream_name.is_null() {
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut contexts_mutex);
    let mut ctx: *mut unified_detection_ctx_t = find_context_by_name(stream_name);
    let mut running: bool = !ctx.is_null()
        && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).running,
        ) != 0;
    pthread_mutex_unlock(&raw mut contexts_mutex);
    return running;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_unified_detection_recording_id(
    mut stream_name: *const ::core::ffi::c_char,
) -> uint64_t {
    if stream_name.is_null() {
        return 0 as uint64_t;
    }
    pthread_mutex_lock(&raw mut contexts_mutex);
    let mut ctx: *mut unified_detection_ctx_t = find_context_by_name(stream_name);
    let mut recording_id: uint64_t = if !ctx.is_null() {
        ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).current_recording_id,
        )
    } else {
        0 as uint64_t
    };
    pthread_mutex_unlock(&raw mut contexts_mutex);
    return recording_id;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_unified_detection_state(
    mut stream_name: *const ::core::ffi::c_char,
) -> unified_detection_state_t {
    if stream_name.is_null() {
        return unified_detection_state_t::UDT_STATE_STOPPED;
    }
    pthread_mutex_lock(&raw mut contexts_mutex);
    let mut ctx: *mut unified_detection_ctx_t = find_context_by_name(stream_name);
    let mut state: unified_detection_state_t = unified_detection_state_t(
        (if !ctx.is_null() {
            ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).state,
            )
        } else {
            unified_detection_state_t::UDT_STATE_STOPPED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    pthread_mutex_unlock(&raw mut contexts_mutex);
    return state;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_unified_detection_effective_status(
    mut stream_name: *const ::core::ffi::c_char,
) -> stream_status_t {
    if stream_name.is_null() {
        return stream_status_t::STREAM_STATUS_STOPPED;
    }
    pthread_mutex_lock(&raw mut contexts_mutex);
    let mut ctx: *mut unified_detection_ctx_t = find_context_by_name(stream_name);
    if ctx.is_null() {
        pthread_mutex_unlock(&raw mut contexts_mutex);
        return stream_status_t::STREAM_STATUS_STOPPED;
    }
    let mut udt_state: unified_detection_state_t =
        unified_detection_state_t(::core::intrinsics::atomic_load::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut (*ctx).state) as ::core::ffi::c_uint);
    let mut reconnect_attempt: ::core::ffi::c_int = (*ctx).reconnect_attempt;
    pthread_mutex_unlock(&raw mut contexts_mutex);
    match udt_state {
        unified_detection_state_t::UDT_STATE_INITIALIZING => {
            return stream_status_t::STREAM_STATUS_STARTING;
        }
        unified_detection_state_t::UDT_STATE_CONNECTING => {
            return stream_status_t(
                (if reconnect_attempt > 0 as ::core::ffi::c_int {
                    stream_status_t::STREAM_STATUS_RECONNECTING.0 as ::core::ffi::c_int
                } else {
                    stream_status_t::STREAM_STATUS_STARTING.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            );
        }
        unified_detection_state_t::UDT_STATE_BUFFERING
        | unified_detection_state_t::UDT_STATE_RECORDING
        | unified_detection_state_t::UDT_STATE_POST_BUFFER => {
            return stream_status_t::STREAM_STATUS_RUNNING;
        }
        unified_detection_state_t::UDT_STATE_RECONNECTING => {
            return stream_status_t::STREAM_STATUS_RECONNECTING;
        }
        unified_detection_state_t::UDT_STATE_STOPPING => {
            return stream_status_t::STREAM_STATUS_STOPPING;
        }
        unified_detection_state_t::UDT_STATE_STOPPED | _ => {
            return stream_status_t::STREAM_STATUS_STOPPED;
        }
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_unified_detection_reconnect_attempts(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut contexts_mutex);
    let mut ctx: *mut unified_detection_ctx_t = find_context_by_name(stream_name);
    let mut attempts: ::core::ffi::c_int = if !ctx.is_null() {
        (*ctx).reconnect_attempt
    } else {
        0 as ::core::ffi::c_int
    };
    pthread_mutex_unlock(&raw mut contexts_mutex);
    return attempts;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_unified_detection_stats(
    mut stream_name: *const ::core::ffi::c_char,
    mut packets_processed: *mut uint64_t,
    mut detections: *mut uint64_t,
    mut recordings: *mut uint64_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut contexts_mutex);
    let mut ctx: *mut unified_detection_ctx_t = find_context_by_name(stream_name);
    if ctx.is_null() {
        pthread_mutex_unlock(&raw mut contexts_mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*ctx).mutex);
    if !packets_processed.is_null() {
        *packets_processed = (*ctx).total_packets_processed;
    }
    if !detections.is_null() {
        *detections = (*ctx).total_detections;
    }
    if !recordings.is_null() {
        *recordings = (*ctx).total_recordings;
    }
    pthread_mutex_unlock(&raw mut (*ctx).mutex);
    pthread_mutex_unlock(&raw mut contexts_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unified_detection_notify_motion(
    mut stream_name: *const ::core::ffi::c_char,
    mut motion_active: bool,
) {
    if stream_name.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut contexts_mutex);
    let mut ctx: *mut unified_detection_ctx_t = find_context_by_name(stream_name);
    if !ctx.is_null() {
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).external_motion_trigger,
            if motion_active as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            },
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] external_motion_trigger set to %d via unified_detection_notify_motion\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            if motion_active as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            },
        );
    }
    pthread_mutex_unlock(&raw mut contexts_mutex);
}
unsafe extern "C" fn connect_to_stream(
    mut ctx: *mut unified_detection_ctx_t,
) -> ::core::ffi::c_int {
    if ctx.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut safe_rtsp_url: [::core::ffi::c_char; 512] = [0; 512];
    if url_redact_for_logging(
        &raw mut (*ctx).rtsp_url as *mut ::core::ffi::c_char,
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
        b"[%s] Connecting to stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        &raw mut safe_rtsp_url as *mut ::core::ffi::c_char,
    );
    (*ctx).input_ctx = avformat_alloc_context();
    if (*ctx).input_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to allocate format context\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    (*(*ctx).input_ctx).interrupt_callback.callback = Some(
        ffmpeg_interrupt_callback
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
    (*(*ctx).input_ctx).interrupt_callback.opaque = ctx as *mut ::core::ffi::c_void;
    let mut opts: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
    av_dict_set(
        &raw mut opts,
        b"rtsp_transport\0".as_ptr() as *const ::core::ffi::c_char,
        b"tcp\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut opts,
        b"timeout\0".as_ptr() as *const ::core::ffi::c_char,
        b"5000000\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut opts,
        b"rw_timeout\0".as_ptr() as *const ::core::ffi::c_char,
        b"5000000\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut opts,
        b"analyzeduration\0".as_ptr() as *const ::core::ffi::c_char,
        b"1000000\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    av_dict_set(
        &raw mut opts,
        b"probesize\0".as_ptr() as *const ::core::ffi::c_char,
        b"1000000\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    (*ctx).input_io_deadline_us =
        (av_gettime_relative() as ::core::ffi::c_longlong + OPEN_TIMEOUT_US) as int64_t;
    let mut ret: ::core::ffi::c_int = avformat_open_input(
        &raw mut (*ctx).input_ctx,
        &raw mut (*ctx).rtsp_url as *mut ::core::ffi::c_char,
        ::core::ptr::null::<AVInputFormat>(),
        &raw mut opts,
    );
    av_dict_free(&raw mut opts);
    if ret < 0 as ::core::ffi::c_int {
        let mut err_buf: [::core::ffi::c_char; 64] = [0; 64];
        av_strerror(
            ret,
            &raw mut err_buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to open input: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            &raw mut err_buf as *mut ::core::ffi::c_char,
        );
        avformat_free_context((*ctx).input_ctx);
        (*ctx).input_ctx = ::core::ptr::null_mut::<AVFormatContext>();
        return -1 as ::core::ffi::c_int;
    }
    (*ctx).input_io_deadline_us = av_gettime_relative() + READ_TIMEOUT_US;
    ret = avformat_find_stream_info(
        (*ctx).input_ctx,
        ::core::ptr::null_mut::<*mut AVDictionary>(),
    );
    if ret < 0 as ::core::ffi::c_int
        || ffmpeg_interrupt_callback(ctx as *mut ::core::ffi::c_void) != 0
    {
        log_stream_info_failure(
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            b"Main stream\0".as_ptr() as *const ::core::ffi::c_char,
            (*ctx).input_ctx,
            ret,
            (*ctx).input_io_deadline_us,
            ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).running,
            ) == 0
                || is_shutdown_initiated() as ::core::ffi::c_int != 0,
        );
        avformat_close_input(&raw mut (*ctx).input_ctx);
        return -1 as ::core::ffi::c_int;
    }
    (*ctx).input_io_deadline_us = 0 as int64_t;
    (*ctx).video_stream_idx = -1 as ::core::ffi::c_int;
    (*ctx).audio_stream_idx = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < (*(*ctx).input_ctx).nb_streams {
        let mut codecpar: *const AVCodecParameters =
            (**(*(*ctx).input_ctx).streams.offset(i as isize)).codecpar;
        if (*codecpar).codec_type.0 == AVMediaType::AVMEDIA_TYPE_VIDEO.0
            && (*ctx).video_stream_idx < 0 as ::core::ffi::c_int
        {
            (*ctx).video_stream_idx = i as ::core::ffi::c_int;
        } else if (*codecpar).codec_type.0 == AVMediaType::AVMEDIA_TYPE_AUDIO.0
            && (*ctx).audio_stream_idx < 0 as ::core::ffi::c_int
        {
            (*ctx).audio_stream_idx = i as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    if (*ctx).video_stream_idx < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] No video stream found\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        avformat_close_input(&raw mut (*ctx).input_ctx);
        return -1 as ::core::ffi::c_int;
    }
    let mut video_stream: *mut AVStream = *(*(*ctx).input_ctx)
        .streams
        .offset((*ctx).video_stream_idx as isize);
    let mut decoder: *const AVCodec = avcodec_find_decoder((*(*video_stream).codecpar).codec_id);
    if decoder.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to find decoder\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        avformat_close_input(&raw mut (*ctx).input_ctx);
        return -1 as ::core::ffi::c_int;
    }
    (*ctx).decoder_ctx = avcodec_alloc_context3(decoder);
    if (*ctx).decoder_ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to allocate decoder context\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        avformat_close_input(&raw mut (*ctx).input_ctx);
        return -1 as ::core::ffi::c_int;
    }
    ret = avcodec_parameters_to_context((*ctx).decoder_ctx, (*video_stream).codecpar);
    if ret < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to copy codec parameters\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        avcodec_free_context(&raw mut (*ctx).decoder_ctx);
        avformat_close_input(&raw mut (*ctx).input_ctx);
        return -1 as ::core::ffi::c_int;
    }
    ret = avcodec_open2(
        (*ctx).decoder_ctx,
        decoder,
        ::core::ptr::null_mut::<*mut AVDictionary>(),
    );
    if ret < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to open decoder\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        avcodec_free_context(&raw mut (*ctx).decoder_ctx);
        avformat_close_input(&raw mut (*ctx).input_ctx);
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Connected successfully (video stream: %d, audio stream: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        (*ctx).video_stream_idx,
        (*ctx).audio_stream_idx,
    );
    let mut vs: *mut AVStream = *(*(*ctx).input_ctx)
        .streams
        .offset((*ctx).video_stream_idx as isize);
    let mut cp: *mut AVCodecParameters = (*vs).codecpar;
    let mut det_width: ::core::ffi::c_int = (*cp).width;
    let mut det_height: ::core::ffi::c_int = (*cp).height;
    let mut det_codec: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut fps_is_provisional: bool = false;
    let mut det_fps: ::core::ffi::c_int = stream_frame_rate_round(stream_frame_rate_select(
        (*vs).avg_frame_rate,
        (*vs).r_frame_rate,
        &raw mut fps_is_provisional,
    ));
    if det_fps == 0 as ::core::ffi::c_int {
        det_fps = STREAM_FRAME_RATE_DEFAULT;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] FPS probe: avg=%d/%d, nominal=%d/%d, time_base=%d/%d; selected=%d%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        (*vs).avg_frame_rate.num,
        (*vs).avg_frame_rate.den,
        (*vs).r_frame_rate.num,
        (*vs).r_frame_rate.den,
        (*vs).time_base.num,
        (*vs).time_base.den,
        det_fps,
        if fps_is_provisional as ::core::ffi::c_int != 0 {
            b" (provisional; measuring arrivals)\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut desc: *const AVCodecDescriptor = avcodec_descriptor_get((*cp).codec_id);
    if !desc.is_null() {
        det_codec = (*desc).name;
    }
    if det_width > 0 as ::core::ffi::c_int && det_height > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Detected video params: %dx%d @ %d fps%s, codec=%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            det_width,
            det_height,
            det_fps,
            if fps_is_provisional as ::core::ffi::c_int != 0 {
                b" (provisional)\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            if !det_codec.is_null() {
                det_codec
            } else {
                b"unknown\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        udt_update_stream_video_params(
            ctx,
            det_width,
            det_height,
            det_fps,
            det_codec,
            fps_is_provisional,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn udt_update_stream_video_params(
    mut ctx: *mut unified_detection_ctx_t,
    mut det_width: ::core::ffi::c_int,
    mut det_height: ::core::ffi::c_int,
    mut det_fps: ::core::ffi::c_int,
    mut det_codec: *const ::core::ffi::c_char,
    mut fps_is_provisional: bool,
) {
    if ctx.is_null() {
        return;
    }
    metrics_set_configured_fps(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        det_fps as ::core::ffi::c_double,
    );
    pthread_mutex_lock(&raw mut (*ctx).mutex);
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*ctx).fps_is_provisional).store(fps_is_provisional, ::core::sync::atomic::Ordering::SeqCst);
    if fps_is_provisional {
        (*ctx).fps_measurement = stream_frame_rate_measurement_t {
            started: false,
            warmup_until_us: 0,
            window_start_us: 0,
            packets: 0,
            fps: 0,
        };
    }
    pthread_mutex_unlock(&raw mut (*ctx).mutex);
    let mut prev_codec: [::core::ffi::c_char; 16] = [
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
    ];
    let mut prev: stream_config_t = stream_config_t {
        camera_uuid: [
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
        ],
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
    };
    if get_stream_config_by_name(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        &raw mut prev,
    ) == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut prev_codec as *mut ::core::ffi::c_char,
            &raw mut prev.codec as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    }
    update_stream_video_params(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        det_width,
        det_height,
        det_fps,
        det_codec,
    );
    if !det_codec.is_null()
        && *det_codec.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && strcasecmp(&raw mut prev_codec as *mut ::core::ffi::c_char, det_codec)
            != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Source codec transitioned %s -> %s; re-registering with go2rtc\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            if prev_codec[0usize] as ::core::ffi::c_int != 0 {
                &raw mut prev_codec as *mut ::core::ffi::c_char
            } else {
                b"(unknown)\0".as_ptr() as *const ::core::ffi::c_char
            },
            det_codec,
        );
        go2rtc_integration_reload_stream_config(
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn disconnect_from_stream(mut ctx: *mut unified_detection_ctx_t) {
    if ctx.is_null() {
        return;
    }
    if !(*ctx).decoder_ctx.is_null() {
        avcodec_free_context(&raw mut (*ctx).decoder_ctx);
        (*ctx).decoder_ctx = ::core::ptr::null_mut::<AVCodecContext>();
    }
    if !(*ctx).input_ctx.is_null() {
        (*ctx).input_io_deadline_us =
            (av_gettime_relative() as ::core::ffi::c_longlong + OPEN_TIMEOUT_US) as int64_t;
        avformat_close_input(&raw mut (*ctx).input_ctx);
        (*ctx).input_ctx = ::core::ptr::null_mut::<AVFormatContext>();
    }
    (*ctx).input_io_deadline_us = 0 as int64_t;
    (*ctx).video_stream_idx = -1 as ::core::ffi::c_int;
    (*ctx).audio_stream_idx = -1 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Disconnected from stream\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn unified_detection_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut ctx: *mut unified_detection_ctx_t = arg as *mut unified_detection_ctx_t;
    if ctx.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"NULL context passed to unified detection thread\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    log_set_thread_context(
        b"Detection\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Unified detection thread started\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    av_log_set_level(AV_LOG_QUIET);
    let mut state: unified_detection_state_t = unified_detection_state_t::UDT_STATE_INITIALIZING;
    let mut reconnect_delay_ms: ::core::ffi::c_int = BASE_RECONNECT_DELAY_MS;
    let mut saw_real_packets: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pkt: *mut AVPacket = av_packet_alloc();
    let mut frame: *mut AVFrame = av_frame_alloc();
    if pkt.is_null() || frame.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to allocate packet/frame\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        if !pkt.is_null() {
            av_packet_free(&raw mut pkt);
        }
        if !frame.is_null() {
            av_frame_free(&raw mut frame);
        }
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).state,
            unified_detection_state_t::UDT_STATE_STOPPED.0 as ::core::ffi::c_int,
        );
        return NULL;
    }
    while ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).running,
    ) != 0
        && !is_shutdown_initiated()
    {
        state = unified_detection_state_t(::core::intrinsics::atomic_load::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut (*ctx).state) as ::core::ffi::c_uint);
        let mut read_ret: ::core::ffi::c_int = 0;
        let mut read_timed_out: bool = false;
        match state {
            unified_detection_state_t::UDT_STATE_INITIALIZING => {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] State: INITIALIZING\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
                state = unified_detection_state_t::UDT_STATE_CONNECTING;
            }
            unified_detection_state_t::UDT_STATE_CONNECTING => {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] State: CONNECTING (attempt %d)\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                    (*ctx).reconnect_attempt + 1 as ::core::ffi::c_int,
                );
                if connect_to_stream(ctx) == 0 as ::core::ffi::c_int {
                    state = unified_detection_state_t::UDT_STATE_BUFFERING;
                    (*ctx).reconnect_attempt = 0 as ::core::ffi::c_int;
                    ::core::intrinsics::atomic_store::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(
                        &raw mut (*ctx).last_packet_time,
                        time(::core::ptr::null_mut::<time_t>()),
                    );
                    saw_real_packets = 0 as ::core::ffi::c_int;
                    pthread_mutex_lock(&raw mut (*ctx).mutex);
                    (*ctx).total_packets_processed = 0 as uint64_t;
                    (*ctx).total_detections = 0 as uint64_t;
                    pthread_mutex_unlock(&raw mut (*ctx).mutex);
                    ::core::intrinsics::atomic_store::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(
                        &raw mut (*ctx).last_detection_check_time,
                        time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_longlong,
                    );
                    (*ctx).stream_connect_time = time(::core::ptr::null_mut::<time_t>());
                    (*ctx).first_video_pts_set = r#false != 0;
                    (*ctx).first_video_pts = AV_NOPTS_VALUE;
                    (*ctx).stream_is_live = r#false != 0;
                } else {
                    (*ctx).reconnect_attempt += 1;
                    ::core::intrinsics::atomic_xadd::<
                        _,
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(
                        &raw mut (*ctx).consecutive_failures,
                        1 as ::core::ffi::c_int,
                    );
                    let mut remaining_ms: ::core::ffi::c_int = reconnect_delay_ms;
                    while remaining_ms > 0 as ::core::ffi::c_int
                        && ::core::intrinsics::atomic_load::<
                            _,
                            { ::core::intrinsics::AtomicOrdering::SeqCst },
                        >(&raw mut (*ctx).running)
                            != 0
                        && !is_shutdown_initiated()
                    {
                        let mut sleep_ms: ::core::ffi::c_int =
                            if remaining_ms > 500 as ::core::ffi::c_int {
                                500 as ::core::ffi::c_int
                            } else {
                                remaining_ms
                            };
                        usleep((sleep_ms * 1000 as ::core::ffi::c_int) as __useconds_t);
                        remaining_ms -= sleep_ms;
                    }
                    reconnect_delay_ms = reconnect_delay_ms * 2 as ::core::ffi::c_int;
                    if reconnect_delay_ms > MAX_RECONNECT_DELAY_MS {
                        reconnect_delay_ms = MAX_RECONNECT_DELAY_MS;
                    }
                }
            }
            unified_detection_state_t::UDT_STATE_BUFFERING
            | unified_detection_state_t::UDT_STATE_RECORDING
            | unified_detection_state_t::UDT_STATE_POST_BUFFER => {
                #[thread_local]
                static mut last_heartbeat: time_t = 0 as time_t;
                let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
                if now - last_heartbeat >= 30 as time_t {
                    last_heartbeat = now;
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"[%s] Heartbeat: state=%s, packets=%lu, detections=%lu, last_check=%lds ago\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        state_to_string(state),
                        (*ctx).total_packets_processed as ::core::ffi::c_ulong,
                        (*ctx).total_detections as ::core::ffi::c_ulong,
                        (now as ::core::ffi::c_longlong
                            - ::core::intrinsics::atomic_load::<
                                _,
                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                            >(&raw mut (*ctx).last_detection_check_time))
                            as ::core::ffi::c_long,
                    );
                }
                (*ctx).input_io_deadline_us = av_gettime_relative() + READ_TIMEOUT_US;
                read_ret = av_read_frame((*ctx).input_ctx, pkt);
                read_timed_out = av_gettime_relative() >= (*ctx).input_io_deadline_us;
                (*ctx).input_io_deadline_us = 0 as int64_t;
                if read_ret >= 0 as ::core::ffi::c_int && !read_timed_out {
                    if (*pkt).size > 0 as ::core::ffi::c_int {
                        ::core::intrinsics::atomic_store::<
                            _,
                            { ::core::intrinsics::AtomicOrdering::SeqCst },
                        >(
                            &raw mut (*ctx).last_packet_time,
                            time(::core::ptr::null_mut::<time_t>()),
                        );
                        if saw_real_packets == 0 {
                            saw_real_packets = 1 as ::core::ffi::c_int;
                            reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;
                            ::core::intrinsics::atomic_store::<
                                _,
                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                            >(
                                &raw mut (*ctx).consecutive_failures,
                                0 as ::core::ffi::c_int,
                            );
                        }
                    }
                    process_packet(ctx, pkt);
                    state = unified_detection_state_t(::core::intrinsics::atomic_load::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*ctx).state)
                        as ::core::ffi::c_uint);
                } else {
                    let mut now_0: time_t = time(::core::ptr::null_mut::<time_t>());
                    let mut last: time_t = ::core::intrinsics::atomic_load::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*ctx).last_packet_time);
                    if read_timed_out as ::core::ffi::c_int != 0
                        || now_0 - last > MAX_PACKET_TIMEOUT_SEC as time_t
                    {
                        if saw_real_packets == 0 {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"[%s] Packet timeout with no media received; backing off %d ms before reconnect\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                reconnect_delay_ms,
                            );
                            ::core::intrinsics::atomic_xadd::<
                                _,
                                _,
                                { ::core::intrinsics::AtomicOrdering::SeqCst },
                            >(
                                &raw mut (*ctx).consecutive_failures,
                                1 as ::core::ffi::c_int,
                            );
                            let mut remaining_ms_0: ::core::ffi::c_int = reconnect_delay_ms;
                            while remaining_ms_0 > 0 as ::core::ffi::c_int
                                && ::core::intrinsics::atomic_load::<
                                    _,
                                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                                >(&raw mut (*ctx).running)
                                    != 0
                                && !is_shutdown_initiated()
                            {
                                let mut sleep_ms_0: ::core::ffi::c_int =
                                    if remaining_ms_0 > 500 as ::core::ffi::c_int {
                                        500 as ::core::ffi::c_int
                                    } else {
                                        remaining_ms_0
                                    };
                                usleep((sleep_ms_0 * 1000 as ::core::ffi::c_int) as __useconds_t);
                                remaining_ms_0 -= sleep_ms_0;
                            }
                            reconnect_delay_ms = reconnect_delay_ms * 2 as ::core::ffi::c_int;
                            if reconnect_delay_ms > MAX_RECONNECT_DELAY_MS {
                                reconnect_delay_ms = MAX_RECONNECT_DELAY_MS;
                            }
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"[%s] %s, reconnecting\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut stream_name as *mut ::core::ffi::c_char,
                                if read_timed_out as ::core::ffi::c_int != 0 {
                                    b"I/O deadline exceeded\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                } else {
                                    b"Packet timeout\0".as_ptr() as *const ::core::ffi::c_char
                                },
                            );
                        }
                        disconnect_from_stream(ctx);
                        state = unified_detection_state_t::UDT_STATE_RECONNECTING;
                    } else {
                        av_usleep(10000 as ::core::ffi::c_uint);
                    }
                }
                av_packet_unref(pkt);
            }
            unified_detection_state_t::UDT_STATE_RECONNECTING => {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] State: RECONNECTING\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
                if !(*ctx).mp4_writer.is_null() {
                    udt_stop_recording(ctx);
                }
                packet_buffer_clear((*ctx).packet_buffer);
                state = unified_detection_state_t::UDT_STATE_CONNECTING;
            }
            unified_detection_state_t::UDT_STATE_STOPPING => {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] State: STOPPING\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
                if !(*ctx).mp4_writer.is_null() {
                    udt_stop_recording(ctx);
                }
                disconnect_from_stream(ctx);
                state = unified_detection_state_t::UDT_STATE_STOPPED;
            }
            unified_detection_state_t::UDT_STATE_STOPPED => {
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx).running,
                    0 as ::core::ffi::c_int,
                );
            }
            _ => {}
        }
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).state,
            state.0 as ::core::ffi::c_int,
        );
    }
    if !(*ctx).mp4_writer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Closing active recording before thread exit\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        udt_stop_recording(ctx);
    }
    if (*ctx).has_onvif_engine {
        stop_onvif_detection_thread(ctx);
    }
    stop_detection_stream_thread(ctx);
    disconnect_from_stream(ctx);
    go2rtc_snapshot_cleanup_thread();
    av_packet_free(&raw mut pkt);
    av_frame_free(&raw mut frame);
    unload_runtime_models(ctx);
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).state,
        unified_detection_state_t::UDT_STATE_STOPPED.0 as ::core::ffi::c_int,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Unified detection thread exiting\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    if !is_shutdown_initiated() {
        pthread_mutex_lock(&raw mut contexts_mutex);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < MAX_UNIFIED_DETECTION_THREADS {
            if detection_contexts[i as usize] == ctx {
                if !(*ctx).packet_buffer.is_null() {
                    destroy_packet_buffer((*ctx).packet_buffer);
                }
                pthread_mutex_destroy(&raw mut (*ctx).detection_stream_result_mutex);
                pthread_mutex_destroy(&raw mut (*ctx).mutex);
                free(ctx as *mut ::core::ffi::c_void);
                detection_contexts[i as usize] = ::core::ptr::null_mut::<unified_detection_ctx_t>();
                break;
            } else {
                i += 1;
            }
        }
        pthread_mutex_unlock(&raw mut contexts_mutex);
    }
    return NULL;
}
unsafe extern "C" fn flush_packet_callback(
    mut packet: *const AVPacket,
    mut user_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut flush_ctx: *mut flush_callback_ctx_t = user_data as *mut flush_callback_ctx_t;
    if flush_ctx.is_null() || (*flush_ctx).ctx.is_null() || packet.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut ctx: *mut unified_detection_ctx_t = (*flush_ctx).ctx;
    if !(*flush_ctx).found_keyframe {
        if (*packet).flags & AV_PKT_FLAG_KEY != 0
            && (*packet).stream_index == (*(*flush_ctx).ctx).video_stream_idx
        {
            (*flush_ctx).found_keyframe = r#true != 0;
        } else {
            return 0 as ::core::ffi::c_int;
        }
    }
    if !(*ctx).mp4_writer.is_null() && !(*ctx).input_ctx.is_null() {
        let mut input_stream: *const AVStream = ::core::ptr::null::<AVStream>();
        if (*packet).stream_index == (*ctx).video_stream_idx
            && (*ctx).video_stream_idx >= 0 as ::core::ffi::c_int
        {
            input_stream = *(*(*ctx).input_ctx)
                .streams
                .offset((*ctx).video_stream_idx as isize);
        } else if (*packet).stream_index == (*ctx).audio_stream_idx
            && (*ctx).audio_stream_idx >= 0 as ::core::ffi::c_int
        {
            input_stream = *(*(*ctx).input_ctx)
                .streams
                .offset((*ctx).audio_stream_idx as isize);
        }
        if !input_stream.is_null() {
            if !(*flush_ctx).writer_initialized && (*(*ctx).mp4_writer).is_initialized == 0 {
                if (*packet).stream_index == (*ctx).video_stream_idx
                    && (*packet).flags & AV_PKT_FLAG_KEY != 0
                {
                    let mut init_ret: ::core::ffi::c_int =
                        mp4_writer_initialize((*ctx).mp4_writer, packet, input_stream);
                    if init_ret < 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"[%s] Failed to initialize MP4 writer\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    (*flush_ctx).writer_initialized = r#true != 0;
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"[%s] MP4 writer initialized on first keyframe\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    );
                } else {
                    return 0 as ::core::ffi::c_int;
                }
            }
            let mut ret: ::core::ffi::c_int =
                mp4_writer_write_packet((*ctx).mp4_writer, packet, input_stream);
            if ret == 0 as ::core::ffi::c_int {
                (*flush_ctx).packets_written += 1;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn process_packet(
    mut ctx: *mut unified_detection_ctx_t,
    mut pkt: *mut AVPacket,
) -> ::core::ffi::c_int {
    if ctx.is_null() || pkt.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut current_state: unified_detection_state_t =
        unified_detection_state_t(::core::intrinsics::atomic_load::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut (*ctx).state) as ::core::ffi::c_uint);
    let mut is_video: bool = (*pkt).stream_index == (*ctx).video_stream_idx;
    let mut is_keyframe: bool =
        is_video as ::core::ffi::c_int != 0 && (*pkt).flags & AV_PKT_FLAG_KEY != 0;
    pthread_mutex_lock(&raw mut (*ctx).mutex);
    (*ctx).total_packets_processed = (*ctx).total_packets_processed.wrapping_add(1);
    let mut measured_fps: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if is_video as ::core::ffi::c_int != 0
        && (*pkt).size > 0 as ::core::ffi::c_int
        && ::core::sync::atomic::AtomicBool::from_ptr(&raw mut (*ctx).fps_is_provisional).load(::core::sync::atomic::Ordering::SeqCst) as ::core::ffi::c_int
            != 0
    {
        measured_fps =
            stream_frame_rate_observe(&raw mut (*ctx).fps_measurement, av_gettime_relative());
    }
    pthread_mutex_unlock(&raw mut (*ctx).mutex);
    if measured_fps > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Runtime FPS measurement: %d fps; replacing provisional rate\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            measured_fps,
        );
        let mut cp: *const AVCodecParameters = (**(*(*ctx).input_ctx)
            .streams
            .offset((*ctx).video_stream_idx as isize))
        .codecpar;
        let mut desc: *const AVCodecDescriptor = avcodec_descriptor_get((*cp).codec_id);
        udt_update_stream_video_params(
            ctx,
            (*cp).width,
            (*cp).height,
            measured_fps,
            if !desc.is_null() {
                (*desc).name
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            },
            r#false != 0,
        );
    }
    let mut should_buffer: bool = r#false != 0;
    if is_video as ::core::ffi::c_int != 0
        && !(*ctx).input_ctx.is_null()
        && (*ctx).video_stream_idx >= 0 as ::core::ffi::c_int
    {
        if !(*ctx).first_video_pts_set {
            if (*pkt).pts != AV_NOPTS_VALUE {
                (*ctx).first_video_pts = (*pkt).pts;
                (*ctx).first_video_pts_set = r#true != 0;
            }
            if (*ctx).stream_is_live as ::core::ffi::c_int != 0
                && is_keyframe as ::core::ffi::c_int != 0
            {
                should_buffer = r#true != 0;
            }
        } else if (*ctx).stream_is_live {
            should_buffer = r#true != 0;
        } else {
            let mut wall_elapsed: ::core::ffi::c_double = difftime(now, (*ctx).stream_connect_time);
            let mut replay_lag: ::core::ffi::c_double = 0.;
            if (*pkt).pts != AV_NOPTS_VALUE && (*ctx).first_video_pts != AV_NOPTS_VALUE {
                let mut tb: AVRational = (**(*(*ctx).input_ctx)
                    .streams
                    .offset((*ctx).video_stream_idx as isize))
                .time_base;
                let mut pts_elapsed: ::core::ffi::c_double =
                    ((*pkt).pts - (*ctx).first_video_pts) as ::core::ffi::c_double * av_q2d(tb);
                replay_lag = wall_elapsed - pts_elapsed;
            } else {
                let mut warmup: ::core::ffi::c_double =
                    ((*ctx).pre_buffer_seconds * 2 as ::core::ffi::c_int) as ::core::ffi::c_double;
                replay_lag = if wall_elapsed < warmup {
                    warmup - wall_elapsed
                } else {
                    0.0f64
                };
            }
            if replay_lag <= (*ctx).pre_buffer_seconds as ::core::ffi::c_double {
                if is_keyframe {
                    packet_buffer_clear((*ctx).packet_buffer);
                    (*ctx).stream_is_live = r#true != 0;
                    should_buffer = r#true != 0;
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"[%s] go2rtc replay ended, pre-buffer active (lag=%.1fs)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                        replay_lag,
                    );
                }
            } else if is_keyframe {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] go2rtc replay in progress (lag=%.1fs), skipping pre-buffer\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    replay_lag,
                );
            }
        }
    } else if !is_video && (*ctx).stream_is_live as ::core::ffi::c_int != 0 {
        should_buffer = r#true != 0;
    }
    if should_buffer {
        packet_buffer_add_packet((*ctx).packet_buffer, pkt, now);
    }
    metrics_record_frame_from_source(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        (*pkt).size,
        is_video,
        metrics_frame_source_t::METRICS_SOURCE_DETECTION,
    );
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).detection_stream_connected,
    ) != 0
    {
        let mut detected: bool = r#false != 0;
        if ::core::intrinsics::atomic_xchg::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).detection_stream_result,
            0 as ::core::ffi::c_int,
        ) != 0
        {
            let mut result: detection_result_t = detection_result_t {
                count: 0,
                detections: [detection_t {
                    label: [0; 32],
                    confidence: 0.,
                    x: 0.,
                    y: 0.,
                    width: 0.,
                    height: 0.,
                    track_id: 0,
                    zone_id: [0; 32],
                }; 20],
            };
            let mut result_ts: time_t = 0;
            pthread_mutex_lock(&raw mut (*ctx).detection_stream_result_mutex);
            result = (*ctx).detection_stream_pending;
            result_ts = (*ctx).detection_stream_pending_ts;
            pthread_mutex_unlock(&raw mut (*ctx).detection_stream_result_mutex);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Detection stream result: %d detection(s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                result.count,
            );
            report_detections(
                ctx,
                &raw mut result,
                if result_ts > 0 as time_t {
                    result_ts
                } else {
                    now
                },
            );
            detected = r#true != 0;
        }
        handle_recording_state(ctx, detected, now);
        current_state = unified_detection_state_t(::core::intrinsics::atomic_load::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut (*ctx).state)
            as ::core::ffi::c_uint);
    }
    if current_state.0 == unified_detection_state_t::UDT_STATE_RECORDING.0
        || current_state.0 == unified_detection_state_t::UDT_STATE_POST_BUFFER.0
    {
        if !(*ctx).mp4_writer.is_null() && !(*ctx).input_ctx.is_null() {
            let mut input_stream: *const AVStream = ::core::ptr::null::<AVStream>();
            if is_video as ::core::ffi::c_int != 0
                && (*ctx).video_stream_idx >= 0 as ::core::ffi::c_int
            {
                input_stream = *(*(*ctx).input_ctx)
                    .streams
                    .offset((*ctx).video_stream_idx as isize);
            } else if !is_video && (*ctx).audio_stream_idx >= 0 as ::core::ffi::c_int {
                input_stream = *(*(*ctx).input_ctx)
                    .streams
                    .offset((*ctx).audio_stream_idx as isize);
            }
            if !input_stream.is_null() {
                if (*(*ctx).mp4_writer).is_initialized == 0
                    && is_video as ::core::ffi::c_int != 0
                    && is_keyframe as ::core::ffi::c_int != 0
                {
                    let mut init_ret: ::core::ffi::c_int =
                        mp4_writer_initialize((*ctx).mp4_writer, pkt, input_stream);
                    if init_ret < 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"[%s] Failed to initialize MP4 writer during live recording\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                        );
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"[%s] MP4 writer initialized during live recording\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                        );
                    }
                }
                if (*(*ctx).mp4_writer).is_initialized != 0 {
                    mp4_writer_write_packet((*ctx).mp4_writer, pkt, input_stream);
                }
            }
        }
        if current_state.0 == unified_detection_state_t::UDT_STATE_POST_BUFFER.0 {
            if now
                >= ::core::intrinsics::atomic_load::<
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(&raw mut (*ctx).post_buffer_end_time) as time_t
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] Post-buffer complete, stopping recording\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                );
                udt_stop_recording(ctx);
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx).state,
                    unified_detection_state_t::UDT_STATE_BUFFERING.0 as ::core::ffi::c_int,
                );
            }
        }
    }
    if is_keyframe {
        let mut ext_trigger: ::core::ffi::c_int =
            ::core::intrinsics::atomic_xchg::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).external_motion_trigger,
                0 as ::core::ffi::c_int,
            );
        if ext_trigger == 1 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] External motion trigger (active) received\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).last_detection_time,
                now as ::core::ffi::c_longlong,
            );
            if current_state.0 == unified_detection_state_t::UDT_STATE_BUFFERING.0 {
                if should_annotate_continuous(ctx) {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"[%s] External trigger linked to active continuous recording\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"[%s] External trigger starting recording\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    );
                    if udt_start_recording(ctx) == 0 as ::core::ffi::c_int {
                        flush_prebuffer_and_correct_start(ctx, now, now);
                        ::core::intrinsics::atomic_store::<
                            _,
                            { ::core::intrinsics::AtomicOrdering::SeqCst },
                        >(
                            &raw mut (*ctx).state,
                            unified_detection_state_t::UDT_STATE_RECORDING.0 as ::core::ffi::c_int,
                        );
                    }
                }
            } else if current_state.0 == unified_detection_state_t::UDT_STATE_POST_BUFFER.0
                && ::core::intrinsics::atomic_load::<
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(&raw mut (*ctx).current_recording_id)
                    != 0 as uint64_t
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] External trigger during post-buffer, continuing recording\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx).state,
                    unified_detection_state_t::UDT_STATE_RECORDING.0 as ::core::ffi::c_int,
                );
            }
        } else if ext_trigger == 2 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] External motion trigger (ended) received\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            );
            if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).current_recording_id,
            ) != 0 as uint64_t
                && current_state.0 == unified_detection_state_t::UDT_STATE_RECORDING.0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] External trigger ending recording, entering post-buffer (%ds)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    (*ctx).post_buffer_seconds,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx).post_buffer_end_time,
                    (now + (*ctx).post_buffer_seconds as time_t) as ::core::ffi::c_longlong,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx).state,
                    unified_detection_state_t::UDT_STATE_POST_BUFFER.0 as ::core::ffi::c_int,
                );
            }
        }
        current_state = unified_detection_state_t(::core::intrinsics::atomic_load::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut (*ctx).state)
            as ::core::ffi::c_uint);
    }
    if is_keyframe as ::core::ffi::c_int != 0
        && current_state.0 != unified_detection_state_t::UDT_STATE_POST_BUFFER.0
        && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).detection_stream_connected,
        ) == 0
    {
        let mut time_since_last_check: time_t = now
            - ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).last_detection_check_time,
            ) as time_t;
        if time_since_last_check > 0 as time_t
            && ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).log_counter,
                1 as ::core::ffi::c_int,
            ) % 10 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Time since last detection check: %ld/%d seconds, model=%s, state=%d\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                time_since_last_check,
                (*ctx).detection_interval,
                &raw mut (*ctx).model_path as *mut ::core::ffi::c_char,
                current_state.0,
            );
        }
        let mut onvif_onset: ::core::ffi::c_longlong = ::core::intrinsics::atomic_load::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut (*ctx).onvif_motion_onset);
        if current_state.0 == unified_detection_state_t::UDT_STATE_BUFFERING.0
            && (*ctx).engine_count <= 1 as ::core::ffi::c_int
            && is_onvif_detection_model(&raw mut (*ctx).model_path as *mut ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
            && ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).onvif_motion_detected,
            ) != 0
            && onvif_onset != (*ctx).onvif_onset_handled
            && time_since_last_check < (*ctx).detection_interval as time_t
        {
            (*ctx).onvif_onset_handled = onvif_onset;
            let mut detected_0: bool = run_detection_on_frame(ctx, pkt, now);
            handle_recording_state(ctx, detected_0, now);
        } else if time_since_last_check >= (*ctx).detection_interval as time_t {
            (*ctx).onvif_onset_handled = onvif_onset;
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).last_detection_check_time,
                now as ::core::ffi::c_longlong,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Running detection (interval=%ds, elapsed=%lds, model=%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                (*ctx).detection_interval,
                time_since_last_check,
                &raw mut (*ctx).model_path as *mut ::core::ffi::c_char,
            );
            let mut detected_1: bool = run_detection_on_frame(ctx, pkt, now);
            handle_recording_state(ctx, detected_1, now);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn udt_start_recording(
    mut ctx: *mut unified_detection_ctx_t,
) -> ::core::ffi::c_int {
    if ctx.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    memset(
        &raw mut (*ctx).placement as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_placement_t>(),
    );
    if storage_placement_select(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        &raw mut (*ctx).placement,
    ) != 0 as ::core::ffi::c_int
        || (*ctx).placement.status.0 != storage_placement_status_t::STORAGE_PLACEMENT_READY.0
        || build_mp4_recording_directory_for_placement(
            get_streaming_config(),
            &raw mut (*ctx).placement,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            now,
            &raw mut (*ctx).output_dir as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Storage placement blocked detection recording (reason: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            &raw mut (*ctx).placement.reason as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut st: stat = stat {
        st_dev: 0 as __dev_t,
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
        &raw mut (*ctx).output_dir as *mut ::core::ffi::c_char,
        &raw mut st,
    ) == -1 as ::core::ffi::c_int
    {
        if mkdir_recursive(&raw mut (*ctx).output_dir as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Failed to create output directory: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                &raw mut (*ctx).output_dir as *mut ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
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
    let mut timestamp: [::core::ffi::c_char; 32] = [0; 32];
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if localtime_r(&raw mut now, &raw mut tm_buf).is_null()
        || strftime(
            &raw mut timestamp as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%Y%m%d_%H%M%S\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut tm_buf,
        ) == 0
        || lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if !strchr(
        &raw mut uuid as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    )
    .is_null()
        || !strchr(
            &raw mut uuid as *mut ::core::ffi::c_char,
            '\\' as ::core::ffi::c_int,
        )
        .is_null()
        || !strstr(
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"..\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut path_length: ::core::ffi::c_int = snprintf(
        &raw mut (*ctx).current_recording_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/detection_%s_%s.mp4\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).output_dir as *mut ::core::ffi::c_char,
        &raw mut timestamp as *mut ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    if path_length < 0 as ::core::ffi::c_int
        || path_length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut root_length: size_t =
        strlen(&raw mut (*ctx).placement.target_root as *mut ::core::ffi::c_char);
    if strncmp(
        &raw mut (*ctx).current_recording_path as *mut ::core::ffi::c_char,
        &raw mut (*ctx).placement.target_root as *mut ::core::ffi::c_char,
        root_length,
    ) != 0 as ::core::ffi::c_int
        || (*ctx).current_recording_path[root_length] as ::core::ffi::c_int
            != '/' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Placed path escaped storage target\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut (*ctx).placement.object_key as *mut ::core::ffi::c_char,
        (&raw mut (*ctx).current_recording_path as *mut ::core::ffi::c_char)
            .offset(root_length as isize)
            .offset(1 as ::core::ffi::c_int as isize),
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Starting detection recording: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        &raw mut (*ctx).current_recording_path as *mut ::core::ffi::c_char,
    );
    (*ctx).mp4_writer = mp4_writer_create(
        &raw mut (*ctx).current_recording_path as *mut ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
    if (*ctx).mp4_writer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to create MP4 writer\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut open_result: ::core::ffi::c_int = recording_file_open(
        &raw mut (*(*ctx).mp4_writer).reserved_output,
        &raw mut (*ctx).current_recording_path as *mut ::core::ffi::c_char,
    );
    if open_result < 0 as ::core::ffi::c_int {
        recording_io_report_failure(
            recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
            recording_io_operation_t::RECORDING_IO_OPERATION_OPEN,
            open_result,
        );
        mp4_writer_close((*ctx).mp4_writer);
        (*ctx).mp4_writer = ::core::ptr::null_mut::<mp4_writer_t>();
        (*ctx).current_recording_path[0usize] = '\0' as ::core::ffi::c_char;
        return -1 as ::core::ffi::c_int;
    }
    (*(*ctx).mp4_writer).placement = (*ctx).placement;
    safe_strcpy(
        &raw mut (*(*ctx).mp4_writer).camera_uuid as *mut ::core::ffi::c_char,
        &raw mut (*ctx).camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    (*(*ctx).mp4_writer).pre_buffer_seconds = (*ctx).pre_buffer_seconds;
    if (*ctx).record_audio as ::core::ffi::c_int != 0
        && (*ctx).audio_stream_idx >= 0 as ::core::ffi::c_int
    {
        mp4_writer_set_audio((*ctx).mp4_writer, 1 as ::core::ffi::c_int);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Audio recording enabled for detection recording\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        let mut ain: *mut AVStream = *(*(*ctx).input_ctx)
            .streams
            .offset((*ctx).audio_stream_idx as isize);
        let mut codec_name: *const ::core::ffi::c_char =
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
        let mut pending: *mut AVCodecParameters = ::core::ptr::null_mut::<AVCodecParameters>();
        if is_pcm_codec((*(*ain).codecpar).codec_id) {
            if transcode_pcm_to_aac(
                (*ain).codecpar,
                &raw mut (*ain).time_base,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                &raw mut pending,
            ) < 0 as ::core::ffi::c_int
                || pending.is_null()
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] Failed to prepare AAC codec params \xE2\x80\x94 disabling audio for this recording\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                );
                mp4_writer_set_audio((*ctx).mp4_writer, 0 as ::core::ffi::c_int);
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] PCM->AAC codec params prepared for MP4 header (eager init)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                );
                set_audio_voice_enhancement(
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    (*ctx).audio_voice_enhancement,
                );
            }
        } else if is_audio_codec_compatible_with_mp4(
            (*(*ain).codecpar).codec_id,
            &raw mut codec_name,
        ) {
            pending = avcodec_parameters_alloc();
            if pending.is_null()
                || avcodec_parameters_copy(pending, (*ain).codecpar) < 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] Failed to copy %s codec params \xE2\x80\x94 disabling audio for this recording\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    codec_name,
                );
                if !pending.is_null() {
                    avcodec_parameters_free(&raw mut pending);
                    pending = ::core::ptr::null_mut::<AVCodecParameters>();
                }
                mp4_writer_set_audio((*ctx).mp4_writer, 0 as ::core::ffi::c_int);
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] %s codec params prepared for MP4 header (eager init)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    codec_name,
                );
            }
        } else {
            is_audio_codec_compatible_with_mp4((*(*ain).codecpar).codec_id, &raw mut codec_name);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Audio codec %s is not MP4-compatible and not PCM \xE2\x80\x94 disabling audio for this recording\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                codec_name,
            );
            mp4_writer_set_audio((*ctx).mp4_writer, 0 as ::core::ffi::c_int);
        }
        if !pending.is_null() {
            (*(*ctx).mp4_writer).pending_audio_codecpar = pending;
            (*(*ctx).mp4_writer).pending_audio_time_base = (*ain).time_base;
        }
    } else {
        mp4_writer_set_audio((*ctx).mp4_writer, 0 as ::core::ffi::c_int);
        if (*ctx).record_audio as ::core::ffi::c_int != 0
            && (*ctx).audio_stream_idx < 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Audio recording requested but no audio stream found\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            );
        }
    }
    safe_strcpy(
        &raw mut (*(*ctx).mp4_writer).trigger_type as *mut ::core::ffi::c_char,
        b"detection\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    let mut stream_config: stream_config_t = stream_config_t {
        camera_uuid: [
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
        ],
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
    };
    if get_stream_config_by_name(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        &raw mut stream_config,
    ) == 0 as ::core::ffi::c_int
    {
        (*(*ctx).mp4_writer).schedule_restricted =
            if stream_config.detection_record_on_schedule as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
    }
    (*(*ctx).mp4_writer).creation_time = now;
    let mut metadata: recording_metadata_t = recording_metadata_t {
        id: 0 as uint64_t,
        stream_name: [0; 64],
        camera_uuid: [0; 37],
        file_path: [0; 512],
        storage_target_uuid: [0; 37],
        object_key: [0; 512],
        placement_reason: [0; 64],
        storage_policy_version: 0,
        start_time: 0,
        end_time: 0,
        size_bytes: 0,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        is_complete: false,
        playback_issue: [0; 32],
        trigger_type: [0; 16],
        protected: false,
        retention_override_days: 0,
        retention_tier: 0,
        disk_pressure_eligible: false,
        schedule_restricted: 0,
    };
    safe_strcpy(
        &raw mut metadata.file_path as *mut ::core::ffi::c_char,
        &raw mut (*ctx).current_recording_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut metadata.stream_name as *mut ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut metadata.camera_uuid as *mut ::core::ffi::c_char,
        &raw mut (*ctx).camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut metadata.storage_target_uuid as *mut ::core::ffi::c_char,
        &raw mut (*ctx).placement.target_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut metadata.object_key as *mut ::core::ffi::c_char,
        &raw mut (*ctx).placement.object_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut metadata.placement_reason as *mut ::core::ffi::c_char,
        &raw mut (*ctx).placement.reason as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    metadata.storage_policy_version = (*ctx).placement.policy_version;
    metadata.start_time = now;
    metadata.end_time = 0 as time_t;
    metadata.size_bytes = 0 as uint64_t;
    metadata.is_complete = r#false != 0;
    metadata.retention_tier = RETENTION_TIER_IMPORTANT;
    metadata.disk_pressure_eligible = r#true != 0;
    metadata.schedule_restricted = (*(*ctx).mp4_writer).schedule_restricted;
    safe_strcpy(
        &raw mut metadata.trigger_type as *mut ::core::ffi::c_char,
        b"detection\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    (*ctx).current_recording_id = add_recording_metadata(&raw mut metadata);
    if (*ctx).current_recording_id > 0 as uint64_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Added detection recording to database (ID: %lu) for file: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            (*ctx).current_recording_id as ::core::ffi::c_ulong,
            &raw mut (*ctx).current_recording_path as *mut ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to add detection recording to database\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
    }
    metrics_set_recording_active(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        r#true != 0,
    );
    pthread_mutex_lock(&raw mut (*ctx).mutex);
    (*ctx).total_recordings = (*ctx).total_recordings.wrapping_add(1);
    pthread_mutex_unlock(&raw mut (*ctx).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Detection recording started successfully\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn udt_stop_recording(
    mut ctx: *mut unified_detection_ctx_t,
) -> ::core::ffi::c_int {
    if ctx.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if (*ctx).mp4_writer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] No detection MP4 to stop\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Stopping detection recording: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        &raw mut (*ctx).current_recording_path as *mut ::core::ffi::c_char,
    );
    let mut end_time: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut start_time: time_t = (*(*ctx).mp4_writer).creation_time;
    let mut schedule_restricted: ::core::ffi::c_int = (*(*ctx).mp4_writer).schedule_restricted;
    let mut duration: ::core::ffi::c_int = (end_time - start_time) as ::core::ffi::c_int;
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
    let mut file_size: int64_t = 0 as int64_t;
    if stat(
        &raw mut (*ctx).current_recording_path as *mut ::core::ffi::c_char,
        &raw mut st,
    ) == 0 as ::core::ffi::c_int
    {
        file_size = st.st_size as int64_t;
    }
    metrics_set_recording_active(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        r#false != 0,
    );
    metrics_record_segment_complete(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        start_time,
        end_time,
        file_size as uint64_t,
    );
    mp4_writer_close((*ctx).mp4_writer);
    (*ctx).mp4_writer = ::core::ptr::null_mut::<mp4_writer_t>();
    if (*ctx).current_recording_id > 0 as uint64_t {
        if update_recording_metadata(
            (*ctx).current_recording_id,
            end_time,
            file_size as uint64_t,
            r#true != 0,
        ) == 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Recording updated in database (ID: %lu, duration: %ds, size: %ld bytes)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                (*ctx).current_recording_id as ::core::ffi::c_ulong,
                duration,
                file_size as ::core::ffi::c_long,
            );
            update_stream_storage_cache_add_recording(
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                file_size as uint64_t,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Failed to update recording in database (ID: %lu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                (*ctx).current_recording_id as ::core::ffi::c_ulong,
            );
        }
    } else if (*ctx).current_recording_path[0usize] as ::core::ffi::c_int
        != '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] No recording ID found, creating new database entry\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
        let mut metadata: recording_metadata_t = recording_metadata_t {
            id: 0 as uint64_t,
            stream_name: [0; 64],
            camera_uuid: [0; 37],
            file_path: [0; 512],
            storage_target_uuid: [0; 37],
            object_key: [0; 512],
            placement_reason: [0; 64],
            storage_policy_version: 0,
            start_time: 0,
            end_time: 0,
            size_bytes: 0,
            width: 0,
            height: 0,
            fps: 0,
            codec: [0; 16],
            is_complete: false,
            playback_issue: [0; 32],
            trigger_type: [0; 16],
            protected: false,
            retention_override_days: 0,
            retention_tier: 0,
            disk_pressure_eligible: false,
            schedule_restricted: 0,
        };
        safe_strcpy(
            &raw mut metadata.file_path as *mut ::core::ffi::c_char,
            &raw mut (*ctx).current_recording_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut metadata.stream_name as *mut ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut metadata.camera_uuid as *mut ::core::ffi::c_char,
            &raw mut (*ctx).camera_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut metadata.storage_target_uuid as *mut ::core::ffi::c_char,
            &raw mut (*ctx).placement.target_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut metadata.object_key as *mut ::core::ffi::c_char,
            &raw mut (*ctx).placement.object_key as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut metadata.placement_reason as *mut ::core::ffi::c_char,
            &raw mut (*ctx).placement.reason as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        metadata.storage_policy_version = (*ctx).placement.policy_version;
        metadata.start_time = start_time;
        metadata.end_time = end_time;
        metadata.size_bytes = file_size as uint64_t;
        metadata.is_complete = r#true != 0;
        metadata.schedule_restricted = schedule_restricted;
        safe_strcpy(
            &raw mut metadata.trigger_type as *mut ::core::ffi::c_char,
            b"detection\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        let mut recording_id: uint64_t = add_recording_metadata(&raw mut metadata);
        if recording_id > 0 as uint64_t {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Recording added to database with ID %lu (duration: %ds, size: %ld bytes)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                recording_id as ::core::ffi::c_ulong,
                duration,
                file_size as ::core::ffi::c_long,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Failed to add recording to database\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            );
        }
    }
    (*ctx).current_recording_path[0usize] = '\0' as ::core::ffi::c_char;
    (*ctx).current_recording_id = 0 as ::core::ffi::c_ulong;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Detection recording stopped (duration: %d seconds)\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        duration,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn flush_prebuffer_to_recording(
    mut ctx: *mut unified_detection_ctx_t,
) -> ::core::ffi::c_int {
    if ctx.is_null() || (*ctx).mp4_writer.is_null() || (*ctx).packet_buffer.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Flushing pre-buffer to recording\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut memory: size_t = 0 as size_t;
    let mut duration: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    packet_buffer_get_stats(
        (*ctx).packet_buffer,
        &raw mut count,
        &raw mut memory,
        &raw mut duration,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Pre-buffer contains %d packets (~%d seconds, %zu bytes)\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        count,
        duration,
        memory,
    );
    let mut flush_ctx: flush_callback_ctx_t = flush_callback_ctx_t {
        ctx: ctx,
        packets_written: 0 as ::core::ffi::c_int,
        found_keyframe: r#false != 0,
        writer_initialized: r#false != 0,
    };
    let mut flushed: ::core::ffi::c_int = packet_buffer_flush(
        (*ctx).packet_buffer,
        Some(
            flush_packet_callback
                as unsafe extern "C" fn(
                    *const AVPacket,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut flush_ctx as *mut ::core::ffi::c_void,
    );
    if flushed >= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Flushed %d packets to recording (%d written starting from keyframe)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            flushed,
            flush_ctx.packets_written,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to flush pre-buffer\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn detection_event_time(
    mut ctx: *mut unified_detection_ctx_t,
    mut now: time_t,
) -> time_t {
    if (*ctx).has_onvif_engine {
        let mut onset: time_t = ::core::intrinsics::atomic_load::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut (*ctx).onvif_motion_onset) as time_t;
        if onset > 0 as time_t
            && onset <= now
            && now - onset <= PACKET_BUFFER_TRIGGER_HEADROOM_SECONDS as time_t
        {
            return onset;
        }
    }
    return now;
}
unsafe extern "C" fn flush_prebuffer_and_correct_start(
    mut ctx: *mut unified_detection_ctx_t,
    mut now: time_t,
    mut event_time: time_t,
) {
    if (*ctx).packet_buffer.is_null() {
        return;
    }
    let mut cutoff: time_t = event_time - (*ctx).pre_buffer_seconds as time_t;
    let mut trimmed: ::core::ffi::c_int =
        packet_buffer_trim_before((*ctx).packet_buffer, cutoff, (*ctx).video_stream_idx);
    if trimmed > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Trimmed %d pre-buffer packets older than the pre-roll window\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            trimmed,
        );
    }
    let mut pre_dur: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pre_cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pre_mem: size_t = 0 as size_t;
    packet_buffer_get_stats(
        (*ctx).packet_buffer,
        &raw mut pre_cnt,
        &raw mut pre_mem,
        &raw mut pre_dur,
    );
    flush_prebuffer_to_recording(ctx);
    if !(*(*ctx).mp4_writer).start_time_corrected
        && pre_dur > 0 as ::core::ffi::c_int
        && (*ctx).current_recording_id > 0 as uint64_t
    {
        let mut max_pre: ::core::ffi::c_int =
            (*(*ctx).packet_buffer).buffer_seconds + 1 as ::core::ffi::c_int;
        let mut clamped_pre: ::core::ffi::c_int = if pre_dur > max_pre { max_pre } else { pre_dur };
        let mut corrected: time_t = now - clamped_pre as time_t;
        (*(*ctx).mp4_writer).creation_time = corrected;
        (*(*ctx).mp4_writer).start_time_corrected = r#true != 0;
        update_recording_start_time((*ctx).current_recording_id, corrected);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Corrected recording start_time by -%ds (pre-buffer, event %lds before trigger)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            clamped_pre,
            now - event_time,
        );
    }
}
unsafe extern "C" fn run_detection_on_frame(
    mut ctx: *mut unified_detection_ctx_t,
    mut pkt: *mut AVPacket,
    mut frame_timestamp: time_t,
) -> bool {
    if ctx.is_null() {
        return r#false != 0;
    }
    let mut now: time_t = frame_timestamp;
    if (*ctx).engine_count <= 1 as ::core::ffi::c_int
        && is_onvif_detection_model(&raw mut (*ctx).model_path as *mut ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
    {
        let mut triggered: bool =
            ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).onvif_motion_detected,
            ) != 0 as ::core::ffi::c_int;
        if triggered {
            pthread_mutex_lock(&raw mut (*ctx).mutex);
            (*ctx).total_detections = (*ctx).total_detections.wrapping_add(1);
            pthread_mutex_unlock(&raw mut (*ctx).mutex);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] ONVIF motion detected (async thread, ts=%lld)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx).onvif_motion_timestamp,
                ),
            );
        }
        return triggered;
    }
    if pkt.is_null() || (*ctx).decoder_ctx.is_null() || (*pkt).flags & AV_PKT_FLAG_KEY == 0 {
        return r#false != 0;
    }
    avcodec_flush_buffers((*ctx).decoder_ctx);
    let mut ret: ::core::ffi::c_int = avcodec_send_packet((*ctx).decoder_ctx, pkt);
    if ret < 0 as ::core::ffi::c_int {
        log_ffmpeg_error(
            ret,
            b"Failed to submit detection keyframe\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut frame: *mut AVFrame = av_frame_alloc();
    if frame.is_null() {
        return r#false != 0;
    }
    ret = avcodec_receive_frame((*ctx).decoder_ctx, frame);
    if ret == -11 as ::core::ffi::c_int {
        ret = avcodec_send_packet((*ctx).decoder_ctx, ::core::ptr::null::<AVPacket>());
        if ret >= 0 as ::core::ffi::c_int {
            ret = avcodec_receive_frame((*ctx).decoder_ctx, frame);
        }
    }
    if ret < 0 as ::core::ffi::c_int {
        log_ffmpeg_error(
            ret,
            b"Failed to decode detection keyframe\0".as_ptr() as *const ::core::ffi::c_char,
        );
        av_frame_free(&raw mut frame);
        return r#false != 0;
    }
    let mut result2: detection_result_t = detection_result_t {
        count: 0,
        detections: [detection_t {
            label: [0; 32],
            confidence: 0.,
            x: 0.,
            y: 0.,
            width: 0.,
            height: 0.,
            track_id: 0,
            zone_id: [0; 32],
        }; 20],
    };
    let mut hit: bool = detect_on_decoded_frame(ctx, frame, now, &raw mut result2);
    av_frame_free(&raw mut frame);
    if hit {
        report_detections(ctx, &raw mut result2, now);
    }
    return hit;
}
unsafe extern "C" fn frame_to_rgb24(mut frame: *const AVFrame) -> *mut uint8_t {
    return ffmpeg_frame_to_rgb24_padded(frame, ::core::ptr::null_mut::<size_t>());
}
unsafe extern "C" fn append_engine_result(
    mut ctx: *mut unified_detection_ctx_t,
    mut engine: *const stream_detection_engine_t,
    mut combined: *mut detection_result_t,
    mut candidate: *mut detection_result_t,
) {
    if (*candidate).count <= 0 as ::core::ffi::c_int {
        return;
    }
    filter_detections_by_zones(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        candidate,
    );
    filter_detections_by_stream_objects(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        candidate,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*candidate).count && (*combined).count < MAX_DETECTIONS {
        if (*candidate).detections[i as usize].confidence >= (*engine).threshold {
            let c2rust_fresh0 = (*combined).count;
            (*combined).count += 1;
            (*combined).detections[c2rust_fresh0 as usize] = (*candidate).detections[i as usize];
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Detection engine %s: %s (%.1f%%)\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                &raw const (*engine).engine_key as *const ::core::ffi::c_char,
                &raw mut (*(&raw mut (*candidate).detections as *mut detection_t)
                    .offset(i as isize))
                .label as *mut ::core::ffi::c_char,
                ((*candidate).detections[i as usize].confidence * 100.0f32)
                    as ::core::ffi::c_double,
            );
        }
        i += 1;
    }
}
unsafe extern "C" fn detect_with_runtime_engines(
    mut ctx: *mut unified_detection_ctx_t,
    mut frame: *mut AVFrame,
    mut now: time_t,
    mut combined: *mut detection_result_t,
) -> bool {
    let mut any_due_frame_engine: bool = r#false != 0;
    let mut onvif_triggered: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*ctx).engine_count {
        let mut engine: *mut stream_detection_engine_t =
            (&raw mut (*ctx).engines as *mut stream_detection_engine_t).offset(i as isize);
        if !(!(*engine).enabled
            || (*ctx).engine_last_run[i as usize] > 0 as time_t
                && now - (*ctx).engine_last_run[i as usize] < (*engine).interval_seconds as time_t)
        {
            if strcmp(
                &raw mut (*engine).engine_type as *mut ::core::ffi::c_char,
                b"motion\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut (*engine).engine_type as *mut ::core::ffi::c_char,
                    b"object\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                any_due_frame_engine = r#true != 0;
            } else if strcmp(
                &raw mut (*engine).engine_type as *mut ::core::ffi::c_char,
                b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*ctx).engine_last_run[i as usize] = now;
                if ::core::intrinsics::atomic_load::<
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(&raw mut (*ctx).onvif_motion_detected)
                    != 0
                {
                    onvif_triggered = r#true != 0;
                }
            } else if strcmp(
                &raw mut (*engine).engine_type as *mut ::core::ffi::c_char,
                b"api\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                if !(*ctx).engine_load_failed[i as usize] {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"[%s] Additional API engine %s is isolated from local fan-out to prevent duplicate persistence\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                        &raw mut (*engine).engine_key as *mut ::core::ffi::c_char,
                    );
                    (*ctx).engine_load_failed[i as usize] = r#true != 0;
                }
                (*ctx).engine_last_run[i as usize] = now;
            }
        }
        i += 1;
    }
    let mut rgb: *mut uint8_t = if any_due_frame_engine as ::core::ffi::c_int != 0 {
        frame_to_rgb24(frame)
    } else {
        ::core::ptr::null_mut::<uint8_t>()
    };
    if any_due_frame_engine as ::core::ffi::c_int != 0 && rgb.is_null() {
        return onvif_triggered;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*ctx).engine_count {
        let mut engine_0: *mut stream_detection_engine_t =
            (&raw mut (*ctx).engines as *mut stream_detection_engine_t).offset(i_0 as isize);
        's_98: {
            if !(!(*engine_0).enabled
                || (*ctx).engine_last_run[i_0 as usize] > 0 as time_t
                    && now - (*ctx).engine_last_run[i_0 as usize]
                        < (*engine_0).interval_seconds as time_t)
            {
                let mut candidate: detection_result_t = detection_result_t {
                    count: 0,
                    detections: [detection_t {
                        label: [0; 32],
                        confidence: 0.,
                        x: 0.,
                        y: 0.,
                        width: 0.,
                        height: 0.,
                        track_id: 0,
                        zone_id: [0; 32],
                    }; 20],
                };
                memset(
                    &raw mut candidate as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<detection_result_t>(),
                );
                let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                if strcmp(
                    &raw mut (*engine_0).engine_type as *mut ::core::ffi::c_char,
                    b"motion\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    result = detect_motion(
                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                        rgb as *const ::core::ffi::c_uchar,
                        (*frame).width,
                        (*frame).height,
                        3 as ::core::ffi::c_int,
                        now,
                        &raw mut candidate,
                    );
                } else if strcmp(
                    &raw mut (*engine_0).engine_type as *mut ::core::ffi::c_char,
                    b"object\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    if (*ctx).engine_models[i_0 as usize].is_null()
                        && !(*ctx).engine_load_failed[i_0 as usize]
                    {
                        (*ctx).engine_models[i_0 as usize] = load_detection_model(
                            &raw mut (*engine_0).model_path as *mut ::core::ffi::c_char,
                            (*engine_0).threshold,
                        );
                        if (*ctx).engine_models[i_0 as usize].is_null() {
                            (*ctx).engine_load_failed[i_0 as usize] = r#true != 0;
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"[%s] Detection engine %s failed to load model\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                                &raw mut (*engine_0).engine_key as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                    if !(*ctx).engine_models[i_0 as usize].is_null() {
                        result = detect_objects(
                            (*ctx).engine_models[i_0 as usize],
                            rgb as *const ::core::ffi::c_uchar,
                            (*frame).width,
                            (*frame).height,
                            3 as ::core::ffi::c_int,
                            &raw mut candidate,
                        );
                    }
                } else {
                    break 's_98;
                }
                (*ctx).engine_last_run[i_0 as usize] = now;
                if result == 0 as ::core::ffi::c_int {
                    append_engine_result(ctx, engine_0, combined, &raw mut candidate);
                } else if !(*ctx).engine_load_failed[i_0 as usize] {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"[%s] Detection engine %s failed\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                        &raw mut (*engine_0).engine_key as *mut ::core::ffi::c_char,
                    );
                }
            }
        }
        i_0 += 1;
    }
    free(rgb as *mut ::core::ffi::c_void);
    return (*combined).count > 0 as ::core::ffi::c_int
        || onvif_triggered as ::core::ffi::c_int != 0;
}
unsafe extern "C" fn detect_on_decoded_frame(
    mut ctx: *mut unified_detection_ctx_t,
    mut frame: *mut AVFrame,
    mut now: time_t,
    mut result: *mut detection_result_t,
) -> bool {
    if ctx.is_null() || frame.is_null() || result.is_null() {
        return r#false != 0;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    if (*ctx).engine_count > 1 as ::core::ffi::c_int {
        return detect_with_runtime_engines(ctx, frame, now, result);
    }
    let mut width: ::core::ffi::c_int = (*frame).width;
    let mut height: ::core::ffi::c_int = (*frame).height;
    if is_api_detection(&raw mut (*ctx).model_path as *mut ::core::ffi::c_char) {
        let mut rec_id: uint64_t = detection_link_recording_id(ctx);
        let mut rgb_buf: *mut uint8_t = frame_to_rgb24(frame);
        if rgb_buf.is_null() {
            return r#false != 0;
        }
        let mut api_url: *const ::core::ffi::c_char = get_actual_api_url(
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            &raw mut (*ctx).model_path as *mut ::core::ffi::c_char,
        );
        let mut api_options: api_detection_options_t = api_detection_options_t {
            format: api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT,
            backend: [0; 32],
            detector_name: [0; 64],
        };
        resolve_api_detection_options(ctx, &raw mut api_options);
        let mut ret: ::core::ffi::c_int = if !api_url.is_null() {
            detect_objects_api_with_options(
                api_url,
                rgb_buf as *const ::core::ffi::c_uchar,
                width,
                height,
                3 as ::core::ffi::c_int,
                result,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                (*ctx).detection_threshold,
                rec_id,
                now,
                &raw mut api_options,
            )
        } else {
            -1 as ::core::ffi::c_int
        };
        free(rgb_buf as *mut ::core::ffi::c_void);
        if ret != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] API detection (frame path) failed with error %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                ret,
            );
            return r#false != 0;
        }
    } else if is_motion_detection_model(&raw mut (*ctx).model_path as *mut ::core::ffi::c_char) {
        let mut rgb_buf_0: *mut uint8_t = frame_to_rgb24(frame);
        if rgb_buf_0.is_null() {
            return r#false != 0;
        }
        let mut ret_0: ::core::ffi::c_int = detect_motion(
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            rgb_buf_0 as *const ::core::ffi::c_uchar,
            width,
            height,
            3 as ::core::ffi::c_int,
            now,
            result,
        );
        free(rgb_buf_0 as *mut ::core::ffi::c_void);
        if ret_0 != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Motion detection failed with error %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                ret_0,
            );
            return r#false != 0;
        }
    } else {
        if (*ctx).model.is_null() {
            if (*ctx).model_path[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                return r#false != 0;
            }
            if (*ctx).model_load_failed {
                return r#false != 0;
            }
            (*ctx).model = load_detection_model(
                &raw mut (*ctx).model_path as *mut ::core::ffi::c_char,
                (*ctx).detection_threshold,
            );
            if (*ctx).model.is_null() {
                static mut kPrefix: [::core::ffi::c_char; 33] = unsafe {
                    ::core::mem::transmute::<[u8; 33], [::core::ffi::c_char; 33]>(
                        *b"Failed to load detection model: \0",
                    )
                };
                let mut msg: [::core::ffi::c_char; 545] = [0; 545];
                snprintf(
                    &raw mut msg as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 545]>(),
                    b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const kPrefix as *const ::core::ffi::c_char,
                    &raw mut (*ctx).model_path as *mut ::core::ffi::c_char,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[%s] %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    &raw mut msg as *mut ::core::ffi::c_char,
                );
                let mut sm: *mut stream_state_manager_t = get_stream_state_by_name(
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                );
                if !sm.is_null() {
                    handle_stream_error(
                        sm,
                        STREAM_ERR_MODEL_LOAD,
                        &raw mut msg as *mut ::core::ffi::c_char,
                    );
                }
                (*ctx).model_load_failed = r#true != 0;
                return r#false != 0;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Loaded detection model: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                &raw mut (*ctx).model_path as *mut ::core::ffi::c_char,
            );
        }
        let mut rgb_buf_1: *mut uint8_t = frame_to_rgb24(frame);
        if rgb_buf_1.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Failed to convert frame to RGB24\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        let mut ret_1: ::core::ffi::c_int = detect_objects(
            (*ctx).model,
            rgb_buf_1 as *const ::core::ffi::c_uchar,
            width,
            height,
            3 as ::core::ffi::c_int,
            result,
        );
        free(rgb_buf_1 as *mut ::core::ffi::c_void);
        if ret_1 != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Embedded model detection failed with error %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                ret_1,
            );
            return r#false != 0;
        }
    }
    if (*result).count == 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    filter_detections_by_zones(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        result,
    );
    filter_detections_by_stream_objects(
        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
        result,
    );
    let mut kept: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*result).count {
        if (*result).detections[i as usize].confidence >= (*ctx).detection_threshold {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Detection: %s (%.1f%%) at [%.2f, %.2f, %.2f, %.2f]\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut (*result).detections as *mut detection_t).offset(i as isize))
                    .label as *mut ::core::ffi::c_char,
                ((*result).detections[i as usize].confidence * 100.0f32) as ::core::ffi::c_double,
                (*result).detections[i as usize].x as ::core::ffi::c_double,
                (*result).detections[i as usize].y as ::core::ffi::c_double,
                (*result).detections[i as usize].width as ::core::ffi::c_double,
                (*result).detections[i as usize].height as ::core::ffi::c_double,
            );
            if kept != i {
                (*result).detections[kept as usize] = (*result).detections[i as usize];
            }
            kept += 1;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Detection below threshold: %s (%.1f%% < %.1f%%)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut (*result).detections as *mut detection_t).offset(i as isize))
                    .label as *mut ::core::ffi::c_char,
                ((*result).detections[i as usize].confidence * 100.0f32) as ::core::ffi::c_double,
                ((*ctx).detection_threshold * 100.0f32) as ::core::ffi::c_double,
            );
        }
        i += 1;
    }
    (*result).count = kept;
    return (*result).count > 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn report_detections(
    mut ctx: *mut unified_detection_ctx_t,
    mut result: *const detection_result_t,
    mut now: time_t,
) {
    if ctx.is_null() || result.is_null() || (*result).count == 0 as ::core::ffi::c_int {
        return;
    }
    if (*ctx).engine_count > 1 as ::core::ffi::c_int
        || !is_api_detection(&raw mut (*ctx).model_path as *mut ::core::ffi::c_char)
    {
        let mut rec_id: uint64_t = detection_link_recording_id(ctx);
        if store_detections_in_db_for_camera(
            &raw mut (*ctx).camera_uuid as *mut ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            result,
            now,
            rec_id,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Failed to store detections in database\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            );
        }
        let mut event_error: [::core::ffi::c_char; 256] = [
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
        if event_producer_publish_detection(
            &raw mut (*ctx).camera_uuid as *mut ::core::ffi::c_char,
            &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            result,
            now,
            &raw mut event_error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Detection event enqueue failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                &raw mut event_error as *mut ::core::ffi::c_char,
            );
        }
    }
    pthread_mutex_lock(&raw mut (*ctx).mutex);
    (*ctx).total_detections = (*ctx)
        .total_detections
        .wrapping_add((*result).count as uint64_t);
    pthread_mutex_unlock(&raw mut (*ctx).mutex);
}
unsafe extern "C" fn handle_recording_state(
    mut ctx: *mut unified_detection_ctx_t,
    mut detected: bool,
    mut now: time_t,
) {
    if ctx.is_null() {
        return;
    }
    let mut current_state: unified_detection_state_t =
        unified_detection_state_t(::core::intrinsics::atomic_load::<
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut (*ctx).state) as ::core::ffi::c_uint);
    if detected {
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*ctx).last_detection_time,
            now as ::core::ffi::c_longlong,
        );
        if current_state.0 == unified_detection_state_t::UDT_STATE_BUFFERING.0
            && should_annotate_continuous(ctx) as ::core::ffi::c_int != 0
        {
            return;
        }
        if current_state.0 == unified_detection_state_t::UDT_STATE_BUFFERING.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Detection triggered, starting recording\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            );
            if udt_start_recording(ctx) == 0 as ::core::ffi::c_int {
                flush_prebuffer_and_correct_start(ctx, now, detection_event_time(ctx, now));
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*ctx).state,
                    unified_detection_state_t::UDT_STATE_RECORDING.0 as ::core::ffi::c_int,
                );
                let mut lookback: time_t = now
                    - ((*ctx).recording_grace_interval + g_config.detection_grace_period) as time_t;
                let mut updated: ::core::ffi::c_int = update_detections_recording_id(
                    &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                    (*ctx).current_recording_id,
                    lookback,
                );
                if updated > 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"[%s] Linked %d recent detections to recording ID %lu\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                        updated,
                        (*ctx).current_recording_id as ::core::ffi::c_ulong,
                    );
                }
            }
        } else if current_state.0 == unified_detection_state_t::UDT_STATE_POST_BUFFER.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] Detection during post-buffer, continuing recording\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).state,
                unified_detection_state_t::UDT_STATE_RECORDING.0 as ::core::ffi::c_int,
            );
        }
    } else if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*ctx).current_recording_id,
    ) != 0 as uint64_t
        && current_state.0 == unified_detection_state_t::UDT_STATE_RECORDING.0
    {
        let mut since_last: time_t = now
            - ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).last_detection_time,
            ) as time_t;
        let mut grace_window: time_t =
            (*ctx).recording_grace_interval as time_t + g_config.detection_grace_period as time_t;
        if since_last > grace_window {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] No detection for %lds, entering post-buffer (%d seconds)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*ctx).stream_name as *mut ::core::ffi::c_char,
                since_last,
                (*ctx).post_buffer_seconds,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).post_buffer_end_time,
                (now + (*ctx).post_buffer_seconds as time_t) as ::core::ffi::c_longlong,
            );
            ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*ctx).state,
                unified_detection_state_t::UDT_STATE_POST_BUFFER.0 as ::core::ffi::c_int,
            );
        }
    }
}
pub const ONVIF_PULL_DROP_RETRY_DELAY_MS: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
pub const ONVIF_DETECT_RETRY: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const STREAM_ERR_MODEL_LOAD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RETENTION_TIER_IMPORTANT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
