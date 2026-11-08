use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type mbedtls_md_info_t;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_null(_: *mut sqlite3_stmt, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int)
    -> sqlite3_int64;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    unsafe fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    unsafe fn sqlite3_db_mutex(_: *mut sqlite3) -> *mut sqlite3_mutex;
    unsafe fn inet_pton(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn inet_ntop(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_void,
        __buf: *mut ::core::ffi::c_char,
        __len: socklen_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn totp_match_step(
        secret: *const ::core::ffi::c_char,
        code: *const ::core::ffi::c_char,
        now: time_t,
        matched_step: *mut int64_t,
    ) -> ::core::ffi::c_int;
    static mut g_config: config_t;
    unsafe fn db_authorization_migrate_legacy_users(
        migrated_count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
    unsafe fn cached_column_exists(
        table_name: *const ::core::ffi::c_char,
        column_name: *const ::core::ffi::c_char,
    ) -> bool;
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
    unsafe fn safe_strcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn trim_ascii_whitespace(value: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn mbedtls_sha256(
        input: *const ::core::ffi::c_uchar,
        ilen: size_t,
        output: *mut ::core::ffi::c_uchar,
        is224: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_md_info_from_type(md_type: mbedtls_md_type_t) -> *const mbedtls_md_info_t;
    unsafe fn mbedtls_md_init(ctx: *mut mbedtls_md_context_t);
    unsafe fn mbedtls_md_free(ctx: *mut mbedtls_md_context_t);
    unsafe fn mbedtls_md_setup(
        ctx: *mut mbedtls_md_context_t,
        md_info: *const mbedtls_md_info_t,
        hmac: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_entropy_init(ctx: *mut mbedtls_entropy_context);
    unsafe fn mbedtls_entropy_free(ctx: *mut mbedtls_entropy_context);
    unsafe fn mbedtls_entropy_func(
        data: *mut ::core::ffi::c_void,
        output: *mut ::core::ffi::c_uchar,
        len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ctr_drbg_init(ctx: *mut mbedtls_ctr_drbg_context);
    unsafe fn mbedtls_ctr_drbg_seed(
        ctx: *mut mbedtls_ctr_drbg_context,
        f_entropy: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_uchar,
                size_t,
            ) -> ::core::ffi::c_int,
        >,
        p_entropy: *mut ::core::ffi::c_void,
        custom: *const ::core::ffi::c_uchar,
        len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ctr_drbg_free(ctx: *mut mbedtls_ctr_drbg_context);
    unsafe fn mbedtls_ctr_drbg_random(
        p_rng: *mut ::core::ffi::c_void,
        output: *mut ::core::ffi::c_uchar,
        output_len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_pkcs5_pbkdf2_hmac(
        ctx: *mut mbedtls_md_context_t,
        password: *const ::core::ffi::c_uchar,
        plen: size_t,
        salt: *const ::core::ffi::c_uchar,
        slen: size_t,
        iteration_count: ::core::ffi::c_uint,
        key_length: uint32_t,
        output: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
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
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type socklen_t = __socklen_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct user_role_t(pub ::core::ffi::c_uint);
impl user_role_t {
    pub const USER_ROLE_ADMIN: Self = Self(0);
    pub const USER_ROLE_USER: Self = Self(1);
    pub const USER_ROLE_VIEWER: Self = Self(2);
    pub const USER_ROLE_API: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_t {
    pub id: int64_t,
    pub username: [::core::ffi::c_char; 64],
    pub email: [::core::ffi::c_char; 128],
    pub role: user_role_t,
    pub api_key: [::core::ffi::c_char; 64],
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub last_login: int64_t,
    pub is_active: bool,
    pub password_change_locked: bool,
    pub must_change_password: bool,
    pub totp_enabled: bool,
    pub allowed_login_cidrs: [::core::ffi::c_char; 1024],
    pub has_login_cidr_restriction: bool,
    pub authorization_mode: [::core::ffi::c_char; 16],
    pub authenticated_via_scoped_token: bool,
    pub api_token_uuid: [::core::ffi::c_char; 37],
    pub authentication_method: [::core::ffi::c_char; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct session_t {
    pub id: int64_t,
    pub user_id: int64_t,
    pub token: [::core::ffi::c_char; 128],
    pub created_at: int64_t,
    pub last_activity_at: int64_t,
    pub idle_expires_at: int64_t,
    pub expires_at: int64_t,
    pub ip_address: [::core::ffi::c_char; 46],
    pub user_agent: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trusted_device_t {
    pub id: int64_t,
    pub user_id: int64_t,
    pub token: [::core::ffi::c_char; 128],
    pub created_at: int64_t,
    pub last_used_at: int64_t,
    pub expires_at: int64_t,
    pub ip_address: [::core::ffi::c_char; 46],
    pub user_agent: [::core::ffi::c_char; 256],
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
pub struct mbedtls_md_context_t {
    pub private_md_info: *const mbedtls_md_info_t,
    pub private_md_ctx: *mut ::core::ffi::c_void,
    pub private_hmac_ctx: *mut ::core::ffi::c_void,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mbedtls_md_type_t(pub ::core::ffi::c_uint);
impl mbedtls_md_type_t {
    pub const MBEDTLS_MD_NONE: Self = Self(0);
    pub const MBEDTLS_MD_MD5: Self = Self(3);
    pub const MBEDTLS_MD_RIPEMD160: Self = Self(4);
    pub const MBEDTLS_MD_SHA1: Self = Self(5);
    pub const MBEDTLS_MD_SHA224: Self = Self(8);
    pub const MBEDTLS_MD_SHA256: Self = Self(9);
    pub const MBEDTLS_MD_SHA384: Self = Self(10);
    pub const MBEDTLS_MD_SHA512: Self = Self(11);
    pub const MBEDTLS_MD_SHA3_224: Self = Self(16);
    pub const MBEDTLS_MD_SHA3_256: Self = Self(17);
    pub const MBEDTLS_MD_SHA3_384: Self = Self(18);
    pub const MBEDTLS_MD_SHA3_512: Self = Self(19);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_entropy_context {
    pub private_accumulator: mbedtls_md_context_t,
    pub private_accumulator_started: ::core::ffi::c_int,
    pub private_source_count: ::core::ffi::c_int,
    pub private_source: [mbedtls_entropy_source_state; 20],
    pub private_mutex: mbedtls_threading_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_threading_mutex_t {
    pub private_mutex: pthread_mutex_t,
    pub private_state: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_entropy_source_state {
    pub private_f_source: mbedtls_entropy_f_source_ptr,
    pub private_p_source: *mut ::core::ffi::c_void,
    pub private_size: size_t,
    pub private_threshold: size_t,
    pub private_strong: ::core::ffi::c_int,
}
pub type mbedtls_entropy_f_source_ptr = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_uchar,
        size_t,
        *mut size_t,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_ctr_drbg_context {
    pub private_counter: [::core::ffi::c_uchar; 16],
    pub private_reseed_counter: ::core::ffi::c_int,
    pub private_prediction_resistance: ::core::ffi::c_int,
    pub private_entropy_len: size_t,
    pub private_reseed_interval: ::core::ffi::c_int,
    pub private_aes_ctx: mbedtls_aes_context,
    pub private_f_entropy: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_p_entropy: *mut ::core::ffi::c_void,
    pub private_mutex: mbedtls_threading_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_aes_context {
    pub private_nr: ::core::ffi::c_int,
    pub private_rk_offset: size_t,
    pub private_buf: [uint32_t; 68],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const DB_AUTH_IDENTITY_ERROR: Self = Self(-1);
    pub const DB_AUTH_IDENTITY_INVALID: Self = Self(-2);
    pub const DB_AUTH_IDENTITY_LIMITED: Self = Self(-3);
    pub const DB_AUTH_IDENTITY_CONFLICT: Self = Self(-4);
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const USER_ALLOWED_LOGIN_CIDRS_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const SALT_LENGTH: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SHA256_DIGEST_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const DEFAULT_SESSION_EXPIRY: ::core::ffi::c_int = 604800 as ::core::ffi::c_int;
pub const DEFAULT_TRUSTED_DEVICE_EXPIRY: ::core::ffi::c_int = 2592000 as ::core::ffi::c_int;
static mut role_names: [*const ::core::ffi::c_char; 4] = [
    b"admin\0".as_ptr() as *const ::core::ffi::c_char,
    b"user\0".as_ptr() as *const ::core::ffi::c_char,
    b"viewer\0".as_ptr() as *const ::core::ffi::c_char,
    b"api\0".as_ptr() as *const ::core::ffi::c_char,
];
unsafe extern "C" fn default_session_absolute_expiry_seconds() -> ::core::ffi::c_int {
    let mut seconds: int64_t = g_config.auth_absolute_timeout_hours as int64_t * 3600 as int64_t;
    if seconds <= 0 as int64_t {
        return DEFAULT_SESSION_EXPIRY;
    }
    if seconds > INT32_MAX as int64_t {
        return INT32_MAX;
    }
    return seconds as ::core::ffi::c_int;
}
unsafe extern "C" fn default_session_idle_expiry_seconds() -> ::core::ffi::c_int {
    let mut seconds: int64_t = g_config.auth_timeout_hours as int64_t * 3600 as int64_t;
    if seconds <= 0 as int64_t {
        return DEFAULT_SESSION_EXPIRY;
    }
    if seconds > INT32_MAX as int64_t {
        return INT32_MAX;
    }
    return seconds as ::core::ffi::c_int;
}
unsafe extern "C" fn default_trusted_device_expiry_seconds() -> ::core::ffi::c_int {
    let mut seconds: int64_t = g_config.trusted_device_days as int64_t * 86400 as int64_t;
    if seconds <= 0 as int64_t {
        return DEFAULT_TRUSTED_DEVICE_EXPIRY;
    }
    if seconds > INT32_MAX as int64_t {
        return INT32_MAX;
    }
    return seconds as ::core::ffi::c_int;
}
unsafe extern "C" fn should_refresh_session_tracking(
    mut now: time_t,
    mut last_activity_at: time_t,
    mut idle_expires_at: time_t,
) -> bool {
    let refresh_interval: time_t = 60 as time_t;
    if last_activity_at <= 0 as time_t {
        return r#true != 0;
    }
    if now - last_activity_at >= refresh_interval {
        return r#true != 0;
    }
    if idle_expires_at <= now + refresh_interval {
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn tracking_value_differs(
    mut stored_value: *const ::core::ffi::c_char,
    mut current_value: *const ::core::ffi::c_char,
) -> bool {
    if current_value.is_null() {
        return r#false != 0;
    }
    let mut normalized_stored: *const ::core::ffi::c_char = if !stored_value.is_null() {
        stored_value
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    };
    return strcmp(normalized_stored, current_value) != 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_cidr_entry(
    mut cidr: *const ::core::ffi::c_char,
    mut family: *mut ::core::ffi::c_int,
    mut network: *mut ::core::ffi::c_uchar,
    mut prefix_len: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if cidr.is_null() || family.is_null() || network.is_null() || prefix_len.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut entry: [::core::ffi::c_char; 51] = [0; 51];
    if strlen(cidr) >= ::core::mem::size_of::<[::core::ffi::c_char; 51]>() {
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut entry as *mut ::core::ffi::c_char,
        cidr,
        ::core::mem::size_of::<[::core::ffi::c_char; 51]>(),
        0 as size_t,
    );
    let mut trimmed: *mut ::core::ffi::c_char =
        trim_ascii_whitespace(&raw mut entry as *mut ::core::ffi::c_char);
    if trimmed.is_null()
        || *trimmed.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut slash: *mut ::core::ffi::c_char = strrchr(trimmed, '/' as ::core::ffi::c_int);
    let mut address: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut prefix: ::core::ffi::c_long = -1 as ::core::ffi::c_long;
    if !slash.is_null() {
        if slash == trimmed
            || *slash.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        *slash = '\0' as ::core::ffi::c_char;
        address = trim_ascii_whitespace(trimmed);
        let mut prefix_text: *mut ::core::ffi::c_char =
            trim_ascii_whitespace(slash.offset(1 as ::core::ffi::c_int as isize));
        if address.is_null()
            || *address.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || prefix_text.is_null()
            || *prefix_text.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        prefix = strtol(prefix_text, &raw mut endptr, 10 as ::core::ffi::c_int);
        if endptr == prefix_text || *endptr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
    } else {
        address = trim_ascii_whitespace(trimmed);
        if address.is_null()
            || *address.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
    }
    if inet_pton(AF_INET, address, network as *mut ::core::ffi::c_void) == 1 as ::core::ffi::c_int {
        if slash.is_null() {
            prefix = 32 as ::core::ffi::c_long;
        } else if prefix < 0 as ::core::ffi::c_long || prefix > 32 as ::core::ffi::c_long {
            return -1 as ::core::ffi::c_int;
        }
        *family = AF_INET;
        *prefix_len = prefix as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if inet_pton(AF_INET6, address, network as *mut ::core::ffi::c_void) == 1 as ::core::ffi::c_int
    {
        if slash.is_null() {
            prefix = 128 as ::core::ffi::c_long;
        } else if prefix < 0 as ::core::ffi::c_long || prefix > 128 as ::core::ffi::c_long {
            return -1 as ::core::ffi::c_int;
        }
        *family = AF_INET6;
        *prefix_len = prefix as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn normalize_allowed_login_cidrs(
    mut allowed_login_cidrs: *const ::core::ffi::c_char,
    mut normalized: *mut ::core::ffi::c_char,
    mut normalized_size: size_t,
    mut has_entries: *mut bool,
) -> ::core::ffi::c_int {
    if normalized.is_null() || normalized_size == 0 as size_t || has_entries.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *normalized.offset(0isize) = '\0' as ::core::ffi::c_char;
    *has_entries = r#false != 0;
    if allowed_login_cidrs.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if strlen(allowed_login_cidrs) >= USER_ALLOWED_LOGIN_CIDRS_MAX as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut input: [::core::ffi::c_char; 1024] = [0; 1024];
    safe_strcpy(
        &raw mut input as *mut ::core::ffi::c_char,
        allowed_login_cidrs,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        0 as size_t,
    );
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token: *mut ::core::ffi::c_char = strtok_r(
        &raw mut input as *mut ::core::ffi::c_char,
        b",\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !token.is_null() {
        let mut trimmed: *mut ::core::ffi::c_char = trim_ascii_whitespace(token);
        if !(trimmed.is_null()
            || *trimmed.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int)
        {
            let mut family: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut prefix_len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut network: [::core::ffi::c_uchar; 16] = [
                0 as ::core::ffi::c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
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
            if parse_cidr_entry(
                trimmed,
                &raw mut family,
                &raw mut network as *mut ::core::ffi::c_uchar,
                &raw mut prefix_len,
            ) != 0 as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            let mut address_text: [::core::ffi::c_char; 46] = [
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
            ];
            if inet_ntop(
                family,
                &raw mut network as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                &raw mut address_text as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 46]>() as socklen_t,
            )
            .is_null()
            {
                return -1 as ::core::ffi::c_int;
            }
            let mut normalized_entry: [::core::ffi::c_char; 51] = [
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
            ];
            let mut written: ::core::ffi::c_int = snprintf(
                &raw mut normalized_entry as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 51]>(),
                b"%s/%d\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut address_text as *mut ::core::ffi::c_char,
                prefix_len,
            );
            if written < 0 as ::core::ffi::c_int
                || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 51]>()
            {
                return -1 as ::core::ffi::c_int;
            }
            let mut current_len: size_t = strlen(normalized);
            let mut token_len: size_t = written as size_t;
            let mut separator_len: size_t = (if *has_entries as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as size_t;
            if current_len
                .wrapping_add(separator_len)
                .wrapping_add(token_len)
                .wrapping_add(1 as size_t)
                > normalized_size
            {
                return -1 as ::core::ffi::c_int;
            }
            if *has_entries {
                let c2rust_fresh4 = current_len;
                current_len = current_len.wrapping_add(1);
                *normalized.offset(c2rust_fresh4 as isize) = '\n' as ::core::ffi::c_char;
                *normalized.offset(current_len as isize) = '\0' as ::core::ffi::c_char;
            }
            memcpy(
                normalized.offset(current_len as isize) as *mut ::core::ffi::c_void,
                &raw mut normalized_entry as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                token_len.wrapping_add(1 as size_t),
            );
            *has_entries = r#true != 0;
        }
        token = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ip_matches_cidr(
    mut client_ip: *const ::core::ffi::c_char,
    mut cidr: *const ::core::ffi::c_char,
) -> bool {
    if client_ip.is_null() || cidr.is_null() {
        return r#false != 0;
    }
    let mut ip: [::core::ffi::c_uchar; 16] = [
        0 as ::core::ffi::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut ip_family: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if inet_pton(
        AF_INET,
        client_ip,
        &raw mut ip as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
    ) == 1 as ::core::ffi::c_int
    {
        ip_family = AF_INET;
    } else if inet_pton(
        AF_INET6,
        client_ip,
        &raw mut ip as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
    ) == 1 as ::core::ffi::c_int
    {
        ip_family = AF_INET6;
    } else {
        return r#false != 0;
    }
    let mut network: [::core::ffi::c_uchar; 16] = [
        0 as ::core::ffi::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut cidr_family: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut prefix_len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if parse_cidr_entry(
        cidr,
        &raw mut cidr_family,
        &raw mut network as *mut ::core::ffi::c_uchar,
        &raw mut prefix_len,
    ) != 0 as ::core::ffi::c_int
        || cidr_family != ip_family
    {
        return r#false != 0;
    }
    let mut full_bytes: ::core::ffi::c_int = prefix_len / 8 as ::core::ffi::c_int;
    let mut remaining_bits: ::core::ffi::c_int = prefix_len % 8 as ::core::ffi::c_int;
    if full_bytes > 0 as ::core::ffi::c_int
        && memcmp(
            &raw mut ip as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            &raw mut network as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            full_bytes as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if remaining_bits == 0 as ::core::ffi::c_int {
        return r#true != 0;
    }
    let mut mask: ::core::ffi::c_uchar = ((0xff as ::core::ffi::c_uint)
        << 8 as ::core::ffi::c_int - remaining_bits)
        as ::core::ffi::c_uchar;
    return ip[full_bytes as usize] as ::core::ffi::c_int & mask as ::core::ffi::c_int
        == network[full_bytes as usize] as ::core::ffi::c_int & mask as ::core::ffi::c_int;
}
unsafe extern "C" fn prepare_user_lookup_stmt(
    mut db: *mut sqlite3,
    mut where_clause: *const ::core::ffi::c_char,
    mut stmt: *mut *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    if db.is_null() || where_clause.is_null() || stmt.is_null() {
        return SQLITE_MISUSE;
    }
    let mut has_totp: bool = cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"totp_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_allowed_login_cidrs: bool = cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"allowed_login_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_authorization_mode: bool = cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"authorization_mode\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_must_change_password: bool = cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"must_change_password\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut sql: [::core::ffi::c_char; 768] = [0; 768];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 768]>(),
        b"SELECT id, username, email, role, api_key, created_at, updated_at, last_login, is_active, password_change_locked, %s, %s, %s, %s FROM users %s;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if has_must_change_password as ::core::ffi::c_int != 0 {
            b"must_change_password\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"0\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_totp as ::core::ffi::c_int != 0 {
            b"totp_enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"0\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_allowed_login_cidrs as ::core::ffi::c_int != 0 {
            b"allowed_login_cidrs\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"NULL\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_authorization_mode as ::core::ffi::c_int != 0 {
            b"authorization_mode\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"'legacy'\0".as_ptr() as *const ::core::ffi::c_char
        },
        where_clause,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 768]>()
    {
        return SQLITE_TOOBIG;
    }
    return sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn populate_user_from_stmt(mut stmt: *mut sqlite3_stmt, mut user: *mut user_t) {
    memset(
        user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    (*user).id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    safe_strcpy(
        &raw mut (*user).username as *mut ::core::ffi::c_char,
        sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    let mut email: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if !email.is_null() {
        safe_strcpy(
            &raw mut (*user).email as *mut ::core::ffi::c_char,
            email,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    }
    (*user).role =
        user_role_t(sqlite3_column_int(stmt, 3 as ::core::ffi::c_int) as ::core::ffi::c_uint);
    let mut api_key: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if !api_key.is_null() {
        safe_strcpy(
            &raw mut (*user).api_key as *mut ::core::ffi::c_char,
            api_key,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    (*user).created_at = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as int64_t;
    (*user).updated_at = sqlite3_column_int64(stmt, 6 as ::core::ffi::c_int) as int64_t;
    (*user).last_login = sqlite3_column_int64(stmt, 7 as ::core::ffi::c_int) as int64_t;
    (*user).is_active =
        sqlite3_column_int(stmt, 8 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*user).password_change_locked =
        sqlite3_column_int(stmt, 9 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*user).must_change_password =
        sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*user).totp_enabled =
        sqlite3_column_int(stmt, 11 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    let mut allowed_login_cidrs: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 12 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if !allowed_login_cidrs.is_null()
        && *allowed_login_cidrs.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut (*user).allowed_login_cidrs as *mut ::core::ffi::c_char,
            allowed_login_cidrs,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            0 as size_t,
        );
        (*user).has_login_cidr_restriction = r#true != 0;
    }
    let mut authorization_mode: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 13 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    safe_strcpy(
        &raw mut (*user).authorization_mode as *mut ::core::ffi::c_char,
        if !authorization_mode.is_null() {
            authorization_mode
        } else {
            b"legacy\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
}
unsafe extern "C" fn generate_random_string(
    mut buffer: *mut ::core::ffi::c_char,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let charset: [::core::ffi::c_char; 63] =
        ::core::mem::transmute::<[u8; 63], [::core::ffi::c_char; 63]>(
            *b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\0",
        );
    let mut entropy: mbedtls_entropy_context = mbedtls_entropy_context {
        private_accumulator: mbedtls_md_context_t {
            private_md_info: ::core::ptr::null::<mbedtls_md_info_t>(),
            private_md_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            private_hmac_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        private_accumulator_started: 0,
        private_source_count: 0,
        private_source: [mbedtls_entropy_source_state {
            private_f_source: None,
            private_p_source: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            private_size: 0,
            private_threshold: 0,
            private_strong: 0,
        }; 20],
        private_mutex: mbedtls_threading_mutex_t {
            private_mutex: pthread_mutex_t {
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
            private_state: 0,
        },
    };
    let mut ctr_drbg: mbedtls_ctr_drbg_context = mbedtls_ctr_drbg_context {
        private_counter: [0; 16],
        private_reseed_counter: 0,
        private_prediction_resistance: 0,
        private_entropy_len: 0,
        private_reseed_interval: 0,
        private_aes_ctx: mbedtls_aes_context {
            private_nr: 0,
            private_rk_offset: 0,
            private_buf: [0; 68],
        },
        private_f_entropy: None,
        private_p_entropy: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        private_mutex: mbedtls_threading_mutex_t {
            private_mutex: pthread_mutex_t {
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
            private_state: 0,
        },
    };
    mbedtls_entropy_init(&raw mut entropy);
    mbedtls_ctr_drbg_init(&raw mut ctr_drbg);
    if mbedtls_ctr_drbg_seed(
        &raw mut ctr_drbg,
        Some(
            mbedtls_entropy_func
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut entropy as *mut ::core::ffi::c_void,
        b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_uchar,
        8 as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to seed random number generator\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mbedtls_ctr_drbg_free(&raw mut ctr_drbg);
        mbedtls_entropy_free(&raw mut entropy);
        return -1 as ::core::ffi::c_int;
    }
    let vla = length as usize;
    let mut random_bytes: Vec<::core::ffi::c_uchar> = ::std::vec::from_elem(0, vla);
    if mbedtls_ctr_drbg_random(
        &raw mut ctr_drbg as *mut ::core::ffi::c_void,
        random_bytes.as_mut_ptr(),
        length,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to generate random bytes\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mbedtls_ctr_drbg_free(&raw mut ctr_drbg);
        mbedtls_entropy_free(&raw mut entropy);
        return -1 as ::core::ffi::c_int;
    }
    let mut i: size_t = 0 as size_t;
    while i < length {
        *buffer.offset(i as isize) =
            charset[(*random_bytes.as_mut_ptr().offset(i as isize) as usize).wrapping_rem(
                ::core::mem::size_of::<[::core::ffi::c_char; 63]>().wrapping_sub(1usize),
            )];
        i = i.wrapping_add(1);
    }
    *buffer.offset(length as isize) = '\0' as ::core::ffi::c_char;
    mbedtls_ctr_drbg_free(&raw mut ctr_drbg);
    mbedtls_entropy_free(&raw mut entropy);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn hash_password(
    mut password: *const ::core::ffi::c_char,
    mut salt: *const ::core::ffi::c_uchar,
    mut salt_length: size_t,
    mut hash: *mut ::core::ffi::c_uchar,
    mut hash_length: size_t,
) -> ::core::ffi::c_int {
    let mut ctx: mbedtls_md_context_t = mbedtls_md_context_t {
        private_md_info: ::core::ptr::null::<mbedtls_md_info_t>(),
        private_md_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        private_hmac_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut md_info: *const mbedtls_md_info_t = ::core::ptr::null::<mbedtls_md_info_t>();
    mbedtls_md_init(&raw mut ctx);
    md_info = mbedtls_md_info_from_type(mbedtls_md_type_t::MBEDTLS_MD_SHA256);
    if md_info.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get SHA-256 info\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if mbedtls_md_setup(&raw mut ctx, md_info, 1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to set up message digest context\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mbedtls_md_free(&raw mut ctx);
        return -1 as ::core::ffi::c_int;
    }
    if mbedtls_pkcs5_pbkdf2_hmac(
        &raw mut ctx,
        password as *const ::core::ffi::c_uchar,
        strlen(password),
        salt,
        salt_length,
        10000 as ::core::ffi::c_uint,
        hash_length as uint32_t,
        hash,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to hash password\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mbedtls_md_free(&raw mut ctx);
        return -1 as ::core::ffi::c_int;
    }
    mbedtls_md_free(&raw mut ctx);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn bin_to_hex(
    mut data: *const ::core::ffi::c_uchar,
    mut data_length: size_t,
    mut hex: *mut ::core::ffi::c_char,
    mut hex_length: size_t,
) -> ::core::ffi::c_int {
    if hex_length
        < data_length
            .wrapping_mul(2 as size_t)
            .wrapping_add(1 as size_t)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Hex buffer too small\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut i: size_t = 0 as size_t;
    while i < data_length {
        snprintf(
            hex.offset(i.wrapping_mul(2 as size_t) as isize),
            3 as size_t,
            b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
            *data.offset(i as isize) as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn hash_token_identifier(
    mut token: *const ::core::ffi::c_char,
    mut token_hash: *mut ::core::ffi::c_char,
    mut token_hash_size: size_t,
) -> ::core::ffi::c_int {
    if token.is_null() || *token.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if token_hash.is_null()
        || token_hash_size
            < (SHA256_DIGEST_LENGTH * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Token hash buffer too small\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut digest: [::core::ffi::c_uchar; 32] = [0; 32];
    mbedtls_sha256(
        token as *const ::core::ffi::c_uchar,
        strlen(token),
        &raw mut digest as *mut ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int,
    );
    return bin_to_hex(
        &raw mut digest as *mut ::core::ffi::c_uchar,
        SHA256_DIGEST_LENGTH as size_t,
        token_hash,
        token_hash_size,
    );
}
unsafe extern "C" fn hex_to_bin(
    mut hex: *const ::core::ffi::c_char,
    mut data: *mut ::core::ffi::c_uchar,
    mut data_length: size_t,
) -> ::core::ffi::c_int {
    let mut hex_length: size_t = strlen(hex);
    if hex_length.wrapping_div(2 as size_t) > data_length {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Data buffer too small\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut i: size_t = 0 as size_t;
    while i < hex_length {
        let byte: [::core::ffi::c_char; 3] = [
            *hex.offset(i as isize),
            *hex.offset(i.wrapping_add(1 as size_t) as isize),
            '\0' as ::core::ffi::c_char,
        ];
        *data.offset(i.wrapping_div(2 as size_t) as isize) = strtol(
            &raw const byte as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            16 as ::core::ffi::c_int,
        ) as ::core::ffi::c_uchar;
        i = i.wrapping_add(2 as size_t);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn bootstrap_admin_locked() -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Initializing authentication system\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    let mut rc: ::core::ffi::c_int = db_auth_get_user_by_username(
        b"admin\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut user,
    );
    if rc == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Default admin user already exists\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut initial_password: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut used_config_password: bool =
        g_config.web_password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    if used_config_password {
        initial_password = &raw mut g_config.web_password as *mut ::core::ffi::c_char;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Creating default admin user with password from config file\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        initial_password = b"admin\0".as_ptr() as *const ::core::ffi::c_char;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Creating default admin user with default password\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null()
        || sqlite3_exec(
            db,
            b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to begin default administrator creation transaction\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut admin_user_id: int64_t = 0 as int64_t;
    rc = db_auth_create_user(
        b"admin\0".as_ptr() as *const ::core::ffi::c_char,
        initial_password,
        ::core::ptr::null::<::core::ffi::c_char>(),
        user_role_t::USER_ROLE_ADMIN,
        r#true != 0,
        &raw mut admin_user_id,
    );
    if rc != 0 as ::core::ffi::c_int {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create default admin user\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !used_config_password {
        let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare_v2(
            db,
            b"UPDATE users SET must_change_password = 1, updated_at = ? WHERE id = ?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to prepare default password-change requirement: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_bind_int64(
            stmt,
            1 as ::core::ffi::c_int,
            time(::core::ptr::null_mut::<time_t>()) as sqlite3_int64,
        );
        sqlite3_bind_int64(
            stmt,
            2 as ::core::ffi::c_int,
            admin_user_id as sqlite3_int64,
        );
        rc = sqlite3_step(stmt);
        sqlite3_finalize(stmt);
        if rc != SQLITE_DONE {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to require a default administrator password change: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if sqlite3_exec(
        db,
        b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to commit default administrator creation: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"********************************************************\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"***    Default admin user created successfully       ***\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"***    Username: admin                               ***\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if used_config_password {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"***    Password: (from config file)                  ***\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"***    Manage users from Settings -> Users           ***\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"***    Password: admin                               ***\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"***    Password change required on first login      ***\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"********************************************************\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_init() -> ::core::ffi::c_int {
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if get_db_handle().is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut result: ::core::ffi::c_int = bootstrap_admin_locked();
    pthread_mutex_unlock(mutex);
    return if result == 0 as ::core::ffi::c_int {
        db_authorization_migrate_legacy_users(::core::ptr::null_mut::<::core::ffi::c_int>())
    } else {
        result
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_create_user(
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut email: *const ::core::ffi::c_char,
    mut role: user_role_t,
    mut is_active: bool,
    mut user_id: *mut int64_t,
) -> ::core::ffi::c_int {
    if username.is_null() || password.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Username and password are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT id FROM users WHERE username = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        username,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if sqlite3_step(stmt) == SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Username already exists: %s\0".as_ptr() as *const ::core::ffi::c_char,
            username,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    let mut salt: [::core::ffi::c_uchar; 16] = [0; 16];
    let mut entropy: mbedtls_entropy_context = mbedtls_entropy_context {
        private_accumulator: mbedtls_md_context_t {
            private_md_info: ::core::ptr::null::<mbedtls_md_info_t>(),
            private_md_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            private_hmac_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        private_accumulator_started: 0,
        private_source_count: 0,
        private_source: [mbedtls_entropy_source_state {
            private_f_source: None,
            private_p_source: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            private_size: 0,
            private_threshold: 0,
            private_strong: 0,
        }; 20],
        private_mutex: mbedtls_threading_mutex_t {
            private_mutex: pthread_mutex_t {
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
            private_state: 0,
        },
    };
    let mut ctr_drbg: mbedtls_ctr_drbg_context = mbedtls_ctr_drbg_context {
        private_counter: [0; 16],
        private_reseed_counter: 0,
        private_prediction_resistance: 0,
        private_entropy_len: 0,
        private_reseed_interval: 0,
        private_aes_ctx: mbedtls_aes_context {
            private_nr: 0,
            private_rk_offset: 0,
            private_buf: [0; 68],
        },
        private_f_entropy: None,
        private_p_entropy: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        private_mutex: mbedtls_threading_mutex_t {
            private_mutex: pthread_mutex_t {
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
            private_state: 0,
        },
    };
    mbedtls_entropy_init(&raw mut entropy);
    mbedtls_ctr_drbg_init(&raw mut ctr_drbg);
    if mbedtls_ctr_drbg_seed(
        &raw mut ctr_drbg,
        Some(
            mbedtls_entropy_func
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut entropy as *mut ::core::ffi::c_void,
        b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_uchar,
        8 as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to seed random number generator\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mbedtls_ctr_drbg_free(&raw mut ctr_drbg);
        mbedtls_entropy_free(&raw mut entropy);
        return -1 as ::core::ffi::c_int;
    }
    if mbedtls_ctr_drbg_random(
        &raw mut ctr_drbg as *mut ::core::ffi::c_void,
        &raw mut salt as *mut ::core::ffi::c_uchar,
        SALT_LENGTH as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to generate salt\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mbedtls_ctr_drbg_free(&raw mut ctr_drbg);
        mbedtls_entropy_free(&raw mut entropy);
        return -1 as ::core::ffi::c_int;
    }
    mbedtls_ctr_drbg_free(&raw mut ctr_drbg);
    mbedtls_entropy_free(&raw mut entropy);
    let mut salt_hex: [::core::ffi::c_char; 33] = [0; 33];
    if bin_to_hex(
        &raw mut salt as *mut ::core::ffi::c_uchar,
        SALT_LENGTH as size_t,
        &raw mut salt_hex as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 33]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to convert salt to hex\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut hash: [::core::ffi::c_uchar; 32] = [0; 32];
    if hash_password(
        password,
        &raw mut salt as *mut ::core::ffi::c_uchar,
        SALT_LENGTH as size_t,
        &raw mut hash as *mut ::core::ffi::c_uchar,
        SHA256_DIGEST_LENGTH as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to hash password\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut hash_hex: [::core::ffi::c_char; 65] = [0; 65];
    if bin_to_hex(
        &raw mut hash as *mut ::core::ffi::c_uchar,
        SHA256_DIGEST_LENGTH as size_t,
        &raw mut hash_hex as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to convert hash to hex\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    rc = sqlite3_prepare_v2(
        db,
        b"INSERT INTO users (username, password_hash, salt, role, email, created_at, updated_at, is_active) VALUES (?, ?, ?, ?, ?, ?, ?, ?);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        username,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw mut hash_hex as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        3 as ::core::ffi::c_int,
        &raw mut salt_hex as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(stmt, 4 as ::core::ffi::c_int, role.0 as ::core::ffi::c_int);
    sqlite3_bind_text(
        stmt,
        5 as ::core::ffi::c_int,
        if !email.is_null() {
            email
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 6 as ::core::ffi::c_int, now as sqlite3_int64);
    sqlite3_bind_int64(stmt, 7 as ::core::ffi::c_int, now as sqlite3_int64);
    sqlite3_bind_int(
        stmt,
        8 as ::core::ffi::c_int,
        if is_active as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to insert user: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut id: int64_t = sqlite3_last_insert_rowid(db) as int64_t;
    if !user_id.is_null() {
        *user_id = id;
    }
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"User created successfully: %s (ID: %lld)\0".as_ptr() as *const ::core::ffi::c_char,
        username,
        id as ::core::ffi::c_longlong,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_update_user(
    mut user_id: int64_t,
    mut username: *const ::core::ffi::c_char,
    mut email: *const ::core::ffi::c_char,
    mut role: ::core::ffi::c_int,
    mut is_active: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT id FROM users WHERE id = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    if sqlite3_step(stmt) != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"User not found: %lld\0".as_ptr() as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    if !username.is_null() {
        rc = sqlite3_prepare_v2(
            db,
            b"SELECT id FROM users WHERE username = ? AND id != ?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to prepare username uniqueness check: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            username,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, user_id as sqlite3_int64);
        if sqlite3_step(stmt) == SQLITE_ROW {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cannot update user %lld: username '%s' already exists\0".as_ptr()
                    as *const ::core::ffi::c_char,
                user_id as ::core::ffi::c_longlong,
                username,
            );
            sqlite3_finalize(stmt);
            return -2 as ::core::ffi::c_int;
        }
        sqlite3_finalize(stmt);
    }
    let mut query: [::core::ffi::c_char; 512] = ::core::mem::transmute::<
        [u8; 512],
        [::core::ffi::c_char; 512],
    >(
        *b"UPDATE users SET updated_at = ?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    if !username.is_null() {
        safe_strcat(
            &raw mut query as *mut ::core::ffi::c_char,
            b", username = ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        );
    }
    if !email.is_null() {
        safe_strcat(
            &raw mut query as *mut ::core::ffi::c_char,
            b", email = ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        );
    }
    if role >= 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut query as *mut ::core::ffi::c_char,
            b", role = ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        );
    }
    if is_active >= 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut query as *mut ::core::ffi::c_char,
            b", is_active = ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        );
    }
    safe_strcat(
        &raw mut query as *mut ::core::ffi::c_char,
        b" WHERE id = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    );
    rc = sqlite3_prepare_v2(
        db,
        &raw mut query as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, now as sqlite3_int64);
    let mut param_index: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    if !username.is_null() {
        let c2rust_fresh0 = param_index;
        param_index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh0,
            username,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
    if !email.is_null() {
        let c2rust_fresh1 = param_index;
        param_index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh1,
            email,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
    if role >= 0 as ::core::ffi::c_int {
        let c2rust_fresh2 = param_index;
        param_index += 1;
        sqlite3_bind_int(stmt, c2rust_fresh2, role);
    }
    if is_active >= 0 as ::core::ffi::c_int {
        let c2rust_fresh3 = param_index;
        param_index += 1;
        sqlite3_bind_int(
            stmt,
            c2rust_fresh3,
            if is_active != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
    }
    sqlite3_bind_int64(stmt, param_index, user_id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update user: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"User updated successfully: %lld\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn revoke_credentials_locked(
    mut db: *mut sqlite3,
    mut user_id: int64_t,
    mut keep_session: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut statements: [*const ::core::ffi::c_char; 2] = [
        b"DELETE FROM sessions WHERE user_id=?1 AND (?2 IS NULL OR token<>?2 OR mfa_pending=1);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"DELETE FROM trusted_devices WHERE user_id=?1;\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if sqlite3_prepare_v2(
            db,
            statements[i],
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) != SQLITE_OK
        {
            return -1 as ::core::ffi::c_int;
        }
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
        if i == 0 as size_t {
            if !keep_session.is_null() && *keep_session as ::core::ffi::c_int != 0 {
                sqlite3_bind_text(
                    stmt,
                    2 as ::core::ffi::c_int,
                    keep_session,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            } else {
                sqlite3_bind_null(stmt, 2 as ::core::ffi::c_int);
            }
        }
        let mut rc: ::core::ffi::c_int = sqlite3_step(stmt);
        sqlite3_finalize(stmt);
        if rc != SQLITE_DONE {
            return -1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_update_user_account(
    mut user_id: int64_t,
    mut username: *const ::core::ffi::c_char,
    mut email: *const ::core::ffi::c_char,
    mut role: ::core::ffi::c_int,
    mut is_active: ::core::ffi::c_int,
    mut password: *const ::core::ffi::c_char,
    mut set_login_cidrs: bool,
    mut allowed_login_cidrs: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut normalized: [::core::ffi::c_char; 1024] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut has_entries: bool = r#false != 0;
    if set_login_cidrs as ::core::ffi::c_int != 0
        && (!cached_column_exists(
            b"users\0".as_ptr() as *const ::core::ffi::c_char,
            b"allowed_login_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
        ) || normalize_allowed_login_cidrs(
            allowed_login_cidrs,
            &raw mut normalized as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            &raw mut has_entries,
        ) != 0 as ::core::ffi::c_int)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    pthread_mutex_lock(mutex);
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut connection_mutex: *mut sqlite3_mutex = sqlite3_db_mutex(db);
    sqlite3_mutex_enter(connection_mutex);
    let mut rc: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        rc = db_auth_update_user(user_id, username, email, role, is_active);
        if rc == 0 as ::core::ffi::c_int && set_login_cidrs as ::core::ffi::c_int != 0 {
            let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            rc = -1 as ::core::ffi::c_int;
            if sqlite3_prepare_v2(
                db,
                b"UPDATE users SET allowed_login_cidrs = ? WHERE id = ?;\0".as_ptr()
                    as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                &raw mut stmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ) == SQLITE_OK
            {
                if has_entries {
                    sqlite3_bind_text(
                        stmt,
                        1 as ::core::ffi::c_int,
                        &raw mut normalized as *mut ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                } else {
                    sqlite3_bind_null(stmt, 1 as ::core::ffi::c_int);
                }
                sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, user_id as sqlite3_int64);
                if sqlite3_step(stmt) == SQLITE_DONE {
                    rc = 0 as ::core::ffi::c_int;
                }
            }
            sqlite3_finalize(stmt);
        }
        if rc == 0 as ::core::ffi::c_int && !password.is_null() {
            let mut password_rc: ::core::ffi::c_int = change_password_locked(user_id, password);
            rc = if password_rc == -2 as ::core::ffi::c_int {
                -3 as ::core::ffi::c_int
            } else if password_rc == 0 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
        if rc == 0 as ::core::ffi::c_int
            && (!password.is_null() || is_active == 0 as ::core::ffi::c_int)
        {
            rc = revoke_credentials_locked(db, user_id, ::core::ptr::null::<::core::ffi::c_char>());
        }
        if rc == 0 as ::core::ffi::c_int
            && sqlite3_exec(
                db,
                b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ) != SQLITE_OK
        {
            rc = -1 as ::core::ffi::c_int;
        }
        if rc != 0 as ::core::ffi::c_int {
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
    }
    sqlite3_mutex_leave(connection_mutex);
    pthread_mutex_unlock(mutex);
    return rc;
}
unsafe extern "C" fn change_password_locked(
    mut user_id: int64_t,
    mut new_password: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if new_password.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"New password is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT id, password_change_locked, must_change_password FROM users WHERE id = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    if sqlite3_step(stmt) != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"User not found: %lld\0".as_ptr() as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut password_locked: bool =
        sqlite3_column_int(stmt, 1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    if password_locked {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Password changes are locked for user: %lld\0".as_ptr() as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -2 as ::core::ffi::c_int;
    }
    let mut must_change_password: bool =
        sqlite3_column_int(stmt, 2 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    if must_change_password as ::core::ffi::c_int != 0
        && strcmp(
            new_password,
            b"admin\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Default password cannot satisfy required password change for user: %lld\0".as_ptr()
                as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -3 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    let mut salt: [::core::ffi::c_uchar; 16] = [0; 16];
    let mut entropy: mbedtls_entropy_context = mbedtls_entropy_context {
        private_accumulator: mbedtls_md_context_t {
            private_md_info: ::core::ptr::null::<mbedtls_md_info_t>(),
            private_md_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            private_hmac_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        private_accumulator_started: 0,
        private_source_count: 0,
        private_source: [mbedtls_entropy_source_state {
            private_f_source: None,
            private_p_source: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            private_size: 0,
            private_threshold: 0,
            private_strong: 0,
        }; 20],
        private_mutex: mbedtls_threading_mutex_t {
            private_mutex: pthread_mutex_t {
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
            private_state: 0,
        },
    };
    let mut ctr_drbg: mbedtls_ctr_drbg_context = mbedtls_ctr_drbg_context {
        private_counter: [0; 16],
        private_reseed_counter: 0,
        private_prediction_resistance: 0,
        private_entropy_len: 0,
        private_reseed_interval: 0,
        private_aes_ctx: mbedtls_aes_context {
            private_nr: 0,
            private_rk_offset: 0,
            private_buf: [0; 68],
        },
        private_f_entropy: None,
        private_p_entropy: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        private_mutex: mbedtls_threading_mutex_t {
            private_mutex: pthread_mutex_t {
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
            private_state: 0,
        },
    };
    mbedtls_entropy_init(&raw mut entropy);
    mbedtls_ctr_drbg_init(&raw mut ctr_drbg);
    if mbedtls_ctr_drbg_seed(
        &raw mut ctr_drbg,
        Some(
            mbedtls_entropy_func
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut entropy as *mut ::core::ffi::c_void,
        b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_uchar,
        8 as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to seed random number generator\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mbedtls_ctr_drbg_free(&raw mut ctr_drbg);
        mbedtls_entropy_free(&raw mut entropy);
        return -1 as ::core::ffi::c_int;
    }
    if mbedtls_ctr_drbg_random(
        &raw mut ctr_drbg as *mut ::core::ffi::c_void,
        &raw mut salt as *mut ::core::ffi::c_uchar,
        SALT_LENGTH as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to generate salt\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mbedtls_ctr_drbg_free(&raw mut ctr_drbg);
        mbedtls_entropy_free(&raw mut entropy);
        return -1 as ::core::ffi::c_int;
    }
    mbedtls_ctr_drbg_free(&raw mut ctr_drbg);
    mbedtls_entropy_free(&raw mut entropy);
    let mut salt_hex: [::core::ffi::c_char; 33] = [0; 33];
    if bin_to_hex(
        &raw mut salt as *mut ::core::ffi::c_uchar,
        SALT_LENGTH as size_t,
        &raw mut salt_hex as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 33]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to convert salt to hex\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut hash: [::core::ffi::c_uchar; 32] = [0; 32];
    if hash_password(
        new_password,
        &raw mut salt as *mut ::core::ffi::c_uchar,
        SALT_LENGTH as size_t,
        &raw mut hash as *mut ::core::ffi::c_uchar,
        SHA256_DIGEST_LENGTH as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to hash password\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut hash_hex: [::core::ffi::c_char; 65] = [0; 65];
    if bin_to_hex(
        &raw mut hash as *mut ::core::ffi::c_uchar,
        SHA256_DIGEST_LENGTH as size_t,
        &raw mut hash_hex as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to convert hash to hex\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    rc = sqlite3_prepare_v2(
        db,
        b"UPDATE users SET password_hash = ?, salt = ?, updated_at = ?, must_change_password = 0 WHERE id = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        &raw mut hash_hex as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw mut salt_hex as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, now as sqlite3_int64);
    sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, user_id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update password: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_change_password_verified(
    mut user_id: int64_t,
    mut new_password: *const ::core::ffi::c_char,
    mut old_password: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if new_password.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = lock_auth_database();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        if !old_password.is_null()
            && db_auth_verify_password(user_id, old_password) != 0 as ::core::ffi::c_int
        {
            result = -4 as ::core::ffi::c_int;
        } else {
            result = change_password_locked(user_id, new_password);
            if result == 0 as ::core::ffi::c_int {
                result = revoke_credentials_locked(
                    db,
                    user_id,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
        if result == 0 as ::core::ffi::c_int
            && sqlite3_exec(
                db,
                b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ) != SQLITE_OK
        {
            result = -1 as ::core::ffi::c_int;
        }
        if result != 0 as ::core::ffi::c_int {
            sqlite3_exec(
                db,
                b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
    }
    unlock_auth_database(db);
    if result == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Password changed and browser credentials revoked for user: %lld\0".as_ptr()
                as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_change_password(
    mut user_id: int64_t,
    mut new_password: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return db_auth_change_password_verified(
        user_id,
        new_password,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_delete_user(mut user_id: int64_t) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT id FROM users WHERE id = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    if sqlite3_step(stmt) != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"User not found: %lld\0".as_ptr() as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    rc = sqlite3_prepare_v2(
        db,
        b"DELETE FROM users WHERE id = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to delete user: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"User deleted successfully: %lld\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_get_user_by_id(
    mut user_id: int64_t,
    mut user: *mut user_t,
) -> ::core::ffi::c_int {
    if user.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"User pointer is NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = prepare_user_lookup_stmt(
        db,
        b"WHERE id = ?\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stmt,
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    if sqlite3_step(stmt) != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"User not found: %lld\0".as_ptr() as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    populate_user_from_stmt(stmt, user);
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_get_user_by_username(
    mut username: *const ::core::ffi::c_char,
    mut user: *mut user_t,
) -> ::core::ffi::c_int {
    if username.is_null() || user.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Username and user pointer are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = prepare_user_lookup_stmt(
        db,
        b"WHERE username = ?\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stmt,
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        username,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if sqlite3_step(stmt) != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"User not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            username,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    populate_user_from_stmt(stmt, user);
    sqlite3_finalize(stmt);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_get_user_by_api_key(
    mut api_key: *const ::core::ffi::c_char,
    mut user: *mut user_t,
) -> ::core::ffi::c_int {
    if api_key.is_null() || user.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"API key and user pointer are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = prepare_user_lookup_stmt(
        db,
        b"WHERE api_key = ?\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stmt,
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        api_key,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if sqlite3_step(stmt) != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"User not found for API key\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3_finalize(stmt);
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    populate_user_from_stmt(stmt, user);
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_generate_api_key(
    mut user_id: int64_t,
    mut api_key: *mut ::core::ffi::c_char,
    mut api_key_size: size_t,
) -> ::core::ffi::c_int {
    if api_key.is_null() || api_key_size < 33 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"API key buffer is too small\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT id FROM users WHERE id = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    if sqlite3_step(stmt) != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"User not found: %lld\0".as_ptr() as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    if generate_random_string(api_key, 32 as size_t) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to generate API key\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    rc = sqlite3_prepare_v2(
        db,
        b"UPDATE users SET api_key = ?, updated_at = ? WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        api_key,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, now as sqlite3_int64);
    sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, user_id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update API key: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"API key generated successfully for user: %lld\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_authenticate(
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut user_id: *mut int64_t,
) -> ::core::ffi::c_int {
    if username.is_null() || password.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Username and password are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT id, password_hash, salt, is_active FROM users WHERE username = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        username,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if sqlite3_step(stmt) != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Authentication failed: User not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            username,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut is_active: ::core::ffi::c_int = sqlite3_column_int(stmt, 3 as ::core::ffi::c_int);
    if is_active == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Authentication failed: User is inactive: %s\0".as_ptr() as *const ::core::ffi::c_char,
            username,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut hash_hex: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    let mut salt_hex: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if hash_hex.is_null() || salt_hex.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Authentication failed: Invalid password hash or salt for user: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            username,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut salt: [::core::ffi::c_uchar; 16] = [0; 16];
    if hex_to_bin(
        salt_hex,
        &raw mut salt as *mut ::core::ffi::c_uchar,
        SALT_LENGTH as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Authentication failed: Failed to convert salt from hex for user: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            username,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut hash: [::core::ffi::c_uchar; 32] = [0; 32];
    if hash_password(
        password,
        &raw mut salt as *mut ::core::ffi::c_uchar,
        SALT_LENGTH as size_t,
        &raw mut hash as *mut ::core::ffi::c_uchar,
        SHA256_DIGEST_LENGTH as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Authentication failed: Failed to hash password for user: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            username,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut computed_hash_hex: [::core::ffi::c_char; 65] = [0; 65];
    if bin_to_hex(
        &raw mut hash as *mut ::core::ffi::c_uchar,
        SHA256_DIGEST_LENGTH as size_t,
        &raw mut computed_hash_hex as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Authentication failed: Failed to convert hash to hex for user: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            username,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    if strcmp(
        hash_hex,
        &raw mut computed_hash_hex as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Authentication failed: Invalid password for user: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            username,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut id: int64_t = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    if !user_id.is_null() {
        *user_id = id;
    }
    sqlite3_finalize(stmt);
    rc = sqlite3_prepare_v2(
        db,
        b"UPDATE users SET last_login = ? WHERE id = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, now as sqlite3_int64);
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update last login time: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Authentication successful for user: %s (ID: %lld)\0".as_ptr()
            as *const ::core::ffi::c_char,
        username,
        id as ::core::ffi::c_longlong,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_verify_password(
    mut user_id: int64_t,
    mut password: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if password.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Password is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT password_hash, salt, is_active FROM users WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    if sqlite3_step(stmt) != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Password verification failed: User not found: %lld\0".as_ptr()
                as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut is_active: ::core::ffi::c_int = sqlite3_column_int(stmt, 2 as ::core::ffi::c_int);
    if is_active == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Password verification failed: User is inactive: %lld\0".as_ptr()
                as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut hash_hex: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    let mut salt_hex: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if hash_hex.is_null() || salt_hex.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Password verification failed: Invalid password hash or salt for user: %lld\0".as_ptr()
                as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut salt: [::core::ffi::c_uchar; 16] = [0; 16];
    if hex_to_bin(
        salt_hex,
        &raw mut salt as *mut ::core::ffi::c_uchar,
        SALT_LENGTH as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Password verification failed: Failed to convert salt from hex for user: %lld\0"
                .as_ptr() as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut hash: [::core::ffi::c_uchar; 32] = [0; 32];
    if hash_password(
        password,
        &raw mut salt as *mut ::core::ffi::c_uchar,
        SALT_LENGTH as size_t,
        &raw mut hash as *mut ::core::ffi::c_uchar,
        SHA256_DIGEST_LENGTH as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Password verification failed: Failed to hash password for user: %lld\0".as_ptr()
                as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut computed_hash_hex: [::core::ffi::c_char; 65] = [0; 65];
    if bin_to_hex(
        &raw mut hash as *mut ::core::ffi::c_uchar,
        SHA256_DIGEST_LENGTH as size_t,
        &raw mut computed_hash_hex as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Password verification failed: Failed to convert hash to hex for user: %lld\0".as_ptr()
                as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    if strcmp(
        hash_hex,
        &raw mut computed_hash_hex as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Password verification failed: Invalid password for user: %lld\0".as_ptr()
                as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Password verification successful for user: %lld\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_set_password_lock(
    mut user_id: int64_t,
    mut locked: bool,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"UPDATE users SET password_change_locked = ?, updated_at = ? WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    sqlite3_bind_int(
        stmt,
        1 as ::core::ffi::c_int,
        if locked as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, now as sqlite3_int64);
    sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, user_id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update password lock status: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Password lock status updated for user: %lld (locked: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
        locked as ::core::ffi::c_int,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_create_session(
    mut user_id: int64_t,
    mut ip_address: *const ::core::ffi::c_char,
    mut user_agent: *const ::core::ffi::c_char,
    mut expiry_seconds: ::core::ffi::c_int,
    mut token: *mut ::core::ffi::c_char,
    mut token_size: size_t,
) -> ::core::ffi::c_int {
    if token.is_null() || token_size < 33 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Token buffer is too small\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT id FROM users WHERE id = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    if sqlite3_step(stmt) != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"User not found: %lld\0".as_ptr() as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    if generate_random_string(token, 32 as size_t) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to generate session token\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut absolute_expiry: ::core::ffi::c_int = if expiry_seconds > 0 as ::core::ffi::c_int {
        expiry_seconds
    } else {
        default_session_absolute_expiry_seconds()
    };
    let mut idle_expiry: ::core::ffi::c_int = if expiry_seconds > 0 as ::core::ffi::c_int {
        expiry_seconds
    } else {
        default_session_idle_expiry_seconds()
    };
    let mut expires_at: time_t = now + absolute_expiry as time_t;
    let mut idle_expires_at: time_t = now + idle_expiry as time_t;
    let mut has_ip_column: bool = cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"ip_address\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_ua_column: bool = cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"user_agent\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_last_activity_column: bool = cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"last_activity_at\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_idle_expires_column: bool = cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"idle_expires_at\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if has_ip_column as ::core::ffi::c_int != 0
        && has_ua_column as ::core::ffi::c_int != 0
        && has_last_activity_column as ::core::ffi::c_int != 0
        && has_idle_expires_column as ::core::ffi::c_int != 0
    {
        sql = b"INSERT INTO sessions (user_id, token, created_at, last_activity_at, idle_expires_at, expires_at, ip_address, user_agent) VALUES (?, ?, ?, ?, ?, ?, ?, ?);\0"
            .as_ptr() as *const ::core::ffi::c_char;
    } else if has_ip_column as ::core::ffi::c_int != 0 && has_ua_column as ::core::ffi::c_int != 0 {
        sql = b"INSERT INTO sessions (user_id, token, created_at, expires_at, ip_address, user_agent) VALUES (?, ?, ?, ?, ?, ?);\0"
            .as_ptr() as *const ::core::ffi::c_char;
    } else if has_ip_column as ::core::ffi::c_int != 0 && !has_ua_column {
        sql = b"INSERT INTO sessions (user_id, token, created_at, expires_at, ip_address) VALUES (?, ?, ?, ?, ?);\0"
            .as_ptr() as *const ::core::ffi::c_char;
    } else if !has_ip_column && has_ua_column as ::core::ffi::c_int != 0 {
        sql = b"INSERT INTO sessions (user_id, token, created_at, expires_at, user_agent) VALUES (?, ?, ?, ?, ?);\0"
            .as_ptr() as *const ::core::ffi::c_char;
    } else {
        sql =
            b"INSERT INTO sessions (user_id, token, created_at, expires_at) VALUES (?, ?, ?, ?);\0"
                .as_ptr() as *const ::core::ffi::c_char;
    }
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare session insert statement: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut param_index: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let c2rust_fresh5 = param_index;
    param_index += 1;
    sqlite3_bind_int64(stmt, c2rust_fresh5, user_id as sqlite3_int64);
    let c2rust_fresh6 = param_index;
    param_index += 1;
    sqlite3_bind_text(
        stmt,
        c2rust_fresh6,
        token,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    let c2rust_fresh7 = param_index;
    param_index += 1;
    sqlite3_bind_int64(stmt, c2rust_fresh7, now as sqlite3_int64);
    if has_last_activity_column as ::core::ffi::c_int != 0
        && has_idle_expires_column as ::core::ffi::c_int != 0
    {
        let c2rust_fresh8 = param_index;
        param_index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh8, now as sqlite3_int64);
        let c2rust_fresh9 = param_index;
        param_index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh9, idle_expires_at as sqlite3_int64);
    }
    let c2rust_fresh10 = param_index;
    param_index += 1;
    sqlite3_bind_int64(stmt, c2rust_fresh10, expires_at as sqlite3_int64);
    if has_ip_column {
        let c2rust_fresh11 = param_index;
        param_index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh11,
            if !ip_address.is_null() {
                ip_address
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
    if has_ua_column {
        let c2rust_fresh12 = param_index;
        param_index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh12,
            if !user_agent.is_null() {
                user_agent
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to insert session: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Session created successfully for user: %lld\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn lock_auth_database() -> *mut sqlite3 {
    pthread_mutex_lock(get_db_mutex());
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        pthread_mutex_unlock(get_db_mutex());
        return ::core::ptr::null_mut::<sqlite3>();
    }
    sqlite3_mutex_enter(sqlite3_db_mutex(db));
    return db;
}
unsafe extern "C" fn unlock_auth_database(mut db: *mut sqlite3) {
    sqlite3_mutex_leave(sqlite3_db_mutex(db));
    pthread_mutex_unlock(get_db_mutex());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_create_mfa_challenge_verified(
    mut user_id: int64_t,
    mut password: *const ::core::ffi::c_char,
    mut ip_address: *const ::core::ffi::c_char,
    mut user_agent: *const ::core::ffi::c_char,
    mut token: *mut ::core::ffi::c_char,
    mut token_size: size_t,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !token.is_null() && token_size != 0 {
        *token.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if token.is_null()
        || token_size < 33 as size_t
        || generate_random_string(token, 32 as size_t) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = lock_auth_database();
    if db.is_null() {
        *token.offset(0isize) = '\0' as ::core::ffi::c_char;
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    '_done: {
        if sqlite3_exec(
            db,
            b"BEGIN IMMEDIATE\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            result = identity_source_allowed_locked(db, user_id, ip_address);
            if result == 0 as ::core::ffi::c_int
                && !password.is_null()
                && db_auth_verify_password(user_id, password) != 0 as ::core::ffi::c_int
            {
                result = C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0;
            }
            if result == 0 as ::core::ffi::c_int {
                result = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
                sql = b"INSERT INTO sessions (user_id, token, created_at, last_activity_at, idle_expires_at, expires_at, ip_address, user_agent, mfa_pending, mfa_attempts) SELECT id, ?1, ?2, ?2, ?2 + 300, ?2 + 300, ?3, ?4, 1, 0 FROM users WHERE id = ?5 AND is_active = 1 AND totp_enabled = 1;\0"
                    .as_ptr() as *const ::core::ffi::c_char;
                if sqlite3_prepare_v2(
                    db,
                    sql,
                    -1 as ::core::ffi::c_int,
                    &raw mut stmt,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                ) == SQLITE_OK
                {
                    sqlite3_bind_text(
                        stmt,
                        1 as ::core::ffi::c_int,
                        token,
                        -1 as ::core::ffi::c_int,
                        SQLITE_STATIC,
                    );
                    sqlite3_bind_int64(
                        stmt,
                        2 as ::core::ffi::c_int,
                        time(::core::ptr::null_mut::<time_t>()) as sqlite3_int64,
                    );
                    sqlite3_bind_text(
                        stmt,
                        3 as ::core::ffi::c_int,
                        if !ip_address.is_null() {
                            ip_address
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        -1 as ::core::ffi::c_int,
                        SQLITE_STATIC,
                    );
                    sqlite3_bind_text(
                        stmt,
                        4 as ::core::ffi::c_int,
                        if !user_agent.is_null() {
                            user_agent
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        -1 as ::core::ffi::c_int,
                        SQLITE_STATIC,
                    );
                    sqlite3_bind_int64(stmt, 5 as ::core::ffi::c_int, user_id as sqlite3_int64);
                    if sqlite3_step(stmt) == SQLITE_DONE
                        && sqlite3_changes(db) == 1 as ::core::ffi::c_int
                    {
                        result = 0 as ::core::ffi::c_int;
                    }
                }
                sqlite3_finalize(stmt);
                stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                if result == 0 as ::core::ffi::c_int
                    && sqlite3_exec(
                        db,
                        b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char,
                        None,
                        NULL,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    ) == SQLITE_OK
                {
                    break '_done;
                } else if result == 0 as ::core::ffi::c_int {
                    result = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
                }
            }
            sqlite3_exec(
                db,
                b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
    }
    sqlite3_finalize(stmt);
    unlock_auth_database(db);
    if result != 0 as ::core::ffi::c_int {
        *token.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_create_mfa_challenge(
    mut user_id: int64_t,
    mut ip_address: *const ::core::ffi::c_char,
    mut user_agent: *const ::core::ffi::c_char,
    mut token: *mut ::core::ffi::c_char,
    mut token_size: size_t,
) -> ::core::ffi::c_int {
    return db_auth_create_mfa_challenge_verified(
        user_id,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ip_address,
        user_agent,
        token,
        token_size,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_validate_mfa_challenge(
    mut token: *const ::core::ffi::c_char,
    mut user_id: *mut int64_t,
    mut ip_address: *const ::core::ffi::c_char,
    mut user_agent: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut id: int64_t = 0;
    let mut owner: int64_t = 0;
    let mut sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if token.is_null() || strnlen(token, 33 as size_t) != 32 as size_t || user_id.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = lock_auth_database();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    '_done: {
        if sqlite3_exec(
            db,
            b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            sql = b"SELECT s.id, s.user_id FROM sessions s JOIN users u ON u.id = s.user_id WHERE s.token = ? AND s.mfa_pending = 1 AND s.mfa_attempts < 5 AND s.expires_at > ? AND u.is_active = 1 AND u.totp_enabled = 1 AND s.ip_address = ? AND s.user_agent = ?;\0"
                .as_ptr() as *const ::core::ffi::c_char;
            if sqlite3_prepare_v2(
                db,
                sql,
                -1 as ::core::ffi::c_int,
                &raw mut stmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ) == SQLITE_OK
            {
                sqlite3_bind_text(
                    stmt,
                    1 as ::core::ffi::c_int,
                    token,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
                sqlite3_bind_int64(
                    stmt,
                    2 as ::core::ffi::c_int,
                    time(::core::ptr::null_mut::<time_t>()) as sqlite3_int64,
                );
                sqlite3_bind_text(
                    stmt,
                    3 as ::core::ffi::c_int,
                    if !ip_address.is_null() {
                        ip_address
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
                sqlite3_bind_text(
                    stmt,
                    4 as ::core::ffi::c_int,
                    if !user_agent.is_null() {
                        user_agent
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
                if sqlite3_step(stmt) == SQLITE_ROW {
                    id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
                    owner = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as int64_t;
                    sqlite3_finalize(stmt);
                    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                    if sqlite3_prepare_v2(
                        db,
                        b"UPDATE sessions SET mfa_attempts = mfa_attempts + 1 WHERE id = ?;\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                        &raw mut stmt,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    ) == SQLITE_OK
                    {
                        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, id as sqlite3_int64);
                        if !(sqlite3_step(stmt) != SQLITE_DONE
                            || sqlite3_changes(db) != 1 as ::core::ffi::c_int)
                        {
                            sqlite3_finalize(stmt);
                            stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                            if sqlite3_exec(
                                db,
                                b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
                                None,
                                NULL,
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                            ) == SQLITE_OK
                            {
                                *user_id = owner;
                                result = 0 as ::core::ffi::c_int;
                                break '_done;
                            }
                        }
                    }
                }
            }
            sqlite3_finalize(stmt);
            stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
    }
    sqlite3_finalize(stmt);
    unlock_auth_database(db);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_consume_mfa_challenge(
    mut token: *const ::core::ffi::c_char,
    mut user_id: int64_t,
) -> ::core::ffi::c_int {
    if token.is_null() || strnlen(token, 33 as size_t) != 32 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = lock_auth_database();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut sql: *const ::core::ffi::c_char = b"DELETE FROM sessions WHERE token = ? AND user_id = ? AND mfa_pending = 1 AND mfa_attempts BETWEEN 1 AND 5 AND expires_at > ? AND user_id IN (SELECT id FROM users WHERE is_active = 1 AND totp_enabled = 1);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            token,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, user_id as sqlite3_int64);
        sqlite3_bind_int64(
            stmt,
            3 as ::core::ffi::c_int,
            time(::core::ptr::null_mut::<time_t>()) as sqlite3_int64,
        );
        if sqlite3_step(stmt) == SQLITE_DONE && sqlite3_changes(db) == 1 as ::core::ffi::c_int {
            result = 0 as ::core::ffi::c_int;
        }
    }
    sqlite3_finalize(stmt);
    unlock_auth_database(db);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_validate_session_with_context(
    mut token: *const ::core::ffi::c_char,
    mut user_id: *mut int64_t,
    mut ip_address: *const ::core::ffi::c_char,
    mut user_agent: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if token.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Token is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut has_idle_expires_column: bool = cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"idle_expires_at\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_last_activity_column: bool = cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"last_activity_at\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_ip_column: bool = cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"ip_address\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_ua_column: bool = cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"user_agent\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_tracking_columns: bool = has_idle_expires_column as ::core::ffi::c_int != 0
        && has_last_activity_column as ::core::ffi::c_int != 0;
    let mut sql: [::core::ffi::c_char; 512] = [0; 512];
    let mut select_written: ::core::ffi::c_int = snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"SELECT s.id, s.user_id, s.expires_at, %s, %s, u.is_active, %s, %s FROM sessions s JOIN users u ON s.user_id = u.id WHERE s.token = ? AND s.mfa_pending = 0;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if has_idle_expires_column as ::core::ffi::c_int != 0 {
            b"s.idle_expires_at\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"s.expires_at\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_tracking_columns as ::core::ffi::c_int != 0 {
            b"COALESCE(s.last_activity_at, s.created_at)\0".as_ptr()
                as *const ::core::ffi::c_char
        } else {
            b"0\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_ip_column as ::core::ffi::c_int != 0 {
            b"COALESCE(s.ip_address, '')\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"''\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_ua_column as ::core::ffi::c_int != 0 {
            b"COALESCE(s.user_agent, '')\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"''\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if select_written < 0 as ::core::ffi::c_int
        || select_written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to build session lookup SQL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut ro: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut ro) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        ro,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(ro),
        );
        db_close_readonly_connection(ro);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        token,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if sqlite3_step(stmt) != SQLITE_ROW {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Session not found for token\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3_finalize(stmt);
        db_close_readonly_connection(ro);
        return -1 as ::core::ffi::c_int;
    }
    let mut session_id: int64_t = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    let mut id: int64_t = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as int64_t;
    let mut expires_at: time_t = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as time_t;
    let mut idle_expires_at: time_t = (if has_idle_expires_column as ::core::ffi::c_int != 0 {
        sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int)
    } else {
        expires_at as sqlite3_int64
    }) as time_t;
    let mut last_activity_at: time_t = (if has_tracking_columns as ::core::ffi::c_int != 0 {
        sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int)
    } else {
        0 as sqlite3_int64
    }) as time_t;
    let mut is_active: ::core::ffi::c_int = sqlite3_column_int(stmt, 5 as ::core::ffi::c_int);
    let mut stored_ip: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 6 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    let mut stored_ua: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 7 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    let mut update_ip: bool = has_ip_column as ::core::ffi::c_int != 0
        && tracking_value_differs(stored_ip, ip_address) as ::core::ffi::c_int != 0;
    let mut update_ua: bool = has_ua_column as ::core::ffi::c_int != 0
        && tracking_value_differs(stored_ua, user_agent) as ::core::ffi::c_int != 0;
    sqlite3_finalize(stmt);
    db_close_readonly_connection(ro);
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if now > expires_at || now > idle_expires_at {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Session has expired\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if is_active == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"User is inactive\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !user_id.is_null() {
        *user_id = id;
    }
    let mut refresh_tracking: bool = has_tracking_columns as ::core::ffi::c_int != 0
        && should_refresh_session_tracking(now, last_activity_at, idle_expires_at)
            as ::core::ffi::c_int
            != 0;
    if refresh_tracking as ::core::ffi::c_int != 0
        || update_ip as ::core::ffi::c_int != 0
        || update_ua as ::core::ffi::c_int != 0
    {
        let mut new_idle_expires_at: time_t = now + default_session_idle_expiry_seconds() as time_t;
        if new_idle_expires_at > expires_at {
            new_idle_expires_at = expires_at;
        }
        let mut update_sql: [::core::ffi::c_char; 256] = ::core::mem::transmute::<
            [u8; 256],
            [::core::ffi::c_char; 256],
        >(
            *b"UPDATE sessions SET \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut sql_len: size_t = strlen(&raw mut update_sql as *mut ::core::ffi::c_char);
        let mut need_comma: bool = r#false != 0;
        if refresh_tracking {
            let mut written: ::core::ffi::c_int = snprintf(
                (&raw mut update_sql as *mut ::core::ffi::c_char).offset(sql_len as isize),
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(sql_len),
                b"%slast_activity_at = ?, idle_expires_at = ?\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if need_comma as ::core::ffi::c_int != 0 {
                    b", \0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            if written < 0 as ::core::ffi::c_int
                || written as size_t
                    >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(sql_len)
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to build session refresh SQL for session %lld\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    session_id as ::core::ffi::c_longlong,
                );
                return 0 as ::core::ffi::c_int;
            }
            sql_len = sql_len.wrapping_add(written as size_t);
            need_comma = r#true != 0;
        }
        if update_ip {
            let mut written_0: ::core::ffi::c_int = snprintf(
                (&raw mut update_sql as *mut ::core::ffi::c_char).offset(sql_len as isize),
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(sql_len),
                b"%sip_address = ?\0".as_ptr() as *const ::core::ffi::c_char,
                if need_comma as ::core::ffi::c_int != 0 {
                    b", \0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            if written_0 < 0 as ::core::ffi::c_int
                || written_0 as size_t
                    >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(sql_len)
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to build session IP refresh SQL for session %lld\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    session_id as ::core::ffi::c_longlong,
                );
                return 0 as ::core::ffi::c_int;
            }
            sql_len = sql_len.wrapping_add(written_0 as size_t);
            need_comma = r#true != 0;
        }
        if update_ua {
            let mut written_1: ::core::ffi::c_int = snprintf(
                (&raw mut update_sql as *mut ::core::ffi::c_char).offset(sql_len as isize),
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(sql_len),
                b"%suser_agent = ?\0".as_ptr() as *const ::core::ffi::c_char,
                if need_comma as ::core::ffi::c_int != 0 {
                    b", \0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            if written_1 < 0 as ::core::ffi::c_int
                || written_1 as size_t
                    >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(sql_len)
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to build session user-agent refresh SQL for session %lld\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    session_id as ::core::ffi::c_longlong,
                );
                return 0 as ::core::ffi::c_int;
            }
            sql_len = sql_len.wrapping_add(written_1 as size_t);
        }
        let mut written_2: ::core::ffi::c_int = snprintf(
            (&raw mut update_sql as *mut ::core::ffi::c_char).offset(sql_len as isize),
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(sql_len),
            b" WHERE id = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if written_2 < 0 as ::core::ffi::c_int
            || written_2 as size_t
                >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(sql_len)
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to finalize session refresh SQL for session %lld\0".as_ptr()
                    as *const ::core::ffi::c_char,
                session_id as ::core::ffi::c_longlong,
            );
            return 0 as ::core::ffi::c_int;
        }
        rc = sqlite3_prepare_v2(
            db,
            &raw mut update_sql as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            let mut param: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            if refresh_tracking {
                let c2rust_fresh13 = param;
                param += 1;
                sqlite3_bind_int64(stmt, c2rust_fresh13, now as sqlite3_int64);
                let c2rust_fresh14 = param;
                param += 1;
                sqlite3_bind_int64(stmt, c2rust_fresh14, new_idle_expires_at as sqlite3_int64);
            }
            if update_ip {
                let c2rust_fresh15 = param;
                param += 1;
                sqlite3_bind_text(
                    stmt,
                    c2rust_fresh15,
                    if !ip_address.is_null() {
                        ip_address
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            if update_ua {
                let c2rust_fresh16 = param;
                param += 1;
                sqlite3_bind_text(
                    stmt,
                    c2rust_fresh16,
                    if !user_agent.is_null() {
                        user_agent
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
            sqlite3_bind_int64(stmt, param, session_id as sqlite3_int64);
            if sqlite3_step(stmt) != SQLITE_DONE {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to refresh tracking for session %lld: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    session_id as ::core::ffi::c_longlong,
                    sqlite3_errmsg(db),
                );
            }
            sqlite3_finalize(stmt);
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to prepare tracking refresh for session %lld: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                session_id as ::core::ffi::c_longlong,
                sqlite3_errmsg(db),
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_validate_session(
    mut token: *const ::core::ffi::c_char,
    mut user_id: *mut int64_t,
) -> ::core::ffi::c_int {
    return db_auth_validate_session_with_context(
        token,
        user_id,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_delete_session(
    mut token: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if token.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Token is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM sessions WHERE token = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        token,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to delete session: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Session deleted successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_delete_user_sessions(mut user_id: int64_t) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM sessions WHERE user_id = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to delete sessions: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Sessions deleted successfully for user: %lld\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_cleanup_sessions() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char = if cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"idle_expires_at\0".as_ptr() as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
    {
        b"DELETE FROM sessions WHERE expires_at < ? OR idle_expires_at < ?;\0".as_ptr()
            as *const ::core::ffi::c_char
    } else {
        b"DELETE FROM sessions WHERE expires_at < ?;\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, now as sqlite3_int64);
    if cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"idle_expires_at\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, now as sqlite3_int64);
    }
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to delete expired sessions: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut deleted: ::core::ffi::c_int = sqlite3_changes(db);
    sqlite3_finalize(stmt);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleaned up %d expired sessions\0".as_ptr() as *const ::core::ffi::c_char,
        deleted,
    );
    rc = sqlite3_prepare_v2(
        db,
        b"DELETE FROM trusted_devices WHERE expires_at < ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, now as sqlite3_int64);
        if sqlite3_step(stmt) != SQLITE_DONE {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to clean up expired trusted devices: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
        }
        sqlite3_finalize(stmt);
    }
    return deleted;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_list_user_sessions(
    mut user_id: int64_t,
    mut sessions: *mut session_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sessions.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut has_idle_expires_column: bool = cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"idle_expires_at\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_last_activity_column: bool = cached_column_exists(
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
        b"last_activity_at\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut sql: [::core::ffi::c_char; 768] = [0; 768];
    snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 768]>(),
        b"SELECT id, user_id, token, created_at, %s, %s, expires_at, COALESCE(ip_address, ''), COALESCE(user_agent, '') FROM sessions WHERE user_id = ? AND mfa_pending = 0 AND expires_at >= ? %s ORDER BY %s DESC, id DESC LIMIT ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if has_last_activity_column as ::core::ffi::c_int != 0 {
            b"COALESCE(last_activity_at, created_at)\0".as_ptr()
                as *const ::core::ffi::c_char
        } else {
            b"created_at\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_idle_expires_column as ::core::ffi::c_int != 0 {
            b"COALESCE(idle_expires_at, expires_at)\0".as_ptr()
                as *const ::core::ffi::c_char
        } else {
            b"expires_at\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_idle_expires_column as ::core::ffi::c_int != 0 {
            b"AND (idle_expires_at IS NULL OR idle_expires_at >= ?)\0".as_ptr()
                as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_last_activity_column as ::core::ffi::c_int != 0 {
            b"COALESCE(last_activity_at, created_at)\0".as_ptr()
                as *const ::core::ffi::c_char
        } else {
            b"created_at\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut param: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let c2rust_fresh17 = param;
    param += 1;
    sqlite3_bind_int64(stmt, c2rust_fresh17, user_id as sqlite3_int64);
    let c2rust_fresh18 = param;
    param += 1;
    sqlite3_bind_int64(stmt, c2rust_fresh18, now as sqlite3_int64);
    if has_idle_expires_column {
        let c2rust_fresh19 = param;
        param += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh19, now as sqlite3_int64);
    }
    sqlite3_bind_int(stmt, param, max_count);
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while count < max_count && {
        rc = sqlite3_step(stmt);
        rc == SQLITE_ROW
    } {
        let c2rust_fresh20 = count;
        count += 1;
        let mut session: *mut session_t = sessions.offset(c2rust_fresh20 as isize);
        memset(
            session as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<session_t>(),
        );
        (*session).id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
        (*session).user_id = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as int64_t;
        let mut token: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut ip: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 7 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut ua: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 8 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !token.is_null() {
            safe_strcpy(
                &raw mut (*session).token as *mut ::core::ffi::c_char,
                token,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                0 as size_t,
            );
        }
        if !ip.is_null() {
            safe_strcpy(
                &raw mut (*session).ip_address as *mut ::core::ffi::c_char,
                ip,
                ::core::mem::size_of::<[::core::ffi::c_char; 46]>(),
                0 as size_t,
            );
        }
        if !ua.is_null() {
            safe_strcpy(
                &raw mut (*session).user_agent as *mut ::core::ffi::c_char,
                ua,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        }
        (*session).created_at = sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as int64_t;
        (*session).last_activity_at =
            sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
        (*session).idle_expires_at = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as int64_t;
        (*session).expires_at = sqlite3_column_int64(stmt, 6 as ::core::ffi::c_int) as int64_t;
    }
    sqlite3_finalize(stmt);
    return if rc == SQLITE_DONE || rc == SQLITE_ROW && count == max_count {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn revoke_owned_credential(
    mut owner: int64_t,
    mut id: int64_t,
    mut current_token: *const ::core::ffi::c_char,
    mut deleted_current: *mut bool,
    mut trusted_device: bool,
) -> ::core::ffi::c_int {
    if deleted_current.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *deleted_current = r#false != 0;
    if owner <= 0 as int64_t || id <= 0 as int64_t {
        return 1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = lock_auth_database();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = if trusted_device as ::core::ffi::c_int != 0 {
        b"DELETE FROM trusted_devices WHERE user_id = ? AND id = ? RETURNING token;\0".as_ptr()
            as *const ::core::ffi::c_char
    } else {
        b"DELETE FROM sessions WHERE user_id = ? AND id = ? AND mfa_pending = 0 RETURNING token;\0"
            .as_ptr() as *const ::core::ffi::c_char
    };
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, owner as sqlite3_int64);
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, id as sqlite3_int64);
        let mut rc: ::core::ffi::c_int = sqlite3_step(stmt);
        if rc == SQLITE_DONE {
            result = 1 as ::core::ffi::c_int;
        } else if rc == SQLITE_ROW {
            let mut removed_token: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            let mut matched: bool = !current_token.is_null()
                && *current_token as ::core::ffi::c_int != 0
                && !removed_token.is_null()
                && strcmp(current_token, removed_token) == 0 as ::core::ffi::c_int;
            if sqlite3_step(stmt) == SQLITE_DONE {
                *deleted_current = matched;
                result = 0 as ::core::ffi::c_int;
            }
        }
    }
    sqlite3_finalize(stmt);
    unlock_auth_database(db);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_revoke_session(
    mut user_id: int64_t,
    mut session_id: int64_t,
    mut current_token: *const ::core::ffi::c_char,
    mut deleted_current: *mut bool,
) -> ::core::ffi::c_int {
    return revoke_owned_credential(
        user_id,
        session_id,
        current_token,
        deleted_current,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_revoke_trusted_device(
    mut user_id: int64_t,
    mut device_id: int64_t,
    mut current_token: *const ::core::ffi::c_char,
    mut deleted_current: *mut bool,
) -> ::core::ffi::c_int {
    return revoke_owned_credential(
        user_id,
        device_id,
        current_token,
        deleted_current,
        r#true != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_delete_session_by_id(
    mut user_id: int64_t,
    mut session_id: int64_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM sessions WHERE id = ? AND user_id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, session_id as sqlite3_int64);
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, user_id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    let mut changes: ::core::ffi::c_int = if rc == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    sqlite3_finalize(stmt);
    return if rc == SQLITE_DONE && changes > 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_create_trusted_device(
    mut user_id: int64_t,
    mut ip_address: *const ::core::ffi::c_char,
    mut user_agent: *const ::core::ffi::c_char,
    mut expiry_seconds: ::core::ffi::c_int,
    mut token: *mut ::core::ffi::c_char,
    mut token_size: size_t,
) -> ::core::ffi::c_int {
    if token.is_null() || token_size < 33 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Token buffer is too small\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if generate_random_string(token, 32 as size_t) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut token_hash: [::core::ffi::c_char; 65] = [0; 65];
    if hash_token_identifier(
        token,
        &raw mut token_hash as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut lifetime: ::core::ffi::c_int = if expiry_seconds > 0 as ::core::ffi::c_int {
        expiry_seconds
    } else {
        default_trusted_device_expiry_seconds()
    };
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"INSERT INTO trusted_devices (user_id, token, ip_address, user_agent, created_at, last_used_at, expires_at) VALUES (?, ?, ?, ?, ?, ?, ?);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw mut token_hash as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        stmt,
        3 as ::core::ffi::c_int,
        if !ip_address.is_null() {
            ip_address
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        4 as ::core::ffi::c_int,
        if !user_agent.is_null() {
            user_agent
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 5 as ::core::ffi::c_int, now as sqlite3_int64);
    sqlite3_bind_int64(stmt, 6 as ::core::ffi::c_int, now as sqlite3_int64);
    sqlite3_bind_int64(
        stmt,
        7 as ::core::ffi::c_int,
        (now + lifetime as time_t) as sqlite3_int64,
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn lookup_trusted_device(
    mut db: *mut sqlite3,
    mut user_id: int64_t,
    mut token: *const ::core::ffi::c_char,
    mut trusted_id: *mut int64_t,
    mut expires_at: *mut time_t,
) -> ::core::ffi::c_int {
    if db.is_null()
        || token.is_null()
        || *token.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut token_hash: [::core::ffi::c_char; 65] = [0; 65];
    if hash_token_identifier(
        token,
        &raw mut token_hash as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT id, expires_at FROM trusted_devices WHERE user_id = ? AND token = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw mut token_hash as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    if sqlite3_step(stmt) != SQLITE_ROW {
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    if !trusted_id.is_null() {
        *trusted_id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    }
    if !expires_at.is_null() {
        *expires_at = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as time_t;
    }
    sqlite3_finalize(stmt);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_get_trusted_device_id(
    mut user_id: int64_t,
    mut token: *const ::core::ffi::c_char,
    mut trusted_device_id: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut resolved_id: int64_t = 0 as int64_t;
    let mut expires_at: time_t = 0 as time_t;
    if lookup_trusted_device(
        db,
        user_id,
        token,
        &raw mut resolved_id,
        &raw mut expires_at,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if time(::core::ptr::null_mut::<time_t>()) > expires_at {
        db_auth_delete_trusted_device_by_id(user_id, resolved_id);
        return -1 as ::core::ffi::c_int;
    }
    if !trusted_device_id.is_null() {
        *trusted_device_id = resolved_id;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_validate_trusted_device(
    mut user_id: int64_t,
    mut token: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if token.is_null() || *token.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut trusted_id: int64_t = 0 as int64_t;
    if db_auth_get_trusted_device_id(user_id, token, &raw mut trusted_id) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"UPDATE trusted_devices SET last_used_at = ? WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(
            stmt,
            1 as ::core::ffi::c_int,
            time(::core::ptr::null_mut::<time_t>()) as sqlite3_int64,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, trusted_id as sqlite3_int64);
        sqlite3_step(stmt);
        sqlite3_finalize(stmt);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_list_trusted_devices(
    mut user_id: int64_t,
    mut devices: *mut trusted_device_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if devices.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT id, user_id, created_at, COALESCE(last_used_at, created_at), expires_at, COALESCE(ip_address, ''), COALESCE(user_agent, '') FROM trusted_devices WHERE user_id = ? AND expires_at >= ? ORDER BY COALESCE(last_used_at, created_at) DESC, id DESC LIMIT ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, now as sqlite3_int64);
    sqlite3_bind_int(stmt, 3 as ::core::ffi::c_int, max_count);
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while count < max_count && {
        rc = sqlite3_step(stmt);
        rc == SQLITE_ROW
    } {
        let c2rust_fresh21 = count;
        count += 1;
        let mut device: *mut trusted_device_t = devices.offset(c2rust_fresh21 as isize);
        memset(
            device as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<trusted_device_t>(),
        );
        (*device).id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
        (*device).user_id = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as int64_t;
        let mut ip: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 5 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut ua: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 6 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !ip.is_null() {
            safe_strcpy(
                &raw mut (*device).ip_address as *mut ::core::ffi::c_char,
                ip,
                ::core::mem::size_of::<[::core::ffi::c_char; 46]>(),
                0 as size_t,
            );
        }
        if !ua.is_null() {
            safe_strcpy(
                &raw mut (*device).user_agent as *mut ::core::ffi::c_char,
                ua,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        }
        (*device).created_at = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as int64_t;
        (*device).last_used_at = sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as int64_t;
        (*device).expires_at = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
    }
    sqlite3_finalize(stmt);
    return if rc == SQLITE_DONE || rc == SQLITE_ROW && count == max_count {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_delete_trusted_device_by_id(
    mut user_id: int64_t,
    mut trusted_device_id: int64_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM trusted_devices WHERE id = ? AND user_id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(
        stmt,
        1 as ::core::ffi::c_int,
        trusted_device_id as sqlite3_int64,
    );
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, user_id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    let mut changes: ::core::ffi::c_int = if rc == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    sqlite3_finalize(stmt);
    return if rc == SQLITE_DONE && changes > 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_get_role_name(
    mut role: user_role_t,
) -> *const ::core::ffi::c_char {
    if role.0 < 0 as ::core::ffi::c_uint
        || role.0 as usize
            >= ::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>()
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return role_names[role.0 as usize];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_get_role_id(
    mut role_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if role_name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if strcmp(role_names[i], role_name) == 0 as ::core::ffi::c_int {
            return i as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_get_totp_info(
    mut user_id: int64_t,
    mut secret: *mut ::core::ffi::c_char,
    mut secret_size: size_t,
    mut enabled: *mut bool,
) -> ::core::ffi::c_int {
    if secret.is_null() || enabled.is_null() || secret_size == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for db_auth_get_totp_info\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    *secret.offset(0isize) = '\0' as ::core::ffi::c_char;
    *enabled = r#false != 0;
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"totp_secret\0".as_ptr() as *const ::core::ffi::c_char,
    ) || !cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"totp_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"TOTP columns not yet available in users table\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT totp_secret, totp_enabled FROM users WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare TOTP query: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    if sqlite3_step(stmt) != SQLITE_ROW {
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut db_secret: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if !db_secret.is_null()
        && *db_secret.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        safe_strcpy(secret, db_secret, secret_size, 0 as size_t);
    }
    *enabled = sqlite3_column_int(stmt, 1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    sqlite3_finalize(stmt);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_set_totp_secret(
    mut user_id: int64_t,
    mut secret: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if secret.is_null() || *secret == 0 {
        return db_auth_disable_totp(user_id, ::core::ptr::null::<::core::ffi::c_char>());
    }
    let mut db: *mut sqlite3 = lock_auth_database();
    if db.is_null() {
        return C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"UPDATE users SET totp_secret=?1,updated_at=?2,totp_setup_expires_at=?2+600,totp_last_used_step=-1 WHERE id=?3 AND totp_enabled=0;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            secret,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(
            stmt,
            2 as ::core::ffi::c_int,
            time(::core::ptr::null_mut::<time_t>()) as sqlite3_int64,
        );
        sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, user_id as sqlite3_int64);
        if sqlite3_step(stmt) == SQLITE_DONE {
            result = if sqlite3_changes(db) == 1 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed::DB_AUTH_IDENTITY_CONFLICT.0
            };
        }
    }
    sqlite3_finalize(stmt);
    unlock_auth_database(db);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_enable_totp(
    mut user_id: int64_t,
    mut enabled: bool,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"totp_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"totp_enabled column not available\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"UPDATE users SET totp_enabled = ?, updated_at = ? WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare TOTP enable update: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(
        stmt,
        1 as ::core::ffi::c_int,
        if enabled as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    sqlite3_bind_int64(
        stmt,
        2 as ::core::ffi::c_int,
        time(::core::ptr::null_mut::<time_t>()) as int64_t as sqlite3_int64,
    );
    sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, user_id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update TOTP enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"TOTP %s for user %lld\0".as_ptr() as *const ::core::ffi::c_char,
        if enabled as ::core::ffi::c_int != 0 {
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char
        },
        user_id as ::core::ffi::c_longlong,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_validate_allowed_login_cidrs(
    mut allowed_login_cidrs: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut normalized: [::core::ffi::c_char; 1024] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut has_entries: bool = r#false != 0;
    return normalize_allowed_login_cidrs(
        allowed_login_cidrs,
        &raw mut normalized as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        &raw mut has_entries,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_set_allowed_login_cidrs(
    mut user_id: int64_t,
    mut allowed_login_cidrs: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"allowed_login_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"allowed_login_cidrs column not available\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut normalized: [::core::ffi::c_char; 1024] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut has_entries: bool = r#false != 0;
    if normalize_allowed_login_cidrs(
        allowed_login_cidrs,
        &raw mut normalized as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        &raw mut has_entries,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid allowed_login_cidrs value for user %lld\0".as_ptr()
                as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"UPDATE users SET allowed_login_cidrs = ?, updated_at = ? WHERE id = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare allowed_login_cidrs update: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    if has_entries {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw mut normalized as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    } else {
        sqlite3_bind_null(stmt, 1 as ::core::ffi::c_int);
    }
    sqlite3_bind_int64(
        stmt,
        2 as ::core::ffi::c_int,
        time(::core::ptr::null_mut::<time_t>()) as int64_t as sqlite3_int64,
    );
    sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, user_id as sqlite3_int64);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to update allowed_login_cidrs: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"allowed_login_cidrs updated for user %lld: %s\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
        if has_entries as ::core::ffi::c_int != 0 {
            &raw mut normalized as *mut ::core::ffi::c_char
        } else {
            b"(unrestricted)\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_ip_allowed_for_user(
    mut user: *const user_t,
    mut client_ip: *const ::core::ffi::c_char,
) -> bool {
    if user.is_null() {
        return r#false != 0;
    }
    if !(*user).has_login_cidr_restriction {
        return r#true != 0;
    }
    if client_ip.is_null()
        || *client_ip.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut cidr_list: [::core::ffi::c_char; 1024] = [0; 1024];
    safe_strcpy(
        &raw mut cidr_list as *mut ::core::ffi::c_char,
        &raw const (*user).allowed_login_cidrs as *const ::core::ffi::c_char,
        USER_ALLOWED_LOGIN_CIDRS_MAX as size_t,
        0 as size_t,
    );
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token: *mut ::core::ffi::c_char = strtok_r(
        &raw mut cidr_list as *mut ::core::ffi::c_char,
        b",\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !token.is_null() {
        let mut trimmed: *mut ::core::ffi::c_char = trim_ascii_whitespace(token);
        if !(trimmed.is_null()
            || *trimmed.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int)
        {
            if ip_matches_cidr(client_ip, trimmed) {
                return r#true != 0;
            }
        }
        token = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    return r#false != 0;
}
unsafe extern "C" fn finish_identity_transaction(
    mut db: *mut sqlite3,
    mut result: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if result != C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0
        && sqlite3_exec(
            db,
            b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) != SQLITE_OK
    {
        result = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    }
    if result == C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0 {
        sqlite3_exec(
            db,
            b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    return result;
}
unsafe extern "C" fn identity_source_allowed_locked(
    mut db: *mut sqlite3,
    mut user_id: int64_t,
    mut ip: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT is_active,allowed_login_cidrs FROM users WHERE id=?\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return result;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
    if step == SQLITE_ROW {
        let mut user: user_t = user_t {
            id: user_id,
            username: [0; 64],
            email: [0; 128],
            role: user_role_t::USER_ROLE_ADMIN,
            api_key: [0; 64],
            created_at: 0,
            updated_at: 0,
            last_login: 0,
            is_active: false,
            password_change_locked: false,
            must_change_password: false,
            totp_enabled: false,
            allowed_login_cidrs: [0; 1024],
            has_login_cidr_restriction: false,
            authorization_mode: [0; 16],
            authenticated_via_scoped_token: false,
            api_token_uuid: [0; 37],
            authentication_method: [0; 24],
        };
        let mut cidrs: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !cidrs.is_null()
            && strlen(cidrs) >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
        {
            result = C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0;
        } else {
            if !cidrs.is_null() {
                safe_strcpy(
                    &raw mut user.allowed_login_cidrs as *mut ::core::ffi::c_char,
                    cidrs,
                    ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                    0 as size_t,
                );
            }
            user.has_login_cidr_restriction = !cidrs.is_null() && *cidrs as ::core::ffi::c_int != 0;
            result = if sqlite3_column_int(stmt, 0 as ::core::ffi::c_int) != 0
                && db_auth_ip_allowed_for_user(&raw mut user, ip) as ::core::ffi::c_int != 0
            {
                0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0
            };
        }
    } else if step == SQLITE_DONE {
        result = C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0;
    }
    sqlite3_finalize(stmt);
    return result;
}
unsafe extern "C" fn insert_identity_session_locked(
    mut db: *mut sqlite3,
    mut user_id: int64_t,
    mut ip: *const ::core::ffi::c_char,
    mut agent: *const ::core::ffi::c_char,
    mut token: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if generate_random_string(token, 32 as size_t) != 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"INSERT INTO sessions(user_id,token,created_at,last_activity_at,idle_expires_at,expires_at,ip_address,user_agent,mfa_pending,mfa_attempts) VALUES(?1,?2,?3,?3,?4,?5,?6,?7,0,0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    }
    let mut now: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    let mut expires: int64_t = now + default_session_absolute_expiry_seconds() as int64_t;
    let mut idle: int64_t = now + default_session_idle_expiry_seconds() as int64_t;
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        token,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, now as sqlite3_int64);
    sqlite3_bind_int64(
        stmt,
        4 as ::core::ffi::c_int,
        (if idle < expires { idle } else { expires }) as sqlite3_int64,
    );
    sqlite3_bind_int64(stmt, 5 as ::core::ffi::c_int, expires as sqlite3_int64);
    sqlite3_bind_text(
        stmt,
        6 as ::core::ffi::c_int,
        if !ip.is_null() {
            ip
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        7 as ::core::ffi::c_int,
        if !agent.is_null() {
            agent
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    let mut rc: ::core::ffi::c_int = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0
    };
}
unsafe extern "C" fn check_account_totp_locked(
    mut db: *mut sqlite3,
    mut user_id: int64_t,
    mut code: *const ::core::ffi::c_char,
    mut enrollment: bool,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    let mut sql: *const ::core::ffi::c_char = b"SELECT totp_secret,totp_enabled,totp_last_used_step,totp_failed_attempts,totp_window_started_at,totp_setup_expires_at,is_active FROM users WHERE id=?\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return result;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
    if step != SQLITE_ROW {
        sqlite3_finalize(stmt);
        return if step == SQLITE_DONE {
            C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0
        } else {
            result
        };
    }
    let mut secret: [::core::ffi::c_char; 64] = [
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
    let mut stored: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    let mut enabled: bool =
        sqlite3_column_int(stmt, 1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    let mut last: int64_t = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as int64_t;
    let mut attempts: int64_t = sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as int64_t;
    let mut started: int64_t = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
    let mut setup_expiry: int64_t = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as int64_t;
    let mut active: bool =
        sqlite3_column_int(stmt, 6 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    let mut valid_secret: bool = !stored.is_null()
        && *stored as ::core::ffi::c_int != 0
        && strlen(stored) < ::core::mem::size_of::<[::core::ffi::c_char; 64]>();
    if valid_secret {
        safe_strcpy(
            &raw mut secret as *mut ::core::ffi::c_char,
            stored,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    sqlite3_finalize(stmt);
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut now: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    if now < 0 as int64_t {
        return result;
    }
    if !active
        || !valid_secret
        || if enrollment as ::core::ffi::c_int != 0 {
            (enabled as ::core::ffi::c_int != 0 || setup_expiry <= now) as ::core::ffi::c_int
        } else {
            !enabled as ::core::ffi::c_int
        } != 0
    {
        return C2Rust_Unnamed::DB_AUTH_IDENTITY_CONFLICT.0;
    }
    let mut window: ::core::ffi::c_int =
        if g_config.login_rate_limit_window_seconds >= 10 as ::core::ffi::c_int {
            g_config.login_rate_limit_window_seconds
        } else {
            300 as ::core::ffi::c_int
        };
    let mut limit: ::core::ffi::c_int =
        if g_config.login_rate_limit_max_attempts > 0 as ::core::ffi::c_int {
            g_config.login_rate_limit_max_attempts
        } else {
            5 as ::core::ffi::c_int
        };
    if started <= now - window as int64_t {
        started = now;
        attempts = 0 as int64_t;
    }
    if g_config.login_rate_limit_enabled as ::core::ffi::c_int != 0 && attempts >= limit as int64_t
    {
        return C2Rust_Unnamed::DB_AUTH_IDENTITY_LIMITED.0;
    }
    let mut matched: int64_t = -1 as int64_t;
    let mut accepted: bool = totp_match_step(
        &raw mut secret as *mut ::core::ffi::c_char,
        code,
        now as time_t,
        &raw mut matched,
    ) == 0 as ::core::ffi::c_int
        && matched > last;
    if accepted {
        sql = b"UPDATE users SET totp_last_used_step=?2,totp_failed_attempts=0,totp_window_started_at=0,totp_enabled=1,totp_setup_expires_at=0,updated_at=?3 WHERE id=?1\0"
            .as_ptr() as *const ::core::ffi::c_char;
    } else {
        sql = b"UPDATE users SET totp_failed_attempts=?2,totp_window_started_at=?3 WHERE id=?1\0"
            .as_ptr() as *const ::core::ffi::c_char;
    }
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return result;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    sqlite3_bind_int64(
        stmt,
        2 as ::core::ffi::c_int,
        (if accepted as ::core::ffi::c_int != 0 {
            matched
        } else if attempts < INT64_MAX as int64_t {
            attempts + 1 as int64_t
        } else {
            attempts
        }) as sqlite3_int64,
    );
    sqlite3_bind_int64(
        stmt,
        3 as ::core::ffi::c_int,
        (if accepted as ::core::ffi::c_int != 0 {
            now
        } else {
            started
        }) as sqlite3_int64,
    );
    if sqlite3_step(stmt) == SQLITE_DONE && sqlite3_changes(db) == 1 as ::core::ffi::c_int {
        result = if accepted as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0
        };
    }
    sqlite3_finalize(stmt);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_verify_totp_enrollment(
    mut user_id: int64_t,
    mut code: *const ::core::ffi::c_char,
    mut keep_session: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = lock_auth_database();
    if db.is_null() {
        return C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    }
    let mut result: ::core::ffi::c_int = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    if sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        result = check_account_totp_locked(db, user_id, code, r#true != 0);
        if result == 0 as ::core::ffi::c_int
            && revoke_credentials_locked(db, user_id, keep_session) != 0 as ::core::ffi::c_int
        {
            result = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
        }
        result = finish_identity_transaction(db, result);
    }
    unlock_auth_database(db);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_disable_totp(
    mut user_id: int64_t,
    mut keep_session: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = lock_auth_database();
    if db.is_null() {
        return C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    if sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        if sqlite3_prepare_v2(
            db,
            b"UPDATE users SET totp_secret=NULL,totp_enabled=0,totp_last_used_step=-1,totp_setup_expires_at=0,totp_failed_attempts=0,totp_window_started_at=0,updated_at=? WHERE id=?\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            sqlite3_bind_int64(
                stmt,
                1 as ::core::ffi::c_int,
                time(::core::ptr::null_mut::<time_t>()) as sqlite3_int64,
            );
            sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, user_id as sqlite3_int64);
            if sqlite3_step(stmt) == SQLITE_DONE
                && sqlite3_changes(db) == 1 as ::core::ffi::c_int
            {
                result = revoke_credentials_locked(db, user_id, keep_session);
            }
        }
        sqlite3_finalize(stmt);
        result = finish_identity_transaction(db, result);
    }
    unlock_auth_database(db);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_complete_totp_login(
    mut user_id: int64_t,
    mut code: *const ::core::ffi::c_char,
    mut challenge: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut ip: *const ::core::ffi::c_char,
    mut agent: *const ::core::ffi::c_char,
    mut trusted_seconds: ::core::ffi::c_int,
    mut session: *mut ::core::ffi::c_char,
    mut trusted: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !session.is_null() {
        *session.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !trusted.is_null() {
        *trusted.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if session.is_null()
        || trusted.is_null()
        || challenge.is_null() as ::core::ffi::c_int == password.is_null() as ::core::ffi::c_int
    {
        return C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    }
    let mut db: *mut sqlite3 = lock_auth_database();
    if db.is_null() {
        return C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    if sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        result = identity_source_allowed_locked(db, user_id, ip);
        '_finish: {
            if result == 0 as ::core::ffi::c_int {
                if !challenge.is_null() {
                    let mut sql: *const ::core::ffi::c_char = b"SELECT id FROM sessions WHERE token=?1 AND user_id=?2 AND mfa_pending=1 AND mfa_attempts BETWEEN 1 AND 5 AND expires_at>?3 AND ip_address=?4 AND user_agent=?5\0"
                        .as_ptr() as *const ::core::ffi::c_char;
                    if sqlite3_prepare_v2(
                        db,
                        sql,
                        -1 as ::core::ffi::c_int,
                        &raw mut stmt,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    ) != SQLITE_OK
                    {
                        result = -1 as ::core::ffi::c_int;
                        break '_finish;
                    } else {
                        sqlite3_bind_text(
                            stmt,
                            1 as ::core::ffi::c_int,
                            challenge,
                            -1 as ::core::ffi::c_int,
                            SQLITE_STATIC,
                        );
                        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, user_id as sqlite3_int64);
                        sqlite3_bind_int64(
                            stmt,
                            3 as ::core::ffi::c_int,
                            time(::core::ptr::null_mut::<time_t>()) as sqlite3_int64,
                        );
                        sqlite3_bind_text(
                            stmt,
                            4 as ::core::ffi::c_int,
                            if !ip.is_null() {
                                ip
                            } else {
                                b"\0".as_ptr() as *const ::core::ffi::c_char
                            },
                            -1 as ::core::ffi::c_int,
                            SQLITE_STATIC,
                        );
                        sqlite3_bind_text(
                            stmt,
                            5 as ::core::ffi::c_int,
                            if !agent.is_null() {
                                agent
                            } else {
                                b"\0".as_ptr() as *const ::core::ffi::c_char
                            },
                            -1 as ::core::ffi::c_int,
                            SQLITE_STATIC,
                        );
                        let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
                        result = if step == SQLITE_ROW {
                            0 as ::core::ffi::c_int
                        } else if step == SQLITE_DONE {
                            C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0
                        } else {
                            C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0
                        };
                        sqlite3_finalize(stmt);
                        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                        if result != 0 as ::core::ffi::c_int {
                            break '_finish;
                        }
                    }
                } else if db_auth_verify_password(user_id, password) != 0 as ::core::ffi::c_int {
                    result = C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0;
                    break '_finish;
                }
                result = check_account_totp_locked(db, user_id, code, r#false != 0);
                if result == 0 as ::core::ffi::c_int {
                    if !challenge.is_null() {
                        if sqlite3_prepare_v2(
                            db,
                            b"DELETE FROM sessions WHERE token=? AND user_id=? AND mfa_pending=1\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                            &raw mut stmt,
                            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                        ) != SQLITE_OK
                        {
                            result = -1 as ::core::ffi::c_int;
                            break '_finish;
                        } else {
                            sqlite3_bind_text(
                                stmt,
                                1 as ::core::ffi::c_int,
                                challenge,
                                -1 as ::core::ffi::c_int,
                                SQLITE_STATIC,
                            );
                            sqlite3_bind_int64(
                                stmt,
                                2 as ::core::ffi::c_int,
                                user_id as sqlite3_int64,
                            );
                            result = if sqlite3_step(stmt) == SQLITE_DONE
                                && sqlite3_changes(db) == 1 as ::core::ffi::c_int
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            };
                            sqlite3_finalize(stmt);
                            stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                            if result != 0 as ::core::ffi::c_int {
                                break '_finish;
                            }
                        }
                    }
                    result = insert_identity_session_locked(db, user_id, ip, agent, session);
                    if result == 0 as ::core::ffi::c_int
                        && trusted_seconds > 0 as ::core::ffi::c_int
                    {
                        result = db_auth_create_trusted_device(
                            user_id,
                            ip,
                            agent,
                            trusted_seconds,
                            trusted,
                            33 as size_t,
                        );
                    }
                }
            }
        }
        result = finish_identity_transaction(db, result);
    }
    sqlite3_finalize(stmt);
    unlock_auth_database(db);
    if result != 0 as ::core::ffi::c_int {
        *trusted.offset(0isize) = '\0' as ::core::ffi::c_char;
        *session.offset(0isize) = *trusted.offset(0isize);
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_auth_create_password_session(
    mut user_id: int64_t,
    mut password: *const ::core::ffi::c_char,
    mut trusted_token: *const ::core::ffi::c_char,
    mut ip: *const ::core::ffi::c_char,
    mut agent: *const ::core::ffi::c_char,
    mut session: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut needs_mfa: bool = false;
    if !session.is_null() {
        *session.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if session.is_null() || password.is_null() {
        return C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    }
    let mut db: *mut sqlite3 = lock_auth_database();
    if db.is_null() {
        return C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    }
    let mut result: ::core::ffi::c_int = C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        result = identity_source_allowed_locked(db, user_id, ip);
        if result == 0 as ::core::ffi::c_int {
            if db_auth_verify_password(user_id, password) != 0 as ::core::ffi::c_int {
                result = C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0;
            } else if sqlite3_prepare_v2(
                db,
                b"SELECT totp_enabled FROM users WHERE id=?\0".as_ptr()
                    as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                &raw mut stmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ) != SQLITE_OK
            {
                result = -1 as ::core::ffi::c_int;
            } else {
                sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
                if sqlite3_step(stmt) != SQLITE_ROW {
                    result = -1 as ::core::ffi::c_int;
                } else {
                    needs_mfa = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int)
                        != 0 as ::core::ffi::c_int;
                    sqlite3_finalize(stmt);
                    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                    if needs_mfa as ::core::ffi::c_int != 0
                        && (trusted_token.is_null()
                            || *trusted_token == 0
                            || db_auth_validate_trusted_device(user_id, trusted_token)
                                != 0 as ::core::ffi::c_int)
                    {
                        result = C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0;
                    } else {
                        result = insert_identity_session_locked(db, user_id, ip, agent, session);
                    }
                }
            }
        }
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        result = finish_identity_transaction(db, result);
    }
    sqlite3_finalize(stmt);
    unlock_auth_database(db);
    if result != 0 as ::core::ffi::c_int {
        *session.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    return result;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
