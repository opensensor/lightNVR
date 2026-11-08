unsafe extern "C" {
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn event_envelope_validate(
        event: *const event_envelope_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_bus_subscribe(
        name: *const ::core::ffi::c_char,
        handler: event_bus_handler_t,
        context: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn event_bus_unsubscribe(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn event_router_evaluate_delivery(
        event: *const event_envelope_t,
        plan: *mut event_route_delivery_plan_t,
    ) -> event_router_result_t;
    unsafe fn event_router_record_destination_enqueued(
        event: *const event_envelope_t,
        plan: *const event_route_delivery_plan_t,
        destination_key: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn event_route_delivery_plan_clear(plan: *mut event_route_delivery_plan_t);
    unsafe fn event_router_shutdown();
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn mqtt_publish_detection(
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
        timestamp: time_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mqtt_set_motion_state(
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
    );
    unsafe fn mqtt_delivery_worker_enqueue(
        event: *const event_envelope_t,
        topic_prefix: *const ::core::ffi::c_char,
        row_id: *mut int64_t,
    ) -> event_outbox_enqueue_result_t;
    unsafe fn mqtt_delivery_worker_enqueue_destination(
        event: *const event_envelope_t,
        destination: *const ::core::ffi::c_char,
        topic_template: *const ::core::ffi::c_char,
        row_id: *mut int64_t,
    ) -> event_outbox_enqueue_result_t;
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_severity_t(pub ::core::ffi::c_uint);
impl event_severity_t {
    pub const EVENT_SEVERITY_INFO: Self = Self(0);
    pub const EVENT_SEVERITY_WARNING: Self = Self(1);
    pub const EVENT_SEVERITY_ERROR: Self = Self(2);
    pub const EVENT_SEVERITY_CRITICAL: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_envelope_t {
    pub specversion: [::core::ffi::c_char; 4],
    pub id: [::core::ffi::c_char; 37],
    pub r#type: [::core::ffi::c_char; 96],
    pub source: [::core::ffi::c_char; 128],
    pub subject: [::core::ffi::c_char; 96],
    pub time: [::core::ffi::c_char; 32],
    pub datacontenttype: [::core::ffi::c_char; 32],
    pub occurred_at: time_t,
    pub expires_at: time_t,
    pub severity: event_severity_t,
    pub data: *mut cJSON,
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
pub struct event_route_delivery_plan_t {
    pub event_id: [::core::ffi::c_char; 37],
    pub event_type: [::core::ffi::c_char; 96],
    pub subject: [::core::ffi::c_char; 96],
    pub entries: *mut event_route_delivery_plan_entry_t,
    pub count: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_route_delivery_plan_entry_t {
    pub route_uuid: [::core::ffi::c_char; 37],
    pub route_revision: int64_t,
    pub destination_key: [::core::ffi::c_char; 128],
    pub topic_template: [::core::ffi::c_char; 512],
    pub suppression_pending: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_router_result_t(pub ::core::ffi::c_int);
impl event_router_result_t {
    pub const EVENT_ROUTER_ERROR: Self = Self(-1);
    pub const EVENT_ROUTER_NO_MATCH: Self = Self(0);
    pub const EVENT_ROUTER_MATCH: Self = Self(1);
    pub const EVENT_ROUTER_DEFAULT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_outbox_enqueue_result_t(pub ::core::ffi::c_int);
impl event_outbox_enqueue_result_t {
    pub const EVENT_OUTBOX_ENQUEUED: Self = Self(0);
    pub const EVENT_OUTBOX_DUPLICATE: Self = Self(1);
    pub const EVENT_OUTBOX_FULL: Self = Self(2);
    pub const EVENT_OUTBOX_ERROR: Self = Self(-1);
}
pub type event_bus_handler_t = Option<
    unsafe extern "C" fn(*const event_envelope_t, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const EVENT_ROUTE_DEFAULT_DESTINATION: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"mqtt:default\0") };
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MQTT_EVENT_SUBSCRIBER: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"mqtt-compatibility\0")
};
pub const DETECTION_EVENT_TYPE: [::core::ffi::c_char; 32] = unsafe {
    ::core::mem::transmute::<[u8; 32], [::core::ffi::c_char; 32]>(
        *b"io.lightnvr.detection.object.v1\0",
    )
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_event_adapter_decode_detection(
    mut event: *const event_envelope_t,
    mut stream_name: *mut ::core::ffi::c_char,
    mut stream_name_size: size_t,
    mut result: *mut detection_result_t,
) -> ::core::ffi::c_int {
    if event.is_null()
        || strcmp(
            &raw const (*event).r#type as *const ::core::ffi::c_char,
            DETECTION_EVENT_TYPE.as_ptr(),
        ) != 0 as ::core::ffi::c_int
        || stream_name.is_null()
        || stream_name_size == 0 as size_t
        || result.is_null()
        || event_envelope_validate(
            event,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut name: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        (*event).data,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut count: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        (*event).data,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut detections: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        (*event).data,
        b"detections\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsString(name) == 0
        || *(*name).valuestring.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strlen((*name).valuestring) >= stream_name_size
        || cJSON_IsNumber(count) == 0
        || (*count).valuedouble != (*count).valueint as ::core::ffi::c_double
        || (*count).valueint <= 0 as ::core::ffi::c_int
        || (*count).valueint > MAX_DETECTIONS
        || cJSON_IsArray(detections) == 0
        || cJSON_GetArraySize(detections) != (*count).valueint
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    safe_strcpy(
        stream_name,
        (*name).valuestring,
        stream_name_size,
        0 as size_t,
    );
    let mut item: *const cJSON = ::core::ptr::null::<cJSON>();
    item = if !detections.is_null() {
        (*detections).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !item.is_null() {
        let mut destination: *mut detection_t =
            (&raw mut (*result).detections as *mut detection_t).offset((*result).count as isize);
        let mut label: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"label\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut confidence: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsString(label) == 0 || cJSON_IsNumber(confidence) == 0 {
            return -1 as ::core::ffi::c_int;
        }
        safe_strcpy(
            &raw mut (*destination).label as *mut ::core::ffi::c_char,
            (*label).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            0 as size_t,
        );
        (*destination).confidence = (*confidence).valuedouble as ::core::ffi::c_float;
        (*destination).track_id = -1 as ::core::ffi::c_int;
        let mut x: *const cJSON =
            cJSON_GetObjectItemCaseSensitive(item, b"x\0".as_ptr() as *const ::core::ffi::c_char);
        let mut y: *const cJSON =
            cJSON_GetObjectItemCaseSensitive(item, b"y\0".as_ptr() as *const ::core::ffi::c_char);
        let mut width: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"width\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut height: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"height\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !x.is_null() {
            (*destination).x = (*x).valuedouble as ::core::ffi::c_float;
        }
        if !y.is_null() {
            (*destination).y = (*y).valuedouble as ::core::ffi::c_float;
        }
        if !width.is_null() {
            (*destination).width = (*width).valuedouble as ::core::ffi::c_float;
        }
        if !height.is_null() {
            (*destination).height = (*height).valuedouble as ::core::ffi::c_float;
        }
        let mut track_id: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"track_id\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsNumber(track_id) != 0
            && (*track_id).valuedouble >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*track_id).valuedouble <= INT_MAX as ::core::ffi::c_double
        {
            (*destination).track_id = (*track_id).valueint;
        }
        let mut zone_id: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"zone_id\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsString(zone_id) != 0 {
            safe_strcpy(
                &raw mut (*destination).zone_id as *mut ::core::ffi::c_char,
                (*zone_id).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                0 as size_t,
            );
        }
        (*result).count += 1;
        item = (*item).next;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn mqtt_event_handler(
    mut event: *const event_envelope_t,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut config: *const config_t = context as *const config_t;
    let mut failed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !config.is_null() {
        let mut delivery_plan: event_route_delivery_plan_t = event_route_delivery_plan_t {
            event_id: [
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
            event_type: [0; 96],
            subject: [0; 96],
            entries: ::core::ptr::null_mut::<event_route_delivery_plan_entry_t>(),
            count: 0,
            capacity: 0,
        };
        let mut route_result: event_router_result_t =
            event_router_evaluate_delivery(event, &raw mut delivery_plan);
        if route_result.0 == event_router_result_t::EVENT_ROUTER_DEFAULT.0
            && (*config).mqtt_enabled as ::core::ffi::c_int != 0
        {
            let mut enqueue_result: event_outbox_enqueue_result_t = mqtt_delivery_worker_enqueue(
                event,
                &raw const (*config).mqtt_topic_prefix as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<int64_t>(),
            );
            if enqueue_result.0 != event_outbox_enqueue_result_t::EVENT_OUTBOX_ENQUEUED.0
                && enqueue_result.0 != event_outbox_enqueue_result_t::EVENT_OUTBOX_DUPLICATE.0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"MQTT event adapter could not persist event %s (%d)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw const (*event).id as *const ::core::ffi::c_char,
                    enqueue_result.0,
                );
                failed = 1 as ::core::ffi::c_int;
            }
        } else if route_result.0 == event_router_result_t::EVENT_ROUTER_MATCH.0 {
            let mut index: size_t = 0 as size_t;
            while index < delivery_plan.count {
                let mut entry: *const event_route_delivery_plan_entry_t =
                    delivery_plan.entries.offset(index as isize);
                let mut already_processed: bool = r#false != 0;
                let mut previous: size_t = 0 as size_t;
                while previous < index {
                    if strcmp(
                        &raw mut (*delivery_plan.entries.offset(previous as isize)).destination_key
                            as *mut ::core::ffi::c_char,
                        &raw const (*entry).destination_key as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        already_processed = r#true != 0;
                        break;
                    } else {
                        previous = previous.wrapping_add(1);
                    }
                }
                's_46: {
                    if !already_processed {
                        let mut enqueue_result_0: event_outbox_enqueue_result_t =
                            event_outbox_enqueue_result_t::EVENT_OUTBOX_ENQUEUED;
                        if strcmp(
                            &raw const (*entry).destination_key as *const ::core::ffi::c_char,
                            EVENT_ROUTE_DEFAULT_DESTINATION.as_ptr(),
                        ) == 0 as ::core::ffi::c_int
                        {
                            if !(*config).mqtt_enabled {
                                break 's_46;
                            } else {
                                enqueue_result_0 = mqtt_delivery_worker_enqueue(
                                    event,
                                    &raw const (*config).mqtt_topic_prefix
                                        as *const ::core::ffi::c_char,
                                    ::core::ptr::null_mut::<int64_t>(),
                                );
                            }
                        } else {
                            enqueue_result_0 = mqtt_delivery_worker_enqueue_destination(
                                event,
                                &raw const (*entry).destination_key as *const ::core::ffi::c_char,
                                &raw const (*entry).topic_template as *const ::core::ffi::c_char,
                                ::core::ptr::null_mut::<int64_t>(),
                            );
                        }
                        if enqueue_result_0.0
                            != event_outbox_enqueue_result_t::EVENT_OUTBOX_ENQUEUED.0
                            && enqueue_result_0.0
                                != event_outbox_enqueue_result_t::EVENT_OUTBOX_DUPLICATE.0
                        {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"MQTT event adapter could not persist event %s for %s (%d)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw const (*event).id as *const ::core::ffi::c_char,
                                &raw const (*entry).destination_key as *const ::core::ffi::c_char,
                                enqueue_result_0.0,
                            );
                            failed = 1 as ::core::ffi::c_int;
                        } else if event_router_record_destination_enqueued(
                            event,
                            &raw mut delivery_plan,
                            &raw const (*entry).destination_key as *const ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int
                        {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"MQTT event adapter could not commit route suppression for event %s and %s\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                &raw const (*event).id as *const ::core::ffi::c_char,
                                &raw const (*entry).destination_key
                                    as *const ::core::ffi::c_char,
                            );
                            failed = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                index = index.wrapping_add(1);
            }
        } else if route_result.0 == event_router_result_t::EVENT_ROUTER_ERROR.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT event adapter could not evaluate routes for %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw const (*event).id as *const ::core::ffi::c_char,
            );
            failed = 1 as ::core::ffi::c_int;
        }
        event_route_delivery_plan_clear(&raw mut delivery_plan);
    }
    if strcmp(
        &raw const (*event).r#type as *const ::core::ffi::c_char,
        DETECTION_EVENT_TYPE.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
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
        if mqtt_event_adapter_decode_detection(
            event,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            &raw mut result,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"MQTT event adapter rejected detection event %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw const (*event).id as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if mqtt_publish_detection(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            &raw mut result,
            (*event).occurred_at,
        ) != 0 as ::core::ffi::c_int
        {
            failed = 1 as ::core::ffi::c_int;
        }
        mqtt_set_motion_state(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            &raw mut result,
        );
    }
    return if failed != 0 {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_event_adapter_register(
    mut config: *const config_t,
) -> ::core::ffi::c_int {
    if config.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return event_bus_subscribe(
        MQTT_EVENT_SUBSCRIBER.as_ptr(),
        Some(
            mqtt_event_handler
                as unsafe extern "C" fn(
                    *const event_envelope_t,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        config as *mut ::core::ffi::c_void,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_event_adapter_unregister() -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = event_bus_unsubscribe(MQTT_EVENT_SUBSCRIBER.as_ptr());
    event_router_shutdown();
    return result;
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
