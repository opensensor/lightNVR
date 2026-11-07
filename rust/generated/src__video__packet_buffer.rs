use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type AVBuffer;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn av_packet_clone(src: *const AVPacket) -> *mut AVPacket;
    unsafe fn av_packet_free(pkt: *mut *mut AVPacket);
    static mut g_config: config_t;
    unsafe fn get_streaming_config() -> *mut config_t;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn ensure_dir(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
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
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
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
pub struct packet_buffer_pool_t {
    pub buffers: [packet_buffer_t; 1024],
    pub pool_mutex: pthread_mutex_t,
    pub active_buffers: ::core::ffi::c_int,
    pub total_memory_limit: size_t,
    pub current_memory_usage: size_t,
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
pub const AV_PKT_FLAG_KEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
#[inline]
unsafe extern "C" fn configured_stream_slots() -> ::core::ffi::c_int {
    let mut slots: ::core::ffi::c_int = g_config.max_streams;
    if slots <= 0 as ::core::ffi::c_int || slots > MAX_STREAMS {
        return MAX_STREAMS;
    }
    return slots;
}
pub const MAX_BUFFER_SECONDS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const MIN_BUFFER_SECONDS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const DEFAULT_BUFFER_SECONDS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PACKET_BUFFER_TRIGGER_HEADROOM_SECONDS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
static mut buffer_pool: packet_buffer_pool_t = packet_buffer_pool_t {
    buffers: [packet_buffer_t {
        stream_name: [0; 256],
        buffer_seconds: 0,
        max_packets: 0,
        mode: buffer_mode_t::BUFFER_MODE_MEMORY,
        packets: ::core::ptr::null_mut::<buffered_packet_t>(),
        head: 0,
        tail: 0,
        count: 0,
        total_packets_buffered: 0,
        total_packets_dropped: 0,
        total_bytes_buffered: 0,
        current_memory_usage: 0,
        peak_memory_usage: 0,
        oldest_packet_time: 0,
        newest_packet_time: 0,
        disk_buffer_path: [0; 512],
        disk_buffer_file: ::core::ptr::null_mut::<FILE>(),
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __glibc_reserved: 0,
                __list: __pthread_list_t {
                    __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                    __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
                },
            },
        },
        mutex_initialized: false,
        active: false,
    }; 1024],
    pool_mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __glibc_reserved: 0,
            __list: __pthread_list_t {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    active_buffers: 0,
    total_memory_limit: 0,
    current_memory_usage: 0,
};
static mut pool_initialized: bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_packet_buffer_pool(
    mut memory_limit_mb: size_t,
) -> ::core::ffi::c_int {
    if pool_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Packet buffer pool already initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    memset(
        &raw mut buffer_pool.buffers as *mut packet_buffer_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (configured_stream_slots() as size_t)
            .wrapping_mul(::core::mem::size_of::<packet_buffer_t>()),
    );
    if pthread_mutex_init(
        &raw mut buffer_pool.pool_mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize buffer pool mutex\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    buffer_pool.total_memory_limit = memory_limit_mb
        .wrapping_mul(1024 as size_t)
        .wrapping_mul(1024 as size_t);
    buffer_pool.current_memory_usage = 0 as size_t;
    buffer_pool.active_buffers = 0 as ::core::ffi::c_int;
    pool_initialized = r#true != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Packet buffer pool initialized (memory limit: %zu MB)\0".as_ptr()
            as *const ::core::ffi::c_char,
        memory_limit_mb,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_packet_buffer_pool() {
    if !pool_initialized {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_STREAMS {
        if buffer_pool.buffers[i as usize].active {
            destroy_packet_buffer(
                (&raw mut buffer_pool.buffers as *mut packet_buffer_t).offset(i as isize),
            );
        }
        i += 1;
    }
    pthread_mutex_destroy(&raw mut buffer_pool.pool_mutex);
    pool_initialized = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Packet buffer pool cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_estimate_packet_count(
    mut fps: ::core::ffi::c_int,
    mut duration_seconds: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return ((fps * duration_seconds) as ::core::ffi::c_double * 1.2f64) as ::core::ffi::c_int;
}
unsafe extern "C" fn estimate_stream_buffer_mb(
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut fps: ::core::ffi::c_int,
    mut pre_buffer_seconds: ::core::ffi::c_int,
) -> size_t {
    if width <= 0 as ::core::ffi::c_int {
        width = 1280 as ::core::ffi::c_int;
    }
    if height <= 0 as ::core::ffi::c_int {
        height = 720 as ::core::ffi::c_int;
    }
    if fps <= 0 as ::core::ffi::c_int {
        fps = 15 as ::core::ffi::c_int;
    }
    if pre_buffer_seconds <= 0 as ::core::ffi::c_int {
        pre_buffer_seconds = DEFAULT_BUFFER_SECONDS;
    }
    if pre_buffer_seconds > MAX_BUFFER_SECONDS {
        pre_buffer_seconds = MAX_BUFFER_SECONDS;
    }
    let mut bytes_per_sec: ::core::ffi::c_double = width as ::core::ffi::c_double
        * height as ::core::ffi::c_double
        * fps as ::core::ffi::c_double
        * 0.1f64
        / 8.0f64;
    bytes_per_sec += 8000.0f64;
    let mut total_bytes: ::core::ffi::c_double =
        bytes_per_sec * pre_buffer_seconds as ::core::ffi::c_double * 1.25f64;
    let mut mb: size_t = (total_bytes / (1024.0f64 * 1024.0f64)) as size_t;
    return if mb < 2 as size_t { 2 as size_t } else { mb };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn calculate_packet_buffer_pool_size() -> size_t {
    let mut cfg: *mut config_t = get_streaming_config();
    if cfg.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"calculate_packet_buffer_pool_size: config unavailable, using 32 MB default\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 32 as size_t;
    }
    let mut total_mb: size_t = 0 as size_t;
    let mut detection_streams: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*cfg).max_streams {
        let mut s: *const stream_config_t = (*cfg).streams.offset(i as isize);
        if !(!(*s).enabled || !(*s).detection_based_recording) {
            let mut pre_buf: ::core::ffi::c_int =
                if (*s).pre_detection_buffer > 0 as ::core::ffi::c_int {
                    (*s).pre_detection_buffer
                } else if (*cfg).default_pre_detection_buffer > 0 as ::core::ffi::c_int {
                    (*cfg).default_pre_detection_buffer
                } else {
                    DEFAULT_BUFFER_SECONDS
                };
            total_mb = total_mb.wrapping_add(estimate_stream_buffer_mb(
                (*s).width,
                (*s).height,
                (*s).fps,
                pre_buf,
            ));
            detection_streams += 1;
        }
        i += 1;
    }
    if detection_streams == 0 as ::core::ffi::c_int {
        return 16 as size_t;
    }
    total_mb = (total_mb as ::core::ffi::c_double * 1.2f64) as size_t;
    if total_mb < 16 as size_t {
        total_mb = 16 as size_t;
    }
    if total_mb > 512 as size_t {
        total_mb = 512 as size_t;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Calculated packet buffer pool size: %zu MB for %d detection stream(s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        total_mb,
        detection_streams,
    );
    return total_mb;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reinit_packet_buffer_pool(
    mut new_memory_limit_mb: size_t,
) -> ::core::ffi::c_int {
    if !pool_initialized {
        return init_packet_buffer_pool(new_memory_limit_mb);
    }
    pthread_mutex_lock(&raw mut buffer_pool.pool_mutex);
    let mut old_limit_mb: size_t = buffer_pool
        .total_memory_limit
        .wrapping_div((1024 as ::core::ffi::c_int as size_t).wrapping_mul(1024 as size_t));
    if old_limit_mb == new_memory_limit_mb {
        pthread_mutex_unlock(&raw mut buffer_pool.pool_mutex);
        return 0 as ::core::ffi::c_int;
    }
    buffer_pool.total_memory_limit = new_memory_limit_mb
        .wrapping_mul(1024 as size_t)
        .wrapping_mul(1024 as size_t);
    pthread_mutex_unlock(&raw mut buffer_pool.pool_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Packet buffer pool memory limit updated: %zu MB -> %zu MB\0".as_ptr()
            as *const ::core::ffi::c_char,
        old_limit_mb,
        new_memory_limit_mb,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_packet_buffer(
    mut stream_name: *const ::core::ffi::c_char,
    mut buffer_seconds: ::core::ffi::c_int,
    mut mode: buffer_mode_t,
) -> *mut packet_buffer_t {
    if !pool_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Packet buffer pool not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<packet_buffer_t>();
    }
    if stream_name.is_null()
        || buffer_seconds < MIN_BUFFER_SECONDS
        || buffer_seconds > MAX_BUFFER_SECONDS + PACKET_BUFFER_TRIGGER_HEADROOM_SECONDS
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for create_packet_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<packet_buffer_t>();
    }
    pthread_mutex_lock(&raw mut buffer_pool.pool_mutex);
    let mut buffer: *mut packet_buffer_t = ::core::ptr::null_mut::<packet_buffer_t>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_STREAMS {
        if !buffer_pool.buffers[i as usize].active {
            buffer = (&raw mut buffer_pool.buffers as *mut packet_buffer_t).offset(i as isize);
            break;
        } else {
            i += 1;
        }
    }
    if buffer.is_null() {
        pthread_mutex_unlock(&raw mut buffer_pool.pool_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No free buffer slots available\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<packet_buffer_t>();
    }
    memset(
        buffer as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<packet_buffer_t>(),
    );
    if pthread_mutex_init(
        &raw mut (*buffer).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize mutex for packet buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut buffer_pool.pool_mutex);
        return ::core::ptr::null_mut::<packet_buffer_t>();
    }
    (*buffer).mutex_initialized = r#true != 0;
    safe_strcpy(
        &raw mut (*buffer).stream_name as *mut ::core::ffi::c_char,
        stream_name,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    (*buffer).buffer_seconds = buffer_seconds;
    (*buffer).mode = mode;
    (*buffer).max_packets =
        packet_buffer_estimate_packet_count(15 as ::core::ffi::c_int, buffer_seconds);
    (*buffer).packets = calloc(
        (*buffer).max_packets as size_t,
        ::core::mem::size_of::<buffered_packet_t>(),
    ) as *mut buffered_packet_t;
    if (*buffer).packets.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate packet array for buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_destroy(&raw mut (*buffer).mutex);
        (*buffer).mutex_initialized = r#false != 0;
        pthread_mutex_unlock(&raw mut buffer_pool.pool_mutex);
        return ::core::ptr::null_mut::<packet_buffer_t>();
    }
    (*buffer).head = 0 as ::core::ffi::c_int;
    (*buffer).tail = 0 as ::core::ffi::c_int;
    (*buffer).count = 0 as ::core::ffi::c_int;
    (*buffer).active = r#true != 0;
    if mode.0 == buffer_mode_t::BUFFER_MODE_DISK.0 || mode.0 == buffer_mode_t::BUFFER_MODE_HYBRID.0
    {
        let mut config: *const config_t = get_streaming_config();
        if !config.is_null() {
            snprintf(
                &raw mut (*buffer).disk_buffer_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                b"%s/.packet_buffer_%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*config).storage_path as *const ::core::ffi::c_char,
                stream_name,
            );
            if ensure_dir(&raw mut (*buffer).disk_buffer_path as *mut ::core::ffi::c_char) != 0 {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to create disk buffer directory %s: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*buffer).disk_buffer_path as *mut ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
                pthread_mutex_destroy(&raw mut (*buffer).mutex);
                (*buffer).mutex_initialized = r#false != 0;
                pthread_mutex_unlock(&raw mut buffer_pool.pool_mutex);
                return ::core::ptr::null_mut::<packet_buffer_t>();
            }
        }
    }
    buffer_pool.active_buffers += 1;
    pthread_mutex_unlock(&raw mut buffer_pool.pool_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Created packet buffer for stream: %s (duration: %ds, max packets: %d, mode: %d)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        buffer_seconds,
        (*buffer).max_packets,
        mode.0,
    );
    return buffer;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_packet_buffer(mut buffer: *mut packet_buffer_t) {
    if buffer.is_null() || !(*buffer).active {
        return;
    }
    if (*buffer).mutex_initialized {
        pthread_mutex_lock(&raw mut (*buffer).mutex);
    }
    if !(*buffer).packets.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*buffer).max_packets {
            if !(*(*buffer).packets.offset(i as isize)).packet.is_null() {
                av_packet_free(&raw mut (*(*buffer).packets.offset(i as isize)).packet);
            }
            i += 1;
        }
        free((*buffer).packets as *mut ::core::ffi::c_void);
        (*buffer).packets = ::core::ptr::null_mut::<buffered_packet_t>();
    }
    if !(*buffer).disk_buffer_file.is_null() {
        fclose((*buffer).disk_buffer_file);
        (*buffer).disk_buffer_file = ::core::ptr::null_mut::<FILE>();
    }
    pthread_mutex_lock(&raw mut buffer_pool.pool_mutex);
    buffer_pool.current_memory_usage = buffer_pool
        .current_memory_usage
        .wrapping_sub((*buffer).current_memory_usage);
    buffer_pool.active_buffers -= 1;
    pthread_mutex_unlock(&raw mut buffer_pool.pool_mutex);
    let mut stream_name_copy: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut stream_name_copy as *mut ::core::ffi::c_char,
        &raw mut (*buffer).stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    (*buffer).active = r#false != 0;
    if (*buffer).mutex_initialized {
        pthread_mutex_unlock(&raw mut (*buffer).mutex);
        pthread_mutex_destroy(&raw mut (*buffer).mutex);
        (*buffer).mutex_initialized = r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Destroyed packet buffer for stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name_copy as *mut ::core::ffi::c_char,
    );
}
pub const PACKET_BUFFER_MAX_PACKETS_PER_SECOND: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
unsafe extern "C" fn packet_buffer_grow_locked(
    mut buffer: *mut packet_buffer_t,
) -> ::core::ffi::c_int {
    let mut ceiling: ::core::ffi::c_int = packet_buffer_estimate_packet_count(
        PACKET_BUFFER_MAX_PACKETS_PER_SECOND,
        (*buffer).buffer_seconds,
    );
    if (*buffer).max_packets >= ceiling {
        return -1 as ::core::ffi::c_int;
    }
    let mut new_max: ::core::ffi::c_int = (*buffer).max_packets * 2 as ::core::ffi::c_int;
    if new_max > ceiling {
        new_max = ceiling;
    }
    let mut grown: *mut buffered_packet_t = calloc(
        new_max as size_t,
        ::core::mem::size_of::<buffered_packet_t>(),
    ) as *mut buffered_packet_t;
    if grown.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] Failed to grow packet buffer to %d slots; evicting instead\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*buffer).stream_name as *mut ::core::ffi::c_char,
            new_max,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*buffer).count {
        *grown.offset(i as isize) = *(*buffer)
            .packets
            .offset((((*buffer).tail + i) % (*buffer).max_packets) as isize);
        i += 1;
    }
    free((*buffer).packets as *mut ::core::ffi::c_void);
    (*buffer).packets = grown;
    (*buffer).tail = 0 as ::core::ffi::c_int;
    (*buffer).head = (*buffer).count;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"[%s] Packet buffer slots grown %d -> %d to honour %ds pre-buffer window\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*buffer).stream_name as *mut ::core::ffi::c_char,
        (*buffer).max_packets,
        new_max,
        (*buffer).buffer_seconds,
    );
    (*buffer).max_packets = new_max;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_add_packet(
    mut buffer: *mut packet_buffer_t,
    mut packet: *const AVPacket,
    mut timestamp: time_t,
) -> ::core::ffi::c_int {
    if buffer.is_null() || !(*buffer).active || packet.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*buffer).mutex);
    while (*buffer).count > 0 as ::core::ffi::c_int {
        let mut oldest: time_t = (*(*buffer).packets.offset((*buffer).tail as isize)).timestamp;
        if timestamp - oldest <= (*buffer).buffer_seconds as time_t {
            break;
        }
        if !(*(*buffer).packets.offset((*buffer).tail as isize))
            .packet
            .is_null()
        {
            (*buffer).current_memory_usage = (*buffer)
                .current_memory_usage
                .wrapping_sub((*(*buffer).packets.offset((*buffer).tail as isize)).data_size);
            av_packet_free(&raw mut (*(*buffer).packets.offset((*buffer).tail as isize)).packet);
            (*(*buffer).packets.offset((*buffer).tail as isize)).packet =
                ::core::ptr::null_mut::<AVPacket>();
        }
        (*buffer).tail = ((*buffer).tail + 1 as ::core::ffi::c_int) % (*buffer).max_packets;
        (*buffer).count -= 1;
        (*buffer).total_packets_dropped = (*buffer).total_packets_dropped.wrapping_add(1);
    }
    if (*buffer).count >= (*buffer).max_packets {
        packet_buffer_grow_locked(buffer);
    }
    if (*buffer).count >= (*buffer).max_packets {
        if !(*(*buffer).packets.offset((*buffer).tail as isize))
            .packet
            .is_null()
        {
            (*buffer).current_memory_usage = (*buffer)
                .current_memory_usage
                .wrapping_sub((*(*buffer).packets.offset((*buffer).tail as isize)).data_size);
            av_packet_free(&raw mut (*(*buffer).packets.offset((*buffer).tail as isize)).packet);
            (*(*buffer).packets.offset((*buffer).tail as isize)).packet =
                ::core::ptr::null_mut::<AVPacket>();
        }
        (*buffer).tail = ((*buffer).tail + 1 as ::core::ffi::c_int) % (*buffer).max_packets;
        (*buffer).count -= 1;
        (*buffer).total_packets_dropped = (*buffer).total_packets_dropped.wrapping_add(1);
    }
    let mut cloned_packet: *mut AVPacket = av_packet_clone(packet);
    if cloned_packet.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to clone packet for buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut (*buffer).mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut slot: *mut buffered_packet_t = (*buffer).packets.offset((*buffer).head as isize);
    (*slot).packet = cloned_packet as *mut AVPacket;
    (*slot).timestamp = timestamp;
    (*slot).pts = (*packet).pts;
    (*slot).dts = (*packet).dts;
    (*slot).stream_index = (*packet).stream_index;
    (*slot).is_keyframe = (*packet).flags & AV_PKT_FLAG_KEY != 0 as ::core::ffi::c_int;
    (*slot).data_size = (*packet).size as size_t;
    (*buffer).current_memory_usage = (*buffer)
        .current_memory_usage
        .wrapping_add((*packet).size as size_t);
    if (*buffer).current_memory_usage > (*buffer).peak_memory_usage {
        (*buffer).peak_memory_usage = (*buffer).current_memory_usage;
    }
    (*buffer).total_bytes_buffered = (*buffer)
        .total_bytes_buffered
        .wrapping_add((*packet).size as uint64_t);
    (*buffer).total_packets_buffered = (*buffer).total_packets_buffered.wrapping_add(1);
    if (*buffer).count == 0 as ::core::ffi::c_int {
        (*buffer).oldest_packet_time = timestamp;
    } else {
        (*buffer).oldest_packet_time =
            (*(*buffer).packets.offset((*buffer).tail as isize)).timestamp;
    }
    (*buffer).newest_packet_time = timestamp;
    (*buffer).head = ((*buffer).head + 1 as ::core::ffi::c_int) % (*buffer).max_packets;
    (*buffer).count += 1;
    pthread_mutex_unlock(&raw mut (*buffer).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_peek_oldest(
    mut buffer: *mut packet_buffer_t,
    mut packet: *mut *mut AVPacket,
) -> ::core::ffi::c_int {
    if buffer.is_null() || !(*buffer).active || packet.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*buffer).mutex);
    if (*buffer).count == 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut (*buffer).mutex);
        return -1 as ::core::ffi::c_int;
    }
    *packet = av_packet_clone((*(*buffer).packets.offset((*buffer).tail as isize)).packet)
        as *mut AVPacket;
    pthread_mutex_unlock(&raw mut (*buffer).mutex);
    return if !(*packet).is_null() {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_pop_oldest(
    mut buffer: *mut packet_buffer_t,
    mut packet: *mut *mut AVPacket,
) -> ::core::ffi::c_int {
    if buffer.is_null() || !(*buffer).active || packet.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*buffer).mutex);
    if (*buffer).count == 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut (*buffer).mutex);
        return -1 as ::core::ffi::c_int;
    }
    *packet = (*(*buffer).packets.offset((*buffer).tail as isize)).packet as *mut AVPacket;
    (*(*buffer).packets.offset((*buffer).tail as isize)).packet =
        ::core::ptr::null_mut::<AVPacket>();
    (*buffer).current_memory_usage = (*buffer)
        .current_memory_usage
        .wrapping_sub((*(*buffer).packets.offset((*buffer).tail as isize)).data_size);
    (*buffer).tail = ((*buffer).tail + 1 as ::core::ffi::c_int) % (*buffer).max_packets;
    (*buffer).count -= 1;
    if (*buffer).count > 0 as ::core::ffi::c_int {
        (*buffer).oldest_packet_time =
            (*(*buffer).packets.offset((*buffer).tail as isize)).timestamp;
    }
    pthread_mutex_unlock(&raw mut (*buffer).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_flush(
    mut buffer: *mut packet_buffer_t,
    mut callback: Option<
        unsafe extern "C" fn(*const AVPacket, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    mut user_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if buffer.is_null() || !(*buffer).active || callback.is_none() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*buffer).mutex);
    let mut flushed_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current_count: ::core::ffi::c_int = (*buffer).count;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < current_count {
        let mut index: ::core::ffi::c_int = ((*buffer).tail + i) % (*buffer).max_packets;
        if !(*(*buffer).packets.offset(index as isize)).packet.is_null() {
            let mut result: ::core::ffi::c_int = callback.expect("non-null function pointer")(
                (*(*buffer).packets.offset(index as isize)).packet,
                user_data,
            );
            if result == 0 as ::core::ffi::c_int {
                flushed_count += 1;
            }
            (*buffer).current_memory_usage = (*buffer)
                .current_memory_usage
                .wrapping_sub((*(*buffer).packets.offset(index as isize)).data_size);
            av_packet_free(&raw mut (*(*buffer).packets.offset(index as isize)).packet);
            (*(*buffer).packets.offset(index as isize)).packet =
                ::core::ptr::null_mut::<AVPacket>();
        }
        i += 1;
    }
    (*buffer).head = 0 as ::core::ffi::c_int;
    (*buffer).tail = 0 as ::core::ffi::c_int;
    (*buffer).count = 0 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut (*buffer).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Flushed %d packets from buffer for stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
        flushed_count,
        &raw mut (*buffer).stream_name as *mut ::core::ffi::c_char,
    );
    return flushed_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_trim_before(
    mut buffer: *mut packet_buffer_t,
    mut cutoff: time_t,
    mut video_stream_index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if buffer.is_null() || !(*buffer).active {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*buffer).mutex);
    let mut start: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*buffer).count {
        let mut slot: *const buffered_packet_t = (*buffer)
            .packets
            .offset((((*buffer).tail + i) % (*buffer).max_packets) as isize);
        if (*slot).timestamp > cutoff {
            break;
        }
        if !(*slot).packet.is_null()
            && (*slot).is_keyframe as ::core::ffi::c_int != 0
            && (*slot).stream_index == video_stream_index
        {
            start = i;
        }
        i += 1;
    }
    let mut dropped: ::core::ffi::c_int = if start > 0 as ::core::ffi::c_int {
        start
    } else {
        0 as ::core::ffi::c_int
    };
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < dropped {
        let mut slot_0: *mut buffered_packet_t = (*buffer).packets.offset((*buffer).tail as isize);
        if !(*slot_0).packet.is_null() {
            (*buffer).current_memory_usage = (*buffer)
                .current_memory_usage
                .wrapping_sub((*slot_0).data_size);
            av_packet_free(&raw mut (*slot_0).packet);
            (*slot_0).packet = ::core::ptr::null_mut::<AVPacket>();
        }
        (*buffer).tail = ((*buffer).tail + 1 as ::core::ffi::c_int) % (*buffer).max_packets;
        (*buffer).count -= 1;
        i_0 += 1;
    }
    if (*buffer).count > 0 as ::core::ffi::c_int {
        (*buffer).oldest_packet_time =
            (*(*buffer).packets.offset((*buffer).tail as isize)).timestamp;
    }
    pthread_mutex_unlock(&raw mut (*buffer).mutex);
    return dropped;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_clear(mut buffer: *mut packet_buffer_t) {
    if buffer.is_null() || !(*buffer).active {
        return;
    }
    pthread_mutex_lock(&raw mut (*buffer).mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*buffer).max_packets {
        if !(*(*buffer).packets.offset(i as isize)).packet.is_null() {
            (*buffer).current_memory_usage = (*buffer)
                .current_memory_usage
                .wrapping_sub((*(*buffer).packets.offset(i as isize)).data_size);
            av_packet_free(&raw mut (*(*buffer).packets.offset(i as isize)).packet);
            (*(*buffer).packets.offset(i as isize)).packet = ::core::ptr::null_mut::<AVPacket>();
        }
        i += 1;
    }
    (*buffer).head = 0 as ::core::ffi::c_int;
    (*buffer).tail = 0 as ::core::ffi::c_int;
    (*buffer).count = 0 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut (*buffer).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleared buffer for stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*buffer).stream_name as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_get_stats(
    mut buffer: *mut packet_buffer_t,
    mut count: *mut ::core::ffi::c_int,
    mut memory_usage: *mut size_t,
    mut duration: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if buffer.is_null() || !(*buffer).active {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*buffer).mutex);
    if !count.is_null() {
        *count = (*buffer).count;
    }
    if !memory_usage.is_null() {
        *memory_usage = (*buffer).current_memory_usage;
    }
    if !duration.is_null() && (*buffer).count > 0 as ::core::ffi::c_int {
        *duration =
            ((*buffer).newest_packet_time - (*buffer).oldest_packet_time) as ::core::ffi::c_int;
    } else if !duration.is_null() {
        *duration = 0 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut (*buffer).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_packet_buffer(
    mut stream_name: *const ::core::ffi::c_char,
) -> *mut packet_buffer_t {
    if !pool_initialized || stream_name.is_null() {
        return ::core::ptr::null_mut::<packet_buffer_t>();
    }
    pthread_mutex_lock(&raw mut buffer_pool.pool_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_STREAMS {
        if buffer_pool.buffers[i as usize].active as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut buffer_pool.buffers as *mut packet_buffer_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            pthread_mutex_unlock(&raw mut buffer_pool.pool_mutex);
            return (&raw mut buffer_pool.buffers as *mut packet_buffer_t).offset(i as isize);
        }
        i += 1;
    }
    pthread_mutex_unlock(&raw mut buffer_pool.pool_mutex);
    return ::core::ptr::null_mut::<packet_buffer_t>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_is_ready(mut buffer: *mut packet_buffer_t) -> bool {
    if buffer.is_null() || !(*buffer).active {
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut (*buffer).mutex);
    let mut ready: bool = r#false != 0;
    if (*buffer).count > 0 as ::core::ffi::c_int {
        let mut duration: ::core::ffi::c_int =
            ((*buffer).newest_packet_time - (*buffer).oldest_packet_time) as ::core::ffi::c_int;
        ready = duration >= (*buffer).buffer_seconds;
    }
    pthread_mutex_unlock(&raw mut (*buffer).mutex);
    return ready;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_get_keyframe_count(
    mut buffer: *mut packet_buffer_t,
) -> ::core::ffi::c_int {
    if buffer.is_null() || !(*buffer).active {
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*buffer).mutex);
    let mut keyframe_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*buffer).count {
        let mut index: ::core::ffi::c_int = ((*buffer).tail + i) % (*buffer).max_packets;
        if (*(*buffer).packets.offset(index as isize)).is_keyframe {
            keyframe_count += 1;
        }
        i += 1;
    }
    pthread_mutex_unlock(&raw mut (*buffer).mutex);
    return keyframe_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_set_memory_limit(
    mut buffer: *mut packet_buffer_t,
    mut limit_mb: size_t,
) -> ::core::ffi::c_int {
    if buffer.is_null() || !(*buffer).active {
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Per-buffer memory limit (%zu MB) requested for %s is not enforced; using pool-level limits\0"
            .as_ptr() as *const ::core::ffi::c_char,
        limit_mb,
        &raw mut (*buffer).stream_name as *mut ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_get_total_memory_usage() -> size_t {
    if !pool_initialized {
        return 0 as size_t;
    }
    pthread_mutex_lock(&raw mut buffer_pool.pool_mutex);
    let mut total: size_t = buffer_pool.current_memory_usage;
    pthread_mutex_unlock(&raw mut buffer_pool.pool_mutex);
    return total;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_buffer_set_disk_fallback(
    mut buffer: *mut packet_buffer_t,
    mut enable: bool,
    mut disk_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if buffer.is_null() || !(*buffer).active {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*buffer).mutex);
    if enable {
        (*buffer).mode = buffer_mode_t::BUFFER_MODE_HYBRID;
        if !disk_path.is_null() {
            safe_strcpy(
                &raw mut (*buffer).disk_buffer_path as *mut ::core::ffi::c_char,
                disk_path,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Enabled disk fallback for buffer: %s (path: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*buffer).stream_name as *mut ::core::ffi::c_char,
            &raw mut (*buffer).disk_buffer_path as *mut ::core::ffi::c_char,
        );
    } else {
        (*buffer).mode = buffer_mode_t::BUFFER_MODE_MEMORY;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Disabled disk fallback for buffer: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*buffer).stream_name as *mut ::core::ffi::c_char,
        );
    }
    pthread_mutex_unlock(&raw mut (*buffer).mutex);
    return 0 as ::core::ffi::c_int;
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
