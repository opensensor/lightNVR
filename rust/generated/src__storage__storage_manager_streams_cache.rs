unsafe extern "C" {
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
    unsafe fn cJSON_AddBoolToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        boolean: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddNumberToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        number: ::core::ffi::c_double,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn get_all_stream_storage_usage(
        stream_info: *mut *mut stream_storage_info_t,
    ) -> ::core::ffi::c_int;
    static mut g_config: config_t;
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub r#type: ::core::ffi::c_int,
    pub valuestring: *mut ::core::ffi::c_char,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: *mut ::core::ffi::c_char,
}
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_storage_info_t {
    pub name: [::core::ffi::c_char; 64],
    pub size_bytes: ::core::ffi::c_ulong,
    pub recording_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_storage_cache_t {
    pub stream_info: *mut stream_storage_info_t,
    pub stream_count: ::core::ffi::c_int,
    pub last_update: time_t,
    pub ttl_seconds: ::core::ffi::c_int,
    pub mutex: pthread_mutex_t,
    pub initialized: ::core::ffi::c_int,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
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
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut cache: stream_storage_cache_t = stream_storage_cache_t {
    stream_info: ::core::ptr::null_mut::<stream_storage_info_t>(),
    stream_count: 0 as ::core::ffi::c_int,
    last_update: 0 as time_t,
    ttl_seconds: 1800 as ::core::ffi::c_int,
    mutex: pthread_mutex_t {
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
    },
    initialized: 0 as ::core::ffi::c_int,
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_storage_manager_streams_cache(
    mut cache_ttl_seconds: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if pthread_mutex_init(
        &raw mut cache.mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize storage manager streams cache mutex\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut cache.mutex);
    cache.ttl_seconds = if cache_ttl_seconds < 10 as ::core::ffi::c_int {
        10 as ::core::ffi::c_int
    } else {
        cache_ttl_seconds
    };
    cache.initialized = 1 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut cache.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Storage manager streams cache initialized with TTL: %d seconds\0".as_ptr()
            as *const ::core::ffi::c_char,
        cache.ttl_seconds,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_cache_stale() -> ::core::ffi::c_int {
    if cache.initialized == 0 {
        return 1 as ::core::ffi::c_int;
    }
    if cache.stream_info.is_null() || cache.stream_count <= 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if now - cache.last_update > cache.ttl_seconds as time_t {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn refresh_cache() -> ::core::ffi::c_int {
    if cache.initialized == 0 {
        init_storage_manager_streams_cache(900 as ::core::ffi::c_int);
    }
    let mut stream_info: *mut stream_storage_info_t =
        ::core::ptr::null_mut::<stream_storage_info_t>();
    let mut stream_count: ::core::ffi::c_int = get_all_stream_storage_usage(&raw mut stream_info);
    if stream_count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get stream storage usage for cache refresh\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if stream_count == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No streams found for cache refresh (this is normal during startup)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    pthread_mutex_lock(&raw mut cache.mutex);
    if !cache.stream_info.is_null() {
        free(cache.stream_info as *mut ::core::ffi::c_void);
    }
    cache.stream_info = stream_info;
    cache.stream_count = stream_count;
    cache.last_update = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_unlock(&raw mut cache.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Storage manager streams cache refreshed with %d streams\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_count,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_cached_stream_storage_usage(
    mut stream_info: *mut *mut stream_storage_info_t,
    mut force_refresh: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stream_info.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for get_cached_stream_storage_usage\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    *stream_info = ::core::ptr::null_mut::<stream_storage_info_t>();
    if force_refresh != 0 || is_cache_stale() != 0 {
        if refresh_cache() != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to refresh storage manager streams cache\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    pthread_mutex_lock(&raw mut cache.mutex);
    if cache.stream_count == 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut cache.mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No streams in cache (this is normal during startup or when no streams are configured)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if cache.stream_info.is_null() {
        pthread_mutex_unlock(&raw mut cache.mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cache stream_count is %d but stream_info is NULL - cache is corrupted\0".as_ptr()
                as *const ::core::ffi::c_char,
            cache.stream_count,
        );
        return -1 as ::core::ffi::c_int;
    }
    *stream_info = malloc(
        (cache.stream_count as size_t)
            .wrapping_mul(::core::mem::size_of::<stream_storage_info_t>()),
    ) as *mut stream_storage_info_t;
    if (*stream_info).is_null() {
        pthread_mutex_unlock(&raw mut cache.mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for stream storage info\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memcpy(
        *stream_info as *mut ::core::ffi::c_void,
        cache.stream_info as *const ::core::ffi::c_void,
        (cache.stream_count as size_t)
            .wrapping_mul(::core::mem::size_of::<stream_storage_info_t>()),
    );
    let mut stream_count: ::core::ffi::c_int = cache.stream_count;
    pthread_mutex_unlock(&raw mut cache.mutex);
    return stream_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_cached_stream_storage_usage_to_json(
    mut json_obj: *mut cJSON,
    mut force_refresh: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if json_obj.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid JSON object for add_cached_stream_storage_usage_to_json\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream_storage_array: *mut cJSON = cJSON_CreateArray();
    if stream_storage_array.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create stream storage JSON array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if cache.initialized == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage manager streams cache not initialized, initializing with default TTL\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        if init_storage_manager_streams_cache(300 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to initialize storage manager streams cache\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            cJSON_AddItemToObject(
                json_obj,
                b"streamStorage\0".as_ptr() as *const ::core::ffi::c_char,
                stream_storage_array,
            );
            return 0 as ::core::ffi::c_int;
        }
        force_refresh = 1 as ::core::ffi::c_int;
    }
    let mut stream_info: *mut stream_storage_info_t =
        ::core::ptr::null_mut::<stream_storage_info_t>();
    let mut stream_count: ::core::ffi::c_int =
        get_cached_stream_storage_usage(&raw mut stream_info, force_refresh);
    if stream_count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get cached stream storage usage\0".as_ptr() as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Attempting to get stream storage usage directly\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(stream_info as *mut ::core::ffi::c_void);
        stream_info = ::core::ptr::null_mut::<stream_storage_info_t>();
        stream_count = get_all_stream_storage_usage(&raw mut stream_info);
        if stream_count < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to get stream storage usage directly\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            cJSON_AddItemToObject(
                json_obj,
                b"streamStorage\0".as_ptr() as *const ::core::ffi::c_char,
                stream_storage_array,
            );
            return 0 as ::core::ffi::c_int;
        }
    }
    if stream_count == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No streams found (this is normal during startup or when no streams are configured)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddItemToObject(
            json_obj,
            b"streamStorage\0".as_ptr() as *const ::core::ffi::c_char,
            stream_storage_array,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < stream_count {
        let mut stream_obj: *mut cJSON = cJSON_CreateObject();
        if !stream_obj.is_null() {
            cJSON_AddStringToObject(
                stream_obj,
                b"name\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*stream_info.offset(i as isize)).name as *mut ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"size\0".as_ptr() as *const ::core::ffi::c_char,
                (*stream_info.offset(i as isize)).size_bytes as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"count\0".as_ptr() as *const ::core::ffi::c_char,
                (*stream_info.offset(i as isize)).recording_count as ::core::ffi::c_double,
            );
            let mut retention_days: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
            let mut detection_retention_days: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
            let mut max_storage_mb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < g_config.max_streams {
                if (*g_config.streams.offset(j as isize)).name[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                    && strcmp(
                        &raw mut (*g_config.streams.offset(j as isize)).name
                            as *mut ::core::ffi::c_char,
                        &raw mut (*stream_info.offset(i as isize)).name as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    retention_days = (*g_config.streams.offset(j as isize)).retention_days;
                    detection_retention_days =
                        (*g_config.streams.offset(j as isize)).detection_retention_days;
                    max_storage_mb = (*g_config.streams.offset(j as isize)).max_storage_mb;
                    break;
                } else {
                    j += 1;
                }
            }
            cJSON_AddNumberToObject(
                stream_obj,
                b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
                retention_days as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"detection_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
                detection_retention_days as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"max_storage_mb\0".as_ptr() as *const ::core::ffi::c_char,
                max_storage_mb as ::core::ffi::c_double,
            );
            cJSON_AddItemToArray(stream_storage_array, stream_obj);
        }
        i += 1;
    }
    cJSON_AddItemToObject(
        json_obj,
        b"streamStorage\0".as_ptr() as *const ::core::ffi::c_char,
        stream_storage_array,
    );
    let mut cache_meta: *mut cJSON = cJSON_CreateObject();
    if !cache_meta.is_null() {
        cJSON_AddNumberToObject(
            cache_meta,
            b"lastUpdate\0".as_ptr() as *const ::core::ffi::c_char,
            cache.last_update as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            cache_meta,
            b"ttlSeconds\0".as_ptr() as *const ::core::ffi::c_char,
            cache.ttl_seconds as ::core::ffi::c_double,
        );
        cJSON_AddBoolToObject(
            cache_meta,
            b"isCached\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddItemToObject(
            json_obj,
            b"streamStorageCache\0".as_ptr() as *const ::core::ffi::c_char,
            cache_meta,
        );
    }
    free(stream_info as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn invalidate_stream_storage_cache() -> ::core::ffi::c_int {
    if cache.initialized == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage manager streams cache not initialized\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut cache.mutex);
    if !cache.stream_info.is_null() {
        free(cache.stream_info as *mut ::core::ffi::c_void);
        cache.stream_info = ::core::ptr::null_mut::<stream_storage_info_t>();
    }
    cache.stream_count = 0 as ::core::ffi::c_int;
    cache.last_update = 0 as time_t;
    pthread_mutex_unlock(&raw mut cache.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Storage manager streams cache invalidated\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn force_refresh_cache() -> ::core::ffi::c_int {
    return refresh_cache();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_stream_storage_cache_add_recording(
    mut stream_name: *const ::core::ffi::c_char,
    mut size_bytes: uint64_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for update_stream_storage_cache_add_recording\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if cache.initialized == 0
        || cache.stream_info.is_null()
        || cache.stream_count <= 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut cache.mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < cache.stream_count {
        if strcmp(
            &raw mut (*cache.stream_info.offset(i as isize)).name as *mut ::core::ffi::c_char,
            stream_name,
        ) == 0 as ::core::ffi::c_int
        {
            (*cache.stream_info.offset(i as isize)).size_bytes =
                (*cache.stream_info.offset(i as isize))
                    .size_bytes
                    .wrapping_add(size_bytes as ::core::ffi::c_ulong);
            (*cache.stream_info.offset(i as isize)).recording_count += 1;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cache incremented for stream '%s': +%llu bytes, count now %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                size_bytes as ::core::ffi::c_ulonglong,
                (*cache.stream_info.offset(i as isize)).recording_count,
            );
            pthread_mutex_unlock(&raw mut cache.mutex);
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    pthread_mutex_unlock(&raw mut cache.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream '%s' not found in cache for add_recording (will sync on next refresh)\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_stream_storage_cache_remove_recording(
    mut stream_name: *const ::core::ffi::c_char,
    mut size_bytes: uint64_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameter for update_stream_storage_cache_remove_recording\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if cache.initialized == 0
        || cache.stream_info.is_null()
        || cache.stream_count <= 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut cache.mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < cache.stream_count {
        if strcmp(
            &raw mut (*cache.stream_info.offset(i as isize)).name as *mut ::core::ffi::c_char,
            stream_name,
        ) == 0 as ::core::ffi::c_int
        {
            if (*cache.stream_info.offset(i as isize)).size_bytes
                >= size_bytes as ::core::ffi::c_ulong
            {
                (*cache.stream_info.offset(i as isize)).size_bytes =
                    (*cache.stream_info.offset(i as isize))
                        .size_bytes
                        .wrapping_sub(size_bytes as ::core::ffi::c_ulong);
            } else {
                (*cache.stream_info.offset(i as isize)).size_bytes = 0 as ::core::ffi::c_ulong;
            }
            if (*cache.stream_info.offset(i as isize)).recording_count > 0 as ::core::ffi::c_int {
                (*cache.stream_info.offset(i as isize)).recording_count -= 1;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cache decremented for stream '%s': -%llu bytes, count now %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                size_bytes as ::core::ffi::c_ulonglong,
                (*cache.stream_info.offset(i as isize)).recording_count,
            );
            pthread_mutex_unlock(&raw mut cache.mutex);
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    pthread_mutex_unlock(&raw mut cache.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stream '%s' not found in cache for remove_recording (will sync on next refresh)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
