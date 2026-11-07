unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn system_health_scope_name(scope: system_health_scope_t) -> *const ::core::ffi::c_char;
    unsafe fn system_health_capability_name(
        capability: system_health_capability_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn system_health_severity_name(
        severity: system_health_severity_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn http_response_set_body(
        res: *mut http_response_t,
        body: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_set_json_error(
        res: *mut http_response_t,
        status_code: ::core::ffi::c_int,
        error_message: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_global_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_stream_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn metrics_snapshot_all(
        out_array: *mut stream_metrics_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn metrics_get_max_streams() -> ::core::ffi::c_int;
    unsafe fn player_telemetry_record(event: *const player_telemetry_event_t);
    unsafe fn recording_io_metrics_snapshot(snapshot: *mut recording_io_metrics_snapshot_t);
    unsafe fn recording_io_reason_name(reason: recording_io_reason_t)
    -> *const ::core::ffi::c_char;
    unsafe fn system_health_snapshot_copy(snapshot: *mut system_health_snapshot_t) -> bool;
    unsafe fn system_health_get_stats(stats: *mut system_health_stats_t);
    unsafe fn system_health_collector_stats_copy(
        stats: *mut system_health_collector_stats_t,
        capacity: size_t,
    ) -> size_t;
    unsafe fn system_health_evaluator_service_active_copy(
        incidents: *mut system_health_incident_view_t,
        capacity: size_t,
    ) -> size_t;
    unsafe fn system_health_evaluator_service_get_stats(
        stats: *mut system_health_evaluator_stats_t,
    );
    unsafe fn system_health_evaluator_service_copy_run(
        run: *mut system_health_process_run_t,
    ) -> bool;
    unsafe fn event_bus_get_stats(stats: *mut event_bus_stats_t);
    unsafe fn db_event_destination_list(
        destinations: *mut event_destination_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_destination_make_key(
        uuid: *const ::core::ffi::c_char,
        key: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn event_router_get_stats(stats: *mut event_router_stats_t);
    unsafe fn db_event_outbox_get_stats(
        destination: *const ::core::ffi::c_char,
        now: int64_t,
        stats: *mut event_outbox_stats_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mqtt_delivery_worker_get_stats(stats: *mut mqtt_delivery_worker_stats_t);
    unsafe fn mqtt_destination_client_list_stats(
        stats: *mut mqtt_destination_client_stats_t,
        capacity: size_t,
    ) -> size_t;
    unsafe fn mqtt_presence_get_stats(stats: *mut mqtt_presence_stats_t);
    unsafe fn get_total_stream_count() -> ::core::ffi::c_int;
    unsafe fn get_storage_health(health: *mut storage_health_t) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::core::ffi::c_uint,
    pub __writers: ::core::ffi::c_uint,
    pub __wrphase_futex: ::core::ffi::c_uint,
    pub __writers_futex: ::core::ffi::c_uint,
    pub __pad3: ::core::ffi::c_uint,
    pub __pad4: ::core::ffi::c_uint,
    pub __cur_writer: ::core::ffi::c_int,
    pub __shared: ::core::ffi::c_int,
    pub __pad1: ::core::ffi::c_ulong,
    pub __pad2: ::core::ffi::c_ulong,
    pub __flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
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
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type int_fast64_t = ::core::ffi::c_long;
pub type uint_fast64_t = ::core::ffi::c_ulong;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_scope_t(pub ::core::ffi::c_uint);
impl system_health_scope_t {
    pub const SYSTEM_HEALTH_SCOPE_PROCESS: Self = Self(0);
    pub const SYSTEM_HEALTH_SCOPE_CONTAINER: Self = Self(1);
    pub const SYSTEM_HEALTH_SCOPE_HOST: Self = Self(2);
    pub const SYSTEM_HEALTH_SCOPE_FILESYSTEM: Self = Self(3);
    pub const SYSTEM_HEALTH_SCOPE_DEVICE: Self = Self(4);
    pub const SYSTEM_HEALTH_SCOPE_COUNT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_capability_t(pub ::core::ffi::c_uint);
impl system_health_capability_t {
    pub const SYSTEM_HEALTH_CAPABILITY_AVAILABLE: Self = Self(0);
    pub const SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED: Self = Self(1);
    pub const SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED: Self = Self(2);
    pub const SYSTEM_HEALTH_CAPABILITY_STALE: Self = Self(3);
    pub const SYSTEM_HEALTH_CAPABILITY_ERROR: Self = Self(4);
    pub const SYSTEM_HEALTH_CAPABILITY_COUNT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_freshness_t(pub ::core::ffi::c_uint);
impl system_health_freshness_t {
    pub const SYSTEM_HEALTH_FRESHNESS_UNKNOWN: Self = Self(0);
    pub const SYSTEM_HEALTH_FRESHNESS_FRESH: Self = Self(1);
    pub const SYSTEM_HEALTH_FRESHNESS_STALE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_unit_t(pub ::core::ffi::c_uint);
impl system_health_unit_t {
    pub const SYSTEM_HEALTH_UNIT_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_UNIT_RATIO: Self = Self(1);
    pub const SYSTEM_HEALTH_UNIT_BYTES: Self = Self(2);
    pub const SYSTEM_HEALTH_UNIT_COUNT: Self = Self(3);
    pub const SYSTEM_HEALTH_UNIT_SECONDS: Self = Self(4);
    pub const SYSTEM_HEALTH_UNIT_CELSIUS: Self = Self(5);
    pub const SYSTEM_HEALTH_UNIT_HERTZ: Self = Self(6);
    pub const SYSTEM_HEALTH_UNIT_BOOLEAN: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_severity_t(pub ::core::ffi::c_uint);
impl system_health_severity_t {
    pub const SYSTEM_HEALTH_SEVERITY_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_SEVERITY_WARNING: Self = Self(1);
    pub const SYSTEM_HEALTH_SEVERITY_ERROR: Self = Self(2);
    pub const SYSTEM_HEALTH_SEVERITY_CRITICAL: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_state_t(pub ::core::ffi::c_uint);
impl system_health_state_t {
    pub const SYSTEM_HEALTH_STATE_UNKNOWN: Self = Self(0);
    pub const SYSTEM_HEALTH_STATE_HEALTHY: Self = Self(1);
    pub const SYSTEM_HEALTH_STATE_PENDING: Self = Self(2);
    pub const SYSTEM_HEALTH_STATE_OPEN: Self = Self(3);
    pub const SYSTEM_HEALTH_STATE_RECOVERING: Self = Self(4);
    pub const SYSTEM_HEALTH_STATE_CLOSED: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_condition_t(pub ::core::ffi::c_uint);
impl system_health_condition_t {
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW: Self = Self(0);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL: Self = Self(1);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_SWAP_THRASH: Self = Self(2);
    pub const SYSTEM_HEALTH_CONDITION_CPU_SATURATION: Self = Self(3);
    pub const SYSTEM_HEALTH_CONDITION_CPU_THROTTLED: Self = Self(4);
    pub const SYSTEM_HEALTH_CONDITION_IO_PRESSURE: Self = Self(5);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW: Self = Self(6);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW: Self = Self(7);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY: Self = Self(8);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED: Self = Self(9);
    pub const SYSTEM_HEALTH_CONDITION_THERMAL_HIGH: Self = Self(10);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN: Self = Self(11);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE: Self = Self(12);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED: Self = Self(13);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_JUMP: Self = Self(14);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION: Self = Self(15);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION: Self = Self(16);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED: Self = Self(17);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL: Self = Self(18);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL: Self = Self(19);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_CORRECTED: Self = Self(20);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE: Self = Self(21);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED: Self = Self(22);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_POWER_UNSTABLE: Self = Self(23);
    pub const SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE: Self = Self(24);
    pub const SYSTEM_HEALTH_CONDITION_UNEXPECTED_RESTART: Self = Self(25);
    pub const SYSTEM_HEALTH_CONDITION_EVENT_DELIVERY_DEGRADED: Self = Self(26);
    pub const SYSTEM_HEALTH_CONDITION_COUNT: Self = Self(27);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_observation_t {
    pub metric: [::core::ffi::c_char; 64],
    pub resource_id: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub capability: system_health_capability_t,
    pub freshness: system_health_freshness_t,
    pub unit: system_health_unit_t,
    pub value_valid: bool,
    pub value: ::core::ffi::c_double,
    pub sampled_monotonic_ms: uint64_t,
    pub observed_wall_time_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_snapshot_t {
    pub observations: [system_health_observation_t; 256],
    pub observation_count: size_t,
    pub observations_dropped: size_t,
    pub sequence: uint64_t,
    pub completed_monotonic_ms: uint64_t,
    pub completed_wall_time_ms: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct http_method_t(pub ::core::ffi::c_uint);
impl http_method_t {
    pub const HTTP_METHOD_GET: Self = Self(0);
    pub const HTTP_METHOD_POST: Self = Self(1);
    pub const HTTP_METHOD_PUT: Self = Self(2);
    pub const HTTP_METHOD_DELETE: Self = Self(3);
    pub const HTTP_METHOD_OPTIONS: Self = Self(4);
    pub const HTTP_METHOD_HEAD: Self = Self(5);
    pub const HTTP_METHOD_PATCH: Self = Self(6);
    pub const HTTP_METHOD_UNKNOWN: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_header_t {
    pub name: [::core::ffi::c_char; 128],
    pub value: [::core::ffi::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_request_t {
    pub method: http_method_t,
    pub method_str: [::core::ffi::c_char; 16],
    pub path: [::core::ffi::c_char; 512],
    pub uri: [::core::ffi::c_char; 1536],
    pub query_string: [::core::ffi::c_char; 1024],
    pub content_type: [::core::ffi::c_char; 128],
    pub content_length: uint64_t,
    pub user_agent: [::core::ffi::c_char; 256],
    pub body: *mut ::core::ffi::c_void,
    pub body_len: size_t,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub client_ip: [::core::ffi::c_char; 64],
    pub request_id: [::core::ffi::c_char; 65],
    pub user_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_response_t {
    pub status_code: ::core::ffi::c_int,
    pub content_type: [::core::ffi::c_char; 128],
    pub body_length: size_t,
    pub body: *mut ::core::ffi::c_void,
    pub body_allocated: bool,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub user_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_metrics_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub active: bool,
    pub health_status: atomic_int,
    pub stream_up: atomic_int,
    pub health_observed: bool,
    pub health_changed_at: time_t,
    pub last_packet_ts: atomic_int_fast64_t,
    pub last_frame_ts: atomic_int_fast64_t,
    pub current_fps: ::core::ffi::c_double,
    pub current_bitrate_bps: ::core::ffi::c_double,
    pub connection_latency_ms: ::core::ffi::c_double,
    pub configured_fps: ::core::ffi::c_double,
    pub frames_total: atomic_uint_fast64_t,
    pub frames_dropped: atomic_uint_fast64_t,
    pub bytes_received: atomic_uint_fast64_t,
    pub reconnects_total: atomic_uint_fast64_t,
    pub uptime_seconds: atomic_uint_fast64_t,
    pub error_decode: atomic_uint_fast64_t,
    pub error_timeout: atomic_uint_fast64_t,
    pub error_protocol: atomic_uint_fast64_t,
    pub error_io: atomic_uint_fast64_t,
    pub recording_active: atomic_int,
    pub recording_bytes_written: atomic_uint_fast64_t,
    pub recording_segments_total: atomic_uint_fast64_t,
    pub recording_gaps_total: atomic_uint_fast64_t,
    pub last_completed_segment_ts: atomic_int_fast64_t,
    pub ring: [metrics_ring_sample_t; 720],
    pub ring_head: ::core::ffi::c_int,
    pub ring_count: ::core::ffi::c_int,
    pub last_segment_end_time: time_t,
    pub expected_segment_duration: ::core::ffi::c_int,
    pub observation_persisted_at: time_t,
    pub stream_start_time: time_t,
    pub source_frames_total: [atomic_uint_fast64_t; 4],
    pub source_bytes_received: [atomic_uint_fast64_t; 4],
    pub prev_source_frames: [uint64_t; 4],
    pub prev_source_bytes: [uint64_t; 4],
    pub prev_sample_time: time_t,
    pub lock: pthread_rwlock_t,
}
pub type atomic_uint_fast64_t = uint_fast64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct metrics_ring_sample_t {
    pub fps: ::core::ffi::c_float,
    pub bitrate_kbps: ::core::ffi::c_float,
    pub timestamp: time_t,
}
pub type atomic_int_fast64_t = int_fast64_t;
pub type atomic_int = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prom_buf_t {
    pub data: *mut ::core::ffi::c_char,
    pub len: size_t,
    pub cap: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_evaluator_stats_t {
    pub tracked_conditions: size_t,
    pub active_incidents: size_t,
    pub pending_persistence: size_t,
    pub transitions: uint64_t,
    pub persistence_failures: uint64_t,
    pub persistence_retries: uint64_t,
    pub oldest_pending_wall_time_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mqtt_presence_stats_t {
    pub configured: bool,
    pub connected: bool,
    pub sequence: uint64_t,
    pub connections: uint64_t,
    pub reconnects: uint64_t,
    pub disconnects: uint64_t,
    pub publish_attempts: uint64_t,
    pub publish_successes: uint64_t,
    pub publish_failures: uint64_t,
    pub last_publish_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_outbox_stats_t {
    pub total_rows: int64_t,
    pub total_bytes: int64_t,
    pub pending_rows: int64_t,
    pub delivering_rows: int64_t,
    pub delivered_rows: int64_t,
    pub dead_rows: int64_t,
    pub due_rows: int64_t,
    pub oldest_pending_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mqtt_destination_client_stats_t {
    pub destination_uuid: [::core::ffi::c_char; 37],
    pub presence_configured: bool,
    pub connected: bool,
    pub connections: uint64_t,
    pub reconnects: uint64_t,
    pub disconnects: uint64_t,
    pub connection_failures: uint64_t,
    pub publish_successes: uint64_t,
    pub publish_failures: uint64_t,
    pub last_success_at_ms: int64_t,
    pub last_failure_at_ms: int64_t,
    pub last_failure: mqtt_destination_failure_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mqtt_destination_failure_t(pub ::core::ffi::c_uint);
impl mqtt_destination_failure_t {
    pub const MQTT_DESTINATION_FAILURE_NONE: Self = Self(0);
    pub const MQTT_DESTINATION_FAILURE_CONFIGURATION: Self = Self(1);
    pub const MQTT_DESTINATION_FAILURE_CONNECTION: Self = Self(2);
    pub const MQTT_DESTINATION_FAILURE_PUBLICATION: Self = Self(3);
    pub const MQTT_DESTINATION_FAILURE_COUNT: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_destination_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub destination_type: [::core::ffi::c_char; 16],
    pub broker_host: [::core::ffi::c_char; 256],
    pub broker_port: ::core::ffi::c_int,
    pub client_id: [::core::ffi::c_char; 128],
    pub topic_template: [::core::ffi::c_char; 512],
    pub status_topic_template: [::core::ffi::c_char; 512],
    pub username: [::core::ffi::c_char; 128],
    pub password_configured: bool,
    pub tls_mode: [::core::ffi::c_char; 16],
    pub ca_file: [::core::ffi::c_char; 512],
    pub cert_file: [::core::ffi::c_char; 512],
    pub key_file: [::core::ffi::c_char; 512],
    pub keepalive_seconds: ::core::ffi::c_int,
    pub qos: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mqtt_delivery_worker_stats_t {
    pub enqueued: uint64_t,
    pub duplicates: uint64_t,
    pub rejected_full: uint64_t,
    pub enqueue_errors: uint64_t,
    pub priority_shed: uint64_t,
    pub expired: uint64_t,
    pub attempted: uint64_t,
    pub delivered: uint64_t,
    pub retried: uint64_t,
    pub dead: uint64_t,
    pub outcome_errors: uint64_t,
    pub disconnected_polls: uint64_t,
    pub managed_profiles: uint64_t,
    pub managed_connected: uint64_t,
    pub profile_reloads: uint64_t,
    pub profile_errors: uint64_t,
    pub pruned: uint64_t,
    pub running: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_router_stats_t {
    pub cache_reloads: uint64_t,
    pub events_evaluated: uint64_t,
    pub default_events: uint64_t,
    pub matched_events: uint64_t,
    pub unmatched_events: uint64_t,
    pub evaluation_errors: uint64_t,
    pub routes_considered: uint64_t,
    pub type_rejections: uint64_t,
    pub scope_rejections: uint64_t,
    pub predicate_rejections: uint64_t,
    pub schedule_rejections: uint64_t,
    pub destination_disabled_rejections: uint64_t,
    pub debounce_suppressions: uint64_t,
    pub cooldown_suppressions: uint64_t,
    pub grouping_suppressions: uint64_t,
    pub rate_suppressions: uint64_t,
    pub suppression_errors: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_bus_stats_t {
    pub accepted_events: uint64_t,
    pub dispatched_events: uint64_t,
    pub callback_deliveries: uint64_t,
    pub handler_failures: uint64_t,
    pub dropped_events: uint64_t,
    pub priority_shed_events: uint64_t,
    pub rejected_events: uint64_t,
    pub queued_events: size_t,
    pub queued_bytes: size_t,
    pub active_dispatches: size_t,
    pub max_events: size_t,
    pub max_bytes: size_t,
    pub subscriber_count: ::core::ffi::c_int,
    pub running: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_stats_t {
    pub initialized: bool,
    pub enabled: bool,
    pub ring_allocated: bool,
    pub worker_threads: uint32_t,
    pub generations_completed: uint64_t,
    pub collections_completed: uint64_t,
    pub collection_errors: uint64_t,
    pub collection_timeouts: uint64_t,
    pub overlap_skips: uint64_t,
    pub observations_dropped: uint64_t,
    pub coverage_overflows: uint64_t,
    pub abandoned_helpers: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_collector_stats_t {
    pub name: [::core::ffi::c_char; 32],
    pub scope: system_health_scope_t,
    pub tier: system_health_sampling_tier_t,
    pub interval_seconds: uint32_t,
    pub stale_after_seconds: uint32_t,
    pub attempts: uint64_t,
    pub completions: uint64_t,
    pub failures: uint64_t,
    pub timeouts: uint64_t,
    pub overlap_skips: uint64_t,
    pub last_duration_ms: uint64_t,
    pub maximum_duration_ms: uint64_t,
    pub last_attempt_monotonic_ms: uint64_t,
    pub last_success_monotonic_ms: uint64_t,
    pub busy: bool,
    pub stale: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_sampling_tier_t(pub ::core::ffi::c_uint);
impl system_health_sampling_tier_t {
    pub const SYSTEM_HEALTH_TIER_FAST: Self = Self(0);
    pub const SYSTEM_HEALTH_TIER_NORMAL: Self = Self(1);
    pub const SYSTEM_HEALTH_TIER_SLOW: Self = Self(2);
    pub const SYSTEM_HEALTH_TIER_DEVICE: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_process_run_t {
    pub run_id: [::core::ffi::c_char; 37],
    pub boot_id: [::core::ffi::c_char; 64],
    pub started_at_ms: int64_t,
    pub closed_at_ms: int64_t,
    pub clean_close: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_io_metrics_snapshot_t {
    pub reason_totals: [[uint64_t; 9]; 2],
    pub last_error: recording_io_last_error_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_io_last_error_t {
    pub valid: bool,
    pub resource: recording_io_resource_t,
    pub operation: recording_io_operation_t,
    pub reason: recording_io_reason_t,
    pub error_code: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct recording_io_reason_t(pub ::core::ffi::c_uint);
impl recording_io_reason_t {
    pub const RECORDING_IO_REASON_NONE: Self = Self(0);
    pub const RECORDING_IO_REASON_NO_SPACE: Self = Self(1);
    pub const RECORDING_IO_REASON_QUOTA: Self = Self(2);
    pub const RECORDING_IO_REASON_READ_ONLY: Self = Self(3);
    pub const RECORDING_IO_REASON_IO: Self = Self(4);
    pub const RECORDING_IO_REASON_TIMEOUT: Self = Self(5);
    pub const RECORDING_IO_REASON_FD_LIMIT: Self = Self(6);
    pub const RECORDING_IO_REASON_ALLOCATION: Self = Self(7);
    pub const RECORDING_IO_REASON_OTHER: Self = Self(8);
    pub const RECORDING_IO_REASON_COUNT: Self = Self(9);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_view_t {
    pub incident_id: [::core::ffi::c_char; 37],
    pub condition: system_health_condition_t,
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub state: system_health_state_t,
    pub severity: system_health_severity_t,
    pub first_observed_at_ms: int64_t,
    pub last_observed_at_ms: int64_t,
    pub observation: system_health_observation_t,
    pub persistence_pending: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct collector_export_t {
    pub name: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub seen: bool,
    pub up: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_health_t {
    pub pressure_level: disk_pressure_level_t,
    pub free_space_pct: ::core::ffi::c_double,
    pub free_space_bytes: uint64_t,
    pub total_space_bytes: uint64_t,
    pub used_space_bytes: uint64_t,
    pub last_check_time: time_t,
    pub last_cleanup_time: time_t,
    pub last_deep_time: time_t,
    pub last_cleanup_deleted: ::core::ffi::c_int,
    pub last_cleanup_freed: uint64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct disk_pressure_level_t(pub ::core::ffi::c_uint);
impl disk_pressure_level_t {
    pub const DISK_PRESSURE_NORMAL: Self = Self(0);
    pub const DISK_PRESSURE_WARNING: Self = Self(1);
    pub const DISK_PRESSURE_CRITICAL: Self = Self(2);
    pub const DISK_PRESSURE_EMERGENCY: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_action_t(pub ::core::ffi::c_int);
impl authorization_action_t {
    pub const AUTHZ_ACTION_INVALID: Self = Self(-1);
    pub const AUTHZ_LIVE_VIEW: Self = Self(0);
    pub const AUTHZ_AUDIO_LISTEN: Self = Self(1);
    pub const AUTHZ_AUDIO_TALK: Self = Self(2);
    pub const AUTHZ_RECORDINGS_REPLAY: Self = Self(3);
    pub const AUTHZ_RECORDINGS_EXPORT: Self = Self(4);
    pub const AUTHZ_SNAPSHOT_CREATE: Self = Self(5);
    pub const AUTHZ_PTZ_CONTROL: Self = Self(6);
    pub const AUTHZ_EVIDENCE_PROTECT: Self = Self(7);
    pub const AUTHZ_RECORDING_DELETE: Self = Self(8);
    pub const AUTHZ_CAMERA_CONFIGURE: Self = Self(9);
    pub const AUTHZ_FLEET_EXECUTE_JOB: Self = Self(10);
    pub const AUTHZ_STORAGE_CONFIGURE: Self = Self(11);
    pub const AUTHZ_EVENTS_CONFIGURE: Self = Self(12);
    pub const AUTHZ_USERS_MANAGE: Self = Self(13);
    pub const AUTHZ_SYSTEM_ADMIN: Self = Self(14);
    pub const AUTHZ_LPR_READ: Self = Self(15);
    pub const AUTHZ_LPR_SEARCH: Self = Self(16);
    pub const AUTHZ_LPR_EXPORT: Self = Self(17);
    pub const AUTHZ_LPR_DELETE: Self = Self(18);
    pub const AUTHZ_ACTION_COUNT: Self = Self(19);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_telemetry_event_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub session_id: [::core::ffi::c_char; 64],
    pub transport: [::core::ffi::c_char; 16],
    pub ttff_ms: ::core::ffi::c_double,
    pub rebuffer_count: ::core::ffi::c_int,
    pub rebuffer_duration_ms: ::core::ffi::c_double,
    pub resolution_switches: ::core::ffi::c_int,
    pub webrtc_rtt_ms: ::core::ffi::c_double,
    pub timestamp: time_t,
}
pub const SYSTEM_HEALTH_MAX_INCIDENTS: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_MAX_COLLECTORS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const EVENT_DESTINATION_MAX_COUNT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_DEFAULT_MAX_ROWS: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_DEFAULT_MAX_BYTES: ::core::ffi::c_longlong = 64 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong;
pub const MQTT_EVENT_OUTBOX_DESTINATION: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"mqtt:default\0") };
unsafe extern "C" fn prom_buf_init(mut b: *mut prom_buf_t, mut initial_cap: size_t) {
    (*b).data = malloc(initial_cap) as *mut ::core::ffi::c_char;
    (*b).len = 0 as size_t;
    (*b).cap = initial_cap;
    if !(*b).data.is_null() {
        *(*b).data.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn prom_buf_append(
    mut b: *mut prom_buf_t,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    if (*b).data.is_null() {
        return;
    }
    let mut ap: ::core::ffi::VaList;
    ap = c2rust_args.clone();
    let mut needed: ::core::ffi::c_int = vsnprintf(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
        fmt,
        ap.clone(),
    );
    if needed < 0 as ::core::ffi::c_int {
        return;
    }
    while (*b)
        .len
        .wrapping_add(needed as size_t)
        .wrapping_add(1 as size_t)
        > (*b).cap
    {
        let mut new_cap: size_t = (*b).cap.wrapping_mul(2 as size_t);
        let mut new_data: *mut ::core::ffi::c_char =
            realloc((*b).data as *mut ::core::ffi::c_void, new_cap) as *mut ::core::ffi::c_char;
        if new_data.is_null() {
            return;
        }
        (*b).data = new_data;
        (*b).cap = new_cap;
    }
    ap = c2rust_args.clone();
    vsnprintf(
        (*b).data.offset((*b).len as isize),
        (*b).cap.wrapping_sub((*b).len),
        fmt,
        ap.clone(),
    );
    (*b).len = (*b).len.wrapping_add(needed as size_t);
}
unsafe extern "C" fn prom_buf_free(mut b: *mut prom_buf_t) {
    free((*b).data as *mut ::core::ffi::c_void);
    (*b).data = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*b).cap = 0 as size_t;
    (*b).len = (*b).cap;
}
unsafe extern "C" fn mqtt_failure_label(
    mut failure: mqtt_destination_failure_t,
) -> *const ::core::ffi::c_char {
    match failure {
        mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_CONFIGURATION => {
            return b"configuration\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_CONNECTION => {
            return b"connection\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_PUBLICATION => {
            return b"publication\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_COUNT | _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn append_outbox_metrics(
    mut buf: *mut prom_buf_t,
    mut label: *const ::core::ffi::c_char,
    mut destination: *const ::core::ffi::c_char,
    mut now: int64_t,
    mut aggregate: *mut event_outbox_stats_t,
) -> bool {
    let mut stats: event_outbox_stats_t = event_outbox_stats_t {
        total_rows: 0,
        total_bytes: 0,
        pending_rows: 0,
        delivering_rows: 0,
        delivered_rows: 0,
        dead_rows: 0,
        due_rows: 0,
        oldest_pending_at: 0,
    };
    if db_event_outbox_get_stats(destination, now, &raw mut stats) != 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    if !aggregate.is_null() {
        *aggregate = stats;
    }
    let rows: [int64_t; 5] = [
        stats.pending_rows,
        stats.delivering_rows,
        stats.delivered_rows,
        stats.dead_rows,
        stats.due_rows,
    ];
    let mut states: [*const ::core::ffi::c_char; 5] = [
        b"pending\0".as_ptr() as *const ::core::ffi::c_char,
        b"delivering\0".as_ptr() as *const ::core::ffi::c_char,
        b"delivered\0".as_ptr() as *const ::core::ffi::c_char,
        b"dead\0".as_ptr() as *const ::core::ffi::c_char,
        b"due\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut index: size_t = 0 as size_t;
    while index < 5 as size_t {
        prom_buf_append(
            buf,
            b"lightnvr_event_outbox_rows{destination=\"%s\",state=\"%s\"} %lld\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            label,
            states[index],
            rows[index] as ::core::ffi::c_longlong,
        );
        index = index.wrapping_add(1);
    }
    prom_buf_append(
        buf,
        b"lightnvr_event_outbox_bytes{destination=\"%s\"} %lld\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        label,
        stats.total_bytes as ::core::ffi::c_longlong,
    );
    let mut oldest_age: int64_t = if stats.pending_rows > 0 as int64_t
        && stats.oldest_pending_at > 0 as int64_t
        && now >= stats.oldest_pending_at
    {
        now - stats.oldest_pending_at
    } else {
        0 as int64_t
    };
    prom_buf_append(
        buf,
        b"lightnvr_event_outbox_oldest_pending_age_seconds{destination=\"%s\"} %lld\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        label,
        oldest_age as ::core::ffi::c_longlong,
    );
    return r#true != 0;
}
unsafe extern "C" fn find_mqtt_destination_stats(
    mut stats: *const mqtt_destination_client_stats_t,
    mut count: size_t,
    mut uuid: *const ::core::ffi::c_char,
) -> *const mqtt_destination_client_stats_t {
    let mut index: size_t = 0 as size_t;
    while index < count {
        if strcmp(
            &raw const (*stats.offset(index as isize)).destination_uuid
                as *const ::core::ffi::c_char,
            uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return stats.offset(index as isize);
        }
        index = index.wrapping_add(1);
    }
    return ::core::ptr::null::<mqtt_destination_client_stats_t>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_metrics_render_self_observability() -> *mut ::core::ffi::c_char {
    let mut buffer: prom_buf_t = prom_buf_t {
        data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        cap: 0,
    };
    prom_buf_init(&raw mut buffer, 16384 as size_t);
    if buffer.data.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut sampler: system_health_stats_t = system_health_stats_t {
        initialized: false,
        enabled: false,
        ring_allocated: false,
        worker_threads: 0,
        generations_completed: 0,
        collections_completed: 0,
        collection_errors: 0,
        collection_timeouts: 0,
        overlap_skips: 0,
        observations_dropped: 0,
        coverage_overflows: 0,
        abandoned_helpers: 0,
    };
    system_health_get_stats(&raw mut sampler);
    let mut collectors: [system_health_collector_stats_t; 33] = [system_health_collector_stats_t {
        name: [0; 32],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        tier: system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST,
        interval_seconds: 0,
        stale_after_seconds: 0,
        attempts: 0,
        completions: 0,
        failures: 0,
        timeouts: 0,
        overlap_skips: 0,
        last_duration_ms: 0,
        maximum_duration_ms: 0,
        last_attempt_monotonic_ms: 0,
        last_success_monotonic_ms: 0,
        busy: false,
        stale: false,
    }; 33];
    let mut collector_count: size_t = system_health_collector_stats_copy(
        &raw mut collectors as *mut system_health_collector_stats_t,
        SYSTEM_HEALTH_MAX_COLLECTORS.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_health_collector_duration_seconds Last and maximum bounded collector duration\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_health_collector_duration_seconds gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_health_collector_events_total Monotonic collector events for this process run\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_health_collector_events_total counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_health_collector_stale Whether a collector has no successful sample inside its staleness bound\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_health_collector_stale gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_health_collector_busy Whether a collector call is currently in progress\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_health_collector_busy gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut index: size_t = 0 as size_t;
    while index < collector_count {
        let mut name: [::core::ffi::c_char; 65] = [0; 65];
        prom_label_escape(
            &raw mut (*(&raw mut collectors as *mut system_health_collector_stats_t)
                .offset(index as isize))
            .name as *mut ::core::ffi::c_char,
            &raw mut name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
        );
        let mut scope: *const ::core::ffi::c_char =
            system_health_scope_name(collectors[index].scope);
        prom_buf_append(
            &raw mut buffer,
            b"lightnvr_health_collector_duration_seconds{collector=\"%s\",scope=\"%s\",kind=\"last\"} %.3f\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut name as *mut ::core::ffi::c_char,
            scope,
            collectors[index].last_duration_ms as ::core::ffi::c_double / 1000.0f64,
        );
        prom_buf_append(
            &raw mut buffer,
            b"lightnvr_health_collector_duration_seconds{collector=\"%s\",scope=\"%s\",kind=\"maximum\"} %.3f\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut name as *mut ::core::ffi::c_char,
            scope,
            collectors[index].maximum_duration_ms as ::core::ffi::c_double / 1000.0f64,
        );
        let events: [uint64_t; 5] = [
            collectors[index].attempts,
            collectors[index].completions,
            collectors[index].failures,
            collectors[index].timeouts,
            collectors[index].overlap_skips,
        ];
        let mut event_names: [*const ::core::ffi::c_char; 5] = [
            b"attempt\0".as_ptr() as *const ::core::ffi::c_char,
            b"completion\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"timeout\0".as_ptr() as *const ::core::ffi::c_char,
            b"overlap_skip\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut event: size_t = 0 as size_t;
        while event < 5 as size_t {
            prom_buf_append(
                &raw mut buffer,
                b"lightnvr_health_collector_events_total{collector=\"%s\",scope=\"%s\",event=\"%s\"} %llu\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut name as *mut ::core::ffi::c_char,
                scope,
                event_names[event],
                events[event] as ::core::ffi::c_ulonglong,
            );
            event = event.wrapping_add(1);
        }
        prom_buf_append(
            &raw mut buffer,
            b"lightnvr_health_collector_stale{collector=\"%s\",scope=\"%s\"} %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut name as *mut ::core::ffi::c_char,
            scope,
            if collectors[index].stale as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        prom_buf_append(
            &raw mut buffer,
            b"lightnvr_health_collector_busy{collector=\"%s\",scope=\"%s\"} %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut name as *mut ::core::ffi::c_char,
            scope,
            if collectors[index].busy as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        index = index.wrapping_add(1);
    }
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_health_coverage_overflows_total Bounded health observations or resources omitted because a cap was reached\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_health_coverage_overflows_total counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_health_coverage_overflows_total %llu\n\0".as_ptr() as *const ::core::ffi::c_char,
        sampler.coverage_overflows as ::core::ffi::c_ulonglong,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_health_sampler_events_total Monotonic sampler events for this process run\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_health_sampler_events_total counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let sampler_values: [uint64_t; 6] = [
        sampler.generations_completed,
        sampler.collections_completed,
        sampler.collection_errors,
        sampler.collection_timeouts,
        sampler.overlap_skips,
        sampler.observations_dropped,
    ];
    let mut sampler_events: [*const ::core::ffi::c_char; 6] = [
        b"generation\0".as_ptr() as *const ::core::ffi::c_char,
        b"collection\0".as_ptr() as *const ::core::ffi::c_char,
        b"error\0".as_ptr() as *const ::core::ffi::c_char,
        b"timeout\0".as_ptr() as *const ::core::ffi::c_char,
        b"overlap_skip\0".as_ptr() as *const ::core::ffi::c_char,
        b"observation_drop\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut index_0: size_t = 0 as size_t;
    while index_0 < 6 as size_t {
        prom_buf_append(
            &raw mut buffer,
            b"lightnvr_health_sampler_events_total{event=\"%s\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            sampler_events[index_0],
            sampler_values[index_0] as ::core::ffi::c_ulonglong,
        );
        index_0 = index_0.wrapping_add(1);
    }
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_health_abandoned_helpers Bounded helper processes still being reaped\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_health_abandoned_helpers gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_health_abandoned_helpers %u\n\0".as_ptr() as *const ::core::ffi::c_char,
        sampler.abandoned_helpers,
    );
    let mut evaluator: system_health_evaluator_stats_t = system_health_evaluator_stats_t {
        tracked_conditions: 0,
        active_incidents: 0,
        pending_persistence: 0,
        transitions: 0,
        persistence_failures: 0,
        persistence_retries: 0,
        oldest_pending_wall_time_ms: 0,
    };
    system_health_evaluator_service_get_stats(&raw mut evaluator);
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_health_evaluator_events_total Monotonic evaluator events for this process run\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_health_evaluator_events_total counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_health_evaluator_events_total{event=\"transition\"} %llu\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        evaluator.transitions as ::core::ffi::c_ulonglong,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_health_evaluator_events_total{event=\"persistence_failure\"} %llu\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        evaluator.persistence_failures as ::core::ffi::c_ulonglong,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_health_evaluator_events_total{event=\"persistence_retry\"} %llu\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        evaluator.persistence_retries as ::core::ffi::c_ulonglong,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_health_persistence_pending Conditions frozen pending durable persistence\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_health_persistence_pending gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_health_persistence_pending %zu\n\0".as_ptr() as *const ::core::ffi::c_char,
        evaluator.pending_persistence,
    );
    let mut bus: event_bus_stats_t = event_bus_stats_t {
        accepted_events: 0,
        dispatched_events: 0,
        callback_deliveries: 0,
        handler_failures: 0,
        dropped_events: 0,
        priority_shed_events: 0,
        rejected_events: 0,
        queued_events: 0,
        queued_bytes: 0,
        active_dispatches: 0,
        max_events: 0,
        max_bytes: 0,
        subscriber_count: 0,
        running: false,
    };
    let mut router: event_router_stats_t = event_router_stats_t {
        cache_reloads: 0,
        events_evaluated: 0,
        default_events: 0,
        matched_events: 0,
        unmatched_events: 0,
        evaluation_errors: 0,
        routes_considered: 0,
        type_rejections: 0,
        scope_rejections: 0,
        predicate_rejections: 0,
        schedule_rejections: 0,
        destination_disabled_rejections: 0,
        debounce_suppressions: 0,
        cooldown_suppressions: 0,
        grouping_suppressions: 0,
        rate_suppressions: 0,
        suppression_errors: 0,
    };
    let mut worker: mqtt_delivery_worker_stats_t = mqtt_delivery_worker_stats_t {
        enqueued: 0,
        duplicates: 0,
        rejected_full: 0,
        enqueue_errors: 0,
        priority_shed: 0,
        expired: 0,
        attempted: 0,
        delivered: 0,
        retried: 0,
        dead: 0,
        outcome_errors: 0,
        disconnected_polls: 0,
        managed_profiles: 0,
        managed_connected: 0,
        profile_reloads: 0,
        profile_errors: 0,
        pruned: 0,
        running: false,
    };
    let mut default_presence: mqtt_presence_stats_t = mqtt_presence_stats_t {
        configured: false,
        connected: false,
        sequence: 0,
        connections: 0,
        reconnects: 0,
        disconnects: 0,
        publish_attempts: 0,
        publish_successes: 0,
        publish_failures: 0,
        last_publish_at_ms: 0,
    };
    event_bus_get_stats(&raw mut bus);
    event_router_get_stats(&raw mut router);
    mqtt_delivery_worker_get_stats(&raw mut worker);
    mqtt_presence_get_stats(&raw mut default_presence);
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_bus_events_total Monotonic in-process event-bus outcomes\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_bus_events_total counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let bus_values: [uint64_t; 6] = [
        bus.accepted_events,
        bus.dispatched_events,
        bus.dropped_events,
        bus.priority_shed_events,
        bus.rejected_events,
        bus.handler_failures,
    ];
    let mut bus_events: [*const ::core::ffi::c_char; 6] = [
        b"accepted\0".as_ptr() as *const ::core::ffi::c_char,
        b"dispatched\0".as_ptr() as *const ::core::ffi::c_char,
        b"dropped\0".as_ptr() as *const ::core::ffi::c_char,
        b"priority_shed\0".as_ptr() as *const ::core::ffi::c_char,
        b"rejected\0".as_ptr() as *const ::core::ffi::c_char,
        b"handler_failure\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut index_1: size_t = 0 as size_t;
    while index_1 < 6 as size_t {
        prom_buf_append(
            &raw mut buffer,
            b"lightnvr_event_bus_events_total{event=\"%s\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            bus_events[index_1],
            bus_values[index_1] as ::core::ffi::c_ulonglong,
        );
        index_1 = index_1.wrapping_add(1);
    }
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_bus_queue Current bounded event-bus queue occupancy\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_bus_queue gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_event_bus_queue{unit=\"events\"} %zu\n\0".as_ptr() as *const ::core::ffi::c_char,
        bus.queued_events,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_event_bus_queue{unit=\"bytes\"} %zu\n\0".as_ptr() as *const ::core::ffi::c_char,
        bus.queued_bytes,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_router_events_total Monotonic event-router outcomes\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_router_events_total counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_event_router_events_total{event=\"evaluated\"} %llu\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        router.events_evaluated as ::core::ffi::c_ulonglong,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_event_router_events_total{event=\"matched\"} %llu\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        router.matched_events as ::core::ffi::c_ulonglong,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_event_router_events_total{event=\"unmatched\"} %llu\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        router.unmatched_events as ::core::ffi::c_ulonglong,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_event_router_events_total{event=\"error\"} %llu\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        router.evaluation_errors as ::core::ffi::c_ulonglong,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_delivery_events_total Monotonic durable-delivery worker outcomes\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_delivery_events_total counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let delivery_values: [uint64_t; 8] = [
        worker.enqueued,
        worker.rejected_full,
        worker.enqueue_errors,
        worker.attempted,
        worker.delivered,
        worker.retried,
        worker.dead,
        worker.profile_errors,
    ];
    let mut delivery_events: [*const ::core::ffi::c_char; 8] = [
        b"enqueued\0".as_ptr() as *const ::core::ffi::c_char,
        b"rejected_full\0".as_ptr() as *const ::core::ffi::c_char,
        b"enqueue_error\0".as_ptr() as *const ::core::ffi::c_char,
        b"attempted\0".as_ptr() as *const ::core::ffi::c_char,
        b"delivered\0".as_ptr() as *const ::core::ffi::c_char,
        b"retried\0".as_ptr() as *const ::core::ffi::c_char,
        b"dead\0".as_ptr() as *const ::core::ffi::c_char,
        b"profile_error\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut index_2: size_t = 0 as size_t;
    while index_2 < 8 as size_t {
        prom_buf_append(
            &raw mut buffer,
            b"lightnvr_event_delivery_events_total{event=\"%s\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            delivery_events[index_2],
            delivery_values[index_2] as ::core::ffi::c_ulonglong,
        );
        index_2 = index_2.wrapping_add(1);
    }
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_outbox_rows Current durable outbox rows by bounded destination and state\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_outbox_rows gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_outbox_bytes Current durable outbox bytes by bounded destination\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_outbox_bytes gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_outbox_oldest_pending_age_seconds Age of the oldest pending durable event\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_outbox_oldest_pending_age_seconds gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut now: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    let mut aggregate: event_outbox_stats_t = event_outbox_stats_t {
        total_rows: 0,
        total_bytes: 0,
        pending_rows: 0,
        delivering_rows: 0,
        delivered_rows: 0,
        dead_rows: 0,
        due_rows: 0,
        oldest_pending_at: 0,
    };
    memset(
        &raw mut aggregate as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_outbox_stats_t>(),
    );
    let mut outbox_available: bool = append_outbox_metrics(
        &raw mut buffer,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        now,
        &raw mut aggregate,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_outbox_stats_available Whether the durable outbox snapshot could be read\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_outbox_stats_available gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_event_outbox_stats_available %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        if outbox_available as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    if default_presence.configured {
        append_outbox_metrics(
            &raw mut buffer,
            b"default\0".as_ptr() as *const ::core::ffi::c_char,
            MQTT_EVENT_OUTBOX_DESTINATION.as_ptr(),
            now,
            ::core::ptr::null_mut::<event_outbox_stats_t>(),
        );
    }
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_destination_connected Whether a bounded MQTT destination is connected\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_destination_connected gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_destination_events_total Monotonic MQTT destination events\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_destination_events_total counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_destination_last_failure Normalized last MQTT destination failure\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_destination_last_failure gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut configured: size_t = (if default_presence.configured as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    }) as size_t;
    let mut connected: size_t = (if default_presence.configured as ::core::ffi::c_int != 0
        && default_presence.connected as ::core::ffi::c_int != 0
    {
        1 as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    }) as size_t;
    let mut only_destination_failed: bool = r#false != 0;
    if default_presence.configured {
        prom_buf_append(
            &raw mut buffer,
            b"lightnvr_event_destination_connected{destination=\"default\"} %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            if default_presence.connected as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        prom_buf_append(
            &raw mut buffer,
            b"lightnvr_event_destination_events_total{destination=\"default\",event=\"reconnect\",failure=\"none\"} %llu\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            default_presence.reconnects as ::core::ffi::c_ulonglong,
        );
        prom_buf_append(
            &raw mut buffer,
            b"lightnvr_event_destination_events_total{destination=\"default\",event=\"publish_failure\",failure=\"publication\"} %llu\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            default_presence.publish_failures as ::core::ffi::c_ulonglong,
        );
    }
    let mut destinations: [mqtt_destination_client_stats_t; 64] =
        [mqtt_destination_client_stats_t {
            destination_uuid: [0; 37],
            presence_configured: false,
            connected: false,
            connections: 0,
            reconnects: 0,
            disconnects: 0,
            connection_failures: 0,
            publish_successes: 0,
            publish_failures: 0,
            last_success_at_ms: 0,
            last_failure_at_ms: 0,
            last_failure: mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_NONE,
        }; 64];
    let mut destination_count: size_t = mqtt_destination_client_list_stats(
        &raw mut destinations as *mut mqtt_destination_client_stats_t,
        EVENT_DESTINATION_MAX_COUNT as size_t,
    );
    let mut profiles: [event_destination_t; 64] = [event_destination_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_type: [0; 16],
        broker_host: [0; 256],
        broker_port: 0,
        client_id: [0; 128],
        topic_template: [0; 512],
        status_topic_template: [0; 512],
        username: [0; 128],
        password_configured: false,
        tls_mode: [0; 16],
        ca_file: [0; 512],
        cert_file: [0; 512],
        key_file: [0; 512],
        keepalive_seconds: 0,
        qos: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    }; 64];
    let mut profile_count: ::core::ffi::c_int = db_event_destination_list(
        &raw mut profiles as *mut event_destination_t,
        EVENT_DESTINATION_MAX_COUNT,
    );
    let mut profiles_available: bool = profile_count >= 0 as ::core::ffi::c_int;
    if profile_count < 0 as ::core::ffi::c_int {
        profile_count = 0 as ::core::ffi::c_int;
    }
    let mut index_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index_3 < profile_count {
        if profiles[index_3 as usize].enabled {
            configured = configured.wrapping_add(1);
            let mut stats: *const mqtt_destination_client_stats_t = find_mqtt_destination_stats(
                &raw mut destinations as *mut mqtt_destination_client_stats_t,
                destination_count,
                &raw mut (*(&raw mut profiles as *mut event_destination_t).offset(index_3 as isize))
                    .uuid as *mut ::core::ffi::c_char,
            );
            if !stats.is_null() && (*stats).connected as ::core::ffi::c_int != 0 {
                connected = connected.wrapping_add(1);
            }
            let mut uuid: [::core::ffi::c_char; 75] = [0; 75];
            prom_label_escape(
                &raw mut (*(&raw mut profiles as *mut event_destination_t).offset(index_3 as isize))
                    .uuid as *mut ::core::ffi::c_char,
                &raw mut uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 75]>(),
            );
            let mut failure: *const ::core::ffi::c_char = if !stats.is_null() {
                mqtt_failure_label((*stats).last_failure)
            } else {
                b"configuration\0".as_ptr() as *const ::core::ffi::c_char
            };
            prom_buf_append(
                &raw mut buffer,
                b"lightnvr_event_destination_connected{destination=\"%s\"} %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut uuid as *mut ::core::ffi::c_char,
                if !stats.is_null() && (*stats).connected as ::core::ffi::c_int != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                },
            );
            prom_buf_append(
                &raw mut buffer,
                b"lightnvr_event_destination_events_total{destination=\"%s\",event=\"reconnect\",failure=\"none\"} %llu\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut uuid as *mut ::core::ffi::c_char,
                (if !stats.is_null() { (*stats).reconnects } else { 0 as uint64_t })
                    as ::core::ffi::c_ulonglong,
            );
            prom_buf_append(
                &raw mut buffer,
                b"lightnvr_event_destination_events_total{destination=\"%s\",event=\"connection_failure\",failure=\"connection\"} %llu\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut uuid as *mut ::core::ffi::c_char,
                (if !stats.is_null() {
                    (*stats).connection_failures
                } else {
                    0 as uint64_t
                }) as ::core::ffi::c_ulonglong,
            );
            prom_buf_append(
                &raw mut buffer,
                b"lightnvr_event_destination_events_total{destination=\"%s\",event=\"publish_failure\",failure=\"publication\"} %llu\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut uuid as *mut ::core::ffi::c_char,
                (if !stats.is_null() {
                    (*stats).publish_failures
                } else {
                    0 as uint64_t
                }) as ::core::ffi::c_ulonglong,
            );
            prom_buf_append(
                &raw mut buffer,
                b"lightnvr_event_destination_last_failure{destination=\"%s\",failure=\"%s\"} 1\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut uuid as *mut ::core::ffi::c_char,
                failure,
            );
            let mut key: [::core::ffi::c_char; 64] = [0; 64];
            if db_event_destination_make_key(
                &raw mut (*(&raw mut profiles as *mut event_destination_t).offset(index_3 as isize))
                    .uuid as *mut ::core::ffi::c_char,
                &raw mut key as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                append_outbox_metrics(
                    &raw mut buffer,
                    &raw mut uuid as *mut ::core::ffi::c_char,
                    &raw mut key as *mut ::core::ffi::c_char,
                    now,
                    ::core::ptr::null_mut::<event_outbox_stats_t>(),
                );
            }
        }
        index_3 += 1;
    }
    if !profiles_available {
        let mut index_4: size_t = 0 as size_t;
        while index_4 < destination_count {
            configured = configured.wrapping_add(1);
            if destinations[index_4].connected {
                connected = connected.wrapping_add(1);
            }
            let mut uuid_0: [::core::ffi::c_char; 75] = [0; 75];
            prom_label_escape(
                &raw mut (*(&raw mut destinations as *mut mqtt_destination_client_stats_t)
                    .offset(index_4 as isize))
                .destination_uuid as *mut ::core::ffi::c_char,
                &raw mut uuid_0 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 75]>(),
            );
            prom_buf_append(
                &raw mut buffer,
                b"lightnvr_event_destination_connected{destination=\"%s\"} %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut uuid_0 as *mut ::core::ffi::c_char,
                if destinations[index_4].connected as ::core::ffi::c_int != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                },
            );
            index_4 = index_4.wrapping_add(1);
        }
    }
    if configured == 1 as size_t {
        if default_presence.configured {
            only_destination_failed =
                !default_presence.connected && default_presence.publish_failures > 0 as uint64_t;
        } else if profiles_available {
            let mut index_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while index_5 < profile_count {
                if !profiles[index_5 as usize].enabled {
                    index_5 += 1;
                } else {
                    let mut stats_0: *const mqtt_destination_client_stats_t =
                        find_mqtt_destination_stats(
                            &raw mut destinations as *mut mqtt_destination_client_stats_t,
                            destination_count,
                            &raw mut (*(&raw mut profiles as *mut event_destination_t)
                                .offset(index_5 as isize))
                            .uuid as *mut ::core::ffi::c_char,
                        );
                    only_destination_failed = stats_0.is_null()
                        || !(*stats_0).connected
                            && (*stats_0).last_failure.0
                                != mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_NONE.0;
                    break;
                }
            }
        } else if destination_count == 1 as size_t {
            only_destination_failed = !destinations[0usize].connected
                && destinations[0usize].last_failure.0
                    != mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_NONE.0;
        }
    }
    let mut outbox_full: bool = outbox_available as ::core::ffi::c_int != 0
        && (aggregate.total_rows >= EVENT_OUTBOX_DEFAULT_MAX_ROWS as int64_t
            || aggregate.total_bytes as ::core::ffi::c_longlong >= EVENT_OUTBOX_DEFAULT_MAX_BYTES);
    let mut undeliverable: bool = outbox_available as ::core::ffi::c_int != 0
        && aggregate.pending_rows > 0 as int64_t
        && connected == 0 as size_t;
    let mut degraded: bool = outbox_full as ::core::ffi::c_int != 0
        || only_destination_failed as ::core::ffi::c_int != 0
        || undeliverable as ::core::ffi::c_int != 0
        || outbox_available as ::core::ffi::c_int != 0 && aggregate.dead_rows > 0 as int64_t
        || !profiles_available && destination_count > 0 as size_t;
    let mut degraded_known: bool = outbox_available as ::core::ffi::c_int != 0
        || only_destination_failed as ::core::ffi::c_int != 0;
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_delivery_degraded Whether durable event delivery is locally degraded\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_delivery_degraded gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if degraded_known {
        prom_buf_append(
            &raw mut buffer,
            b"lightnvr_event_delivery_degraded %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            if degraded as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
    }
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_event_delivery_circular_report_path Whether the only alert path is the degraded destination itself\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_event_delivery_circular_report_path gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_event_delivery_circular_report_path %d\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        if configured == 1 as size_t
            && (only_destination_failed as ::core::ffi::c_int != 0
                || outbox_full as ::core::ffi::c_int != 0
                || undeliverable as ::core::ffi::c_int != 0)
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    let mut oldest_pending_ms: int64_t = evaluator.oldest_pending_wall_time_ms;
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_health_persistence_retry_age_seconds Age of the oldest incident awaiting durable persistence\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_health_persistence_retry_age_seconds gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut now_ms: int64_t = if now > 0 as int64_t {
        now * 1000 as int64_t
    } else {
        0 as int64_t
    };
    let mut retry_age: ::core::ffi::c_double =
        if oldest_pending_ms > 0 as int64_t && now_ms >= oldest_pending_ms {
            (now_ms - oldest_pending_ms) as ::core::ffi::c_double / 1000.0f64
        } else {
            0.0f64
        };
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_health_persistence_retry_age_seconds %.3f\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        retry_age,
    );
    return buffer.data;
}
unsafe extern "C" fn prom_label_escape(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) {
    let mut used: size_t = 0 as size_t;
    if output.is_null() || output_size == 0 as size_t {
        return;
    }
    if input.is_null() {
        input = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    while *input as ::core::ffi::c_int != 0 && used.wrapping_add(1 as size_t) < output_size {
        let mut replacement: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        if *input as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
            replacement = b"\\\\\0".as_ptr() as *const ::core::ffi::c_char;
        } else if *input as ::core::ffi::c_int == '"' as ::core::ffi::c_int {
            replacement = b"\\\"\0".as_ptr() as *const ::core::ffi::c_char;
        } else if *input as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            || *input as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
        {
            replacement = b"\\n\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if !replacement.is_null() {
            if used.wrapping_add(2 as size_t) >= output_size {
                break;
            }
            let c2rust_fresh0 = used;
            used = used.wrapping_add(1);
            *output.offset(c2rust_fresh0 as isize) = *replacement.offset(0isize);
            let c2rust_fresh1 = used;
            used = used.wrapping_add(1);
            *output.offset(c2rust_fresh1 as isize) = *replacement.offset(1isize);
        } else {
            let c2rust_fresh2 = used;
            used = used.wrapping_add(1);
            *output.offset(c2rust_fresh2 as isize) = *input;
        }
        input = input.offset(1);
    }
    *output.offset(used as isize) = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn observation_usable(
    mut observation: *const system_health_observation_t,
) -> bool {
    return !observation.is_null()
        && (*observation).value_valid as ::core::ffi::c_int != 0
        && (*observation).capability.0
            == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        && (*observation).freshness.0
            != system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_STALE.0;
}
unsafe extern "C" fn find_observation(
    mut snapshot: *const system_health_snapshot_t,
    mut metric: *const ::core::ffi::c_char,
    mut resource_id: *const ::core::ffi::c_char,
) -> *const system_health_observation_t {
    if snapshot.is_null() || metric.is_null() {
        return ::core::ptr::null::<system_health_observation_t>();
    }
    let mut index: size_t = 0 as size_t;
    while index < (*snapshot).observation_count {
        let mut observation: *const system_health_observation_t =
            (&raw const (*snapshot).observations as *const system_health_observation_t)
                .offset(index as isize);
        if strcmp(
            &raw const (*observation).metric as *const ::core::ffi::c_char,
            metric,
        ) == 0 as ::core::ffi::c_int
            && (resource_id.is_null()
                || strcmp(
                    &raw const (*observation).resource_id as *const ::core::ffi::c_char,
                    resource_id,
                ) == 0 as ::core::ffi::c_int)
        {
            return observation;
        }
        index = index.wrapping_add(1);
    }
    return ::core::ptr::null::<system_health_observation_t>();
}
unsafe extern "C" fn collector_for_metric(
    mut o: *const system_health_observation_t,
) -> *const ::core::ffi::c_char {
    if strcmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"hardware.provider.visible\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return &raw const (*o).resource_id as *const ::core::ffi::c_char;
    }
    if strncmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"storage.device.\0".as_ptr() as *const ::core::ffi::c_char,
        15 as size_t,
    ) == 0 as ::core::ffi::c_int
        || strncmp(
            &raw const (*o).metric as *const ::core::ffi::c_char,
            b"hardware.\0".as_ptr() as *const ::core::ffi::c_char,
            9 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        return b"linux_hardware\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"kernel.\0".as_ptr() as *const ::core::ffi::c_char,
        7 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return b"kernel_log\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"storage.filesystem.\0".as_ptr() as *const ::core::ffi::c_char,
        19 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return b"storage_targets\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"filesystem.\0".as_ptr() as *const ::core::ffi::c_char,
        11 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return b"linux_filesystem\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"network.\0".as_ptr() as *const ::core::ffi::c_char,
        8 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return b"linux_network\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"thermal.\0".as_ptr() as *const ::core::ffi::c_char,
        8 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return b"linux_thermal\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"process.\0".as_ptr() as *const ::core::ffi::c_char,
        8 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return b"linux_process\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"container.\0".as_ptr() as *const ::core::ffi::c_char,
        10 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return b"linux_cgroup\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"host.\0".as_ptr() as *const ::core::ffi::c_char,
        5 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return b"linux_proc\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"clock.\0".as_ptr() as *const ::core::ffi::c_char,
        6 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return b"linux_clock\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcmp(
        &raw const (*o).metric as *const ::core::ffi::c_char,
        b"system.uptime_seconds\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return b"linux_restart\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn append_single_observation(
    mut buf: *mut prom_buf_t,
    mut family: *const ::core::ffi::c_char,
    mut labels: *const ::core::ffi::c_char,
    mut o: *const system_health_observation_t,
) {
    if !observation_usable(o) {
        return;
    }
    prom_buf_append(
        buf,
        b"%s%s %.17g\n\0".as_ptr() as *const ::core::ffi::c_char,
        family,
        if !labels.is_null() {
            labels
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        (*o).value,
    );
}
unsafe extern "C" fn append_system_observations(
    mut buf: *mut prom_buf_t,
    mut snapshot: *const system_health_snapshot_t,
) {
    let mut o: *const system_health_observation_t =
        ::core::ptr::null::<system_health_observation_t>();
    let mut cpu: *const system_health_observation_t = find_observation(
        snapshot,
        b"container.cpu.usage_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !observation_usable(cpu) {
        cpu = find_observation(
            snapshot,
            b"host.cpu.usage_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if !observation_usable(cpu) {
        cpu = find_observation(
            snapshot,
            b"host.cpu.busy_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    prom_buf_append(
        buf,
        b"# HELP lightnvr_system_cpu_usage_ratio Effective CPU usage as a ratio\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_system_cpu_usage_ratio gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    append_single_observation(
        buf,
        b"lightnvr_system_cpu_usage_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        cpu,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_system_load_average Host runnable load average\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_system_load_average gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_system_load_ratio Host load average divided by effective CPU capacity when that capacity is visible\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_system_load_ratio gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut load_metrics: [*const ::core::ffi::c_char; 3] = [
        b"host.load.1\0".as_ptr() as *const ::core::ffi::c_char,
        b"host.load.5\0".as_ptr() as *const ::core::ffi::c_char,
        b"host.load.15\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut load_periods: [*const ::core::ffi::c_char; 3] = [
        b"1m\0".as_ptr() as *const ::core::ffi::c_char,
        b"5m\0".as_ptr() as *const ::core::ffi::c_char,
        b"15m\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut cpu_capacity: *const system_health_observation_t = find_observation(
        snapshot,
        b"container.cpu.quota_cores\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !observation_usable(cpu_capacity) {
        cpu_capacity = find_observation(
            snapshot,
            b"host.cpu.quota_cores\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    let mut index: size_t = 0 as size_t;
    while index < 3 as size_t {
        o = find_observation(
            snapshot,
            load_metrics[index],
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        let mut labels: [::core::ffi::c_char; 32] = [0; 32];
        snprintf(
            &raw mut labels as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"{period=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
            load_periods[index],
        );
        append_single_observation(
            buf,
            b"lightnvr_system_load_average\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut labels as *mut ::core::ffi::c_char,
            o,
        );
        if observation_usable(o) as ::core::ffi::c_int != 0
            && observation_usable(cpu_capacity) as ::core::ffi::c_int != 0
            && (*cpu_capacity).value > 0.0f64
        {
            prom_buf_append(
                buf,
                b"lightnvr_system_load_ratio%s %.17g\n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut labels as *mut ::core::ffi::c_char,
                (*o).value / (*cpu_capacity).value,
            );
        }
        index = index.wrapping_add(1);
    }
    prom_buf_append(
        buf,
        b"# HELP lightnvr_system_pressure_stall_ratio Linux PSI stall ratio\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_system_pressure_stall_ratio gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut pressure_resources: [*const ::core::ffi::c_char; 3] = [
        b"cpu\0".as_ptr() as *const ::core::ffi::c_char,
        b"memory\0".as_ptr() as *const ::core::ffi::c_char,
        b"io\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut pressure_kinds: [*const ::core::ffi::c_char; 2] = [
        b"some\0".as_ptr() as *const ::core::ffi::c_char,
        b"full\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut resource: size_t = 0 as size_t;
    while resource < 3 as size_t {
        let mut kind: size_t = 0 as size_t;
        while kind < 2 as size_t {
            let mut metric: [::core::ffi::c_char; 64] = [0; 64];
            snprintf(
                &raw mut metric as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"host.pressure.%s.%s_ratio\0".as_ptr() as *const ::core::ffi::c_char,
                pressure_resources[resource],
                pressure_kinds[kind],
            );
            o = find_observation(
                snapshot,
                &raw mut metric as *mut ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            let mut labels_0: [::core::ffi::c_char; 128] = [0; 128];
            snprintf(
                &raw mut labels_0 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                b"{resource=\"%s\",kind=\"%s\",window=\"10s\"}\0".as_ptr()
                    as *const ::core::ffi::c_char,
                pressure_resources[resource],
                pressure_kinds[kind],
            );
            append_single_observation(
                buf,
                b"lightnvr_system_pressure_stall_ratio\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut labels_0 as *mut ::core::ffi::c_char,
                o,
            );
            kind = kind.wrapping_add(1);
        }
        resource = resource.wrapping_add(1);
    }
    prom_buf_append(
        buf,
        b"# HELP lightnvr_system_cpu_throttled_ratio Cgroup CPU throttled-period ratio\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_system_cpu_throttled_ratio gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut throttle_metrics: [*const ::core::ffi::c_char; 2] = [
        b"container.cpu.throttled_ratio\0".as_ptr() as *const ::core::ffi::c_char,
        b"host.cpu.throttled_ratio\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut throttle_scopes: [*const ::core::ffi::c_char; 2] = [
        b"container\0".as_ptr() as *const ::core::ffi::c_char,
        b"host\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut index_0: size_t = 0 as size_t;
    while index_0 < 2 as size_t {
        o = find_observation(
            snapshot,
            throttle_metrics[index_0],
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        let mut labels_1: [::core::ffi::c_char; 48] = [0; 48];
        snprintf(
            &raw mut labels_1 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 48]>(),
            b"{scope=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
            throttle_scopes[index_0],
        );
        append_single_observation(
            buf,
            b"lightnvr_system_cpu_throttled_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut labels_1 as *mut ::core::ffi::c_char,
            o,
        );
        index_0 = index_0.wrapping_add(1);
    }
    let mut memory_total: *const system_health_observation_t = find_observation(
        snapshot,
        b"container.memory.limit_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut memory_available: *const system_health_observation_t = find_observation(
        snapshot,
        b"container.memory.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut memory_used: *const system_health_observation_t = find_observation(
        snapshot,
        b"container.memory.current_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !observation_usable(memory_total) {
        memory_total = find_observation(
            snapshot,
            b"host.memory.total_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        memory_available = find_observation(
            snapshot,
            b"host.memory.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        memory_used = ::core::ptr::null::<system_health_observation_t>();
    }
    prom_buf_append(
        buf,
        b"# HELP lightnvr_system_memory_bytes Effective memory by state\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_system_memory_bytes gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    append_single_observation(
        buf,
        b"lightnvr_system_memory_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"{state=\"total\"}\0".as_ptr() as *const ::core::ffi::c_char,
        memory_total,
    );
    append_single_observation(
        buf,
        b"lightnvr_system_memory_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"{state=\"available\"}\0".as_ptr() as *const ::core::ffi::c_char,
        memory_available,
    );
    if observation_usable(memory_used) {
        append_single_observation(
            buf,
            b"lightnvr_system_memory_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            b"{state=\"used\"}\0".as_ptr() as *const ::core::ffi::c_char,
            memory_used,
        );
    } else if observation_usable(memory_total) as ::core::ffi::c_int != 0
        && observation_usable(memory_available) as ::core::ffi::c_int != 0
    {
        let mut used: ::core::ffi::c_double = if (*memory_total).value > (*memory_available).value {
            (*memory_total).value - (*memory_available).value
        } else {
            0.0f64
        };
        prom_buf_append(
            buf,
            b"lightnvr_system_memory_bytes{state=\"used\"} %.17g\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            used,
        );
    }
    prom_buf_append(
        buf,
        b"# HELP lightnvr_system_swap_bytes Host swap by state\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_system_swap_bytes gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    append_single_observation(
        buf,
        b"lightnvr_system_swap_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        b"{state=\"used\"}\0".as_ptr() as *const ::core::ffi::c_char,
        find_observation(
            snapshot,
            b"host.swap.used_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ),
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_system_vm_events_delta VM events observed during the latest collection interval\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_system_vm_events_delta gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut vm_metrics: [*const ::core::ffi::c_char; 3] = [
        b"host.vm.major_faults_delta\0".as_ptr() as *const ::core::ffi::c_char,
        b"host.vm.swap_in_pages_delta\0".as_ptr() as *const ::core::ffi::c_char,
        b"host.vm.swap_out_pages_delta\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut vm_events: [*const ::core::ffi::c_char; 3] = [
        b"major_fault\0".as_ptr() as *const ::core::ffi::c_char,
        b"swap_in\0".as_ptr() as *const ::core::ffi::c_char,
        b"swap_out\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut index_1: size_t = 0 as size_t;
    while index_1 < 3 as size_t {
        let mut labels_2: [::core::ffi::c_char; 64] = [0; 64];
        snprintf(
            &raw mut labels_2 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"{event=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
            vm_events[index_1],
        );
        append_single_observation(
            buf,
            b"lightnvr_system_vm_events_delta\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut labels_2 as *mut ::core::ffi::c_char,
            find_observation(
                snapshot,
                vm_metrics[index_1],
                ::core::ptr::null::<::core::ffi::c_char>(),
            ),
        );
        index_1 = index_1.wrapping_add(1);
    }
    append_single_observation(
        buf,
        b"lightnvr_system_vm_events_delta\0".as_ptr() as *const ::core::ffi::c_char,
        b"{event=\"oom_kill\"}\0".as_ptr() as *const ::core::ffi::c_char,
        find_observation(
            snapshot,
            b"container.memory.oom_kills_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ),
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_process_open_fds Open file descriptors in the LightNVR process\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_process_open_fds gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    append_single_observation(
        buf,
        b"lightnvr_process_open_fds\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        find_observation(
            snapshot,
            b"process.open_fds\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ),
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_process_max_fds Process file descriptor limit\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_process_max_fds gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    append_single_observation(
        buf,
        b"lightnvr_process_max_fds\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        find_observation(
            snapshot,
            b"process.fd_limit\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ),
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_process_threads Threads in the LightNVR process\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_process_threads gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    append_single_observation(
        buf,
        b"lightnvr_process_threads\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        find_observation(
            snapshot,
            b"process.threads\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ),
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_process_max_pids Effective process or cgroup PID limit\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_process_max_pids gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    o = find_observation(
        snapshot,
        b"container.pids.limit\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !observation_usable(o) {
        o = find_observation(
            snapshot,
            b"process.pid_limit\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    append_single_observation(
        buf,
        b"lightnvr_process_max_pids\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        o,
    );
    let mut storage_filesystem_seen: bool = r#false != 0;
    let mut index_2: size_t = 0 as size_t;
    while index_2 < (*snapshot).observation_count {
        if strncmp(
            &raw const (*(&raw const (*snapshot).observations
                as *const system_health_observation_t)
                .offset(index_2 as isize))
            .metric as *const ::core::ffi::c_char,
            b"storage.filesystem.\0".as_ptr() as *const ::core::ffi::c_char,
            19 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            storage_filesystem_seen = r#true != 0;
            break;
        } else {
            index_2 = index_2.wrapping_add(1);
        }
    }
    let mut filesystem_prefix: *const ::core::ffi::c_char =
        if storage_filesystem_seen as ::core::ffi::c_int != 0 {
            b"storage.filesystem.\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"filesystem.\0".as_ptr() as *const ::core::ffi::c_char
        };
    prom_buf_append(
        buf,
        b"# HELP lightnvr_filesystem_bytes Filesystem bytes by logical filesystem and state\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_filesystem_bytes gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_filesystem_inodes Filesystem inodes by logical filesystem and state\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_filesystem_inodes gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_filesystem_read_only Whether a logical filesystem is read-only\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_filesystem_read_only gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_filesystem_probe_duration_seconds Last bounded write probe duration\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_filesystem_probe_duration_seconds gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut index_3: size_t = 0 as size_t;
    while index_3 < (*snapshot).observation_count {
        o = (&raw const (*snapshot).observations as *const system_health_observation_t)
            .offset(index_3 as isize);
        if !(strncmp(
            &raw const (*o).metric as *const ::core::ffi::c_char,
            filesystem_prefix,
            strlen(filesystem_prefix),
        ) != 0 as ::core::ffi::c_int
            || !observation_usable(o))
        {
            let mut suffix: *const ::core::ffi::c_char = (&raw const (*o).metric
                as *const ::core::ffi::c_char)
                .offset(strlen(filesystem_prefix) as isize);
            let mut resource_0: [::core::ffi::c_char; 129] = [0; 129];
            let mut labels_3: [::core::ffi::c_char; 176] = [0; 176];
            prom_label_escape(
                &raw const (*o).resource_id as *const ::core::ffi::c_char,
                &raw mut resource_0 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 129]>(),
            );
            if strcmp(
                suffix,
                b"capacity_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    suffix,
                    b"available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                snprintf(
                    &raw mut labels_3 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 176]>(),
                    b"{filesystem=\"%s\",state=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource_0 as *mut ::core::ffi::c_char,
                    if *suffix.offset(0isize) as ::core::ffi::c_int == 'c' as ::core::ffi::c_int {
                        b"total\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"available\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                append_single_observation(
                    buf,
                    b"lightnvr_filesystem_bytes\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels_3 as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strcmp(
                suffix,
                b"capacity_inodes\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    suffix,
                    b"available_inodes\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                snprintf(
                    &raw mut labels_3 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 176]>(),
                    b"{filesystem=\"%s\",state=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource_0 as *mut ::core::ffi::c_char,
                    if *suffix.offset(0isize) as ::core::ffi::c_int == 'c' as ::core::ffi::c_int {
                        b"total\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"available\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                append_single_observation(
                    buf,
                    b"lightnvr_filesystem_inodes\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels_3 as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strcmp(
                suffix,
                b"read_only\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                snprintf(
                    &raw mut labels_3 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 176]>(),
                    b"{filesystem=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource_0 as *mut ::core::ffi::c_char,
                );
                append_single_observation(
                    buf,
                    b"lightnvr_filesystem_read_only\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels_3 as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strcmp(
                suffix,
                b"probe_latency_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                snprintf(
                    &raw mut labels_3 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 176]>(),
                    b"{filesystem=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource_0 as *mut ::core::ffi::c_char,
                );
                append_single_observation(
                    buf,
                    b"lightnvr_filesystem_probe_duration_seconds\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut labels_3 as *mut ::core::ffi::c_char,
                    o,
                );
            }
        }
        index_3 = index_3.wrapping_add(1);
    }
    prom_buf_append(
        buf,
        b"# HELP lightnvr_thermal_celsius Temperature sensor reading\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_thermal_celsius gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_network_packets_total Network packets since kernel counter reset\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_network_packets_total counter\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_network_bytes_total Network bytes since kernel counter reset\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_network_bytes_total counter\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_network_errors_total Network errors and drops since kernel counter reset\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_network_errors_total counter\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut index_4: size_t = 0 as size_t;
    while index_4 < (*snapshot).observation_count {
        o = (&raw const (*snapshot).observations as *const system_health_observation_t)
            .offset(index_4 as isize);
        if observation_usable(o) {
            let mut resource_1: [::core::ffi::c_char; 129] = [0; 129];
            prom_label_escape(
                &raw const (*o).resource_id as *const ::core::ffi::c_char,
                &raw mut resource_1 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 129]>(),
            );
            if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"thermal.temperature_celsius\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"storage.device.temperature_celsius\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let mut labels_4: [::core::ffi::c_char; 160] = [0; 160];
                snprintf(
                    &raw mut labels_4 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
                    b"{sensor=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource_1 as *mut ::core::ffi::c_char,
                );
                append_single_observation(
                    buf,
                    b"lightnvr_thermal_celsius\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels_4 as *mut ::core::ffi::c_char,
                    o,
                );
            }
            let mut direction: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            let mut kind_0: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut bytes: bool = r#false != 0;
            if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"network.rx_bytes_total\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                direction = b"rx\0".as_ptr() as *const ::core::ffi::c_char;
                bytes = r#true != 0;
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"network.tx_bytes_total\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                direction = b"tx\0".as_ptr() as *const ::core::ffi::c_char;
                bytes = r#true != 0;
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"network.rx_packets_total\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                direction = b"rx\0".as_ptr() as *const ::core::ffi::c_char;
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"network.tx_packets_total\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                direction = b"tx\0".as_ptr() as *const ::core::ffi::c_char;
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"network.rx_errors_total\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                direction = b"rx\0".as_ptr() as *const ::core::ffi::c_char;
                kind_0 = b"error\0".as_ptr() as *const ::core::ffi::c_char;
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"network.tx_errors_total\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                direction = b"tx\0".as_ptr() as *const ::core::ffi::c_char;
                kind_0 = b"error\0".as_ptr() as *const ::core::ffi::c_char;
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"network.rx_drops_total\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                direction = b"rx\0".as_ptr() as *const ::core::ffi::c_char;
                kind_0 = b"drop\0".as_ptr() as *const ::core::ffi::c_char;
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"network.tx_drops_total\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                direction = b"tx\0".as_ptr() as *const ::core::ffi::c_char;
                kind_0 = b"drop\0".as_ptr() as *const ::core::ffi::c_char;
            }
            if !direction.is_null() {
                let mut labels_5: [::core::ffi::c_char; 208] = [0; 208];
                if !kind_0.is_null() {
                    snprintf(
                        &raw mut labels_5 as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 208]>(),
                        b"{interface=\"%s\",direction=\"%s\",kind=\"%s\"}\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut resource_1 as *mut ::core::ffi::c_char,
                        direction,
                        kind_0,
                    );
                    append_single_observation(
                        buf,
                        b"lightnvr_network_errors_total\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut labels_5 as *mut ::core::ffi::c_char,
                        o,
                    );
                } else {
                    snprintf(
                        &raw mut labels_5 as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 208]>(),
                        b"{interface=\"%s\",direction=\"%s\"}\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut resource_1 as *mut ::core::ffi::c_char,
                        direction,
                    );
                    append_single_observation(
                        buf,
                        if bytes as ::core::ffi::c_int != 0 {
                            b"lightnvr_network_bytes_total\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"lightnvr_network_packets_total\0".as_ptr()
                                as *const ::core::ffi::c_char
                        },
                        &raw mut labels_5 as *mut ::core::ffi::c_char,
                        o,
                    );
                }
            }
        }
        index_4 = index_4.wrapping_add(1);
    }
    prom_buf_append(
        buf,
        b"# HELP lightnvr_clock_synchronized Whether the visible host clock is synchronized\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_clock_synchronized gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    append_single_observation(
        buf,
        b"lightnvr_clock_synchronized\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        find_observation(
            snapshot,
            b"clock.synchronized\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ),
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_clock_jump_seconds Last detected realtime-to-monotonic clock jump\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_clock_jump_seconds gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    append_single_observation(
        buf,
        b"lightnvr_clock_jump_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        find_observation(
            snapshot,
            b"clock.jump_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ),
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_system_uptime_seconds Visible host uptime\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_system_uptime_seconds gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    append_single_observation(
        buf,
        b"lightnvr_system_uptime_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        find_observation(
            snapshot,
            b"system.uptime_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ),
    );
}
unsafe extern "C" fn append_hardware_observations(
    mut buf: *mut prom_buf_t,
    mut snapshot: *const system_health_snapshot_t,
) {
    prom_buf_append(
        buf,
        b"# HELP lightnvr_device_wear_ratio Storage-device wear ratio\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_device_wear_ratio gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_device_available_spare_ratio Storage-device available spare ratio\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_device_available_spare_ratio gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_device_health_flag Normalized storage-device health flag\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_device_health_flag gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_device_events_delta Storage-device events in the latest provider interval\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_device_events_delta gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_device_smart_attribute Normalized current SMART attribute value\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_device_smart_attribute gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_hardware_ecc_errors_delta ECC errors in the latest provider interval\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_hardware_ecc_errors_delta gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_hardware_fan_rpm Hardware fan speed\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_hardware_fan_rpm gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_hardware_fan_flag Normalized fan condition\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_hardware_fan_flag gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_hardware_power_flag Normalized board power or throttling condition\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_hardware_power_flag gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# HELP lightnvr_kernel_events_delta Normalized kernel events in the latest provider interval\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_kernel_events_delta gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut index: size_t = 0 as size_t;
    while index < (*snapshot).observation_count {
        let mut o: *const system_health_observation_t = (&raw const (*snapshot).observations
            as *const system_health_observation_t)
            .offset(index as isize);
        if observation_usable(o) {
            let mut resource: [::core::ffi::c_char; 129] = [0; 129];
            let mut labels: [::core::ffi::c_char; 224] = [0; 224];
            prom_label_escape(
                &raw const (*o).resource_id as *const ::core::ffi::c_char,
                &raw mut resource as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 129]>(),
            );
            if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"storage.device.life_used_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"storage.device.percentage_used_ratio\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let mut source: *const ::core::ffi::c_char = if !strstr(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"percentage\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
                {
                    b"percentage_used\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"life_used\0".as_ptr() as *const ::core::ffi::c_char
                };
                snprintf(
                    &raw mut labels as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 224]>(),
                    b"{device=\"%s\",source=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource as *mut ::core::ffi::c_char,
                    source,
                );
                append_single_observation(
                    buf,
                    b"lightnvr_device_wear_ratio\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"storage.device.available_spare_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                snprintf(
                    &raw mut labels as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 224]>(),
                    b"{device=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource as *mut ::core::ffi::c_char,
                );
                append_single_observation(
                    buf,
                    b"lightnvr_device_available_spare_ratio\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut labels as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"storage.device.pre_eol\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"storage.device.prefail\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"storage.device.critical\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let mut kind: *const ::core::ffi::c_char = strrchr(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    '.' as ::core::ffi::c_int,
                );
                snprintf(
                    &raw mut labels as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 224]>(),
                    b"{device=\"%s\",kind=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource as *mut ::core::ffi::c_char,
                    if !kind.is_null() {
                        kind.offset(1 as ::core::ffi::c_int as isize)
                    } else {
                        b"unknown\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                append_single_observation(
                    buf,
                    b"lightnvr_device_health_flag\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"storage.device.reallocated_sectors\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"storage.device.pending_sectors\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let mut attribute: *const ::core::ffi::c_char = if !strstr(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"reallocated\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
                {
                    b"reallocated_sectors\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"pending_sectors\0".as_ptr() as *const ::core::ffi::c_char
                };
                snprintf(
                    &raw mut labels as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 224]>(),
                    b"{device=\"%s\",attribute=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource as *mut ::core::ffi::c_char,
                    attribute,
                );
                append_single_observation(
                    buf,
                    b"lightnvr_device_smart_attribute\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"storage.device.media_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"storage.device.unsafe_shutdowns_delta\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"storage.device.uncorrectable_errors_delta\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"storage.device.interface_crc_errors_delta\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let mut event: *const ::core::ffi::c_char = if !strstr(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"media_errors\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
                {
                    b"media_error\0".as_ptr() as *const ::core::ffi::c_char
                } else if !strstr(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"unsafe_shutdowns\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
                {
                    b"unsafe_shutdown\0".as_ptr() as *const ::core::ffi::c_char
                } else if !strstr(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"interface_crc\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
                {
                    b"interface_crc_error\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"uncorrectable_error\0".as_ptr() as *const ::core::ffi::c_char
                };
                snprintf(
                    &raw mut labels as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 224]>(),
                    b"{device=\"%s\",event=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource as *mut ::core::ffi::c_char,
                    event,
                );
                append_single_observation(
                    buf,
                    b"lightnvr_device_events_delta\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"hardware.ecc.corrected_delta\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"hardware.ecc.uncorrectable_delta\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let mut kind_0: *const ::core::ffi::c_char = if !strstr(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"uncorrectable\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
                {
                    b"uncorrectable\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"corrected\0".as_ptr() as *const ::core::ffi::c_char
                };
                snprintf(
                    &raw mut labels as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 224]>(),
                    b"{resource=\"%s\",correctability=\"%s\"}\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut resource as *mut ::core::ffi::c_char,
                    kind_0,
                );
                append_single_observation(
                    buf,
                    b"lightnvr_hardware_ecc_errors_delta\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"hardware.fan.rpm\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"hardware.fan.minimum_rpm\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                snprintf(
                    &raw mut labels as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 224]>(),
                    b"{fan=\"%s\",state=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource as *mut ::core::ffi::c_char,
                    if !strstr(
                        &raw const (*o).metric as *const ::core::ffi::c_char,
                        b"minimum\0".as_ptr() as *const ::core::ffi::c_char,
                    )
                    .is_null()
                    {
                        b"minimum\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"current\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                append_single_observation(
                    buf,
                    b"lightnvr_hardware_fan_rpm\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"hardware.fan.hot\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"hardware.fan.failed\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                snprintf(
                    &raw mut labels as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 224]>(),
                    b"{fan=\"%s\",kind=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource as *mut ::core::ffi::c_char,
                    if !strstr(
                        &raw const (*o).metric as *const ::core::ffi::c_char,
                        b"failed\0".as_ptr() as *const ::core::ffi::c_char,
                    )
                    .is_null()
                    {
                        b"failed\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"hot\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                append_single_observation(
                    buf,
                    b"lightnvr_hardware_fan_flag\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strcmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"hardware.throttled\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"hardware.power.unstable\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                snprintf(
                    &raw mut labels as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 224]>(),
                    b"{resource=\"%s\",kind=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut resource as *mut ::core::ffi::c_char,
                    if !strstr(
                        &raw const (*o).metric as *const ::core::ffi::c_char,
                        b"power\0".as_ptr() as *const ::core::ffi::c_char,
                    )
                    .is_null()
                    {
                        b"unstable\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"throttled\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                append_single_observation(
                    buf,
                    b"lightnvr_hardware_power_flag\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels as *mut ::core::ffi::c_char,
                    o,
                );
            } else if strncmp(
                &raw const (*o).metric as *const ::core::ffi::c_char,
                b"kernel.\0".as_ptr() as *const ::core::ffi::c_char,
                7 as size_t,
            ) == 0 as ::core::ffi::c_int
                && !strstr(
                    &raw const (*o).metric as *const ::core::ffi::c_char,
                    b"_delta\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
            {
                let mut event_0: [::core::ffi::c_char; 64] = [0; 64];
                let mut length: size_t = strlen(
                    (&raw const (*o).metric as *const ::core::ffi::c_char)
                        .offset(7 as ::core::ffi::c_int as isize),
                );
                if length >= ::core::mem::size_of::<[::core::ffi::c_char; 64]>() {
                    length = ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
                        .wrapping_sub(1usize) as size_t;
                }
                memcpy(
                    &raw mut event_0 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    (&raw const (*o).metric as *const ::core::ffi::c_char)
                        .offset(7 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    length,
                );
                event_0[length] = '\0' as ::core::ffi::c_char;
                let mut delta: *mut ::core::ffi::c_char = strstr(
                    &raw mut event_0 as *mut ::core::ffi::c_char,
                    b"_delta\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if !delta.is_null()
                    && *delta.offset(6isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                {
                    *delta = '\0' as ::core::ffi::c_char;
                }
                snprintf(
                    &raw mut labels as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 224]>(),
                    b"{event=\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut event_0 as *mut ::core::ffi::c_char,
                );
                append_single_observation(
                    buf,
                    b"lightnvr_kernel_events_delta\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut labels as *mut ::core::ffi::c_char,
                    o,
                );
            }
        }
        index = index.wrapping_add(1);
    }
}
unsafe extern "C" fn append_health_coverage(
    mut buf: *mut prom_buf_t,
    mut snapshot: *const system_health_snapshot_t,
) {
    let mut capability_counts: [[uint32_t; 5]; 5] =
        [[0 as uint32_t, 0, 0, 0, 0], [0; 5], [0; 5], [0; 5], [0; 5]];
    let mut collectors: [collector_export_t; 32] = [
        collector_export_t {
            name: [
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
            ],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
        collector_export_t {
            name: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            seen: false,
            up: false,
        },
    ];
    let mut collector_count: size_t = 0 as size_t;
    let mut index: size_t = 0 as size_t;
    while index < (*snapshot).observation_count {
        let mut o: *const system_health_observation_t = (&raw const (*snapshot).observations
            as *const system_health_observation_t)
            .offset(index as isize);
        if (*o).scope.0 >= 0 as ::core::ffi::c_uint
            && (*o).scope.0 < system_health_scope_t::SYSTEM_HEALTH_SCOPE_COUNT.0
            && (*o).capability.0 >= 0 as ::core::ffi::c_uint
            && (*o).capability.0 < system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_COUNT.0
        {
            capability_counts[(*o).scope.0 as usize][(*o).capability.0 as usize] =
                capability_counts[(*o).scope.0 as usize][(*o).capability.0 as usize]
                    .wrapping_add(1);
        }
        let mut name: *const ::core::ffi::c_char = collector_for_metric(o);
        's_10: {
            if !(name.is_null()
                || *name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int)
            {
                let mut slot: size_t = collector_count;
                let mut candidate: size_t = 0 as size_t;
                while candidate < collector_count {
                    if collectors[candidate].scope.0 == (*o).scope.0
                        && strcmp(
                            &raw mut (*(&raw mut collectors as *mut collector_export_t)
                                .offset(candidate as isize))
                            .name as *mut ::core::ffi::c_char,
                            name,
                        ) == 0 as ::core::ffi::c_int
                    {
                        slot = candidate;
                        break;
                    } else {
                        candidate = candidate.wrapping_add(1);
                    }
                }
                if slot == collector_count {
                    if collector_count >= SYSTEM_HEALTH_MAX_COLLECTORS as size_t {
                        break 's_10;
                    } else {
                        snprintf(
                            &raw mut (*(&raw mut collectors as *mut collector_export_t)
                                .offset(slot as isize))
                            .name as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            name,
                        );
                        collectors[slot].scope = (*o).scope;
                        collectors[slot].seen = r#true != 0;
                        collector_count = collector_count.wrapping_add(1);
                    }
                }
                if observation_usable(o) as ::core::ffi::c_int != 0
                    && (strcmp(
                        &raw const (*o).metric as *const ::core::ffi::c_char,
                        b"hardware.provider.visible\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                        || (*o).value != 0.0f64)
                {
                    collectors[slot].up = r#true != 0;
                }
            }
        }
        index = index.wrapping_add(1);
    }
    prom_buf_append(
        buf,
        b"# HELP lightnvr_health_collector_up Whether a bounded collector has at least one fresh available observation\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_health_collector_up gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut index_0: size_t = 0 as size_t;
    while index_0 < collector_count {
        let mut name_0: [::core::ffi::c_char; 129] = [0; 129];
        prom_label_escape(
            &raw mut (*(&raw mut collectors as *mut collector_export_t).offset(index_0 as isize))
                .name as *mut ::core::ffi::c_char,
            &raw mut name_0 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 129]>(),
        );
        prom_buf_append(
            buf,
            b"lightnvr_health_collector_up{collector=\"%s\",scope=\"%s\"} %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut name_0 as *mut ::core::ffi::c_char,
            system_health_scope_name(collectors[index_0].scope),
            if collectors[index_0].up as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        index_0 = index_0.wrapping_add(1);
    }
    prom_buf_append(
        buf,
        b"# HELP lightnvr_health_observations Number of bounded observations by scope and capability\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_health_observations gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut scope: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while scope < system_health_scope_t::SYSTEM_HEALTH_SCOPE_COUNT.0 as ::core::ffi::c_int {
        let mut capability: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while capability
            < system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_COUNT.0 as ::core::ffi::c_int
        {
            if capability_counts[scope as usize][capability as usize] != 0 as uint32_t {
                prom_buf_append(
                    buf,
                    b"lightnvr_health_observations{scope=\"%s\",capability=\"%s\"} %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    system_health_scope_name(system_health_scope_t(scope as ::core::ffi::c_uint)),
                    system_health_capability_name(system_health_capability_t(
                        capability as ::core::ffi::c_uint,
                    )),
                    capability_counts[scope as usize][capability as usize],
                );
            }
            capability += 1;
        }
        scope += 1;
    }
    prom_buf_append(
        buf,
        b"# HELP lightnvr_health_snapshot_observations_dropped Observations omitted from this immutable snapshot because the cap was reached\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_health_snapshot_observations_dropped gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"lightnvr_health_snapshot_observations_dropped %zu\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*snapshot).observations_dropped,
    );
}
unsafe extern "C" fn append_health_service_metrics(
    mut buf: *mut prom_buf_t,
    mut sampler: *const system_health_stats_t,
    mut evaluator: *const system_health_evaluator_stats_t,
    mut incidents: *const system_health_incident_view_t,
    mut incident_count: size_t,
    mut recording: *const recording_io_metrics_snapshot_t,
    mut run: *const system_health_process_run_t,
    mut run_valid: bool,
) {
    prom_buf_append(
        buf,
        b"# HELP lightnvr_health_incidents Active operational incidents by severity\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_health_incidents gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut severities: [size_t; 4] = [0 as size_t, 0, 0, 0];
    let mut index: size_t = 0 as size_t;
    while index < incident_count {
        if (*incidents.offset(index as isize)).severity.0
            > system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0
            && (*incidents.offset(index as isize)).severity.0
                <= system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0
        {
            severities[(*incidents.offset(index as isize)).severity.0 as usize] =
                severities[(*incidents.offset(index as isize)).severity.0 as usize].wrapping_add(1);
        }
        index = index.wrapping_add(1);
    }
    let mut severity: ::core::ffi::c_int =
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0 as ::core::ffi::c_int;
    while severity
        <= system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0 as ::core::ffi::c_int
    {
        prom_buf_append(
            buf,
            b"lightnvr_health_incidents{severity=\"%s\"} %zu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            system_health_severity_name(system_health_severity_t(severity as ::core::ffi::c_uint)),
            severities[severity as usize],
        );
        severity += 1;
    }
    prom_buf_append(
        buf,
        b"# HELP lightnvr_recording_io_failures_total Recording write failures by bounded resource and reason\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        buf,
        b"# TYPE lightnvr_recording_io_failures_total counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut resources: [*const ::core::ffi::c_char; 2] = [
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
        b"hls\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut resource: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while resource < recording_io_resource_t::RECORDING_IO_RESOURCE_COUNT.0 as ::core::ffi::c_int {
        let mut reason: ::core::ffi::c_int =
            recording_io_reason_t::RECORDING_IO_REASON_NO_SPACE.0 as ::core::ffi::c_int;
        while reason < recording_io_reason_t::RECORDING_IO_REASON_COUNT.0 as ::core::ffi::c_int {
            prom_buf_append(
                buf,
                b"lightnvr_recording_io_failures_total{resource=\"%s\",reason=\"%s\"} %llu\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                resources[resource as usize],
                recording_io_reason_name(recording_io_reason_t(reason as ::core::ffi::c_uint)),
                (*recording).reason_totals[resource as usize][reason as usize]
                    as ::core::ffi::c_ulonglong,
            );
            reason += 1;
        }
        resource += 1;
    }
    if run_valid {
        let mut boot_id: [::core::ffi::c_char; 129] = [0; 129];
        prom_label_escape(
            &raw const (*run).boot_id as *const ::core::ffi::c_char,
            &raw mut boot_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 129]>(),
        );
        prom_buf_append(
            buf,
            b"# HELP lightnvr_process_start_time_seconds Process run start time in Unix seconds\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        prom_buf_append(
            buf,
            b"# TYPE lightnvr_process_start_time_seconds gauge\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        prom_buf_append(
            buf,
            b"lightnvr_process_start_time_seconds %.3f\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*run).started_at_ms as ::core::ffi::c_double / 1000.0f64,
        );
        prom_buf_append(
            buf,
            b"# HELP lightnvr_system_boot_info Current bounded boot identity for counter-reset provenance\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        prom_buf_append(
            buf,
            b"# TYPE lightnvr_system_boot_info gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        prom_buf_append(
            buf,
            b"lightnvr_system_boot_info{boot_id=\"%s\"} 1\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut boot_id as *mut ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn api_metrics_render_system_health(
    mut snapshot: *const system_health_snapshot_t,
    mut sampler: *const system_health_stats_t,
    mut evaluator: *const system_health_evaluator_stats_t,
    mut incidents: *const system_health_incident_view_t,
    mut incident_count: size_t,
    mut recording: *const recording_io_metrics_snapshot_t,
    mut run: *const system_health_process_run_t,
    mut run_valid: bool,
) -> *mut ::core::ffi::c_char {
    if snapshot.is_null()
        || sampler.is_null()
        || evaluator.is_null()
        || recording.is_null()
        || incidents.is_null() && incident_count != 0 as size_t
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut buffer: prom_buf_t = prom_buf_t {
        data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        cap: 0,
    };
    prom_buf_init(&raw mut buffer, 32768 as size_t);
    if buffer.data.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    prom_buf_append(
        &raw mut buffer,
        b"# HELP lightnvr_health_snapshot_sequence Last completed immutable health snapshot sequence\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"# TYPE lightnvr_health_snapshot_sequence gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buffer,
        b"lightnvr_health_snapshot_sequence %llu\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*snapshot).sequence as ::core::ffi::c_ulonglong,
    );
    append_system_observations(&raw mut buffer, snapshot);
    append_hardware_observations(&raw mut buffer, snapshot);
    append_health_coverage(&raw mut buffer, snapshot);
    append_health_service_metrics(
        &raw mut buffer,
        sampler,
        evaluator,
        incidents,
        incident_count,
        recording,
        run,
        run_valid,
    );
    return buffer.data;
}
unsafe extern "C" fn get_go2rtc_rss_bytes() -> uint64_t {
    unsafe extern "C" {
        #[link_name = "get_go2rtc_memory_usage"]
        unsafe fn get_go2rtc_memory_usage_0(memory_usage: *mut ::core::ffi::c_ulonglong) -> bool;
    }
    let mut mem: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
    if get_go2rtc_memory_usage_0(&raw mut mem) {
        return mem as uint64_t;
    }
    return 0 as uint64_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_metrics(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut health_snapshot: system_health_snapshot_t = system_health_snapshot_t {
        observations: [system_health_observation_t {
            metric: [0; 64],
            resource_id: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            freshness: system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_UNKNOWN,
            unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
            value_valid: false,
            value: 0.,
            sampled_monotonic_ms: 0,
            observed_wall_time_ms: 0,
        }; 256],
        observation_count: 0,
        observations_dropped: 0,
        sequence: 0,
        completed_monotonic_ms: 0,
        completed_wall_time_ms: 0,
    };
    memset(
        &raw mut health_snapshot as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_snapshot_t>(),
    );
    let mut health_available: bool = system_health_snapshot_copy(&raw mut health_snapshot);
    let mut max: ::core::ffi::c_int = metrics_get_max_streams();
    if max <= 0 as ::core::ffi::c_int {
        (*res).status_code = 503 as ::core::ffi::c_int;
        safe_strcpy(
            &raw mut (*res).content_type as *mut ::core::ffi::c_char,
            b"text/plain\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
        http_response_set_body(
            res,
            b"# Metrics subsystem not initialized\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut snaps: *mut stream_metrics_t =
        calloc(max as size_t, ::core::mem::size_of::<stream_metrics_t>()) as *mut stream_metrics_t;
    if snaps.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = metrics_snapshot_all(snaps, max);
    let mut buf: prom_buf_t = prom_buf_t {
        data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        cap: 0,
    };
    prom_buf_init(&raw mut buf, 32768 as size_t);
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_stream_up Whether stream is connected and producing frames\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_stream_up gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_up{stream=\"%s\"} %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i as isize)).stream_up,
        );
        i += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_stream_fps Current measured frame rate\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_stream_fps gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_fps{stream=\"%s\"} %.1f\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_0 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_0 as isize)).current_fps,
        );
        i_0 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_stream_bitrate_bps Current measured bitrate in bits per second\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_stream_bitrate_bps gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_bitrate_bps{stream=\"%s\"} %.0f\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_1 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_1 as isize)).current_bitrate_bps,
        );
        i_1 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_stream_frames_total Total frames received since stream start\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_stream_frames_total counter\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_2 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_frames_total{stream=\"%s\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_2 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_2 as isize)).frames_total as ::core::ffi::c_ulonglong,
        );
        i_2 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_stream_frames_dropped Frames dropped due to errors\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_stream_frames_dropped counter\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_3 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_frames_dropped{stream=\"%s\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_3 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_3 as isize)).frames_dropped as ::core::ffi::c_ulonglong,
        );
        i_3 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_stream_reconnects_total Number of RTSP reconnection events\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_stream_reconnects_total counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_4 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_reconnects_total{stream=\"%s\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_4 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_4 as isize)).reconnects_total as ::core::ffi::c_ulonglong,
        );
        i_4 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_stream_uptime_seconds Cumulative seconds the stream has been up\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_stream_uptime_seconds counter\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_5 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_uptime_seconds{stream=\"%s\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_5 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_5 as isize)).uptime_seconds as ::core::ffi::c_ulonglong,
        );
        i_5 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_stream_last_frame_ts Timestamp of last received frame\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_stream_last_frame_ts gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_6 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_last_frame_ts{stream=\"%s\"} %lld\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_6 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_6 as isize)).last_frame_ts as ::core::ffi::c_longlong,
        );
        i_6 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_stream_connection_latency_ms Time from RTSP SETUP to first frame\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_stream_connection_latency_ms gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_7 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_connection_latency_ms{stream=\"%s\"} %.1f\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_7 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_7 as isize)).connection_latency_ms,
        );
        i_7 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_stream_error_total Total stream errors by type\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_stream_error_total counter\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_8 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_error_total{stream=\"%s\",type=\"decode\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_8 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_8 as isize)).error_decode as ::core::ffi::c_ulonglong,
        );
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_error_total{stream=\"%s\",type=\"timeout\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_8 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_8 as isize)).error_timeout as ::core::ffi::c_ulonglong,
        );
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_error_total{stream=\"%s\",type=\"protocol\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_8 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_8 as isize)).error_protocol as ::core::ffi::c_ulonglong,
        );
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_stream_error_total{stream=\"%s\",type=\"io\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_8 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_8 as isize)).error_io as ::core::ffi::c_ulonglong,
        );
        i_8 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_recording_active Whether recording is active for stream\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_recording_active gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_9 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_recording_active{stream=\"%s\"} %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_9 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_9 as isize)).recording_active,
        );
        i_9 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_recording_bytes_written Total bytes written to storage\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_recording_bytes_written counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut i_10: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_10 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_recording_bytes_written{stream=\"%s\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_10 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_10 as isize)).recording_bytes_written as ::core::ffi::c_ulonglong,
        );
        i_10 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_recording_segments_total Total recording segments created\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_recording_segments_total counter\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut i_11: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_11 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_recording_segments_total{stream=\"%s\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_11 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_11 as isize)).recording_segments_total as ::core::ffi::c_ulonglong,
        );
        i_11 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_recording_gaps_total Recording gaps detected\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_recording_gaps_total counter\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i_12: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_12 < count {
        prom_buf_append(
            &raw mut buf,
            b"lightnvr_recording_gaps_total{stream=\"%s\"} %llu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*snaps.offset(i_12 as isize)).stream_name as *mut ::core::ffi::c_char,
            (*snaps.offset(i_12 as isize)).recording_gaps_total as ::core::ffi::c_ulonglong,
        );
        i_12 += 1;
    }
    let mut storage_health: storage_health_t = storage_health_t {
        pressure_level: disk_pressure_level_t::DISK_PRESSURE_NORMAL,
        free_space_pct: 0.,
        free_space_bytes: 0,
        total_space_bytes: 0,
        used_space_bytes: 0,
        last_check_time: 0,
        last_cleanup_time: 0,
        last_deep_time: 0,
        last_cleanup_deleted: 0,
        last_cleanup_freed: 0,
    };
    get_storage_health(&raw mut storage_health);
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_storage_used_bytes Total storage consumed by recordings\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_storage_used_bytes gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"lightnvr_storage_used_bytes %.0f\n\0".as_ptr() as *const ::core::ffi::c_char,
        storage_health.used_space_bytes as ::core::ffi::c_double,
    );
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_storage_available_bytes Available storage on recording volume\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_storage_available_bytes gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"lightnvr_storage_available_bytes %.0f\n\0".as_ptr() as *const ::core::ffi::c_char,
        storage_health.free_space_bytes as ::core::ffi::c_double,
    );
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_instance_streams_configured Number of streams configured\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_instance_streams_configured gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"lightnvr_instance_streams_configured %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        get_total_stream_count(),
    );
    let mut streams_up: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_13: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_13 < count {
        if (*snaps.offset(i_13 as isize)).stream_up != 0 {
            streams_up += 1;
        }
        i_13 += 1;
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_instance_streams_up Number of streams currently up\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_instance_streams_up gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"lightnvr_instance_streams_up %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        streams_up,
    );
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_instance_cpu_percent Process CPU usage\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_instance_cpu_percent gauge\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if health_available {
        let mut process_cpu: *const system_health_observation_t = find_observation(
            &raw mut health_snapshot,
            b"process.cpu.usage_ratio\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if observation_usable(process_cpu) {
            prom_buf_append(
                &raw mut buf,
                b"lightnvr_instance_cpu_percent %.17g\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*process_cpu).value * 100.0f64,
            );
        }
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_instance_memory_rss_bytes Resident set size of lightnvr process\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_instance_memory_rss_bytes gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if health_available {
        append_single_observation(
            &raw mut buf,
            b"lightnvr_instance_memory_rss_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            find_observation(
                &raw mut health_snapshot,
                b"process.rss_bytes\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ),
        );
    }
    prom_buf_append(
        &raw mut buf,
        b"# HELP lightnvr_instance_go2rtc_memory_bytes RSS of go2rtc companion process\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"# TYPE lightnvr_instance_go2rtc_memory_bytes gauge\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    prom_buf_append(
        &raw mut buf,
        b"lightnvr_instance_go2rtc_memory_bytes %llu\n\0".as_ptr() as *const ::core::ffi::c_char,
        get_go2rtc_rss_bytes() as ::core::ffi::c_ulonglong,
    );
    if health_available {
        let mut sampler_stats: system_health_stats_t = system_health_stats_t {
            initialized: false,
            enabled: false,
            ring_allocated: false,
            worker_threads: 0,
            generations_completed: 0,
            collections_completed: 0,
            collection_errors: 0,
            collection_timeouts: 0,
            overlap_skips: 0,
            observations_dropped: 0,
            coverage_overflows: 0,
            abandoned_helpers: 0,
        };
        let mut evaluator_stats: system_health_evaluator_stats_t =
            system_health_evaluator_stats_t {
                tracked_conditions: 0,
                active_incidents: 0,
                pending_persistence: 0,
                transitions: 0,
                persistence_failures: 0,
                persistence_retries: 0,
                oldest_pending_wall_time_ms: 0,
            };
        let mut incidents: [system_health_incident_view_t; 64] = [system_health_incident_view_t {
            incident_id: [0; 37],
            condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
            subject: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
            severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
            first_observed_at_ms: 0,
            last_observed_at_ms: 0,
            observation: system_health_observation_t {
                metric: [0; 64],
                resource_id: [0; 64],
                scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
                capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                freshness: system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_UNKNOWN,
                unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
                value_valid: false,
                value: 0.,
                sampled_monotonic_ms: 0,
                observed_wall_time_ms: 0,
            },
            persistence_pending: false,
        }; 64];
        let mut recording_metrics: recording_io_metrics_snapshot_t =
            recording_io_metrics_snapshot_t {
                reason_totals: [[0; 9]; 2],
                last_error: recording_io_last_error_t {
                    valid: false,
                    resource: recording_io_resource_t::RECORDING_IO_RESOURCE_RECORDING,
                    operation: recording_io_operation_t::RECORDING_IO_OPERATION_ALLOCATE,
                    reason: recording_io_reason_t::RECORDING_IO_REASON_NONE,
                    error_code: 0,
                },
            };
        let mut run: system_health_process_run_t = system_health_process_run_t {
            run_id: [0; 37],
            boot_id: [0; 64],
            started_at_ms: 0,
            closed_at_ms: 0,
            clean_close: false,
        };
        system_health_get_stats(&raw mut sampler_stats);
        system_health_evaluator_service_get_stats(&raw mut evaluator_stats);
        let mut incident_count: size_t = system_health_evaluator_service_active_copy(
            &raw mut incidents as *mut system_health_incident_view_t,
            SYSTEM_HEALTH_MAX_INCIDENTS as size_t,
        );
        recording_io_metrics_snapshot(&raw mut recording_metrics);
        let mut run_valid: bool = system_health_evaluator_service_copy_run(&raw mut run);
        let mut health_metrics: *mut ::core::ffi::c_char = api_metrics_render_system_health(
            &raw mut health_snapshot,
            &raw mut sampler_stats,
            &raw mut evaluator_stats,
            &raw mut incidents as *mut system_health_incident_view_t,
            incident_count,
            &raw mut recording_metrics,
            &raw mut run,
            run_valid,
        );
        if !health_metrics.is_null() {
            prom_buf_append(
                &raw mut buf,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                health_metrics,
            );
            free(health_metrics as *mut ::core::ffi::c_void);
        }
    }
    let mut self_metrics: *mut ::core::ffi::c_char = api_metrics_render_self_observability();
    if !self_metrics.is_null() {
        prom_buf_append(
            &raw mut buf,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            self_metrics,
        );
        free(self_metrics as *mut ::core::ffi::c_void);
    }
    (*res).status_code = 200 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut (*res).content_type as *mut ::core::ffi::c_char,
        b"text/plain; version=0.0.4; charset=utf-8\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    http_response_set_body(
        res,
        if !buf.data.is_null() {
            buf.data
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    prom_buf_free(&raw mut buf);
    free(snaps as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_player_telemetry(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if (*req).body.is_null() || (*req).body_len == 0 as size_t {
        (*res).status_code = 204 as ::core::ffi::c_int;
        return;
    }
    let mut body_str: *mut ::core::ffi::c_char =
        malloc((*req).body_len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if body_str.is_null() {
        (*res).status_code = 204 as ::core::ffi::c_int;
        return;
    }
    memcpy(
        body_str as *mut ::core::ffi::c_void,
        (*req).body,
        (*req).body_len,
    );
    *body_str.offset((*req).body_len as isize) = '\0' as ::core::ffi::c_char;
    let mut json: *mut cJSON = cJSON_Parse(body_str);
    free(body_str as *mut ::core::ffi::c_void);
    if json.is_null() {
        (*res).status_code = 204 as ::core::ffi::c_int;
        return;
    }
    let mut event: player_telemetry_event_t = player_telemetry_event_t {
        stream_name: [0; 256],
        session_id: [0; 64],
        transport: [0; 16],
        ttff_ms: 0.,
        rebuffer_count: 0,
        rebuffer_duration_ms: 0.,
        resolution_switches: 0,
        webrtc_rtt_ms: 0.,
        timestamp: 0,
    };
    memset(
        &raw mut event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<player_telemetry_event_t>(),
    );
    let mut item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    item = cJSON_GetObjectItem(
        json,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !item.is_null() && cJSON_IsString(item) != 0 {
        safe_strcpy(
            &raw mut event.stream_name as *mut ::core::ffi::c_char,
            (*item).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    }
    item = cJSON_GetObjectItem(json, b"session_id\0".as_ptr() as *const ::core::ffi::c_char);
    if !item.is_null() && cJSON_IsString(item) != 0 {
        safe_strcpy(
            &raw mut event.session_id as *mut ::core::ffi::c_char,
            (*item).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    item = cJSON_GetObjectItem(json, b"transport\0".as_ptr() as *const ::core::ffi::c_char);
    if !item.is_null() && cJSON_IsString(item) != 0 {
        safe_strcpy(
            &raw mut event.transport as *mut ::core::ffi::c_char,
            (*item).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    }
    item = cJSON_GetObjectItem(json, b"ttff_ms\0".as_ptr() as *const ::core::ffi::c_char);
    if !item.is_null() && cJSON_IsNumber(item) != 0 {
        event.ttff_ms = (*item).valuedouble;
    }
    item = cJSON_GetObjectItem(
        json,
        b"rebuffer_count\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !item.is_null() && cJSON_IsNumber(item) != 0 {
        event.rebuffer_count = (*item).valueint;
    }
    item = cJSON_GetObjectItem(
        json,
        b"rebuffer_duration_ms\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !item.is_null() && cJSON_IsNumber(item) != 0 {
        event.rebuffer_duration_ms = (*item).valuedouble;
    }
    item = cJSON_GetObjectItem(
        json,
        b"resolution_switches\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !item.is_null() && cJSON_IsNumber(item) != 0 {
        event.resolution_switches = (*item).valueint;
    }
    item = cJSON_GetObjectItem(
        json,
        b"webrtc_rtt_ms\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !item.is_null() && cJSON_IsNumber(item) != 0 {
        event.webrtc_rtt_ms = (*item).valuedouble;
    }
    event.timestamp = time(::core::ptr::null_mut::<time_t>());
    cJSON_Delete(json);
    if event.stream_name[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        (*res).status_code = 204 as ::core::ffi::c_int;
        return;
    }
    if httpd_authorize_stream_action(
        req,
        res,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut event.stream_name as *mut ::core::ffi::c_char,
    ) == 0
    {
        return;
    }
    player_telemetry_record(&raw mut event);
    (*res).status_code = 204 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
