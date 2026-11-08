unsafe extern "C" {
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn storage_placement_select(
        stream_name: *const ::core::ffi::c_char,
        placement: *mut storage_placement_t,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn sanitize_stream_name(
        input: *const ::core::ffi::c_char,
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    );
    unsafe fn mkdir_recursive(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn chmod_path(path: *const ::core::ffi::c_char, mode: mode_t) -> ::core::ffi::c_int;
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
}
pub type size_t = usize;
pub type __mode_t = ::core::ffi::c_uint;
pub type __time_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
pub type time_t = __time_t;
pub type int64_t = i64;
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
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub struct storage_placement_status_t(pub ::core::ffi::c_uint);
impl storage_placement_status_t {
    pub const STORAGE_PLACEMENT_READY: Self = Self(0);
    pub const STORAGE_PLACEMENT_PAUSED: Self = Self(1);
    pub const STORAGE_PLACEMENT_FAILED: Self = Self(2);
    pub const STORAGE_PLACEMENT_ERROR: Self = Self(3);
}
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
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MP4_DIRECTORY_FORMAT_FLAT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"flat\0") };
pub const MP4_DIRECTORY_FORMAT_YEAR_MONTH: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"year_month\0") };
pub const MP4_DIRECTORY_FORMAT_YEAR_MONTH_DAY: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"year_month_day\0") };
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mp4_directory_format_is_valid(
    mut value: *const ::core::ffi::c_char,
) -> bool {
    return !value.is_null()
        && (strcmp(value, MP4_DIRECTORY_FORMAT_FLAT.as_ptr()) == 0 as ::core::ffi::c_int
            || strcmp(value, MP4_DIRECTORY_FORMAT_YEAR_MONTH.as_ptr()) == 0 as ::core::ffi::c_int
            || strcmp(value, MP4_DIRECTORY_FORMAT_YEAR_MONTH_DAY.as_ptr())
                == 0 as ::core::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_mp4_recording_directory(
    mut config: *const config_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut timestamp: time_t,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    if config.is_null()
        || stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || output.is_null()
        || output_size == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut root: *const ::core::ffi::c_char =
        &raw const (*config).storage_path as *const ::core::ffi::c_char;
    let mut default_root: [::core::ffi::c_char; 512] = [0; 512];
    if (*config).record_mp4_directly as ::core::ffi::c_int != 0
        && (*config).mp4_storage_path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        root = &raw const (*config).mp4_storage_path as *const ::core::ffi::c_char;
    } else {
        let mut root_len: ::core::ffi::c_int = snprintf(
            &raw mut default_root as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/mp4\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).storage_path as *const ::core::ffi::c_char,
        );
        if root_len < 0 as ::core::ffi::c_int
            || root_len as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        {
            return -1 as ::core::ffi::c_int;
        }
        root = &raw mut default_root as *mut ::core::ffi::c_char;
    }
    return build_mp4_recording_directory_at_root(
        config,
        root,
        stream_name,
        timestamp,
        output,
        output_size,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_mp4_recording_directory_at_root(
    mut config: *const config_t,
    mut recording_root: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut timestamp: time_t,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    if config.is_null()
        || recording_root.is_null()
        || *recording_root.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        || stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || output.is_null()
        || output_size == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut safe_stream: [::core::ffi::c_char; 256] = [0; 256];
    sanitize_stream_name(
        stream_name,
        &raw mut safe_stream as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    let mut format: *const ::core::ffi::c_char =
        &raw const (*config).mp4_directory_format as *const ::core::ffi::c_char;
    if !mp4_directory_format_is_valid(format) {
        format = MP4_DIRECTORY_FORMAT_YEAR_MONTH_DAY.as_ptr();
    }
    let mut written: ::core::ffi::c_int = 0;
    if strcmp(format, MP4_DIRECTORY_FORMAT_FLAT.as_ptr()) == 0 as ::core::ffi::c_int {
        written = snprintf(
            output,
            output_size,
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            recording_root,
            &raw mut safe_stream as *mut ::core::ffi::c_char,
        );
    } else {
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
        if localtime_r(&raw mut timestamp, &raw mut tm_buf).is_null() {
            return -1 as ::core::ffi::c_int;
        }
        if strcmp(format, MP4_DIRECTORY_FORMAT_YEAR_MONTH.as_ptr()) == 0 as ::core::ffi::c_int {
            written = snprintf(
                output,
                output_size,
                b"%s/%s/%04d/%02d\0".as_ptr() as *const ::core::ffi::c_char,
                recording_root,
                &raw mut safe_stream as *mut ::core::ffi::c_char,
                tm_buf.tm_year + 1900 as ::core::ffi::c_int,
                tm_buf.tm_mon + 1 as ::core::ffi::c_int,
            );
        } else {
            written = snprintf(
                output,
                output_size,
                b"%s/%s/%04d/%02d/%02d\0".as_ptr() as *const ::core::ffi::c_char,
                recording_root,
                &raw mut safe_stream as *mut ::core::ffi::c_char,
                tm_buf.tm_year + 1900 as ::core::ffi::c_int,
                tm_buf.tm_mon + 1 as ::core::ffi::c_int,
                tm_buf.tm_mday,
            );
        }
    }
    return if written < 0 as ::core::ffi::c_int || written as size_t >= output_size {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_recording_transcode_cache_path(
    mut storage_path: *const ::core::ffi::c_char,
    mut recording_id: uint64_t,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    if storage_path.is_null()
        || *storage_path.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        || recording_id == 0 as uint64_t
        || output.is_null()
        || output_size == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut written: ::core::ffi::c_int = snprintf(
        output,
        output_size,
        b"%s/transcoded/%llu.mp4\0".as_ptr() as *const ::core::ffi::c_char,
        storage_path,
        recording_id as ::core::ffi::c_ulonglong,
    );
    return if written < 0 as ::core::ffi::c_int || written as size_t >= output_size {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn effective_placement_root(
    mut config: *const config_t,
    mut placement: *const storage_placement_t,
    mut root: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if config.is_null()
        || placement.is_null()
        || (*placement).target_root[0usize] as ::core::ffi::c_int != '/' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if (*placement).target_is_default as ::core::ffi::c_int != 0 && !(*config).record_mp4_directly {
        let mut written: ::core::ffi::c_int = snprintf(
            root,
            MAX_PATH_LENGTH as size_t,
            b"%s/mp4\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*placement).target_root as *const ::core::ffi::c_char,
        );
        return if written < 0 as ::core::ffi::c_int || written >= MAX_PATH_LENGTH {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    safe_strcpy(
        root,
        &raw const (*placement).target_root as *const ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_mp4_recording_directory_for_placement(
    mut config: *const config_t,
    mut placement: *const storage_placement_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut timestamp: time_t,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    let mut root: [::core::ffi::c_char; 512] = [0; 512];
    if effective_placement_root(config, placement, &raw mut root as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    return build_mp4_recording_directory_at_root(
        config,
        &raw mut root as *mut ::core::ffi::c_char,
        stream_name,
        timestamp,
        output,
        output_size,
    );
}
unsafe extern "C" fn prepare_at_root(
    mut config: *const config_t,
    mut root: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut timestamp: time_t,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    let mut directory: [::core::ffi::c_char; 512] = [0; 512];
    if build_mp4_recording_directory_at_root(
        config,
        root,
        stream_name,
        timestamp,
        &raw mut directory as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if mkdir_recursive(&raw mut directory as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    chmod_path(
        &raw mut directory as *mut ::core::ffi::c_char,
        0o755 as mode_t,
    );
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
    if localtime_r(&raw mut timestamp, &raw mut tm_buf).is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut timestamp_text: [::core::ffi::c_char; 32] = [0; 32];
    if strftime(
        &raw mut timestamp_text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%Y%m%d_%H%M%S\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut tm_buf,
    ) == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char)
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
    let mut written: ::core::ffi::c_int = snprintf(
        output,
        output_size,
        b"%s/recording_%s_%s.mp4\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut directory as *mut ::core::ffi::c_char,
        &raw mut timestamp_text as *mut ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    return if written < 0 as ::core::ffi::c_int || written as size_t >= output_size {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_mp4_recording_path(
    mut config: *const config_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut timestamp: time_t,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    if config.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut root: *const ::core::ffi::c_char =
        &raw const (*config).storage_path as *const ::core::ffi::c_char;
    let mut default_root: [::core::ffi::c_char; 512] = [0; 512];
    if (*config).record_mp4_directly as ::core::ffi::c_int != 0
        && (*config).mp4_storage_path[0usize] as ::core::ffi::c_int != 0
    {
        root = &raw const (*config).mp4_storage_path as *const ::core::ffi::c_char;
    } else {
        let mut written: ::core::ffi::c_int = snprintf(
            &raw mut default_root as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/mp4\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).storage_path as *const ::core::ffi::c_char,
        );
        if written < 0 as ::core::ffi::c_int
            || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        {
            return -1 as ::core::ffi::c_int;
        }
        root = &raw mut default_root as *mut ::core::ffi::c_char;
    }
    return prepare_at_root(config, root, stream_name, timestamp, output, output_size);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_placed_mp4_recording_path(
    mut config: *const config_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut timestamp: time_t,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
    mut placement: *mut storage_placement_t,
) -> ::core::ffi::c_int {
    if placement.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        placement as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_placement_t>(),
    );
    (*placement).status = storage_placement_status_t::STORAGE_PLACEMENT_ERROR;
    safe_strcpy(
        &raw mut (*placement).reason as *mut ::core::ffi::c_char,
        b"invalid-request\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    if config.is_null()
        || storage_placement_select(stream_name, placement) != 0 as ::core::ffi::c_int
        || (*placement).status.0 != storage_placement_status_t::STORAGE_PLACEMENT_READY.0
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut root: [::core::ffi::c_char; 512] = [0; 512];
    if effective_placement_root(config, placement, &raw mut root as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
        || prepare_at_root(
            config,
            &raw mut root as *mut ::core::ffi::c_char,
            stream_name,
            timestamp,
            output,
            output_size,
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut root_length: size_t =
        strlen(&raw mut (*placement).target_root as *mut ::core::ffi::c_char);
    if strncmp(
        output,
        &raw mut (*placement).target_root as *mut ::core::ffi::c_char,
        root_length,
    ) != 0 as ::core::ffi::c_int
        || *output.offset(root_length as isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut (*placement).object_key as *mut ::core::ffi::c_char,
        output
            .offset(root_length as isize)
            .offset(1 as ::core::ffi::c_int as isize),
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    return if (*placement).object_key[0usize] as ::core::ffi::c_int != 0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
