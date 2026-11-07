unsafe extern "C" {
    pub type mosquitto;
    unsafe fn db_event_destination_validate(
        destination: *const event_destination_t,
        password: *const ::core::ffi::c_char,
        validate_password: bool,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> db_event_destination_result_t;
    unsafe fn db_event_destination_get_password(
        uuid: *const ::core::ffi::c_char,
        expected_revision: int64_t,
        password: *mut ::core::ffi::c_char,
        password_size: size_t,
    ) -> db_event_destination_result_t;
    unsafe fn mosquitto_username_pw_set(
        mosq: *mut mosquitto,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_connect_callback_set(
        mosq: *mut mosquitto,
        on_connect_0: LIBMOSQ_CB_connect,
    );
    unsafe fn mosquitto_disconnect_callback_set(
        mosq: *mut mosquitto,
        on_disconnect_0: LIBMOSQ_CB_disconnect,
    );
    unsafe fn mosquitto_publish_callback_set(
        mosq: *mut mosquitto,
        on_publish_0: LIBMOSQ_CB_publish,
    );
    unsafe fn mosquitto_log_callback_set(mosq: *mut mosquitto, on_log_0: LIBMOSQ_CB_log);
    unsafe fn mosquitto_connect_async(
        mosq: *mut mosquitto,
        host: *const ::core::ffi::c_char,
        port: ::core::ffi::c_int,
        keepalive: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_disconnect(mosq: *mut mosquitto) -> ::core::ffi::c_int;
    unsafe fn mosquitto_new(
        id: *const ::core::ffi::c_char,
        clean_session: bool,
        obj: *mut ::core::ffi::c_void,
    ) -> *mut mosquitto;
    unsafe fn mosquitto_destroy(mosq: *mut mosquitto);
    unsafe fn mosquitto_loop_start(mosq: *mut mosquitto) -> ::core::ffi::c_int;
    unsafe fn mosquitto_loop_stop(mosq: *mut mosquitto, force: bool) -> ::core::ffi::c_int;
    unsafe fn mosquitto_int_option(
        mosq: *mut mosquitto,
        option: mosq_opt_t,
        value: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_reconnect_delay_set(
        mosq: *mut mosquitto,
        reconnect_delay: ::core::ffi::c_uint,
        reconnect_delay_max: ::core::ffi::c_uint,
        reconnect_exponential_backoff: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_publish(
        mosq: *mut mosquitto,
        mid: *mut ::core::ffi::c_int,
        topic: *const ::core::ffi::c_char,
        payloadlen: ::core::ffi::c_int,
        payload: *const ::core::ffi::c_void,
        qos: ::core::ffi::c_int,
        retain: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_tls_set(
        mosq: *mut mosquitto,
        cafile: *const ::core::ffi::c_char,
        capath: *const ::core::ffi::c_char,
        certfile: *const ::core::ffi::c_char,
        keyfile: *const ::core::ffi::c_char,
        pw_callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_tls_opts_set(
        mosq: *mut mosquitto,
        cert_reqs: ::core::ffi::c_int,
        tls_version: *const ::core::ffi::c_char,
        ciphers: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_will_set(
        mosq: *mut mosquitto,
        topic: *const ::core::ffi::c_char,
        payloadlen: ::core::ffi::c_int,
        payload: *const ::core::ffi::c_void,
        qos: ::core::ffi::c_int,
        retain: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn mosquitto_strerror(mosq_errno: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn mosquitto_connack_string(
        connack_code: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
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
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn event_identity_get_source(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn system_health_snapshot_copy(snapshot: *mut system_health_snapshot_t) -> bool;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn system_health_evaluator_service_active_copy(
        incidents: *mut system_health_incident_view_t,
        capacity: size_t,
    ) -> size_t;
    unsafe fn system_health_evaluator_service_copy_run(
        run: *mut system_health_process_run_t,
    ) -> bool;
    unsafe fn secure_zero_memory(ptr: *mut ::core::ffi::c_void, size: size_t);
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
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
pub union pthread_condattr_t {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
pub type uint64_t = u64;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_event_destination_result_t(pub ::core::ffi::c_int);
impl db_event_destination_result_t {
    pub const DB_EVENT_DESTINATION_OK: Self = Self(0);
    pub const DB_EVENT_DESTINATION_NOT_FOUND: Self = Self(-1);
    pub const DB_EVENT_DESTINATION_CONFLICT: Self = Self(-2);
    pub const DB_EVENT_DESTINATION_INVALID: Self = Self(-3);
    pub const DB_EVENT_DESTINATION_STALE: Self = Self(-4);
    pub const DB_EVENT_DESTINATION_LIMIT: Self = Self(-5);
    pub const DB_EVENT_DESTINATION_IN_USE: Self = Self(-6);
    pub const DB_EVENT_DESTINATION_ERROR: Self = Self(-7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mqtt_destination_client {
    pub profile: event_destination_t,
    pub mosq: *mut mosquitto,
    pub mutex: pthread_mutex_t,
    pub wake: pthread_cond_t,
    pub heartbeat_wake: pthread_cond_t,
    pub heartbeat_thread: pthread_t,
    pub synchronization_ready: bool,
    pub heartbeat_condition_ready: bool,
    pub heartbeat_started: bool,
    pub heartbeat_stop: bool,
    pub loop_started: bool,
    pub connected: bool,
    pub ever_connected: bool,
    pub shutting_down: bool,
    pub publish_waiting: bool,
    pub publish_acknowledged: bool,
    pub publish_failed: bool,
    pub presence_configured: bool,
    pub publish_mid: ::core::ffi::c_int,
    pub stats_slot: ::core::ffi::c_int,
    pub presence_sequence: uint64_t,
    pub status_topic: [::core::ffi::c_char; 512],
    pub installation_uuid: [::core::ffi::c_char; 37],
    pub run_id: [::core::ffi::c_char; 37],
    pub boot_id: [::core::ffi::c_char; 64],
}
pub type mqtt_destination_client_t = mqtt_destination_client;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct managed_stats_slot_t {
    pub occupied: bool,
    pub value: mqtt_destination_client_stats_t,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mqtt_presence_state_t(pub ::core::ffi::c_uint);
impl mqtt_presence_state_t {
    pub const MQTT_PRESENCE_OFFLINE: Self = Self(0);
    pub const MQTT_PRESENCE_ONLINE: Self = Self(1);
    pub const MQTT_PRESENCE_STOPPING: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mqtt_operational_state_t(pub ::core::ffi::c_uint);
impl mqtt_operational_state_t {
    pub const MQTT_OPERATIONAL_UNKNOWN: Self = Self(0);
    pub const MQTT_OPERATIONAL_HEALTHY: Self = Self(1);
    pub const MQTT_OPERATIONAL_WARNING: Self = Self(2);
    pub const MQTT_OPERATIONAL_ERROR: Self = Self(3);
    pub const MQTT_OPERATIONAL_CRITICAL: Self = Self(4);
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
pub struct system_health_freshness_t(pub ::core::ffi::c_uint);
impl system_health_freshness_t {
    pub const SYSTEM_HEALTH_FRESHNESS_UNKNOWN: Self = Self(0);
    pub const SYSTEM_HEALTH_FRESHNESS_FRESH: Self = Self(1);
    pub const SYSTEM_HEALTH_FRESHNESS_STALE: Self = Self(2);
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
pub struct system_health_severity_t(pub ::core::ffi::c_uint);
impl system_health_severity_t {
    pub const SYSTEM_HEALTH_SEVERITY_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_SEVERITY_WARNING: Self = Self(1);
    pub const SYSTEM_HEALTH_SEVERITY_ERROR: Self = Self(2);
    pub const SYSTEM_HEALTH_SEVERITY_CRITICAL: Self = Self(3);
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
pub struct system_health_process_run_t {
    pub run_id: [::core::ffi::c_char; 37],
    pub boot_id: [::core::ffi::c_char; 64],
    pub started_at_ms: int64_t,
    pub closed_at_ms: int64_t,
    pub clean_close: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mosq_opt_t(pub ::core::ffi::c_uint);
impl mosq_opt_t {
    pub const MOSQ_OPT_PROTOCOL_VERSION: Self = Self(1);
    pub const MOSQ_OPT_SSL_CTX: Self = Self(2);
    pub const MOSQ_OPT_SSL_CTX_WITH_DEFAULTS: Self = Self(3);
    pub const MOSQ_OPT_RECEIVE_MAXIMUM: Self = Self(4);
    pub const MOSQ_OPT_SEND_MAXIMUM: Self = Self(5);
    pub const MOSQ_OPT_TLS_KEYFORM: Self = Self(6);
    pub const MOSQ_OPT_TLS_ENGINE: Self = Self(7);
    pub const MOSQ_OPT_TLS_ENGINE_KPASS_SHA1: Self = Self(8);
    pub const MOSQ_OPT_TLS_OCSP_REQUIRED: Self = Self(9);
    pub const MOSQ_OPT_TLS_ALPN: Self = Self(10);
    pub const MOSQ_OPT_TCP_NODELAY: Self = Self(11);
    pub const MOSQ_OPT_BIND_ADDRESS: Self = Self(12);
    pub const MOSQ_OPT_TLS_USE_OS_CERTS: Self = Self(13);
    pub const MOSQ_OPT_DISABLE_SOCKETPAIR: Self = Self(14);
    pub const MOSQ_OPT_TRANSPORT: Self = Self(15);
    pub const MOSQ_OPT_HTTP_PATH: Self = Self(16);
    pub const MOSQ_OPT_HTTP_HEADER_SIZE: Self = Self(17);
}
pub type LIBMOSQ_CB_log = Option<
    unsafe extern "C" fn(
        *mut mosquitto,
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
    ) -> (),
>;
pub type LIBMOSQ_CB_publish = Option<
    unsafe extern "C" fn(*mut mosquitto, *mut ::core::ffi::c_void, ::core::ffi::c_int) -> (),
>;
pub type LIBMOSQ_CB_disconnect = Option<
    unsafe extern "C" fn(*mut mosquitto, *mut ::core::ffi::c_void, ::core::ffi::c_int) -> (),
>;
pub type LIBMOSQ_CB_connect = Option<
    unsafe extern "C" fn(*mut mosquitto, *mut ::core::ffi::c_void, ::core::ffi::c_int) -> (),
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mosq_err_t(pub ::core::ffi::c_int);
impl mosq_err_t {
    pub const MOSQ_ERR_QUOTA_EXCEEDED: Self = Self(-6);
    pub const MOSQ_ERR_AUTH_DELAYED: Self = Self(-5);
    pub const MOSQ_ERR_AUTH_CONTINUE: Self = Self(-4);
    pub const MOSQ_ERR_NO_SUBSCRIBERS: Self = Self(-3);
    pub const MOSQ_ERR_SUB_EXISTS: Self = Self(-2);
    pub const MOSQ_ERR_CONN_PENDING: Self = Self(-1);
    pub const MOSQ_ERR_SUCCESS: Self = Self(0);
    pub const MOSQ_ERR_NOMEM: Self = Self(1);
    pub const MOSQ_ERR_PROTOCOL: Self = Self(2);
    pub const MOSQ_ERR_INVAL: Self = Self(3);
    pub const MOSQ_ERR_NO_CONN: Self = Self(4);
    pub const MOSQ_ERR_CONN_REFUSED: Self = Self(5);
    pub const MOSQ_ERR_NOT_FOUND: Self = Self(6);
    pub const MOSQ_ERR_CONN_LOST: Self = Self(7);
    pub const MOSQ_ERR_TLS: Self = Self(8);
    pub const MOSQ_ERR_PAYLOAD_SIZE: Self = Self(9);
    pub const MOSQ_ERR_NOT_SUPPORTED: Self = Self(10);
    pub const MOSQ_ERR_AUTH: Self = Self(11);
    pub const MOSQ_ERR_ACL_DENIED: Self = Self(12);
    pub const MOSQ_ERR_UNKNOWN: Self = Self(13);
    pub const MOSQ_ERR_ERRNO: Self = Self(14);
    pub const MOSQ_ERR_EAI: Self = Self(15);
    pub const MOSQ_ERR_PROXY: Self = Self(16);
    pub const MOSQ_ERR_PLUGIN_DEFER: Self = Self(17);
    pub const MOSQ_ERR_MALFORMED_UTF8: Self = Self(18);
    pub const MOSQ_ERR_KEEPALIVE: Self = Self(19);
    pub const MOSQ_ERR_LOOKUP: Self = Self(20);
    pub const MOSQ_ERR_MALFORMED_PACKET: Self = Self(21);
    pub const MOSQ_ERR_DUPLICATE_PROPERTY: Self = Self(22);
    pub const MOSQ_ERR_TLS_HANDSHAKE: Self = Self(23);
    pub const MOSQ_ERR_QOS_NOT_SUPPORTED: Self = Self(24);
    pub const MOSQ_ERR_OVERSIZE_PACKET: Self = Self(25);
    pub const MOSQ_ERR_OCSP: Self = Self(26);
    pub const MOSQ_ERR_TIMEOUT: Self = Self(27);
    pub const MOSQ_ERR_ALREADY_EXISTS: Self = Self(31);
    pub const MOSQ_ERR_PLUGIN_IGNORE: Self = Self(32);
    pub const MOSQ_ERR_HTTP_BAD_ORIGIN: Self = Self(33);
    pub const MOSQ_ERR_UNSPECIFIED: Self = Self(128);
    pub const MOSQ_ERR_IMPLEMENTATION_SPECIFIC: Self = Self(131);
    pub const MOSQ_ERR_UNSUPPORTED_PROTOCOL_VERSION: Self = Self(132);
    pub const MOSQ_ERR_CLIENT_IDENTIFIER_NOT_VALID: Self = Self(133);
    pub const MOSQ_ERR_BAD_USERNAME_OR_PASSWORD: Self = Self(134);
    pub const MOSQ_ERR_SERVER_UNAVAILABLE: Self = Self(136);
    pub const MOSQ_ERR_SERVER_BUSY: Self = Self(137);
    pub const MOSQ_ERR_BANNED: Self = Self(138);
    pub const MOSQ_ERR_SERVER_SHUTTING_DOWN: Self = Self(139);
    pub const MOSQ_ERR_BAD_AUTHENTICATION_METHOD: Self = Self(140);
    pub const MOSQ_ERR_SESSION_TAKEN_OVER: Self = Self(142);
    pub const MOSQ_ERR_TOPIC_FILTER_INVALID: Self = Self(143);
    pub const MOSQ_ERR_TOPIC_NAME_INVALID: Self = Self(144);
    pub const MOSQ_ERR_PACKET_ID_IN_USE: Self = Self(145);
    pub const MOSQ_ERR_PACKET_ID_NOT_FOUND: Self = Self(146);
    pub const MOSQ_ERR_RECEIVE_MAXIMUM_EXCEEDED: Self = Self(147);
    pub const MOSQ_ERR_TOPIC_ALIAS_INVALID: Self = Self(148);
    pub const MOSQ_ERR_MESSAGE_RATE_TOO_HIGH: Self = Self(150);
    pub const MOSQ_ERR_ADMINISTRATIVE_ACTION: Self = Self(152);
    pub const MOSQ_ERR_PAYLOAD_FORMAT_INVALID: Self = Self(153);
    pub const MOSQ_ERR_RETAIN_NOT_SUPPORTED: Self = Self(154);
    pub const MOSQ_ERR_USE_ANOTHER_SERVER: Self = Self(156);
    pub const MOSQ_ERR_SERVER_MOVED: Self = Self(157);
    pub const MOSQ_ERR_SHARED_SUBS_NOT_SUPPORTED: Self = Self(158);
    pub const MOSQ_ERR_CONNECTION_RATE_EXCEEDED: Self = Self(159);
    pub const MOSQ_ERR_MAXIMUM_CONNECT_TIME: Self = Self(160);
    pub const MOSQ_ERR_SUBSCRIPTION_IDS_NOT_SUPPORTED: Self = Self(161);
    pub const MOSQ_ERR_WILDCARD_SUBS_NOT_SUPPORTED: Self = Self(162);
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
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const EVENT_DESTINATION_MAX_COUNT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_STATUS_INSTALLATION_PLACEHOLDER: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"{installation_uuid}\0")
};
pub const EVENT_DESTINATION_STATUS_DESTINATION_PLACEHOLDER: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"{destination_uuid}\0")
};
pub const LIGHTNVR_VERSION_STRING: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"0.47.2\0") };
pub const MOSQ_LOG_WARNING: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const MOSQ_LOG_ERR: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"MQTTDestination\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const SYSTEM_HEALTH_MAX_INCIDENTS: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const MANAGED_PRESENCE_INTERVAL_SECONDS: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const MANAGED_PRESENCE_VISIBILITY: [::core::ffi::c_char; 34] = unsafe {
    ::core::mem::transmute::<[u8; 34], [::core::ffi::c_char; 34]>(
        *b"process,container,host,filesystem\0",
    )
};
static mut stats_lock: pthread_mutex_t = pthread_mutex_t {
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
static mut stats_slots: [managed_stats_slot_t; 64] = [managed_stats_slot_t {
    occupied: false,
    value: mqtt_destination_client_stats_t {
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
    },
}; 64];
unsafe extern "C" fn increment_saturated(mut value: uint64_t) -> uint64_t {
    return if value == UINT64_MAX as uint64_t {
        value
    } else {
        value.wrapping_add(1 as uint64_t)
    };
}
unsafe extern "C" fn wall_time_ms() -> int64_t {
    let mut value: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_REALTIME, &raw mut value) != 0 as ::core::ffi::c_int {
        return 0 as int64_t;
    }
    return value.tv_sec as int64_t * 1000 as int64_t
        + value.tv_nsec as int64_t / 1000000 as int64_t;
}
unsafe extern "C" fn reserve_stats_slot(
    mut profile: *const event_destination_t,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    pthread_mutex_lock(&raw mut stats_lock);
    let mut current: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while current < EVENT_DESTINATION_MAX_COUNT {
        if !stats_slots[current as usize].occupied {
            index = current;
            memset(
                (&raw mut stats_slots as *mut managed_stats_slot_t).offset(current as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<managed_stats_slot_t>(),
            );
            stats_slots[current as usize].occupied = r#true != 0;
            snprintf(
                &raw mut (*(&raw mut stats_slots as *mut managed_stats_slot_t)
                    .offset(current as isize))
                .value
                .destination_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*profile).uuid as *const ::core::ffi::c_char,
            );
            stats_slots[current as usize].value.presence_configured =
                (*profile).status_topic_template[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int;
            break;
        } else {
            current += 1;
        }
    }
    pthread_mutex_unlock(&raw mut stats_lock);
    return index;
}
unsafe extern "C" fn release_stats_slot(mut index: ::core::ffi::c_int) {
    if index < 0 as ::core::ffi::c_int || index >= EVENT_DESTINATION_MAX_COUNT {
        return;
    }
    pthread_mutex_lock(&raw mut stats_lock);
    memset(
        (&raw mut stats_slots as *mut managed_stats_slot_t).offset(index as isize)
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<managed_stats_slot_t>(),
    );
    pthread_mutex_unlock(&raw mut stats_lock);
}
unsafe extern "C" fn stats_connection(
    mut index: ::core::ffi::c_int,
    mut connected: bool,
    mut reconnect: bool,
) {
    if index < 0 as ::core::ffi::c_int || index >= EVENT_DESTINATION_MAX_COUNT {
        return;
    }
    pthread_mutex_lock(&raw mut stats_lock);
    if stats_slots[index as usize].occupied {
        let mut stats: *mut mqtt_destination_client_stats_t =
            &raw mut (*(&raw mut stats_slots as *mut managed_stats_slot_t).offset(index as isize))
                .value;
        (*stats).connected = connected;
        if connected {
            (*stats).connections = increment_saturated((*stats).connections);
            if reconnect {
                (*stats).reconnects = increment_saturated((*stats).reconnects);
            }
        } else {
            (*stats).disconnects = increment_saturated((*stats).disconnects);
        }
    }
    pthread_mutex_unlock(&raw mut stats_lock);
}
unsafe extern "C" fn stats_publish(
    mut index: ::core::ffi::c_int,
    mut succeeded: bool,
    mut observed_at_ms: int64_t,
) {
    if index < 0 as ::core::ffi::c_int || index >= EVENT_DESTINATION_MAX_COUNT {
        return;
    }
    pthread_mutex_lock(&raw mut stats_lock);
    if stats_slots[index as usize].occupied {
        let mut stats: *mut mqtt_destination_client_stats_t =
            &raw mut (*(&raw mut stats_slots as *mut managed_stats_slot_t).offset(index as isize))
                .value;
        if succeeded {
            (*stats).publish_successes = increment_saturated((*stats).publish_successes);
            if observed_at_ms > 0 as int64_t {
                (*stats).last_success_at_ms = observed_at_ms;
            }
        } else {
            (*stats).publish_failures = increment_saturated((*stats).publish_failures);
            (*stats).last_failure =
                mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_PUBLICATION;
            if observed_at_ms > 0 as int64_t {
                (*stats).last_failure_at_ms = observed_at_ms;
            }
        }
    }
    pthread_mutex_unlock(&raw mut stats_lock);
}
unsafe extern "C" fn stats_failure(
    mut index: ::core::ffi::c_int,
    mut failure: mqtt_destination_failure_t,
) {
    if index < 0 as ::core::ffi::c_int || index >= EVENT_DESTINATION_MAX_COUNT {
        return;
    }
    pthread_mutex_lock(&raw mut stats_lock);
    if stats_slots[index as usize].occupied {
        let mut stats: *mut mqtt_destination_client_stats_t =
            &raw mut (*(&raw mut stats_slots as *mut managed_stats_slot_t).offset(index as isize))
                .value;
        if failure.0 == mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_CONNECTION.0 {
            (*stats).connection_failures = increment_saturated((*stats).connection_failures);
        }
        (*stats).last_failure = failure;
        (*stats).last_failure_at_ms = wall_time_ms();
    }
    pthread_mutex_unlock(&raw mut stats_lock);
}
unsafe extern "C" fn replace_placeholder(
    mut output: *mut ::core::ffi::c_char,
    mut capacity: size_t,
    mut used: *mut size_t,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut length: size_t = strlen(value);
    if (*used).wrapping_add(length) >= capacity {
        return r#false != 0;
    }
    memcpy(
        output.offset(*used as isize) as *mut ::core::ffi::c_void,
        value as *const ::core::ffi::c_void,
        length,
    );
    *used = (*used).wrapping_add(length);
    return r#true != 0;
}
unsafe extern "C" fn expand_status_topic(
    mut profile: *const event_destination_t,
    mut installation_uuid: *const ::core::ffi::c_char,
    mut topic: *mut ::core::ffi::c_char,
    mut topic_size: size_t,
) -> ::core::ffi::c_int {
    if profile.is_null()
        || installation_uuid.is_null()
        || topic.is_null()
        || topic_size == 0 as size_t
        || !lightnvr_uuid_is_valid(installation_uuid)
        || !lightnvr_uuid_is_valid(&raw const (*profile).uuid as *const ::core::ffi::c_char)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut used: size_t = 0 as size_t;
    let mut input: *const ::core::ffi::c_char =
        &raw const (*profile).status_topic_template as *const ::core::ffi::c_char;
    while *input != 0 {
        if strncmp(
            input,
            EVENT_DESTINATION_STATUS_INSTALLATION_PLACEHOLDER.as_ptr(),
            ::core::mem::size_of::<[::core::ffi::c_char; 20]>().wrapping_sub(1 as size_t),
        ) == 0 as ::core::ffi::c_int
        {
            if !replace_placeholder(topic, topic_size, &raw mut used, installation_uuid) {
                return -1 as ::core::ffi::c_int;
            }
            input = input.offset(
                ::core::mem::size_of::<[::core::ffi::c_char; 20]>().wrapping_sub(1usize) as isize,
            );
        } else if strncmp(
            input,
            EVENT_DESTINATION_STATUS_DESTINATION_PLACEHOLDER.as_ptr(),
            ::core::mem::size_of::<[::core::ffi::c_char; 19]>().wrapping_sub(1 as size_t),
        ) == 0 as ::core::ffi::c_int
        {
            if !replace_placeholder(
                topic,
                topic_size,
                &raw mut used,
                &raw const (*profile).uuid as *const ::core::ffi::c_char,
            ) {
                return -1 as ::core::ffi::c_int;
            }
            input = input.offset(
                ::core::mem::size_of::<[::core::ffi::c_char; 19]>().wrapping_sub(1usize) as isize,
            );
        } else {
            if used.wrapping_add(1 as size_t) >= topic_size {
                return -1 as ::core::ffi::c_int;
            }
            let c2rust_fresh0 = input;
            input = input.offset(1);
            let c2rust_fresh1 = used;
            used = used.wrapping_add(1);
            *topic.offset(c2rust_fresh1 as isize) = *c2rust_fresh0;
        }
    }
    *topic.offset(used as isize) = '\0' as ::core::ffi::c_char;
    return if used > 0 as size_t {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn presence_state_name(
    mut state: mqtt_presence_state_t,
) -> *const ::core::ffi::c_char {
    match state {
        mqtt_presence_state_t::MQTT_PRESENCE_OFFLINE => {
            return b"offline\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_presence_state_t::MQTT_PRESENCE_ONLINE => {
            return b"online\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_presence_state_t::MQTT_PRESENCE_STOPPING => {
            return b"stopping\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return ::core::ptr::null::<::core::ffi::c_char>(),
    };
}
unsafe extern "C" fn operational_state_name(
    mut state: mqtt_operational_state_t,
) -> *const ::core::ffi::c_char {
    match state {
        mqtt_operational_state_t::MQTT_OPERATIONAL_UNKNOWN => {
            return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_operational_state_t::MQTT_OPERATIONAL_HEALTHY => {
            return b"healthy\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_operational_state_t::MQTT_OPERATIONAL_WARNING => {
            return b"warning\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_operational_state_t::MQTT_OPERATIONAL_ERROR => {
            return b"error\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_operational_state_t::MQTT_OPERATIONAL_CRITICAL => {
            return b"critical\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return ::core::ptr::null::<::core::ffi::c_char>(),
    };
}
unsafe extern "C" fn current_operational_state(
    mut active_count: *mut size_t,
) -> mqtt_operational_state_t {
    if !active_count.is_null() {
        *active_count = 0 as size_t;
    }
    let mut snapshot: system_health_snapshot_t = system_health_snapshot_t {
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
    let mut baseline_known: bool =
        system_health_snapshot_copy(&raw mut snapshot) as ::core::ffi::c_int != 0
            && snapshot.sequence > 0 as uint64_t
            && snapshot.observation_count > 0 as size_t;
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
    let mut count: size_t = system_health_evaluator_service_active_copy(
        &raw mut incidents as *mut system_health_incident_view_t,
        SYSTEM_HEALTH_MAX_INCIDENTS as size_t,
    );
    if !active_count.is_null() {
        *active_count = count;
    }
    let mut maximum: system_health_severity_t =
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE;
    let mut index: size_t = 0 as size_t;
    while index < count {
        if incidents[index].severity.0 > maximum.0 {
            maximum = incidents[index].severity;
        }
        index = index.wrapping_add(1);
    }
    if maximum.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL.0 {
        return mqtt_operational_state_t::MQTT_OPERATIONAL_CRITICAL;
    }
    if maximum.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_ERROR.0 {
        return mqtt_operational_state_t::MQTT_OPERATIONAL_ERROR;
    }
    if maximum.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING.0 {
        return mqtt_operational_state_t::MQTT_OPERATIONAL_WARNING;
    }
    return mqtt_operational_state_t(
        (if baseline_known as ::core::ffi::c_int != 0 {
            mqtt_operational_state_t::MQTT_OPERATIONAL_HEALTHY.0 as ::core::ffi::c_int
        } else {
            mqtt_operational_state_t::MQTT_OPERATIONAL_UNKNOWN.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn build_presence_payload(
    mut client: *mut mqtt_destination_client_t,
    mut state: mqtt_presence_state_t,
    mut operational: mqtt_operational_state_t,
    mut active_incidents: size_t,
    mut observed_at_ms: int64_t,
    mut sequence: uint64_t,
    mut payload: *mut ::core::ffi::c_char,
    mut payload_size: size_t,
) -> ::core::ffi::c_int {
    let mut state_name: *const ::core::ffi::c_char = presence_state_name(state);
    let mut operational_name: *const ::core::ffi::c_char = operational_state_name(operational);
    if client.is_null()
        || state_name.is_null()
        || operational_name.is_null()
        || observed_at_ms <= 0 as int64_t
        || payload.is_null()
        || payload_size == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut written: ::core::ffi::c_int = snprintf(
        payload,
        payload_size,
        b"{\"schema_version\":1,\"installation_uuid\":\"%s\",\"run_id\":\"%s\",\"boot_id\":\"%s\",\"visibility_scope\":\"%s\",\"version\":\"%s\",\"sequence\":%lu,\"timestamp_ms\":%ld,\"state\":\"%s\",\"overall_state\":\"%s\",\"active_incidents\":%zu}\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*client).installation_uuid as *mut ::core::ffi::c_char,
        &raw mut (*client).run_id as *mut ::core::ffi::c_char,
        &raw mut (*client).boot_id as *mut ::core::ffi::c_char,
        MANAGED_PRESENCE_VISIBILITY.as_ptr(),
        LIGHTNVR_VERSION_STRING.as_ptr(),
        sequence,
        observed_at_ms,
        state_name,
        operational_name,
        active_incidents,
    );
    if written < 0 as ::core::ffi::c_int || written as size_t >= payload_size {
        *payload.offset(0isize) = '\0' as ::core::ffi::c_char;
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn publish_presence(
    mut client: *mut mqtt_destination_client_t,
    mut state: mqtt_presence_state_t,
) -> ::core::ffi::c_int {
    if client.is_null() || !(*client).presence_configured {
        return 0 as ::core::ffi::c_int;
    }
    let mut active_count: size_t = 0 as size_t;
    let mut operational: mqtt_operational_state_t =
        current_operational_state(&raw mut active_count);
    let mut now: int64_t = wall_time_ms();
    let mut payload: [::core::ffi::c_char; 1024] = [0; 1024];
    pthread_mutex_lock(&raw mut (*client).mutex);
    if !(*client).connected && state.0 != mqtt_presence_state_t::MQTT_PRESENCE_OFFLINE.0
        || (*client).shutting_down as ::core::ffi::c_int != 0
            && state.0 != mqtt_presence_state_t::MQTT_PRESENCE_STOPPING.0
    {
        pthread_mutex_unlock(&raw mut (*client).mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut sequence: uint64_t = if state.0 == mqtt_presence_state_t::MQTT_PRESENCE_OFFLINE.0 {
        0 as uint64_t
    } else {
        increment_saturated((*client).presence_sequence)
    };
    if build_presence_payload(
        client,
        state,
        operational,
        active_count,
        now,
        sequence,
        &raw mut payload as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(&raw mut (*client).mutex);
        stats_publish((*client).stats_slot, r#false != 0, now);
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = mosquitto_publish(
        (*client).mosq,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut (*client).status_topic as *mut ::core::ffi::c_char,
        strlen(&raw mut payload as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
        &raw mut payload as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        (*client).profile.qos,
        r#true != 0,
    );
    if result == mosq_err_t::MOSQ_ERR_SUCCESS.0
        && state.0 != mqtt_presence_state_t::MQTT_PRESENCE_OFFLINE.0
    {
        (*client).presence_sequence = sequence;
    }
    pthread_mutex_unlock(&raw mut (*client).mutex);
    stats_publish(
        (*client).stats_slot,
        result == mosq_err_t::MOSQ_ERR_SUCCESS.0,
        now,
    );
    return if result == mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn on_connect(
    mut mosq: *mut mosquitto,
    mut userdata: *mut ::core::ffi::c_void,
    mut result: ::core::ffi::c_int,
) {
    let mut client: *mut mqtt_destination_client_t = userdata as *mut mqtt_destination_client_t;
    if client.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut (*client).mutex);
    let mut reconnect: bool = (*client).ever_connected;
    let mut accepted: bool = !(*client).shutting_down && result == 0 as ::core::ffi::c_int;
    (*client).connected = accepted;
    if accepted {
        (*client).ever_connected = r#true != 0;
    }
    pthread_mutex_unlock(&raw mut (*client).mutex);
    if accepted {
        stats_connection((*client).stats_slot, r#true != 0, reconnect);
        publish_presence(client, mqtt_presence_state_t::MQTT_PRESENCE_ONLINE);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Managed destination '%s' connected to %s:%d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*client).profile.name as *mut ::core::ffi::c_char,
            &raw mut (*client).profile.broker_host as *mut ::core::ffi::c_char,
            (*client).profile.broker_port,
        );
    } else {
        stats_failure(
            (*client).stats_slot,
            mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_CONNECTION,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Managed destination '%s' connection refused: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*client).profile.name as *mut ::core::ffi::c_char,
            mosquitto_connack_string(result),
        );
    };
}
unsafe extern "C" fn on_disconnect(
    mut mosq: *mut mosquitto,
    mut userdata: *mut ::core::ffi::c_void,
    mut result: ::core::ffi::c_int,
) {
    let mut client: *mut mqtt_destination_client_t = userdata as *mut mqtt_destination_client_t;
    if client.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut (*client).mutex);
    let mut was_connected: bool = (*client).connected;
    (*client).connected = r#false != 0;
    if (*client).publish_waiting {
        (*client).publish_failed = r#true != 0;
        pthread_cond_broadcast(&raw mut (*client).wake);
    }
    let mut shutting_down: bool = (*client).shutting_down;
    pthread_mutex_unlock(&raw mut (*client).mutex);
    if was_connected {
        stats_connection((*client).stats_slot, r#false != 0, r#false != 0);
    }
    if !shutting_down && result != 0 as ::core::ffi::c_int {
        stats_failure(
            (*client).stats_slot,
            mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_CONNECTION,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Managed destination '%s' disconnected (rc=%d); reconnecting\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*client).profile.name as *mut ::core::ffi::c_char,
            result,
        );
    }
}
unsafe extern "C" fn on_publish(
    mut mosq: *mut mosquitto,
    mut userdata: *mut ::core::ffi::c_void,
    mut mid: ::core::ffi::c_int,
) {
    let mut client: *mut mqtt_destination_client_t = userdata as *mut mqtt_destination_client_t;
    if client.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut (*client).mutex);
    if (*client).publish_waiting as ::core::ffi::c_int != 0 && (*client).publish_mid == mid {
        (*client).publish_acknowledged = r#true != 0;
        pthread_cond_broadcast(&raw mut (*client).wake);
    }
    pthread_mutex_unlock(&raw mut (*client).mutex);
}
unsafe extern "C" fn on_log(
    mut mosq: *mut mosquitto,
    mut userdata: *mut ::core::ffi::c_void,
    mut level: ::core::ffi::c_int,
    mut message: *const ::core::ffi::c_char,
) {
    let mut client: *mut mqtt_destination_client_t = userdata as *mut mqtt_destination_client_t;
    if client.is_null() || message.is_null() {
        return;
    }
    if level == MOSQ_LOG_ERR {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Managed destination '%s': %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*client).profile.name as *mut ::core::ffi::c_char,
            message,
        );
    } else if level == MOSQ_LOG_WARNING {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Managed destination '%s': %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*client).profile.name as *mut ::core::ffi::c_char,
            message,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Managed destination '%s': %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*client).profile.name as *mut ::core::ffi::c_char,
            message,
        );
    };
}
unsafe extern "C" fn configure_tls(
    mut client: *mut mqtt_destination_client_t,
) -> ::core::ffi::c_int {
    if strcmp(
        &raw mut (*client).profile.tls_mode as *mut ::core::ffi::c_char,
        b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut system_trust: bool = strcmp(
        &raw mut (*client).profile.tls_mode as *mut ::core::ffi::c_char,
        b"system\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut result: ::core::ffi::c_int = mosq_err_t::MOSQ_ERR_SUCCESS.0;
    if system_trust {
        result = mosquitto_int_option(
            (*client).mosq,
            mosq_opt_t::MOSQ_OPT_TLS_USE_OS_CERTS,
            1 as ::core::ffi::c_int,
        );
    } else {
        let mut mutual: bool = strcmp(
            &raw mut (*client).profile.tls_mode as *mut ::core::ffi::c_char,
            b"mutual\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
        result = mosquitto_tls_set(
            (*client).mosq,
            &raw mut (*client).profile.ca_file as *mut ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            if mutual as ::core::ffi::c_int != 0 {
                &raw mut (*client).profile.cert_file as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            if mutual as ::core::ffi::c_int != 0 {
                &raw mut (*client).profile.key_file as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            None,
        );
    }
    if result == mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        result = mosquitto_tls_opts_set(
            (*client).mosq,
            1 as ::core::ffi::c_int,
            b"tlsv1.2\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    return result;
}
unsafe extern "C" fn configure_presence(
    mut client: *mut mqtt_destination_client_t,
) -> ::core::ffi::c_int {
    if (*client).profile.status_topic_template[0usize] == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut source: [::core::ffi::c_char; 128] = [0; 128];
    static mut prefix: [::core::ffi::c_char; 14] = unsafe {
        ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"urn:lightnvr:\0")
    };
    let mut run: system_health_process_run_t = system_health_process_run_t {
        run_id: [0; 37],
        boot_id: [0; 64],
        started_at_ms: 0,
        closed_at_ms: 0,
        clean_close: false,
    };
    memset(
        &raw mut run as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_process_run_t>(),
    );
    if event_identity_get_source(
        &raw mut source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    ) != 0 as ::core::ffi::c_int
        || strncmp(
            &raw mut source as *mut ::core::ffi::c_char,
            &raw const prefix as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 14]>().wrapping_sub(1 as size_t),
        ) != 0 as ::core::ffi::c_int
        || !system_health_evaluator_service_copy_run(&raw mut run)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut installation_uuid: *const ::core::ffi::c_char = (&raw mut source
        as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<[::core::ffi::c_char; 14]>() as isize)
        .offset(-(1 as ::core::ffi::c_uint as isize));
    if expand_status_topic(
        &raw mut (*client).profile,
        installation_uuid,
        &raw mut (*client).status_topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    snprintf(
        &raw mut (*client).installation_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        installation_uuid,
    );
    snprintf(
        &raw mut (*client).run_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut run.run_id as *mut ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*client).boot_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut run.boot_id as *mut ::core::ffi::c_char,
    );
    (*client).presence_configured = r#true != 0;
    let mut now: int64_t = wall_time_ms();
    let mut payload: [::core::ffi::c_char; 1024] = [0; 1024];
    if build_presence_payload(
        client,
        mqtt_presence_state_t::MQTT_PRESENCE_OFFLINE,
        mqtt_operational_state_t::MQTT_OPERATIONAL_UNKNOWN,
        0 as size_t,
        now,
        0 as uint64_t,
        &raw mut payload as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    return mosquitto_will_set(
        (*client).mosq,
        &raw mut (*client).status_topic as *mut ::core::ffi::c_char,
        strlen(&raw mut payload as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
        &raw mut payload as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        (*client).profile.qos,
        r#true != 0,
    );
}
unsafe extern "C" fn heartbeat_main(
    mut argument: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut client: *mut mqtt_destination_client_t = argument as *mut mqtt_destination_client_t;
    pthread_mutex_lock(&raw mut (*client).mutex);
    while !(*client).heartbeat_stop {
        let mut deadline: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_REALTIME, &raw mut deadline);
        deadline.tv_sec += MANAGED_PRESENCE_INTERVAL_SECONDS as __time_t;
        let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !(*client).heartbeat_stop && result != ETIMEDOUT {
            result = pthread_cond_timedwait(
                &raw mut (*client).heartbeat_wake,
                &raw mut (*client).mutex,
                &raw mut deadline,
            );
            if result != 0 as ::core::ffi::c_int && result != ETIMEDOUT {
                break;
            }
        }
        let mut publish: bool = !(*client).heartbeat_stop
            && (*client).connected as ::core::ffi::c_int != 0
            && result == ETIMEDOUT;
        pthread_mutex_unlock(&raw mut (*client).mutex);
        if publish {
            publish_presence(client, mqtt_presence_state_t::MQTT_PRESENCE_ONLINE);
        }
        pthread_mutex_lock(&raw mut (*client).mutex);
    }
    pthread_mutex_unlock(&raw mut (*client).mutex);
    return NULL;
}
unsafe extern "C" fn destroy_partial(mut client: *mut mqtt_destination_client_t) {
    if client.is_null() {
        return;
    }
    if (*client).loop_started as ::core::ffi::c_int != 0 && !(*client).mosq.is_null() {
        mosquitto_disconnect((*client).mosq);
        mosquitto_loop_stop((*client).mosq, r#true != 0);
    }
    if !(*client).mosq.is_null() {
        mosquitto_destroy((*client).mosq);
    }
    if (*client).heartbeat_condition_ready {
        pthread_cond_destroy(&raw mut (*client).heartbeat_wake);
    }
    if (*client).synchronization_ready {
        pthread_cond_destroy(&raw mut (*client).wake);
        pthread_mutex_destroy(&raw mut (*client).mutex);
    }
    release_stats_slot((*client).stats_slot);
    memset(
        client as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mqtt_destination_client_t>(),
    );
    free(client as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_destination_client_create(
    mut profile: *const event_destination_t,
) -> *mut mqtt_destination_client_t {
    let mut validation_error: [::core::ffi::c_char; 256] = [
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
    if profile.is_null()
        || !(*profile).enabled
        || db_event_destination_validate(
            profile,
            ::core::ptr::null::<::core::ffi::c_char>(),
            r#false != 0,
            &raw mut validation_error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        )
        .0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0
    {
        return ::core::ptr::null_mut::<mqtt_destination_client_t>();
    }
    let mut client: *mut mqtt_destination_client_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<mqtt_destination_client_t>(),
    ) as *mut mqtt_destination_client_t;
    if client.is_null() {
        return ::core::ptr::null_mut::<mqtt_destination_client_t>();
    }
    (*client).profile = *profile;
    (*client).publish_mid = -1 as ::core::ffi::c_int;
    (*client).stats_slot = -1 as ::core::ffi::c_int;
    (*client).stats_slot = reserve_stats_slot(profile);
    if (*client).stats_slot < 0 as ::core::ffi::c_int {
        free(client as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<mqtt_destination_client_t>();
    }
    if pthread_mutex_init(
        &raw mut (*client).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        destroy_partial(client);
        return ::core::ptr::null_mut::<mqtt_destination_client_t>();
    }
    if pthread_cond_init(
        &raw mut (*client).wake,
        ::core::ptr::null::<pthread_condattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_destroy(&raw mut (*client).mutex);
        release_stats_slot((*client).stats_slot);
        free(client as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<mqtt_destination_client_t>();
    }
    (*client).synchronization_ready = r#true != 0;
    if pthread_cond_init(
        &raw mut (*client).heartbeat_wake,
        ::core::ptr::null::<pthread_condattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        destroy_partial(client);
        return ::core::ptr::null_mut::<mqtt_destination_client_t>();
    }
    (*client).heartbeat_condition_ready = r#true != 0;
    let mut password: [::core::ffi::c_char; 256] = [
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
    let mut password_result: db_event_destination_result_t = db_event_destination_get_password(
        &raw const (*profile).uuid as *const ::core::ffi::c_char,
        (*profile).revision,
        &raw mut password as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if password_result.0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0 {
        secure_zero_memory(
            &raw mut password as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        );
        stats_failure(
            (*client).stats_slot,
            mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_CONFIGURATION,
        );
        destroy_partial(client);
        return ::core::ptr::null_mut::<mqtt_destination_client_t>();
    }
    (*client).mosq = mosquitto_new(
        &raw const (*profile).client_id as *const ::core::ffi::c_char,
        r#true != 0,
        client as *mut ::core::ffi::c_void,
    );
    if (*client).mosq.is_null() {
        secure_zero_memory(
            &raw mut password as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        );
        stats_failure(
            (*client).stats_slot,
            mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_CONFIGURATION,
        );
        destroy_partial(client);
        return ::core::ptr::null_mut::<mqtt_destination_client_t>();
    }
    mosquitto_connect_callback_set(
        (*client).mosq,
        Some(
            on_connect
                as unsafe extern "C" fn(
                    *mut mosquitto,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    mosquitto_disconnect_callback_set(
        (*client).mosq,
        Some(
            on_disconnect
                as unsafe extern "C" fn(
                    *mut mosquitto,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    mosquitto_publish_callback_set(
        (*client).mosq,
        Some(
            on_publish
                as unsafe extern "C" fn(
                    *mut mosquitto,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    mosquitto_log_callback_set(
        (*client).mosq,
        Some(
            on_log
                as unsafe extern "C" fn(
                    *mut mosquitto,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                ) -> (),
        ),
    );
    let mut result: ::core::ffi::c_int = mosq_err_t::MOSQ_ERR_SUCCESS.0;
    if (*profile).username[0usize] != 0 {
        result = mosquitto_username_pw_set(
            (*client).mosq,
            &raw const (*profile).username as *const ::core::ffi::c_char,
            if password[0usize] as ::core::ffi::c_int != 0 {
                &raw mut password as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
        );
    }
    secure_zero_memory(
        &raw mut password as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if result == mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        result = configure_tls(client);
    }
    if result == mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        result = configure_presence(client);
    }
    if result == mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        result = mosquitto_reconnect_delay_set(
            (*client).mosq,
            1 as ::core::ffi::c_uint,
            30 as ::core::ffi::c_uint,
            r#true != 0,
        );
    }
    if result == mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        result = mosquitto_connect_async(
            (*client).mosq,
            &raw const (*profile).broker_host as *const ::core::ffi::c_char,
            (*profile).broker_port,
            (*profile).keepalive_seconds,
        );
    }
    if result == mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        result = mosquitto_loop_start((*client).mosq);
        (*client).loop_started = result == mosq_err_t::MOSQ_ERR_SUCCESS.0;
    }
    if result == mosq_err_t::MOSQ_ERR_SUCCESS.0
        && (*client).presence_configured as ::core::ffi::c_int != 0
    {
        result = if pthread_create(
            &raw mut (*client).heartbeat_thread,
            ::core::ptr::null::<pthread_attr_t>(),
            Some(
                heartbeat_main
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            client as *mut ::core::ffi::c_void,
        ) == 0 as ::core::ffi::c_int
        {
            mosq_err_t::MOSQ_ERR_SUCCESS.0
        } else {
            mosq_err_t::MOSQ_ERR_ERRNO.0
        };
        (*client).heartbeat_started = result == mosq_err_t::MOSQ_ERR_SUCCESS.0;
    }
    if result != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        stats_failure(
            (*client).stats_slot,
            mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_CONFIGURATION,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Could not initialize managed destination '%s': %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*profile).name as *const ::core::ffi::c_char,
            mosquitto_strerror(result),
        );
        destroy_partial(client);
        return ::core::ptr::null_mut::<mqtt_destination_client_t>();
    }
    return client;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_destination_client_destroy(
    mut client: *mut mqtt_destination_client_t,
) {
    if client.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut (*client).mutex);
    (*client).heartbeat_stop = r#true != 0;
    pthread_cond_broadcast(&raw mut (*client).heartbeat_wake);
    pthread_mutex_unlock(&raw mut (*client).mutex);
    if (*client).heartbeat_started {
        pthread_join(
            (*client).heartbeat_thread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        (*client).heartbeat_started = r#false != 0;
    }
    if mqtt_destination_client_is_connected(client) {
        publish_presence(client, mqtt_presence_state_t::MQTT_PRESENCE_STOPPING);
    }
    pthread_mutex_lock(&raw mut (*client).mutex);
    (*client).shutting_down = r#true != 0;
    (*client).connected = r#false != 0;
    if (*client).publish_waiting {
        (*client).publish_failed = r#true != 0;
        pthread_cond_broadcast(&raw mut (*client).wake);
    }
    pthread_mutex_unlock(&raw mut (*client).mutex);
    destroy_partial(client);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_destination_client_is_connected(
    mut client: *mut mqtt_destination_client_t,
) -> bool {
    if client.is_null() {
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut (*client).mutex);
    let mut connected: bool =
        (*client).connected as ::core::ffi::c_int != 0 && !(*client).shutting_down;
    pthread_mutex_unlock(&raw mut (*client).mutex);
    return connected;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_destination_client_get_stats(
    mut client: *mut mqtt_destination_client_t,
    mut stats: *mut mqtt_destination_client_stats_t,
) -> bool {
    if client.is_null()
        || stats.is_null()
        || (*client).stats_slot < 0 as ::core::ffi::c_int
        || (*client).stats_slot >= EVENT_DESTINATION_MAX_COUNT
    {
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut stats_lock);
    let mut available: bool = stats_slots[(*client).stats_slot as usize].occupied;
    if available {
        *stats = stats_slots[(*client).stats_slot as usize].value;
    }
    pthread_mutex_unlock(&raw mut stats_lock);
    return available;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_destination_client_list_stats(
    mut stats: *mut mqtt_destination_client_stats_t,
    mut capacity: size_t,
) -> size_t {
    if stats.is_null() || capacity == 0 as size_t {
        return 0 as size_t;
    }
    let mut count: size_t = 0 as size_t;
    pthread_mutex_lock(&raw mut stats_lock);
    let mut index: size_t = 0 as size_t;
    while index < EVENT_DESTINATION_MAX_COUNT as size_t && count < capacity {
        if stats_slots[index].occupied {
            let c2rust_fresh2 = count;
            count = count.wrapping_add(1);
            *stats.offset(c2rust_fresh2 as isize) = stats_slots[index].value;
        }
        index = index.wrapping_add(1);
    }
    pthread_mutex_unlock(&raw mut stats_lock);
    let mut index_0: size_t = 1 as size_t;
    while index_0 < count {
        let mut value: mqtt_destination_client_stats_t = *stats.offset(index_0 as isize);
        let mut position: size_t = index_0;
        while position > 0 as size_t
            && strcmp(
                &raw mut (*stats.offset(position.wrapping_sub(1 as size_t) as isize))
                    .destination_uuid as *mut ::core::ffi::c_char,
                &raw mut value.destination_uuid as *mut ::core::ffi::c_char,
            ) > 0 as ::core::ffi::c_int
        {
            *stats.offset(position as isize) =
                *stats.offset(position.wrapping_sub(1 as size_t) as isize);
            position = position.wrapping_sub(1);
        }
        *stats.offset(position as isize) = value;
        index_0 = index_0.wrapping_add(1);
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqtt_destination_client_publish_confirmed(
    mut client: *mut mqtt_destination_client_t,
    mut topic: *const ::core::ffi::c_char,
    mut payload: *const ::core::ffi::c_char,
    mut retain: bool,
    mut timeout_ms: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if client.is_null()
        || topic.is_null()
        || payload.is_null()
        || timeout_ms <= 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut payload_length: size_t = strlen(payload);
    if payload_length > INT_MAX as size_t {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*client).mutex);
    if !(*client).connected
        || (*client).shutting_down as ::core::ffi::c_int != 0
        || (*client).publish_waiting as ::core::ffi::c_int != 0
    {
        pthread_mutex_unlock(&raw mut (*client).mutex);
        stats_publish((*client).stats_slot, r#false != 0, wall_time_ms());
        return -1 as ::core::ffi::c_int;
    }
    (*client).publish_waiting = r#true != 0;
    (*client).publish_acknowledged = r#false != 0;
    (*client).publish_failed = r#false != 0;
    (*client).publish_mid = -1 as ::core::ffi::c_int;
    let mut mid: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut result: ::core::ffi::c_int = mosquitto_publish(
        (*client).mosq,
        &raw mut mid,
        topic,
        payload_length as ::core::ffi::c_int,
        payload as *const ::core::ffi::c_void,
        (*client).profile.qos,
        retain,
    );
    (*client).publish_mid = mid;
    if result != mosq_err_t::MOSQ_ERR_SUCCESS.0 {
        (*client).publish_waiting = r#false != 0;
        (*client).publish_mid = -1 as ::core::ffi::c_int;
        pthread_mutex_unlock(&raw mut (*client).mutex);
        stats_publish((*client).stats_slot, r#false != 0, wall_time_ms());
        return -1 as ::core::ffi::c_int;
    }
    let mut deadline: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut deadline);
    deadline.tv_sec += (timeout_ms / 1000 as ::core::ffi::c_int) as __time_t;
    deadline.tv_nsec += (timeout_ms % 1000 as ::core::ffi::c_int) as ::core::ffi::c_long
        * 1000000 as ::core::ffi::c_long;
    if deadline.tv_nsec >= 1000000000 as ::core::ffi::c_long {
        deadline.tv_sec += 1;
        deadline.tv_nsec -= 1000000000 as ::core::ffi::c_long;
    }
    while !(*client).publish_acknowledged && !(*client).publish_failed {
        result = pthread_cond_timedwait(
            &raw mut (*client).wake,
            &raw mut (*client).mutex,
            &raw mut deadline,
        );
        if result == ETIMEDOUT {
            break;
        }
        if result == 0 as ::core::ffi::c_int {
            continue;
        }
        (*client).publish_failed = r#true != 0;
        break;
    }
    let mut acknowledged: bool =
        (*client).publish_acknowledged as ::core::ffi::c_int != 0 && !(*client).publish_failed;
    (*client).publish_waiting = r#false != 0;
    (*client).publish_acknowledged = r#false != 0;
    (*client).publish_failed = r#false != 0;
    (*client).publish_mid = -1 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut (*client).mutex);
    stats_publish((*client).stats_slot, acknowledged, wall_time_ms());
    return if acknowledged as ::core::ffi::c_int != 0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
